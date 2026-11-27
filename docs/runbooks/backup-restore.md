[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

<a id="-prove-a-backup-can-actually-restore"></a>

# [Prove a backup can actually restore](https://github.com/Herbertofury/INTA/wiki/Job-backup-restore)

Validate recovery credentials, data integrity, application usability, and recovery time.

**Backup & continuity** · Intermediate · 30–90 min; full restore varies · 6 steps

[Tools and useful links](#working-kit) · [Quick answer](#quick-answer) · [Next action](#next-action) · [Decision route](#decision-route) · [Tools](#toolkit) · [Procedure](#walkthrough) · [Verification](#acceptance) · [Sources](#sources) · [Interactive checklist](https://herbertofury.github.io/INTA/#job/backup-restore)

<a id="working-kit"></a>

## 🧰 Tools and links for this job

**[Open working view](https://herbertofury.github.io/INTA/#job/backup-restore)** · Keep the tool panel beside the instructions.

**First move:** Identify the existing backup product, repository, recovery point and credentials. Use the matching restore tool rather than converting formats during an incident.

| Use this for | Official tool + exact action |
| --- | --- |
| Restore an existing supported [Windows](https://www.microsoft.com/windows)-agent backup. | **[Veeam Agent for Microsoft Windows](https://www.veeam.com/products/free/microsoft-windows)** · [Official page](https://www.veeam.com/products/free/microsoft-windows) · **[Sign in for the Windows download](https://www.veeam.com/products/physical/windows-backup-recovery-download.html)** |
| Inspect and restore an existing [restic](https://restic.net/) repository. | **[restic](https://restic.net/)** · [Official page](https://restic.net/) · **[Download Windows x64 · ZIP](https://github.com/restic/restic/releases/download/v0.20.0/restic_0.20.0_windows_amd64.zip)** |
| Restore files from an existing [Kopia](https://kopia.io/) snapshot. | **[Kopia](https://kopia.io/)** · [Official page](https://kopia.io/) · **[Download Windows desktop · ZIP](https://github.com/kopia/kopia/releases/download/v0.23.1/KopiaUI-0.23.1-win.zip)** |

### File snapshots

| Open the exact resource | What to do there |
| --- | --- |
| **[restic repository checks](https://restic.readthedocs.io/en/stable/045_working_with_repos.html)** | Check repository integrity before declaring a backup usable. |
| **[restic restore procedure](https://restic.readthedocs.io/en/stable/050_restore.html)** | Choose the snapshot and a separate restore destination. |
| **[Kopia snapshot restoration](https://kopia.io/docs/getting-started/)** | Use the GUI snapshot browser or documented CLI restore to a separate folder. |

### System backups

| Open the exact resource | What to do there |
| --- | --- |
| **[Start Veeam file-level restore](https://helpcenter.veeam.com/docs/agentforwindows/userguide/files_restore_launch.html)** | Launch the restore wizard and choose the existing backup and recovery point. |
| **[Veeam recovery media and bare-metal restore](https://helpcenter.veeam.com/docs/agentforwindows/userguide/recovery_media.html)** | Use compatible recovery media for an existing image backup; verify the target before overwrite. |

<a id="quick-answer"></a>

## ⚡ Quick answer

<a id="quick-file-backup"></a>

### 🗄️ Restore a file backup

**Identify the backup format, snapshot date and recoverable credentials before choosing the restore tool.**

1. For a [restic](https://restic.net/) repository, inspect snapshots and run the documented repository checks.
2. Restore the required snapshot or files to a separate destination using the scoped procedure.
3. Open representative files, check permissions and record the restored date and any missing data.

**[restic](https://restic.net/)** · [Official page](https://restic.net/) · **[Download Windows x64 · ZIP](https://github.com/restic/restic/releases/download/v0.20.0/restic_0.20.0_windows_amd64.zip)**

<a id="next-action"></a>

## 🧭 Match the symptom

| You see this | Do this next | You should get |
| --- | --- | --- |
| **A file or folder must be restored** | Select the correct device and snapshot date in the matching backup tool and restore to a separate location. | The expected version exists without overwriting the current working copy. |
| **The existing backup is a [Veeam Agent](https://www.veeam.com/products/free/microsoft-windows) backup** | Open File Level Restore from the Veeam Start-menu entry, select the [backup and restore](https://github.com/Herbertofury/INTA/wiki/Job-backup-restore) point, then copy the required files to a separate destination in the backup browser. | The restored version opens correctly before any working file is replaced. |
| **The existing backup is a [Kopia](https://kopia.io/) snapshot** | Connect KopiaUI to the existing repository, open Snapshots, select the source Path and dated snapshot, then restore the required item to a separate folder. | The selected files exist in the restore folder and open with the expected contents. |
| **A [restic](https://restic.net/) backup reports success but has not been tested** | Inspect its snapshots, run the documented repository check and restore representative files to a separate target. | The recovery point and usable contents are demonstrated rather than inferred from a green job. |
| **A whole computer must be recovered** | Use that backup product’s compatible recovery media and check storage/network drivers before selecting the erase target. | The restored system boots and its critical application opens the expected data. |
| **A password, repository or required snapshot is missing** | Record the exact missing dependency and contact its owner before modifying the repository. | The recovery gap has an owner and the remaining copies stay intact. |

### ✅ Finish the job

- [ ] The requested recovery date and a real file/application test are recorded.
- [ ] Permissions and ownership allow the intended user to work.
- [ ] Measured restore time and missing data are documented.

**Change route when:** The only backup would be overwritten or exposed to an active compromise. · Required credentials or a valid recovery point cannot be established.

> [!NOTE]
> **Start here.** Identify what must be recovered, the acceptable data-loss window, and the target recovery time. A green backup status is only the starting point.

<a id="decision-route"></a>

<a id="-choose-the-right-branch"></a>

## 🧭 Choose a route

```mermaid
flowchart TD
    A["Define recovery data and time targets"]:::action --> B{"Credentials and backup accessible?"}:::decision
    B -->|"No"| C["Resolve recovery access now"]:::stop
    B -->|"Yes"| D["Inspect job health and repository integrity"]:::action
    D --> E{"Integrity errors?"}:::decision
    E -->|"Yes"| F["Protect copies and repair the backup path"]:::stop
    E -->|"No"| G["Restore to a separate empty target"]:::action
    G --> H{"Real application opens restored data?"}:::decision
    H -->|"No"| F
    H -->|"Yes"| I["Test outage access and measure recovery"]:::action
    I --> J["Record coverage, timing, and remaining gaps"]:::done
    classDef decision fill:#eaf0f8,stroke:#607fa7,color:#172b46
    classDef action fill:#f1f3f6,stroke:#8792a2,color:#263445
    classDef stop fill:#faf0f0,stroke:#a46b70,color:#5e3137
    classDef done fill:#e9edf3,stroke:#6c7d94,color:#253449
```

**Route links:** [Continue this workflow](#walkthrough) · [Acceptance checks](#acceptance) · [Clone a healthy system or migrate to a new SSD](disk-migration.md) · [Bring an office back after a power outage](outage-recovery.md) · [Contain ransomware or a wider compromise](ransomware-response.md) · [Run a useful workstation maintenance visit](preventive-maintenance.md)

| Checkpoint | What to carry forward |
| --- | --- |
| **Before changing anything** | Identify recovery credentials, unique data, and a separate test target. |
| **Keep during the job** | Snapshot, integrity result, restore log, elapsed time, and test evidence. |
| **Before handing back** | Real restored data opens and the outage path is usable. |

<a id="toolkit"></a>

<a id="-build-the-right-toolkit"></a>

## Recommended tools

Select a tool for the next diagnostic or repair step. Check its platform, access and licensing requirements before use.

| Tool / official page + download | Use it for | Platform & access | Documentation |
| --- | --- | --- | --- |
| **[Veeam Agent for Microsoft Windows](https://www.veeam.com/products/free/microsoft-windows)** · [Official page](https://www.veeam.com/products/free/microsoft-windows) · **[Sign in for the Windows download](https://www.veeam.com/products/physical/windows-backup-recovery-download.html)** | Restore an existing supported [Windows](https://www.microsoft.com/windows)-agent backup. | [Windows](https://www.microsoft.com/windows); Free and paid editions; service-provider terms differ | [Open guide](https://helpcenter.veeam.com/docs/agentforwindows/userguide/overview.html) |
| **[restic](https://restic.net/)** · [Official page](https://restic.net/) · **[Download Windows x64 · ZIP](https://github.com/restic/restic/releases/download/v0.20.0/restic_0.20.0_windows_amd64.zip)** | Inspect and restore an existing [restic](https://restic.net/) repository. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos); Free / open source | [Open guide](https://restic.readthedocs.io/en/stable/) |
| **[Kopia](https://kopia.io/)** · [Official page](https://kopia.io/) · **[Download Windows desktop · ZIP](https://github.com/kopia/kopia/releases/download/v0.23.1/KopiaUI-0.23.1-win.zip)** | Restore files from an existing [Kopia](https://kopia.io/) snapshot. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos); Free / open source | [Open guide](https://kopia.io/docs/) |
| **[Rescuezilla](https://rescuezilla.com/)** · [Official page](https://rescuezilla.com/) · **[Download x64 rescue image · ISO](https://github.com/rescuezilla/rescuezilla/releases/download/2.6.2/rescuezilla-2.6.2-64bit.resolute.iso)** | A visual bare-metal backup or restore workflow at the bench. | Bootable PC; Free / open source | [Open guide](https://rescuezilla.com/help) |

<a id="walkthrough"></a>

<a id="-work-the-case"></a>

## Procedure

Work in order, record the result of each step, and change route when the evidence calls for it.

<a id="step-01"></a>

### 01 · Inventory the recovery set

- [ ] List protected devices, folders, application data, backup destination, retention, encryption keys, and the last successful job. Confirm the person covering an outage can obtain the credentials. Identify data that lives outside the usual Documents/Desktop folders.

<a id="step-02"></a>

### 02 · Inspect repository and job health

- [ ] Read recent backup logs, skipped-file reports, capacity, and retention errors. Use the product’s integrity check. For [restic](https://restic.net/), a normal check validates repository structure; reading and verifying stored data requires the additional data-check option and an appropriate maintenance window.

<a id="step-03"></a>

### 03 · Select a representative restore

- [ ] Choose a known snapshot and business-relevant files, including a large file and a format used by the client. Restore to an empty separate folder or isolated test machine. Avoid restoring directly over live files during a routine verification.

<a id="step-04"></a>

### 04 · Test the content with the real application

- [ ] Open documents, photos, archives, and business data in their intended applications. Check permissions and required metadata. For databases or server workloads, follow the application’s supported recovery procedure and confirm consistency, not just file presence.

<a id="step-05"></a>

### 05 · Exercise the outage path

- [ ] For system recovery, boot the recovery media in a test environment and confirm it can see the backup destination and required storage/network hardware. Measure restore time. Verify recovery still works when the original PC or ordinary sign-in method is unavailable.

<a id="step-06"></a>

### 06 · Record and resolve gaps

- [ ] Save the restore date, snapshot, scope, integrity result, elapsed time, and evidence in the private service record. Assign an owner to missing data, inaccessible keys, slow restore paths, or capacity problems. Keep backup copies isolated from ordinary endpoint write access where supported.

<a id="acceptance"></a>

<a id="-prove-the-result"></a>

## Verification

- [ ] A real restore has completed to a separate target.
- [ ] Restored data opens correctly and required permissions work.
- [ ] Recovery keys, media, network access, and time expectations were tested.
- [ ] Any integrity or coverage gap has an owner and corrective action.

> [!WARNING]
> **Pause this route and escalate when:**
> - A repository check reports corruption or essential credentials are missing.
> - The restore plan would overwrite the only good copy or live production data.

<a id="-continue-with-the-evidence"></a>

## Related playbooks

| Next playbook | What it resolves |
| --- | --- |
| [Clone a healthy system or migrate to a new SSD](disk-migration.md) | Move a working system with clear source/destination control and a rollback copy. |
| [Bring an office back after a power outage](outage-recovery.md) | Restore dependencies in order and verify services before releasing the office back to work. |
| [Contain ransomware or a wider compromise](ransomware-response.md) | Stop spread, preserve useful evidence, and restore services through a clean recovery path. |
| [Run a useful workstation maintenance visit](preventive-maintenance.md) | Check security, updates, hardware, backups, and usability without unnecessary cleanup rituals. |

<a id="sources"></a>

<a id="-source-trail"></a>

## Official sources

- [restic: repository consistency and integrity](https://restic.readthedocs.io/en/stable/045_working_with_repos.html)
- [restic: restore a backup](https://restic.readthedocs.io/en/stable/050_restore.html)

Reviewed **2026-11-26**. Timing is a planning range. A successful command is evidence about that operation; acceptance belongs to the user’s original task.


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)

[← All playbooks](README.md) · [Live Wiki](https://github.com/Herbertofury/INTA/wiki) · [Contribution guide](https://github.com/Herbertofury/INTA/blob/main/CONTRIBUTING.md)
