[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

<a id="-bring-an-office-back-after-a-power-outage"></a>

# [Bring an office back after a power outage](https://github.com/Herbertofury/INTA/wiki/Job-outage-recovery)

Restore dependencies in order and verify services before releasing the office back to work.

**Backup & continuity** · Advanced · 30–120 min; infrastructure varies · 6 steps

[Tools and useful links](#working-kit) · [Quick answer](#quick-answer) · [Next action](#next-action) · [Decision route](#decision-route) · [Tools](#toolkit) · [Procedure](#walkthrough) · [Verification](#acceptance) · [Sources](#sources) · [Interactive checklist](https://herbertofury.github.io/INTA/#job/outage-recovery)

<a id="working-kit"></a>

## 🧰 Tools and links for this job

**[Open working view](https://herbertofury.github.io/INTA/#job/outage-recovery)** · Keep the tool panel beside the instructions.

**First move:** Confirm safe power and identify the affected service. Assign one recovery owner before restarting shared systems.

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

<a id="quick-answer"></a>

## ⚡ Quick answer

<a id="quick-recover-an-outage"></a>

### 🏢 Recover an office outage

**Confirm safe power and the affected business services, then assign one recovery owner.**

1. Check power, UPS status and network reachability without repeatedly cycling unstable equipment.
2. Restore dependencies in the guide’s order: network, storage, identity and the affected application services.
3. Test a real user workflow and backup status, then record the outage timeline and remaining faults.

**No extra download needed.** Follow the checklist below.

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

> [!NOTE]
> **Start here.** Confirm power is stable and establish which devices and services are down. Use the site’s private dependency diagram and recovery plan.

<a id="decision-route"></a>

<a id="-choose-the-right-branch"></a>

## 🧭 Choose a route

```mermaid
flowchart TD
    A["Confirm stable power and scope"]:::action --> B{"Damage or unstable power?"}:::decision
    B -->|"Yes"| C["Keep affected equipment out of service"]:::stop
    B -->|"No"| D["Restore network dependencies"]:::action
    D --> E{"Storage and core services healthy?"}:::decision
    E -->|"No"| F["Follow the approved recovery plan"]:::stop
    E -->|"Yes"| G["Test business services from one client"]:::action
    G --> H{"End-to-end tests pass?"}:::decision
    H -->|"No"| I["Diagnose the first failing dependency"]:::action
    I --> E
    H -->|"Yes"| J["Resume users and confirm backups"]:::done
    classDef decision fill:#eaf0f8,stroke:#607fa7,color:#172b46
    classDef action fill:#f1f3f6,stroke:#8792a2,color:#263445
    classDef stop fill:#faf0f0,stroke:#a46b70,color:#5e3137
    classDef done fill:#e9edf3,stroke:#6c7d94,color:#253449
```

**Route links:** [Continue this workflow](#walkthrough) · [Acceptance checks](#acceptance) · [Fix internet, DNS, or local-network problems](https://github.com/Herbertofury/INTA/wiki/Job-network-dns) · [Diagnose crashes, blue screens, and overheating](https://github.com/Herbertofury/INTA/wiki/Job-hardware-bsod) · [Prove a backup can actually restore](https://github.com/Herbertofury/INTA/wiki/Job-backup-restore) · [Close a repair with a clear client handoff](https://github.com/Herbertofury/INTA/wiki/Job-client-handoff)

| Checkpoint | What to carry forward |
| --- | --- |
| **Before changing anything** | Confirm stable power and use the private dependency plan. |
| **Keep during the job** | Alarms, startup order, outage timeline, and client-side service tests. |
| **Before handing back** | Business services, backup jobs, and remote management are healthy. |

<a id="toolkit"></a>

<a id="-build-the-right-toolkit"></a>

## Recommended tools

Select a tool for the next diagnostic or repair step. Check its platform, access and licensing requirements before use.

| Tool / official page + download | Use it for | Platform & access | Documentation |
| --- | --- | --- | --- |
| **[PowerShell](https://learn.microsoft.com/en-us/powershell/)** · [Official page](https://learn.microsoft.com/en-us/powershell/) · **[Use Windows PowerShell 5.1](https://learn.microsoft.com/en-us/powershell/scripting/windows-powershell/starting-windows-powershell)** | Inspect recovered [Windows](https://www.microsoft.com/windows) services and connection state. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos); Free / open source | [Open guide](https://learn.microsoft.com/en-us/powershell/) |
| **[Quick machine recovery](https://learn.microsoft.com/en-us/windows/configuration/quick-machine-recovery/?tabs=csp)** · [Official page](https://learn.microsoft.com/en-us/windows/configuration/quick-machine-recovery/?tabs=csp) · **[Check supported recovery setup](https://learn.microsoft.com/en-us/windows/configuration/quick-machine-recovery/?tabs=csp)** | Check for a matching cloud remedy on eligible, configured devices with a widespread startup failure. | [Windows 11](https://www.microsoft.com/windows/windows-11), [Windows Server](https://learn.microsoft.com/en-us/windows-server/) 2025; Included with supported [Windows](https://www.microsoft.com/windows); availability depends on release, edition and management policy | [Open guide](https://learn.microsoft.com/en-us/windows/configuration/quick-machine-recovery/?tabs=csp) |
| **[smartmontools](https://www.smartmontools.org/)** · [Official page](https://www.smartmontools.org/) · **[Download Windows setup · EXE](https://github.com/smartmontools/smartmontools/releases/download/RELEASE_7_5/smartmontools-7.5.win32-setup.exe)** | Reading SMART and NVMe health from a rescue environment or supported host OS. | [Linux](https://www.kernel.org/), [Windows](https://www.microsoft.com/windows), [macOS](https://support.apple.com/macos); Free / open source | [Open guide](https://github.com/smartmontools/smartmontools) |
| **[Microsoft Sysinternals Suite](https://learn.microsoft.com/en-us/sysinternals/)** · [Official page](https://learn.microsoft.com/en-us/sysinternals/) · **[Download Windows portable · ZIP](https://download.sysinternals.com/files/SysinternalsSuite.zip)** | The core portable [Windows](https://www.microsoft.com/windows) troubleshooting kit. | [Windows](https://www.microsoft.com/windows); Free Microsoft utilities | [Open guide](https://learn.microsoft.com/en-us/sysinternals/downloads/) |

<a id="walkthrough"></a>

<a id="-work-the-case"></a>

## Procedure

Work in order, record the result of each step, and change route when the evidence calls for it.

<a id="step-01"></a>

### 01 · Check the physical environment

- [ ] Inspect UPS status, breakers, power strips, equipment alarms, and visible damage. Keep equipment with electrical damage, liquid exposure, or repeated power cycling out of service. Record the approximate outage and shutdown times from available logs.

<a id="step-02"></a>

### 02 · Restore the network foundation

- [ ] Bring up the internet handoff, firewall/router, switches, and wireless infrastructure in the site’s documented dependency order. Wait for each to reach a healthy state, then verify local link, addressing, routing, and name resolution before starting application tests.

<a id="step-03"></a>

### 03 · Recover storage and core services

- [ ] Check storage controllers and arrays for degraded state before normal workload resumes. Bring up required hypervisors, storage, identity/DNS/DHCP, and dependent services in their actual site order. Avoid repeated hard resets of a server performing recovery or consistency work.

<a id="step-04"></a>

### 04 · Validate applications from one workstation

- [ ] Use one known-good client to test domain sign-in, shared folders, printing, email, and each critical line-of-business application. An open server console or a successful ping does not prove the business service is healthy.

<a id="step-05"></a>

### 05 · Review shutdown and recovery evidence

- [ ] Inspect UPS logs and relevant event logs. Kernel-Power Event ID 41 indicates [Windows](https://www.microsoft.com/windows) did not shut down cleanly; it does not by itself identify the power supply or the original cause. Correlate the event with other evidence before replacing parts.

<a id="step-06"></a>

### 06 · Resume operations and verify backups

- [ ] Release workstations in a controlled order, confirm time synchronization and remote management, and check that backup jobs resume. Record any disk repairs, degraded components, interrupted transactions, and remaining follow-up with the designated service owner.

<a id="acceptance"></a>

<a id="-prove-the-result"></a>

## Verification

- [ ] Network, identity, storage, and business applications pass end-to-end tests.
- [ ] No degraded storage or critical recovery alarm is left unassigned.
- [ ] Backups and remote access have resumed.
- [ ] Users can complete their actual work from a client workstation.

> [!WARNING]
> **Pause this route and escalate when:**
> - Power is unstable or equipment shows physical/electrical damage.
> - Storage is degraded, rebuilding, or producing new errors without an approved recovery plan.

<a id="-continue-with-the-evidence"></a>

## Related playbooks

| Next playbook | What it resolves |
| --- | --- |
| [Fix internet, DNS, or local-network problems](https://github.com/Herbertofury/INTA/wiki/Job-network-dns) | Locate the failing layer before resetting adapters or changing the whole network. |
| [Diagnose crashes, blue screens, and overheating](https://github.com/Herbertofury/INTA/wiki/Job-hardware-bsod) | Correlate crash evidence with memory, storage, temperature, power, and driver changes. |
| [Prove a backup can actually restore](https://github.com/Herbertofury/INTA/wiki/Job-backup-restore) | Validate recovery credentials, data integrity, application usability, and recovery time. |
| [Close a repair with a clear client handoff](https://github.com/Herbertofury/INTA/wiki/Job-client-handoff) | Turn completed technical work into proof the customer can use, with clean access and an actionable service record. |

<a id="sources"></a>

<a id="-source-trail"></a>

## Official sources

- [Microsoft: interpreting Kernel-Power Event ID 41](https://learn.microsoft.com/en-us/troubleshoot/windows-client/performance/event-id-41-restart)
- [restic: repository consistency and integrity](https://restic.readthedocs.io/en/stable/045_working_with_repos.html)

Reviewed **2026-11-26**. Timing is a planning range. A successful command is evidence about that operation; acceptance belongs to the user’s original task.


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
