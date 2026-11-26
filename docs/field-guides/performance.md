# Performance & Stability Troubleshooting

Ask when the issue started, whether constant/intermittent, whether one user or many, and what changed. Classify symptom: startup, app launch, browser, network, disk, thermal throttling, freezes or crashes.

## Read-only checks
```powershell
Get-Process | Sort-Object WorkingSet64 -Descending |
  Select-Object -First 15 Name, Id, @{n='RAM_MB';e={[math]::Round($_.WorkingSet64/1MB,1)}}
Get-Process | Sort-Object CPU -Descending | Select-Object -First 15 Name, Id, CPU
Get-CimInstance Win32_LogicalDisk -Filter 'DriveType=3' |
  Select-Object DeviceID, @{n='FreeGB';e={[math]::Round($_.FreeSpace/1GB,1)}}
Get-WinEvent -FilterHashtable @{LogName='System';Level=1,2;StartTime=(Get-Date).AddDays(-2)} -MaxEvents 50
Get-WinEvent -FilterHashtable @{LogName='Application';Level=1,2;StartTime=(Get-Date).AddDays(-2)} -MaxEvents 50
```

## Use the right instrument
- Task Manager: quick CPU, memory, disk, network and startup view.
- Resource Monitor: process-level disk/network/CPU correlation.
- Reliability Monitor (perfmon /rel): crashes, updates and driver changes over time.
- Performance Monitor: repeatable counter baselines.
- Windows Performance Recorder/Analyzer: boot, CPU, disk and ETW tracing.
- Process Monitor: file/registry/process activity; filter to avoid huge traces.
- HWiNFO: temperature, clocks, power and throttling.

Common causes include low RAM, failing storage, thermal throttling, runaway processes, browser extensions, startup load, low disk space, driver regressions, security scans, updates and network latency. Compare baseline and symptom windows. Do not disable services, scheduled tasks, security agents or update components indiscriminately. Make one change at a time and keep rollback.
