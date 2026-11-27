# Prove the service outcome

[Home](https://github.com/Herbertofury/INTA/wiki) › [Decision Desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) › **[Service evidence](https://github.com/Herbertofury/INTA/wiki/Service-Evidence)**

**Finish with evidence that matches the customer's job and the remaining access or recovery risk.** Copy the relevant record into the approved private ticket. Record references to secrets and sensitive evidence, never the secrets themselves.

Documentation reviewed **November 26, 2026**. The checklists below are <a href="https://herbertofury.github.io/INTA/" title="Open INTA official download or service page" aria-label="Open INTA official download or service page" data-inta-brand="app-inta"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA official page" width="18" height="18" valign="middle" /></a> [INTA](https://herbertofury.github.io/INTA/) working methods based on the linked primary guidance and the explicit findings/coverage distinction in <a href="https://github.com/Herbertofury/Athena" title="Open Athena official download or service page" aria-label="Open Athena official download or service page" data-inta-brand="app-athena"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/99f4edb33272b70febb8.png" alt="Athena official page" width="18" height="18" valign="middle" /></a> [ATHENA's endpoint-security research](https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/engineering/docs/windows-endpoint-security.md).

## Choose the acceptance record

| Job | Record that closes the uncertainty |
| --- | --- |
| [Remote support](https://github.com/Herbertofury/INTA/wiki/Job-remote-support) | Original task result, ended channels and the exact access that remains. |
| [Malware cleanup](https://github.com/Herbertofury/INTA/wiki/Job-malware-removal) | Findings **and** coverage, persistence/account review, and a cleanup, incident or rebuild decision. |
| [Failing-drive recovery](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive) | Acquisition coverage, a preserved image/mapfile checkpoint and separate file-usability evidence. |

## Remote session acceptance

Use [remote capability preflight](https://github.com/Herbertofury/INTA/wiki/Job-remote-support) at the start and [temporary access revocation](https://github.com/Herbertofury/INTA/wiki/Job-remote-support) at closure.

| Private record field | Useful evidence |
| --- | --- |
| Intended endpoint and technician | Customer/device identifier and named support account, checked against the request. |
| Session model | Attended or explicitly managed; client/server selection; permissions actually used. |
| Work proved | Original user task, test performed, result and any remaining limitation. |
| Files handled | Purpose, direction, destination and retention/removal decision. |
| Access closed | End time; desktop, file and terminal channels closed; temporary credentials/enrollments removed if created. |
| Access retained | Named owner, customer assignment, rights and review date for continuing management. |
| Interruption | Last known state, local contact and literal next action if the job stopped early. |

For <a href="https://remotedesktop.google.com/support" title="Open Chrome Remote Desktop official download or service page" aria-label="Open Chrome Remote Desktop official download or service page" data-inta-brand="chrome-remote-desktop"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4a3cfc17be3f4e8ad2c8.png" alt="Chrome Remote Desktop official page" width="18" height="18" valign="middle" /></a> [Chrome Remote Desktop](https://support.google.com/chrome/answer/1649523), ending a support session and disabling a configured persistent connection are different actions. For <a href="https://github.com/Ylianst/MeshCentral" title="Open MeshCentral official download or service page" aria-label="Open MeshCentral official download or service page" data-inta-brand="meshcentral"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4cbfe96e77bec6b7d7f4.png" alt="MeshCentral official page" width="18" height="18" valign="middle" /></a> [MeshCentral](https://docs.meshcentral.com/meshcentral/devicetabs/), check each channel used and review the relevant event record. For <a href="https://apps.microsoft.com/detail/9P7BP5VNWKX5" title="Open Microsoft Quick Assist official download or service page" aria-label="Open Microsoft Quick Assist official download or service page" data-inta-brand="quick-assist"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="Microsoft Quick Assist official page" width="18" height="18" valign="middle" /></a> [Microsoft Quick Assist](https://learn.microsoft.com/en-us/windows/client-management/client-tools/quick-assist), retain your own ticket notes: the documented session does not create local logs on either participant's device.

## Malware decision ladder

Start with the [malware-removal playbook](https://github.com/Herbertofury/INTA/wiki/Job-malware-removal). Keep the decision tied to the observed scope:

| Observed situation | Next decision | Evidence required before changing path |
| --- | --- | --- |
| Isolated unwanted browser software with an understood source and no wider-compromise indicators | Proceed with scoped cleanup and settings review. | Detection and affected browser state, the specific changes made, and a stable retest. |
| A scan reports no detection but was cancelled, blocked, timed out or unable to inspect required areas | Keep the result incomplete; resolve the coverage gap or escalate. | What was and was not scanned, the failure reason and a supported next check. |
| Detections return, unexplained privileged changes remain, or system integrity cannot be established | Preserve findings and prepare a trusted rebuild decision. | Entry-point assessment, recovery scope, preserved data/evidence and an accountable decision owner. |
| Active encryption, wider spread or privileged-account compromise is suspected | Use [incident containment](https://github.com/Herbertofury/INTA/wiki/Job-ransomware-response) and the incident owner. | Affected devices/accounts, containment status and the agreed evidence/recovery priority. |

The table is an <a href="https://herbertofury.github.io/INTA/" title="Open INTA official download or service page" aria-label="Open INTA official download or service page" data-inta-brand="app-inta"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA official page" width="18" height="18" valign="middle" /></a> [INTA](https://herbertofury.github.io/INTA/) operational decision rule. <a href="https://www.microsoft.com/" title="Open microsoft.com official download or service page" aria-label="Open microsoft.com official download or service page" data-inta-brand="site-www-microsoft-com"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="microsoft.com official page" width="18" height="18" valign="middle" /></a> [Microsoft's recovery guidance](https://support.microsoft.com/en-us/windows/experience/backup-recovery/recovery-options-in-windows) identifies installation-media recovery for suspected infection; <a href="https://www.cisa.gov/" title="Open cisa.gov official download or service page" aria-label="Open cisa.gov official download or service page" data-inta-brand="site-www-cisa-gov"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/b9d3cac9cab36524676d.png" alt="cisa.gov official page" width="18" height="18" valign="middle" /></a> [the CISA #StopRansomware Guide](https://www.cisa.gov/sites/default/files/2025-03/StopRansomware-Guide%20508.pdf) treats rebuilt systems, affected accounts, persistence and clean restoration as connected work.

### Record findings and coverage separately

Apply [scan coverage reconciliation](https://github.com/Herbertofury/INTA/wiki/Job-malware-removal) to each product run:

| Field | Record |
| --- | --- |
| Identity | Product, scan mode, engine/intelligence version where exposed, and exact host. |
| Scope | Volumes, folders, boot environment or other areas requested; exclusions and inaccessible areas. |
| Completion | Start/end, complete/cancelled/failed, and any reboot or follow-up still pending. |
| Findings | Detection, affected object, action, result after reboot and any recurrence. |
| Evidence | Product report location and the technician's interpretation with unresolved limits. |

<a href="https://learn.microsoft.com/en-us/defender-endpoint/safety-scanner-download" title="Open Microsoft Safety Scanner official download or service page" aria-label="Open Microsoft Safety Scanner official download or service page" data-inta-brand="safety-scanner"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="Microsoft Safety Scanner official page" width="18" height="18" valign="middle" /></a> [Microsoft Safety Scanner](https://learn.microsoft.com/en-us/defender-endpoint/safety-scanner-download) is an on-demand tool whose download expires after 10 days. Its detailed report is at `%SYSTEMROOT%\debug\msert.log`. Keep that report with the case; it does not replace continuing endpoint protection.

<a href="https://learn.microsoft.com/en-us/defender-endpoint/microsoft-defender-offline" title="Open Microsoft Defender Offline official download or service page" aria-label="Open Microsoft Defender Offline official download or service page" data-inta-brand="defender-offline"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="Microsoft Defender Offline official page" width="18" height="18" valign="middle" /></a> [Microsoft Defender Offline](https://learn.microsoft.com/en-us/defender-endpoint/microsoft-defender-offline) has platform and preparation requirements: [Windows](https://www.microsoft.com/windows) on ARM and [Windows Server](https://learn.microsoft.com/en-us/windows-server/) are excluded; its update route requires <a href="https://learn.microsoft.com/en-us/defender-endpoint/microsoft-defender-antivirus-windows" title="Open Microsoft Defender Antivirus official download or service page" aria-label="Open Microsoft Defender Antivirus official download or service page" data-inta-brand="app-defender-antivirus"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="Microsoft Defender Antivirus official page" width="18" height="18" valign="middle" /></a> [Microsoft Defender Antivirus](https://learn.microsoft.com/en-us/defender-endpoint/microsoft-defender-antivirus-windows) as primary, local administrative access and an enabled recovery environment. Follow its <a href="https://learn.microsoft.com/en-us/windows/security/operating-system-security/data-protection/bitlocker/" title="Open BitLocker official download or service page" aria-label="Open BitLocker official download or service page" data-inta-brand="app-bitlocker"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="BitLocker official page" width="18" height="18" valign="middle" /></a> [BitLocker](https://learn.microsoft.com/en-us/windows/security/operating-system-security/data-protection/bitlocker/) preparation and retain the recovery key through the approved store. Confirm that the scan actually ran and that protection is restored afterward.

### Before a destructive rebuild

Use [clean rebuild acceptance](https://github.com/Herbertofury/INTA/wiki/Job-malware-removal). Record the following before proceeding:

- The exact target and approved reinstall scope, including what will be erased.
- Required case evidence and its private storage reference.
- Which data must survive, where it is preserved and the restore proof already obtained.
- Recovery material, essential applications and device-specific configuration needed for return to service.
- Trusted installation-media provenance and the controlled path for initial updates.
- Who owns any exposed accounts, session revocation and the later reconnect decision.

Follow the <a href="https://support.microsoft.com/" title="Open support.microsoft.com official download or service page" aria-label="Open support.microsoft.com official download or service page" data-inta-brand="site-support-microsoft-com"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="support.microsoft.com official page" width="18" height="18" valign="middle" /></a> [official clean-install instructions](https://support.microsoft.com/en-us/windows/deployment/install-upgrade/reinstall-windows-with-the-installation-media) for the actual device. Preserve the required evidence before erasing a disk. Recovery options that keep old applications and settings are different procedures; do not label one as a clean rebuild.

### Before returning a rebuilt endpoint

| Proof | Passing evidence |
| --- | --- |
| System baseline | The intended installation completed; relevant updates and device support are in place. |
| Endpoint protection | The approved product is healthy, current and working after reboot. |
| Data and applications | Required data opens or restores in its owning application; applications came from trusted sources. |
| Entry point | The known cause is addressed, or the remaining uncertainty has a named escalation owner. |
| Accounts | Exposed credentials and sessions have a separate response record. Reinstalling a PC does not revoke a cloud session. |
| Customer outcome | The user completes the original task; limitations and follow-up are explicit. |

## Failing-drive acquisition acceptance

Follow [image-first recovery](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive). Keep the failing source protected and work on healthy storage. A favorable health indicator does not override physical symptoms or the value of irreplaceable data.

Apply [image and mapfile reconciliation](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive) at each stopped acquisition checkpoint. Record the exact devices, source size, rescue domain, command/options, image path, mapfile path and saved status. The <a href="https://ftp.gnu.org/gnu/ddrescue/" title="Open GNU ddrescue official download or service page" aria-label="Open GNU ddrescue official download or service page" data-inta-brand="ddrescue"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/5599e1400b179d1550a0.png" alt="GNU ddrescue official page" width="18" height="18" valign="middle" /></a> [GNU ddrescue manual](https://www.gnu.org/software/ddrescue/manual/ddrescue_manual.html#Mapfile-structure) defines these mapfile block states:

| Mark | Acquisition meaning |
| --- | --- |
| `+` | Finished range |
| `?` | Not tried |
| `*` | Failed, not trimmed |
| `/` | Failed, not scraped |
| `-` | Bad sectors |

A process marked finished is different from every range being recovered. Preserve the remaining incomplete ranges and the intended rescue domain in the case record. Do not use image size or a nonfatal process exit as a substitute for the mapfile.

### Separate the three kinds of proof

| Layer | What to verify | What it does not prove |
| --- | --- | --- |
| Acquisition coverage | Source/destination identity, intended rescue domain and saved mapfile state. | That every recovered file is usable. |
| Checkpoint identity | After writes stop, hash the image on healthy storage; preserve its matching mapfile and verify any working-copy hash. | That missing source sectors were recovered or that the original filesystem was consistent. |
| User-data usability | Open agreed representative files; validate priority databases/archives with the owning application on a copy. | That untested files are intact. |

Do not reread the failing source just to obtain a comparison hash. A checksum protects the identity of the acquired copy; it cannot fill a coverage gap. If another imaging pass changes the image, retain the earlier checkpoint identity and record the new image/mapfile pair.

The acceptance record should include **recovered priorities, tested samples, failed checks, known missing data and the image-retention decision**. Escalate when symptoms worsen, device identity is uncertain, or the data value exceeds the agreed bench attempt. Keep the original recovery image unchanged during filesystem investigation and extraction.

**Continue:** [Failing-drive playbook](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive) · [Tool Selection](https://github.com/Herbertofury/INTA/wiki/Tool-Selection) · [Technician handoff](https://github.com/Herbertofury/INTA/wiki/Technician-Handoff)
