[CmdletBinding()]
param(
    [string]$PackagePath = (Join-Path $PSScriptRoot 'inta-agent-win-x64.zip'),
    [switch]$RunAsAdministrator
)

$ErrorActionPreference = 'Stop'
$PackagePath = (Resolve-Path -LiteralPath $PackagePath).Path
$checksumPath = $PackagePath + '.sha256'
if (-not (Test-Path -LiteralPath $checksumPath)) { throw "Missing checksum sidecar: $checksumPath" }
$line = Get-Content -LiteralPath $checksumPath | Select-Object -First 1
$expected = ($line -split '\s+')[0].ToLowerInvariant()
$actual = (Get-FileHash -LiteralPath $PackagePath -Algorithm SHA256).Hash.ToLowerInvariant()
if ($actual -ne $expected) { throw 'Checksum verification failed. INTA Agent was not installed.' }

$temp = Join-Path $env:TEMP ('inta-agent-' + [guid]::NewGuid().ToString('N'))
$install = Join-Path $env:LOCALAPPDATA 'Programs\INTA\Agent'
try {
    Expand-Archive -LiteralPath $PackagePath -DestinationPath $temp -Force
    New-Item -ItemType Directory -Path $install -Force | Out-Null
    Copy-Item -Path (Join-Path $temp '*') -Destination $install -Recurse -Force

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
finally { Remove-Item -Path $temp -Recurse -Force -ErrorAction SilentlyContinue }
