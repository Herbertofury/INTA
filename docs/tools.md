# INTA tool catalog

| Task | In the live image | Upstream |
| --- | --- | --- |
| Read drive health | [GSmartControl](https://gsmartcontrol.shaduri.dev/), [smartmontools](https://www.smartmontools.org/), NVMe CLI, hdparm | [GSmartControl](https://gsmartcontrol.shaduri.dev/), [NVMe CLI](https://github.com/linux-nvme/nvme-cli) |
| Clone, image and recover drives | [Clonezilla](https://clonezilla.org/), Partclone, [GNU ddrescue](https://www.gnu.org/software/ddrescue/), [TestDisk](https://www.cgsecurity.org/wiki/TestDisk), [PhotoRec](https://www.cgsecurity.org/wiki/PhotoRec) | [Clonezilla](https://clonezilla.org/), [ddrescue](https://www.gnu.org/software/ddrescue/), [CGSecurity](https://www.cgsecurity.org/) |
| Inspect file systems and volumes | [GParted](https://gparted.org/), NTFS-3G, exfatprogs, e2fsprogs, Btrfs tools, XFS tools, LVM, mdadm, [cryptsetup](https://gitlab.com/cryptsetup/cryptsetup), [dislocker](https://github.com/Aorimn/dislocker) | [GParted](https://gparted.org/), [dislocker](https://github.com/Aorimn/dislocker) |
| Inspect [Windows](https://www.microsoft.com/windows) evidence offline | Sleuth Kit, hivex, libevtx utilities, YARA, [ClamAV](https://www.clamav.net/) | [Sleuth Kit](https://www.sleuthkit.org/), [libevtx](https://github.com/libyal/libevtx), [YARA](https://virustotal.github.io/yara/), [ClamAV](https://docs.clamav.net/) |
| Inspect hardware and networking | inxi, lshw, dmidecode, lm-sensors, memtester, stress-ng, ethtool, [tcpdump](https://www.tcpdump.org/) | [tcpdump](https://www.tcpdump.org/) |
| [AI-assisted triage](https://github.com/Herbertofury/INTA/wiki/Job-ai-triage) | [Firefox](https://www.mozilla.org/firefox/) with a [ChatGPT](https://chatgpt.com/) launcher, [Codex CLI](https://github.com/openai/codex), [Claude Code](https://code.claude.com/docs/en/overview) | [ChatGPT](https://chatgpt.com/), [Codex](https://github.com/openai/codex), [Claude Code](https://code.claude.com/docs/en/setup) |

## Companion tools

| Tool | Use |
| --- | --- |
| [Ventoy](https://www.ventoy.net/en/download.html) | Install or update the [Ventoy](https://www.ventoy.net/) bootloader with its upstream utility. INTA [Media Creator](https://github.com/Herbertofury/INTA/blob/main/docs/rescue.md) handles verified ISO copy and refresh on the mounted data volume. |
| [Rufus](https://rufus.ie/en/) | Write the downloaded INTA ISO directly to USB from [Windows](https://www.microsoft.com/windows). |
| [Windows 11 installation media](https://www.microsoft.com/software-download/windows11) | Keep on a [Ventoy](https://www.ventoy.net/) volume as separate media for [Windows Recovery Environment](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference) and [Windows](https://www.microsoft.com/windows)-native boot repairs using [BCDBoot](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/bcdboot-command-line-options-techref-di), [DISM](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image) and [SFC](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc). |
| [Rescuezilla](https://rescuezilla.com/download) | Separate graphical recovery image for teams that prefer it for [Clonezilla](https://clonezilla.org/)-compatible imaging. |
| [YARA-X](https://virustotal.github.io/yara-x/) | [Athena](https://github.com/Herbertofury/Athena) follow-on candidate for defensive file-scanning rules. Verify rule compatibility before promoting it to the image. |
| [PE-sieve](https://github.com/hasherezade/pe-sieve), [HollowsHunter](https://github.com/hasherezade/hollows_hunter), [System Informer](https://systeminformer.sourceforge.io/) | [Windows](https://www.microsoft.com/windows)-native running-process inspection candidates from [Athena](https://github.com/Herbertofury/Athena) endpoint-security research. Use these in an authorized [Windows](https://www.microsoft.com/windows) session. |

INTA uses [Debian](https://www.debian.org/)'s authenticated package repositories for base system packages. See the package list for exactly what the build requests and the release manifest for resolved [Debian](https://www.debian.org/) packages.

[Athena](https://github.com/Herbertofury/Athena) reference: [Windows endpoint security](https://github.com/Herbertofury/Athena/blob/c976a3d35c507e4fe94aefafc3db6f6f97e8eb8e/docs/windows-endpoint-security.md).
