# PowerShell Field Cookbook

Read-only discovery first. Run elevated only when required. Commands vary by Windows edition, policy and PowerShell version. Review outputs for sensitive information before sharing.

## 1. Create a local diagnostic snapshot

```powershell
$stamp = Get-Date -Format 'yyyyMMdd-HHmmss'
$case = Join-Path $env:USERPROFILE "Desktop\INTA-Diagnostics-$stamp"
New-Item -ItemType Directory -Path $case -Force | Out-Null

Get-ComputerInfo |
  Select-Object CsName, WindowsProductName, WindowsVersion, OsBuildNumber,
                OsArchitecture, CsManufacturer, CsModel |
  Format-List | Out-File "$case\system.txt" -Width 240
Get-CimInstance Win32_OperatingSystem |
  Select-Object Caption, Version, BuildNumber, LastBootUpTime |
  Format-List | Out-File "$case\os.txt"
Get-Volume |
  Select-Object DriveLetter, FileSystemLabel, FileSystem, HealthStatus,
                OperationalStatus, Size, SizeRemaining |
  Format-Table -AutoSize | Out-File "$case\volumes.txt" -Width 240
Get-NetIPConfiguration | Format-List * |
  Out-File "$case\network.txt" -Width 240
Write-Host "Diagnostics saved to $case"
```

Inspect the folder for usernames, hostnames, IP addresses and sensitive configuration before transferring it.

## 2. Hardware, drivers and devices

```powershell
Get-CimInstance Win32_ComputerSystem | Select-Object Manufacturer, Model, TotalPhysicalMemory
Get-CimInstance Win32_BIOS | Select-Object SMBIOSBIOSVersion, ReleaseDate
Get-CimInstance Win32_Processor | Select-Object Name, NumberOfCores, NumberOfLogicalProcessors
Get-CimInstance Win32_PhysicalMemory | Select-Object Manufacturer, Capacity, Speed, PartNumber
Get-CimInstance Win32_VideoController | Select-Object Name, DriverVersion
Get-CimInstance Win32_PnPSignedDriver |
  Select-Object DeviceName, DriverVersion, DriverDate, Manufacturer, IsSigned
Get-CimInstance Win32_PnPEntity |
  Where-Object ConfigManagerErrorCode -ne 0 |
  Select-Object Name, PNPDeviceID, ConfigManagerErrorCode
```

Use the exact model/hardware ID and vendor support page to select drivers. Avoid third-party driver bundle sites.

## 3. Disk and volume health

```powershell
Get-PhysicalDisk | Select-Object FriendlyName, MediaType, HealthStatus, OperationalStatus, Size
Get-Disk | Format-Table Number, FriendlyName, PartitionStyle, OperationalStatus, HealthStatus, Size
Get-Partition | Format-Table DiskNumber, PartitionNumber, DriveLetter, Type, Size
Get-Volume | Format-Table DriveLetter, FileSystemLabel, FileSystem, HealthStatus, Size, SizeRemaining
Get-CimInstance Win32_LogicalDisk -Filter 'DriveType=3' |
  Select-Object DeviceID,
    @{n='FreeGB';e={[math]::Round($_.FreeSpace/1GB,1)}},
    @{n='TotalGB';e={[math]::Round($_.Size/1GB,1)}}
```

SMART health is not a guarantee. Clicking, disconnecting or media errors mean stop unnecessary reads/writes and follow the recovery workflow. Do not run CHKDSK repair on a failing source before recovery assessment.

## 4. Processes, services and startup

```powershell
Get-Process | Sort-Object WorkingSet64 -Descending |
  Select-Object -First 20 Name, Id, @{n='RAM_MB';e={[math]::Round($_.WorkingSet64/1MB,1)}}
Get-Process | Sort-Object CPU -Descending | Select-Object -First 20 Name, Id, CPU
Get-CimInstance Win32_Service |
  Where-Object { $_.StartMode -eq 'Auto' -and $_.State -ne 'Running' } |
  Select-Object Name, DisplayName, State, StartMode
Get-CimInstance Win32_StartupCommand | Select-Object Name, Command, Location, User
```

Use Task Manager, Resource Monitor and Performance Monitor for time-series evidence; one snapshot does not establish root cause.

## 5. Network, DNS, proxy

```powershell
Get-NetAdapter | Format-Table Name, Status, LinkSpeed
Get-NetIPConfiguration
Get-NetRoute -AddressFamily IPv4 | Sort-Object RouteMetric
Get-DnsClientServerAddress -AddressFamily IPv4
Resolve-DnsName example.com
Test-Connection 1.1.1.1 -Count 4
Test-NetConnection example.com -Port 443 -InformationLevel Detailed
netsh winhttp show proxy
Get-NetConnectionProfile
```

If IP works but DNS fails, investigate name resolution. If DNS works but a TCP test fails, investigate routing, firewall, proxy and service availability. Avoid a full network reset as a first step.

## 6. Event logs and reliability

```powershell
Get-WinEvent -FilterHashtable @{
  LogName='System'; Level=1,2; StartTime=(Get-Date).AddDays(-3)
} -MaxEvents 100 |
  Select-Object TimeCreated, Id, ProviderName, LevelDisplayName, Message
Get-WinEvent -FilterHashtable @{
  LogName='Application'; Level=1,2; StartTime=(Get-Date).AddDays(-3)
} -MaxEvents 100 |
  Select-Object TimeCreated, Id, ProviderName, Message
perfmon /rel
wevtutil epl System "$env:USERPROFILE\Desktop\System.evtx"
```

Kernel-Power 41 records an unexpected shutdown; it is not by itself the root cause. Preserve logs before clearing them.

## 7. Defender status and scan

```powershell
Get-Service WinDefend, WdNisSvc -ErrorAction SilentlyContinue
Get-MpComputerStatus |
  Select-Object AntivirusEnabled, RealTimeProtectionEnabled, BehaviorMonitorEnabled,
                AntivirusSignatureVersion, AntivirusSignatureLastUpdated
Update-MpSignature
Start-MpScan -ScanType QuickScan
# Full scan, potentially long-running:
Start-MpScan -ScanType FullScan
```

For active compromise, use the security incident playbook first. Do not disable Defender or tamper protection to accommodate another scanner.

## 8. Windows repair (changes system files)

Run elevated, after checking backups and recording the original error:

```powershell
DISM.exe /Online /Cleanup-Image /CheckHealth
DISM.exe /Online /Cleanup-Image /ScanHealth
# Repair component store:
DISM.exe /Online /Cleanup-Image /RestoreHealth
# Verify/repair protected system files:
sfc.exe /scannow
```

Use matching official Windows media if repair source files are needed. Do not use random ISO files or mismatched builds.

## 9. Updates and installed software inventory

```powershell
Get-HotFix | Sort-Object InstalledOn -Descending | Select-Object -First 30
$uninstall = @(
 'HKLM:\Software\Microsoft\Windows\CurrentVersion\Uninstall\*',
 'HKLM:\Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\*',
 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\*'
)
Get-ItemProperty $uninstall -ErrorAction SilentlyContinue |
  Where-Object DisplayName |
  Select-Object DisplayName, DisplayVersion, Publisher, InstallDate |
  Sort-Object DisplayName
winget --version
winget list
```

Follow organizational patch rings and policy. Verify package ID, publisher, source and installer behavior before deployment.

## 10. BitLocker and backup checks

```powershell
Get-BitLockerVolume |
  Select-Object MountPoint, VolumeType, ProtectionStatus, LockStatus, EncryptionMethod
wbadmin get versions
```

Never export recovery keys into a ticket, chat, diagnostic bundle or this repository. Confirm approved recovery escrow before firmware, TPM, boot or partition changes.

## 11. Targeted actions — only after diagnosis

```powershell
# Clears the local DNS resolver cache:
Clear-DnsClientCache

# DHCP renewal; briefly interrupts network connectivity:
ipconfig /release
ipconfig /renew

# Winsock reset; change operation, may require reboot and affect VPN/security agents:
netsh winsock reset

# Restarts spooler and may interrupt print jobs:
Restart-Service Spooler
```

Do not run every command as a generic fix. Check with endpoint/VPN/domain owners before network-stack changes.

## 12. Hash collected files

```powershell
Get-ChildItem $case -File -Recurse |
  Get-FileHash -Algorithm SHA256 |
  Export-Csv "$case\SHA256-manifest.csv" -NoTypeInformation -Encoding utf8
```

Review for personal data, tokens and internal network details before transfer. Use approved evidence storage.

## Official references

- [PowerShell](https://learn.microsoft.com/powershell/)
- [DISM](https://learn.microsoft.com/windows-hardware/manufacture/desktop/what-is-dism)
- [SFC](https://support.microsoft.com/windows/use-the-system-file-checker-tool-to-repair-missing-or-corrupted-system-files)
- [Defender PowerShell module](https://learn.microsoft.com/powershell/module/defender/)
- [Get-WinEvent](https://learn.microsoft.com/powershell/module/microsoft.powershell.diagnostics/get-winevent)
