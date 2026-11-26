[Home](https://github.com/Herbertofury/INTA/wiki) · [All playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Interactive checklist](https://herbertofury.github.io/INTA/#job/boot-recovery)

# Recover a Windows PC that will not boot

> Separate firmware, storage, encryption, and Windows startup faults before changing the boot configuration.

| Job profile | Details |
| --- | --- |
| Category | Recovery & boot |
| Working time | 30–120 min |
| Experience | Advanced |

## Start here

Record the exact screen and failure stage. Obtain the device’s recovery-key access path before any firmware, TPM, or boot changes.

## Your toolkit

| Tool | Why it belongs here |
| --- | --- |
| [Windows Recovery Environment](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference) | A Windows machine that will not boot or needs supported recovery options. |
| [Windows 11 installation media](https://www.microsoft.com/software-download/windows11) | A clean installation or supported repair using the correct Windows edition and architecture. |
| [Ventoy](https://www.ventoy.net/en/download.html) | Maintaining a technician USB with several independently updated rescue images. |
| [Rufus](https://rufus.ie/#download) | Preparing one ISO on a dedicated USB drive. |
| [SystemRescue](https://www.system-rescue.org/Download/) | A ready-made Linux recovery environment for an unbootable or damaged machine. |
| [Hiren's BootCD PE](https://www.hirensbootcd.org/download/) | A familiar Windows-oriented offline bench toolkit. |

## Follow the workflow

### 01. Identify the failure stage

- [ ] Check whether the system powers on, completes firmware POST, detects its drive, reaches Windows Boot Manager, or fails after the Windows logo. Disconnect only nonessential peripherals. A missing drive or hardware error goes to hardware or failing-drive diagnosis.

### 02. Protect data and encryption access

- [ ] Confirm the BitLocker recovery key is available through the customer’s approved account or management system. Never publish it in the wiki or ticket attachments visible to unrelated staff. Do not clear the TPM, initialize the disk, or reinstall over the only data copy.

### 03. Enter Windows Recovery Environment

- [ ] Use the built-in recovery environment or official Windows installation media and select Repair my PC. Check the actual volume layout and encryption status before working on an offline installation; drive letters can differ from the normal Windows session.

### 04. Try the narrow recovery option

- [ ] Use Startup Repair for a startup fault. If the failure follows a known update or driver change, consider the relevant update removal or restore option after checking its effect on data. Review the result before trying another repair.

### 05. Reserve manual boot edits for a confirmed diagnosis

- [ ] If boot files or the EFI partition require repair, follow Microsoft’s procedure for the observed firmware mode, disk layout, and Windows installation. Have another technician verify the selected partitions. Do not paste generic bootrec, bcdboot, diskpart, or format sequences.

### 06. Boot, validate, and preserve a recovery path

- [ ] Test a normal boot and a second restart, confirm encryption is healthy, and run the real business workflow. Record the root cause and boot changes. If recovery remains uncertain, export or image the data before planning a supported reinstall.

## Verify before handoff

- [ ] The correct Windows installation boots reliably twice.
- [ ] User data, required applications, and encryption remain accessible.
- [ ] Recovery-key escrow and backup status are confirmed.

## Stop and escalate when

- The recovery key or correct target disk cannot be established.
- The drive shows physical failure or the proposed action would overwrite the only copy.

## Related playbooks

- [Failing Drive](failing-drive.md)
- [Boot Usb](boot-usb.md)
- [Windows Repair](windows-repair.md)
- [Backup Restore](backup-restore.md)

## Official references

- [Microsoft: Windows recovery environment](https://support.microsoft.com/en-us/windows/experience/backup-recovery/windows-recovery-environment)
- [Microsoft: startup troubleshooting](https://learn.microsoft.com/en-us/windows/client-management/advanced-troubleshooting-boot-problems)

---

INTA Computer Services · Editorial review: 2026-11-26. Timing is an estimate; stop conditions take priority.
