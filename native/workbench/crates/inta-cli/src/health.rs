use anyhow::{Result, bail};
use serde_json::Value;

// Only diagnostic properties enter the report. The device inventory stays aggregated.
const DRIVERS: &str = r#"
$devices = @(Get-CimInstance Win32_PnPEntity -Property Present,ConfigManagerErrorCode)
$drivers = @(Get-CimInstance Win32_PnPSignedDriver -Property IsSigned)
$present = @($devices | Where-Object { $_.Present -eq $true })
$problems = @($present | Where-Object { $null -ne $_.ConfigManagerErrorCode -and $_.ConfigManagerErrorCode -ne 0 })
$signed = @($drivers | Where-Object { $_.IsSigned -eq $true }).Count
$unsigned = @($drivers | Where-Object { $_.IsSigned -eq $false }).Count
[pscustomobject]@{
    source='Plug and Play inventory'; presentDevices=$present.Count
    unknownPresence=@($devices | Where-Object { $null -eq $_.Present }).Count
    unknownProblemStatus=@($present | Where-Object { $null -eq $_.ConfigManagerErrorCode }).Count
    problemDevices=$problems.Count; driverRecords=$drivers.Count
    reportedSigned=$signed; reportedUnsigned=$unsigned; unknownSignature=$drivers.Count-$signed-$unsigned
}
$problems | Group-Object ConfigManagerErrorCode | Sort-Object { [int]$_.Name } | ForEach-Object {
    [pscustomobject]@{source='Device problem code'; code=[int]$_.Name; count=$_.Count}
}
"#;

const EVENTS: &str = r#"
$filter = '*[System[(Level=1 or Level=2) and TimeCreated[timediff(@SystemTime) >= 0 and timediff(@SystemTime) <= 86400000]]]'
$query = [System.Diagnostics.Eventing.Reader.EventLogQuery]::new('System',[System.Diagnostics.Eventing.Reader.PathType]::LogName,$filter)
$query.ReverseDirection = $true
$reader = [System.Diagnostics.Eventing.Reader.EventLogReader]::new($query)
$counts = @{}
$count = 0
$capped = $false
try {
    while ($null -ne ($event = $reader.ReadEvent())) {
        try {
            if ($count -ge 1000) { $capped = $true; break }
            $provider = [string]$event.ProviderName
            $eventId = $event.Id
            $key = "$provider/$eventId"
            if (!$counts.ContainsKey($key)) {
                $counts[$key] = [pscustomobject]@{source='Event group'; provider=$provider; eventId=$eventId; critical=0; errors=0}
            }
            if ($event.Level -eq 1) { $counts[$key].critical++ } else { $counts[$key].errors++ }
            $count++
        } finally { $event.Dispose() }
    }
} finally { $reader.Dispose() }
[pscustomobject]@{source='System event log'; lookbackHours=24; limit=1000; sampled=$count; truncated=$capped}
$counts.Values | Sort-Object provider,eventId
"#;

const PERFORMANCE: &str = r#"
$cpu = Get-CimInstance Win32_PerfFormattedData_PerfOS_Processor -Filter "Name='_Total'" -Property PercentProcessorTime
$system = Get-CimInstance Win32_PerfFormattedData_PerfOS_System -Property Processes
$os = Get-CimInstance Win32_OperatingSystem -Property TotalVisibleMemorySize,FreePhysicalMemory
if ($null -eq $cpu.PercentProcessorTime -or $cpu.PercentProcessorTime -gt 100 -or $null -eq $system.Processes -or $system.Processes -lt 1 -or $null -eq $os.FreePhysicalMemory -or $os.TotalVisibleMemorySize -lt 1 -or $os.FreePhysicalMemory -gt $os.TotalVisibleMemorySize) {
    throw 'Performance counters unavailable'
}
[pscustomobject]@{
    source='Windows performance counters'; cpuBusyPercent=[double]$cpu.PercentProcessorTime
    processes=[long]$system.Processes; memoryTotalMiB=[math]::Round($os.TotalVisibleMemorySize/1024,1)
    memoryAvailableMiB=[math]::Round($os.FreePhysicalMemory/1024,1)
    memoryUsedPercent=[math]::Round(100 * (1-$os.FreePhysicalMemory/[double]$os.TotalVisibleMemorySize),1)
}
"#;

pub fn inspect(kind: &str) -> Result<Value> {
    let query = match kind {
        "drivers" => DRIVERS,
        "events" => EVENTS,
        "performance" => PERFORMANCE,
        _ => bail!("Unknown health inspection"),
    };
    crate::inspect::run_query(kind, query)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unregistered_operation_is_rejected_before_provider_execution() {
        assert_eq!(
            inspect("events; exit").unwrap_err().to_string(),
            "Unknown health inspection"
        );
    }

    #[test]
    #[cfg(windows)]
    fn unavailable_provider_returns_a_sanitized_error() {
        let failure =
            crate::inspect::run_query("performance", "throw 'sensitive fixture diagnostic'")
                .unwrap_err()
                .to_string();
        assert!(failure.contains("provider is unavailable"));
        assert!(!failure.contains("sensitive fixture"));
    }
}
