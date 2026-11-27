# Current tool choices by service scenario

[Home](https://github.com/Herbertofury/INTA/wiki) · [All service scenarios](https://github.com/Herbertofury/INTA/wiki/Service-Map) · [Malware tool comparison](https://github.com/Herbertofury/INTA/wiki/Malware-Tool-Choices)

**Reviewed November 26, 2026: all 27 quick answers across all 22 service jobs.** The selections expose distinct useful roles directly on the front pages and in each working kit. They are choices for an identified task and environment; this review does not claim a universal winner, a new performance benchmark or a repair tested on every customer machine.

The [earlier job-tool review](https://github.com/Herbertofury/INTA/blob/main/docs/reference/job-tool-review-2026-11-26.md) remains as historical evidence. The live selections now come from [quick-start.json](https://github.com/Herbertofury/INTA/blob/main/data/quick-start.json), using the canonical [catalog](https://github.com/Herbertofury/INTA/blob/main/data/tools.json), [acquisition routes](https://github.com/Herbertofury/INTA/blob/main/data/tool-access.json) and [job workspaces](https://github.com/Herbertofury/INTA/blob/main/data/job-workspaces.json). Existing primary tools, complete guides and source evidence remain available.

## Findings that change the choice

- **[Malwarebytes](https://www.malwarebytes.com/) Free / MBAM:** the current feature matrix offers Quick and Custom scans in Free; Threat Scan and real-time protection require a paid tier. Home terms cover personal non-commercial use and restrict business-used devices and remote operation. The small-business exception is for paid Home under Teams terms, not blanket Free technician rights. [Feature matrix](https://help.malwarebytes.com/hc/en-us/articles/49849786059035-Free-vs-Paid-Malwarebytes-Security-features) · [Home terms](https://www.malwarebytes.com/eula).
- **[AdwCleaner](https://www.malwarebytes.com/adwcleaner):** its separate terms include a qualifying repair-shop allowance: the entity primarily diagnoses/remediates external customer devices and has no more than 10 staff across the entity and affiliates. Other for-profit use requires the applicable per-device business rights. The cleaner’s role is adware/PUP and browser-hijacker cleanup. [Usage terms](https://www.malwarebytes.com/adwcleaner/eula) · [Scan workflow](https://help.malwarebytes.com/hc/en-us/articles/31589231147035-Scan-and-clean-a-device-with-AdwCleaner).
- **[Bitdefender Antivirus Free](https://www.bitdefender.com/en-us/consumer/free-antivirus):** an ongoing-protection choice for an eligible personal [Windows](https://www.microsoft.com/windows) PC. Keep a single chosen real-time antivirus. Its consumer Free terms do not establish a business-endpoint entitlement. [Product](https://www.bitdefender.com/en-us/consumer/free-antivirus) · [Subscription terms](https://www.bitdefender.com/en-us/site/view/subscription-agreement-and-terms-of-services-for-home-user-solutions).
- **Repair media and [data recovery](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive):** a ready recovery ISO, supported undelete and file carving solve different problems. Preserve unstable storage before repair writes and recover to a different destination. [Included recovery tools](https://www.system-rescue.org/System-tools/) · [GNU ddrescue manual](https://www.gnu.org/software/ddrescue/manual/ddrescue_manual.html) · [Supported NTFS undelete](https://www.cgsecurity.org/wiki/TestDisk:_undelete_file_for_NTFS) · [File carving](https://www.cgsecurity.org/wiki/PhotoRec).
- **Commercial access remains visible:** [WizTree](https://diskanalyzer.com/) and [WizFile](https://antibody-software.com/wizfile/) require the applicable business license; [Veeam agent](https://www.veeam.com/products/free/microsoft-windows) restore choices keep service-provider terms separate from the existence of a Free edition. The free open-source alternatives remain available where appropriate. [WizTree terms](https://diskanalyzer.com/eula) · [WizFile terms](https://antibody-software.com/wizfile/eula) · [Veeam licensing](https://www.veeam.com/legal/licensing-policy.html).

## Coverage and disposition

| Quick answer | Service job | Visible choices | Disposition |
| --- | --- | ---: | --- |
| [Remove viruses](https://github.com/Herbertofury/INTA/wiki/Remove-viruses) | [Remove malware and browser hijackers](https://github.com/Herbertofury/INTA/wiki/Job-malware-removal) | 5 | Retain original primary; expose distinct task roles |
| [Fix a printer](https://github.com/Herbertofury/INTA/wiki/Fix-a-printer) | [Restore printing without breaking other queues](https://github.com/Herbertofury/INTA/wiki/Job-printer-repair) | 4 | Retain original primary; expose distinct task roles |
| [Fix internet](https://github.com/Herbertofury/INTA/wiki/Fix-internet) | [Fix internet, DNS, or local-network problems](https://github.com/Herbertofury/INTA/wiki/Job-network-dns) | 3 | Retain original primary; expose distinct task roles |
| [Fix email](https://github.com/Herbertofury/INTA/wiki/Fix-email) | [Fix Outlook or email send/receive problems](https://github.com/Herbertofury/INTA/wiki/Job-outlook-email) | 1 | Retain the specific supported tool; no extra app added |
| [Recover a failing drive](https://github.com/Herbertofury/INTA/wiki/Recover-a-failing-drive) | [Rescue data from a failing or unreadable drive](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive) | 3 | Retain original primary; expose distinct task roles |
| [Recover deleted files](https://github.com/Herbertofury/INTA/wiki/Recover-deleted-files) | [Rescue data from a failing or unreadable drive](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive) | 2 | Retain original primary; expose distinct task roles |
| [Fix a slow PC](https://github.com/Herbertofury/INTA/wiki/Fix-a-slow-PC) | [Diagnose a slow, freezing, or hitching PC](https://github.com/Herbertofury/INTA/wiki/Job-slow-pc) | 4 | Retain original primary; expose distinct task roles |
| [Help remotely](https://github.com/Herbertofury/INTA/wiki/Help-remotely) | [Start and finish a clean remote-support session](https://github.com/Herbertofury/INTA/wiki/Job-remote-support) | 4 | Retain original primary; expose distinct task roles |
| [Fix Windows updates](https://github.com/Herbertofury/INTA/wiki/Fix-Windows-updates) | [Fix a failing Windows update](https://github.com/Herbertofury/INTA/wiki/Job-windows-update) | 4 | Retain original primary; expose distinct task roles |
| [Repair Windows files](https://github.com/Herbertofury/INTA/wiki/Repair-Windows-files) | [Repair Windows component or system-file corruption](https://github.com/Herbertofury/INTA/wiki/Job-windows-repair) | 3 | Retain original primary; expose distinct task roles |
| [Fix a PC that will not boot](https://github.com/Herbertofury/INTA/wiki/Fix-a-PC-that-will-not-boot) | [Recover a Windows PC that will not boot](https://github.com/Herbertofury/INTA/wiki/Job-boot-recovery) | 4 | Retain original primary; expose distinct task roles |
| [Diagnose crashes](https://github.com/Herbertofury/INTA/wiki/Diagnose-crashes) | [Diagnose crashes, blue screens, and overheating](https://github.com/Herbertofury/INTA/wiki/Job-hardware-bsod) | 3 | Retain original primary; expose distinct task roles |
| [Free disk space](https://github.com/Herbertofury/INTA/wiki/Free-disk-space) | [Diagnose a slow, freezing, or hitching PC](https://github.com/Herbertofury/INTA/wiki/Job-slow-pc) | 2 | Retain original primary; expose distinct task roles |
| [Clone or move a drive](https://github.com/Herbertofury/INTA/wiki/Clone-or-move-a-drive) | [Clone a healthy system or migrate to a new SSD](https://github.com/Herbertofury/INTA/wiki/Job-disk-migration) | 3 | Retain original primary; expose distinct task roles |
| [Restore a file backup](https://github.com/Herbertofury/INTA/wiki/Restore-a-file-backup) | [Prove a backup can actually restore](https://github.com/Herbertofury/INTA/wiki/Job-backup-restore) | 3 | Retain original primary; expose distinct task roles |
| [Make a boot USB](https://github.com/Herbertofury/INTA/wiki/Make-a-boot-USB) | [Build and refresh a technician rescue USB](https://github.com/Herbertofury/INTA/wiki/Job-boot-usb) | 4 | Retain original primary; expose distinct task roles |
| [Set up a new PC](https://github.com/Herbertofury/INTA/wiki/Set-up-a-new-PC) | [Set up a dependable new workstation](https://github.com/Herbertofury/INTA/wiki/Job-new-pc) | 3 | Retain original primary; expose distinct task roles |
| [Maintain a PC](https://github.com/Herbertofury/INTA/wiki/Maintain-a-PC) | [Run a useful workstation maintenance visit](https://github.com/Herbertofury/INTA/wiki/Job-preventive-maintenance) | 3 | Retain original primary; expose distinct task roles |
| [Start a repair](https://github.com/Herbertofury/INTA/wiki/Start-a-repair) | [Start here: triage any computer problem](https://github.com/Herbertofury/INTA/wiki/Job-intake-triage) | 0 | Retain guide-first response; no extra download required |
| [Contain ransomware](https://github.com/Herbertofury/INTA/wiki/Contain-ransomware) | [Contain ransomware or a wider compromise](https://github.com/Herbertofury/INTA/wiki/Job-ransomware-response) | 0 | Retain guide-first response; no extra download required |
| [Recover an office outage](https://github.com/Herbertofury/INTA/wiki/Recover-an-office-outage) | [Bring an office back after a power outage](https://github.com/Herbertofury/INTA/wiki/Job-outage-recovery) | 0 | Retain guide-first response; no extra download required |
| [Repair domain sign-in](https://github.com/Herbertofury/INTA/wiki/Repair-domain-sign-in) | [Repair a workstation’s domain trust relationship](https://github.com/Herbertofury/INTA/wiki/Job-ad-trust) | 1 | Retain the specific supported tool; no extra app added |
| [Troubleshoot with AI](https://github.com/Herbertofury/INTA/wiki/Troubleshoot-with-AI) | [Use AI as a careful troubleshooting copilot](https://github.com/Herbertofury/INTA/wiki/Job-ai-triage) | 2 | Retain original primary; expose distinct task roles |
| [Finish the job and hand over](https://github.com/Herbertofury/INTA/wiki/Finish-the-job-and-hand-over) | [Close a repair with a clear client handoff](https://github.com/Herbertofury/INTA/wiki/Job-client-handoff) | 0 | Retain guide-first response; no extra download required |
| [Find a file](https://github.com/Herbertofury/INTA/wiki/Find-a-file) | [Run a useful workstation maintenance visit](https://github.com/Herbertofury/INTA/wiki/Job-preventive-maintenance) | 2 | Retain original primary; expose distinct task roles |
| [Inspect network traffic](https://github.com/Herbertofury/INTA/wiki/Inspect-network-traffic) | [Fix internet, DNS, or local-network problems](https://github.com/Herbertofury/INTA/wiki/Job-network-dns) | 3 | Retain original primary; expose distinct task roles |
| [Test memory](https://github.com/Herbertofury/INTA/wiki/Test-memory) | [Diagnose crashes, blue screens, and overheating](https://github.com/Herbertofury/INTA/wiki/Job-hardware-bsod) | 1 | Retain the specific supported tool; no extra app added |

## Sources and selection reasoning by answer

### 🛡️ Remove viruses

[Quick answer](https://github.com/Herbertofury/INTA/wiki/Remove-viruses) · [Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-malware-removal)

Separate broad on-demand cleanup, adware/PUP cleanup and ongoing protection. Keep the standalone Microsoft scan accessible for a job whose commercial or servicing scope does not fit consumer Free terms. Offline scanning is a conditional persistence branch.

| Tool and exact action | Role and reason | Access boundary | Primary evidence |
| --- | --- | --- | --- |
| [Malwarebytes](https://www.malwarebytes.com/) · [Download Windows installer · EXE](https://downloads.malwarebytes.com/file/mb-windows) | **On-demand cleanup.** Use an available Free Quick or Custom Scan for broad cleanup on an eligible personal device. | Free personal-use scans; business servicing rights differ. | [Source 1](https://help.malwarebytes.com/hc/en-us/articles/49849786059035-Free-vs-Paid-Malwarebytes-Security-features) · [Source 2](https://www.malwarebytes.com/eula) |
| [Malwarebytes AdwCleaner](https://www.malwarebytes.com/adwcleaner) · [Download Windows portable cleaner · EXE](https://adwcleaner.malwarebytes.com/adwcleaner?channel=release) | **Adware and hijackers.** Target adware, unwanted software and browser hijackers, then review findings before quarantine. | Free for qualifying repair firms with ≤10 total staff; other business use needs rights. | [Source 1](https://help.malwarebytes.com/hc/en-us/articles/31589231147035-Scan-and-clean-a-device-with-AdwCleaner) · [Source 2](https://www.malwarebytes.com/adwcleaner/eula) |
| [Bitdefender Antivirus Free](https://www.bitdefender.com/en-us/consumer/free-antivirus) · [Download Windows setup · EXE](https://download.bitdefender.com/windows/installer/en-us/bitdefender_avfree.exe) | **Personal PC protection.** Choose it as the single ongoing antivirus on an eligible personal [Windows](https://www.microsoft.com/windows) PC. | Personal/non-commercial only; account required. | [Source 1](https://www.bitdefender.com/en-us/consumer/free-antivirus) · [Source 2](https://www.bitdefender.com/en-us/site/view/subscription-agreement-and-terms-of-services-for-home-user-solutions) |
| [Microsoft Safety Scanner](https://learn.microsoft.com/en-us/defender-endpoint/safety-scanner-download) · [Download Windows x64 scanner · EXE](https://go.microsoft.com/fwlink/?LinkId=212732) | **Portable second opinion.** Download a fresh standalone scanner for a logged on-demand check without replacing existing protection. | Free Microsoft scanner; download expires after 10 days. | [Source 1](https://learn.microsoft.com/en-us/defender-endpoint/safety-scanner-download) |
| [Microsoft Defender Offline](https://learn.microsoft.com/en-us/defender-endpoint/microsoft-defender-offline) · [Run the built-in offline scan](https://learn.microsoft.com/en-us/defender-endpoint/microsoft-defender-offline) | **Scan outside [Windows](https://www.microsoft.com/windows).** Use the built-in offline scan when persistence warrants it and the supported platform and recovery prerequisites are met. | Included with supported [Windows](https://www.microsoft.com/windows) | [Source 1](https://learn.microsoft.com/en-us/defender-endpoint/microsoft-defender-offline) |

### 🖨️ Fix a printer

[Quick answer](https://github.com/Herbertofury/INTA/wiki/Fix-a-printer) · [Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-printer-repair)

Use the printer-specific diagnostic, exact model/OS driver route, IP discovery and a separate scanner test. A generic driver-updater bundle does not establish model, architecture or protected-print compatibility.

| Tool and exact action | Role and reason | Access boundary | Primary evidence |
| --- | --- | --- | --- |
| [Windows Get Help](https://support.microsoft.com/en-us/support/get-help/about-get-help) · [Start the printer troubleshooter](https://aka.ms/PrinterConnection) | **Printer diagnostic.** Start the printer-specific diagnostic after confirming the physical device and connection. | Included with supported [Windows](https://www.microsoft.com/windows) | [Source 1](https://support.microsoft.com/en-us/windows/hardware/printer/fix-printer-connection-and-printing-problems-in-windows) |
| [Official OEM driver and firmware portals](https://support.microsoft.com/en-us/windows/hardware/drivers/update-drivers-through-device-manager-in-windows) · [Find the correct printer driver](https://support.microsoft.com/en-us/windows/hardware/printer/download-and-install-the-latest-printer-drivers) | **Exact-model driver.** Use the manufacturer/model/OS route when that driver path is supported; check protected print mode first. | Vendor downloads; match model, architecture and policy. | [Source 1](https://support.microsoft.com/en-us/windows/hardware/printer/download-and-install-the-latest-printer-drivers) · [Source 2](https://learn.microsoft.com/en-us/windows/modern-print/windows-protected-print-mode/windows-protected-print-mode) |
| [Advanced IP Scanner](https://www.advanced-ip-scanner.com/) · [Download Windows scanner · EXE](https://download.advanced-ip-scanner.com/download/files/Advanced_IP_Scanner_2.5.4594.1.exe) | **Find the printer IP.** Identify the printer on the authorized local subnet, then match its model or MAC to the queue. | Free [Windows](https://www.microsoft.com/windows) tool; scan only an approved subnet. | [Source 1](https://www.advanced-ip-scanner.com/help/) · [Source 2](https://www.advanced-ip-scanner.com/) |
| [NAPS2](https://www.naps2.com/) · [Download Windows x64 portable · ZIP](https://github.com/cyanfish/naps2/releases/download/v8.6.1/naps2-8.6.1-win-x64.zip) | **Scanner and ADF test.** Test flatbed, feeder and multipage output independently of the printer queue. | Free and open source | [Source 1](https://www.naps2.com/) · [Source 2](https://www.naps2.com/windows-scanning) |

### 🌐 Fix internet

[Quick answer](https://github.com/Herbertofury/INTA/wiki/Fix-internet) · [Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-network-dns)

Use built-in connection checks first, then distinguish DNS/TCP and HTTP/TLS failures. Packet capture remains available in its more specific case instead of being the first response to every Wi-Fi fault.

| Tool and exact action | Role and reason | Access boundary | Primary evidence |
| --- | --- | --- | --- |
| [Windows Get Help](https://support.microsoft.com/en-us/support/get-help/about-get-help) · [Open network troubleshooting](https://support.microsoft.com/en-us/windows/experience/connectivity-networking/fix-wi-fi-connection-issues-in-windows) | **Connection diagnostic.** Run the supported network diagnostic after checking link, address, gateway and DNS. | Included with supported [Windows](https://www.microsoft.com/windows) | [Source 1](https://support.microsoft.com/en-us/windows/experience/connectivity-networking/fix-wi-fi-connection-issues-in-windows) |
| [PowerShell](https://learn.microsoft.com/en-us/powershell/) · [Use Windows PowerShell 5.1](https://learn.microsoft.com/en-us/powershell/scripting/windows-powershell/starting-windows-powershell) | **DNS and port checks.** Use existing commands to separate name resolution, routing and a required TCP service. | Included with supported [Windows](https://www.microsoft.com/windows) | [Source 1](https://learn.microsoft.com/en-us/powershell/module/dnsclient/resolve-dnsname) · [Source 2](https://learn.microsoft.com/en-us/powershell/module/nettcpip/test-netconnection) |
| [curl](https://curl.se/) · [Use the Windows-included curl.exe](https://curl.se/windows/microsoft.html) | **HTTP and TLS check.** Test the documented endpoint when ordinary connectivity works but an application request fails. | Included with supported [Windows](https://www.microsoft.com/windows) | [Source 1](https://curl.se/docs/manpage.html) |

### ✉️ Fix email

[Quick answer](https://github.com/Herbertofury/INTA/wiki/Fix-email) · [Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-outlook-email)

Retain the classic [Outlook](https://support.microsoft.com/outlook)-specific diagnostic. New [Outlook](https://support.microsoft.com/outlook) follows the existing separate workflow; changing to another repair utility does not remove this edition distinction.

| Tool and exact action | Role and reason | Access boundary | Primary evidence |
| --- | --- | --- | --- |
| [Windows Get Help](https://support.microsoft.com/en-us/support/get-help/about-get-help) · [Choose a classic Outlook diagnostic](https://support.microsoft.com/en-us/support/get-help/classic-outlook-troubleshooters) | **Classic [Outlook](https://support.microsoft.com/outlook) diagnostic.** Choose the matching classic [Outlook](https://support.microsoft.com/outlook) test after comparing webmail; new [Outlook](https://support.microsoft.com/outlook) uses its separate branch. | Included diagnostics; email service licensing is separate. | [Source 1](https://support.microsoft.com/en-us/support/get-help/classic-outlook-troubleshooters) |

### 💾 Recover a failing drive

[Quick answer](https://github.com/Herbertofury/INTA/wiki/Recover-a-failing-drive) · [Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive)

Retain the ready recovery ISO as the first acquisition. The included imaging workflow preserves a failing source; metadata inspection and file carving follow on the copy. A source tarball is not presented as a [Windows repair](https://github.com/Herbertofury/INTA/wiki/Job-windows-repair) installer.

| Tool and exact action | Role and reason | Access boundary | Primary evidence |
| --- | --- | --- | --- |
| [SystemRescue](https://www.system-rescue.org/) · [Download x64 rescue image · ISO](https://fastly-cdn.system-rescue.org/releases/13.03/systemrescue-13.03-amd64.iso) | **Image the failing source.** Boot the ready recovery environment and use its [GNU ddrescue](https://www.gnu.org/software/ddrescue/) workflow before repair writes. | Free and open source | [Source 1](https://www.system-rescue.org/) · [Source 2](https://www.system-rescue.org/System-tools/) · [Source 3](https://www.gnu.org/software/ddrescue/manual/ddrescue_manual.html) |
| [TestDisk](https://www.cgsecurity.org/wiki/TestDisk) · [Download Windows x64 tools · ZIP](https://www.cgsecurity.org/testdisk-7.2.win64.zip) | **Inspect the working copy.** Investigate partition or filesystem metadata on the preserved image or clone. | Free and open source | [Source 1](https://www.cgsecurity.org/wiki/TestDisk) · [Source 2](https://www.cgsecurity.org/wiki/TestDisk_Step_By_Step) |
| [PhotoRec](https://www.cgsecurity.org/wiki/PhotoRec) · [Download PhotoRec / TestDisk · ZIP](https://www.cgsecurity.org/testdisk-7.2.win64.zip) | **Recover file contents.** Carve file contents from the preserved copy when usable filesystem metadata is unavailable. | Free and open source | [Source 1](https://www.cgsecurity.org/wiki/PhotoRec) |

### 📁 Recover deleted files

[Quick answer](https://github.com/Herbertofury/INTA/wiki/Recover-deleted-files) · [Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive)

Keep signature-based recovery and add supported filesystem undelete as a distinct route when metadata and names can be retained. Both utilities ship in the same publisher bundle; recovery output still goes to a separate device.

| Tool and exact action | Role and reason | Access boundary | Primary evidence |
| --- | --- | --- | --- |
| [PhotoRec](https://www.cgsecurity.org/wiki/PhotoRec) · [Download PhotoRec / TestDisk · ZIP](https://www.cgsecurity.org/testdisk-7.2.win64.zip) | **Recover by file signature.** Recover recognizable file contents to a separate device when directory metadata cannot be used. | Free and open source | [Source 1](https://www.cgsecurity.org/wiki/PhotoRec) |
| [TestDisk](https://www.cgsecurity.org/wiki/TestDisk) · [Download Windows x64 tools · ZIP](https://www.cgsecurity.org/testdisk-7.2.win64.zip) | **Undelete supported files.** Try supported filesystem undelete on healthy media or a copy when retaining names is possible. | Free and open source | [Source 1](https://www.cgsecurity.org/wiki/TestDisk:_undelete_file_for_NTFS) |

### 🐢 Fix a slow PC

[Quick answer](https://github.com/Herbertofury/INTA/wiki/Fix-a-slow-PC) · [Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-slow-pc)

Expose the particular process and I/O tools needed to explain a symptom while preserving the full suite. Check storage warnings before treating the case as an ordinary performance problem.

| Tool and exact action | Role and reason | Access boundary | Primary evidence |
| --- | --- | --- | --- |
| [Process Explorer](https://learn.microsoft.com/en-us/sysinternals/downloads/process-explorer) · [Download Windows portable · ZIP](https://download.sysinternals.com/files/ProcessExplorer.zip) | **Find the busy process.** Identify the process responsible for sustained CPU, memory or handle pressure before changing software. | Free Microsoft utility | [Source 1](https://learn.microsoft.com/en-us/sysinternals/downloads/process-explorer) |
| [Process Monitor](https://learn.microsoft.com/en-us/sysinternals/downloads/procmon) · [Download Windows portable · ZIP](https://download.sysinternals.com/files/ProcessMonitor.zip) | **Explain an I/O stall.** Capture a short filtered trace of the reproducible file or registry delay. | Free Microsoft utility | [Source 1](https://learn.microsoft.com/en-us/sysinternals/downloads/procmon) |
| [CrystalDiskInfo](https://crystalmark.info/en/software/crystaldiskinfo/) · [Get Windows ZIP · 9.9.5](https://sourceforge.net/projects/crystaldiskinfo/files/9.9.5/CrystalDiskInfo9_9_5.zip/download) | **Check storage warnings.** Check supported drive-health indicators before ordinary performance repair or stress tests. | Free and open source | [Source 1](https://crystalmark.info/en/software/crystaldiskinfo/) |
| [Microsoft Sysinternals Suite](https://learn.microsoft.com/en-us/sysinternals/) · [Download Windows portable · ZIP](https://download.sysinternals.com/files/SysinternalsSuite.zip) | **Complete diagnostic kit.** Keep the full utility suite available when the next evidence-backed branch needs another component. | Free Microsoft utilities | [Source 1](https://learn.microsoft.com/en-us/sysinternals/downloads/) |

### 🖥️ Help remotely

[Quick answer](https://github.com/Herbertofury/INTA/wiki/Help-remotely) · [Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-remote-support)

Choose by attended [Windows](https://www.microsoft.com/windows) help, cross-platform support, an existing managed fleet, or a [Linux](https://www.kernel.org/) technician client. Existing paid commercial products remain in the catalog; their personal-use grants are not assumed for INTA service work.

| Tool and exact action | Role and reason | Access boundary | Primary evidence |
| --- | --- | --- | --- |
| [Microsoft Quick Assist](https://learn.microsoft.com/en-us/windows/client-management/client-tools/quick-assist) · [Get Quick Assist in Microsoft Store](https://apps.microsoft.com/detail/9P7BP5VNWKX5) | **Attended [Windows](https://www.microsoft.com/windows) help.** Use a short-lived approved session with a user present to share and authorize control. | Included app; helper Microsoft account required. | [Source 1](https://learn.microsoft.com/en-us/windows/client-management/client-tools/quick-assist) |
| [RustDesk](https://rustdesk.com/) · [Download Windows x64 · EXE](https://github.com/rustdesk/rustdesk/releases/download/1.5.3/rustdesk-1.5.3-x86_64.exe) | **Cross-platform support.** Use the approved client/server configuration for cross-platform desktop support. | Open-source components; Pro and hosting costs vary. | [Source 1](https://rustdesk.com/docs/en/) |
| [MeshCentral](https://meshcentral.com/) · [Open server installation instructions](https://github.com/Ylianst/MeshCentral#installation) | **Existing managed fleet.** Use the organization’s existing server and enrolled device for managed access. | Open source; hosting and operation are separate. | [Source 1](https://github.com/Ylianst/MeshCentral) |
| [Remmina](https://remmina.org/) · [Install Linux package · Flatpak or distribution](https://www.remmina.org/how-to-install-remmina/) | **[Linux](https://www.kernel.org/) remote client.** Connect a [Linux](https://www.kernel.org/) technician workstation to an already configured RDP, VNC or SSH service. | Free and open source | [Source 1](https://www.remmina.org/how-to-install-remmina/) · [Source 2](https://remmina.org/) |

### 🔄 Fix [Windows](https://www.microsoft.com/windows) updates

[Quick answer](https://github.com/Herbertofury/INTA/wiki/Fix-Windows-updates) · [Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-windows-update)

Differentiate the ordinary update diagnostic, a failed feature-upgrade log analysis, [component-store repair](https://github.com/Herbertofury/INTA/wiki/Job-windows-repair) and a supported same-version reinstall. New app packages do not repair the operating-system update stack.

| Tool and exact action | Role and reason | Access boundary | Primary evidence |
| --- | --- | --- | --- |
| [Windows Get Help](https://support.microsoft.com/en-us/support/get-help/about-get-help) · [Open Windows Update troubleshooting](https://support.microsoft.com/en-us/windows/experience/windows-update-troubleshooter) | **Update diagnostic.** Start with the current [Windows Update](https://learn.microsoft.com/en-us/windows/deployment/update/) diagnostic and record the exact error. | Included with supported [Windows](https://www.microsoft.com/windows) | [Source 1](https://support.microsoft.com/en-us/windows/experience/windows-update-troubleshooter) |
| [SetupDiag](https://learn.microsoft.com/en-us/windows/deployment/upgrade/setupdiag) · [Download Windows upgrade analyzer · EXE](https://go.microsoft.com/fwlink/?linkid=870142) | **Explain an upgrade rollback.** Analyze preserved setup logs for the failed upgrade phase and matching failure rule. | Free Microsoft diagnostic | [Source 1](https://learn.microsoft.com/en-us/windows/deployment/upgrade/setupdiag) |
| [DISM](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image) · [Open built-in repair instructions](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image) | **[Component-store repair](https://github.com/Herbertofury/INTA/wiki/Job-windows-repair).** Use the supported servicing workflow when update evidence points to component-store damage. | Included with supported [Windows](https://www.microsoft.com/windows) | [Source 1](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image) |
| [Windows Recovery Environment](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference) · [Reinstall current Windows via Windows Update](https://support.microsoft.com/en-us/windows/deployment/install-upgrade/fix-issues-by-reinstalling-the-current-version-of-windows) | **Same-version reinstall.** Use the [Windows Update](https://learn.microsoft.com/en-us/windows/deployment/update/) repair reinstall if available and the machine can still run [Windows](https://www.microsoft.com/windows). | Included with supported [Windows](https://www.microsoft.com/windows) | [Source 1](https://support.microsoft.com/en-us/windows/deployment/install-upgrade/fix-issues-by-reinstalling-the-current-version-of-windows) |

### 🧰 Repair [Windows](https://www.microsoft.com/windows) files

[Quick answer](https://github.com/Herbertofury/INTA/wiki/Repair-Windows-files) · [Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-windows-repair)

Keep [component-store repair](https://github.com/Herbertofury/INTA/wiki/Job-windows-repair) and protected-file verification as separate operations, with the supported same-version reinstall available when appropriate. Do not invent a universal third-party [Windows repair](https://github.com/Herbertofury/INTA/wiki/Job-windows-repair) command.

| Tool and exact action | Role and reason | Access boundary | Primary evidence |
| --- | --- | --- | --- |
| [DISM](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image) · [Open built-in repair instructions](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image) | **Repair component store.** Repair the supported running or explicitly identified offline image using a matching source. | Included with supported [Windows](https://www.microsoft.com/windows) | [Source 1](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image) |
| [System File Checker](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc) · [Open built-in system-file checks](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc) | **Verify protected files.** Check and repair protected system files after the component store is healthy. | Included with supported [Windows](https://www.microsoft.com/windows) | [Source 1](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc) |
| [Windows Recovery Environment](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference) · [Reinstall current Windows via Windows Update](https://support.microsoft.com/en-us/windows/deployment/install-upgrade/fix-issues-by-reinstalling-the-current-version-of-windows) | **Same-version reinstall.** Use the supported current-version repair path when ordinary servicing does not resolve the fault. | Included with supported [Windows](https://www.microsoft.com/windows) | [Source 1](https://support.microsoft.com/en-us/windows/deployment/install-upgrade/fix-issues-by-reinstalling-the-current-version-of-windows) |

### 🚀 Fix a PC that will not boot

[Quick answer](https://github.com/Herbertofury/INTA/wiki/Fix-a-PC-that-will-not-boot) · [Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-boot-recovery)

Keep local recovery, official installation media and [image-first recovery](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive) accessible. Cloud recovery stays conditional on an eligible configured device and an available matching remedy; preview rollback features are not the default.

| Tool and exact action | Role and reason | Access boundary | Primary evidence |
| --- | --- | --- | --- |
| [Windows Recovery Environment](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference) · [Open recovery instructions](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference) | **[Startup recovery](https://github.com/Herbertofury/INTA/wiki/Job-boot-recovery).** Use the least disruptive applicable recovery option after preserving data and recovery keys. | Included with supported [Windows](https://www.microsoft.com/windows) | [Source 1](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference) |
| [Windows 11 installation media](https://www.microsoft.com/software-download/windows11) · [Download media creation tool · EXE](https://go.microsoft.com/fwlink/?linkid=2156295) | **Trusted repair media.** Create official matching-architecture media on a working PC when local recovery is unavailable. | Media is free; [Windows](https://www.microsoft.com/windows) licensing remains separate. | [Source 1](https://www.microsoft.com/software-download/windows11) |
| [SystemRescue](https://www.system-rescue.org/) · [Download x64 rescue image · ISO](https://fastly-cdn.system-rescue.org/releases/13.03/systemrescue-13.03-amd64.iso) | **Preserve unstable storage.** Use a separate recovery environment for image-first work when the disk may be failing. | Free and open source | [Source 1](https://www.system-rescue.org/manual/) |
| [Quick machine recovery](https://learn.microsoft.com/en-us/windows/configuration/quick-machine-recovery/?tabs=csp) · [Check supported recovery setup](https://learn.microsoft.com/en-us/windows/configuration/quick-machine-recovery/?tabs=csp) | **Eligible widespread failure.** Check the configured cloud recovery option on a supported device when a matching widespread boot remedy exists. | Included on eligible, configured [Windows](https://www.microsoft.com/windows) devices. | [Source 1](https://learn.microsoft.com/en-us/windows/configuration/quick-machine-recovery/?tabs=csp) |

### 🧩 Diagnose crashes

[Quick answer](https://github.com/Herbertofury/INTA/wiki/Diagnose-crashes) · [Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-hardware-bsod)

Use a dump debugger for recorded crashes, a bootable test for suspected memory faults and sensor observation for a reproduced thermal/power question. A single sensor reading or guessed driver name does not settle the cause.

| Tool and exact action | Role and reason | Access boundary | Primary evidence |
| --- | --- | --- | --- |
| [WinDbg](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/) · [Download Windows debugger · APPINSTALLER](https://aka.ms/windbg/download) | **Analyze a crash dump.** Inspect the recorded dump and stack before blaming a driver or replacing hardware. | Free Microsoft debugger | [Source 1](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/) |
| [Memtest86+](https://www.memtest.org/) · [Download x64 memory-test ISO · ZIP](https://www.memtest.org/download/v9.00/mt86plus_9.00_x86_64.iso.zip) | **Test memory stability.** Boot a memory test when evidence points to RAM or its configuration. | Free and open source | [Source 1](https://github.com/memtest86plus/memtest86plus) |
| [Libre Hardware Monitor](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor) · [Download Windows portable · ZIP](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor/releases/download/v0.9.6/LibreHardwareMonitor.zip) | **Observe temperatures.** Log relevant sensors during the reproduced workload and validate labels against the hardware. | Free and open source | [Source 1](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor) |

### 📊 Free disk space

[Quick answer](https://github.com/Herbertofury/INTA/wiki/Free-disk-space) · [Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-slow-pc)

Retain the open-source capacity map and expose the licensed alternative explicitly. No benchmark was run here, so the review does not claim a measured speed winner or treat a large file as permission to delete it.

| Tool and exact action | Role and reason | Access boundary | Primary evidence |
| --- | --- | --- | --- |
| [WinDirStat](https://windirstat.net/) · [Download Windows x64 · MSI](https://github.com/windirstat/windirstat/releases/download/release/v2.9.3/WinDirStat-x64.msi) | **Free capacity map.** Inspect directory and file-type usage on healthy storage before choosing a supported cleanup. | Free and open source | [Source 1](https://github.com/windirstat/windirstat) |
| [WizTree](https://diskanalyzer.com/) · [Download Windows portable · ZIP](https://diskanalyzer.com/files/wiztree_4_34_portable.zip) | **Licensed capacity analysis.** Use its supported disk analysis when the technician has the appropriate commercial entitlement. | Personal use is free; business use requires a license. | [Source 1](https://diskanalyzer.com/) · [Source 2](https://diskanalyzer.com/eula) |

### 🔁 Clone or move a drive

[Quick answer](https://github.com/Herbertofury/INTA/wiki/Clone-or-move-a-drive) · [Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-disk-migration)

Offer a graphical image workflow, the more advanced imaging workflow and a files-only copy path. Folder synchronization remains elsewhere in the toolkit because it is not an interchangeable whole-disk clone or backup proof.

| Tool and exact action | Role and reason | Access boundary | Primary evidence |
| --- | --- | --- | --- |
| [Rescuezilla](https://rescuezilla.com/) · [Download x64 rescue image · ISO](https://github.com/rescuezilla/rescuezilla/releases/download/2.6.2/rescuezilla-2.6.2-64bit.resolute.iso) | **Graphical image or clone.** Use a guided source/destination workflow for healthy storage after checking disk identity. | Free and open source | [Source 1](https://rescuezilla.com/help) |
| [Clonezilla](https://clonezilla.org/) · [Get x64 rescue image · ISO](https://downloads.sourceforge.net/clonezilla/clonezilla-live-3.3.5-3-amd64.iso) | **Advanced image workflow.** Use the documented imaging or cloning mode when its filesystem and destination constraints fit. | Free and open source | [Source 1](https://clonezilla.org/clonezilla-live-doc.php) |
| [Robocopy](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy) · [Open built-in copy instructions](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy) | **Files-only migration.** Copy the agreed file trees with logs when an entire disk image is unnecessary. | Included with supported [Windows](https://www.microsoft.com/windows) | [Source 1](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy) |

### 🗄️ Restore a file backup

[Quick answer](https://github.com/Herbertofury/INTA/wiki/Restore-a-file-backup) · [Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-backup-restore)

The existing repository format selects the restore tool. Keep native restore routes for the three supported backup families; a newer application cannot simply open another product’s backup format.

| Tool and exact action | Role and reason | Access boundary | Primary evidence |
| --- | --- | --- | --- |
| [restic](https://restic.net/) · [Download Windows x64 · ZIP](https://github.com/restic/restic/releases/download/v0.20.0/restic_0.20.0_windows_amd64.zip) | **Restore a [restic](https://restic.net/) repository.** Use its native snapshot and restore commands for an existing [restic](https://restic.net/) backup. | Free software; repository storage is separate. | [Source 1](https://restic.readthedocs.io/en/stable/050_restore.html) · [Source 2](https://restic.readthedocs.io/en/stable/045_working_with_repos.html) |
| [Kopia](https://kopia.io/) · [Download Windows desktop · ZIP](https://github.com/kopia/kopia/releases/download/v0.23.1/KopiaUI-0.23.1-win.zip) | **Restore a [Kopia](https://kopia.io/) snapshot.** Open the existing repository and restore the selected snapshot into a separate location. | Free software; repository storage is separate. | [Source 1](https://kopia.io/docs/) |
| [Veeam Agent for Microsoft Windows](https://www.veeam.com/products/free/microsoft-windows) · [Sign in for the Windows download](https://www.veeam.com/products/physical/windows-backup-recovery-download.html) | **Restore an agent backup.** Use the matching agent recovery workflow for an existing supported [Windows](https://www.microsoft.com/windows) backup. | Free edition exists; service-provider terms differ. | [Source 1](https://helpcenter.veeam.com/docs/agentforwindows/userguide/files_restore.html) · [Source 2](https://www.veeam.com/legal/licensing-policy.html) |

### 🔌 Make a boot USB

[Quick answer](https://github.com/Herbertofury/INTA/wiki/Make-a-boot-USB) · [Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-boot-usb)

Choose a single-image writer or a multiboot loader, with actual official [Windows](https://www.microsoft.com/windows) and recovery media directly available. Preserve target-drive identification and actual boot verification.

| Tool and exact action | Role and reason | Access boundary | Primary evidence |
| --- | --- | --- | --- |
| [Rufus](https://rufus.ie/) · [Download Windows x64 portable · EXE](https://github.com/pbatard/rufus/releases/download/v4.16/rufus-4.16p.exe) | **Write one official ISO.** Write the selected supported image to the verified USB and test the actual target boot. | Free and open source | [Source 1](https://github.com/pbatard/rufus/wiki/FAQ) |
| [Ventoy](https://www.ventoy.net/) · [Download Windows package · ZIP](https://github.com/ventoy/Ventoy/releases/download/v1.1.18/ventoy-1.1.18-windows.zip) | **Maintain multiple ISOs.** Install the multiboot loader on the verified USB and keep each image independently sourced and tested. | Free and open source | [Source 1](https://www.ventoy.net/en/doc_start.html) |
| [Windows 11 installation media](https://www.microsoft.com/software-download/windows11) · [Download media creation tool · EXE](https://go.microsoft.com/fwlink/?linkid=2156295) | **Official [Windows](https://www.microsoft.com/windows) media.** Obtain the supported [Windows](https://www.microsoft.com/windows) installer or media-creation route for the target architecture. | Media is free; [Windows](https://www.microsoft.com/windows) licensing remains separate. | [Source 1](https://www.microsoft.com/software-download/windows11) |
| [SystemRescue](https://www.system-rescue.org/) · [Download x64 rescue image · ISO](https://fastly-cdn.system-rescue.org/releases/13.03/systemrescue-13.03-amd64.iso) | **Ready recovery ISO.** Add an independent recovery environment when the job requires disk imaging or recovery. | Free and open source | [Source 1](https://www.system-rescue.org/manual/) |

### 💻 Set up a new PC

[Quick answer](https://github.com/Herbertofury/INTA/wiki/Set-up-a-new-PC) · [Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-new-pc)

Use exact approved package IDs, manufacturer-supported drivers and a logged agreed-file copy. Avoid a generic debloat/driver-updater bundle as a universal setup recipe.

| Tool and exact action | Role and reason | Access boundary | Primary evidence |
| --- | --- | --- | --- |
| [WinGet](https://learn.microsoft.com/en-us/windows/package-manager/winget/) · [Download App Installer · MSIXBUNDLE](https://github.com/microsoft/winget-cli/releases/download/v1.29.380/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle) | **Approved application setup.** Identify exact packages and sources before installing the agreed applications. | Included App Installer; each app keeps its own license. | [Source 1](https://learn.microsoft.com/en-us/windows/package-manager/winget/) |
| [Official OEM driver and firmware portals](https://support.microsoft.com/en-us/windows/hardware/drivers/update-drivers-through-device-manager-in-windows) · [Find the correct driver source](https://support.microsoft.com/en-us/windows/hardware/drivers/update-drivers-through-device-manager-in-windows) | **Model-specific drivers.** Use the device manufacturer’s supported drivers and firmware for the exact model and OS. | Vendor downloads; support terms vary. | [Source 1](https://support.microsoft.com/en-us/windows/hardware/drivers/update-drivers-through-device-manager-in-windows) |
| [Robocopy](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy) · [Open built-in copy instructions](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy) | **Move agreed user files.** Copy the approved data with an inspectable log and verify it as the target user. | Included with supported [Windows](https://www.microsoft.com/windows) | [Source 1](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy) |

### 🛠️ Maintain a PC

[Quick answer](https://github.com/Herbertofury/INTA/wiki/Maintain-a-PC) · [Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-preventive-maintenance)

Separate application update review, storage-health review and capacity inspection. The retained full kit supports deeper follow-up after one of these checks identifies a real issue.

| Tool and exact action | Role and reason | Access boundary | Primary evidence |
| --- | --- | --- | --- |
| [WinGet](https://learn.microsoft.com/en-us/windows/package-manager/winget/) · [Download App Installer · MSIXBUNDLE](https://github.com/microsoft/winget-cli/releases/download/v1.29.380/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle) | **Review app updates.** Review package identity, source and available upgrades before approved changes. | Included App Installer; each app keeps its own license. | [Source 1](https://learn.microsoft.com/en-us/windows/package-manager/winget/) |
| [CrystalDiskInfo](https://crystalmark.info/en/software/crystaldiskinfo/) · [Get Windows ZIP · 9.9.5](https://sourceforge.net/projects/crystaldiskinfo/files/9.9.5/CrystalDiskInfo9_9_5.zip/download) | **Review storage health.** Check supported health indicators and route new warnings to recovery before cleanup. | Free and open source | [Source 1](https://crystalmark.info/en/software/crystaldiskinfo/) |
| [WinDirStat](https://windirstat.net/) · [Download Windows x64 · MSI](https://github.com/windirstat/windirstat/releases/download/release/v2.9.3/WinDirStat-x64.msi) | **Explain capacity use.** Identify where space went on healthy storage without assuming large files are disposable. | Free and open source | [Source 1](https://github.com/windirstat/windirstat) |

### 🧭 Start a repair

[Quick answer](https://github.com/Herbertofury/INTA/wiki/Start-a-repair) · [Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-intake-triage)

No extra application is required to establish the symptom, device identity, backup state and scope. Existing diagnostic tools remain in the job kit for the branch selected by that intake.

**Starting action:** Record the exact symptom, affected device and desired result before making changes.

**Source basis:** [Reference 1](https://github.com/Herbertofury/INTA/wiki/Job-intake-triage) · [Reference 2](https://support.microsoft.com/en-us/windows/experience/backup-recovery/recovery-options-in-windows).

### 🚨 Contain ransomware

[Quick answer](https://github.com/Herbertofury/INTA/wiki/Contain-ransomware) · [Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-ransomware-response)

The first response is isolation, evidence preservation, ownership and recovery protection. Installing a new cleaner or telemetry agent first can consume time or change evidence; the incident workflow determines later tooling.

**Starting action:** Isolate affected connections and alert the incident owner immediately if files are being encrypted or compromise is spreading.

**Source basis:** [Reference 1](https://www.cisa.gov/sites/default/files/2025-03/StopRansomware-Guide%20508.pdf) · [Reference 2](https://learn.microsoft.com/en-us/security/compass/incident-response-playbook-dart-ransomware-approach).

### 🏢 Recover an office outage

[Quick answer](https://github.com/Herbertofury/INTA/wiki/Recover-an-office-outage) · [Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-outage-recovery)

Begin with safe power, ownership and dependency scope. Existing operating-system and service tools in the working kit support the affected service; an unrelated downloaded repair utility is not a prerequisite.

**Starting action:** Confirm safe power and the affected business services, then assign one recovery owner.

**Source basis:** [Reference 1](https://github.com/Herbertofury/INTA/wiki/Job-outage-recovery) · [Reference 2](https://learn.microsoft.com/en-us/troubleshoot/windows-client/performance/event-id-41-restart).

### 🔐 Repair domain sign-in

[Quick answer](https://github.com/Herbertofury/INTA/wiki/Repair-domain-sign-in) · [Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-ad-trust)

Retain existing [Windows PowerShell 5.1](https://learn.microsoft.com/en-us/powershell/scripting/windows-powershell/starting-windows-powershell) and the documented member-computer cmdlet. A newer shell, blanket domain rejoin or local-administrator management product is not automatically the repair for a failed secure channel.

| Tool and exact action | Role and reason | Access boundary | Primary evidence |
| --- | --- | --- | --- |
| [PowerShell](https://learn.microsoft.com/en-us/powershell/) · [Use Windows PowerShell 5.1](https://learn.microsoft.com/en-us/powershell/scripting/windows-powershell/starting-windows-powershell) | **Member secure-channel test.** Use existing [Windows PowerShell 5.1](https://learn.microsoft.com/en-us/powershell/scripting/windows-powershell/starting-windows-powershell) on the domain member after DNS, time and connectivity checks. | Included with supported [Windows](https://www.microsoft.com/windows) | [Source 1](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.management/test-computersecurechannel?view=powershell-5.1) |

### 💡 Troubleshoot with AI

[Quick answer](https://github.com/Herbertofury/INTA/wiki/Troubleshoot-with-AI) · [Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-ai-triage)

Separate evidence-driven hypothesis assistance from a scoped code/script agent. Both need organization-approved input, human review and verification against the real target; no model benchmark or autonomous-repair certification is claimed.

| Tool and exact action | Role and reason | Access boundary | Primary evidence |
| --- | --- | --- | --- |
| [ChatGPT](https://chatgpt.com/) · [Open ChatGPT](https://chatgpt.com/) | **Compare hypotheses.** Use approved redacted evidence to request competing explanations and one discriminating test. | Free tier available; account and organization limits apply. | [Source 1](https://help.openai.com/en/articles/9275245-using-chatgpt-s-free-tier-faq) · [Source 2](https://help.openai.com/en/articles/7730893-data-controls-in-chatgpt) |
| [OpenAI Codex CLI](https://github.com/openai/codex) · [Download Windows x64 CLI · EXE](https://github.com/openai/codex/releases/download/rust-v0.172.0/codex-x86_64-pc-windows-msvc.exe) | **Review a scoped script.** Review or test an authorized code change inside a bounded workspace with execution controls. | Open-source client; account/API usage terms apply. | [Source 1](https://github.com/openai/codex) · [Source 2](https://developers.openai.com/codex/security/) |

### ✅ Finish the job and hand over

[Quick answer](https://github.com/Herbertofury/INTA/wiki/Finish-the-job-and-hand-over) · [Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-client-handoff)

No additional download is needed to prove the original task, remove case-only access and establish ownership. The existing handoff template and working kit remain available.

**Starting action:** Repeat the original user task and collect acceptance evidence before closing the case.

**Source basis:** [Reference 1](https://github.com/Herbertofury/INTA/wiki/Job-client-handoff) · [Reference 2](https://github.com/Herbertofury/INTA/wiki/Technician-Handoff).

### 🔎 Find a file

[Quick answer](https://github.com/Herbertofury/INTA/wiki/Find-a-file) · [Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-preventive-maintenance)

Keep the free local filename index and expose the alternative with its commercial-use boundary. Search results require path/time validation and do not establish that a file is safe or disposable.

| Tool and exact action | Role and reason | Access boundary | Primary evidence |
| --- | --- | --- | --- |
| [Everything](https://www.voidtools.com/) · [Download Windows x64 EXE](https://www.voidtools.com/Everything-1.4.1.1032.x64-Setup.exe) | **Find a local filename.** Search the approved local index by name, then confirm path and modification time. | Free desktop edition, including business use. | [Source 1](https://www.voidtools.com/faq/) · [Source 2](https://www.voidtools.com/License.txt) |
| [WizFile](https://antibody-software.com/wizfile/) · [Download Windows portable · ZIP](https://antibody-software.com/files/wizfile_3_15_portable.zip) | **Licensed file search.** Use the supported file-search workflow when its filters and the technician’s license fit the task. | Personal use is free; business use requires a license. | [Source 1](https://antibody-software.com/wizfile/quick-start) · [Source 2](https://antibody-software.com/wizfile/eula) |

### 🌐 Inspect network traffic

[Quick answer](https://github.com/Herbertofury/INTA/wiki/Inspect-network-traffic) · [Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-network-dns)

Choose a scoped packet trace, a targeted HTTP/TLS request or a supported path comparison based on the unresolved layer. The path tool is not presented as a native [Windows](https://www.microsoft.com/windows) executable.

| Tool and exact action | Role and reason | Access boundary | Primary evidence |
| --- | --- | --- | --- |
| [Wireshark](https://www.wireshark.org/) · [Download Windows x64 · EXE](https://2.na.dl.wireshark.org/win64/Wireshark-4.6.10-x64.exe) | **Protocol evidence.** Capture the smallest authorized exchange that reproduces the unresolved failure. | Free; [Windows](https://www.microsoft.com/windows) capture-driver terms apply. | [Source 1](https://www.wireshark.org/docs/wsug_html_chunked/ChapterIntroduction.html) · [Source 2](https://npcap.com/guide/index.html) |
| [curl](https://curl.se/) · [Use the Windows-included curl.exe](https://curl.se/windows/microsoft.html) | **HTTP/TLS comparison.** Inspect the specific HTTP or TLS failure when a full packet capture is unnecessary. | Included with supported [Windows](https://www.microsoft.com/windows) | [Source 1](https://curl.se/docs/manpage.html) |
| [mtr](https://www.bitwizard.nl/mtr/) · [Install a Linux/WSL distribution package](https://github.com/traviscross/mtr#using-mtr-on-windows) | **Path-loss comparison.** Compare a reproducible path problem from a supported [Linux](https://www.kernel.org/) or [macOS](https://support.apple.com/macos) bench or a configured WSL environment. | Free and open source | [Source 1](https://github.com/traviscross/mtr) |

### 🧪 Test memory

[Quick answer](https://github.com/Herbertofury/INTA/wiki/Test-memory) · [Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-hardware-bsod)

Retain the open-source bootable memory test for this narrow case. Dump analysis is available under the broader crash case; a second memory product is not needed merely to fill a list.

| Tool and exact action | Role and reason | Access boundary | Primary evidence |
| --- | --- | --- | --- |
| [Memtest86+](https://www.memtest.org/) · [Download x64 memory-test ISO · ZIP](https://www.memtest.org/download/v9.00/mt86plus_9.00_x86_64.iso.zip) | **Bootable memory test.** Test the documented hardware configuration and use module/slot comparisons to isolate repeatable errors. | Free and open source | [Source 1](https://www.memtest.org/) · [Source 2](https://github.com/memtest86plus/memtest86plus) |

## Evidence limits and maintenance

Publisher documentation and current observed acquisition routes support these role and compatibility decisions. File headers, response status and release metadata validate a route at the stated review time; they do not prove malware-detection superiority, a successful physical repair or compatibility with an untested customer device. Malware lab results apply only to the exact tested product, edition, platform and test period. The separate [malware review](https://github.com/Herbertofury/INTA/wiki/Malware-Tool-Choices) preserves those boundaries.

Keep new findings under the existing scenario and tool identity. Add a distinct role when it solves a demonstrated gap; retain useful alternatives and earlier primary evidence. Update a review date only after the relevant sources and selection conditions have been checked.
