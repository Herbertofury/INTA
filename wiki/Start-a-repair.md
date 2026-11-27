[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

# 🧭 Start a repair

**Record the exact symptom, affected device and desired result before making changes.**

<a id="working-kit"></a>

## 🧰 Tools and links for this job

**[Open working view](https://herbertofury.github.io/INTA/#job/intake-triage/quick/start-a-repair)** · Keep the tool panel beside the instructions.

**First move:** Record the exact symptom, affected device and desired result before making changes.

| Use this for | Official tool + exact action |
| --- | --- |
| Identify the process involved in the failure. | **[Process Explorer](https://learn.microsoft.com/en-us/sysinternals/downloads/process-explorer)** · [Official page](https://learn.microsoft.com/en-us/sysinternals/downloads/process-explorer) · **[Download Windows portable · ZIP](https://download.sysinternals.com/files/ProcessExplorer.zip)** |
| Check drive warnings before repair writes. | **[CrystalDiskInfo](https://crystalmark.info/en/software/crystaldiskinfo/)** · [Official page](https://crystalmark.info/en/software/crystaldiskinfo/) · **[Get Windows ZIP · 9.9.5](https://sourceforge.net/projects/crystaldiskinfo/files/9.9.5/CrystalDiskInfo9_9_5.zip/download)** |

### Case record

| Open the exact resource | What to do there |
| --- | --- |
| **[Intake and handoff sheet](https://github.com/Herbertofury/INTA/wiki/Technician-Handoff)** | Capture the symptom, permission, recovery plan and acceptance test. |

### First observations

| Open the exact resource | What to do there |
| --- | --- |
| **[Reliability history command](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/perfmon)** | Open failure history with perfmon /rel. |

### Protect data

| Open the exact resource | What to do there |
| --- | --- |
| **[Windows recovery choices](https://support.microsoft.com/en-us/windows/experience/backup-recovery/recovery-options-in-windows)** | Check which recovery action preserves files and applications. |

## ⚡ First three actions

1. Confirm permission, the last working time and any recent change with the user.
2. Check for spreading compromise, failing storage and missing recovery access before normal troubleshooting.
3. Reproduce the problem, record a baseline and choose the matching guide with a clear success test.

**No extra download needed.**

**[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-intake-triage)** · **[Selection evidence](https://github.com/Herbertofury/INTA/wiki/Free-Tool-Picks)**

<a id="next-action"></a>

## 🧭 Match the symptom

| You see this | Do this next | You should get |
| --- | --- | --- |
| **The error appears in one app or account** | Repeat the same task in another approved account or app; record what changes. | The fault is scoped to an app, profile or wider device problem. |
| **Several users or devices fail together** | Compare one known-working device and the same service; use the office-outage route when the dependency is shared. | The ticket identifies the shared service and its recovery owner. |
| **The timing or cause is unclear** | Press Win+R, run perfmon /rel, select the failure time and open technical details. | A dated failure or recent change can be compared with the user’s timeline. |
| **A drive clicks, disappears or reports repeated I/O errors** | Stop ordinary repair and route to failing-drive recovery before scans or file-system repairs. | The source is preserved and the imaging or specialist decision is recorded. |

### ✅ Finish the job

- [ ] One exact reproduction and expected outcome are recorded.
- [ ] The next guide, responsible technician and recoverable data copy are identified.

**Change route when:** Active encryption or spreading compromise requires the incident route. · The next action would risk the only usable data copy.


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
