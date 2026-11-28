[CmdletBinding()]
param(
    [Parameter(Mandatory)][ValidateSet('Initialize','Identity','Sign')][string]$Action,
    [string]$UnsignedApk,
    [ValidatePattern('^[a-fA-F0-9]{64}$')][string]$ExpectedSha256,
    [ValidatePattern('^[a-fA-F0-9]{40}$')][string]$Revision,
    [string]$ApkSignerClasspath,
    [string]$OutputDirectory
)
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Security
$local = [Environment]::GetFolderPath('LocalApplicationData')
$parent = Join-Path $local 'INTA\Signing'
$vault = Join-Path $parent 'Android'
$sid = [Security.Principal.WindowsIdentity]::GetCurrent().User
$scope = [Security.Cryptography.DataProtectionScope]::CurrentUser
$utf8 = [Text.UTF8Encoding]::new($false)

function Assert-LocalPath($path) {
    for ($node = [IO.DirectoryInfo]::new($path); $node; $node = $node.Parent) {
        if ($node.Exists -and ($node.Attributes -band [IO.FileAttributes]::ReparsePoint)) {
            throw "A signing location is redirected: $($node.FullName)"
        }
    }
}
function Protect-Folder($path) {
    $acl = [Security.AccessControl.DirectorySecurity]::new()
    $acl.SetOwner($sid)
    $acl.SetAccessRuleProtection($true, $false)
    foreach ($principal in @($sid.Value,'S-1-5-18','S-1-5-32-544')) {
        $rule = [Security.AccessControl.FileSystemAccessRule]::new(
            [Security.Principal.SecurityIdentifier]::new($principal), 'FullControl',
            'ContainerInherit,ObjectInherit', 'None', 'Allow')
        $acl.AddAccessRule($rule)
    }
    Set-Acl -LiteralPath $path -AclObject $acl
}
function Digest($path) { (Get-FileHash -LiteralPath $path -Algorithm SHA256).Hash.ToLowerInvariant() }
function Check-Identity($path) {
    Assert-LocalPath $path
    $acl = Get-Acl -LiteralPath $path
    if (!$acl.AreAccessRulesProtected -or $acl.GetOwner([Security.Principal.SecurityIdentifier]).Value -ne $sid.Value) {
        throw 'The Android signing directory has unexpected permissions or owner.'
    }
    foreach ($rule in $acl.GetAccessRules($true, $true, [Security.Principal.SecurityIdentifier])) {
        if ($rule.AccessControlType -eq 'Allow' -and $rule.IdentityReference.Value -notin @($sid.Value,'S-1-5-18','S-1-5-32-544')) {
            throw 'An unexpected principal has access to the Android signer.'
        }
    }
    foreach ($name in 'release.p12','password.dpapi','certificate.der') {
        $file = Get-Item -LiteralPath (Join-Path $path $name)
        if ($file.PSIsContainer -or $file.Length -eq 0 -or ($file.Attributes -band [IO.FileAttributes]::ReparsePoint)) {
            throw 'The Android signing material is incomplete or redirected.'
        }
    }
}

Assert-LocalPath $vault
$keytool = (Get-Command keytool.exe).Source
$java = Join-Path (Split-Path $keytool) 'java.exe'
if ($Action -eq 'Initialize' -and !(Test-Path -LiteralPath $vault)) {
    [void][IO.Directory]::CreateDirectory($parent)
    $stage = [IO.Directory]::CreateDirectory((Join-Path $parent ('creating-' + [Guid]::NewGuid().ToString('N')))).FullName
    Protect-Folder $stage
    $seed = New-Object byte[] 40
    $rng = [Security.Cryptography.RandomNumberGenerator]::Create()
    try {
        $rng.GetBytes($seed)
        $env:INTA_ANDROID_SIGNING_PASSWORD = [Convert]::ToBase64String($seed)
        $bytes = $utf8.GetBytes($env:INTA_ANDROID_SIGNING_PASSWORD)
        $wrapped = [Security.Cryptography.ProtectedData]::Protect($bytes, $null, $scope)
        [IO.File]::WriteAllBytes((Join-Path $stage 'password.dpapi'), $wrapped)
        $ks = Join-Path $stage 'release.p12'
        & $keytool -genkeypair -alias inta-mobile -keyalg RSA -keysize 3072 -sigalg SHA256withRSA -validity 15000 -dname 'CN=INTA Mobile, O=INTA' -storetype PKCS12 -keystore $ks -storepass:env INTA_ANDROID_SIGNING_PASSWORD -keypass:env INTA_ANDROID_SIGNING_PASSWORD
        if ($LASTEXITCODE) { throw 'Generating the Android update identity failed.' }
        & $keytool -exportcert -alias inta-mobile -keystore $ks -storepass:env INTA_ANDROID_SIGNING_PASSWORD -file (Join-Path $stage 'certificate.der')
        if ($LASTEXITCODE) { throw 'Exporting the public Android certificate failed.' }
        Check-Identity $stage
        [IO.Directory]::Move($stage, $vault)
    } finally {
        $rng.Dispose()
        [Array]::Clear($seed, 0, $seed.Length)
        if ($bytes) { [Array]::Clear($bytes, 0, $bytes.Length) }
        [Environment]::SetEnvironmentVariable('INTA_ANDROID_SIGNING_PASSWORD', $null, 'Process')
    }
}
Check-Identity $vault
$certHash = Digest (Join-Path $vault 'certificate.der')
if ($Action -ne 'Sign') {
    [pscustomobject]@{Alias='inta-mobile'; CertificateSha256=$certHash; Directory=$vault}
    return
}
if (!$ExpectedSha256 -or !$Revision -or !$UnsignedApk -or !$ApkSignerClasspath -or !$OutputDirectory) {
    throw 'Signing requires the accepted unsigned APK, its SHA-256 and Git revision, the apksigner classpath and a new output directory.'
}
$apk = (Resolve-Path -LiteralPath $UnsignedApk).ProviderPath
$out = [IO.Path]::GetFullPath($OutputDirectory)
if (Test-Path -LiteralPath $out) { throw 'Select an unused output directory.' }
if ((Digest $apk) -ne $ExpectedSha256) { throw 'The Android input differs from its accepted emulator artifact.' }
$stage = [IO.Directory]::CreateDirectory((Join-Path (Split-Path $out) ('.android-package-' + [Guid]::NewGuid().ToString('N')))).FullName
$signed = Join-Path $stage 'INTA-Mobile-Android.apk'
$tool = @('-cp', $ApkSignerClasspath, 'com.android.apksigner.ApkSignerTool')
$plain = [Security.Cryptography.ProtectedData]::Unprotect([IO.File]::ReadAllBytes((Join-Path $vault 'password.dpapi')), $null, $scope)
try {
    $env:INTA_ANDROID_SIGNING_PASSWORD = $utf8.GetString($plain)
    & $java @tool sign --v1-signing-enabled false --v2-signing-enabled true --v3-signing-enabled true --v4-signing-enabled false --ks (Join-Path $vault 'release.p12') --ks-key-alias inta-mobile --ks-pass env:INTA_ANDROID_SIGNING_PASSWORD --key-pass env:INTA_ANDROID_SIGNING_PASSWORD --out $signed $apk
    if ($LASTEXITCODE) { throw 'Android package signing failed.' }
} finally {
    [Array]::Clear($plain, 0, $plain.Length)
    [Environment]::SetEnvironmentVariable('INTA_ANDROID_SIGNING_PASSWORD', $null, 'Process')
}
$proof = @(& $java @tool verify --verbose --print-certs --min-sdk-version 26 $signed)
if ($LASTEXITCODE -or 'Number of signers: 1' -notin $proof -or
    "Signer #1 certificate SHA-256 digest: $certHash" -notin $proof -or
    'Verified using v2 scheme (APK Signature Scheme v2): true' -notin $proof -or
    'Verified using v3 scheme (APK Signature Scheme v3): true' -notin $proof) {
    throw 'The release signature differs from the expected Android identity or scheme.'
}
$receipt = Join-Path $stage 'verification.txt'
[IO.File]::WriteAllLines($receipt, [string[]](@("Source revision: $Revision","Verified input SHA-256: $ExpectedSha256") + $proof), $utf8)
[IO.File]::WriteAllLines((Join-Path $stage 'SHA256SUMS'), [string[]]@("$(Digest $signed)  INTA-Mobile-Android.apk","$(Digest $receipt)  verification.txt"), $utf8)
[IO.Directory]::Move($stage, $out)
Get-Content -LiteralPath (Join-Path $out 'SHA256SUMS')
