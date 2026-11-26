# INTA tool catalog

The live-image column lists tools requested by the current build package manifest. Companion tools are external Windows/Linux downloads or services and are not silently bundled.

| Task | In the live image | Upstream |
| --- | --- | --- |
| Read drive health | GSmartControl, smartmontools, NVMe CLI, hdparm | [GSmartControl](https://gsmartcontrol.shaduri.dev/), [NVMe CLI](https://github.com/linux-nvme/nvme-cli) |
| Clone, image and recover drives | Clonezilla, Partclone, GNU ddrescue, TestDisk, PhotoRec | [Clonezilla](https://clonezilla.org/), [ddrescue](https://www.gnu.org/software/ddrescue/), [CGSecurity](https://www.cgsecurity.org/) |
| Inspect file systems and volumes | GParted, NTFS-3G, exfatprogs, e2fsprogs, Btrfs tools, XFS tools, LVM, mdadm, cryptsetup, dislocker | [GParted](https://gparted.org/), [dislocker](https://github.com/Aorimn/dislocker) |
| Inspect Windows evidence offline | Sleuth Kit, hivex, libevtx utilities, YARA, ClamAV | [Sleuth Kit](https://www.sleuthkit.org/), [libevtx](https://github.com/libyal/libevtx), [YARA](https://virustotal.github.io/yara/), [ClamAV](https://docs.clamav.net/) |
| Inspect hardware and networking | inxi, lshw, dmidecode, lm-sensors, memtester, stress-ng, ethtool, tcpdump | [tcpdump](https://www.tcpdump.org/) |
| AI-assisted triage | Firefox, Codex CLI, Claude Code | [ChatGPT](https://chatgpt.com/), [Codex](https://github.com/openai/codex), [Claude Code](https://code.claude.com/docs/en/setup) |

## Windows security and endpoint tools

| Tool | Job | Status / caution |
| --- | --- | --- |
| [Microsoft Defender](https://learn.microsoft.com/defender/) | Built-in endpoint protection and managed EDR ecosystem | Prefer the organization's managed configuration. |
| [Microsoft Safety Scanner (MSERT)](https://learn.microsoft.com/defender-endpoint/safety-scanner-download) | On-demand second-opinion scan | Download a fresh copy from Microsoft; signature usefulness is time-limited. |
| [Microsoft Defender Offline](https://learn.microsoft.com/defender-endpoint/microsoft-defender-offline) | Offline scan outside normal Windows session | Check BitLocker recovery and incident approval. |
| [Malwarebytes](https://www.malwarebytes.com/) / [ESET Online Scanner](https://www.eset.com/int/home/online-scanner/) | Additional on-demand scan options | Review current license, policy and product behavior; vendor-direct only. |
| [YARA-X](https://virustotal.github.io/yara-x/) | Rule-based file triage | Validate rules and compatibility; not an antivirus replacement. |
| [PE-sieve](https://github.com/hasherezade/pe-sieve), [HollowsHunter](https://github.com/hasherezade/hollows_hunter) | Specialist process/memory inspection | Authorized Windows session; preserve evidence before action. |
| [System Informer](https://systeminformer.sourceforge.io/) | Process and system inspection | Specialist tool; verify official distribution and approval. |

## Remote support and management

| Tool | Use | Review |
| --- | --- | --- |
| [Windows Quick Assist](https://support.microsoft.com/windows/use-quick-assist-to-help-users) | Attended one-time support | User participation; not unattended fleet management. |
| [TeamViewer](https://www.teamviewer.com/) / [AnyDesk](https://anydesk.com/) | Commercial remote support | Tenant, license, MFA, unattended access, audit and privacy controls. |
| [RustDesk](https://rustdesk.com/) | Remote support with self-hosting options | Self-hosting requires secure relay/ID infrastructure and patching. |
| [MeshCentral](https://meshcentral.com/) | Self-hosted device management | Requires hardened hosting and access governance. |
| [Tactical RMM](https://github.com/amidaware/tacticalrmm) | RMM and automation | High-privilege platform; formal security and supply-chain review. |
| [Microsoft Intune](https://learn.microsoft.com/mem/intune/) | Managed endpoint deployment and configuration | Tenant and licensing requirements apply. |

## Companion tools

| Tool | Use |
| --- | --- |
| [Ventoy](https://www.ventoy.net/en/download.html) | Install/update Ventoy bootloader with upstream utility. INTA Media Creator handles verified ISO copy/refresh on mounted data volume. |
| [Rufus](https://rufus.ie/en/) | Write INTA ISO to USB from Windows; writing erases the selected device. |
| [Windows 11 installation media](https://www.microsoft.com/software-download/windows11) | Separate Windows Recovery Environment and Windows-native boot repairs using BCDBoot, DISM and SFC. |
| [Rescuezilla](https://rescuezilla.com/download) | Separate graphical recovery image for Clonezilla-compatible imaging. |
| [HWiNFO](https://www.hwinfo.com/) / [CrystalDiskInfo](https://crystalmark.info/en/software/crystaldiskinfo/) | Windows hardware, sensor and SMART diagnostics. |
| [MemTest86](https://www.memtest86.com/) | Bootable memory diagnostics. |
| [Wireshark](https://www.wireshark.org/) / [Nmap](https://nmap.org/) | Authorized packet capture and network discovery only. |

## Security and sourcing

Use official publisher/project download pages. Verify signatures and hashes when provided. Evaluate licensing, commercial-use rights, supported OS versions, update cadence, telemetry, cloud processing, drivers/services, privilege, persistence and uninstall behavior before approval. Portable does not mean safe. Never upload client files or packet captures to public services without authorization.

INTA uses Debian authenticated repositories for base system packages. See [the package list](../live/package-lists/inta.list.chroot) for the actual live-image request set and the release manifest for resolved packages.

## Field manuals

- [INTA Computer Services wiki home](index.md)
- [PowerShell field cookbook](powershell-cookbook.md)
- [Security response](security-response.md)
- [Remote support](remote-support.md)
- [Tool selection matrix](tool-selection.md)
- [Athena Windows endpoint security research](https://github.com/Herbertofury/Athena/blob/c976a3d35c507e4fe94aefafc3db6f6f97e8eb8e/docs/windows-endpoint-security.md)
