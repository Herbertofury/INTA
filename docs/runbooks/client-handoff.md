[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

<a id="-close-a-repair-with-a-clear-client-handoff"></a>

# [Close a repair with a clear client handoff](https://github.com/Herbertofury/INTA/wiki/Job-client-handoff)

Turn completed technical work into proof the customer can use, with clean access and an actionable service record.

**Getting started** · Starter · 10–20 min · 6 steps

[Tools and useful links](#working-kit) · [Quick answer](#quick-answer) · [Next action](#next-action) · [Decision route](#decision-route) · [Tools](#toolkit) · [Procedure](#walkthrough) · [Verification](#acceptance) · [Sources](#sources) · [Interactive checklist](https://herbertofury.github.io/INTA/#job/client-handoff)

<a id="working-kit"></a>

## 🧰 Tools and links for this job

**[Open working view](https://herbertofury.github.io/INTA/#job/client-handoff)** · Keep the tool panel beside the instructions.

**First move:** Repeat the original requested task with the user before declaring the repair complete.

### Close the case

| Open the exact resource | What to do there |
| --- | --- |
| **[Technician handoff template](https://github.com/Herbertofury/INTA/wiki/Technician-Handoff)** | Record cause, changes, tests, limits and ownership. |
| **[Service acceptance evidence](https://github.com/Herbertofury/INTA/wiki/Service-Evidence)** | Record observations rather than a generic “fixed” status. |

### Case handling

| Open the exact resource | What to do there |
| --- | --- |
| **[Public knowledge and private case material](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards)** | Keep credentials, recovery keys and customer evidence in the private record. |

<a id="quick-answer"></a>

## ⚡ Quick answer

<a id="quick-complete-a-handoff"></a>

### ✅ Finish the job and hand over

**Repeat the original user task and collect acceptance evidence before closing the case.**

1. Verify the requested outcome and record any remaining issue or agreed limitation.
2. Remove temporary access and case-only tools; record changes and private recovery references in the handoff template.
3. Confirm user acceptance, the next owner and any follow-up date.

**No extra download needed.** Follow the checklist below.

<a id="next-action"></a>

## 🧭 Match the symptom

| You see this | Do this next | You should get |
| --- | --- | --- |
| **The repair looks successful** | Have the user perform the original task and record its actual result, including required peripherals or shared access. | The acceptance evidence matches the requested outcome. |
| **Temporary remote access or one-case tools remain** | End temporary sessions, remove case-only access settings and confirm approved persistent management still works. | The device has no unplanned temporary access or monitoring. |
| **Some work is deferred or uncertain** | Record the exact remaining symptom, next action, owner and follow-up time. | Another technician can continue without repeating completed diagnostics. |
| **Recovery information must be handed over** | Store the approved private reference and confirm the authorized owner can retrieve it; do not paste the secret into the public wiki. | The recovery path is usable without exposing credentials. |

### ✅ Finish the job

- [ ] The user accepts the demonstrated result or explicitly recorded limitation.
- [ ] Changes, recovery references and follow-up ownership are complete.

**Change route when:** The original task still fails or acceptance cannot be demonstrated. · Temporary access or required recovery ownership remains unresolved.

> [!NOTE]
> **Start here.** Return to the acceptance test written at intake. A completed checklist is useful only when the customer’s original task works.

<a id="decision-route"></a>

<a id="-choose-the-right-branch"></a>

## 🧭 Choose a route

```mermaid
flowchart TD
    A["Repeat the intake acceptance task"]:::action --> B{"Original task works?"}:::decision
    B -->|"No"| C["Record blocker and exact next action"]:::stop
    B -->|"Yes"| D["Check adjacent essential functions"]:::action
    D --> E{"Temporary access or settings remain?"}:::decision
    E -->|"Yes"| F["Resolve temporary changes"]:::action
    E -->|"No"| G["Explain cause, changes, and proof"]:::action
    F --> G
    G --> H{"Follow-up needed?"}:::decision
    H -->|"Yes"| I["Assign owner and next action"]:::action
    H -->|"No"| J["Complete the accepted service record"]:::done
    I --> J
    classDef decision fill:#eaf0f8,stroke:#607fa7,color:#172b46
    classDef action fill:#f1f3f6,stroke:#8792a2,color:#263445
    classDef stop fill:#faf0f0,stroke:#a46b70,color:#5e3137
    classDef done fill:#e9edf3,stroke:#6c7d94,color:#253449
```

**Route links:** [Continue this workflow](#walkthrough) · [Acceptance checks](#acceptance) · [Start here: triage any computer problem](intake-triage.md) · [Prove a backup can actually restore](backup-restore.md) · [Run a useful workstation maintenance visit](preventive-maintenance.md)

| Checkpoint | What to carry forward |
| --- | --- |
| **Before changing anything** | Return to the original acceptance test and retained rollback. |
| **Keep during the job** | Verified change summary, evidence, residual issues, and user response. |
| **Before handing back** | Acceptance recorded, temporary access closed, and follow-up owned. |

<a id="toolkit"></a>

<a id="-build-the-right-toolkit"></a>

## Recommended tools

Use the approved private service record and the [intake and handoff template](https://github.com/Herbertofury/INTA/wiki/Technician-Handoff). No specialist application is required.

<a id="walkthrough"></a>

<a id="-work-the-case"></a>

## Procedure

Work in order, record the result of each step, and change route when the evidence calls for it.

<a id="step-01"></a>

### 01 · Retest the original job with the user

- [ ] Have the user repeat the failing action with real, representative data. Compare the result with the intake baseline. Record unresolved symptoms honestly rather than hiding them behind a successful diagnostic command.

<a id="step-02"></a>

### 02 · Check adjacent essentials

- [ ] Verify restart, network, email, printing, security health, backup state, and any feature touched by the repair. Confirm restored files open and that no temporary setting changed a normal workflow.

<a id="step-03"></a>

### 03 · Remove temporary access and artifacts

- [ ] End support sessions, remove case-specific temporary agents if appropriate, and return diagnostic logging to the intended level. Keep required evidence in the private service record; remove credentials and sensitive captures from temporary work areas according to the agreed handling process.

<a id="step-04"></a>

### 04 · Explain the result in plain language

- [ ] Tell the user what caused the problem, what changed, and what they should notice. Distinguish confirmed causes from likely causes. Include any action they must take, such as signing in, storing a recovery key, or scheduling hardware replacement.

<a id="step-05"></a>

### 05 · Record rollback and follow-up

- [ ] Document changed versions/settings, recovery images or backups, tested tasks, remaining issues, and who owns them. Give the next technician the exact next action and the failed routes that should not be repeated.

<a id="step-06"></a>

### 06 · Complete the private handoff record

- [ ] Use the intake/handoff template linked from this wiki. Keep customer names, serials, network details, credentials, and case evidence in the approved private system. The public wiki holds reusable methods only.

<a id="acceptance"></a>

<a id="-prove-the-result"></a>

## Verification

- [ ] The user accepts the tested result or an explicit remaining-work plan.
- [ ] Temporary access is closed and the normal configuration is restored.
- [ ] The service record contains changes, proof, rollback, and next actions.

> [!WARNING]
> **Pause this route and escalate when:**
> - The original symptom still reproduces without a clear next step.
> - Temporary access or sensitive evidence is left in an uncontrolled location.

<a id="-continue-with-the-evidence"></a>

## Related playbooks

| Next playbook | What it resolves |
| --- | --- |
| [Start here: triage any computer problem](intake-triage.md) | Turn a vague symptom into a clear next action, a protected data set, and a useful service record. |
| [Prove a backup can actually restore](backup-restore.md) | Validate recovery credentials, data integrity, application usability, and recovery time. |
| [Run a useful workstation maintenance visit](preventive-maintenance.md) | Check security, updates, hardware, backups, and usability without unnecessary cleanup rituals. |

<a id="sources"></a>

<a id="-source-trail"></a>

## Official sources

- [Microsoft: recovery options in Windows](https://support.microsoft.com/en-us/windows/experience/backup-recovery/recovery-options-in-windows)

Reviewed **2026-11-26**. Timing is a planning range. A successful command is evidence about that operation; acceptance belongs to the user’s original task.


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)

[← All playbooks](README.md) · [Live Wiki](https://github.com/Herbertofury/INTA/wiki) · [Contribution guide](https://github.com/Herbertofury/INTA/blob/main/CONTRIBUTING.md)
