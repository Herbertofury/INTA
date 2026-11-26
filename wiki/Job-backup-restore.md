[Home](https://github.com/Herbertofury/INTA/wiki) · [All playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Interactive checklist](https://herbertofury.github.io/INTA/#job/backup-restore)

# Prove a backup can actually restore

> Validate recovery credentials, data integrity, application usability, and recovery time.

| Job profile | Details |
| --- | --- |
| Category | Backup & continuity |
| Working time | 30–90 min; full restore varies |
| Experience | Intermediate |

## Start here

Identify what must be recovered, the acceptable data-loss window, and the target recovery time. A green backup status is only the starting point.

## Your toolkit

| Tool | Why it belongs here |
| --- | --- |
| [restic](https://github.com/restic/restic/releases/latest) | Repeatable file backups with explicit integrity and restore checks. |
| [Veeam Agent for Microsoft Windows](https://www.veeam.com/products/free/microsoft-windows) | Image backup and recovery media for a supported Windows endpoint. |
| [Rescuezilla](https://rescuezilla.com/download) | A visual bare-metal backup or restore workflow at the bench. |

## Follow the workflow

### 01. Inventory the recovery set

- [ ] List protected devices, folders, application data, backup destination, retention, encryption keys, and the last successful job. Confirm the person covering an outage can obtain the credentials. Identify data that lives outside the usual Documents/Desktop folders.

### 02. Inspect repository and job health

- [ ] Read recent backup logs, skipped-file reports, capacity, and retention errors. Use the product’s integrity check. For restic, a normal check validates repository structure; reading and verifying stored data requires the additional data-check option and an appropriate maintenance window.

### 03. Select a representative restore

- [ ] Choose a known snapshot and business-relevant files, including a large file and a format used by the client. Restore to an empty separate folder or isolated test machine. Avoid restoring directly over live files during a routine verification.

### 04. Test the content with the real application

- [ ] Open documents, photos, archives, and business data in their intended applications. Check permissions and required metadata. For databases or server workloads, follow the application’s supported recovery procedure and confirm consistency, not just file presence.

### 05. Exercise the outage path

- [ ] For system recovery, boot the recovery media in a test environment and confirm it can see the backup destination and required storage/network hardware. Measure restore time. Verify recovery still works when the original PC or ordinary sign-in method is unavailable.

### 06. Record and resolve gaps

- [ ] Save the restore date, snapshot, scope, integrity result, elapsed time, and evidence in the private service record. Assign an owner to missing data, inaccessible keys, slow restore paths, or capacity problems. Keep backup copies isolated from ordinary endpoint write access where supported.

## Verify before handoff

- [ ] A real restore has completed to a separate target.
- [ ] Restored data opens correctly and required permissions work.
- [ ] Recovery keys, media, network access, and time expectations were tested.
- [ ] Any integrity or coverage gap has an owner and corrective action.

## Stop and escalate when

- A repository check reports corruption or essential credentials are missing.
- The restore plan would overwrite the only good copy or live production data.

## Related playbooks

- [Disk Migration](https://github.com/Herbertofury/INTA/wiki/Job-disk-migration)
- [Outage Recovery](https://github.com/Herbertofury/INTA/wiki/Job-outage-recovery)
- [Ransomware Response](https://github.com/Herbertofury/INTA/wiki/Job-ransomware-response)
- [Preventive Maintenance](https://github.com/Herbertofury/INTA/wiki/Job-preventive-maintenance)

## Official references

- [restic: repository consistency and integrity](https://restic.readthedocs.io/en/stable/045_working_with_repos.html)
- [restic: restore a backup](https://restic.readthedocs.io/en/stable/050_restore.html)

---

INTA Computer Services · Editorial review: 2026-11-26. Timing is an estimate; stop conditions take priority.
