[CmdletBinding()]
param(
    [Parameter(Mandatory)][ValidatePattern('^[0-9A-Fa-f]{40}$')][string]$Thumbprint,
    [Parameter(Mandatory,ParameterSetName='Files')][string[]]$Files,
    [Parameter(Mandatory,ParameterSetName='Payload')][string]$Payload,
    [Parameter(Mandatory,ParameterSetName='Launchers')][string]$Launchers,
    [switch]$VerifyOnly
)
$ErrorActionPreference = 'Stop'
Import-Module (Join-Path $PSHOME 'Modules\Microsoft.PowerShell.Security\Microsoft.PowerShell.Security.psd1') -ErrorAction Stop
$store = [Security.Cryptography.X509Certificates.X509Store]::new('My','CurrentUser')
$store.Open([Security.Cryptography.X509Certificates.OpenFlags]::ReadOnly)
try {
    $certificate = $store.Certificates.Find([Security.Cryptography.X509Certificates.X509FindType]::FindByThumbprint,$Thumbprint,$false) | Select-Object -First 1
} finally { $store.Close() }
if (!$certificate -or (!$VerifyOnly -and !$certificate.HasPrivateKey)) {
    throw 'The selected signing identity is unavailable.'
}
if ($certificate.NotBefore -gt [DateTime]::Now -or $certificate.NotAfter -le [DateTime]::Now) {
    throw 'The channel signing certificate is outside its validity period.'
}
# Windows reports CERT_E_UNTRUSTEDROOT for the private channel on a default machine.
# Check the exact localized HRESULT text so a bad file digest or revoked signer fails.
$untrustedRoot = [ComponentModel.Win32Exception]::new(-2146762487).Message.Trim()
if ($PSCmdlet.ParameterSetName -eq 'Payload') {
    $Files = @('inta.exe','inta-desktop.exe','inta-capabilities.exe','inta-diagnostic-handoff.exe','inta-views.exe','INTA System Tools.exe') | ForEach-Object { Join-Path $Payload $_ }
}
if ($PSCmdlet.ParameterSetName -eq 'Launchers') {
    $Files = @((Join-Path $Launchers 'inta.exe'), (Join-Path $Launchers 'INTA Desktop.exe'), (Join-Path $Launchers 'INTA Verify.exe'))
}
foreach ($file in $Files) {
    $path = (Get-Item -LiteralPath $file).FullName
    if ([IO.Path]::GetExtension($path) -notin @('.exe','.msi')) { throw "Unexpected signing target $path" }
    if (!$VerifyOnly) {
        Set-AuthenticodeSignature -LiteralPath $path -Certificate $certificate -HashAlgorithm SHA256 -IncludeChain Signer | Out-Null
    }
    $result = Get-AuthenticodeSignature -LiteralPath $path
    $privateRoot = $result.Status -in @('UnknownError','NotTrusted') -and $result.StatusMessage.Trim() -eq $untrustedRoot
    if ($result.SignerCertificate.Thumbprint -ne $Thumbprint -or ($result.Status -ne 'Valid' -and !$privateRoot)) {
        throw "Authenticode verification failed for $path. $($result.StatusMessage)"
    }
}
