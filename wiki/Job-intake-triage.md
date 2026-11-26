[Home](https://github.com/Herbertofury/INTA/wiki) · [All playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Interactive checklist](https://herbertofury.github.io/INTA/#job/intake-triage)

# Start here: triage any computer problem

> Turn a vague symptom into a clear next action, a protected data set, and a useful service record.

| Job profile | Details |
| --- | --- |
| Category | Getting started |
| Working time | 10–20 min |
| Experience | Starter |

## Start here

Ask the user to show the exact failing task once. Record what they expected, what happened, and when it last worked.

## Your toolkit

| Tool | Why it belongs here |
| --- | --- |
| [Microsoft Sysinternals Suite](https://learn.microsoft.com/en-us/sysinternals/downloads/sysinternals-suite) | The core portable Windows troubleshooting kit. |
| [CrystalDiskInfo](https://crystalmark.info/en/download/) | First-pass HDD and SSD health inspection in Windows. |
| [HWiNFO](https://www.hwinfo.com/download/) | Investigating temperatures, clocks, power and component identity. |

## Follow the workflow

### 01. Establish the working boundary

- [ ] Identify the device, operating system, asset label, contact, business impact, and maintenance window. Ask whether work is saved and whether the device contains the only copy of important data. Use the private ticket for identifying information; this public wiki stays generic.

### 02. Choose the urgent branch

- [ ] Encryption spreading or a compromise alert goes to ransomware response. Clicking drives, vanishing volumes, repeated I/O errors, or a device dropped in liquid go to the failing-drive or hardware path before scans, repairs, or repeated boots. A widespread outage goes to the outage guide.

### 03. Capture the smallest useful baseline

- [ ] Record the exact error text and time, OS build, recent changes, uptime, free space, and Task Manager CPU/memory/disk usage during the symptom. On Windows, open Reliability Monitor from Run with the command below to correlate failures with changes; save evidence to the private service record.

```text
perfmon /rel
```

### 04. Reproduce and narrow the scope

- [ ] Test whether the failure affects one user, one app, one device, or everyone. Compare with a working account or device where practical. Change one variable at a time and note whether it changes the symptom.

### 05. Protect a rollback point

- [ ] Confirm a usable backup or export of the data that the next step may affect. Record the current configuration before changing it. A restore point is useful for some system changes but is not a replacement for a file backup.

### 06. Pick one guide and define success

- [ ] Choose the closest job below and write a concrete acceptance test, such as opening the real accounting file, completing a print job, or restoring a document. Capture each change and result so another technician can resume without repeating failed work.

## Verify before handoff

- [ ] The original symptom and business impact are recorded.
- [ ] The next guide matches observed evidence.
- [ ] A data-protection plan and a measurable acceptance test exist.

## Stop and escalate when

- There is active encryption, physical damage, or storage failure.
- The only copy of important data would be put at risk by the next action.

## Related playbooks

- [Malware Removal](https://github.com/Herbertofury/INTA/wiki/Job-malware-removal)
- [Slow Pc](https://github.com/Herbertofury/INTA/wiki/Job-slow-pc)
- [Failing Drive](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive)
- [Outage Recovery](https://github.com/Herbertofury/INTA/wiki/Job-outage-recovery)
- [Client Handoff](https://github.com/Herbertofury/INTA/wiki/Job-client-handoff)

## Official references

- [Microsoft: recovery options in Windows](https://support.microsoft.com/en-us/windows/experience/backup-recovery/recovery-options-in-windows)
- [Microsoft Sysinternals: Process Explorer](https://learn.microsoft.com/en-us/sysinternals/downloads/process-explorer/)

---

INTA Computer Services · Editorial review: 2026-11-26. Timing is an estimate; stop conditions take priority.
