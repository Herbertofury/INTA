[Open the technician hub](https://herbertofury.github.io/INTA/) · [Wiki home](https://github.com/Herbertofury/INTA/wiki) · [Source repository](https://github.com/Herbertofury/INTA)

# Windows diagnosis & repair

20 purpose-selected tools. Use the official download link for the current publisher offering; check architecture and release notes before installing.

| Tool | Best use | Platform | Kit level |
| --- | --- | --- | --- |
| [Microsoft Sysinternals Suite](#sysinternals) | The core portable Windows troubleshooting kit. | Windows | Core |
| [Autoruns](#autoruns) | Explaining unexpected startup behavior and investigating persistence. | Windows | Core |
| [Process Explorer](#process-explorer) | Finding which process owns a locked file or consumes resources. | Windows | Core |
| [Process Monitor](#procmon) | Reproducing an application failure and locating denied or missing resources. | Windows | Core |
| [TCPView](#tcpview) | Identifying which local application is opening a network connection. | Windows | Core |
| [RAMMap](#rammap) | Explaining high memory use when Task Manager does not show a clear owner. | Windows | Core |
| [ProcDump](#procdump) | Collecting evidence for a repeatable crash or hung application. | Windows | Specialist |
| [Sigcheck](#sigcheck) | Checking publisher and file identity before running or escalating a suspicious binary. | Windows | Core |
| [DISM](#dism) | Component-store corruption and controlled image servicing. | Windows | Core |
| [System File Checker](#sfc) | System-file corruption after establishing disk health and servicing status. | Windows | Core |
| [Windows Recovery Environment](#windows-recovery) | A Windows machine that will not boot or needs supported recovery options. | Windows | Core |
| [WinGet](#winget) | Repeatable application setup and individually reviewed updates. | Windows | Core |
| [PowerShell](#powershell) | Documented administration and scripts with clear input, output and rollback. | Windows, Linux, macOS | Core |
| [Windows Terminal](#windows-terminal) | A consistent technician command-line workspace. | Windows | Core |
| [System Informer](#system-informer) | Advanced process investigation beyond the standard task manager. | Windows | Specialist |
| [WinDirStat](#windirstat) | Finding what consumes disk space before choosing a cleanup action. | Windows | Core |
| [Bulk Crap Uninstaller](#bcuninstaller) | Removing a selected set of unwanted applications from a workstation. | Windows | Core |
| [Windows Performance Toolkit](#windows-performance-toolkit) | Investigating intermittent CPU, storage or application responsiveness problems with evidence. | Windows | Specialist |
| [WinDbg](#windbg) | Interpreting a crash dump when basic event logs do not establish the cause. | Windows | Specialist |
| [WinSAT (legacy)](#winsat) | Understanding a legacy assessment when the target system still provides it; prefer WPR/WPA for current investigations. | Windows | Evaluate |

<a name="sysinternals"></a>

## Microsoft Sysinternals Suite

Microsoft collection for investigating processes, startup, files, networking and system behavior.

**Choose it for:** The core portable Windows troubleshooting kit.

**Platforms:** Windows · **Delivery:** portable · **Kit level:** Core

**Access / pricing:** Free Microsoft utilities

[Official download](https://learn.microsoft.com/en-us/sysinternals/downloads/sysinternals-suite) · [Documentation](https://learn.microsoft.com/en-us/sysinternals/downloads/) · [Publisher](https://learn.microsoft.com/en-us/sysinternals/)

> **Before you use it:** Use each utility for a specific question; process termination and startup changes are separate actions.

**Source:** https://learn.microsoft.com/en-us/sysinternals/downloads/  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="autoruns"></a>

## Autoruns

Enumerates the startup locations used by applications, services, drivers and other persistence.

**Choose it for:** Explaining unexpected startup behavior and investigating persistence.

**Platforms:** Windows · **Delivery:** portable · **Kit level:** Core

**Access / pricing:** Free Microsoft utility

[Official download](https://learn.microsoft.com/en-us/sysinternals/downloads/autoruns) · [Documentation](https://learn.microsoft.com/en-us/sysinternals/downloads/autoruns) · [Publisher](https://learn.microsoft.com/en-us/sysinternals/downloads/autoruns)

> **Before you use it:** Export the baseline and disable one understood item at a time; do not delete unknown entries in bulk.

**Source:** https://learn.microsoft.com/en-us/sysinternals/downloads/autoruns  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json


<a name="process-explorer"></a>

## Process Explorer

Shows process trees, open handles, loaded DLLs and detailed process information.

**Choose it for:** Finding which process owns a locked file or consumes resources.

**Platforms:** Windows · **Delivery:** portable · **Kit level:** Core

**Access / pricing:** Free Microsoft utility

[Official download](https://learn.microsoft.com/en-us/sysinternals/downloads/process-explorer) · [Documentation](https://learn.microsoft.com/en-us/sysinternals/downloads/process-explorer) · [Publisher](https://learn.microsoft.com/en-us/sysinternals/downloads/process-explorer)

> **Before you use it:** A familiar name or valid signature alone does not prove a process is benign.

**Source:** https://learn.microsoft.com/en-us/sysinternals/downloads/process-explorer  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json


<a name="procmon"></a>

## Process Monitor

Captures Windows file-system, Registry and process/thread activity with rich filtering.

**Choose it for:** Reproducing an application failure and locating denied or missing resources.

**Platforms:** Windows · **Delivery:** portable · **Kit level:** Core

**Access / pricing:** Free Microsoft utility

[Official download](https://learn.microsoft.com/en-us/sysinternals/downloads/procmon) · [Documentation](https://learn.microsoft.com/en-us/sysinternals/downloads/procmon) · [Publisher](https://learn.microsoft.com/en-us/sysinternals/downloads/procmon)

> **Before you use it:** Capture a short reproduction with focused filters; trace files can contain client paths and data.

**Source:** https://learn.microsoft.com/en-us/sysinternals/downloads/procmon  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="tcpview"></a>

## TCPView

Displays TCP and UDP endpoints with their owning processes.

**Choose it for:** Identifying which local application is opening a network connection.

**Platforms:** Windows · **Delivery:** portable · **Kit level:** Core

**Access / pricing:** Free Microsoft utility

[Official download](https://learn.microsoft.com/en-us/sysinternals/downloads/tcpview) · [Documentation](https://learn.microsoft.com/en-us/sysinternals/downloads/tcpview) · [Publisher](https://learn.microsoft.com/en-us/sysinternals/downloads/tcpview)

> **Before you use it:** Connection ownership is a clue, not a malware verdict.

**Source:** https://learn.microsoft.com/en-us/sysinternals/downloads/tcpview  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="rammap"></a>

## RAMMap

Breaks down how Windows uses physical memory, including cache and driver allocations.

**Choose it for:** Explaining high memory use when Task Manager does not show a clear owner.

**Platforms:** Windows · **Delivery:** portable · **Kit level:** Core

**Access / pricing:** Free Microsoft utility

[Official download](https://learn.microsoft.com/en-us/sysinternals/downloads/rammap) · [Documentation](https://learn.microsoft.com/en-us/sysinternals/downloads/rammap) · [Publisher](https://learn.microsoft.com/en-us/sysinternals/downloads/rammap)

> **Before you use it:** Inspect first. Emptying caches can make subsequent workloads slower and is not a general repair.

**Source:** https://learn.microsoft.com/en-us/sysinternals/downloads/rammap  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="procdump"></a>

## ProcDump

Captures process dumps around exceptions, hangs and chosen performance conditions.

**Choose it for:** Collecting evidence for a repeatable crash or hung application.

**Platforms:** Windows · **Delivery:** portable · **Kit level:** Specialist

**Access / pricing:** Free Microsoft utility

[Official download](https://learn.microsoft.com/en-us/sysinternals/downloads/procdump) · [Documentation](https://learn.microsoft.com/en-us/sysinternals/downloads/procdump) · [Publisher](https://learn.microsoft.com/en-us/sysinternals/downloads/procdump)

> **Before you use it:** Dumps may contain credentials and document contents; store them in the private job record.

**Source:** https://learn.microsoft.com/en-us/sysinternals/downloads/procdump  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="sigcheck"></a>

## Sigcheck

Reports file signatures, versions and hashes for executable inspection.

**Choose it for:** Checking publisher and file identity before running or escalating a suspicious binary.

**Platforms:** Windows · **Delivery:** portable · **Kit level:** Core

**Access / pricing:** Free Microsoft utility

[Official download](https://learn.microsoft.com/en-us/sysinternals/downloads/sigcheck) · [Documentation](https://learn.microsoft.com/en-us/sysinternals/downloads/sigcheck) · [Publisher](https://learn.microsoft.com/en-us/sysinternals/downloads/sigcheck)

> **Before you use it:** Keep external reputation queries separate from local checking and from uploading client files.

**Source:** https://learn.microsoft.com/en-us/sysinternals/downloads/sigcheck  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="dism"></a>

## DISM

Windows servicing tool for examining and repairing the component store and offline images.

**Choose it for:** Component-store corruption and controlled image servicing.

**Platforms:** Windows · **Delivery:** built-in · **Kit level:** Core

**Access / pricing:** Included with Windows

[Official download](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image) · [Documentation](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image) · [Publisher](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image)

> **Before you use it:** Choose the correct online or offline image and a compatible repair source. Save the DISM log.

**Source:** https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="sfc"></a>

## System File Checker

Checks protected Windows system files and can repair them from the component store.

**Choose it for:** System-file corruption after establishing disk health and servicing status.

**Platforms:** Windows · **Delivery:** built-in · **Kit level:** Core

**Access / pricing:** Included with Windows

[Official download](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc) · [Documentation](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc) · [Publisher](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc)

> **Before you use it:** Use the right offline paths in recovery. A successful scan does not diagnose every crash or performance problem.

**Source:** https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="windows-recovery"></a>

## Windows Recovery Environment

Windows startup repair, recovery and troubleshooting environment.

**Choose it for:** A Windows machine that will not boot or needs supported recovery options.

**Platforms:** Windows · **Delivery:** built-in · **Kit level:** Core

**Access / pricing:** Included with Windows

[Official download](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference) · [Documentation](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference) · [Publisher](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference)

> **Before you use it:** Identify the Windows volume and preserve BitLocker recovery access before recovery actions.

**Source:** https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="winget"></a>

## WinGet

Microsoft package manager for finding, installing and updating Windows applications.

**Choose it for:** Repeatable application setup and individually reviewed updates.

**Platforms:** Windows · **Delivery:** installed · **Kit level:** Core

**Access / pricing:** Free / open source

[Official download](https://github.com/microsoft/winget-cli/releases/latest) · [Documentation](https://learn.microsoft.com/en-us/windows/package-manager/winget/) · [Publisher](https://learn.microsoft.com/en-us/windows/package-manager/winget/)

> **Before you use it:** Review source, exact package ID and upgrade changes; avoid blind bulk upgrades on a client machine.

**Source:** https://learn.microsoft.com/en-us/windows/package-manager/winget/  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json


<a name="powershell"></a>

## PowerShell

Object-oriented shell for repeatable inspection, automation and evidence collection.

**Choose it for:** Documented administration and scripts with clear input, output and rollback.

**Platforms:** Windows, Linux, macOS · **Delivery:** installed · **Kit level:** Core

**Access / pricing:** Free / open source

[Official download](https://github.com/PowerShell/PowerShell/releases/latest) · [Documentation](https://learn.microsoft.com/en-us/powershell/) · [Publisher](https://learn.microsoft.com/en-us/powershell/)

> **Before you use it:** PowerShell 7 and Windows PowerShell 5.1 are distinct; modules may target only one.

**Source:** https://learn.microsoft.com/en-us/powershell/  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json


<a name="windows-terminal"></a>

## Windows Terminal

Tabbed terminal host for PowerShell, command prompt, WSL and other shells.

**Choose it for:** A consistent technician command-line workspace.

**Platforms:** Windows · **Delivery:** installed · **Kit level:** Core

**Access / pricing:** Free / open source

[Official download](https://github.com/microsoft/terminal/releases/latest) · [Documentation](https://learn.microsoft.com/en-us/windows/terminal/) · [Publisher](https://learn.microsoft.com/en-us/windows/terminal/)

**Source:** https://learn.microsoft.com/en-us/windows/terminal/  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json


<a name="system-informer"></a>

## System Informer

Detailed process and resource inspection with debugging and system-information features.

**Choose it for:** Advanced process investigation beyond the standard task manager.

**Platforms:** Windows · **Delivery:** portable · **Kit level:** Specialist

**Access / pricing:** Free / open source

[Official download](https://systeminformer.sourceforge.io/downloads) · [Documentation](https://github.com/winsiderss/systeminformer) · [Publisher](https://systeminformer.sourceforge.io/)

> **Before you use it:** Use the selected stable or explicitly evaluated build; privileged features need careful interpretation.

**Source:** https://github.com/winsiderss/systeminformer  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json


<a name="windirstat"></a>

## WinDirStat

Visual disk-usage map and directory statistics.

**Choose it for:** Finding what consumes disk space before choosing a cleanup action.

**Platforms:** Windows · **Delivery:** installed · **Kit level:** Core

**Access / pricing:** Free / open source

[Official download](https://github.com/windirstat/windirstat/releases/latest) · [Documentation](https://github.com/windirstat/windirstat) · [Publisher](https://windirstat.net/)

> **Before you use it:** Inspection is safe; deletion is a separate decision. Protected system files and application caches need context.

**Source:** https://github.com/windirstat/windirstat  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json


<a name="bcuninstaller"></a>

## Bulk Crap Uninstaller

Application inventory and batch-uninstall orchestration with leftover review.

**Choose it for:** Removing a selected set of unwanted applications from a workstation.

**Platforms:** Windows · **Delivery:** portable · **Kit level:** Core

**Access / pricing:** Free / open source

[Official download](https://github.com/BCUninstaller/Bulk-Crap-Uninstaller/releases/latest) · [Documentation](https://github.com/BCUninstaller/Bulk-Crap-Uninstaller) · [Publisher](https://www.bcuninstaller.com/)

> **Before you use it:** Review shared files and proposed leftovers. Preserve the application and configuration needed to reverse a removal.

**Source:** https://github.com/BCUninstaller/Bulk-Crap-Uninstaller  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json


<a name="windows-performance-toolkit"></a>

## Windows Performance Toolkit

Windows Performance Recorder and Analyzer for detailed system traces.

**Choose it for:** Investigating intermittent CPU, storage or application responsiveness problems with evidence.

**Platforms:** Windows · **Delivery:** installed · **Kit level:** Specialist

**Access / pricing:** Free Microsoft toolkit

[Official download](https://learn.microsoft.com/en-us/windows-hardware/get-started/adk-install) · [Documentation](https://learn.microsoft.com/en-us/windows-hardware/test/wpt/) · [Publisher](https://learn.microsoft.com/en-us/windows-hardware/test/wpt/)

> **Before you use it:** Record the smallest relevant reproduction and protect trace data in the job record.

**Source:** https://learn.microsoft.com/en-us/windows-hardware/test/wpt/  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="windbg"></a>

## WinDbg

Microsoft debugger for user-mode and kernel debugging and crash-dump analysis.

**Choose it for:** Interpreting a crash dump when basic event logs do not establish the cause.

**Platforms:** Windows · **Delivery:** installed · **Kit level:** Specialist

**Access / pricing:** Free Microsoft tool

[Official download](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/) · [Documentation](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/) · [Publisher](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/)

> **Before you use it:** A faulting module alone does not prove root cause; corroborate symbols, stack and reproduction.

**Source:** https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="winsat"></a>

## WinSAT (legacy)

Legacy Windows performance-assessment interfaces retained here for interpreting older job records.

**Choose it for:** Understanding a legacy assessment when the target system still provides it; prefer WPR/WPA for current investigations.

**Platforms:** Windows · **Delivery:** built-in · **Kit level:** Evaluate

**Access / pricing:** Included on supported Windows versions

[Official download](https://learn.microsoft.com/en-us/windows/win32/winsat/windows-system-assessment-tool-portal) · [Documentation](https://learn.microsoft.com/en-us/windows/win32/winsat/windows-system-assessment-tool-portal) · [Publisher](https://learn.microsoft.com/en-us/windows/win32/winsat/windows-system-assessment-tool-portal)

> **Before you use it:** Microsoft says WinSAT may change or be unavailable after Windows 8.1. Check availability and do not treat it as a modern health certification.

**Source:** https://learn.microsoft.com/en-us/windows/win32/winsat/windows-system-assessment-tool-portal  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation
