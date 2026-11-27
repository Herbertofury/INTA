[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

# 💾 Recover a failing drive

**Stop using the source disk; clicking, physical damage or irreplaceable data needs a recovery-specialist decision first.**

<a id="working-kit"></a>

## 🧰 Tools and links for this job

**[Open working view](https://herbertofury.github.io/INTA/#job/failing-drive/quick/failing-drive)** · Keep the tool panel beside the instructions.

**First move:** Stop using the source disk; clicking, physical damage or irreplaceable data needs a recovery-specialist decision first.

| Use this for | Official tool + exact action |
| --- | --- |
| Recover a failing drive | **[SystemRescue](https://www.system-rescue.org/)** · [Official page](https://www.system-rescue.org/) · **[Download x64 rescue image · ISO](https://fastly-cdn.system-rescue.org/releases/13.03/systemrescue-13.03-amd64.iso)** |
| Recover file contents from healthy media or an image. | **[PhotoRec](https://www.cgsecurity.org/wiki/PhotoRec)** · [Official page](https://www.cgsecurity.org/wiki/PhotoRec) · **[Download PhotoRec / TestDisk · ZIP](https://www.cgsecurity.org/testdisk-7.2.win64.zip)** |
| Inspect partitions on a working copy when that branch is justified. | **[TestDisk](https://www.cgsecurity.org/wiki/TestDisk)** · [Official page](https://www.cgsecurity.org/wiki/TestDisk) · **[Download Windows x64 tools · ZIP](https://www.cgsecurity.org/testdisk-7.2.win64.zip)** |

### Preserve the source

| Open the exact resource | What to do there |
| --- | --- |
| **[GNU ddrescue imaging manual](https://www.gnu.org/software/ddrescue/manual/ddrescue_manual.html)** | Use verified devices and a persistent mapfile before file-system repair. |
| **[SystemRescue included recovery tools](https://www.system-rescue.org/System-tools/)** | Confirm the imaging and identification tools in the boot environment. |

### Recover from a copy

| Open the exact resource | What to do there |
| --- | --- |
| **[PhotoRec recovery steps](https://www.cgsecurity.org/wiki/PhotoRec_Step_By_Step)** | Write recovered output to another disk and inspect actual files. |
| **[TestDisk partition analysis](https://www.cgsecurity.org/wiki/TestDisk_Step_By_Step)** | Distinguish inspection from partition-table writes. |

## ⚡ First three actions

1. Boot a separate [SystemRescue](https://www.system-rescue.org/) USB and identify source and destination by model, serial and size.
2. Follow the [GNU ddrescue](https://www.gnu.org/software/ddrescue/) imaging procedure with a persistent mapfile; do not run file-system repairs on the source.
3. Work on an image copy, record unreadable ranges and open the client’s priority files to verify recovery.

**[SystemRescue](https://www.system-rescue.org/)** · **[Download x64 rescue image · ISO](https://fastly-cdn.system-rescue.org/releases/13.03/systemrescue-13.03-amd64.iso)**

**[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive)** · **[Selection evidence](https://github.com/Herbertofury/INTA/wiki/Free-Tool-Picks)**

<a id="next-action"></a>

## 🧭 Match the symptom

| You see this | Do this next | You should get |
| --- | --- | --- |
| **The drive clicks, was dropped or repeatedly disconnects** | Stop repeated boots and scans; record symptoms and have the owner choose specialist recovery. | The remaining recoverability is not consumed by routine repair attempts. |
| **A damaged drive remains readable enough to image** | Boot [SystemRescue](https://www.system-rescue.org/), identify source and destination by model/serial/size and follow [GNU ddrescue](https://www.gnu.org/software/ddrescue/) with a mapfile on persistent healthy storage. | An image and resumable mapfile preserve readable ranges and identify unreadable gaps. |
| **Files were deleted from healthy media** | Check backups first; otherwise run [PhotoRec](https://www.cgsecurity.org/wiki/PhotoRec) against the healthy source or image and save to a separate disk. | Recovered files open successfully; lost names or folder structure are recorded. |
| **The image is encrypted or recovery reports missing ranges** | Obtain the correct recovery material and assess the client’s priority files on a working copy. | Usable files and unrecoverable areas are explicitly distinguished. |

### ✅ Finish the job

- [ ] Source and destination identities are recorded; when imaging was required, the image and mapfile identities are recorded.
- [ ] Priority recovered files open in their actual applications.
- [ ] Any unreadable ranges and missing files are documented before acceptance.

**Change route when:** There is physical damage or an unstable connection that prevents controlled imaging. · A proposed repair would write to the only source or overwrite an unverified destination.


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
