[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

<a id="-windows-diagnosis--repair"></a>

# [Windows](https://www.microsoft.com/windows) diagnosis & repair

Inspect first, change a confirmed cause, and repeat the original workload.

**22 tools** · [All categories](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Selection standards](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards)

**Selection question:** Performance, servicing, startup, or application failure?

<a id="-start-with-a-workflow"></a>

## Related playbooks

- [Diagnose a slow, freezing, or hitching PC](https://github.com/Herbertofury/INTA/wiki/Job-slow-pc)
- [Repair Windows component or system-file corruption](https://github.com/Herbertofury/INTA/wiki/Job-windows-repair)
- [Fix a failing Windows update](https://github.com/Herbertofury/INTA/wiki/Job-windows-update)
- [Diagnose crashes, blue screens, and overheating](https://github.com/Herbertofury/INTA/wiki/Job-hardware-bsod)

<a id="compare-options"></a>

<a id="️-compare-the-options"></a>
<a id="-compare-the-options"></a>

## Tool comparison

| Tool | Choose it for | Platforms / role | Official page + download |
| --- | --- | --- | --- |
| [Autoruns](#autoruns) | Explaining unexpected startup behavior and investigating persistence. | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://learn.microsoft.com/en-us/sysinternals/downloads/autoruns) · **[Download Windows portable · ZIP](https://download.sysinternals.com/files/Autoruns.zip)** |
| [Bulk Crap Uninstaller](#bcuninstaller) | Removing a selected set of unwanted applications from a workstation. | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://www.bcuninstaller.com/) · **[Download Windows portable · 7Z](https://github.com/BCUninstaller/Bulk-Crap-Uninstaller/releases/download/v6.3/BCUninstaller_6.3.0_portable.7z)** |
| [DISM](#dism) | Component-store corruption and controlled image servicing. | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image) · **[Open built-in repair instructions](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image)** |
| [Microsoft Sysinternals Suite](#sysinternals) | The core portable [Windows](https://www.microsoft.com/windows) troubleshooting kit. | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://learn.microsoft.com/en-us/sysinternals/) · **[Download Windows portable · ZIP](https://download.sysinternals.com/files/SysinternalsSuite.zip)** |
| [PowerShell](#powershell) | Documented administration and scripts with clear input, output and rollback. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) · Core | [Official page](https://learn.microsoft.com/en-us/powershell/) · **[Download Windows x64 portable · ZIP](https://github.com/PowerShell/PowerShell/releases/download/v7.7.0/PowerShell-7.7.0-win-x64.zip)** |
| [Process Explorer](#process-explorer) | Finding which process owns a locked file or consumes resources. | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://learn.microsoft.com/en-us/sysinternals/downloads/process-explorer) · **[Download Windows portable · ZIP](https://download.sysinternals.com/files/ProcessExplorer.zip)** |
| [Process Monitor](#procmon) | Reproducing an application failure and locating denied or missing resources. | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://learn.microsoft.com/en-us/sysinternals/downloads/procmon) · **[Download Windows portable · ZIP](https://download.sysinternals.com/files/ProcessMonitor.zip)** |
| [RAMMap](#rammap) | Explaining high memory use when [Task Manager](https://support.microsoft.com/en-us/windows/experience/system-configuration-tools-in-windows) does not show a clear owner. | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://learn.microsoft.com/en-us/sysinternals/downloads/rammap) · **[Download Windows portable · ZIP](https://download.sysinternals.com/files/RAMMap.zip)** |
| [Sigcheck](#sigcheck) | Checking publisher and file identity before running or escalating a suspicious binary. | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://learn.microsoft.com/en-us/sysinternals/downloads/sigcheck) · **[Download Windows portable · ZIP](https://download.sysinternals.com/files/Sigcheck.zip)** |
| [System File Checker](#sfc) | System-file corruption after establishing disk health and servicing status. | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc) · **[Open built-in system-file checks](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc)** |
| [TCPView](#tcpview) | Identifying which local application is opening a network connection. | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://learn.microsoft.com/en-us/sysinternals/downloads/tcpview) · **[Download Windows portable · ZIP](https://download.sysinternals.com/files/TCPView.zip)** |
| [WinDirStat](#windirstat) | Finding what consumes disk space before choosing a cleanup action. | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://windirstat.net/) · **[Download Windows x64 · MSI](https://github.com/windirstat/windirstat/releases/download/release/v2.9.3/WinDirStat-x64.msi)** |
| [Windows Get Help](#get-help) | Starting Microsoft’s guided [Windows](https://www.microsoft.com/windows) troubleshooters before manual printer, network or update repairs. | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://support.microsoft.com/en-us/support/get-help/about-get-help) · **[Open Windows troubleshooters](https://support.microsoft.com/en-us/support/get-help/windows-troubleshooters)** |
| [Windows Recovery Environment](#windows-recovery) | A [Windows](https://www.microsoft.com/windows) machine that will not boot or needs supported recovery options. | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference) · **[Open recovery instructions](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference)** |
| [Windows Terminal](#windows-terminal) | A consistent technician command-line workspace. | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://learn.microsoft.com/en-us/windows/terminal/) · **[Download Windows x64 portable · ZIP](https://github.com/microsoft/terminal/releases/download/v1.25.3131.0/Microsoft.WindowsTerminal_1.25.3131.0_x64.zip)** |
| [WinGet](#winget) | Repeatable application setup and individually reviewed updates. | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://learn.microsoft.com/en-us/windows/package-manager/winget/) · **[Download App Installer · MSIXBUNDLE](https://github.com/microsoft/winget-cli/releases/download/v1.29.380/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle)** |
| [WizTree](#wiztree) | Finding the folders and files responsible for a capacity problem before selecting an approved cleanup. | [Windows](https://www.microsoft.com/windows), [macOS](https://support.apple.com/macos), [Linux](https://www.kernel.org/) · Core | [Official page](https://diskanalyzer.com/) · **[Download Windows portable · ZIP](https://diskanalyzer.com/files/wiztree_4_34_portable.zip)** |
| [ProcDump](#procdump) | Collecting evidence for a repeatable crash or hung application. | [Windows](https://www.microsoft.com/windows) · Specialist | [Official page](https://learn.microsoft.com/en-us/sysinternals/downloads/procdump) · **[Download Windows portable · ZIP](https://download.sysinternals.com/files/Procdump.zip)** |
| [System Informer](#system-informer) | Advanced process investigation beyond the standard [task manager](https://support.microsoft.com/en-us/windows/experience/system-configuration-tools-in-windows). | [Windows](https://www.microsoft.com/windows) · Specialist | [Official page](https://systeminformer.sourceforge.io/) · **[Get Windows portable ZIP · download page](https://sourceforge.net/projects/systeminformer/files/systeminformer-4.0.26241.138-bin.zip/download)** |
| [WinDbg](#windbg) | Interpreting a crash dump when basic event logs do not establish the cause. | [Windows](https://www.microsoft.com/windows) · Specialist | [Official page](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/) · **[Download Windows debugger installer](https://aka.ms/windbg/download)** |
| [Windows Performance Toolkit](#windows-performance-toolkit) | Investigating intermittent CPU, storage or application responsiveness problems with evidence. | [Windows](https://www.microsoft.com/windows) · Specialist | [Official page](https://learn.microsoft.com/en-us/windows-hardware/test/wpt/) · **[Download Windows ADK setup · EXE](https://go.microsoft.com/fwlink/?linkid=2289980)** |
| [WinSAT (legacy)](#winsat) | Understanding a legacy assessment when the target system still provides it; prefer WPR/WPA for current investigations. | [Windows](https://www.microsoft.com/windows) · Evaluate | [Official page](https://learn.microsoft.com/en-us/windows/win32/winsat/windows-system-assessment-tool-portal) · **[Read legacy command instructions](https://learn.microsoft.com/en-us/windows/win32/winsat/windows-system-assessment-tool-portal)** |

Core means an everyday candidate; Specialist means a focused advanced use; Evaluate means prove it in a suitable test environment before promotion.

<a id="autoruns"></a>

## [Autoruns](https://learn.microsoft.com/en-us/sysinternals/downloads/autoruns)

Enumerates the startup locations used by applications, services, drivers and other persistence.

[Official page](https://learn.microsoft.com/en-us/sysinternals/downloads/autoruns) · **[Download Windows portable · ZIP](https://download.sysinternals.com/files/Autoruns.zip)** · [Documentation](https://learn.microsoft.com/en-us/sysinternals/downloads/autoruns)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Explaining unexpected startup behavior and investigating persistence. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | portable |
| **Access / pricing** | Free Microsoft utility |
| **Selection role** | Core |

> **Before using it:** Export the baseline and disable one understood item at a time; do not delete unknown entries in bulk.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://learn.microsoft.com/en-us/sysinternals/downloads/autoruns)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json

</details>

[Back to comparison](#compare-options)

<a id="bcuninstaller"></a>

## [Bulk Crap Uninstaller](https://www.bcuninstaller.com/)

Application inventory and batch-uninstall orchestration with leftover review.

[Official page](https://www.bcuninstaller.com/) · **[Download Windows portable · 7Z](https://github.com/BCUninstaller/Bulk-Crap-Uninstaller/releases/download/v6.3/BCUninstaller_6.3.0_portable.7z)** · [All download options](https://github.com/BCUninstaller/Bulk-Crap-Uninstaller/releases/latest) · [Documentation](https://github.com/BCUninstaller/Bulk-Crap-Uninstaller)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Removing a selected set of unwanted applications from a workstation. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | portable |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** Review shared files and proposed leftovers. Preserve the application and configuration needed to reverse a removal.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://github.com/BCUninstaller/Bulk-Crap-Uninstaller)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json

</details>

[Back to comparison](#compare-options)

<a id="dism"></a>

## [DISM](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image)

[Windows](https://www.microsoft.com/windows) servicing tool for examining and repairing the component store and offline images.

[Official page](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image) · **[Open built-in repair instructions](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image)** · [Documentation](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Component-store corruption and controlled image servicing. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | built-in |
| **Access / pricing** | Included with [Windows](https://www.microsoft.com/windows) |
| **Selection role** | Core |

> **Before using it:** Choose the correct online or offline image and a compatible repair source. Save the [DISM](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image) log.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="sysinternals"></a>

## [Microsoft Sysinternals Suite](https://learn.microsoft.com/en-us/sysinternals/)

Microsoft collection for investigating processes, startup, files, networking and system behavior.

[Official page](https://learn.microsoft.com/en-us/sysinternals/) · **[Download Windows portable · ZIP](https://download.sysinternals.com/files/SysinternalsSuite.zip)** · [All download options](https://learn.microsoft.com/en-us/sysinternals/downloads/sysinternals-suite) · [Documentation](https://learn.microsoft.com/en-us/sysinternals/downloads/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | The core portable [Windows](https://www.microsoft.com/windows) troubleshooting kit. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | portable |
| **Access / pricing** | Free Microsoft utilities |
| **Selection role** | Core |

> **Before using it:** Use each utility for a specific question; process termination and startup changes are separate actions.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://learn.microsoft.com/en-us/sysinternals/downloads/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="powershell"></a>

## [PowerShell](https://learn.microsoft.com/en-us/powershell/)

Object-oriented shell for repeatable inspection, automation and evidence collection.

[Official page](https://learn.microsoft.com/en-us/powershell/) · **[Download Windows x64 portable · ZIP](https://github.com/PowerShell/PowerShell/releases/download/v7.7.0/PowerShell-7.7.0-win-x64.zip)** · [Use Windows PowerShell 5.1](https://learn.microsoft.com/en-us/powershell/scripting/windows-powershell/starting-windows-powershell) · [All download options](https://github.com/PowerShell/PowerShell/releases/latest) · [Documentation](https://learn.microsoft.com/en-us/powershell/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Documented administration and scripts with clear input, output and rollback. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | installed |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** [PowerShell](https://learn.microsoft.com/en-us/powershell/) 7 and [Windows PowerShell 5.1](https://learn.microsoft.com/en-us/powershell/scripting/windows-powershell/starting-windows-powershell) are distinct; modules may target only one.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://learn.microsoft.com/en-us/powershell/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json

</details>

[Back to comparison](#compare-options)

<a id="process-explorer"></a>

## [Process Explorer](https://learn.microsoft.com/en-us/sysinternals/downloads/process-explorer)

Shows process trees, open handles, loaded DLLs and detailed process information.

[Official page](https://learn.microsoft.com/en-us/sysinternals/downloads/process-explorer) · **[Download Windows portable · ZIP](https://download.sysinternals.com/files/ProcessExplorer.zip)** · [Documentation](https://learn.microsoft.com/en-us/sysinternals/downloads/process-explorer)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Finding which process owns a locked file or consumes resources. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | portable |
| **Access / pricing** | Free Microsoft utility |
| **Selection role** | Core |

> **Before using it:** A familiar name or valid signature alone does not prove a process is benign.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://learn.microsoft.com/en-us/sysinternals/downloads/process-explorer)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json

</details>

[Back to comparison](#compare-options)

<a id="procmon"></a>

## [Process Monitor](https://learn.microsoft.com/en-us/sysinternals/downloads/procmon)

Captures [Windows](https://www.microsoft.com/windows) file-system, Registry and process/thread activity with rich filtering.

[Official page](https://learn.microsoft.com/en-us/sysinternals/downloads/procmon) · **[Download Windows portable · ZIP](https://download.sysinternals.com/files/ProcessMonitor.zip)** · [Documentation](https://learn.microsoft.com/en-us/sysinternals/downloads/procmon)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Reproducing an application failure and locating denied or missing resources. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | portable |
| **Access / pricing** | Free Microsoft utility |
| **Selection role** | Core |

> **Before using it:** Capture a short reproduction with focused filters; trace files can contain client paths and data.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://learn.microsoft.com/en-us/sysinternals/downloads/procmon)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="rammap"></a>

## [RAMMap](https://learn.microsoft.com/en-us/sysinternals/downloads/rammap)

Breaks down how [Windows](https://www.microsoft.com/windows) uses physical memory, including cache and driver allocations.

[Official page](https://learn.microsoft.com/en-us/sysinternals/downloads/rammap) · **[Download Windows portable · ZIP](https://download.sysinternals.com/files/RAMMap.zip)** · [Documentation](https://learn.microsoft.com/en-us/sysinternals/downloads/rammap)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Explaining high memory use when [Task Manager](https://support.microsoft.com/en-us/windows/experience/system-configuration-tools-in-windows) does not show a clear owner. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | portable |
| **Access / pricing** | Free Microsoft utility |
| **Selection role** | Core |

> **Before using it:** Inspect first. Emptying caches can make subsequent workloads slower and is not a general repair.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://learn.microsoft.com/en-us/sysinternals/downloads/rammap)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="sigcheck"></a>

## [Sigcheck](https://learn.microsoft.com/en-us/sysinternals/downloads/sigcheck)

Reports file signatures, versions and hashes for executable inspection.

[Official page](https://learn.microsoft.com/en-us/sysinternals/downloads/sigcheck) · **[Download Windows portable · ZIP](https://download.sysinternals.com/files/Sigcheck.zip)** · [Documentation](https://learn.microsoft.com/en-us/sysinternals/downloads/sigcheck)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Checking publisher and file identity before running or escalating a suspicious binary. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | portable |
| **Access / pricing** | Free Microsoft utility |
| **Selection role** | Core |

> **Before using it:** Keep external reputation queries separate from local checking and from uploading client files.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://learn.microsoft.com/en-us/sysinternals/downloads/sigcheck)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="sfc"></a>

## [System File Checker](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc)

Checks protected [Windows](https://www.microsoft.com/windows) system files and can repair them from the component store.

[Official page](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc) · **[Open built-in system-file checks](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc)** · [Documentation](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc)

| Decision | Detail |
| --- | --- |
| **Choose it for** | System-file corruption after establishing disk health and servicing status. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | built-in |
| **Access / pricing** | Included with [Windows](https://www.microsoft.com/windows) |
| **Selection role** | Core |

> **Before using it:** Use the right offline paths in recovery. A successful scan does not diagnose every crash or performance problem.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="tcpview"></a>

## [TCPView](https://learn.microsoft.com/en-us/sysinternals/downloads/tcpview)

Displays TCP and UDP endpoints with their owning processes.

[Official page](https://learn.microsoft.com/en-us/sysinternals/downloads/tcpview) · **[Download Windows portable · ZIP](https://download.sysinternals.com/files/TCPView.zip)** · [Documentation](https://learn.microsoft.com/en-us/sysinternals/downloads/tcpview)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Identifying which local application is opening a network connection. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | portable |
| **Access / pricing** | Free Microsoft utility |
| **Selection role** | Core |

> **Before using it:** Connection ownership is a clue, not a malware verdict.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://learn.microsoft.com/en-us/sysinternals/downloads/tcpview)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="windirstat"></a>

## [WinDirStat](https://windirstat.net/)

Visual disk-usage map and directory statistics.

[Official page](https://windirstat.net/) · **[Download Windows x64 · MSI](https://github.com/windirstat/windirstat/releases/download/release/v2.9.3/WinDirStat-x64.msi)** · [All download options](https://github.com/windirstat/windirstat/releases/latest) · [Documentation](https://github.com/windirstat/windirstat)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Finding what consumes disk space before choosing a cleanup action. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | installed |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** Inspection is safe; deletion is a separate decision. Protected system files and application caches need context.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://github.com/windirstat/windirstat)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json

</details>

[Back to comparison](#compare-options)

<a id="get-help"></a>

## [Windows Get Help](https://support.microsoft.com/en-us/support/get-help/about-get-help)

Microsoft’s [Windows](https://www.microsoft.com/windows) support application with guided troubleshooters for common printer, network, audio and update problems.

[Official page](https://support.microsoft.com/en-us/support/get-help/about-get-help) · **[Open Windows troubleshooters](https://support.microsoft.com/en-us/support/get-help/windows-troubleshooters)** · [Start the printer troubleshooter](https://aka.ms/PrinterConnection) · [Open network troubleshooting](https://support.microsoft.com/en-us/windows/experience/connectivity-networking/fix-wi-fi-connection-issues-in-windows) · [Open Windows Update troubleshooting](https://support.microsoft.com/en-us/windows/experience/windows-update-troubleshooter) · [Choose a classic Outlook diagnostic](https://support.microsoft.com/en-us/support/get-help/classic-outlook-troubleshooters) · [Start classic Outlook connectivity diagnostic](https://aka.ms/SaRA-OutlookDisconnect-sarahome) · [Documentation](https://support.microsoft.com/en-us/support/get-help/windows-troubleshooters)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Starting Microsoft’s guided [Windows](https://www.microsoft.com/windows) troubleshooters before manual printer, network or update repairs. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | built-in |
| **Access / pricing** | Included with supported [Windows](https://www.microsoft.com/windows); assisted support availability depends on the account and support agreement. |
| **Selection role** | Core |

> **Before using it:** Troubleshooter launch buttons require the Get Help application on supported [Windows](https://www.microsoft.com/windows). Classic [Outlook](https://support.microsoft.com/outlook) diagnostics do not apply to new [Outlook](https://support.microsoft.com/outlook). Record the findings and test the affected task after repair.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://support.microsoft.com/en-us/windows/hardware/printer/fix-printer-connection-and-printing-problems-in-windows)
- **Editorial review:** 2026-11-26
- **Evidence status:** Official documentation and launch route reviewed; not an INTA [Windows](https://www.microsoft.com/windows) runtime certification.
- **Selection provenance:**
  - Microsoft [Windows](https://www.microsoft.com/windows) printer troubleshooting instructions
  - https://support.microsoft.com/en-us/windows/hardware/printer/fix-printer-connection-and-printing-problems-in-windows
  - https://support.microsoft.com/en-us/support/get-help/windows-troubleshooters

</details>

[Back to comparison](#compare-options)

<a id="windows-recovery"></a>

## [Windows Recovery Environment](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference)

[Windows](https://www.microsoft.com/windows) [startup repair](https://support.microsoft.com/en-us/windows/experience/startup-boot/startup-repair), recovery and troubleshooting environment.

[Official page](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference) · **[Open recovery instructions](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference)** · [Documentation](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A [Windows](https://www.microsoft.com/windows) machine that will not boot or needs supported recovery options. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | built-in |
| **Access / pricing** | Included with [Windows](https://www.microsoft.com/windows) |
| **Selection role** | Core |

> **Before using it:** Identify the [Windows](https://www.microsoft.com/windows) volume and preserve [BitLocker](https://learn.microsoft.com/en-us/windows/security/operating-system-security/data-protection/bitlocker/) recovery access before recovery actions.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="windows-terminal"></a>

## [Windows Terminal](https://learn.microsoft.com/en-us/windows/terminal/)

Tabbed terminal host for [PowerShell](https://learn.microsoft.com/en-us/powershell/), [command prompt](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/cmd), WSL and other shells.

[Official page](https://learn.microsoft.com/en-us/windows/terminal/) · **[Download Windows x64 portable · ZIP](https://github.com/microsoft/terminal/releases/download/v1.25.3131.0/Microsoft.WindowsTerminal_1.25.3131.0_x64.zip)** · [All download options](https://github.com/microsoft/terminal/releases/latest) · [Documentation](https://learn.microsoft.com/en-us/windows/terminal/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A consistent technician command-line workspace. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | installed |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://learn.microsoft.com/en-us/windows/terminal/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json

</details>

[Back to comparison](#compare-options)

<a id="winget"></a>

## [WinGet](https://learn.microsoft.com/en-us/windows/package-manager/winget/)

Microsoft package manager for finding, installing and updating [Windows](https://www.microsoft.com/windows) applications.

[Official page](https://learn.microsoft.com/en-us/windows/package-manager/winget/) · **[Download App Installer · MSIXBUNDLE](https://github.com/microsoft/winget-cli/releases/download/v1.29.380/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle)** · [All download options](https://github.com/microsoft/winget-cli/releases/latest) · [Documentation](https://learn.microsoft.com/en-us/windows/package-manager/winget/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Repeatable application setup and individually reviewed updates. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | installed |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** Review source, exact package ID and upgrade changes; avoid blind bulk upgrades on a client machine.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://learn.microsoft.com/en-us/windows/package-manager/winget/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json

</details>

[Back to comparison](#compare-options)

<a id="wiztree"></a>

## [WizTree](https://diskanalyzer.com/)

Disk-space analysis with a folder tree, visual treemap and largest-file view.

[Official page](https://diskanalyzer.com/) · **[Download Windows portable · ZIP](https://diskanalyzer.com/files/wiztree_4_34_portable.zip)** · [All download options](https://diskanalyzer.com/download) · [Documentation](https://diskanalyzer.com/about)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Finding the folders and files responsible for a capacity problem before selecting an approved cleanup. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [macOS](https://support.apple.com/macos), [Linux](https://www.kernel.org/) |
| **How you use it** | portable / installed |
| **Access / pricing** | Free for personal use; commercial supporter or enterprise license required by business site and staff size. |
| **Selection role** | Core |

> **Before using it:** Choose the correct platform build: the [macOS](https://support.apple.com/macos) and [Linux](https://www.kernel.org/) editions are [WizTreeMac](https://diskanalyzer.com/) and [WizTreeNix](https://diskanalyzer.com/). Size and duplicate-name/size/date matches are inspection evidence, not approval to delete. Use a suitably licensed technician copy for client work; client staff use has its own license boundary.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://diskanalyzer.com/eula)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - Explicit INTA user request
  - https://diskanalyzer.com/download
  - https://diskanalyzer.com/eula

</details>

[Back to comparison](#compare-options)

<a id="procdump"></a>

## [ProcDump](https://learn.microsoft.com/en-us/sysinternals/downloads/procdump)

Captures process dumps around exceptions, hangs and chosen performance conditions.

[Official page](https://learn.microsoft.com/en-us/sysinternals/downloads/procdump) · **[Download Windows portable · ZIP](https://download.sysinternals.com/files/Procdump.zip)** · [Documentation](https://learn.microsoft.com/en-us/sysinternals/downloads/procdump)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Collecting evidence for a repeatable crash or hung application. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | portable |
| **Access / pricing** | Free Microsoft utility |
| **Selection role** | Specialist |

> **Before using it:** Dumps may contain credentials and document contents; store them in the private job record.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://learn.microsoft.com/en-us/sysinternals/downloads/procdump)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="system-informer"></a>

## [System Informer](https://systeminformer.sourceforge.io/)

Detailed process and resource inspection with debugging and system-information features.

[Official page](https://systeminformer.sourceforge.io/) · **[Get Windows portable ZIP · download page](https://sourceforge.net/projects/systeminformer/files/systeminformer-4.0.26241.138-bin.zip/download)** · [All download options](https://systeminformer.sourceforge.io/downloads) · [Documentation](https://github.com/winsiderss/systeminformer)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Advanced process investigation beyond the standard [task manager](https://support.microsoft.com/en-us/windows/experience/system-configuration-tools-in-windows). |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | portable |
| **Access / pricing** | Free / open source |
| **Selection role** | Specialist |

> **Before using it:** Use the selected stable or explicitly evaluated build; privileged features need careful interpretation.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://github.com/winsiderss/systeminformer)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json

</details>

[Back to comparison](#compare-options)

<a id="windbg"></a>

## [WinDbg](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/)

Microsoft debugger for user-mode and kernel debugging and crash-dump analysis.

[Official page](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/) · **[Download Windows debugger installer](https://aka.ms/windbg/download)** · [Documentation](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Interpreting a crash dump when basic event logs do not establish the cause. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | installed |
| **Access / pricing** | Free Microsoft tool |
| **Selection role** | Specialist |

> **Before using it:** A faulting module alone does not prove root cause; corroborate symbols, stack and reproduction.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="windows-performance-toolkit"></a>

## [Windows Performance Toolkit](https://learn.microsoft.com/en-us/windows-hardware/test/wpt/)

[Windows](https://www.microsoft.com/windows) Performance Recorder and Analyzer for detailed system traces.

[Official page](https://learn.microsoft.com/en-us/windows-hardware/test/wpt/) · **[Download Windows ADK setup · EXE](https://go.microsoft.com/fwlink/?linkid=2289980)** · [All download options](https://learn.microsoft.com/en-us/windows-hardware/get-started/adk-install) · [Documentation](https://learn.microsoft.com/en-us/windows-hardware/test/wpt/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Investigating intermittent CPU, storage or application responsiveness problems with evidence. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | installed |
| **Access / pricing** | Free Microsoft toolkit |
| **Selection role** | Specialist |

> **Before using it:** Record the smallest relevant reproduction and protect trace data in the job record.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://learn.microsoft.com/en-us/windows-hardware/test/wpt/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="winsat"></a>

## [WinSAT (legacy)](https://learn.microsoft.com/en-us/windows/win32/winsat/windows-system-assessment-tool-portal)

Legacy [Windows](https://www.microsoft.com/windows) performance-assessment interfaces retained here for interpreting older job records.

[Official page](https://learn.microsoft.com/en-us/windows/win32/winsat/windows-system-assessment-tool-portal) · **[Read legacy command instructions](https://learn.microsoft.com/en-us/windows/win32/winsat/windows-system-assessment-tool-portal)** · [Documentation](https://learn.microsoft.com/en-us/windows/win32/winsat/windows-system-assessment-tool-portal)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Understanding a legacy assessment when the target system still provides it; prefer WPR/WPA for current investigations. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | built-in |
| **Access / pricing** | Included on supported [Windows](https://www.microsoft.com/windows) versions |
| **Selection role** | Evaluate |

> **Before using it:** Microsoft says [WinSAT](https://learn.microsoft.com/en-us/windows/win32/winsat/windows-system-assessment-tool-portal) may change or be unavailable after [Windows](https://www.microsoft.com/windows) 8.1. Check availability and do not treat it as a modern health certification.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://learn.microsoft.com/en-us/windows/win32/winsat/windows-system-assessment-tool-portal)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
