[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Service map](https://github.com/Herbertofury/INTA/wiki/Service-Map) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

# 🔁 Clone or move a drive

**Confirm a healthy source, a verified backup and a destination large enough for the selected clone or restore.**

<a id="working-kit"></a>

## 🧰 Tools and links for this job

**[Open working view](https://herbertofury.github.io/INTA/#job/disk-migration/quick/clone-a-drive)** · Keep the tool panel beside the instructions.

**First move:** Confirm a healthy source, a verified backup and a destination large enough for the selected clone or restore.

| Use this for | Official tool + exact action |
| --- | --- |
| Clone or move a drive | **[Rescuezilla](https://rescuezilla.com/)** · [Official page](https://rescuezilla.com/) · **[Download x64 rescue image · ISO](https://github.com/rescuezilla/rescuezilla/releases/download/2.6.2/rescuezilla-2.6.2-64bit.resolute.iso)** |
| Copy approved file trees with an inspectable log. | **[Robocopy](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy)** · [Official page](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy) · **[Open built-in copy instructions](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy)** |
| Inventory and reapply approved application packages. | **[WinGet](https://learn.microsoft.com/en-us/windows/package-manager/winget/)** · [Official page](https://learn.microsoft.com/en-us/windows/package-manager/winget/) · **[Download App Installer · MSIXBUNDLE](https://github.com/microsoft/winget-cli/releases/download/v1.29.380/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle)** |
| Synchronize an explicitly chosen healthy-data folder between approved machines; keep an independent backup. | **[Syncthing](https://syncthing.net/)** · [Official page](https://syncthing.net/) · **[Download Windows x64 · ZIP](https://github.com/syncthing/syncthing/releases/download/v2.1.7/syncthing-windows-amd64-v2.1.7.zip)** |
| Comparing a saved configuration with the changed version or checking a copied folder before accepting a repair or migration. | **[WinMerge](https://winmerge.org/)** · [Official page](https://winmerge.org/) · **[Download Windows x64 portable · ZIP](https://github.com/WinMerge/winmerge/releases/download/v2.16.60/winmerge-2.16.60-x64-exe.zip)** |

### Choose the move

| Open the exact resource | What to do there |
| --- | --- |
| **[Rescuezilla migration limitations](https://rescuezilla.com/help)** | Plan smaller-disk restores and preserve an unmodified backup. |

### File migration

| Open the exact resource | What to do there |
| --- | --- |
| **[Robocopy reference](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy)** | Review source, destination and copy flags before execution. |

### Application setup

| Open the exact resource | What to do there |
| --- | --- |
| **[WinGet export](https://learn.microsoft.com/en-us/windows/package-manager/winget/export)** | Export supported application identities for later review. |
| **[WinGet import](https://learn.microsoft.com/en-us/windows/package-manager/winget/import)** | Review and install approved packages; user data is separate. |

## ⚡ First three actions

1. Boot [Rescuezilla](https://rescuezilla.com/) from a separate USB and identify every source and destination by model, serial and size.
2. Choose the intended clone or image operation; confirm the destination may be overwritten before starting.
3. Boot the destination alone and verify files, applications, encryption recovery and backup operation.

**[Rescuezilla](https://rescuezilla.com/)** · **[Download x64 rescue image · ISO](https://github.com/rescuezilla/rescuezilla/releases/download/2.6.2/rescuezilla-2.6.2-64bit.resolute.iso)**

**[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-disk-migration)** · **[Specific fixes for this job](https://herbertofury.github.io/INTA/#job/disk-migration/quick/clone-a-drive?section=fixes)** · **[Selection evidence](https://github.com/Herbertofury/INTA/wiki/Free-Tool-Picks)**

<a id="next-action"></a>

## 🧭 Match the symptom

| You see this | Do this next | You should get |
| --- | --- | --- |
| **The source shows read errors or instability** | Change route to [image-first recovery](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive) rather than treating the operation as routine migration. | The source is preserved before intensive copy or resize work. |
| **The destination is smaller than the source layout** | Check the tool’s supported geometry and create a rollback image before any separately planned resize. | The target layout fits by evidence, not merely by free-space assumptions. |
| **A complete healthy drive must be moved** | Boot [Rescuezilla](https://rescuezilla.com/), verify source/destination identities, then confirm the deliberate clone or restore operation. | The operation completes and the destination can be booted by itself. |
| **Only approved files need moving** | Choose exact source and destination folders, review [Robocopy](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy) flags and retain its result log; do not use a mirror/delete option by default. | Expected files, timestamps and access are verified at the new location. |
| **The copied system is ready for release** | Boot with the old disk disconnected and test sign-in, applications, encryption recovery and backup. | The new system works independently and the old disk remains the agreed rollback copy. |

### ✅ Finish the job

- [ ] The destination boots independently or the copied file set passes application checks.
- [ ] Required access, recovery material and backup work on the destination.

**Change route when:** Source/destination identity or erase permission is ambiguous. · Storage failure or incompatible target geometry makes the planned migration unsafe.


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
