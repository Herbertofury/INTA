[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

# 🧭 Free tools for urgent jobs

[Home](https://github.com/Herbertofury/INTA/wiki) › [Start Here](https://github.com/Herbertofury/INTA/wiki/Start-Here) › **Free tool picks**

Choose the symptom, open the official tool page, then follow the matching guide. These are practical no-cost starting points for INTA service work, selected for the stated task and platform. **Reviewed November 26, 2026.**

> [!IMPORTANT]
> For active ransomware or spreading compromise, start with [containment](https://github.com/Herbertofury/INTA/wiki/Job-ransomware-response). For physical storage damage or irreplaceable data, use the [recovery decision](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive) before powering up or scanning again.

## 🧭 Choose a starting point

| What needs attention? | Primary free tool | Follow the procedure |
| --- | --- | --- |
| **🛡️ [Suspected malware](#suspected-malware)** | **[Microsoft Safety Scanner](https://learn.microsoft.com/en-us/defender-endpoint/safety-scanner-download)** | [Open the guide](https://github.com/Herbertofury/INTA/wiki/Job-malware-removal) |
| **📁 [Deleted files](#deleted-files)** | **[PhotoRec](https://www.cgsecurity.org/wiki/TestDisk_Download)** | [Open the guide](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive) |
| **💾 [Failing drive](#failing-drive)** | **[GNU ddrescue](https://ftp.gnu.org/gnu/ddrescue/)** | [Open the guide](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive) |
| **🐢 [Slow PC](#slow-pc)** | **[Microsoft Sysinternals Suite](https://learn.microsoft.com/en-us/sysinternals/downloads/sysinternals-suite)** | [Open the guide](https://github.com/Herbertofury/INTA/wiki/Job-slow-pc) |
| **📊 [Disk space](#disk-space)** | **[WinDirStat](https://github.com/windirstat/windirstat/releases/latest)** | [Open the guide](https://github.com/Herbertofury/INTA/wiki/Job-slow-pc) |
| **🔎 [Find a file](#find-a-file)** | **[Everything](https://www.voidtools.com/downloads/)** | [Open the guide](https://github.com/Herbertofury/INTA/wiki/Job-preventive-maintenance) |
| **🖥️ [Remote help](#remote-help)** | **[Microsoft Quick Assist](https://apps.microsoft.com/detail/9P7BP5VNWKX5)** | [Open the guide](https://github.com/Herbertofury/INTA/wiki/Job-remote-support) |
| **🔌 [Boot USB](#boot-usb)** | **[Rufus](https://rufus.ie/#download)** | [Open the guide](https://github.com/Herbertofury/INTA/wiki/Job-boot-usb) |
| **🧰 [Windows repair](#windows-repair)** | **[DISM](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image)** | [Open the guide](https://github.com/Herbertofury/INTA/wiki/Job-windows-repair) |
| **🌐 [Inspect network traffic](#inspect-network-traffic)** | **[Wireshark](https://www.wireshark.org/download.html)** | [Open the guide](https://github.com/Herbertofury/INTA/wiki/Job-network-dns) |
| **🧪 [Suspected RAM fault](#suspected-ram-fault)** | **[Memtest86+](https://www.memtest.org/#download)** | [Open the guide](https://github.com/Herbertofury/INTA/wiki/Job-hardware-bsod) |
| **🗄️ [File backup](#file-backup)** | **[restic](https://github.com/restic/restic/releases/latest)** | [Open the guide](https://github.com/Herbertofury/INTA/wiki/Job-backup-restore) |

## 🧪 How the picks were chosen

Each pick has an official source for its purpose, a route into an existing service guide, a no-cost use case that fits technician work, and a limit that changes the next action. Commercial subscriptions, cloud storage, support contracts and the operating system itself can still carry a cost.

[WizTree](https://diskanalyzer.com/download) and [WizFile](https://antibody-software.com/wizfile/download) remain useful catalog options. Their [current disk-analyzer terms](https://diskanalyzer.com/eula) and [current file-search terms](https://antibody-software.com/wizfile/eula) reserve free use for personal work, so [WinDirStat](https://github.com/windirstat/windirstat/releases/latest) and the desktop edition of [Everything](https://www.voidtools.com/downloads/) lead these no-cost business scenarios.

### Current independent test evidence

[AV-TEST’s September–October 2026 report](https://www.av-test.org/en/antivirus/home-windows/windows-11/october-2026/microsoft-defender-antivirus-consumer-4.18-261516/) gave [Microsoft Defender Antivirus](https://learn.microsoft.com/en-us/defender-endpoint/microsoft-defender-antivirus-windows) Consumer 4.18 on [Windows 11](https://www.microsoft.com/windows/windows-11) **6/6 in protection, performance and usability**. [AV-Comparatives’ July–October 2026 real-world test](https://av-comparatives.org/tests/real-world-protection-test-july-october-2026/), released November 16, placed that product in its **Standard** award tier. Different workloads, time periods and scoring methods matter; neither result establishes a universal winner.

Those are resident-antivirus protection results. They do not rate the standalone cleanup effectiveness of [Microsoft Safety Scanner](https://learn.microsoft.com/en-us/defender-endpoint/safety-scanner-download) or [Microsoft Defender Offline](https://learn.microsoft.com/en-us/defender-endpoint/microsoft-defender-offline). The choices below use documented capabilities and service fit. We have not established comparable independent ratings for every utility, so no invented stars, overall scores or “highest-rated” badges are assigned.

## Suspected malware

**🛡️ Start with [Microsoft Safety Scanner](https://learn.microsoft.com/en-us/defender-endpoint/safety-scanner-download) · [Open the full guide](https://github.com/Herbertofury/INTA/wiki/Job-malware-removal)**

Run a portable, on-demand malware scan without replacing the client’s existing protection.

**Free use:** No-cost Microsoft download; no paid antivirus subscription required.

**Boundary:** Contain spreading compromise first; download a fresh copy for the job and treat a clean scan as limited evidence.

Save the detailed log and follow the guide’s [cleanup decision ladder](https://github.com/Herbertofury/INTA/wiki/Job-malware-removal). An unresolved compromise needs the documented escalation or rebuild route.

**Evidence:** Microsoft documents malware detection/removal, a detailed scan log and a 10-day download expiry; this is a workflow-based pick, not a comparative cleanup rating. [Source 1](https://learn.microsoft.com/en-us/defender-endpoint/safety-scanner-download) · [Source 2](https://www.microsoft.com/en-us/security/blog/2012/03/22/free-pc-safety-scan/).

## Deleted files

**📁 Start with [PhotoRec](https://www.cgsecurity.org/wiki/TestDisk_Download) · [Open the full guide](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive)**

Recover recognizable file contents from healthy media or an image when an intact backup cannot restore them.

**Free use:** Free GPL-licensed software for personal and business use.

**Boundary:** Stop source-disk writes and recover to another disk; original names and folders may be lost, and failing media needs imaging first.

Check an existing backup first. Work from a stable image when needed, then open representative recovered files; a folder full of recovered output is not sufficient acceptance.

**Evidence:** The project documents read-only source access and recovery by file signatures across damaged file systems; recoverability still depends on surviving data. [Source 1](https://www.cgsecurity.org/wiki/PhotoRec) · [Source 2](https://www.cgsecurity.org/wiki/PhotoRec_Step_By_Step) · [Source 3](https://www.gnu.org/licenses/gpl-faq.html#DoesTheGPLAllowUseOfTheProgramForAnyPurpose).

## Failing drive

**💾 Start with [GNU ddrescue](https://ftp.gnu.org/gnu/ddrescue/) · [Open the full guide](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive)**

Preserve readable data in an image with a resumable mapfile before attempting file-system repairs.

**Free use:** Free GPL-licensed software for personal and business use; separate destination storage is required.

**Boundary:** Confirm source and destination; stop for physical damage, repeated disconnects or irreplaceable data that warrants a recovery specialist.

Keep the image and mapfile together. The [image completeness check](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive) records unreadable ranges, and [recovered-file acceptance](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive) checks the client’s actual priorities.

**Evidence:** The GNU manual explains good-area-first recovery, resumable mapfiles and unreadable-range accounting; an image alone does not prove usable files. [Source 1](https://www.gnu.org/software/ddrescue/manual/ddrescue_manual.html) · [Source 2](https://www.gnu.org/software/ddrescue/) · [Source 3](https://www.gnu.org/licenses/gpl-faq.html#DoesTheGPLAllowUseOfTheProgramForAnyPurpose).

## Slow PC

**🐢 Start with [Microsoft Sysinternals Suite](https://learn.microsoft.com/en-us/sysinternals/downloads/sysinternals-suite) · [Open the full guide](https://github.com/Herbertofury/INTA/wiki/Job-slow-pc)**

Inspect process activity and startup entries to identify a concrete cause before changing settings.

**Free use:** No-cost Microsoft utilities under the vendor license; obtain them from the official source for the serviced device.

**Boundary:** Choose utilities supported on the client’s [Windows](https://www.microsoft.com/windows) release; observe before terminating processes or disabling startup entries.

Use [Process Explorer](https://learn.microsoft.com/en-us/sysinternals/downloads/process-explorer) for the process question and [Autoruns](https://learn.microsoft.com/en-us/sysinternals/downloads/autoruns) for the startup question; the suite contains both. Capture the observation before changing an entry.

**Evidence:** Microsoft documents process ownership, handles, loaded modules and startup inspection; these observations support a measured troubleshooting decision. [Source 1](https://learn.microsoft.com/en-us/sysinternals/downloads/sysinternals-suite) · [Source 2](https://learn.microsoft.com/en-us/sysinternals/downloads/process-explorer) · [Source 3](https://learn.microsoft.com/en-us/sysinternals/downloads/autoruns) · [Source 4](https://learn.microsoft.com/en-us/sysinternals/license-terms).

## Disk space

**📊 Start with [WinDirStat](https://github.com/windirstat/windirstat/releases/latest) · [Open the full guide](https://github.com/Herbertofury/INTA/wiki/Job-slow-pc)**

See which folders and file types occupy a healthy disk before choosing a cleanup target.

**Free use:** Free GPL-3.0 software for personal and business use.

**Boundary:** Avoid scanning failing media; confirm file ownership and a recovery route before deleting anything.

Inspection and deletion are separate actions. Select a specific folder or drive, review the largest allocations with the client, and recheck free space after an agreed change.

**Evidence:** The official project documents size-sorted lists, file-type statistics and treemaps that make disk allocation inspectable. [Source 1](https://github.com/windirstat/windirstat) · [Source 2](https://github.com/windirstat/windirstat/blob/master/LICENSE.md) · [Source 3](https://www.gnu.org/licenses/gpl-faq.html#DoesTheGPLAllowUseOfTheProgramForAnyPurpose).

## Find a file

**🔎 Start with [Everything](https://www.voidtools.com/downloads/) · [Open the full guide](https://github.com/Herbertofury/INTA/wiki/Job-preventive-maintenance)**

Locate files and folders by name using a local index instead of repeatedly browsing directories.

**Free use:** The desktop application is free for commercial use under its published license.

**Boundary:** Scope the index to approved locations; the separate [Everything Server](https://www.voidtools.com/everything/server/buy/) product has paid business licensing.

This recommendation covers local desktop search. [Everything Server](https://www.voidtools.com/everything/server/buy/) has a separate business site license. Close case-only searches and remove temporary indexing settings at handoff.

**Evidence:** The official FAQ documents filename indexing, live updates and NTFS requirements; the published license and developer guidance permit commercial desktop use. [Source 1](https://www.voidtools.com/faq/) · [Source 2](https://www.voidtools.com/License.txt) · [Source 3](https://www.voidtools.com/forum/viewtopic.php?t=7508) · [Source 4](https://www.voidtools.com/everything/server/buy/).

## Remote help

**🖥️ Start with [Microsoft Quick Assist](https://apps.microsoft.com/detail/9P7BP5VNWKX5) · [Open the full guide](https://github.com/Herbertofury/INTA/wiki/Job-remote-support)**

Start a one-time attended [Windows](https://www.microsoft.com/windows) support session with separate screen-sharing and control approval.

**Free use:** Free Microsoft application for the documented support-staff workflow; organization policy still applies.

**Boundary:** Requires internet and a present user; use the organization’s approved managed tool when unattended access or session audit logs are required.

Confirm the helper’s identity through the existing support channel. End the session with the user and record the work in the service ticket; the application does not create local session audit logs.

**Evidence:** Microsoft’s IT guidance documents the session-code flow, helper sign-in, user approval and the absence of local session logs. [Source 1](https://learn.microsoft.com/en-us/windows/client-management/client-tools/quick-assist) · [Source 2](https://apps.microsoft.com/detail/9P7BP5VNWKX5).

## Boot USB

**🔌 Start with [Rufus](https://rufus.ie/#download) · [Open the full guide](https://github.com/Herbertofury/INTA/wiki/Job-boot-usb)**

Write a trusted operating-system or rescue image to a USB drive with explicit target and boot-mode choices.

**Free use:** Free GPL-3.0-or-later software for personal and business use; the selected image has its own license.

**Boundary:** Writing erases the selected USB; verify its identity and use a trusted image that matches the target firmware.

Read the tool’s log and test the finished media on a suitable machine. A completed write does not prove the client firmware will boot it.

**Evidence:** The official project documents BIOS/UEFI media creation, image checksums and portable operation. [Source 1](https://github.com/pbatard/rufus) · [Source 2](https://rufus.ie/en/) · [Source 3](https://github.com/pbatard/rufus/wiki/FAQ).

## [Windows repair](https://github.com/Herbertofury/INTA/wiki/Job-windows-repair)

**🧰 Start with [DISM](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image) · [Open the full guide](https://github.com/Herbertofury/INTA/wiki/Job-windows-repair)**

Check and repair the [Windows](https://www.microsoft.com/windows) component store before the guide’s system-file verification step.

**Free use:** Included with licensed [Windows](https://www.microsoft.com/windows); no separate repair utility purchase or download is needed.

**Boundary:** Check storage health and recovery options first; use the correct online or offline target and a compatible repair source.

The primary link opens the built-in repair instructions. Follow the guide through [System File Checker](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc) and symptom retesting, then retain the relevant repair logs.

**Evidence:** Microsoft documents corruption checks, repairable/non-repairable image states and controlled repair sources. [Source 1](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image) · [Source 2](https://support.microsoft.com/en-us/windows/experience/backup-recovery/using-system-file-checker-in-windows).

## Inspect network traffic

**🌐 Start with [Wireshark](https://www.wireshark.org/download.html) · [Open the full guide](https://github.com/Herbertofury/INTA/wiki/Job-network-dns)**

Explain a stubborn DNS or TCP failure with a narrowly scoped packet capture after basic connection checks.

**Free use:** Free GPL software; on [Windows](https://www.microsoft.com/windows), [Npcap](https://npcap.com/)’s free-use exception covers copies used only with [Wireshark](https://www.wireshark.org/).

**Boundary:** Start with the guide’s address, gateway and DNS checks; capture only authorized traffic and keep case data private.

Use the official package with its supported [capture driver](https://npcap.com/guide/index.html).

**Evidence:** The official guide documents packet inspection and filtering; the capture-driver publisher confirms the relevant free-use exception. [Source 1](https://www.wireshark.org/docs/wsug_html_chunked/ChapterIntroduction.html) · [Source 2](https://npcap.com/guide/index.html) · [Source 3](https://www.wireshark.org/docs/wsdg_html_chunked/ChLibsPcap).

## Suspected RAM fault

**🧪 Start with [Memtest86+](https://www.memtest.org/#download) · [Open the full guide](https://github.com/Herbertofury/INTA/wiki/Job-hardware-bsod)**

Test memory outside the running operating system when crash evidence points to RAM or memory stability.

**Free use:** Free GPL-2.0 software for personal and business use.

**Boundary:** Record firmware settings and test duration; memory errors need isolation, and a passing run does not rule out other crash causes.

Check the project’s current boot and Secure Boot instructions. Preserve recovery keys before firmware changes and restore the intended firmware configuration afterward.

**Evidence:** The official project documents independent boot testing, error investigation and supported firmware/platform requirements. [Source 1](https://www.memtest.org/) · [Source 2](https://github.com/memtest86plus/memtest86plus) · [Source 3](https://www.gnu.org/licenses/gpl-faq.html#DoesTheGPLAllowUseOfTheProgramForAnyPurpose).

## File backup

**🗄️ Start with [restic](https://github.com/restic/restic/releases/latest) · [Open the full guide](https://github.com/Herbertofury/INTA/wiki/Job-backup-restore)**

Create encrypted file snapshots and restore a representative sample to prove the backup is usable.

**Free use:** Free BSD-2-Clause software for personal and business use; storage or hosting may cost money.

**Boundary:** This is a file-backup workflow, not a bare-metal image; retain the repository password and test a separate restore target.

Choose an existing or separate destination with enough capacity. Keep credentials recoverable, run the documented repository checks, and perform a restore before calling the job complete.

**Evidence:** The official documentation provides repository checks, snapshot selection and explicit restore commands. [Source 1](https://restic.net/) · [Source 2](https://restic.readthedocs.io/en/stable/045_working_with_repos.html) · [Source 3](https://restic.readthedocs.io/en/stable/050_restore.html).

## ➜ Continue

Use the [Decision Desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) when the symptom spans several jobs, the [complete Tool Directory](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) for specialist alternatives, and [Service Evidence](https://github.com/Herbertofury/INTA/wiki/Service-Evidence) for the proof needed at handoff.

The shared starting rows come from the [quick-start catalog](https://github.com/Herbertofury/INTA/blob/main/data/quick-start.json). Update its evidence and limits when changing a recommendation, following the [Knowledge Standards](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards).

---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
