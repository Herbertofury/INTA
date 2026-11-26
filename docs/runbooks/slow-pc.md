[Home](https://github.com/Herbertofury/INTA/wiki) · [All playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Interactive checklist](https://herbertofury.github.io/INTA/#job/slow-pc)

# Diagnose a slow, freezing, or hitching PC

> Find the actual bottleneck and prove the improvement with the same workload.

| Job profile | Details |
| --- | --- |
| Category | Windows & performance |
| Working time | 20–60 min |
| Experience | Intermediate |

## Start here

Reproduce the exact slow action and measure it before changing anything. Ask whether the slowdown began after an update, new drive, app, or security product.

## Your toolkit

| Tool | Why it belongs here |
| --- | --- |
| [Microsoft Sysinternals Suite](https://learn.microsoft.com/en-us/sysinternals/downloads/sysinternals-suite) | The core portable Windows troubleshooting kit. |
| [CrystalDiskInfo](https://crystalmark.info/en/download/) | First-pass HDD and SSD health inspection in Windows. |
| [HWiNFO](https://www.hwinfo.com/download/) | Investigating temperatures, clocks, power and component identity. |
| [Windows Performance Toolkit](https://learn.microsoft.com/en-us/windows-hardware/get-started/adk-install) | Investigating intermittent CPU, storage or application responsiveness problems with evidence. |

## Follow the workflow

### 01. Capture a repeatable baseline

- [ ] Time sign-in and the specific failing task separately. Note AC power, current applications, storage connections, network state, and background updates. Repeat the same short workload after each change so an apparent improvement is comparable.

### 02. Inspect pressure while the lag happens

- [ ] Use Task Manager or Resource Monitor to identify CPU, committed memory, disk response, and network pressure. A high percentage alone is not a diagnosis: correlate it with the lag and the responsible process, path, or service.

```text
resmon
```

### 03. Check storage and hardware first

- [ ] Review drive health and temperatures. If a disk disappears, clicks, reports I/O errors, or stalls the system when accessed, stop repair scans and use the failing-drive guide. Check a suspect external drive separately after outstanding writes have finished; preserve the user’s files.

### 04. Trace the responsible application

- [ ] Use Process Explorer for process detail and a short, filtered Process Monitor capture for file or registry stalls. Capture only the affected interval, stop recording, and save the trace privately. Inspect missing paths, repeated failures, or excessive work before choosing a fix.

### 05. Apply one reversible correction

- [ ] Fix the observed cause: a failing device, incompatible driver, misconfigured backup, unwanted startup app, or software regression. Use vendor-supported settings. Do not apply blanket registry cleaners, remove needed services, or disable endpoint protection simply to improve a benchmark.

### 06. Measure and keep the evidence

- [ ] Repeat the baseline workload with the same data and conditions. Check functionality, responsiveness, and resource use together. Restore changes that did not help and record the measured before/after result plus anything still reproducible.

## Verify before handoff

- [ ] The original task is measurably faster or the root cause is clearly identified.
- [ ] Required applications, protection, backup, and peripherals still work.
- [ ] The improvement survives a restart and the normal user workload.

## Stop and escalate when

- Storage-health symptoms appear during testing.
- A proposed optimization removes required functionality or only hides the symptom.

## Related playbooks

- [Failing Drive](failing-drive.md)
- [Hardware Bsod](hardware-bsod.md)
- [Windows Repair](windows-repair.md)
- [Preventive Maintenance](preventive-maintenance.md)

## Official references

- [Microsoft Sysinternals: Process Explorer](https://learn.microsoft.com/en-us/sysinternals/downloads/process-explorer/)
- [Microsoft Sysinternals: Process Monitor](https://learn.microsoft.com/en-us/sysinternals/downloads/procmon)

---

INTA Computer Services · Editorial review: 2026-11-26. Timing is an estimate; stop conditions take priority.
