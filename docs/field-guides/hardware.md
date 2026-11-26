# Hardware Diagnostics

## Intake and baseline
Record model, asset identity in the approved ticketing system, BIOS version, warranty status, symptoms and recent impact. Confirm backup status and authorization to open the device. Follow ESD and battery safety procedures.

## Recommended sequence
1. Visual inspection: damage, liquid, dust, blocked vents, fan noise or swollen battery. Do not handle a damaged/swollen battery; escalate.
2. Check adapter, cable, dock, display, ports and peripherals.
3. Run vendor BIOS/UEFI diagnostics for the exact model.
4. Capture read-only OS inventory and event logs.
5. Inspect SMART/NVMe health, memory and thermal behavior.
6. Run controlled stress tests only when needed and safe.
7. Record findings, replace confirmed faulty components and repeat tests.

## Windows inventory
```powershell
Get-CimInstance Win32_ComputerSystem | Select-Object Manufacturer, Model, TotalPhysicalMemory
Get-CimInstance Win32_BIOS | Select-Object SMBIOSBIOSVersion, ReleaseDate
Get-CimInstance Win32_Processor | Select-Object Name, NumberOfCores, NumberOfLogicalProcessors
Get-CimInstance Win32_PhysicalMemory | Select-Object Capacity, Speed, Manufacturer, PartNumber
Get-PhysicalDisk | Select-Object FriendlyName, MediaType, HealthStatus, OperationalStatus, Size
Get-CimInstance Win32_VideoController | Select-Object Name, DriverVersion
```

## Tool choices
- [HWiNFO](https://www.hwinfo.com/): detailed sensor and hardware inventory.
- [CrystalDiskInfo](https://crystalmark.info/en/software/crystaldiskinfo/): Windows SMART view.
- [smartmontools](https://www.smartmontools.org/): cross-platform drive diagnostics.
- [MemTest86](https://www.memtest86.com/): bootable memory testing.
- Vendor diagnostics: select by exact Dell, HP, Lenovo or component model.

SMART "Good" does not guarantee future reliability. Stress tests create heat/load; monitor and stop if unsafe. Do not update firmware merely because a newer version exists: check vendor advisories, power stability, BitLocker/TPM recovery and approved rollback plan.
