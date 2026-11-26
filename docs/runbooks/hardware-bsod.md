[Home](https://github.com/Herbertofury/INTA/wiki) · [All playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Interactive checklist](https://herbertofury.github.io/INTA/#job/hardware-bsod)

# Diagnose crashes, blue screens, and overheating

> Correlate crash evidence with memory, storage, temperature, power, and driver changes.

| Job profile | Details |
| --- | --- |
| Category | Hardware |
| Working time | 30–120 min plus extended tests |
| Experience | Advanced |

## Start here

Record the exact stop code, timestamp, workload, recent hardware/driver changes, and whether the machine restarted or lost power.

## Your toolkit

| Tool | Why it belongs here |
| --- | --- |
| [HWiNFO](https://www.hwinfo.com/download/) | Investigating temperatures, clocks, power and component identity. |
| [Memtest86+](https://www.memtest.org/#download) | Investigating memory instability and repeatable hardware errors. |
| [CrystalDiskInfo](https://crystalmark.info/en/download/) | First-pass HDD and SSD health inspection in Windows. |
| [Microsoft Sysinternals Suite](https://learn.microsoft.com/en-us/sysinternals/downloads/sysinternals-suite) | The core portable Windows troubleshooting kit. |
| [WinDbg](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/) | Interpreting a crash dump when basic event logs do not establish the cause. |

## Follow the workflow

### 01. Capture evidence before repeated testing

- [ ] Save the stop code, Reliability Monitor history, relevant events, and available crash dump in the private case. Note the dump’s sensitivity because it may contain user data. A named module is a clue that still needs context.

### 02. Check basic stability

- [ ] Inspect cooling, dust, fans, physical connections, and any changed overclock or memory profile. Review temperatures under the normal failing workload. If a recent tuning change correlates with the issue, return it to the documented supported baseline before stress testing.

### 03. Rule out storage problems

- [ ] Inspect storage health and I/O errors before filesystem repairs or long benchmarks. A disappearing or unhealthy drive moves to data recovery. Keep the user’s data safe before intentionally reproducing a crash.

### 04. Test one hypothesis at a time

- [ ] Use bootable memory diagnostics for suspected RAM and the hardware vendor’s diagnostics for the relevant component. Stop a stress test if temperatures or behavior become abnormal. Record test configuration and duration; one clean test does not prove every intermittent fault absent.

### 05. Investigate the driver or power path

- [ ] Compare the failure with the installation timeline and use an OEM-supported driver update or rollback when evidence supports it. Review dumps with an appropriate debugger for persistent crashes. Event ID 41 records an unclean shutdown and must not be treated as a complete power-supply diagnosis.

### 06. Recreate the original workload

- [ ] After the targeted fix, repeat the real application or idle/sleep/resume scenario that failed. Verify restart, sleep, peripherals, and normal temperatures. Record the cause confidence, replaced component or driver, and remaining monitoring instructions.

## Verify before handoff

- [ ] The original crash scenario no longer reproduces during the defined test.
- [ ] Relevant diagnostics and configuration are recorded.
- [ ] Normal temperatures, storage health, and required devices are confirmed.

## Stop and escalate when

- Cooling failure, electrical damage, or storage deterioration appears.
- A proposed test risks unbacked-up data or production availability.

## Related playbooks

- [Slow Pc](slow-pc.md)
- [Failing Drive](failing-drive.md)
- [Boot Recovery](boot-recovery.md)
- [Backup Restore](backup-restore.md)

## Official references

- [Microsoft: stop code and bug check troubleshooting](https://learn.microsoft.com/en-us/windows/client-management/troubleshoot-stop-errors)
- [Microsoft: interpreting Kernel-Power Event ID 41](https://learn.microsoft.com/en-us/troubleshoot/windows-client/performance/event-id-41-restart)

---

INTA Computer Services · Editorial review: 2026-11-26. Timing is an estimate; stop conditions take priority.
