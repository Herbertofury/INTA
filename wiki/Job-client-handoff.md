[🏠 Home](https://github.com/Herbertofury/INTA/wiki) · [🧭 Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [📚 Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [🧰 Tool directory](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [✨ Open interactive hub](https://herbertofury.github.io/INTA/)

# 🤝 [Close a repair with a clear client handoff](https://github.com/Herbertofury/INTA/wiki/Job-client-handoff)

Turn completed technical work into proof the customer can use, with clean access and an actionable service record.

| Work area | Experience | Plan your time | Workflow |
| --- | --- | --- | --- |
| Getting started | Starter | 10–20 min | 6 steps + acceptance |

**[▶ Open the interactive checklist](https://herbertofury.github.io/INTA/#job/client-handoff)** · [Decision route](#decision-route) · [Toolkit](#toolkit) · [Walkthrough](#walkthrough) · [Acceptance](#acceptance) · [Sources](#sources)

> [!TIP]
> **Your starting move:** Return to the acceptance test written at intake. A completed checklist is useful only when the customer’s original task works.

<a id="decision-route"></a>

## 🧭 Choose the right branch

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
    classDef decision fill:#16323d,stroke:#5eead4,color:#f0fdfa
    classDef action fill:#172033,stroke:#64748b,color:#f8fafc
    classDef stop fill:#3b222b,stroke:#fb7185,color:#fff1f2
    classDef done fill:#12352d,stroke:#34d399,color:#ecfdf5
```

**Diagram routes:** [Continue this workflow](#walkthrough) · [Acceptance checks](#acceptance) · [Start here: triage any computer problem](https://github.com/Herbertofury/INTA/wiki/Job-intake-triage) · [Prove a backup can actually restore](https://github.com/Herbertofury/INTA/wiki/Job-backup-restore) · [Run a useful workstation maintenance visit](https://github.com/Herbertofury/INTA/wiki/Job-preventive-maintenance)

| Checkpoint | What to carry forward |
| --- | --- |
| **Before changing anything** | Return to the original acceptance test and retained rollback. |
| **Keep during the job** | Verified change summary, evidence, residual issues, and user response. |
| **Before handing back** | Acceptance recorded, temporary access closed, and follow-up owned. |

<a id="toolkit"></a>

## 🧰 Build the right toolkit

Use the approved private service record and the [intake and handoff template](https://github.com/Herbertofury/INTA/wiki/Technician-Handoff). No specialist application is required.

<a id="walkthrough"></a>

## 🪜 Work the case

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

## ✅ Prove the result

- [ ] The user accepts the tested result or an explicit remaining-work plan.
- [ ] Temporary access is closed and the normal configuration is restored.
- [ ] The service record contains changes, proof, rollback, and next actions.

> [!WARNING]
> **Pause this route and escalate when:**
> - The original symptom still reproduces without a clear next step.
> - Temporary access or sensitive evidence is left in an uncontrolled location.

## 🔀 Continue with the evidence

| Next playbook | What it resolves |
| --- | --- |
| [Start here: triage any computer problem](https://github.com/Herbertofury/INTA/wiki/Job-intake-triage) | Turn a vague symptom into a clear next action, a protected data set, and a useful service record. |
| [Prove a backup can actually restore](https://github.com/Herbertofury/INTA/wiki/Job-backup-restore) | Validate recovery credentials, data integrity, application usability, and recovery time. |
| [Run a useful workstation maintenance visit](https://github.com/Herbertofury/INTA/wiki/Job-preventive-maintenance) | Check security, updates, hardware, backups, and usability without unnecessary cleanup rituals. |

Found a reusable improvement? [Contribute the correction](https://github.com/Herbertofury/INTA/wiki/Contribute) and include the test that proved it.

<a id="sources"></a>

## 🔎 Source trail

<details>
<summary>Official references and editorial review</summary>

- [Microsoft: recovery options in Windows](https://support.microsoft.com/en-us/windows/experience/backup-recovery/recovery-options-in-windows)

Reviewed **2026-11-26**. Timing is a planning range. A successful command is evidence about that operation; acceptance belongs to the user’s original task.

</details>

---

**Source review: 2026-11-26.** Guidance review and tool release versions are separate. [Report a correction](https://github.com/Herbertofury/INTA/issues/new/choose) · [How we maintain this guide](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards)
