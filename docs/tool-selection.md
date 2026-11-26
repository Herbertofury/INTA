# INTA Tool Selection Matrix

Tool candidates are not automatically organization-approved. Verify license, privacy, deployment, support, security and current vendor terms before use.

## Security and malware

| Job | Strong starting point | Alternatives / specialist | Notes |
|---|---|---|---|
| Managed endpoint protection | Microsoft Defender / Defender for Endpoint where licensed | Existing managed EDR | Prefer centrally managed protection. |
| On-demand second opinion | Microsoft Safety Scanner (MSERT) | Malwarebytes scanner, ESET Online Scanner | Vendor-direct download; verify current terms. |
| Offline scan | Microsoft Defender Offline | Trusted rescue media | Check BitLocker recovery and incident approval first. |
| File triage | Defender, approved sandbox, YARA/YARA-X | PEStudio | Do not upload client files to public scanners without permission. |
| Process/memory inspection | EDR investigation tools | System Informer, PE-sieve, HollowsHunter | Specialist; preserve evidence before action. |
| Forensic analysis | INTA, Sleuth Kit, Autopsy | FTK Imager | Chain of custody and write-blocking may apply. |

Official sources: [Defender](https://learn.microsoft.com/defender/), [MSERT](https://learn.microsoft.com/defender-endpoint/safety-scanner-download), [Defender Offline](https://learn.microsoft.com/defender-endpoint/microsoft-defender-offline), [Malwarebytes](https://www.malwarebytes.com/), [ESET](https://www.eset.com/int/home/online-scanner/), [YARA](https://virustotal.github.io/yara/), [YARA-X](https://virustotal.github.io/yara-x/).

## Remote support and RMM

| Scenario | Candidate | Trade-off |
|---|---|---|
| One-time attended Windows help | [Quick Assist](https://support.microsoft.com/windows/use-quick-assist-to-help-users) | User participation; not fleet RMM. |
| Commercial remote support | [TeamViewer](https://www.teamviewer.com/), [AnyDesk](https://anydesk.com/) | Tenant, licensing, MFA, unattended access and audit review. |
| Self-hosted remote access | [RustDesk](https://rustdesk.com/) | INTA owns hardening, relay security, updates and monitoring. |
| Fleet configuration | [Microsoft Intune](https://learn.microsoft.com/mem/intune/) | Licensing, tenant and enrollment requirements. |
| Self-hosted device management | [MeshCentral](https://meshcentral.com/) | Requires secure hosting, patching and access governance. |
| RMM automation | [Tactical RMM](https://github.com/amidaware/tacticalrmm) | High privilege; conduct formal security and supply-chain review. |
| Windows administration | PowerShell Remoting / [Windows Admin Center](https://learn.microsoft.com/windows-server/manage/windows-admin-center/overview) | Secure identity, transport, firewall scope and audit required. |

## Storage and recovery

| Job | Tool | Notes |
|---|---|---|
| SMART / disk health | [smartmontools](https://www.smartmontools.org/), [GSmartControl](https://gsmartcontrol.shaduri.dev/) | Health status is not a guarantee. |
| Imaging | [Clonezilla](https://clonezilla.org/), [Partclone](https://partclone.org/) | Verify source/destination; destination is overwritten. |
| Failing media | [GNU ddrescue](https://www.gnu.org/software/ddrescue/) | Image to healthy storage; mapfile supports resumption. |
| Partition/file recovery | [TestDisk / PhotoRec](https://www.cgsecurity.org/) | Recover to a different disk. |
| Forensics | [Sleuth Kit](https://www.sleuthkit.org/), [Autopsy](https://www.autopsy.com/) | Specialist workflow. |

## Hardware and performance

| Job | Tool | Notes |
|---|---|---|
| Sensors and inventory | [HWiNFO](https://www.hwinfo.com/), inxi, Windows CIM | Compare measurements to exact vendor limits. |
| Windows SMART view | [CrystalDiskInfo](https://crystalmark.info/en/software/crystaldiskinfo/) | Official builds only. |
| Memory testing | [MemTest86](https://www.memtest86.com/), Windows Memory Diagnostic | Run sufficient passes. |
| Stability testing | [OCCT](https://www.ocbase.com/), vendor diagnostics | Stress tests create heat/load; monitor and stop if unsafe. |
| Performance tracing | WPR/WPA, PerfMon, [Process Monitor](https://learn.microsoft.com/sysinternals/downloads/procmon) | Traces may contain sensitive paths and process data. |

## Networking and deployment

- [Wireshark](https://www.wireshark.org/) / [tcpdump](https://www.tcpdump.org/): packet capture; obtain authorization and protect captures.
- [Nmap](https://nmap.org/): scan only hosts/networks explicitly in scope.
- [WinGet](https://learn.microsoft.com/windows/package-manager/winget/): verify package identity, publisher and source.
- [Windows Autopilot](https://learn.microsoft.com/autopilot/) / [Intune](https://learn.microsoft.com/mem/intune/): managed provisioning where licensed.
- [Ventoy](https://www.ventoy.net/) / [Rufus](https://rufus.ie/): bootable USB; writing an image can erase the selected device.

## Approval checklist

Record publisher, canonical download, license/cost, supported OS/architecture, drivers/services/persistence, privilege needs, uninstall path, update channel, signature/hash support, telemetry/cloud processing, file-upload behavior, MFA/RBAC/audit, offline support, configuration/data paths, backup/restore and known conflicts.

Portable does not mean safe: a portable executable can still access user data, make network calls or require elevation.
