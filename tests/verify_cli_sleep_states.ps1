param([Parameter(Mandatory)][string]$Agent)
$ErrorActionPreference = 'Stop'
Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;

public static class SleepCapsReference {
    [DllImport("powrprof.dll", ExactSpelling = true)]
    [DefaultDllImportSearchPaths(DllImportSearchPath.System32)]
    private static extern byte GetPwrCapabilities([Out] byte[] capabilities);

    public static bool[] Read() {
        var bytes = new byte[256];
        if (GetPwrCapabilities(bytes) == 0) throw new Exception("Power capability reference unavailable");
        return new bool[] {bytes[3] != 0, bytes[4] != 0, bytes[5] != 0, bytes[6] != 0, bytes[8] != 0};
    }
}
'@
$agentFile = (Resolve-Path -LiteralPath $Agent).Path
$pin = (Get-FileHash -Algorithm SHA256 -LiteralPath $agentFile).Hash.ToLowerInvariant()
$power = Join-Path $env:SystemRoot 'System32\powercfg.exe'
$activeBefore = (& $power /getactivescheme) -join "`n"
if ($LASTEXITCODE -ne 0) { throw 'Independent power plan read failed' }
$before = [SleepCapsReference]::Read()
$result = python -B scripts/cli-anything-inta.py --json sleep-states --agent-binary $agentFile --agent-sha256 $pin
if ($LASTEXITCODE -ne 0) { throw 'Native CLI reply not ready' }
$snapshot = $result | ConvertFrom-Json
if ($snapshot.schema -ne 1 -or $snapshot.operation -ne 'sleep-states' -or $snapshot.status -ne 'ready' -or -not $snapshot.readOnly) {
    throw 'Unexpected native sleep capability schema'
}
$actual = @($snapshot.states.standbyS1, $snapshot.states.standbyS2, $snapshot.states.standbyS3,
            $snapshot.states.hibernateS4, $snapshot.states.hibernationFilePresent)
$after = [SleepCapsReference]::Read()
$activeAfter = (& $power /getactivescheme) -join "`n"
if ($LASTEXITCODE -ne 0 -or $activeBefore -cne $activeAfter) { throw 'Active plan changed during the capability read' }
for ($i = 0; $i -lt 5; $i++) {
    if ($actual[$i] -isnot [bool] -or $actual[$i] -ne $before[$i] -or $actual[$i] -ne $after[$i]) {
        throw 'Native capability read did not match the independent documented query'
    }
}
Write-Host 'Verified existing native capability flags against GetPwrCapabilities and unchanged active plan.'
