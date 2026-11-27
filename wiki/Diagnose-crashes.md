[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Service map](https://github.com/Herbertofury/INTA/wiki/Service-Map) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

# 🧩 Diagnose crashes

**Record the stop code, failure time and recent hardware or driver changes before replacing components.**

<a id="tool-choices"></a>

## 🧰 Choose the tool for this job

Choose by the task shown in the first column. Each option includes its acquisition route and access boundary.

| Need | Official tool + exact action | Why this choice / access |
| --- | --- | --- |
| **Analyze a crash dump** | **[WinDbg](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/)** · **[Download Windows debugger · APPINSTALLER](https://aka.ms/windbg/download)** | Inspect the recorded dump and stack before blaming a driver or replacing hardware. **Access:** Free Microsoft debugger |
| **Test memory stability** | **[Memtest86+](https://www.memtest.org/)** · **[Download x64 memory-test ISO · ZIP](https://www.memtest.org/download/v9.00/mt86plus_9.00_x86_64.iso.zip)** | Boot a memory test when evidence points to RAM or its configuration. **Access:** Free and open source |
| **Observe temperatures** | **[Libre Hardware Monitor](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor)** · **[Download Windows portable · ZIP](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor/releases/download/v0.9.6/LibreHardwareMonitor.zip)** | Log relevant sensors during the reproduced workload and validate labels against the hardware. **Access:** Free and open source |

**[Current selection evidence](https://github.com/Herbertofury/INTA/wiki/Current-Tool-Choices)**

<a id="working-kit"></a>

## 🧰 Tools and links for this job

**[Open working view](https://herbertofury.github.io/INTA/#job/hardware-bsod/quick/diagnose-crashes)** · Keep the tool panel beside the instructions.

**First move:** Record the stop code, failure time and recent hardware or driver changes before replacing components.

| Use this for | Official tool + exact action |
| --- | --- |
| Diagnose crashes | **[WinDbg](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/)** · [Official page](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/) · **[Download Windows debugger · APPINSTALLER](https://aka.ms/windbg/download)** |
| Test memory outside the running OS when evidence points there. | **[Memtest86+](https://www.memtest.org/)** · [Official page](https://www.memtest.org/) · **[Download x64 memory-test ISO · ZIP](https://www.memtest.org/download/v9.00/mt86plus_9.00_x86_64.iso.zip)** |
| Observe sensor behavior without claiming a sensor alone proves the cause. | **[Libre Hardware Monitor](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor)** · [Official page](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor) · **[Download Windows portable · ZIP](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor/releases/download/v0.9.6/LibreHardwareMonitor.zip)** |
| Check storage warnings before further stress. | **[CrystalDiskInfo](https://crystalmark.info/en/software/crystaldiskinfo/)** · [Official page](https://crystalmark.info/en/software/crystaldiskinfo/) · **[Get Windows ZIP · 9.9.5](https://sourceforge.net/projects/crystaldiskinfo/files/9.9.5/CrystalDiskInfo9_9_5.zip/download)** |
| Identifying the actual USB port, hub, device and negotiated connection when a printer, scanner or peripheral disconnects or runs slowly. | **[USB Device Tree Viewer](https://www.uwe-sieber.de/usbtreeview_e.html)** · [Official page](https://www.uwe-sieber.de/usbtreeview_e.html) · **[Download Windows x64 portable · ZIP](https://www.uwe-sieber.de/files/UsbTreeView_x64.zip)** |
| Collecting battery and sleep evidence before changing hardware or power policy. | **[powercfg](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/powercfg-command-line-options)** · [Official page](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/powercfg-command-line-options) · **[Open built-in battery and sleep diagnostics](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/powercfg-command-line-options)** |
| Measure a reproducible storage-performance problem on healthy media with a bounded test file. | **[DiskSpd](https://github.com/microsoft/diskspd)** · [Official page](https://github.com/microsoft/diskspd) · **[Download Windows x64/ARM64 tools · ZIP](https://github.com/microsoft/diskspd/releases/download/v2.3/DiskSpd.ZIP)** |

### Find the cause

| Open the exact resource | What to do there |
| --- | --- |
| **[Microsoft stop-code troubleshooting](https://learn.microsoft.com/en-us/troubleshoot/windows-client/performance/stop-code-error-troubleshooting)** | Correlate stop codes, changes and relevant hardware tests. |
| **[Unexpected-restart evidence](https://learn.microsoft.com/en-us/troubleshoot/windows-client/performance/event-id-41-restart)** | Event 41 is a symptom record, not proof of a failed PSU. |

### Crash analysis

| Open the exact resource | What to do there |
| --- | --- |
| **[WinDbg documentation](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/)** | Open the dump and inspect analysis in the correct context. |

### Hardware tests

| Open the exact resource | What to do there |
| --- | --- |
| **[Memtest86+ boot and error guidance](https://www.memtest.org/)** | Match firmware support and record the tested configuration. |

## ⚡ First three actions

1. Check reliability and event history, available crash dumps and storage health to narrow the failure.
2. For a dump, use [WinDbg](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/) to inspect the crash evidence; follow the indicated driver, memory or hardware branch.
3. Change one supported cause at a time and repeat the failing workload while checking for new crashes.

**[WinDbg](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/)** · **[Download Windows debugger · APPINSTALLER](https://aka.ms/windbg/download)**

**[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-hardware-bsod)** · **[Specific fixes for this job](https://herbertofury.github.io/INTA/#job/hardware-bsod/quick/diagnose-crashes?section=fixes)** · **[Selection evidence](https://github.com/Herbertofury/INTA/wiki/Free-Tool-Picks)**

<a id="next-action"></a>

## 🧭 Match the symptom

| You see this | Do this next | You should get |
| --- | --- | --- |
| **A crash dump is available** | Open it in [WinDbg](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/) and run !analyze -v; compare the stop code and stack with other failures and the change timeline. | Repeated evidence identifies a driver/component branch; one named module is not treated as proof by itself. |
| **Crashes began after a specific driver or hardware change** | Use the exact vendor-supported rollback or correction for that identified change and repeat the failing workload. | The same reproduction no longer crashes, or the hypothesis is rejected. |
| **Evidence points to RAM instability** | Record firmware/overclock settings, boot [Memtest86+](https://www.memtest.org/) and isolate modules or slots using the hardware procedure. | Errors are reproduced and narrowed to a configuration, or the test result is recorded without excluding other causes. |
| **The system overheats, loses power or storage disappears** | Stop stress testing and inspect the indicated cooling, power or storage path with the appropriate service guidance. | The physical fault is addressed before further workload testing. |

### ✅ Finish the job

- [ ] The same workload runs without a new crash after the evidenced correction.
- [ ] Crash/test results, firmware settings and any component change are recorded.

**Change route when:** There is unsafe temperature, electrical damage or unstable storage. · The fault requires disassembly or specialist testing beyond the available service capability.


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
