[Home](https://github.com/Herbertofury/INTA/wiki) · [All playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Interactive checklist](https://herbertofury.github.io/INTA/#job/preventive-maintenance)

# Run a useful workstation maintenance visit

> Check security, updates, hardware, backups, and usability without unnecessary cleanup rituals.

| Job profile | Details |
| --- | --- |
| Category | Maintenance |
| Working time | 30–60 min |
| Experience | Intermediate |

## Start here

Agree on the maintenance window and identify the user’s current pain points. Preserve a baseline of the system’s normal work.

## Your toolkit

| Tool | Why it belongs here |
| --- | --- |
| [WinGet](https://github.com/microsoft/winget-cli/releases/latest) | Repeatable application setup and individually reviewed updates. |
| [CrystalDiskInfo](https://crystalmark.info/en/download/) | First-pass HDD and SSD health inspection in Windows. |
| [HWiNFO](https://www.hwinfo.com/download/) | Investigating temperatures, clocks, power and component identity. |
| [Microsoft Sysinternals Suite](https://learn.microsoft.com/en-us/sysinternals/downloads/sysinternals-suite) | The core portable Windows troubleshooting kit. |
| [Veeam Agent for Microsoft Windows](https://www.veeam.com/products/free/microsoft-windows) | Image backup and recovery media for a supported Windows endpoint. |

## Follow the workflow

### 01. Review health and recurring failures

- [ ] Check Reliability Monitor, storage health, available capacity, temperatures, and recent support history. Separate actionable errors from harmless noise. Investigate symptoms that affect the user before scheduling broad changes.

### 02. Verify protection and access

- [ ] Confirm the intended real-time security product is healthy and current, firewall policy is intact, and approved remote-management access belongs to the right people. Review unwanted startup apps and browser extensions with the user; remove only what is understood.

### 03. Review updates before applying them

- [ ] Check Windows and application updates, reboot requirements, vendor advisories, and line-of-business compatibility. WinGet with no target lists available upgrades; review that list and update the intended packages rather than blindly upgrading every application during production work.

```text
winget upgrade
```

### 04. Check backups through a restore

- [ ] Read backup logs and capacity, then restore a small representative item to a separate folder. Confirm recovery-key access and that a coworker can follow the recovery path. If the backup is not restorable, treat that as a repair task with an owner.

### 05. Handle storage and physical upkeep

- [ ] Use the system’s supported storage cleanup controls after reviewing what will be removed. Preserve downloads, local mail, and application data that may be unique. Clean physical dust using the device’s service guidance; avoid registry cleaners and indiscriminate driver-updater bundles.

### 06. Retest and schedule only real follow-up

- [ ] After the planned restart, test the user’s main applications, network, printers, backup, and remote access. Record versions and changes, measurable improvements, deferred work, and hardware replacement recommendations based on evidence.

## Verify before handoff

- [ ] Security, updates, backup restore, and hardware health were reviewed.
- [ ] The normal workload works after maintenance and restart.
- [ ] Every deferred or failed item has an owner and next action.

## Stop and escalate when

- The maintenance would interrupt an unplanned critical workload.
- Storage failure or active compromise changes the job into recovery or incident response.

## Related playbooks

- [Slow Pc](slow-pc.md)
- [Windows Update](windows-update.md)
- [Backup Restore](backup-restore.md)
- [Client Handoff](client-handoff.md)

## Official references

- [Microsoft: WinGet upgrade](https://learn.microsoft.com/en-us/windows/package-manager/winget/upgrade)
- [restic: restore a backup](https://restic.readthedocs.io/en/stable/050_restore.html)

---

INTA Computer Services · Editorial review: 2026-11-26. Timing is an estimate; stop conditions take priority.
