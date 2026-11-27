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

if __name__ == '__main__':
    run(pathlib.Path(sys.argv[1]).resolve(strict=True))
