[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Service map](https://github.com/Herbertofury/INTA/wiki/Service-Map) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

# 🗄️ Restore a file backup

**Identify the backup format, snapshot date and recoverable credentials before choosing the restore tool.**

<a id="working-kit"></a>

## 🧰 Tools and links for this job

**[Open working view](https://herbertofury.github.io/INTA/#job/backup-restore/quick/file-backup)** · Keep the tool panel beside the instructions.

**First move:** Identify the backup format, snapshot date and recoverable credentials before choosing the restore tool.

| Use this for | Official tool + exact action |
| --- | --- |
| Restore a file backup | **[restic](https://restic.net/)** · [Official page](https://restic.net/) · **[Download Windows x64 · ZIP](https://github.com/restic/restic/releases/download/v0.20.0/restic_0.20.0_windows_amd64.zip)** |
| Restore an existing supported [Windows](https://www.microsoft.com/windows)-agent backup. | **[Veeam Agent for Microsoft Windows](https://www.veeam.com/products/free/microsoft-windows)** · [Official page](https://www.veeam.com/products/free/microsoft-windows) · **[Sign in for the Windows download](https://www.veeam.com/products/physical/windows-backup-recovery-download.html)** |
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

## ⚡ First three actions

1. For a [restic](https://restic.net/) repository, inspect snapshots and run the documented repository checks.
2. Restore the required snapshot or files to a separate destination using the scoped procedure.
3. Open representative files, check permissions and record the restored date and any missing data.

**[restic](https://restic.net/)** · **[Download Windows x64 · ZIP](https://github.com/restic/restic/releases/download/v0.20.0/restic_0.20.0_windows_amd64.zip)**

**[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-backup-restore)** · **[Specific fixes for this job](https://herbertofury.github.io/INTA/#job/backup-restore/quick/file-backup?section=fixes)** · **[Selection evidence](https://github.com/Herbertofury/INTA/wiki/Free-Tool-Picks)**

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


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
