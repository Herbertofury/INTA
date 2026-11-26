# INTA tool catalog

| Task | In the live image | Upstream |
| --- | --- | --- |
| Read drive health | GSmartControl, smartmontools, NVMe CLI, hdparm | [GSmartControl](https://gsmartcontrol.shaduri.dev/), [NVMe CLI](https://github.com/linux-nvme/nvme-cli) |
| Clone, image and recover drives | Clonezilla, Partclone, GNU ddrescue, TestDisk, PhotoRec | [Clonezilla](https://clonezilla.org/), [ddrescue](https://www.gnu.org/software/ddrescue/), [CGSecurity](https://www.cgsecurity.org/) |
| Inspect file systems and volumes | GParted, NTFS-3G, exfatprogs, e2fsprogs, Btrfs tools, XFS tools, LVM, mdadm, cryptsetup, dislocker | [GParted](https://gparted.org/), [dislocker](https://github.com/Aorimn/dislocker) |
| Inspect Windows evidence offline | Sleuth Kit, hivex, libevtx utilities, YARA, ClamAV | [Sleuth Kit](https://www.sleuthkit.org/), [libevtx](https://github.com/libyal/libevtx), [YARA](https://virustotal.github.io/yara/), [ClamAV](https://docs.clamav.net/) |
| Inspect hardware and networking | inxi, lshw, dmidecode, lm-sensors, memtester, stress-ng, ethtool, tcpdump | [tcpdump](https://www.tcpdump.org/) |
| AI-assisted triage | Firefox with a ChatGPT launcher, Codex CLI, Claude Code | [ChatGPT](https://chatgpt.com/), [Codex](https://github.com/openai/codex), [Claude Code](https://code.claude.com/docs/en/setup) |

## Companion tools

| Tool | Use |
| --- | --- |
| [Ventoy](https://www.ventoy.net/en/download.html) | Install or update the Ventoy bootloader with its upstream utility. INTA Media Creator handles verified ISO copy and refresh on the mounted data volume. |
| [Rufus](https://rufus.ie/en/) | Write the downloaded INTA ISO directly to USB from Windows. |
| [Windows 11 installation media](https://www.microsoft.com/software-download/windows11) | Keep on a Ventoy volume as separate media for Windows Recovery Environment and Windows-native boot repairs using BCDBoot, DISM and SFC. |
| [Rescuezilla](https://rescuezilla.com/download) | Separate graphical recovery image for teams that prefer it for Clonezilla-compatible imaging. |
| [YARA-X](https://virustotal.github.io/yara-x/) | Athena follow-on candidate for defensive file-scanning rules. Verify rule compatibility before promoting it to the image. |
| [PE-sieve](https://github.com/hasherezade/pe-sieve), [HollowsHunter](https://github.com/hasherezade/hollows_hunter), [System Informer](https://systeminformer.sourceforge.io/) | Windows-native running-process inspection candidates from Athena endpoint-security research. Use these in an authorized Windows session. |

INTA uses Debian's authenticated package repositories for base system packages. See the package list for exactly what the build requests and the release manifest for resolved Debian packages.

Athena reference: [Windows endpoint security](https://github.com/Herbertofury/Athena/blob/c976a3d35c507e4fe94aefafc3db6f6f97e8eb8e/docs/windows-endpoint-security.md).
