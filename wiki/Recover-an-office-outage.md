[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

# 🏢 Recover an office outage

**Confirm safe power and the affected business services, then assign one recovery owner.**

<a id="working-kit"></a>

## 🧰 Tools and links for this job

**[Open working view](https://herbertofury.github.io/INTA/#job/outage-recovery/quick/recover-an-outage)** · Keep the tool panel beside the instructions.

**First move:** Confirm safe power and the affected business services, then assign one recovery owner.

| Use this for | Official tool + exact action |
| --- | --- |
| Inspect recovered [Windows](https://www.microsoft.com/windows) services and connection state. | **[PowerShell](https://learn.microsoft.com/en-us/powershell/)** · [Official page](https://learn.microsoft.com/en-us/powershell/) · **[Use Windows PowerShell 5.1](https://learn.microsoft.com/en-us/powershell/scripting/windows-powershell/starting-windows-powershell)** |
| Check for a matching cloud remedy on eligible, configured devices with a widespread startup failure. | **[Quick machine recovery](https://learn.microsoft.com/en-us/windows/configuration/quick-machine-recovery/?tabs=csp)** · [Official page](https://learn.microsoft.com/en-us/windows/configuration/quick-machine-recovery/?tabs=csp) · **[Check supported recovery setup](https://learn.microsoft.com/en-us/windows/configuration/quick-machine-recovery/?tabs=csp)** |

### Find the failure

| Open the exact resource | What to do there |
| --- | --- |
| **[Interpret unexpected-restart evidence](https://learn.microsoft.com/en-us/troubleshoot/windows-client/performance/event-id-41-restart)** | Event 41 alone does not identify the shutdown cause. |

### Shared endpoint failures

| Open the exact resource | What to do there |
| --- | --- |
| **[Quick machine recovery — conditional](https://learn.microsoft.com/en-us/windows/configuration/quick-machine-recovery/?tabs=csp)** | [Windows 11](https://www.microsoft.com/windows/windows-11) 24H2 build 26100.4700 or later; verify policy, network access and a matching remedy. Enterprise defaults can be off. |

### Restore service

| Open the exact resource | What to do there |
| --- | --- |
| **[Verify a backup restore](https://github.com/Herbertofury/INTA/wiki/Job-backup-restore)** | Prove the recovery point and actual restored application. |

### Case record

| Open the exact resource | What to do there |
| --- | --- |
| **[Record owners and outstanding faults](https://github.com/Herbertofury/INTA/wiki/Technician-Handoff)** | Keep the service timeline and next actions in the private record. |

## ⚡ First three actions

1. Check power, UPS status and network reachability without repeatedly cycling unstable equipment.
2. Restore dependencies in the guide’s order: network, storage, identity and the affected application services.
3. Test a real user workflow and backup status, then record the outage timeline and remaining faults.

**No extra download needed.**

**[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-outage-recovery)** · **[Selection evidence](https://github.com/Herbertofury/INTA/wiki/Free-Tool-Picks)**

<a id="next-action"></a>

## 🧭 Match the symptom

| You see this | Do this next | You should get |
| --- | --- | --- |
| **Power or hardware is visibly unsafe** | Stop repeated power cycling and involve the qualified power or hardware owner. | The equipment can be energized safely or remains isolated for repair. |
| **Network is back but storage or hosts are not ready** | Inspect the relevant console and storage status before starting dependent workloads; do not initialize or format missing volumes. | The required storage and host resources are stable before application recovery. |
| **Identity or DNS is unavailable** | Restore the approved identity/network dependencies in the site runbook, then test name resolution and sign-in from a client. | The user can authenticate and reach the required service names. |
| **Services show running but users still cannot work** | Test the actual application and its data/share/database dependency from a normal user session. | A real transaction or equivalent business task succeeds. |
| **Many supported [Windows 11](https://www.microsoft.com/windows/windows-11) endpoints fail at startup** | Check the common change and whether [Quick machine recovery](https://learn.microsoft.com/en-us/windows/configuration/quick-machine-recovery/?tabs=csp) is enabled and applicable; use the normal recovery plan if no remedy is available. | A verified remedy or a controlled rollback is chosen without assuming cloud recovery will succeed. |

### ✅ Finish the job

- [ ] Critical user workflows succeed from the affected network.
- [ ] Backup operation and remaining infrastructure alarms have been checked.
- [ ] The timeline, cause evidence and follow-up owner are recorded.

**Change route when:** Storage/RAID damage, unsafe power or active compromise changes the recovery route. · The required restore would overwrite the only recovery copy.


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
