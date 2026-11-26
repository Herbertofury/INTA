[Home](https://github.com/Herbertofury/INTA/wiki) · [All playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Interactive checklist](https://herbertofury.github.io/INTA/#job/disk-migration)

# Clone a healthy system or migrate to a new SSD

> Move a working system with clear source/destination control and a rollback copy.

| Job profile | Details |
| --- | --- |
| Category | Storage & recovery |
| Working time | 45–180 min |
| Experience | Intermediate |

## Start here

Verify the source drive is healthy. A failing drive needs the recovery workflow instead of an ordinary clone.

## Your toolkit

| Tool | Why it belongs here |
| --- | --- |
| [Rescuezilla](https://rescuezilla.com/download) | A visual bare-metal backup or restore workflow at the bench. |
| [Clonezilla](https://clonezilla.org/downloads.php) | Capturing or restoring a supported disk image with a repeatable procedure. |
| [CrystalDiskInfo](https://crystalmark.info/en/download/) | First-pass HDD and SSD health inspection in Windows. |
| [WinGet](https://github.com/microsoft/winget-cli/releases/latest) | Repeatable application setup and individually reviewed updates. |

## Follow the workflow

### 01. Inventory before the move

- [ ] Record source and destination model/serial/capacity, firmware mode, partition layout, encryption, and the applications that must survive. Confirm the chosen tool supports the filesystem and intended target size. Shrinking to a smaller drive needs a specifically supported procedure.

### 02. Create an independent rollback

- [ ] Make a verified file backup and save recovery-key access. Export application inventory and record licenses or sign-in requirements through the client’s approved process. Keep the old drive intact until the new system is accepted.

### 03. Prepare the destination and media

- [ ] Use current trusted rescue media and connect stable power. Disconnect unrelated removable storage where practical. Match source and target by serial number, not only the drive letter or position in a list; cloning overwrites the selected target.

### 04. Perform and verify the copy

- [ ] Use the imaging application’s supported clone or image/restore workflow. Review the source, target, partition plan, and confirmation screen before committing. Keep the operation log and use the tool’s image verification feature when creating an image.

### 05. Boot the replacement on its own

- [ ] For the first boot, disconnect or otherwise isolate the original system disk so you can prove the replacement is the actual boot device. Resolve any driver or boot-mode issue through supported vendor guidance. Check encryption state after the migration.

### 06. Validate applications and rollback readiness

- [ ] Test the real business files, sign-in, network, printers, backup, and restart. Confirm usable capacity and partition layout. Retain the original drive or image for the agreed acceptance window; erase it only through a separately authorized disposal task.

## Verify before handoff

- [ ] The new drive boots independently.
- [ ] Required data, applications, peripherals, and backup pass the client’s tests.
- [ ] The old source remains usable as a rollback until acceptance.

## Stop and escalate when

- The source exhibits I/O errors or unhealthy behavior.
- The selected tool cannot support the target size or partition/encryption layout.

## Related playbooks

- [Failing Drive](failing-drive.md)
- [Backup Restore](backup-restore.md)
- [New Pc](new-pc.md)
- [Client Handoff](client-handoff.md)

## Official references

- [Clonezilla: imaging and clone walkthroughs](https://clonezilla.org/clonezilla-live-doc.php)
- [Microsoft: recovery options in Windows](https://support.microsoft.com/en-us/windows/experience/backup-recovery/recovery-options-in-windows)
- [Microsoft: WinGet export](https://learn.microsoft.com/en-us/windows/package-manager/winget/export)

---

INTA Computer Services · Editorial review: 2026-11-26. Timing is an estimate; stop conditions take priority.
