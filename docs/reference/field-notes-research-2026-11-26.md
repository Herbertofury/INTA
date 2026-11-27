# Specific fault research — November 26, 2026

[Service map](https://github.com/Herbertofury/INTA/wiki/Service-Map) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Current tool choices](https://github.com/Herbertofury/INTA/wiki/Current-Tool-Choices)

**74 practical fault cases across all 22 service jobs.** Each case adds a concrete first action, three follow-up steps, an observable result, a fallback, and its supporting primary sources. Open a case below from its complete job guide; the working view also indexes each specific fix. The canonical records are in [field-notes.json](https://github.com/Herbertofury/INTA/blob/main/data/field-notes.json).

## Scope and evidence

**Editorial review: November 26, 2026.** This is a documentation review of specific operating procedures. It does not certify that a repair ran on a customer machine, that a download is safe in every environment, or that one product ranks above every alternative. Platform, identity, licensing and physical-device conditions still determine which branch applies. Existing guide, tool and release dates are preserved.

The sources below are publisher documentation, official project repositories or public cybersecurity guidance. Links were checked separately from their technical meaning; a successful HTTP response is evidence of reachability only. Moved support pages were replaced with observed current destinations. The CISA guide is linked through the current observed PDF route; its publication date is distinct from this review date.

## Decisions that change the repair

| Finding | Practical consequence | Primary evidence |
| --- | --- | --- |
| A notification can be a website permission rather than an installed infection. | Identify the sender and block that exact site; use the malware procedure when other compromise evidence remains. | [Browser notifications](https://support.microsoft.com/en-us/edge/manage-website-notifications-in-microsoft-edge) |
| An account and an endpoint can remain compromised independently. | Contain identity access from a clean device and preserve account evidence while the endpoint investigation continues. | [Revoke access](https://learn.microsoft.com/en-us/entra/identity/users/users-revoke-access) · [Compromised accounts](https://learn.microsoft.com/en-us/defender-office-365/responding-to-a-compromised-email-account) |
| Recovery writes can remove the chance of a later recovery. | Preserve failing sources and their image/mapfile identity; recover output elsewhere and test decryptors only on copies in a clean isolated environment. | [GNU ddrescue manual](https://www.gnu.org/software/ddrescue/manual/ddrescue_manual.html) · [No More Ransom decryptor instructions](https://www.nomoreransom.org/en/decryption-tools.html) |
| A package inventory, a cloud placeholder and a repository integrity check prove different things. | Verify application data separately, materialize needed cloud files, and restore representative files into a separate location. | [Package export](https://learn.microsoft.com/en-us/windows/package-manager/winget/export) · [Files On-Demand](https://support.microsoft.com/en-gb/onedrive/save-disk-space-with-onedrive-files-on-demand-for-windows) · [Repository checks](https://restic.readthedocs.io/en/stable/045_working_with_repos.html) |
| Driver architecture and protected print mode affect the available print path. | Identify the real system architecture and print policy before choosing a driver. Test scanning separately from printing. | [ARM compatibility](https://support.microsoft.com/en-us/windows/experience/platform-variants/windows-arm-based-pcs-faq) · [Protected print mode](https://learn.microsoft.com/en-us/windows/modern-print/windows-protected-print-mode/windows-protected-print-mode) · [Scanner driver choices](https://www.naps2.com/windows-scanning) |
| A supported diagnostic is not automatically a repair. | Use [SetupDiag](https://learn.microsoft.com/en-us/windows/deployment/upgrade/setupdiag) to explain setup logs, [USB Device Tree Viewer](https://www.uwe-sieber.de/usbtreeview_e.html) to inspect topology, and an ordinary [SumatraPDF](https://www.sumatrapdfreader.org/) print comparison to isolate a document problem. No preview-only printing flags are assumed. | [SetupDiag](https://learn.microsoft.com/en-us/windows/deployment/upgrade/setupdiag) · [USB Device Tree Viewer](https://www.uwe-sieber.de/usbtreeview_e.html) · [SumatraPDF documentation](https://www.sumatrapdfreader.org/docs/SumatraPDF-documentation) |
| Evidence sent to an AI system may contain secrets or hostile instructions. | Redact locally, share only the approved excerpt, and treat instructions inside logs or retrieved pages as untrusted content. | [Data controls](https://help.openai.com/en/articles/7730893-data-controls-in-chatgpt) · [Agent security](https://developers.openai.com/codex/security/) |

## Coverage

| Service job | New cases | Specific problems added |
| --- | ---: | --- |
| [Start here: triage any computer problem](https://github.com/Herbertofury/INTA/wiki/Job-intake-triage) | 3 | Choose the correct model and architecture; Find the background program blocking an app; Correct a bad clock before chasing sign-in errors |
| [Remove malware and browser hijackers](https://github.com/Herbertofury/INTA/wiki/Job-malware-removal) | 3 | Stop website notification popups; Undo an accidental allowed-threat decision; Find why the offline scan never runs |
| [Contain ransomware or a wider compromise](https://github.com/Herbertofury/INTA/wiki/Job-ransomware-response) | 3 | Protect backups before testing a restore; Contain stolen account access as well as the PC; Evaluate a decryptor without risking the evidence |
| [Diagnose a slow, freezing, or hitching PC](https://github.com/Herbertofury/INTA/wiki/Job-slow-pc) | 4 | Separate useful cache from memory pressure; Find the optional app delaying sign-in; Find what an app is waiting for; Explain a laptop that slows down on battery |
| [Repair Windows component or system-file corruption](https://github.com/Herbertofury/INTA/wiki/Job-windows-repair) | 3 | Resolve a missing [DISM](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image) repair source; Identify what [SFC](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc) could not repair; Repair one broken app without resetting the PC |
| [Fix a failing Windows update](https://github.com/Herbertofury/INTA/wiki/Job-windows-update) | 3 | Clear an update’s storage blocker safely; Find why a feature upgrade rolled back; Check why a feature update is not offered |
| [Recover a Windows PC that will not boot](https://github.com/Herbertofury/INTA/wiki/Job-boot-recovery) | 3 | Find a disk missing from recovery media; Identify the [Windows](https://www.microsoft.com/windows) volume inside recovery; Find the matching [BitLocker](https://learn.microsoft.com/en-us/windows/security/operating-system-security/data-protection/bitlocker/) key |
| [Rescue data from a failing or unreadable drive](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive) | 3 | Try filesystem undelete before file carving; Resume imaging without swapping source and target; Validate carved files without trusting their filenames |
| [Clone a healthy system or migrate to a new SSD](https://github.com/Herbertofury/INTA/wiki/Job-disk-migration) | 3 | Use remaining space after a successful clone; Read a copy result correctly; Finish what an app-list import cannot migrate |
| [Prove a backup can actually restore](https://github.com/Herbertofury/INTA/wiki/Job-backup-restore) | 4 | Include cloud-only files in the recovery plan; Check repository data as well as metadata; Recover an older version without overwriting current work; Prove the backup can be opened after the PC is lost |
| [Fix internet, DNS, or local-network problems](https://github.com/Herbertofury/INTA/wiki/Job-network-dns) | 5 | Fix a PC stuck on a self-assigned address; Restore internal name resolution on VPN; Complete a captive Wi-Fi sign-in; Separate a blocked port from a DNS problem; Locate a throughput bottleneck |
| [Bring an office back after a power outage](https://github.com/Herbertofury/INTA/wiki/Job-outage-recovery) | 3 | Read an unexpected-restart event correctly; Recover a dependent service in the right order; Restore a share after the server returns |
| [Start and finish a clean remote-support session](https://github.com/Herbertofury/INTA/wiki/Job-remote-support) | 3 | Move a required file through an approved route; Restore Mac screen sharing and control; Avoid losing the session during a restart |
| [Repair a workstation’s domain trust relationship](https://github.com/Herbertofury/INTA/wiki/Job-ad-trust) | 3 | Fix domain discovery before changing trust; Restore the domain time path; Check computer identity after restoring an old image |
| [Fix Outlook or email send/receive problems](https://github.com/Herbertofury/INTA/wiki/Job-outlook-email) | 4 | Bring classic [Outlook](https://support.microsoft.com/outlook) back online; Release a message stuck behind a large attachment; Move an active PST out of the sync bottleneck; Find mail missing only from the local cache |
| [Restore printing without breaking other queues](https://github.com/Herbertofury/INTA/wiki/Job-printer-repair) | 5 | Match a stale printer port to the current device; Resolve an incompatible printer-driver package; Fix streaks that occur only through the feeder; Make the scanner appear in the scan profile; Separate a bad document from a broken printer |
| [Diagnose crashes, blue screens, and overheating](https://github.com/Herbertofury/INTA/wiki/Job-hardware-bsod) | 3 | Capture usable evidence for the next crash; Isolate a memory fault after an upgrade; Find a device failing through a USB hub |
| [Set up a dependable new workstation](https://github.com/Herbertofury/INTA/wiki/Job-new-pc) | 3 | Restore network access during device setup; Make the package manager available to the user; Confirm the intended organization device state |
| [Close a repair with a clear client handoff](https://github.com/Herbertofury/INTA/wiki/Job-client-handoff) | 3 | Prove the repair under the real user account; Make recovery information retrievable; Remove access that was only for this repair |
| [Build and refresh a technician rescue USB](https://github.com/Herbertofury/INTA/wiki/Job-boot-usb) | 3 | Resolve a Secure Boot rejection deliberately; Choose media that can boot the target; Handle an image larger than the USB filesystem limit |
| [Use AI as a careful troubleshooting copilot](https://github.com/Herbertofury/INTA/wiki/Job-ai-triage) | 3 | Create a safe diagnostic excerpt; Check a suggested command before running it; Keep a log or webpage from changing the task |
| [Run a useful workstation maintenance visit](https://github.com/Herbertofury/INTA/wiki/Job-preventive-maintenance) | 4 | Use the supported optimization for the drive type; Update an app missing from the package-manager list; Resume a stalled cloud-sync workflow; Find an operating system outside its support plan |

## Case and source index

Each case link opens the exact fix in the working view. The complete Wiki procedure is linked separately under each job heading. The index keeps the first move visible; the linked case contains the remaining steps, expected result and escalation route.

### [Start here: triage any computer problem](https://github.com/Herbertofury/INTA/wiki/Job-intake-triage)

[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-intake-triage)

| Specific fix | First move | Supporting primary sources |
| --- | --- | --- |
| [Choose the correct model and architecture](https://herbertofury.github.io/INTA/#job/intake-triage?section=fix-triage-confirm-model-and-architecture) | Open Settings > System > About and record the system type and [Windows](https://www.microsoft.com/windows) version. | [Microsoft support 1](https://support.microsoft.com/en-us/windows/experience/find-information-about-your-windows-device) · [Microsoft support 2](https://support.microsoft.com/en-us/windows/experience/platform-variants/windows-arm-based-pcs-faq) |
| [Find the background program blocking an app](https://herbertofury.github.io/INTA/#job/intake-triage?section=fix-triage-isolate-background-conflict) | Record the exact reproduction and the currently enabled startup items before changing them. | [Microsoft support 1](https://support.microsoft.com/en-us/windows/experience/startup-boot/how-to-perform-a-clean-boot-in-windows) |
| [Correct a bad clock before chasing sign-in errors](https://herbertofury.github.io/INTA/#job/intake-triage?section=fix-triage-clock-breaks-signin) | Compare the device’s date, time and time zone with a trusted current reference. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/windows-server/networking/windows-time-service/windows-time-service-tools-and-settings) |

### [Remove malware and browser hijackers](https://github.com/Herbertofury/INTA/wiki/Job-malware-removal)

[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-malware-removal)

| Specific fix | First move | Supporting primary sources |
| --- | --- | --- |
| [Stop website notification popups](https://herbertofury.github.io/INTA/#job/malware-removal?section=fix-malware-browser-notification-popups) | Record the sender and close the alert without opening its link. | [Microsoft support 1](https://support.microsoft.com/en-us/edge/manage-website-notifications-in-microsoft-edge) |
| [Undo an accidental allowed-threat decision](https://herbertofury.github.io/INTA/#job/malware-removal?section=fix-malware-allowed-threat-keeps-returning) | Open [Windows Security](https://support.microsoft.com/en-us/windows/security/windows-security/windows-security-app-overview) > Virus & threat protection and review the exact detection and its earlier disposition. | [Microsoft support 1](https://support.microsoft.com/en-us/windows/security/windows-security/protection-history-in-the-windows-security-app) |
| [Find why the offline scan never runs](https://herbertofury.github.io/INTA/#job/malware-removal?section=fix-malware-offline-scan-does-not-start) | Check the device platform and the [Microsoft Defender Offline](https://learn.microsoft.com/en-us/defender-endpoint/microsoft-defender-offline) prerequisites before requesting another restart. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/defender-endpoint/microsoft-defender-offline) · [Microsoft documentation 2](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/reagentc-command-line-options) |

### [Contain ransomware or a wider compromise](https://github.com/Herbertofury/INTA/wiki/Job-ransomware-response)

[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-ransomware-response)

| Specific fix | First move | Supporting primary sources |
| --- | --- | --- |
| [Protect backups before testing a restore](https://herbertofury.github.io/INTA/#job/ransomware-response?section=fix-ransomware-protect-recovery-copies) | Have the incident lead separate backup access from the affected systems using a known-clean administration path. | [CISA guidance 1](https://www.cisa.gov/sites/default/files/2025-03/StopRansomware-Guide%20508.pdf) |
| [Contain stolen account access as well as the PC](https://herbertofury.github.io/INTA/#job/ransomware-response?section=fix-ransomware-compromised-admin-account) | Use a known-clean device and have the identity administrator identify the affected account and scope. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/entra/identity/users/users-revoke-access) · [Microsoft documentation 2](https://learn.microsoft.com/en-us/defender-office-365/responding-to-a-compromised-email-account) |
| [Evaluate a decryptor without risking the evidence](https://herbertofury.github.io/INTA/#job/ransomware-response?section=fix-ransomware-test-decryptor-on-copies) | Preserve the ransom note and encrypted originals, and check the family against the official No More Ransom resources. | [No More Ransom 1](https://www.nomoreransom.org/en/decryption-tools.html) · [CISA guidance 2](https://www.cisa.gov/sites/default/files/2025-03/StopRansomware-Guide%20508.pdf) |

### [Diagnose a slow, freezing, or hitching PC](https://github.com/Herbertofury/INTA/wiki/Job-slow-pc)

[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-slow-pc)

| Specific fix | First move | Supporting primary sources |
| --- | --- | --- |
| [Separate useful cache from memory pressure](https://herbertofury.github.io/INTA/#job/slow-pc?section=fix-slow-pc-standby-memory-is-not-a-leak) | Reproduce the slowdown while watching [Task Manager](https://support.microsoft.com/en-us/windows/experience/system-configuration-tools-in-windows) > Performance > Memory. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/sysinternals/downloads/rammap) |
| [Find the optional app delaying sign-in](https://herbertofury.github.io/INTA/#job/slow-pc?section=fix-slow-pc-startup-app-delay) | Time the same sign-in-to-ready task before making changes. | [Microsoft support 1](https://support.microsoft.com/en-us/windows/experience/startup-boot/configure-startup-applications-in-windows) |
| [Find what an app is waiting for](https://herbertofury.github.io/INTA/#job/slow-pc?section=fix-slow-pc-filter-a-short-io-trace) | Check storage health before recording a short trace on healthy storage. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/sysinternals/downloads/procmon) |
| [Explain a laptop that slows down on battery](https://herbertofury.github.io/INTA/#job/slow-pc?section=fix-slow-pc-only-on-battery) | Repeat a short, identical workload on the approved charger and on battery. | [Microsoft support 1](https://support.microsoft.com/en-us/windows/experience/change-the-power-mode-for-your-windows-pc) |

### [Repair Windows component or system-file corruption](https://github.com/Herbertofury/INTA/wiki/Job-windows-repair)

[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-windows-repair)

| Specific fix | First move | Supporting primary sources |
| --- | --- | --- |
| [Resolve a missing DISM repair source](https://herbertofury.github.io/INTA/#job/windows-repair?section=fix-repair-dism-source-not-found) | Record the exact servicing error, installed [Windows](https://www.microsoft.com/windows) build, architecture and language. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/configure-a-windows-repair-source) · [Microsoft documentation 2](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image) |
| [Identify what SFC could not repair](https://herbertofury.github.io/INTA/#job/windows-repair?section=fix-repair-read-the-current-sfc-result) | Note the start/end time of the latest scan rather than interpreting every old log entry. | [Microsoft support 1](https://support.microsoft.com/en-gb/windows/experience/backup-recovery/use-the-system-file-checker-tool-to-repair-missing-or-corrupted-system-files) |
| [Repair one broken app without resetting the PC](https://herbertofury.github.io/INTA/#job/windows-repair?section=fix-repair-one-app-without-resetting-windows) | Confirm that the same failure is limited to that application and protect its local-only data. | [Microsoft support 1](https://support.microsoft.com/en-us/windows/apps/repair-apps-and-programs-in-windows) |

### [Fix a failing Windows update](https://github.com/Herbertofury/INTA/wiki/Job-windows-update)

[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-windows-update)

| Specific fix | First move | Supporting primary sources |
| --- | --- | --- |
| [Clear an update’s storage blocker safely](https://herbertofury.github.io/INTA/#job/windows-update?section=fix-update-free-the-space-the-installer-needs) | Open Settings > System > Storage and record available space before deleting anything. | [Microsoft support 1](https://support.microsoft.com/en-us/windows/deployment/updates-lifecycle/free-up-space-for-windows-updates) |
| [Find why a feature upgrade rolled back](https://herbertofury.github.io/INTA/#job/windows-update?section=fix-update-explain-a-feature-upgrade-rollback) | Preserve the failed-upgrade logs before cleanup or another installation attempt. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/windows/deployment/upgrade/setupdiag) · [Microsoft documentation 2](https://learn.microsoft.com/en-us/windows/deployment/upgrade/log-files) |
| [Check why a feature update is not offered](https://herbertofury.github.io/INTA/#job/windows-update?section=fix-update-respect-a-safeguard-hold) | Record the installed edition/build and whether update settings are organization-managed. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/windows/deployment/update/safeguard-holds) · [Microsoft documentation 2](https://learn.microsoft.com/en-us/windows/release-health/) |

### [Recover a Windows PC that will not boot](https://github.com/Herbertofury/INTA/wiki/Job-boot-recovery)

[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-boot-recovery)

| Specific fix | First move | Supporting primary sources |
| --- | --- | --- |
| [Find a disk missing from recovery media](https://herbertofury.github.io/INTA/#job/boot-recovery?section=fix-boot-installer-cannot-see-storage) | Confirm the disk is consistently detected in firmware and identify the exact storage controller/model. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/add-device-drivers-to-windows-during-windows-setup) |
| [Identify the Windows volume inside recovery](https://herbertofury.github.io/INTA/#job/boot-recovery?section=fix-boot-check-recovery-drive-letters) | Treat drive letters in the recovery environment as unverified until the actual volumes are identified. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/list-volume) · [Microsoft documentation 2](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image) |
| [Find the matching BitLocker key](https://herbertofury.github.io/INTA/#job/boot-recovery?section=fix-boot-recovery-key-does-not-match) | Read the recovery-key ID displayed by the affected device. | [Microsoft support 1](https://support.microsoft.com/en-us/windows/security/encryption/find-your-bitlocker-recovery-key) |

### [Rescue data from a failing or unreadable drive](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive)

[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive)

| Specific fix | First move | Supporting primary sources |
| --- | --- | --- |
| [Try filesystem undelete before file carving](https://herbertofury.github.io/INTA/#job/failing-drive?section=fix-recovery-preserve-names-on-ntfs) | Stop writes to the affected filesystem and prepare a separate destination. | [CGSecurity instructions 1](https://www.cgsecurity.org/wiki/TestDisk:_undelete_file_for_NTFS) |
| [Resume imaging without swapping source and target](https://herbertofury.github.io/INTA/#job/failing-drive?section=fix-recovery-resume-the-correct-disk-image) | Match the source and destination by model, serial and size before resuming anything. | [GNU manual 1](https://www.gnu.org/software/ddrescue/manual/ddrescue_manual.html) |
| [Validate carved files without trusting their filenames](https://herbertofury.github.io/INTA/#job/failing-drive?section=fix-recovery-carved-files-have-wrong-names) | Treat recovered names and extensions as output labels, then test the contents. | [CGSecurity instructions 1](https://www.cgsecurity.org/wiki/PhotoRec) · [CGSecurity instructions 2](https://www.cgsecurity.org/wiki/PhotoRec_Step_By_Step) |

### [Clone a healthy system or migrate to a new SSD](https://github.com/Herbertofury/INTA/wiki/Job-disk-migration)

[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-disk-migration)

| Specific fix | First move | Supporting primary sources |
| --- | --- | --- |
| [Use remaining space after a successful clone](https://herbertofury.github.io/INTA/#job/disk-migration?section=fix-migration-new-ssd-shows-old-capacity) | Open Disk Management and inspect the partition order on the verified new disk. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/windows-server/storage/disk-management/extend-a-basic-volume) |
| [Read a copy result correctly](https://herbertofury.github.io/INTA/#job/disk-migration?section=fix-migration-interpret-robocopy-exit-code) | Retain the copy log and the exact exit code before rerunning. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy) |
| [Finish what an app-list import cannot migrate](https://herbertofury.github.io/INTA/#job/disk-migration?section=fix-migration-package-list-is-not-app-data) | Separate the application inventory from the user-data and license migration records. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/windows/package-manager/winget/export) · [Microsoft documentation 2](https://learn.microsoft.com/en-us/windows/package-manager/winget/import) |

### [Prove a backup can actually restore](https://github.com/Herbertofury/INTA/wiki/Job-backup-restore)

[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-backup-restore)

| Specific fix | First move | Supporting primary sources |
| --- | --- | --- |
| [Include cloud-only files in the recovery plan](https://herbertofury.github.io/INTA/#job/backup-restore?section=fix-backup-cloud-placeholders-are-not-local-files) | Inspect the affected files’ [OneDrive](https://www.microsoft.com/microsoft-365/onedrive/online-cloud-storage) status icons and confirm the intended account. | [Microsoft support 1](https://support.microsoft.com/en-gb/onedrive/save-disk-space-with-onedrive-files-on-demand-for-windows) |
| [Check repository data as well as metadata](https://herbertofury.github.io/INTA/#job/backup-restore?section=fix-backup-restic-check-is-not-full-data-proof) | Record the snapshot ID and the scope of the check that actually ran. | [restic documentation 1](https://restic.readthedocs.io/en/stable/045_working_with_repos.html) · [restic documentation 2](https://restic.readthedocs.io/en/stable/050_restore.html) |
| [Recover an older version without overwriting current work](https://herbertofury.github.io/INTA/#job/backup-restore?section=fix-backup-restore-beside-current-files) | Confirm the required recovery date and create a separate restore destination. | [helpcenter.veeam.com 1](https://helpcenter.veeam.com/docs/agentforwindows/userguide/files_restore.html) · [kopia.io 2](https://kopia.io/docs/getting-started/) |
| [Prove the backup can be opened after the PC is lost](https://herbertofury.github.io/INTA/#job/backup-restore?section=fix-backup-test-credentials-away-from-source) | Identify the approved credential/recovery store without copying secrets into the public record. | [kopia.io 1](https://kopia.io/docs/getting-started/) · [restic documentation 2](https://restic.readthedocs.io/en/stable/030_preparing_a_new_repo.html) |

### [Fix internet, DNS, or local-network problems](https://github.com/Herbertofury/INTA/wiki/Job-network-dns)

[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-network-dns)

| Specific fix | First move | Supporting primary sources |
| --- | --- | --- |
| [Fix a PC stuck on a self-assigned address](https://herbertofury.github.io/INTA/#job/network-dns?section=fix-network-dhcp-self-assigned-address) | Run ipconfig /all and identify the affected adapter, DHCP state and lease information. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/troubleshoot/windows-server/networking/troubleshoot-problems-dhcp-client) · [Microsoft documentation 2](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/ipconfig) |
| [Restore internal name resolution on VPN](https://herbertofury.github.io/INTA/#job/network-dns?section=fix-network-internal-names-fail-on-vpn) | Compare the failing full hostname and DNS settings with a working approved client. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/troubleshoot/windows-client/networking/troubleshoot-dns-client-resolution-issues) · [Microsoft documentation 2](https://learn.microsoft.com/en-us/troubleshoot/windows-client/networking/firewall-profile-not-switch-to-domain) |
| [Complete a captive Wi-Fi sign-in](https://herbertofury.github.io/INTA/#job/network-dns?section=fix-network-captive-portal-signin) | Confirm the exact network name with the venue or network owner. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/windows-server/networking/ncsi/ncsi-overview) · [Microsoft support 2](https://support.microsoft.com/en-us/windows/experience/connectivity-networking/fix-wi-fi-connection-issues-in-windows) |
| [Separate a blocked port from a DNS problem](https://herbertofury.github.io/INTA/#job/network-dns?section=fix-network-name-resolves-but-port-fails) | Identify the application’s documented destination and TCP port. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/powershell/module/nettcpip/test-netconnection) |
| [Locate a throughput bottleneck](https://herbertofury.github.io/INTA/#job/network-dns?section=fix-network-measure-lan-before-blaming-isp) | Choose approved test endpoints and compare the same transfer over wired and wireless paths. | [software.es.net 1](https://software.es.net/iperf/) · [software.es.net 2](https://software.es.net/iperf/invoking.html) |

### [Bring an office back after a power outage](https://github.com/Herbertofury/INTA/wiki/Job-outage-recovery)

[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-outage-recovery)

| Specific fix | First move | Supporting primary sources |
| --- | --- | --- |
| [Read an unexpected-restart event correctly](https://herbertofury.github.io/INTA/#job/outage-recovery?section=fix-outage-event-41-is-not-the-cause) | Match the event time with the actual outage and user report. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/troubleshoot/windows-client/performance/event-id-41-restart) |
| [Recover a dependent service in the right order](https://herbertofury.github.io/INTA/#job/outage-recovery?section=fix-outage-one-service-did-not-recover) | Identify the exact service name and its required dependencies before restarting anything. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.management/get-service?view=powershell-5.1) |
| [Restore a share after the server returns](https://herbertofury.github.io/INTA/#job/outage-recovery?section=fix-outage-share-unavailable-after-restart) | Test the approved share by its normal hostname from an affected user session. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/windows-server/storage/file-server/troubleshoot/troubleshooting-smb) |

### [Start and finish a clean remote-support session](https://github.com/Herbertofury/INTA/wiki/Job-remote-support)

[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-remote-support)

| Specific fix | First move | Supporting primary sources |
| --- | --- | --- |
| [Move a required file through an approved route](https://herbertofury.github.io/INTA/#job/remote-support?section=fix-remote-copy-route-not-proven) | Confirm the exact file, transfer direction and approved destination with the user. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/windows/client-management/client-tools/quick-assist) · [RustDesk documentation 2](https://rustdesk.com/docs/en/client/) · [Official repository 3](https://github.com/Ylianst/MeshCentral) |
| [Restore Mac screen sharing and control](https://herbertofury.github.io/INTA/#job/remote-support?section=fix-remote-mac-screen-or-control-permission) | Have the authorized local user inspect the permissions on the controlled Mac. | [RustDesk documentation 1](https://rustdesk.com/docs/en/client/mac/) |
| [Avoid losing the session during a restart](https://herbertofury.github.io/INTA/#job/remote-support?section=fix-remote-reboot-needs-a-reconnect-plan) | Agree how the user and technician will reconnect before making the change. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/windows/client-management/client-tools/quick-assist) · [Official repository 2](https://github.com/Ylianst/MeshCentral) |

### [Repair a workstation’s domain trust relationship](https://github.com/Herbertofury/INTA/wiki/Job-ad-trust)

[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-ad-trust)

| Specific fix | First move | Supporting primary sources |
| --- | --- | --- |
| [Fix domain discovery before changing trust](https://herbertofury.github.io/INTA/#job/ad-trust?section=fix-trust-domain-dns-before-account-repair) | Read the member’s active IP and DNS configuration on the intended network or VPN. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/troubleshoot/windows-server/windows-security/broken-trust-relationship-domain-joined-device-its-domain-secure-channel-issues) · [Microsoft documentation 2](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.management/test-computersecurechannel?view=powershell-5.1) |
| [Restore the domain time path](https://herbertofury.github.io/INTA/#job/ad-trust?section=fix-trust-clock-skew-blocks-authentication) | Read the current clock and w32tm /query /status without changing time providers. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/windows-server/networking/windows-time-service/windows-time-service-tools-and-settings) |
| [Check computer identity after restoring an old image](https://herbertofury.github.io/INTA/#job/ad-trust?section=fix-trust-error-after-old-image-restore) | Record the restoration time and whether another active PC uses the same computer identity. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/troubleshoot/windows-server/windows-security/broken-trust-relationship-domain-joined-device-its-domain-secure-channel-issues) |

### [Fix Outlook or email send/receive problems](https://github.com/Herbertofury/INTA/wiki/Job-outlook-email)

[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-outlook-email)

| Specific fix | First move | Supporting primary sources |
| --- | --- | --- |
| [Bring classic Outlook back online](https://herbertofury.github.io/INTA/#job/outlook-email?section=fix-email-classic-outlook-working-offline) | Check the status bar and whether Send/Receive > Work Offline is highlighted. | [Microsoft support 1](https://support.microsoft.com/en-us/outlook/getstarted/how-to-work-offline-in-outlook-for-windows) |
| [Release a message stuck behind a large attachment](https://herbertofury.github.io/INTA/#job/outlook-email?section=fix-email-large-attachment-stuck-outbox) | Confirm the recipient and preserve the message text before changing the stuck item. | [Microsoft support 1](https://support.microsoft.com/en-us/outlook/outlook-email-stuck) |
| [Move an active PST out of the sync bottleneck](https://herbertofury.github.io/INTA/#job/outlook-email?section=fix-email-pst-stalls-onedrive-sync) | Identify the exact PST and protect a closed, usable backup before moving it. | [Microsoft support 1](https://support.microsoft.com/en-us/onedrive/how-to-remove-an-outlook-pst-data-file-from-onedrive) · [Microsoft support 2](https://support.microsoft.com/en-us/onedrive/restrictions-and-limitations-in-onedrive-and-sharepoint) |
| [Find mail missing only from the local cache](https://herbertofury.github.io/INTA/#job/outlook-email?section=fix-email-old-messages-missing-offline) | Confirm the same mailbox and folder in webmail before treating the messages as deleted. | [Microsoft support 1](https://support.microsoft.com/en-us/outlook/getstarted/how-to-work-offline-in-outlook-for-windows) |

### [Restore printing without breaking other queues](https://github.com/Herbertofury/INTA/wiki/Job-printer-repair)

[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-printer-repair)

| Specific fix | First move | Supporting primary sources |
| --- | --- | --- |
| [Match a stale printer port to the current device](https://herbertofury.github.io/INTA/#job/printer-repair?section=fix-printer-offline-after-router-change) | Print the printer’s own network report and match its model/MAC to the intended device. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/powershell/module/printmanagement/get-printerport) · [Microsoft support 2](https://support.microsoft.com/en-us/windows/hardware/printer/troubleshooting-offline-printer-problems-in-windows) |
| [Resolve an incompatible printer-driver package](https://herbertofury.github.io/INTA/#job/printer-repair?section=fix-printer-driver-rejected-on-arm-or-wpp) | Check Settings > System > About and the active [Windows protected print mode](https://learn.microsoft.com/en-us/windows/modern-print/windows-protected-print-mode/windows-protected-print-mode) policy. | [Microsoft support 1](https://support.microsoft.com/en-us/windows/experience/platform-variants/windows-arm-based-pcs-faq) · [Microsoft documentation 2](https://learn.microsoft.com/en-us/windows/modern-print/windows-protected-print-mode/windows-protected-print-mode) |
| [Fix streaks that occur only through the feeder](https://herbertofury.github.io/INTA/#job/printer-repair?section=fix-printer-adf-lines-flatbed-clean) | Copy the same clean original once from the flatbed and once from the feeder, marking which is which. | [HP instructions 1](https://support.hp.com/us-en/document/c03231457) · [HP instructions 2](https://support.hp.com/ee-en/document/c06432233) |
| [Make the scanner appear in the scan profile](https://herbertofury.github.io/INTA/#job/printer-repair?section=fix-printer-scanner-missing-from-app) | Confirm the scanner’s connection and the actual OS architecture independently of the print queue. | [NAPS2 documentation 1](https://www.naps2.com/windows-scanning) |
| [Separate a bad document from a broken printer](https://herbertofury.github.io/INTA/#job/printer-repair?section=fix-printer-one-pdf-will-not-print) | Keep the original file and test a simple one-page document on the same queue. | [SumatraPDF documentation 1](https://www.sumatrapdfreader.org/docs/SumatraPDF-documentation) · [Microsoft support 2](https://support.microsoft.com/en-us/windows/hardware/printer/fix-printer-connection-and-printing-problems-in-windows) |

### [Diagnose crashes, blue screens, and overheating](https://github.com/Herbertofury/INTA/wiki/Job-hardware-bsod)

[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-hardware-bsod)

| Specific fix | First move | Supporting primary sources |
| --- | --- | --- |
| [Capture usable evidence for the next crash](https://herbertofury.github.io/INTA/#job/hardware-bsod?section=fix-hardware-crash-without-a-dump) | Check the configured dump type, target path, free space and page-file state. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/troubleshoot/windows-client/performance/configure-system-failure-and-recovery-options) · [Microsoft documentation 2](https://learn.microsoft.com/en-us/troubleshoot/windows-client/performance/how-to-determine-the-appropriate-page-file-size-for-64-bit-versions-of-windows) |
| [Isolate a memory fault after an upgrade](https://herbertofury.github.io/INTA/#job/hardware-bsod?section=fix-hardware-memory-errors-after-upgrade) | Record the installed modules, slots and current firmware settings before opening the device. | [Official repository 1](https://github.com/memtest86plus/memtest86plus) |
| [Find a device failing through a USB hub](https://herbertofury.github.io/INTA/#job/hardware-bsod?section=fix-hardware-usb-dock-device-drops) | Record the device, hub/dock, cable and physical port used for the failure. | [Publisher documentation 1](https://www.uwe-sieber.de/usbtreeview_e.html) |

### [Set up a dependable new workstation](https://github.com/Herbertofury/INTA/wiki/Job-new-pc)

[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-new-pc)

| Specific fix | First move | Supporting primary sources |
| --- | --- | --- |
| [Restore network access during device setup](https://herbertofury.github.io/INTA/#job/new-pc?section=fix-newpc-network-missing-during-setup) | Identify the exact device model and architecture from the hardware record. | [Microsoft support 1](https://support.microsoft.com/en-us/windows/experience/connectivity-networking/get-connected-when-setting-up-your-windows-device) |
| [Make the package manager available to the user](https://herbertofury.github.io/INTA/#job/new-pc?section=fix-newpc-winget-not-found) | Check the supported [Windows](https://www.microsoft.com/windows) version, current user sign-in and App Installer availability. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/windows/package-manager/winget/) |
| [Confirm the intended organization device state](https://herbertofury.github.io/INTA/#job/new-pc?section=fix-newpc-registered-is-not-joined) | Compare the intended enrollment/join design with Settings > Accounts > Access work or school. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/entra/identity/devices/concept-device-registration) · [Microsoft documentation 2](https://learn.microsoft.com/en-us/entra/identity/devices/concept-directory-join) |

### [Close a repair with a clear client handoff](https://github.com/Herbertofury/INTA/wiki/Job-client-handoff)

[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-client-handoff)

| Specific fix | First move | Supporting primary sources |
| --- | --- | --- |
| [Prove the repair under the real user account](https://herbertofury.github.io/INTA/#job/client-handoff?section=fix-handoff-admin-works-user-denied) | Reproduce the exact task under the intended standard-user account. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/icacls) |
| [Make recovery information retrievable](https://herbertofury.github.io/INTA/#job/client-handoff?section=fix-handoff-recovery-record-cannot-be-found) | Identify the approved account or organization store that should hold the recovery key. | [Microsoft support 1](https://support.microsoft.com/en-us/windows/security/encryption/back-up-your-bitlocker-recovery-key) · [Microsoft support 2](https://support.microsoft.com/en-us/windows/security/encryption/find-your-bitlocker-recovery-key) |
| [Remove access that was only for this repair](https://herbertofury.github.io/INTA/#job/client-handoff?section=fix-handoff-temporary-support-app-persists) | Compare the current installation/access state with the agreed end state for this device. | [Microsoft support 1](https://support.microsoft.com/en-us/windows/apps/uninstall-or-remove-apps-and-programs-in-windows) · [RustDesk documentation 2](https://rustdesk.com/docs/en/client/) · [Official repository 3](https://github.com/Ylianst/MeshCentral) |

### [Build and refresh a technician rescue USB](https://github.com/Herbertofury/INTA/wiki/Job-boot-usb)

[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-boot-usb)

| Specific fix | First move | Supporting primary sources |
| --- | --- | --- |
| [Resolve a Secure Boot rejection deliberately](https://herbertofury.github.io/INTA/#job/boot-usb?section=fix-bootusb-secure-boot-blocks-rescue-menu) | Record the exact firmware message and verify the USB image/publisher before changing firmware settings. | [ventoy.net 1](https://www.ventoy.net/en/doc_secure.html) · [Official repository 2](https://github.com/pbatard/rufus/wiki/FAQ) |
| [Choose media that can boot the target](https://herbertofury.github.io/INTA/#job/boot-usb?section=fix-bootusb-image-is-wrong-architecture) | Check the target’s actual architecture before rewriting the same USB again. | [microsoft.com 1](https://www.microsoft.com/en-us/software-download/windows11arm64) |
| [Handle an image larger than the USB filesystem limit](https://herbertofury.github.io/INTA/#job/boot-usb?section=fix-bootusb-large-iso-will-not-copy) | Check the USB filesystem and the exact file size before deleting files or changing partitions. | [ventoy.net 1](https://www.ventoy.net/en/doc_disk_layout.html) · [Official repository 2](https://github.com/pbatard/rufus/wiki/FAQ) |

### [Use AI as a careful troubleshooting copilot](https://github.com/Herbertofury/INTA/wiki/Job-ai-triage)

[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-ai-triage)

| Specific fix | First move | Supporting primary sources |
| --- | --- | --- |
| [Create a safe diagnostic excerpt](https://herbertofury.github.io/INTA/#job/ai-triage?section=fix-ai-redact-logs-before-sharing) | Prepare a local copy and keep the original evidence in the approved private case record. | [OpenAI help 1](https://help.openai.com/en/articles/7730893-data-controls-in-chatgpt) · [openai.com 2](https://openai.com/enterprise-privacy/) |
| [Check a suggested command before running it](https://herbertofury.github.io/INTA/#job/ai-triage?section=fix-ai-command-does-not-match-the-machine) | Identify the actual shell/version and whether the command reads or changes state. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/get-command) · [Microsoft documentation 2](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/get-help) |
| [Keep a log or webpage from changing the task](https://herbertofury.github.io/INTA/#job/ai-triage?section=fix-ai-untrusted-instructions-inside-evidence) | Treat that text as untrusted evidence rather than authorization. | [OpenAI documentation 1](https://developers.openai.com/codex/security/) · [OpenAI documentation 2](https://platform.openai.com/docs/guides/safety-best-practices) |

### [Run a useful workstation maintenance visit](https://github.com/Herbertofury/INTA/wiki/Job-preventive-maintenance)

[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-preventive-maintenance)

| Specific fix | First move | Supporting primary sources |
| --- | --- | --- |
| [Use the supported optimization for the drive type](https://herbertofury.github.io/INTA/#job/preventive-maintenance?section=fix-maintenance-optimize-the-right-media) | Check drive health and open Defragment and Optimize Drives to identify the media type and current status. | [Microsoft support 1](https://support.microsoft.com/en-us/windows/experience/storage-filemanagement/defragment-optimize-your-data-drives-in-windows) |
| [Update an app missing from the package-manager list](https://herbertofury.github.io/INTA/#job/preventive-maintenance?section=fix-maintenance-package-not-in-upgrade-list) | Identify the publisher, installed version and how the application was originally deployed. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/windows/package-manager/winget/upgrade) · [Microsoft documentation 2](https://learn.microsoft.com/en-us/windows/package-manager/winget/pinning) |
| [Resume a stalled cloud-sync workflow](https://herbertofury.github.io/INTA/#job/preventive-maintenance?section=fix-maintenance-onedrive-sync-is-paused) | Open the sync client’s activity/status view and identify the actual pause or error. | [Microsoft support 1](https://support.microsoft.com/en-us/onedrive/how-to-pause-and-resume-onedrive-sync) · [Microsoft support 2](https://support.microsoft.com/en-us/onedrive/fix-onedrive-sync-problems) |
| [Find an operating system outside its support plan](https://herbertofury.github.io/INTA/#job/preventive-maintenance?section=fix-maintenance-check-os-support-state) | Record the exact edition, version and build from Settings > System > About. | [Microsoft documentation 1](https://learn.microsoft.com/en-us/windows/release-health/) · [Microsoft documentation 2](https://learn.microsoft.com/en-us/windows/whats-new/extended-security-updates) |

## Review boundaries and next contributions

These cases intentionally stop at the point where an exact device manual, tenant administrator, incident owner or data-recovery specialist is needed. They do not invent a universal storage driver, a universal remote-reconnection feature, a password-reset-only incident closure, or a destructive repair command that fits every computer.

For another observed fault, add a distinct case under its existing service job and verify the applicable publisher procedure. Keep the stable case ID, record the actual source review date, preserve earlier evidence and use the [research queue](https://github.com/Herbertofury/INTA/blob/main/data/research-queue.json) to rotate coverage. See [Knowledge Organization](https://github.com/Herbertofury/INTA/wiki/Knowledge-Organization) and [Knowledge Standards](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) for contribution rules.
