[CmdletBinding()]
param([Parameter(Mandatory)][string]$OutputDirectory,[Parameter(Mandatory)][string]$Thumbprint,[string]$MsiPythonVersion='3.8')
$ErrorActionPreference='Stop'
$root=Split-Path (Split-Path $PSScriptRoot)
$out=[IO.Path]::GetFullPath($OutputDirectory)
if(Test-Path -LiteralPath $out){throw 'Select a new release output directory.'}
[void][IO.Directory]::CreateDirectory($out)
$stage=[IO.Directory]::CreateDirectory((Join-Path $root ("target\release-bundle-"+[Guid]::NewGuid().ToString('N')))).FullName
$payload=Join-Path $stage 'app'
[void][IO.Directory]::CreateDirectory($payload)
$python=(Get-Command python.exe -ErrorAction Stop).Source
& $python -I (Join-Path $PSScriptRoot 'build.py') --output (Join-Path $payload 'base') --thumbprint $Thumbprint
if($LASTEXITCODE){throw 'Application validation failed'}
$csc=Join-Path $env:windir 'Microsoft.NET\Framework64\v4.0.30319\csc.exe'
foreach($entry in @(@('GUI','INTA Desktop.exe','winexe'),@('CLI','inta.exe','exe'),@('VERIFY','INTA Verify.exe','exe'))){
    & $csc /nologo /optimize+ /platform:x64 "/win32icon:$(Join-Path $root 'native\workbench\desktop\inta.ico')" "/define:$($entry[0])" "/target:$($entry[2])" "/out:$(Join-Path $payload $entry[1])" /r:System.dll /r:System.Core.dll /r:System.Security.dll /r:System.Web.Extensions.dll /r:System.Windows.Forms.dll (Join-Path $PSScriptRoot 'Launch.cs')
    if($LASTEXITCODE){throw 'Launcher build failed'}
}
& (Join-Path $PSScriptRoot 'sign-windows.ps1') -Thumbprint $Thumbprint -Launchers $payload
Export-Certificate -Cert (Get-Item -LiteralPath "Cert:\CurrentUser\My\$Thumbprint") -FilePath (Join-Path $payload 'channel.cer') | Out-Null
Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'devsync.py') -Destination $payload
$manifest=Get-Content -LiteralPath (Join-Path $payload 'base\manifest.json') -Raw | ConvertFrom-Json
& (Join-Path $payload 'INTA Verify.exe') (Join-Path $payload 'base') $manifest.sourceDigest
if($LASTEXITCODE){throw 'Packaged signature or contents failed verification'}
$msi=Join-Path $out 'INTA-Desktop-x64.msi'
& py "-$MsiPythonVersion" -I (Join-Path $PSScriptRoot 'make-msi.py') $payload $msi
if($LASTEXITCODE){throw 'Windows Installer database creation failed'}
& (Join-Path $PSScriptRoot 'sign-windows.ps1') -Thumbprint $Thumbprint -Files $msi
[IO.File]::WriteAllText((Join-Path $payload 'inta-portable.marker'),'INTA Desktop portable v1')
Add-Type -AssemblyName System.IO.Compression.FileSystem
$zip=Join-Path $out 'INTA-Desktop-portable-x64.zip'
[IO.Compression.ZipFile]::CreateFromDirectory($payload,$zip,[IO.Compression.CompressionLevel]::Optimal,$false)
$lines=foreach($file in $msi,$zip){"$((Get-FileHash -Algorithm SHA256 -LiteralPath $file).Hash.ToLower())  $([IO.Path]::GetFileName($file))"}
[IO.File]::WriteAllLines((Join-Path $out 'SHA256SUMS'),[string[]]$lines,[Text.UTF8Encoding]::new($false))
$manifest | Select-Object sourceDigest,revision

