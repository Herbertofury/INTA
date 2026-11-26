# Windows Repair Workflow

## Before changes
Confirm symptoms and exact errors; check backup status, BitLocker recovery readiness and free space; capture OS build, recent updates, relevant event logs and Reliability Monitor history. Determine whether the endpoint is managed. If malware or hardware failure is suspected, use that workflow first.

## Repair ladder
1. Reproduce and isolate the symptom.
2. Check Windows Update, drivers, service state and event logs.
3. Use built-in troubleshooting or application repair.
4. Inspect component store health.
5. Repair component store if indicated.
6. Verify/repair protected system files.
7. Reboot if requested and repeat the original test.
8. Escalate to repair install/recovery/rebuild only after data and recovery planning.

## Commands (elevated; repair operations)
```powershell
DISM.exe /Online /Cleanup-Image /CheckHealth
DISM.exe /Online /Cleanup-Image /ScanHealth
# CHANGE: repair component store
DISM.exe /Online /Cleanup-Image /RestoreHealth
# CHANGE: verify and repair protected files
sfc.exe /scannow
```

If repair source files are required, use matching official Windows media and documented DISM source syntax. Do not use random ISO files or mismatched builds.

## Boot and recovery
Use Windows Recovery Environment and official installation media. Understand Startup Repair, System Restore, uninstall updates, Safe Mode, Reset this PC and reinstall; some actions remove apps/data. Confirm user consent and verified backup before proceeding.

Avoid unknown debloat/registry scripts, disabling security/update services, downloading DLLs from third-party sites, CHKDSK repair on failing media before recovery assessment, and reset/reinstall without recovery keys.

References: [DISM](https://learn.microsoft.com/windows-hardware/manufacture/desktop/what-is-dism), [SFC](https://support.microsoft.com/windows/use-the-system-file-checker-tool-to-repair-missing-or-corrupted-system-files), [Windows Recovery](https://support.microsoft.com/windows/recovery-options-in-windows).
