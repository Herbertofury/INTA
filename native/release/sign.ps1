[CmdletBinding()]
param([Parameter(Mandatory)][string]$Directory, [Parameter(Mandatory)][string]$Thumbprint)
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Security
if ($Thumbprint -notmatch '^[0-9A-Fa-f]{40}$') { throw 'Invalid certificate thumbprint' }
$store=[Security.Cryptography.X509Certificates.X509Store]::new('My','CurrentUser')
$store.Open([Security.Cryptography.X509Certificates.OpenFlags]::ReadOnly)
try {
    $certificate=$store.Certificates.Find([Security.Cryptography.X509Certificates.X509FindType]::FindByThumbprint,$Thumbprint,$false) | Select-Object -First 1
} finally { $store.Close() }
if (!$certificate.HasPrivateKey -or $certificate.NotAfter -lt [DateTime]::Now) { throw 'Development signing key unavailable or expired' }
$info = [Security.Cryptography.Pkcs.ContentInfo]::new([IO.File]::ReadAllBytes((Join-Path $Directory 'manifest.json')))
$cms = [Security.Cryptography.Pkcs.SignedCms]::new($info,$true)
$signer = [Security.Cryptography.Pkcs.CmsSigner]::new($certificate)
$signer.DigestAlgorithm = [Security.Cryptography.Oid]::new('2.16.840.1.101.3.4.2.1')
$signer.IncludeOption = [Security.Cryptography.X509Certificates.X509IncludeOption]::EndCertOnly
$cms.ComputeSignature($signer,$true)
$cms.CheckSignature($true)
[IO.File]::WriteAllBytes((Join-Path $Directory 'manifest.p7s'),$cms.Encode())

