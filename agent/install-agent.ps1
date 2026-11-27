[CmdletBinding()]
param([switch]$RunAsAdministrator)

$ErrorActionPreference = 'Stop'
$repo = 'Herbertofury/INTA'
$release = Invoke-RestMethod -Uri "https://api.github.com/repos/$repo/releases/latest" -Headers @{ 'User-Agent' = 'INTA-Local-Agent-Installer'; Accept = 'application/vnd.github+json' }
$asset = $release.assets | Where-Object name -eq 'inta-agent-win-x64.zip' | Select-Object -First 1
$checksums = $release.assets | Where-Object name -eq 'SHA256SUMS.txt' | Select-Object -First 1
if (-not $asset -or -not $checksums) { throw 'The latest release does not contain the Windows agent and checksum manifest.' }

$temp = Join-Path $env:TEMP ('inta-agent-' + [guid]::NewGuid().ToString('N'))
$install = Join-Path $env:LOCALAPPDATA 'Programs\INTA\Agent'
New-Item -ItemType Directory -Path $temp -Force | Out-Null
try {
    $zip = Join-Path $temp $asset.name
    $sumFile = Join-Path $temp 'SHA256SUMS.txt'
    Invoke-WebRequest -Uri $asset.browser_download_url -OutFile $zip
    Invoke-WebRequest -Uri $checksums.browser_download_url -OutFile $sumFile
    $line = Get-Content $sumFile | Where-Object { $_ -match '  inta-agent-win-x64\.zip$' } | Select-Object -First 1
    if (-not $line) { throw 'Windows package checksum is missing.' }
    $expected = ($line -split '\s+')[0].ToLowerInvariant()
    $actual = (Get-FileHash -Path $zip -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($actual -ne $expected) { throw 'Checksum verification failed. The agent was not installed.' }

    $stage = Join-Path $temp 'publish'
    Expand-Archive -Path $zip -DestinationPath $stage -Force
    New-Item -ItemType Directory -Path $install -Force | Out-Null
    Copy-Item -Path (Join-Path $stage '*') -Destination $install -Recurse -Force

    $launch = Join-Path $install 'Start-INTA-Agent.cmd'
    Set-Content -Path $launch -Encoding ASCII -Value '@echo off','cd /d "%~dp0"','inta-agent.exe'
    $admin = Join-Path $install 'Start-INTA-Agent-Admin.cmd'
    Set-Content -Path $admin -Encoding ASCII -Value '@echo off','powershell.exe -NoProfile -Command "Start-Process -FilePath ''%~dp0inta-agent.exe'' -Verb RunAs"'
    $menu = Join-Path $env:APPDATA 'Microsoft\Windows\Start Menu\Programs\INTA'
    New-Item -ItemType Directory -Path $menu -Force | Out-Null
    $shell = New-Object -ComObject WScript.Shell
    $shortcut = $shell.CreateShortcut((Join-Path $menu 'INTA Local Agent.lnk'))
    $shortcut.TargetPath = $launch
    $shortcut.WorkingDirectory = $install
    $shortcut.Description = 'Start the local INTA maintenance agent'
    $shortcut.Save()
    $adminShortcut = $shell.CreateShortcut((Join-Path $menu 'INTA Local Agent (Administrator).lnk'))
    $adminShortcut.TargetPath = $admin
    $adminShortcut.WorkingDirectory = $install
    $adminShortcut.Description = 'Start INTA Local Agent elevated for system-wide maintenance'
    $adminShortcut.Save()

    Write-Host "Installed INTA Local Agent to $install"
    Write-Host 'Start it from the Start menu, then open INTA > Updates & Maintenance > Connect this computer.'
    if ($RunAsAdministrator) { Start-Process -FilePath $admin }
    else { Start-Process -FilePath $launch }
}
finally {
    Remove-Item -Path $temp -Recurse -Force -ErrorAction SilentlyContinue
}
