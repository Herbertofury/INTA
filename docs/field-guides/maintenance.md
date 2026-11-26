# Endpoint Maintenance

## Routine cadence
- **Each service visit:** reported issue, free space, EDR health, encryption, update state and backup status.
- **Weekly / managed cadence:** patch compliance, failed updates, endpoint alerts and backup failures in approved consoles.
- **Monthly:** OS/app/firmware updates, vulnerability advisories, stale accounts/devices, recovery readiness and exceptions.
- **Quarterly:** restore procedures, remote-access roles, local admin membership, asset inventory and documentation.

Use organizational patch rings, maintenance windows and approvals. Do not override centrally managed policy independently.

## Read-only checks
```powershell
Get-HotFix | Sort-Object InstalledOn -Descending | Select-Object -First 20
Get-Service WinDefend, wuauserv, bits -ErrorAction SilentlyContinue
Get-MpComputerStatus | Select-Object AntivirusEnabled, RealTimeProtectionEnabled, AntivirusSignatureLastUpdated
Get-BitLockerVolume | Select-Object MountPoint, ProtectionStatus, LockStatus
Get-Volume | Select-Object DriveLetter, HealthStatus, SizeRemaining
winget list
```

## Software updates
Prefer official vendor channels, WinGet/Microsoft Store where package identity is verified, and enterprise management. Verify publisher, package ID, architecture, release notes and installer behavior. Pilot updates; check compatibility and rollback. Avoid generic driver/app updater scripts of unknown provenance.

## Firmware
Confirm exact model/revision, vendor advisory, power stability, BitLocker/TPM recovery, maintenance window and recovery method. Do not interrupt firmware updates.

## Backups
A backup is only useful if it can be restored. Check success, scope, retention, encryption, off-device/offline protection and periodic restore tests. Sync is not a complete backup.
