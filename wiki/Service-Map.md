[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Service map](https://github.com/Herbertofury/INTA/wiki/Service-Map) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

# 🗺️ Service map

**Choose the field, then the exact job or fix.** Case titles open that fix in the working view; full-guide links open the Wiki. Every guide has one primary home.

**[Open the interactive service map](https://herbertofury.github.io/INTA/#services)** · [Urgent answers](Home) · [All tools](Tool-Directory)

**[🧭 Intake and handoff](#area-first-response)** · **[🛡️ Security incidents](#area-security)** · **[🪟 System repair](#area-windows)** · **[💾 Storage and recovery](#area-recovery)** · **[🌐 Network and services](#area-office)** · **[🖨️ Email and printing](#area-workplace)** · **[🧰 Deployment and maintenance](#area-deployment)** · **[🧠 Assisted troubleshooting](#area-ai)**

<a id="area-first-response"></a>

## 🧭 First response & handoff

Frame the case, connect to the right device, and close with evidence.

### [🧭 Triage a new issue](https://github.com/Herbertofury/INTA/wiki/Job-intake-triage)

**First move:** Ask the user to show the failing task once. Record the exact error, last working time, affected users and whether important data has another copy.

**[Working view](https://herbertofury.github.io/INTA/#job/intake-triage)** · **[Full guide](https://github.com/Herbertofury/INTA/wiki/Job-intake-triage)** · 3 specific fixes · 3 official resources

- **[Choose the correct model and architecture](https://herbertofury.github.io/INTA/#job/intake-triage?section=fix-triage-confirm-model-and-architecture)** — A driver or repair package says it is incompatible, or the device model is uncertain.
- **[Find the background program blocking an app](https://herbertofury.github.io/INTA/#job/intake-triage?section=fix-triage-isolate-background-conflict)** — An application fails in normal startup but works when optional background software is absent.
- **[Correct a bad clock before chasing sign-in errors](https://herbertofury.github.io/INTA/#job/intake-triage?section=fix-triage-clock-breaks-signin)** — Several secure sites or sign-ins fail and the displayed date or time is wrong.

### [🖥️ Help remotely](https://github.com/Herbertofury/INTA/wiki/Job-remote-support)

**First move:** Confirm the user and helper through the existing support channel. Decide whether the session is attended or requires an approved managed agent.

**[Working view](https://herbertofury.github.io/INTA/#job/remote-support)** · **[Full guide](https://github.com/Herbertofury/INTA/wiki/Job-remote-support)** · 3 specific fixes · 3 official resources

- **[Move a required file through an approved route](https://herbertofury.github.io/INTA/#job/remote-support?section=fix-remote-copy-route-not-proven)** — Screen control works, but the required installer or diagnostic file cannot be transferred.
- **[Restore Mac screen sharing and control](https://herbertofury.github.io/INTA/#job/remote-support?section=fix-remote-mac-screen-or-control-permission)** — A [RustDesk](https://rustdesk.com/) session connects to a Mac but the screen is blank or input does not work.
- **[Avoid losing the session during a restart](https://herbertofury.github.io/INTA/#job/remote-support?section=fix-remote-reboot-needs-a-reconnect-plan)** — The next repair requires rebooting, changing the network adapter or restarting security software.

### [📋 Complete the handoff](https://github.com/Herbertofury/INTA/wiki/Job-client-handoff)

**First move:** Repeat the original requested task with the user before declaring the repair complete.

**[Working view](https://herbertofury.github.io/INTA/#job/client-handoff)** · **[Full guide](https://github.com/Herbertofury/INTA/wiki/Job-client-handoff)** · 3 specific fixes · 3 official resources

- **[Prove the repair under the real user account](https://herbertofury.github.io/INTA/#job/client-handoff?section=fix-handoff-admin-works-user-denied)** — The technician can open a repaired or restored file, but the normal user gets Access denied.
- **[Make recovery information retrievable](https://herbertofury.github.io/INTA/#job/client-handoff?section=fix-handoff-recovery-record-cannot-be-found)** — Encryption works, but the owner cannot locate the recovery record during handoff.
- **[Remove access that was only for this repair](https://herbertofury.github.io/INTA/#job/client-handoff?section=fix-handoff-temporary-support-app-persists)** — A temporary support program or unattended setting still starts after the case ends.

<a id="area-security"></a>

## 🛡️ Security & [incident response](https://github.com/Herbertofury/INTA/wiki/Job-ransomware-response)

Separate a routine cleanup from a wider compromise.

### [🦠 Remove viruses](https://github.com/Herbertofury/INTA/wiki/Job-malware-removal)

**First move:** If encryption, privileged-account compromise or multiple affected devices are present, isolate and use the incident route. Otherwise record detections and keep one real-time protection product.

**[Working view](https://herbertofury.github.io/INTA/#job/malware-removal)** · **[Full guide](https://github.com/Herbertofury/INTA/wiki/Job-malware-removal)** · 3 specific fixes · 5 official resources

- **[Stop website notification popups](https://herbertofury.github.io/INTA/#job/malware-removal?section=fix-malware-browser-notification-popups)** — Desktop alerts claim the PC is infected, but the alerts identify a browser or website sender.
- **[Undo an accidental allowed-threat decision](https://herbertofury.github.io/INTA/#job/malware-removal?section=fix-malware-allowed-threat-keeps-returning)** — A known detection was allowed earlier and the same item remains present.
- **[Find why the offline scan never runs](https://herbertofury.github.io/INTA/#job/malware-removal?section=fix-malware-offline-scan-does-not-start)** — The offline-scan request returns to [Windows](https://www.microsoft.com/windows) without a documented completed scan.

### [🚨 Contain ransomware](https://github.com/Herbertofury/INTA/wiki/Job-ransomware-response)

**First move:** Disconnect affected network access and contact the incident lead through a clean channel. Do not attach backups to the affected environment.

**[Working view](https://herbertofury.github.io/INTA/#job/ransomware-response)** · **[Full guide](https://github.com/Herbertofury/INTA/wiki/Job-ransomware-response)** · 3 specific fixes · 3 official resources

- **[Protect backups before testing a restore](https://herbertofury.github.io/INTA/#job/ransomware-response?section=fix-ransomware-protect-recovery-copies)** — Compromised machines can still reach the backup destination or its administration account.
- **[Contain stolen account access as well as the PC](https://herbertofury.github.io/INTA/#job/ransomware-response?section=fix-ransomware-compromised-admin-account)** — The incident involves privileged credentials, suspicious sessions or activity on more than one device.
- **[Evaluate a decryptor without risking the evidence](https://herbertofury.github.io/INTA/#job/ransomware-response?section=fix-ransomware-test-decryptor-on-copies)** — A supposed decryptor is offered for the observed ransomware family.

<a id="area-windows"></a>

## 🪟 [Windows](https://www.microsoft.com/windows) & system stability

Find the bottleneck, repair the cause, and verify the workload.

### [🐢 Fix a slow PC](https://github.com/Herbertofury/INTA/wiki/Job-slow-pc)

**First move:** Time one slow action. Press Ctrl+Shift+Esc and watch CPU, memory and disk while reproducing it; check drive health before a heavy scan.

**[Working view](https://herbertofury.github.io/INTA/#job/slow-pc)** · **[Full guide](https://github.com/Herbertofury/INTA/wiki/Job-slow-pc)** · 4 specific fixes · 3 official resources

- **[Separate useful cache from memory pressure](https://herbertofury.github.io/INTA/#job/slow-pc?section=fix-slow-pc-standby-memory-is-not-a-leak)** — A PC shows high memory use and someone proposes a RAM-cleaning utility.
- **[Find the optional app delaying sign-in](https://herbertofury.github.io/INTA/#job/slow-pc?section=fix-slow-pc-startup-app-delay)** — The desktop appears quickly but the user must wait before starting normal work.
- **[Find what an app is waiting for](https://herbertofury.github.io/INTA/#job/slow-pc?section=fix-slow-pc-filter-a-short-io-trace)** — One application freezes while disk or network-file activity continues.
- **[Explain a laptop that slows down on battery](https://herbertofury.github.io/INTA/#job/slow-pc?section=fix-slow-pc-only-on-battery)** — The same application is responsive on AC power but slow when unplugged.

### [🛠️ Repair Windows](https://github.com/Herbertofury/INTA/wiki/Job-windows-repair)

**First move:** Check stable storage, saved work and a recovery copy. The online repair commands below apply to the running [Windows](https://www.microsoft.com/windows) installation, not an unverified [WinRE](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference) drive.

**[Working view](https://herbertofury.github.io/INTA/#job/windows-repair)** · **[Full guide](https://github.com/Herbertofury/INTA/wiki/Job-windows-repair)** · 3 specific fixes · 4 official resources

- **[Resolve a missing DISM repair source](https://herbertofury.github.io/INTA/#job/windows-repair?section=fix-repair-dism-source-not-found)** — [DISM](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image) reports that source files could not be found, including a source-related servicing error.
- **[Identify what SFC could not repair](https://herbertofury.github.io/INTA/#job/windows-repair?section=fix-repair-read-the-current-sfc-result)** — [System File Checker](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc) reports unresolved corruption after the documented component-store sequence.
- **[Repair one broken app without resetting the PC](https://herbertofury.github.io/INTA/#job/windows-repair?section=fix-repair-one-app-without-resetting-windows)** — [Windows](https://www.microsoft.com/windows) works normally but one installed application will not open or behaves incorrectly.

### [🔄 Fix Windows updates](https://github.com/Herbertofury/INTA/wiki/Job-windows-update)

**First move:** Open Settings > [Windows Update](https://learn.microsoft.com/en-us/windows/deployment/update/) > Update history. Record the failing KB, error and restart state before retrying.

**[Working view](https://herbertofury.github.io/INTA/#job/windows-update)** · **[Full guide](https://github.com/Herbertofury/INTA/wiki/Job-windows-update)** · 3 specific fixes · 5 official resources

- **[Clear an update’s storage blocker safely](https://herbertofury.github.io/INTA/#job/windows-update?section=fix-update-free-the-space-the-installer-needs)** — [Windows Update](https://learn.microsoft.com/en-us/windows/deployment/update/) asks for more space or fails during a low-space installation.
- **[Find why a feature upgrade rolled back](https://herbertofury.github.io/INTA/#job/windows-update?section=fix-update-explain-a-feature-upgrade-rollback)** — [Windows](https://www.microsoft.com/windows) returns to the previous version after an attempted feature upgrade.
- **[Check why a feature update is not offered](https://herbertofury.github.io/INTA/#job/windows-update?section=fix-update-respect-a-safeguard-hold)** — A device receives routine updates but a newer feature release is missing.

### [🌡️ Diagnose crashes / BSOD](https://github.com/Herbertofury/INTA/wiki/Job-hardware-bsod)

**First move:** Record the stop code, crash time and recent changes. Check storage health and safe temperatures before load testing.

**[Working view](https://herbertofury.github.io/INTA/#job/hardware-bsod)** · **[Full guide](https://github.com/Herbertofury/INTA/wiki/Job-hardware-bsod)** · 3 specific fixes · 4 official resources

- **[Capture usable evidence for the next crash](https://herbertofury.github.io/INTA/#job/hardware-bsod?section=fix-hardware-crash-without-a-dump)** — A real stop error occurs but no usable memory dump is available.
- **[Isolate a memory fault after an upgrade](https://herbertofury.github.io/INTA/#job/hardware-bsod?section=fix-hardware-memory-errors-after-upgrade)** — Crashes or memory-test errors began after changing RAM or firmware memory settings.
- **[Find a device failing through a USB hub](https://herbertofury.github.io/INTA/#job/hardware-bsod?section=fix-hardware-usb-dock-device-drops)** — A scanner, printer or storage device disconnects through a dock but works intermittently elsewhere.

<a id="area-recovery"></a>

## 💾 Storage, backup & rescue

Protect the source and prove the recovery path.

### [🚑 Fix boot failure](https://github.com/Herbertofury/INTA/wiki/Job-boot-recovery)

**First move:** Record the exact startup screen. Check whether firmware sees the disk and whether [BitLocker](https://learn.microsoft.com/en-us/windows/security/operating-system-security/data-protection/bitlocker/) recovery material is available before repair.

**[Working view](https://herbertofury.github.io/INTA/#job/boot-recovery)** · **[Full guide](https://github.com/Herbertofury/INTA/wiki/Job-boot-recovery)** · 3 specific fixes · 5 official resources

- **[Find a disk missing from recovery media](https://herbertofury.github.io/INTA/#job/boot-recovery?section=fix-boot-installer-cannot-see-storage)** — Firmware sees the disk but [Windows](https://www.microsoft.com/windows) installation or recovery media does not.
- **[Identify the Windows volume inside recovery](https://herbertofury.github.io/INTA/#job/boot-recovery?section=fix-boot-check-recovery-drive-letters)** — An offline repair cannot find [Windows](https://www.microsoft.com/windows) or appears to target the wrong drive.
- **[Find the matching BitLocker key](https://herbertofury.github.io/INTA/#job/boot-recovery?section=fix-boot-recovery-key-does-not-match)** — A stored recovery key is rejected or several similarly named device records exist.

### [💾 Recover data](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive)

**First move:** Stop writing to the source. Clicking, impact damage or irreplaceable data warrants a recovery-specialist decision before more power-on time.

**[Working view](https://herbertofury.github.io/INTA/#job/failing-drive)** · **[Full guide](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive)** · 3 specific fixes · 4 official resources

- **[Try filesystem undelete before file carving](https://herbertofury.github.io/INTA/#job/failing-drive?section=fix-recovery-preserve-names-on-ntfs)** — Files were deleted from a healthy NTFS volume and original names or folders matter.
- **[Resume imaging without swapping source and target](https://herbertofury.github.io/INTA/#job/failing-drive?section=fix-recovery-resume-the-correct-disk-image)** — A recovery session stopped and device names changed after reconnecting or rebooting.
- **[Validate carved files without trusting their filenames](https://herbertofury.github.io/INTA/#job/failing-drive?section=fix-recovery-carved-files-have-wrong-names)** — [PhotoRec](https://www.cgsecurity.org/wiki/PhotoRec) produces numbered folders and files rather than the original directory tree.

### [💽 Clone or migrate a disk](https://github.com/Herbertofury/INTA/wiki/Job-disk-migration)

**First move:** Identify every disk by model, serial and capacity. Confirm the source is healthy, backed up and the chosen destination may be overwritten.

**[Working view](https://herbertofury.github.io/INTA/#job/disk-migration)** · **[Full guide](https://github.com/Herbertofury/INTA/wiki/Job-disk-migration)** · 3 specific fixes · 4 official resources

- **[Use remaining space after a successful clone](https://herbertofury.github.io/INTA/#job/disk-migration?section=fix-migration-new-ssd-shows-old-capacity)** — A larger replacement disk boots correctly but the [Windows](https://www.microsoft.com/windows) volume is still the old size.
- **[Read a copy result correctly](https://herbertofury.github.io/INTA/#job/disk-migration?section=fix-migration-interpret-robocopy-exit-code)** — A [Robocopy](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy) job reports a nonzero exit code and a wrapper labels the whole migration failed.
- **[Finish what an app-list import cannot migrate](https://herbertofury.github.io/INTA/#job/disk-migration?section=fix-migration-package-list-is-not-app-data)** — Applications reinstall on the new PC but settings, licenses or data are missing.

### [🗄️ Test a backup restore](https://github.com/Herbertofury/INTA/wiki/Job-backup-restore)

**First move:** Identify the existing backup product, repository, recovery point and credentials. Use the matching restore tool rather than converting formats during an incident.

**[Working view](https://herbertofury.github.io/INTA/#job/backup-restore)** · **[Full guide](https://github.com/Herbertofury/INTA/wiki/Job-backup-restore)** · 4 specific fixes · 5 official resources

- **[Include cloud-only files in the recovery plan](https://herbertofury.github.io/INTA/#job/backup-restore?section=fix-backup-cloud-placeholders-are-not-local-files)** — A local backup or migrated folder contains [OneDrive](https://www.microsoft.com/microsoft-365/onedrive/online-cloud-storage) placeholders instead of usable offline files.
- **[Check repository data as well as metadata](https://herbertofury.github.io/INTA/#job/backup-restore?section=fix-backup-restic-check-is-not-full-data-proof)** — A [restic](https://restic.net/) repository check passes but no restored file has been tested.
- **[Recover an older version without overwriting current work](https://herbertofury.github.io/INTA/#job/backup-restore?section=fix-backup-restore-beside-current-files)** — The owner needs an earlier file version but current edits must be retained.
- **[Prove the backup can be opened after the PC is lost](https://herbertofury.github.io/INTA/#job/backup-restore?section=fix-backup-test-credentials-away-from-source)** — Backups run automatically, but only the original machine appears to know the repository password.

### [🔌 Build a rescue USB](https://github.com/Herbertofury/INTA/wiki/Job-boot-usb)

**First move:** Identify the USB by model, capacity and contents; copy off anything that must be kept before a write operation.

**[Working view](https://herbertofury.github.io/INTA/#job/boot-usb)** · **[Full guide](https://github.com/Herbertofury/INTA/wiki/Job-boot-usb)** · 3 specific fixes · 4 official resources

- **[Resolve a Secure Boot rejection deliberately](https://herbertofury.github.io/INTA/#job/boot-usb?section=fix-bootusb-secure-boot-blocks-rescue-menu)** — A prepared [rescue USB](https://github.com/Herbertofury/INTA/wiki/Job-boot-usb) is detected but its bootloader is rejected by Secure Boot.
- **[Choose media that can boot the target](https://herbertofury.github.io/INTA/#job/boot-usb?section=fix-bootusb-image-is-wrong-architecture)** — A rescue image works on an x64 PC but not on an ARM device.
- **[Handle an image larger than the USB filesystem limit](https://herbertofury.github.io/INTA/#job/boot-usb?section=fix-bootusb-large-iso-will-not-copy)** — Copying a large ISO or installation file fails even though the USB has enough total free space.

<a id="area-office"></a>

## 🌐 Network & office continuity

Restore dependencies in the right order.

### [🌐 Fix internet & DNS](https://github.com/Herbertofury/INTA/wiki/Job-network-dns)

**First move:** Compare another device on the same network. Establish whether the fault affects this PC, one service, the VPN or the whole office.

**[Working view](https://herbertofury.github.io/INTA/#job/network-dns)** · **[Full guide](https://github.com/Herbertofury/INTA/wiki/Job-network-dns)** · 5 specific fixes · 4 official resources

- **[Fix a PC stuck on a self-assigned address](https://herbertofury.github.io/INTA/#job/network-dns?section=fix-network-dhcp-self-assigned-address)** — A DHCP-configured adapter has a 169.254 address and cannot reach the normal gateway.
- **[Restore internal name resolution on VPN](https://herbertofury.github.io/INTA/#job/network-dns?section=fix-network-internal-names-fail-on-vpn)** — Public websites work, but a company share or application name fails after VPN connection.
- **[Complete a captive Wi-Fi sign-in](https://herbertofury.github.io/INTA/#job/network-dns?section=fix-network-captive-portal-signin)** — The PC associates with guest Wi-Fi but internet access is limited until the network’s sign-in step.
- **[Separate a blocked port from a DNS problem](https://herbertofury.github.io/INTA/#job/network-dns?section=fix-network-name-resolves-but-port-fails)** — The expected hostname resolves, but one application cannot connect.
- **[Locate a throughput bottleneck](https://herbertofury.github.io/INTA/#job/network-dns?section=fix-network-measure-lan-before-blaming-isp)** — Large transfers are slow, but browsing and connectivity checks pass.

### [⚡ Recover from an outage](https://github.com/Herbertofury/INTA/wiki/Job-outage-recovery)

**First move:** Confirm safe power and identify the affected service. Assign one recovery owner before restarting shared systems.

**[Working view](https://herbertofury.github.io/INTA/#job/outage-recovery)** · **[Full guide](https://github.com/Herbertofury/INTA/wiki/Job-outage-recovery)** · 3 specific fixes · 4 official resources

- **[Read an unexpected-restart event correctly](https://herbertofury.github.io/INTA/#job/outage-recovery?section=fix-outage-event-41-is-not-the-cause)** — Event 41 appears after a power outage or unexplained restart.
- **[Recover a dependent service in the right order](https://herbertofury.github.io/INTA/#job/outage-recovery?section=fix-outage-one-service-did-not-recover)** — The server is running after power restoration but one business service remains unavailable.
- **[Restore a share after the server returns](https://herbertofury.github.io/INTA/#job/outage-recovery?section=fix-outage-share-unavailable-after-restart)** — The server responds, but a previously working file share or mapped drive does not.

### [🔑 Repair domain trust](https://github.com/Herbertofury/INTA/wiki/Job-ad-trust)

**First move:** Use the affected member PC. Check its domain DNS, time and domain-controller reachability before considering a trust repair.

**[Working view](https://herbertofury.github.io/INTA/#job/ad-trust)** · **[Full guide](https://github.com/Herbertofury/INTA/wiki/Job-ad-trust)** · 3 specific fixes · 3 official resources

- **[Fix domain discovery before changing trust](https://herbertofury.github.io/INTA/#job/ad-trust?section=fix-trust-domain-dns-before-account-repair)** — The member PC reports a trust/sign-in fault while internal domain names also fail.
- **[Restore the domain time path](https://herbertofury.github.io/INTA/#job/ad-trust?section=fix-trust-clock-skew-blocks-authentication)** — Domain authentication fails and the member clock differs materially from the domain’s reference.
- **[Check computer identity after restoring an old image](https://herbertofury.github.io/INTA/#job/ad-trust?section=fix-trust-error-after-old-image-restore)** — Trust broke after cloning, renaming or restoring a domain-joined workstation.

<a id="area-workplace"></a>

## 🖨️ Everyday workplace fixes

Get the actual mail, document, and printing workflow working.

### [📧 Fix email](https://github.com/Herbertofury/INTA/wiki/Job-outlook-email)

**First move:** Test the same account in webmail and identify classic [Outlook](https://support.microsoft.com/outlook) versus new [Outlook](https://support.microsoft.com/outlook) before changing a local profile.

**[Working view](https://herbertofury.github.io/INTA/#job/outlook-email)** · **[Full guide](https://github.com/Herbertofury/INTA/wiki/Job-outlook-email)** · 4 specific fixes · 3 official resources

- **[Bring classic Outlook back online](https://herbertofury.github.io/INTA/#job/outlook-email?section=fix-email-classic-outlook-working-offline)** — Messages stay in Outbox and the classic client shows Working Offline.
- **[Release a message stuck behind a large attachment](https://herbertofury.github.io/INTA/#job/outlook-email?section=fix-email-large-attachment-stuck-outbox)** — One classic [Outlook](https://support.microsoft.com/outlook) message remains in Outbox while sending is stalled.
- **[Move an active PST out of the sync bottleneck](https://herbertofury.github.io/INTA/#job/outlook-email?section=fix-email-pst-stalls-onedrive-sync)** — [OneDrive](https://www.microsoft.com/microsoft-365/onedrive/online-cloud-storage) stays on Processing changes or reports a file in use while [Outlook](https://support.microsoft.com/outlook) uses a PST in the sync folder.
- **[Find mail missing only from the local cache](https://herbertofury.github.io/INTA/#job/outlook-email?section=fix-email-old-messages-missing-offline)** — Older mail appears in webmail but is absent or unavailable offline in classic [Outlook](https://support.microsoft.com/outlook).

### [🖨️ Fix a printer](https://github.com/Herbertofury/INTA/wiki/Job-printer-repair)

**First move:** Read the make/model and IP from the printer’s own network/configuration report. On the PC open Settings > Bluetooth & devices > Printers & scanners and select the intended queue.

**[Working view](https://herbertofury.github.io/INTA/#job/printer-repair)** · **[Full guide](https://github.com/Herbertofury/INTA/wiki/Job-printer-repair)** · 5 specific fixes · 16 official resources

- **[Match a stale printer port to the current device](https://herbertofury.github.io/INTA/#job/printer-repair?section=fix-printer-offline-after-router-change)** — A network printer went offline after a router change, lease change or network move.
- **[Resolve an incompatible printer-driver package](https://herbertofury.github.io/INTA/#job/printer-repair?section=fix-printer-driver-rejected-on-arm-or-wpp)** — A vendor installer is rejected on an ARM PC or a protected-print device.
- **[Fix streaks that occur only through the feeder](https://herbertofury.github.io/INTA/#job/printer-repair?section=fix-printer-adf-lines-flatbed-clean)** — Scans or copies show a vertical line from the ADF, while a flatbed copy is clean.
- **[Make the scanner appear in the scan profile](https://herbertofury.github.io/INTA/#job/printer-repair?section=fix-printer-scanner-missing-from-app)** — Printing works, but the scan application cannot find the multifunction device.
- **[Separate a bad document from a broken printer](https://herbertofury.github.io/INTA/#job/printer-repair?section=fix-printer-one-pdf-will-not-print)** — A single PDF fails, but the printer’s internal page and other documents print.

<a id="area-deployment"></a>

## 🧰 Deployment & maintenance

Build a dependable workstation and keep it dependable.

### [🆕 Set up a new PC](https://github.com/Herbertofury/INTA/wiki/Job-new-pc)

**First move:** Confirm the owner, organization enrollment method, required apps and migrated data. Check Settings > System > About for the actual [Windows](https://www.microsoft.com/windows) version and architecture.

**[Working view](https://herbertofury.github.io/INTA/#job/new-pc)** · **[Full guide](https://github.com/Herbertofury/INTA/wiki/Job-new-pc)** · 3 specific fixes · 4 official resources

- **[Restore network access during device setup](https://herbertofury.github.io/INTA/#job/new-pc?section=fix-newpc-network-missing-during-setup)** — A newly installed PC has no working network adapter during [Windows](https://www.microsoft.com/windows) setup.
- **[Make the package manager available to the user](https://herbertofury.github.io/INTA/#job/new-pc?section=fix-newpc-winget-not-found)** — The [winget](https://learn.microsoft.com/en-us/windows/package-manager/winget/) command is not recognized on a newly prepared supported [Windows](https://www.microsoft.com/windows) PC.
- **[Confirm the intended organization device state](https://herbertofury.github.io/INTA/#job/new-pc?section=fix-newpc-registered-is-not-joined)** — A work account is present, but the device does not receive expected management or sign-in behavior.

### [🧹 Maintain a workstation](https://github.com/Herbertofury/INTA/wiki/Job-preventive-maintenance)

**First move:** Agree the maintenance window and identify the user’s current problem. Verify a usable backup before changes.

**[Working view](https://herbertofury.github.io/INTA/#job/preventive-maintenance)** · **[Full guide](https://github.com/Herbertofury/INTA/wiki/Job-preventive-maintenance)** · 4 specific fixes · 4 official resources

- **[Use the supported optimization for the drive type](https://herbertofury.github.io/INTA/#job/preventive-maintenance?section=fix-maintenance-optimize-the-right-media)** — A maintenance request proposes repeatedly defragmenting or cleaning every drive.
- **[Update an app missing from the package-manager list](https://herbertofury.github.io/INTA/#job/preventive-maintenance?section=fix-maintenance-package-not-in-upgrade-list)** — An installed program is outdated but does not appear as a normal [WinGet](https://learn.microsoft.com/en-us/windows/package-manager/winget/) upgrade.
- **[Resume a stalled cloud-sync workflow](https://herbertofury.github.io/INTA/#job/preventive-maintenance?section=fix-maintenance-onedrive-sync-is-paused)** — New files remain on one PC and the [OneDrive](https://www.microsoft.com/microsoft-365/onedrive/online-cloud-storage) icon shows paused or an error state.
- **[Find an operating system outside its support plan](https://herbertofury.github.io/INTA/#job/preventive-maintenance?section=fix-maintenance-check-os-support-state)** — The device is usable but its installed [Windows](https://www.microsoft.com/windows) release may no longer receive the required updates.

<a id="area-ai"></a>

## 🧠 AI-assisted investigation

Turn sanitized evidence into a reviewed next test.

### [🧠 Troubleshoot with AI](https://github.com/Herbertofury/INTA/wiki/Job-ai-triage)

**First move:** Prepare the exact error, timing and environment with customer identities, credentials, internal addresses and file contents removed.

**[Working view](https://herbertofury.github.io/INTA/#job/ai-triage)** · **[Full guide](https://github.com/Herbertofury/INTA/wiki/Job-ai-triage)** · 3 specific fixes · 3 official resources

- **[Create a safe diagnostic excerpt](https://herbertofury.github.io/INTA/#job/ai-triage?section=fix-ai-redact-logs-before-sharing)** — A log or screenshot contains customer names, addresses, tokens or recovery information.
- **[Check a suggested command before running it](https://herbertofury.github.io/INTA/#job/ai-triage?section=fix-ai-command-does-not-match-the-machine)** — An AI answer supplies a command for a different shell, OS version or target.
- **[Keep a log or webpage from changing the task](https://herbertofury.github.io/INTA/#job/ai-triage?section=fix-ai-untrusted-instructions-inside-evidence)** — Retrieved content tells the assistant to ignore instructions, reveal secrets or run unrelated commands.

## How this stays organized

The service groups come from one [canonical layout](https://github.com/Herbertofury/INTA/blob/main/data/wiki-layout.json). New cases extend the existing guide. A new guide needs a distinct job, a complete procedure and a place in this map.

**[Organization rules](Knowledge-Organization)** · **[Coverage and review health](Knowledge-Health)** · **[Research queue](Research-Queue)**
