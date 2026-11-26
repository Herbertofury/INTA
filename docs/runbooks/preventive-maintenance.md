[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

<a id="-run-a-useful-workstation-maintenance-visit"></a>

# [Run a useful workstation maintenance visit](https://github.com/Herbertofury/INTA/wiki/Job-preventive-maintenance)

Check security, updates, hardware, backups, and usability without unnecessary cleanup rituals.

**Maintenance** · Intermediate · 30–60 min · 6 steps

[Decision route](#decision-route) · [Tools](#toolkit) · [Procedure](#walkthrough) · [Verification](#acceptance) · [Sources](#sources) · [Interactive checklist](https://herbertofury.github.io/INTA/#job/preventive-maintenance)

> [!NOTE]
> **Start here.** Agree on the maintenance window and identify the user’s current pain points. Preserve a baseline of the system’s normal work.

<a id="decision-route"></a>

<a id="-choose-the-right-branch"></a>

## 🧭 Choose a route

<details>
<summary>View the decision map and escalation routes</summary>

```mermaid
flowchart TD
    A["Agree window and capture normal baseline"]:::action --> B{"Active compromise or failing storage?"}:::decision
    B -->|"Yes"| C["Switch to incident or recovery work"]:::stop
    B -->|"No"| D["Review security, health, and updates"]:::action
    D --> E{"Update fits the business workload?"}:::decision
    E -->|"No"| F["Defer with cause and owner"]:::action
    E -->|"Yes"| G["Apply reviewed, scoped updates"]:::action
    F --> H["Prove a restore and check physical upkeep"]:::action
    G --> H
    H --> I{"Normal workload passes after restart?"}:::decision
    I -->|"No"| J["Resolve or roll back the change"]:::action
    I -->|"Yes"| K["Record health and owned follow-up"]:::done
    classDef decision fill:#eaf0f8,stroke:#607fa7,color:#172b46
    classDef action fill:#f1f3f6,stroke:#8792a2,color:#263445
    classDef stop fill:#faf0f0,stroke:#a46b70,color:#5e3137
    classDef done fill:#e9edf3,stroke:#6c7d94,color:#253449
```

**Route links:** [Continue this workflow](#walkthrough) · [Acceptance checks](#acceptance) · [Diagnose a slow, freezing, or hitching PC](slow-pc.md) · [Fix a failing Windows update](windows-update.md) · [Prove a backup can actually restore](backup-restore.md) · [Close a repair with a clear client handoff](client-handoff.md)

</details>

| Checkpoint | What to carry forward |
| --- | --- |
| **Before changing anything** | Agree the window and preserve normal-workload evidence. |
| **Keep during the job** | Health findings, reviewed updates, restore result, and deferred changes. |
| **Before handing back** | Normal work passes after restart and every open item has an owner. |

<a id="toolkit"></a>

<a id="-build-the-right-toolkit"></a>

## Recommended tools

Select a tool for the next diagnostic or repair step. Check its platform, access and licensing requirements before use.

| Tool / official download | Use it for | Platform & access | Documentation |
| --- | --- | --- | --- |
| [WinGet](https://github.com/microsoft/winget-cli/releases/latest) | Repeatable application setup and individually reviewed updates. | [Windows](https://www.microsoft.com/windows); Free / open source | [Open guide](https://learn.microsoft.com/en-us/windows/package-manager/winget/) |
| [CrystalDiskInfo](https://crystalmark.info/en/download/) | First-pass HDD and SSD health inspection in [Windows](https://www.microsoft.com/windows). | [Windows](https://www.microsoft.com/windows); Free / open source | [Open guide](https://crystalmark.info/en/software/crystaldiskinfo/) |
| [HWiNFO](https://www.hwinfo.com/download/) | Investigating temperatures, clocks, power and component identity. | [Windows](https://www.microsoft.com/windows); Pro license for commercial use of 64/ARM64 editions | [Open guide](https://www.hwinfo.com/licenses/) |
| [Microsoft Sysinternals Suite](https://learn.microsoft.com/en-us/sysinternals/downloads/sysinternals-suite) | The core portable [Windows](https://www.microsoft.com/windows) troubleshooting kit. | [Windows](https://www.microsoft.com/windows); Free Microsoft utilities | [Open guide](https://learn.microsoft.com/en-us/sysinternals/downloads/) |
| [Veeam Agent for Microsoft Windows](https://www.veeam.com/products/free/microsoft-windows) | Image backup and recovery media for a supported [Windows](https://www.microsoft.com/windows) endpoint. | [Windows](https://www.microsoft.com/windows); Free and paid editions; service-provider terms differ | [Open guide](https://helpcenter.veeam.com/docs/agentforwindows/userguide/overview.html) |
| [WizTree](https://diskanalyzer.com/download) | Finding the folders and files responsible for a capacity problem before selecting an approved cleanup. | [Windows](https://www.microsoft.com/windows), [macOS](https://support.apple.com/macos), [Linux](https://www.kernel.org/); Free for personal use; commercial supporter or enterprise license required by business site and staff size. | [Open guide](https://diskanalyzer.com/about) |
| [WizFile](https://antibody-software.com/wizfile/download) | Locating a known file, log or recently changed group of files without guessing its folder. | [Windows](https://www.microsoft.com/windows); Free for personal use; commercial supporter or enterprise license required by business site and staff size. | [Open guide](https://antibody-software.com/wizfile/quick-start) |

<a id="walkthrough"></a>

<a id="-work-the-case"></a>

## Procedure

Work in order, record the result of each step, and change route when the evidence calls for it.

<a id="step-01"></a>

### 01 · Review health and recurring failures

- [ ] Check [Reliability Monitor](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/perfmon), storage health, available capacity, temperatures, and recent support history. Separate actionable errors from harmless noise. Investigate symptoms that affect the user before scheduling broad changes.

<a id="step-02"></a>

### 02 · Verify protection and access

- [ ] Confirm the intended real-time security product is healthy and current, firewall policy is intact, and approved remote-management access belongs to the right people. Review unwanted startup apps and browser extensions with the user; remove only what is understood.

<a id="step-03"></a>

### 03 · Review updates before applying them

- [ ] Check [Windows](https://www.microsoft.com/windows) and application updates, reboot requirements, vendor advisories, and line-of-business compatibility. [WinGet](https://learn.microsoft.com/en-us/windows/package-manager/winget/) with no target lists available upgrades; review that list and update the intended packages rather than blindly upgrading every application during production work.

**Command for the environment described above:**

```text
winget upgrade
```

**Command legend:** [WinGet upgrade](https://learn.microsoft.com/en-us/windows/package-manager/winget/upgrade) · [Return to the step context](#step-03).

<a id="step-04"></a>

### 04 · Check backups through a restore

- [ ] Read backup logs and capacity, then restore a small representative item to a separate folder. Confirm recovery-key access and that a coworker can follow the recovery path. If the backup is not restorable, treat that as a repair task with an owner.

<a id="step-05"></a>

### 05 · Handle storage and physical upkeep

- [ ] Use the system’s supported storage cleanup controls after reviewing what will be removed. Preserve downloads, local mail, and application data that may be unique. Clean physical dust using the device’s service guidance; avoid registry cleaners and indiscriminate driver-updater bundles. Use [WizTree](https://diskanalyzer.com/) to explain capacity use and [WizFile](https://antibody-software.com/wizfile/) to locate a specific retained file or log before cleanup. Compare the proposed deletion with the owner's retention needs and the available backup. Recheck free space and the original workload afterward; exit [WizFile](https://antibody-software.com/wizfile/) and remove any one-case startup setting so a temporary visit does not leave unplanned monitoring.

<a id="step-06"></a>

### 06 · Retest and schedule only real follow-up

- [ ] After the planned restart, test the user’s main applications, network, printers, backup, and remote access. Record versions and changes, measurable improvements, deferred work, and hardware replacement recommendations based on evidence.

<a id="acceptance"></a>

<a id="-prove-the-result"></a>

## Verification

- [ ] Security, updates, backup restore, and hardware health were reviewed.
- [ ] The normal workload works after maintenance and restart.
- [ ] Every deferred or failed item has an owner and next action.
- [ ] Storage cleanup has an explained target, retention decision and observed result; temporary search or capture utilities are closed.

> [!WARNING]
> **Pause this route and escalate when:**
> - The maintenance would interrupt an unplanned critical workload.
> - Storage failure or active compromise changes the job into recovery or [incident response](https://github.com/Herbertofury/INTA/wiki/Job-ransomware-response).

<a id="-continue-with-the-evidence"></a>

## Related playbooks

| Next playbook | What it resolves |
| --- | --- |
| [Diagnose a slow, freezing, or hitching PC](slow-pc.md) | Find the actual bottleneck and prove the improvement with the same workload. |
| [Fix a failing Windows update](windows-update.md) | Diagnose the update code, servicing state, and available space before broad reset scripts. |
| [Prove a backup can actually restore](backup-restore.md) | Validate recovery credentials, data integrity, application usability, and recovery time. |
| [Close a repair with a clear client handoff](client-handoff.md) | Turn completed technical work into proof the customer can use, with clean access and an actionable service record. |

<a id="sources"></a>

<a id="-source-trail"></a>

## Official sources

<details>
<summary>References and review notes</summary>

- [Microsoft: WinGet upgrade](https://learn.microsoft.com/en-us/windows/package-manager/winget/upgrade)
- [restic: restore a backup](https://restic.readthedocs.io/en/stable/050_restore.html)
- [Microsoft: Sysinternals Suite](https://learn.microsoft.com/en-us/sysinternals/downloads/sysinternals-suite)
- [WizTree: disk-space analysis and export](https://diskanalyzer.com/about)
- [WizFile: search filters and shutdown](https://antibody-software.com/wizfile/quick-start)

Reviewed **2026-11-26**. Timing is a planning range. A successful command is evidence about that operation; acceptance belongs to the user’s original task.

</details>

---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)

[← All playbooks](README.md) · [Live Wiki](https://github.com/Herbertofury/INTA/wiki) · [Contribution guide](https://github.com/Herbertofury/INTA/blob/main/CONTRIBUTING.md)
