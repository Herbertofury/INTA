"""Verify the packaged one-shot startup inventory without writing startup entries."""
import json
import pathlib
import subprocess
import sys

def invoke(argv):
    return subprocess.run(argv, capture_output=True, text=True, encoding='utf-8', timeout=20)

def run(exe):
    command = [str(exe), '--startup-inventory']
    bad = invoke(command + ['--write'])
    assert bad.returncode == 2 and not bad.stdout and 'Usage' in bad.stderr
    if sys.platform != 'win32':
        unsupported = invoke(command)
        assert unsupported.returncode == 3 and not unsupported.stdout and 'not supported' in unsupported.stderr
        extended = invoke(command + ['--include-folders'])
        assert extended.returncode == 3 and not extended.stdout
        print('Unsupported platform and unrelated arguments rejected with no inventory output.')
        return

    # Independent PowerShell process reads only registry value names; no fixture
    # writes, command payloads, registry argument access, or subprocess launch.
    source_cmd = r"""
$ErrorActionPreference = 'Stop'
[Console]::OutputEncoding = [Text.UTF8Encoding]::new($false)
function Get-Names($hive, $view, $name, $scope, $display) {
    $status = 'not-present'
    $entryNames = @()
    try {
        $base = [Microsoft.Win32.RegistryKey]::OpenBaseKey($hive, $view)
        try {
            $key = $base.OpenSubKey(('Software\Microsoft\Windows\CurrentVersion\' + $name), $false)
            if ($null -ne $key) {
                try { $entryNames = $key.GetValueNames(); $status = 'read' } finally { $key.Dispose() }
            }
        } finally { $base.Dispose() }
    } catch {
        if ($_.Exception.GetBaseException() -is [System.Security.SecurityException] -or
            $_.Exception.GetBaseException() -is [System.UnauthorizedAccessException] -or
            $_.Exception.GetBaseException() -is [System.IO.IOException]) { $status = 'unavailable' }
        else { throw }
    }
    @{scope = $scope; view = $display; key = $name; status = $status; entryNames = @($entryNames)}
}
$rows = foreach ($name in @('Run', 'RunOnce')) {
    Get-Names 'CurrentUser' 'Default' $name 'current-user' 'shared'
    if ([Environment]::Is64BitOperatingSystem) {
        Get-Names 'LocalMachine' 'Registry64' $name 'machine' '64-bit'
        Get-Names 'LocalMachine' 'Registry32' $name 'machine' '32-bit'
    } else { Get-Names 'LocalMachine' 'Default' $name 'machine' 'default' }
}
ConvertTo-Json -InputObject @($rows) -Compress -Depth 4
"""
    def reference():
        value = invoke(['powershell.exe', '-NoProfile', '-NonInteractive', '-Command', source_cmd])
        assert value.returncode == 0 and not value.stderr, 'Registry reference read failed'
        return json.loads(value.stdout)

    before = reference()
    result = invoke(command)
    assert result.returncode == 0 and not result.stderr
    after = reference()
    assert before == after, 'The host startup registrations changed during the test'
    report = json.loads(result.stdout)
    assert set(report) == {'schema', 'scope', 'readOnly', 'enabledState', 'argumentsCollected', 'sources'}
    assert report['schema'] == 1 and report['scope'] == 'windows-logon-run-keys-only'
    assert report['readOnly'] is True and report['argumentsCollected'] is False
    assert report['enabledState'] == 'not-evaluated'
    assert len(report['sources']) == len(before)
    for observed, expected in zip(report['sources'], before):
        assert set(observed) == set(expected) | {'truncated'}
        for key in ('scope', 'view', 'key', 'status'):
            assert observed[key] == expected[key]
        assert observed['truncated'] is False, 'Expected a bounded fixture host'
        assert sorted(observed['entryNames']) == sorted(expected['entryNames'])
    print('Native read-only inventory agrees with all probed registry views; sampled name inventories unchanged.')

    def read_folders():
        result = invoke(['powershell.exe', '-NoProfile', '-NonInteractive', '-Command', r'''
$ErrorActionPreference = 'Stop'
[Console]::OutputEncoding = [Text.UTF8Encoding]::new($false)
$rows = foreach ($kind in @('Startup','CommonStartup')) {
    $loc = [Environment]::GetFolderPath($kind,[Environment+SpecialFolderOption]::DoNotVerify)
    $scope = if ($kind -eq 'Startup') {'current-user'} else {'all-users'}
    if (-not $loc) { @{scope=$scope;status='unavailable';entryNames=@()}; continue }
    $root = [IO.Path]::GetPathRoot($loc)
    if (-not $root -or $root.Length -ne 3 -or ([IO.DriveInfo]::new($root)).DriveType -notin @('Fixed','Removable','Ram')) {
        @{scope=$scope;status='location-not-scanned';entryNames=@()}; continue
    }
    $parts = [Collections.Generic.Stack[IO.DirectoryInfo]]::new()
    for ($node = [IO.DirectoryInfo]::new($loc); $null -ne $node; $node = $node.Parent) { $parts.Push($node) }
    $status = 'read'
    while ($parts.Count) {
        $node = $parts.Pop()
        try { $atts = [IO.File]::GetAttributes($node.FullName) }
        catch {
            $cause = $_.Exception.GetBaseException()
            if ($cause -is [IO.DirectoryNotFoundException] -or $cause -is [IO.FileNotFoundException]) { $status='not-present' }
            elseif ($cause -is [IO.IOException] -or $cause -is [UnauthorizedAccessException] -or $cause -is [Security.SecurityException]) { $status='unavailable' }
            else { throw }
            break
        }
        if ($atts -band [IO.FileAttributes]::ReparsePoint) { $status='location-not-scanned'; break }
        if (-not ($atts -band [IO.FileAttributes]::Directory)) { $status='unavailable'; break }
    }
    $names = @()
    if ($status -eq 'read') {
        $names = @(Get-ChildItem -LiteralPath $loc -Force | ForEach-Object { $_.Name } | Sort-Object -CaseSensitive)
    }
    @{scope=$scope;status=$status;entryNames=$names}
}
ConvertTo-Json -InputObject @($rows) -Compress -Depth 4
'''])
        assert result.returncode == 0 and not result.stderr, 'Windows Startup folder reference read failed'
        return json.loads(result.stdout)

    folders_before = read_folders()
    extended = invoke(command + ['--include-folders'])
    after = read_folders()
    assert after == folders_before
    assert extended.returncode == 0 and not extended.stderr
    new = json.loads(extended.stdout)
    assert set(new) == set(report) | {'startupFolders', 'startupSettingsUri'}
    assert new['schema'] == 2 and new['scope'] == 'windows-logon-registrations-and-local-startup-folders'
    assert new['startupSettingsUri'] == 'ms-settings:startupapps'
    for key in ('readOnly', 'enabledState', 'argumentsCollected', 'sources'):
        assert new[key] == report[key]
    assert [x['scope'] for x in new['startupFolders']] == ['current-user', 'all-users']
    for observed, expected in zip(new['startupFolders'], folders_before):
        assert set(observed) == set(expected) | {'truncated'}
        assert not observed['truncated']
        assert observed['scope'] == expected['scope'] and observed['status'] == expected['status']
        assert sorted(observed['entryNames']) == sorted(expected['entryNames'])
        assert all(not ('/' in v or chr(92) in v or ':' in v) for v in observed['entryNames'])
    print('Packaged combined inventory matches two OS-resolved Startup folder name snapshots. Legacy response unchanged.')

if __name__ == '__main__':
    run(pathlib.Path(sys.argv[1]).resolve(strict=True))
