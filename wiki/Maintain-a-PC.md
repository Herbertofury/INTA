[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Service map](https://github.com/Herbertofury/INTA/wiki/Service-Map) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

# 🛠️ Maintain a PC

**Agree a maintenance window and confirm the backup can restore files before changing software.**

<a id="tool-choices"></a>

## 🧰 Choose the tool for this job

Choose by the task shown in the first column. Each option includes its acquisition route and access boundary.

| Need | Official tool + exact action | Why this choice / access |
| --- | --- | --- |
| **Review app updates** | **[WinGet](https://learn.microsoft.com/en-us/windows/package-manager/winget/)** · **[Download App Installer · MSIXBUNDLE](https://github.com/microsoft/winget-cli/releases/download/v1.29.380/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle)** | Review package identity, source and available upgrades before approved changes. **Access:** Included App Installer; each app keeps its own license. |
| **Review storage health** | **[CrystalDiskInfo](https://crystalmark.info/en/software/crystaldiskinfo/)** · **[Get Windows ZIP · 9.9.5](https://sourceforge.net/projects/crystaldiskinfo/files/9.9.5/CrystalDiskInfo9_9_5.zip/download)** | Check supported health indicators and route new warnings to recovery before cleanup. **Access:** Free and open source |
| **Explain capacity use** | **[WinDirStat](https://windirstat.net/)** · **[Download Windows x64 · MSI](https://github.com/windirstat/windirstat/releases/download/release/v2.9.3/WinDirStat-x64.msi)** | Identify where space went on healthy storage without assuming large files are disposable. **Access:** Free and open source |

**[Current selection evidence](https://github.com/Herbertofury/INTA/wiki/Current-Tool-Choices)**

<a id="working-kit"></a>

## 🧰 Tools and links for this job

**[Open working view](https://herbertofury.github.io/INTA/#job/preventive-maintenance/quick/maintain-a-pc)** · Keep the tool panel beside the instructions.

**First move:** Agree a maintenance window and confirm the backup can restore files before changing software.

| Use this for | Official tool + exact action |
| --- | --- |
| Maintain a PC | **[WinGet](https://learn.microsoft.com/en-us/windows/package-manager/winget/)** · [Official page](https://learn.microsoft.com/en-us/windows/package-manager/winget/) · **[Download App Installer · MSIXBUNDLE](https://github.com/microsoft/winget-cli/releases/download/v1.29.380/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle)** |
| Review storage warnings before cleanup. | **[CrystalDiskInfo](https://crystalmark.info/en/software/crystaldiskinfo/)** · [Official page](https://crystalmark.info/en/software/crystaldiskinfo/) · **[Get Windows ZIP · 9.9.5](https://sourceforge.net/projects/crystaldiskinfo/files/9.9.5/CrystalDiskInfo9_9_5.zip/download)** |
| Find capacity use on a healthy drive. | **[WinDirStat](https://windirstat.net/)** · [Official page](https://windirstat.net/) · **[Download Windows x64 · MSI](https://github.com/windirstat/windirstat/releases/download/release/v2.9.3/WinDirStat-x64.msi)** |
| Locate a named local file or log without unnecessary cleanup. | **[Everything](https://www.voidtools.com/)** · [Official page](https://www.voidtools.com/) · **[Download Windows x64 EXE](https://www.voidtools.com/Everything-1.4.1.1032.x64-Setup.exe)** |
| Collecting battery and sleep evidence before changing hardware or power policy. | **[powercfg](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/powercfg-command-line-options)** · [Official page](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/powercfg-command-line-options) · **[Open built-in battery and sleep diagnostics](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/powercfg-command-line-options)** |
| Establishing recoverable, per-device administrator access during managed workstation setup and support planning. | **[Windows LAPS](https://learn.microsoft.com/en-us/windows-server/identity/laps/laps-overview)** · [Official page](https://learn.microsoft.com/en-us/windows-server/identity/laps/laps-overview) · **[Configure built-in administrator password management](https://learn.microsoft.com/en-us/windows-server/identity/laps/laps-overview)** |
| Use existing endpoint telemetry for investigation; plan new deployment after active evidence-preservation needs are resolved. | **[Sysmon](https://learn.microsoft.com/en-us/sysinternals/downloads/sysmon)** · [Official page](https://learn.microsoft.com/en-us/sysinternals/downloads/sysmon) · **[Download Windows monitoring tools · ZIP](https://download.sysinternals.com/files/Sysmon.zip)** |
| Use the supported file-search workflow when its filters and the technician’s license fit the task. | **[WizFile](https://antibody-software.com/wizfile/)** · [Official page](https://antibody-software.com/wizfile/) · **[Download Windows portable · ZIP](https://antibody-software.com/files/wizfile_3_15_portable.zip)** |

### Updates

| Open the exact resource | What to do there |
| --- | --- |
| **[Review WinGet upgrades](https://learn.microsoft.com/en-us/windows/package-manager/winget/upgrade)** | List available changes and select approved packages. |

### Recovery

| Open the exact resource | What to do there |
| --- | --- |
| **[Test the existing backup](https://github.com/Herbertofury/INTA/wiki/Job-backup-restore)** | Restore a representative item to a separate folder. |

### Find retained files

| Open the exact resource | What to do there |
| --- | --- |
| **[Everything desktop search guidance](https://www.voidtools.com/support/everything/)** | Limit indexing/search to approved local locations. |

### Handoff

| Open the exact resource | What to do there |
| --- | --- |
| **[Record changes and deferred work](https://github.com/Herbertofury/INTA/wiki/Technician-Handoff)** | Capture the observed result and follow-up owner. |

## ⚡ First three actions

1. Check storage health, free space, protection status and a representative backup restore.
2. Review available [Windows](https://www.microsoft.com/windows) and application updates; use [WinGet](https://learn.microsoft.com/en-us/windows/package-manager/winget/) only for approved package changes.
3. Reboot in the agreed window, retest essential tasks and record the versions and remaining issues.

**[WinGet](https://learn.microsoft.com/en-us/windows/package-manager/winget/)** · **[Download App Installer · MSIXBUNDLE](https://github.com/microsoft/winget-cli/releases/download/v1.29.380/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle)**

**[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-preventive-maintenance)** · **[Specific fixes for this job](https://herbertofury.github.io/INTA/#job/preventive-maintenance/quick/maintain-a-pc?section=fixes)** · **[Selection evidence](https://github.com/Herbertofury/INTA/wiki/Free-Tool-Picks)**

<a id="next-action"></a>

## 🧭 Match the symptom

| You see this | Do this next | You should get |
| --- | --- | --- |
| **The device has recurring failures or storage warnings** | Check [Reliability Monitor](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/perfmon) and drive health; move actual faults to their repair or recovery guide before routine maintenance. | The maintenance plan is based on observed faults rather than cleanup rituals. |
| **Application updates are available** | Run [winget](https://learn.microsoft.com/en-us/windows/package-manager/winget/) upgrade to list them, review compatibility and update only the approved packages in the maintenance window. | The selected versions are installed and any planned restart is completed. |
| **A full disk or missing local file is the concern** | Use [WinDirStat](https://windirstat.net/) for capacity or [Everything](https://www.voidtools.com/) for a filename; confirm ownership and retention before deleting or moving anything. | The relevant path is identified and any approved cleanup has a measured result. |
| **Backup jobs report success** | Restore a representative item to a separate location and open it with the intended application. | The backup is demonstrably usable, with its recovery point recorded. |
| **Changes are complete** | Restart as planned and repeat the user’s main app, network, printing and backup checks. | Normal work succeeds and all deferred issues have owners. |

### ✅ Finish the job

- [ ] Protection, hardware health and backup restoration have been checked.
- [ ] The normal workload works after maintenance and restart.
- [ ] Each unresolved item has an explicit next action and owner.

**Change route when:** Storage failure or active compromise changes the service route. · An unplanned critical workload would be interrupted.


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
