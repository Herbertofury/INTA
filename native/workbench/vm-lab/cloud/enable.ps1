[CmdletBinding()]
param()
$ErrorActionPreference = 'Stop'
$wsl = Join-Path $env:SystemRoot 'System32\wsl.exe'
$distro = 'INTA-Rescue-Builder'
$digest = & $wsl --distribution $distro --user root --exec /usr/bin/sha256sum /usr/local/bin/inta-cloud-hypervisor
if ($LASTEXITCODE -or $digest -notmatch '^ce7133bf535ed5a4e2a63ea1f26025445a5b1241f73ddfb81ff2fbcb2e803a59 ') { throw 'The prepared Debian lab lacks the pinned Cloud Hypervisor v55.0 build.' }
& $wsl --distribution $distro --user root --exec /usr/bin/id -u inta-lab 2>$null | Out-Null
if ($LASTEXITCODE) {
    & $wsl --distribution $distro --user root --exec /usr/sbin/useradd --system --create-home --user-group --groups kvm --shell /usr/sbin/nologin inta-lab
    if ($LASTEXITCODE) { throw 'Unable to provision the dedicated KVM account.' }
}
& $wsl --distribution $distro --user inta-lab --exec /usr/bin/test -w /dev/kvm
if ($LASTEXITCODE) { throw 'The dedicated nonroot account could not access KVM.' }
$dir = Join-Path $env:LOCALAPPDATA 'INTA\lab'
[void][IO.Directory]::CreateDirectory($dir)
[IO.File]::WriteAllText((Join-Path $dir 'cloud-hypervisor-v55.enabled'), 'INTA-Rescue-Builder/inta-lab/v55')
Write-Output 'Cloud Hypervisor v55.0 is available to the dedicated KVM account.'
