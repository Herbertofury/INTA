[CmdletBinding()]
param([Parameter(Mandatory)][string]$SourceDirectory)
$ErrorActionPreference='Stop'
$source=(Resolve-Path -LiteralPath $SourceDirectory).ProviderPath
$install=Join-Path $env:LOCALAPPDATA 'Programs\INTA Desktop'
$dev=Join-Path $env:LOCALAPPDATA 'INTA\desktop-development'
$signer=[Security.Cryptography.X509Certificates.X509Certificate2]::new((Join-Path $install 'channel.cer')).Thumbprint
$owner=[Security.Principal.WindowsIdentity]::GetCurrent().User.Value
$allowed=@($owner,'S-1-5-18','S-1-5-32-544','S-1-3-0')
$acl=Get-Acl -LiteralPath $source
if(!$acl.AreAccessRulesProtected -or $acl.GetOwner([Security.Principal.SecurityIdentifier]).Value -notin $allowed){throw 'Secure the development checkout owner and disable broad inherited write access first.'}
$writes=[Security.AccessControl.FileSystemRights]::Write -bor [Security.AccessControl.FileSystemRights]::Delete -bor [Security.AccessControl.FileSystemRights]::ChangePermissions -bor [Security.AccessControl.FileSystemRights]::TakeOwnership -bor [Security.AccessControl.FileSystemRights]::DeleteSubdirectoriesAndFiles
foreach($rule in $acl.GetAccessRules($true,$true,[Security.Principal.SecurityIdentifier])){
    if($rule.AccessControlType -eq 'Allow' -and $rule.IdentityReference.Value -notin $allowed -and ($rule.FileSystemRights -band $writes)){throw 'A non-owner principal can write this development checkout.'}
}
$python=(Get-Command python.exe -ErrorAction Stop).Source
$pythonw=Join-Path (Split-Path $python) 'pythonw.exe'
# Numeric NTFS IDs can exceed JSON consumer integer precision.
$identity=& $python -I -c 'import os,sys,json; s=os.stat(sys.argv[1],follow_symlinks=False); print(json.dumps([str(s.st_dev),str(s.st_ino)]))' $source | ConvertFrom-Json
if($LASTEXITCODE -or !$identity){throw 'Unable to pin source directory identity'}
[void][IO.Directory]::CreateDirectory($dev)
$config=@{source=$source;volume=$identity[0];fileId=$identity[1];thumbprint=$signer;buildPath=$env:PATH}
[IO.File]::WriteAllText((Join-Path $dev 'source.json'),($config|ConvertTo-Json),[Text.UTF8Encoding]::new($false))
$worker=Join-Path $install 'devsync.py'
& $python -I $worker sync
$initialBuild=$LASTEXITCODE
$startup=[Environment]::GetFolderPath([Environment+SpecialFolder]::Startup)
$shell=New-Object -ComObject WScript.Shell
$link=$shell.CreateShortcut((Join-Path $startup 'INTA Desktop Development Sync.lnk'))
$link.TargetPath=$pythonw
$link.Arguments='-I "'+$worker+'" watch'
$link.WorkingDirectory=$install
$link.Description='Activate verified builds of the local INTA development checkout.'
$link.Save()
Start-Process -FilePath $pythonw -ArgumentList $link.Arguments -WorkingDirectory $install -WindowStyle Hidden
if($initialBuild){Write-Warning "The installed base remains available. The watcher will reconcile the current source. Build output: $(Join-Path $dev 'last-build.log')"}
if(Test-Path -LiteralPath (Join-Path $dev 'status.json')){Get-Content -LiteralPath (Join-Path $dev 'status.json')}

