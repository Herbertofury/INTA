param([Parameter(Mandatory)][string]$UnsignedPackage,
      [Parameter(Mandatory)][string]$ExecutableHash,
      [Parameter(Mandatory)][Version]$Version)
$ErrorActionPreference = 'Stop'
if ($env:GITHUB_ACTIONS -ne 'true' -or $env:RUNNER_ENVIRONMENT -ne 'github-hosted') {
    throw 'This install and uninstall test requires a disposable GitHub-hosted Windows runner.'
}
$identity = 'INTA.Desktop.Preview'
if (Get-AppxPackage -Name $identity) { throw 'A pre-existing registration would invalidate the clean install test.' }
$work = Join-Path $env:RUNNER_TEMP ('inta-msix-install-' + [guid]::NewGuid().ToString('N'))
$null = New-Item -ItemType Directory -Path $work
$candidate = Join-Path $work 'fixture.msix'
Copy-Item -LiteralPath $UnsignedPackage -Destination $candidate
$sdk = Join-Path ${env:ProgramFiles(x86)} 'Windows Kits\10\bin\10.0.26100.0\x64'
$cert = $null
$trusted = $null
try {
    $cert = New-SelfSignedCertificate -Type CodeSigningCert -Subject 'CN=INTA Desktop Preview' `
        -CertStoreLocation Cert:\CurrentUser\My -KeyExportPolicy NonExportable -HashAlgorithm SHA256 `
        -NotAfter ([DateTime]::Now.AddDays(1))
    $public = Join-Path $work 'fixture.cer'
    $null = Export-Certificate -Cert $cert -FilePath $public
    $trusted = Import-Certificate -FilePath $public -CertStoreLocation Cert:\LocalMachine\TrustedPeople
    & (Join-Path $sdk 'signtool.exe') sign /fd SHA256 /s My /sha1 $cert.Thumbprint $candidate
    if ($LASTEXITCODE -ne 0) { throw 'Ephemeral fixture signing failed.' }
    & (Join-Path $sdk 'signtool.exe') verify /pa $candidate
    if ($LASTEXITCODE -ne 0) { throw 'Windows did not trust the disposable fixture signature.' }
    Add-AppxPackage -Path $candidate
    $added = @(Get-AppxPackage -Name $identity)
    if ($added.Count -ne 1 -or $added[0].Version -ne $Version.ToString() -or $added[0].Publisher -ne $cert.Subject) {
        throw 'Installed package identity did not match the fixture.'
    }
    $registered = Get-AppxPackageManifest -Package $added[0].PackageFullName
    if ($registered.Package.Applications.Application.Executable -cne 'INTA.Desktop.exe') { throw 'Incorrect registered application.' }
    $exe = Join-Path $added[0].InstallLocation 'INTA.Desktop.exe'
    if ((Get-FileHash -Algorithm SHA256 -LiteralPath $exe).Hash -ne $ExecutableHash) { throw 'Installed bytes changed.' }
    & (Join-Path $PSScriptRoot '..\verify-native-ui.ps1') $exe
} finally {
    $ours = Get-AppxPackage -Name $identity
    if ($ours -and $cert -and $ours.Publisher -eq $cert.Subject -and $ours.Version -eq $Version.ToString()) {
        Remove-AppxPackage -Package $ours.PackageFullName
    }
    if ($trusted) { Remove-Item -LiteralPath ('Cert:\LocalMachine\TrustedPeople\' + $trusted.Thumbprint) }
    if ($cert) { Remove-Item -LiteralPath ('Cert:\CurrentUser\My\' + $cert.Thumbprint) -DeleteKey }
}
if (Get-AppxPackage -Name $identity) { throw 'The test registration survived uninstall.' }
'Installed, exercised and uninstalled the disposable package with verified executable bytes.'
