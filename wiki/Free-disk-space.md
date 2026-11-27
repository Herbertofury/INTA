[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Service map](https://github.com/Herbertofury/INTA/wiki/Service-Map) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

# 📊 Free disk space

**Confirm the disk is healthy and choose the drive to inspect before deleting any files.**

<a id="tool-choices"></a>

## 🧰 Choose the tool for this job

Choose by the task shown in the first column. Each option includes its acquisition route and access boundary.

| Need | Official tool + exact action | Why this choice / access |
| --- | --- | --- |
| **Free capacity map** | **[WinDirStat](https://windirstat.net/)** · **[Download Windows x64 · MSI](https://github.com/windirstat/windirstat/releases/download/release/v2.9.3/WinDirStat-x64.msi)** | Inspect directory and file-type usage on healthy storage before choosing a supported cleanup. **Access:** Free and open source |
| **Licensed capacity analysis** | **[WizTree](https://diskanalyzer.com/)** · **[Download Windows portable · ZIP](https://diskanalyzer.com/files/wiztree_4_34_portable.zip)** | Use its supported disk analysis when the technician has the appropriate commercial entitlement. **Access:** Personal use is free; business use requires a license. |

**[Current selection evidence](https://github.com/Herbertofury/INTA/wiki/Current-Tool-Choices)**

<a id="working-kit"></a>

## 🧰 Tools and links for this job

**[Open working view](https://herbertofury.github.io/INTA/#job/slow-pc/quick/disk-space)** · Keep the tool panel beside the instructions.

**First move:** Confirm the disk is healthy and choose the drive to inspect before deleting any files.

| Use this for | Official tool + exact action |
| --- | --- |
| Free disk space | **[WinDirStat](https://windirstat.net/)** · [Official page](https://windirstat.net/) · **[Download Windows x64 · MSI](https://github.com/windirstat/windirstat/releases/download/release/v2.9.3/WinDirStat-x64.msi)** |
| Identify the busy process and its owner. | **[Process Explorer](https://learn.microsoft.com/en-us/sysinternals/downloads/process-explorer)** · [Official page](https://learn.microsoft.com/en-us/sysinternals/downloads/process-explorer) · **[Download Windows portable · ZIP](https://download.sysinternals.com/files/ProcessExplorer.zip)** |
| Capture a short, filtered file or registry stall. | **[Process Monitor](https://learn.microsoft.com/en-us/sysinternals/downloads/procmon)** · [Official page](https://learn.microsoft.com/en-us/sysinternals/downloads/procmon) · **[Download Windows portable · ZIP](https://download.sysinternals.com/files/ProcessMonitor.zip)** |
| Separate storage warnings from ordinary software lag. | **[CrystalDiskInfo](https://crystalmark.info/en/software/crystaldiskinfo/)** · [Official page](https://crystalmark.info/en/software/crystaldiskinfo/) · **[Get Windows ZIP · 9.9.5](https://sourceforge.net/projects/crystaldiskinfo/files/9.9.5/CrystalDiskInfo9_9_5.zip/download)** |
| Collecting battery and sleep evidence before changing hardware or power policy. | **[powercfg](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/powercfg-command-line-options)** · [Official page](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/powercfg-command-line-options) · **[Open built-in battery and sleep diagnostics](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/powercfg-command-line-options)** |
| Measure a reproducible storage-performance problem on healthy media with a bounded test file. | **[DiskSpd](https://github.com/microsoft/diskspd)** · [Official page](https://github.com/microsoft/diskspd) · **[Download Windows x64/ARM64 tools · ZIP](https://github.com/microsoft/diskspd/releases/download/v2.3/DiskSpd.ZIP)** |
| Keep the full utility suite available when the next evidence-backed branch needs another component. | **[Microsoft Sysinternals Suite](https://learn.microsoft.com/en-us/sysinternals/)** · [Official page](https://learn.microsoft.com/en-us/sysinternals/) · **[Download Windows portable · ZIP](https://download.sysinternals.com/files/SysinternalsSuite.zip)** |
| Use its supported disk analysis when the technician has the appropriate commercial entitlement. | **[WizTree](https://diskanalyzer.com/)** · [Official page](https://diskanalyzer.com/) · **[Download Windows portable · ZIP](https://diskanalyzer.com/files/wiztree_4_34_portable.zip)** |

### Find the cause

| Open the exact resource | What to do there |
| --- | --- |
| **[Process Explorer usage](https://learn.microsoft.com/en-us/sysinternals/downloads/process-explorer)** | Inspect the exact process before stopping or changing it. |
| **[Process Monitor filtering](https://learn.microsoft.com/en-us/sysinternals/downloads/procmon)** | Limit capture to the affected process and interval. |

### Advanced

| Open the exact resource | What to do there |
| --- | --- |
| **[Windows performance tracing](https://learn.microsoft.com/en-us/windows-hardware/test/wpt/)** | Use a measured trace when ordinary process views do not explain the delay. |

## ⚡ First three actions

1. Scan that drive with [WinDirStat](https://windirstat.net/) and identify the largest folders and file types.
2. Confirm ownership and recovery options, then remove only approved files or application-managed caches.
3. Recheck free space and open the affected application to confirm it still works.

**[WinDirStat](https://windirstat.net/)** · **[Download Windows x64 · MSI](https://github.com/windirstat/windirstat/releases/download/release/v2.9.3/WinDirStat-x64.msi)**

**[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-slow-pc)** · **[Specific fixes for this job](https://herbertofury.github.io/INTA/#job/slow-pc/quick/disk-space?section=fixes)** · **[Selection evidence](https://github.com/Herbertofury/INTA/wiki/Free-Tool-Picks)**

<a id="next-action"></a>

## 🧭 Match the symptom

| You see this | Do this next | You should get |
| --- | --- | --- |
| **CPU stays high during the slow task** | Sort by CPU in [Task Manager](https://support.microsoft.com/en-us/windows/experience/system-configuration-tools-in-windows); inspect the identified process in [Process Explorer](https://learn.microsoft.com/en-us/sysinternals/downloads/process-explorer) and check its image path and owner. | One process or service correlates with the delay. |
| **Memory pressure rises as responsiveness falls** | Check [Task Manager](https://support.microsoft.com/en-us/windows/experience/system-configuration-tools-in-windows) > Performance > Memory, then close an agreed nonessential workload and repeat the task. | The before/after result distinguishes workload pressure from another cause. |
| **Disk activity stalls the system** | Check health first; on stable media use a short [Process Monitor](https://learn.microsoft.com/en-us/sysinternals/downloads/procmon) capture filtered to the affected process and path. | The slow I/O path is identified, or failing storage is routed to recovery. |
| **The healthy system drive is nearly full** | Use [WinDirStat](https://windirstat.net/) to identify the largest owned folders; remove only approved content through supported cleanup controls. | Free space improves and the original application still works. |
| **Only a task using a share or online service is slow** | Compare an approved local equivalent and follow the network guide for the remote dependency. | The evidence separates local performance from service or network latency. |

### ✅ Finish the job

- [ ] The same workload has a recorded before/after result.
- [ ] The improvement survives restart without breaking protection, backup or required apps.

**Change route when:** Drive disconnects, read errors or physical damage appear. · A proposed optimization disables required functionality without evidence.


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
