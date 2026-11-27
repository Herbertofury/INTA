[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Service map](https://github.com/Herbertofury/INTA/wiki/Service-Map) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

<a id="-start-here-triage-any-computer-problem"></a>

# [Start here: triage any computer problem](https://github.com/Herbertofury/INTA/wiki/Job-intake-triage)

Turn a vague symptom into a clear next action, a protected data set, and a useful service record.

**Getting started** · Starter · 10–20 min · 6 steps

[Tools and useful links](#working-kit) · [Quick answer](#quick-answer) · [Next action](#next-action) · [Specific fixes](#specific-fixes) · [Decision route](#decision-route) · [Tools](#toolkit) · [Procedure](#walkthrough) · [Verification](#acceptance) · [Sources](#sources) · [Interactive checklist](https://herbertofury.github.io/INTA/#job/intake-triage)

<a id="working-kit"></a>

## 🧰 Tools and links for this job

**[Open working view](https://herbertofury.github.io/INTA/#job/intake-triage)** · Keep the tool panel beside the instructions.

**First move:** Ask the user to show the failing task once. Record the exact error, last working time, affected users and whether important data has another copy.

| Use this for | Official tool + exact action |
| --- | --- |
| Identify the process involved in the failure. | **[Process Explorer](https://learn.microsoft.com/en-us/sysinternals/downloads/process-explorer)** · [Official page](https://learn.microsoft.com/en-us/sysinternals/downloads/process-explorer) · **[Download Windows portable · ZIP](https://download.sysinternals.com/files/ProcessExplorer.zip)** |
| Check drive warnings before repair writes. | **[CrystalDiskInfo](https://crystalmark.info/en/software/crystaldiskinfo/)** · [Official page](https://crystalmark.info/en/software/crystaldiskinfo/) · **[Get Windows ZIP · 9.9.5](https://sourceforge.net/projects/crystaldiskinfo/files/9.9.5/CrystalDiskInfo9_9_5.zip/download)** |
| Identifying the actual USB port, hub, device and negotiated connection when a printer, scanner or peripheral disconnects or runs slowly. | **[USB Device Tree Viewer](https://www.uwe-sieber.de/usbtreeview_e.html)** · [Official page](https://www.uwe-sieber.de/usbtreeview_e.html) · **[Download Windows x64 portable · ZIP](https://www.uwe-sieber.de/files/UsbTreeView_x64.zip)** |
| Collecting battery and sleep evidence before changing hardware or power policy. | **[powercfg](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/powercfg-command-line-options)** · [Official page](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/powercfg-command-line-options) · **[Open built-in battery and sleep diagnostics](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/powercfg-command-line-options)** |

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

<a id="quick-answer"></a>

## ⚡ Quick answer

<a id="quick-start-a-repair"></a>

### 🧭 Start a repair

**Record the exact symptom, affected device and desired result before making changes.**

1. Confirm permission, the last working time and any recent change with the user.
2. Check for spreading compromise, failing storage and missing recovery access before normal troubleshooting.
3. Reproduce the problem, record a baseline and choose the matching guide with a clear success test.

**No extra download needed.** Follow the checklist below.

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

### 🎯 Go straight to a specific fix

- **[Choose the correct model and architecture](#fix-triage-confirm-model-and-architecture)** — A driver or repair package says it is incompatible, or the device model is uncertain.
- **[Find the background program blocking an app](#fix-triage-isolate-background-conflict)** — An application fails in normal startup but works when optional background software is absent.
- **[Correct a bad clock before chasing sign-in errors](#fix-triage-clock-breaks-signin)** — Several secure sites or sign-ins fail and the displayed date or time is wrong.

> [!NOTE]
> **Start here.** Ask the user to show the exact failing task once. Record what they expected, what happened, and when it last worked.

<a id="decision-route"></a>

<a id="-choose-the-right-branch"></a>

## 🧭 Choose a route

```mermaid
flowchart TD
    A["Observe the failing task"]:::action --> B{"Immediate risk?"}:::decision
    B -->|"Active compromise"| C["Contain the incident"]:::stop
    B -->|"Storage or physical damage"| D["Protect the source"]:::stop
    B -->|"No"| E{"Who is affected?"}:::decision
    E -->|"Many devices"| F["Check shared dependencies"]:::action
    E -->|"One user or device"| G["Capture a local baseline"]:::action
    F --> H["Choose a guide and acceptance test"]:::done
    G --> H
    classDef decision fill:#eaf0f8,stroke:#607fa7,color:#172b46
    classDef action fill:#f1f3f6,stroke:#8792a2,color:#263445
    classDef stop fill:#faf0f0,stroke:#a46b70,color:#5e3137
    classDef done fill:#e9edf3,stroke:#6c7d94,color:#253449
```

**Route links:** [Continue this workflow](#walkthrough) · [Acceptance checks](#acceptance) · [Remove malware and browser hijackers](https://github.com/Herbertofury/INTA/wiki/Job-malware-removal) · [Diagnose a slow, freezing, or hitching PC](https://github.com/Herbertofury/INTA/wiki/Job-slow-pc) · [Rescue data from a failing or unreadable drive](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive) · [Bring an office back after a power outage](https://github.com/Herbertofury/INTA/wiki/Job-outage-recovery) · [Close a repair with a clear client handoff](https://github.com/Herbertofury/INTA/wiki/Job-client-handoff)

| Checkpoint | What to carry forward |
| --- | --- |
| **Before changing anything** | Identify unique data and the working boundary. |
| **Keep during the job** | Exact error, time, scope, baseline, and failed tests. |
| **Before handing back** | A specific next guide and measurable acceptance test. |

<a id="toolkit"></a>

<a id="-build-the-right-toolkit"></a>

## Recommended tools

Select a tool for the next diagnostic or repair step. Check its platform, access and licensing requirements before use.

| Tool / official page + download | Use it for | Platform & access | Documentation |
| --- | --- | --- | --- |
| **[Process Explorer](https://learn.microsoft.com/en-us/sysinternals/downloads/process-explorer)** · [Official page](https://learn.microsoft.com/en-us/sysinternals/downloads/process-explorer) · **[Download Windows portable · ZIP](https://download.sysinternals.com/files/ProcessExplorer.zip)** | Identify the process involved in the failure. | [Windows](https://www.microsoft.com/windows); Free Microsoft utility | [Open guide](https://learn.microsoft.com/en-us/sysinternals/downloads/process-explorer) |
| **[CrystalDiskInfo](https://crystalmark.info/en/software/crystaldiskinfo/)** · [Official page](https://crystalmark.info/en/software/crystaldiskinfo/) · **[Get Windows ZIP · 9.9.5](https://sourceforge.net/projects/crystaldiskinfo/files/9.9.5/CrystalDiskInfo9_9_5.zip/download)** | Check drive warnings before repair writes. | [Windows](https://www.microsoft.com/windows); Free / open source | [Open guide](https://crystalmark.info/en/software/crystaldiskinfo/) |
| **[Microsoft Sysinternals Suite](https://learn.microsoft.com/en-us/sysinternals/)** · [Official page](https://learn.microsoft.com/en-us/sysinternals/) · **[Download Windows portable · ZIP](https://download.sysinternals.com/files/SysinternalsSuite.zip)** | The core portable [Windows](https://www.microsoft.com/windows) troubleshooting kit. | [Windows](https://www.microsoft.com/windows); Free Microsoft utilities | [Open guide](https://learn.microsoft.com/en-us/sysinternals/downloads/) |
| **[HWiNFO](https://www.hwinfo.com/)** · [Official page](https://www.hwinfo.com/) · **[Download Windows x64 EXE](https://www.hwinfo.com/files/hwi64_860.exe)** | Investigating temperatures, clocks, power and component identity. | [Windows](https://www.microsoft.com/windows); Pro license for commercial use of 64/ARM64 editions | [Open guide](https://www.hwinfo.com/licenses/) |
| **[USB Device Tree Viewer](https://www.uwe-sieber.de/usbtreeview_e.html)** · [Official page](https://www.uwe-sieber.de/usbtreeview_e.html) · **[Download Windows x64 portable · ZIP](https://www.uwe-sieber.de/files/UsbTreeView_x64.zip)** | Identifying the actual USB port, hub, device and negotiated connection when a printer, scanner or peripheral disconnects or runs slowly. | [Windows](https://www.microsoft.com/windows); Freeware; publisher explicitly permits use in commercial environments | [Open guide](https://www.uwe-sieber.de/usbtreeview_e.html) |
| **[powercfg](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/powercfg-command-line-options)** · [Official page](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/powercfg-command-line-options) · **[Open built-in battery and sleep diagnostics](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/powercfg-command-line-options)** | Collecting battery and sleep evidence before changing hardware or power policy. | [Windows](https://www.microsoft.com/windows); Included with [Windows](https://www.microsoft.com/windows) | [Open guide](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/powercfg-command-line-options) |

<a id="walkthrough"></a>

<a id="-work-the-case"></a>

## Procedure

Work in order, record the result of each step, and change route when the evidence calls for it.

<a id="step-01"></a>

### 01 · Establish the working boundary

- [ ] Identify the device, operating system, asset label, contact, business impact, and maintenance window. Ask whether work is saved and whether the device contains the only copy of important data. Use the private ticket for identifying information; this public wiki stays generic.

<a id="step-02"></a>

### 02 · Choose the urgent branch

- [ ] Encryption spreading or a compromise alert goes to [ransomware response](https://github.com/Herbertofury/INTA/wiki/Job-ransomware-response). Clicking drives, vanishing volumes, repeated I/O errors, or a device dropped in liquid go to the failing-drive or hardware path before scans, repairs, or repeated boots. A widespread outage goes to the outage guide.

<a id="step-03"></a>

### 03 · Capture the smallest useful baseline

- [ ] Record the exact error text and time, OS build, recent changes, uptime, free space, and [Task Manager](https://support.microsoft.com/en-us/windows/experience/system-configuration-tools-in-windows) CPU/memory/disk usage during the symptom. On [Windows](https://www.microsoft.com/windows), open [Reliability Monitor](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/perfmon) from Run with the command below to correlate failures with changes; save evidence to the private service record.

**Command for the environment described above:**

```text
perfmon /rel
```

**Command legend:** [Reliability Monitor](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/perfmon) · [Return to the step context](#step-03).

<a id="step-04"></a>

### 04 · Reproduce and narrow the scope

- [ ] Test whether the failure affects one user, one app, one device, or everyone. Compare with a working account or device where practical. Change one variable at a time and note whether it changes the symptom.

<a id="step-05"></a>

### 05 · Protect a rollback point

- [ ] Confirm a usable backup or export of the data that the next step may affect. Record the current configuration before changing it. A restore point is useful for some system changes but is not a replacement for a file backup.

<a id="step-06"></a>

### 06 · Pick one guide and define success

- [ ] Choose the closest job below and write a concrete acceptance test, such as opening the real accounting file, completing a print job, or restoring a document. Capture each change and result so another technician can resume without repeating failed work.

<a id="acceptance"></a>

<a id="-prove-the-result"></a>

## Verification

- [ ] The original symptom and business impact are recorded.
- [ ] The next guide matches observed evidence.
- [ ] A data-protection plan and a measurable acceptance test exist.

> [!WARNING]
> **Pause this route and escalate when:**
> - There is active encryption, physical damage, or storage failure.
> - The only copy of important data would be put at risk by the next action.

<a id="specific-fixes"></a>

## 🎯 Specific fixes

Choose the case that matches the evidence. Each case gives a first action, a short procedure and a result to prove.

<a id="fix-triage-confirm-model-and-architecture"></a>

### Choose the correct model and architecture

**Use when:** A driver or repair package says it is incompatible, or the device model is uncertain.

**Start here:** Open Settings > System > About and record the system type and [Windows](https://www.microsoft.com/windows) version.

**[Official OEM driver and firmware portals](https://support.microsoft.com/en-us/windows/hardware/drivers/update-drivers-through-device-manager-in-windows)** · [Official page](https://support.microsoft.com/en-us/windows/hardware/drivers/update-drivers-through-device-manager-in-windows) · **[Find the correct driver source](https://support.microsoft.com/en-us/windows/hardware/drivers/update-drivers-through-device-manager-in-windows)**

1. Run msinfo32 and read System Manufacturer, System Model and System Type; use the actual machine rather than a similar-looking product name.
2. Match the package to that model, [Windows](https://www.microsoft.com/windows) version and processor architecture. An x64 application running under emulation does not make an x64 hardware driver suitable for an ARM device.
3. Keep the model/build record in the private ticket and choose the matching manufacturer download before retrying.

**Prove it worked:** The selected package explicitly supports the actual model and architecture and opens without the compatibility error.

**If this does not resolve it:** If no supported driver exists, use a supported class-driver/device path or escalate compatibility; do not force an unrelated INF.

**Official evidence:** [Source 1](https://support.microsoft.com/en-us/windows/experience/find-information-about-your-windows-device) · [Source 2](https://support.microsoft.com/en-us/windows/experience/platform-variants/windows-arm-based-pcs-faq) · Reviewed 2026-11-26

[Back to specific fixes](#specific-fixes) · [Full procedure](#walkthrough)

<a id="fix-triage-isolate-background-conflict"></a>

### Find the background program blocking an app

**Use when:** An application fails in normal startup but works when optional background software is absent.

**Start here:** Record the exact reproduction and the currently enabled startup items before changing them.

1. Use Microsoft’s clean-boot procedure in an agreed service window: hide Microsoft services and identify the optional services/startup items under test. Coordinate any managed security or remote-access dependency.
2. Retest the same application. If the failure disappears, re-enable a recorded subset at a time and restart to narrow the conflict.
3. Restore the normal configuration after the test; repair or update the identified conflicting product instead of leaving unrelated services disabled.

**Prove it worked:** A specific enabled item reproducibly changes the result, and normal startup is restored with the resolved configuration.

**If this does not resolve it:** If the same failure persists in the controlled startup state, stop disabling more software and move to the application’s own diagnostic route.

**Official evidence:** [Source 1](https://support.microsoft.com/en-us/windows/experience/startup-boot/how-to-perform-a-clean-boot-in-windows) · Reviewed 2026-11-26

[Back to specific fixes](#specific-fixes) · [Full procedure](#walkthrough)

<a id="fix-triage-clock-breaks-signin"></a>

### Correct a bad clock before chasing sign-in errors

**Use when:** Several secure sites or sign-ins fail and the displayed date or time is wrong.

**Start here:** Compare the device’s date, time and time zone with a trusted current reference.

**[PowerShell](https://learn.microsoft.com/en-us/powershell/)** · [Official page](https://learn.microsoft.com/en-us/powershell/) · **[Download Windows x64 portable · ZIP](https://github.com/PowerShell/PowerShell/releases/download/v7.7.0/PowerShell-7.7.0-win-x64.zip)**

1. Open Settings > Time & language > Date & time and distinguish an incorrect time zone from an incorrect clock.
2. On a managed domain PC, read w32tm /query /status to identify the existing synchronization source; restore the organization’s time path rather than choosing an arbitrary public server.
3. On an unmanaged PC, correct the supported automatic-time settings, synchronize, then reopen the affected sign-in or site.

**Prove it worked:** The clock stays correct and the original authentication or certificate-date failure no longer occurs.

**If this does not resolve it:** If the clock resets after power loss, investigate firmware time/battery or management policy; do not bypass certificate warnings.

**Official evidence:** [Source 1](https://learn.microsoft.com/en-us/windows-server/networking/windows-time-service/windows-time-service-tools-and-settings) · Reviewed 2026-11-26

[Back to specific fixes](#specific-fixes) · [Full procedure](#walkthrough)


<a id="-continue-with-the-evidence"></a>

## Related playbooks

| Next playbook | What it resolves |
| --- | --- |
| [Remove malware and browser hijackers](https://github.com/Herbertofury/INTA/wiki/Job-malware-removal) | A measured cleanup workflow for a single [Windows](https://www.microsoft.com/windows) endpoint, with a separate path for a wider compromise. |
| [Diagnose a slow, freezing, or hitching PC](https://github.com/Herbertofury/INTA/wiki/Job-slow-pc) | Find the actual bottleneck and prove the improvement with the same workload. |
| [Rescue data from a failing or unreadable drive](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive) | Preserve the source, create a recoverable image, and perform recovery on a copy. |
| [Bring an office back after a power outage](https://github.com/Herbertofury/INTA/wiki/Job-outage-recovery) | Restore dependencies in order and verify services before releasing the office back to work. |
| [Close a repair with a clear client handoff](https://github.com/Herbertofury/INTA/wiki/Job-client-handoff) | Turn completed technical work into proof the customer can use, with clean access and an actionable service record. |

<a id="sources"></a>

<a id="-source-trail"></a>

## Official sources

- [Microsoft: recovery options in Windows](https://support.microsoft.com/en-us/windows/experience/backup-recovery/recovery-options-in-windows)
- [Microsoft Sysinternals: Process Explorer](https://learn.microsoft.com/en-us/sysinternals/downloads/process-explorer/)

Reviewed **2026-11-26**. Timing is a planning range. A successful command is evidence about that operation; acceptance belongs to the user’s original task.


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
