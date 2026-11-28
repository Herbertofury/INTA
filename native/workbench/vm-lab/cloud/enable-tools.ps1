[CmdletBinding()]
param()
$ErrorActionPreference = 'Stop'
$wsl = Join-Path ([Environment]::SystemDirectory) 'wsl.exe'
$distro = 'INTA-Rescue-Builder'
$repo = (Resolve-Path (Join-Path $PSScriptRoot '../..')).Path
function Invoke-Lab([string]$User, [string[]]$Command) {
    $output = & $wsl -d $distro -u $User --exec @Command
    if ($LASTEXITCODE) { throw "Tool environment operation failed ($LASTEXITCODE)." }
    return $output
}
$uid = Invoke-Lab 'inta-lab' @('/usr/bin/id','-u')
if ($uid -notmatch '^[1-9][0-9]*$') { throw 'Prepare the dedicated Linux lab account first.' }
foreach ($file in @(
    @('live/includes.chroot/usr/local/bin/inta-tool','/usr/local/bin/inta-tool','0755'),
    @('data/tools.json','/usr/share/inta/tool-actions/tools.json','0644'),
    @('data/tool-actions.json','/usr/share/inta/tool-actions/tool-actions.json','0644')
)) {
    $origin = Invoke-Lab 'inta-lab' @('/usr/bin/wslpath','-a','-u',(Join-Path $repo $file[0]))
    Invoke-Lab 'root' @('/usr/bin/install','-D','-o','root','-g','root','-m',$file[2],$origin,$file[1])
}
$report = Invoke-Lab 'inta-lab' @('/usr/local/bin/inta-tool','--inventory')
$inventory = $report | ConvertFrom-Json
if ($null -eq $inventory.installed -or $null -eq $inventory.packages) { throw 'Invalid Linux inventory.' }
$dir = Join-Path $env:LOCALAPPDATA 'INTA/lab'
[void][IO.Directory]::CreateDirectory($dir)
$file = Join-Path $dir 'linux-tool-inventory.json'
$tmp = Join-Path $dir ([Guid]::NewGuid().ToString() + '.tmp')
[IO.File]::WriteAllText($tmp, $report, (New-Object Text.UTF8Encoding($false)))
if ([IO.File]::Exists($file)) { [IO.File]::Replace($tmp, $file, [System.Management.Automation.Language.NullString]::Value) } else { [IO.File]::Move($tmp, $file) }
$report
