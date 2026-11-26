[Home](https://github.com/Herbertofury/INTA/wiki) · [All playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Interactive checklist](https://herbertofury.github.io/INTA/#job/new-pc)

# Set up a dependable new workstation

> Build a usable, recoverable workstation around the employee’s actual job.

| Job profile | Details |
| --- | --- |
| Category | Deployment |
| Working time | 60–180 min |
| Experience | Intermediate |

## Start here

Collect the job role, approved applications, identity/join requirements, data sources, printers, and acceptance tests before provisioning.

## Your toolkit

| Tool | Why it belongs here |
| --- | --- |
| [WinGet](https://github.com/microsoft/winget-cli/releases/latest) | Repeatable application setup and individually reviewed updates. |
| [Microsoft PowerToys](https://github.com/microsoft/PowerToys/releases/latest) | Making the technician workstation easier to navigate and operate. |
| [Veeam Agent for Microsoft Windows](https://www.veeam.com/products/free/microsoft-windows) | Image backup and recovery media for a supported Windows endpoint. |
| [RustDesk](https://github.com/rustdesk/rustdesk/releases/latest) | A flexible remote-control option across mixed client platforms. |
| [MeshCentral](https://github.com/Ylianst/MeshCentral) | INTA-managed unattended support with its own server and device groups. |

## Follow the workflow

### 01. Record the new device and baseline

- [ ] Capture model, serial, warranty, Windows edition/build, firmware, storage, and memory in the private asset record. Use the supported OS and vendor driver path. Confirm hardware compatibility before applying firmware or feature updates.

### 02. Establish identity and recovery

- [ ] Give the device a unique approved name and join the intended management/domain system. Set up the employee’s standard account and approved administrative access. Configure encryption and verify recovery-key escrow through the organization’s chosen management system.

### 03. Install the approved application set

- [ ] Use official installers or reviewed WinGet package IDs. An exported WinGet list can help migration, but unmatched apps, settings, licenses, and data still need separate handling. Review the package list before importing; do not blindly reproduce obsolete or unwanted software.

### 04. Migrate and verify data

- [ ] Restore approved user data from a validated backup or migration source. Preserve folder structure and permissions, and open the client’s important files. Do not overwrite the source computer until the new device has passed acceptance.

### 05. Finish the operational services

- [ ] Configure the single intended real-time security product, backup, printers, VPN, email, browser settings, and approved remote management. Confirm notifications and background jobs fit the employee’s working hours without disabling essential protection.

### 06. Run a user-led acceptance session

- [ ] Have the employee sign in and complete the real tasks: open files, send and receive email, print, reach shared resources, and restart. Prove a small backup restore. Record any remaining app or access gap with an owner and due date.

## Verify before handoff

- [ ] The employee completes the agreed business tasks.
- [ ] Encryption recovery, backup restore, and approved remote support are verified.
- [ ] The old source remains available until the migration is accepted.

## Stop and escalate when

- Required licensing, access, recovery keys, or data sources are unresolved.
- A migration action would destroy the only good source.

## Related playbooks

- [Disk Migration](disk-migration.md)
- [Backup Restore](backup-restore.md)
- [Remote Support](remote-support.md)
- [Client Handoff](client-handoff.md)

## Official references

- [Microsoft: WinGet export](https://learn.microsoft.com/en-us/windows/package-manager/winget/export)
- [Microsoft: WinGet import](https://learn.microsoft.com/en-us/windows/package-manager/winget/import)

---

INTA Computer Services · Editorial review: 2026-11-26. Timing is an estimate; stop conditions take priority.
