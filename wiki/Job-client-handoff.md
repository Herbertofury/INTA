[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Service map](https://github.com/Herbertofury/INTA/wiki/Service-Map) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

<a id="-close-a-repair-with-a-clear-client-handoff"></a>

# [Close a repair with a clear client handoff](https://github.com/Herbertofury/INTA/wiki/Job-client-handoff)

Turn completed technical work into proof the customer can use, with clean access and an actionable service record.

**Getting started** · Starter · 10–20 min · 6 steps

[Tools and useful links](#working-kit) · [Quick answer](#quick-answer) · [Next action](#next-action) · [Specific fixes](#specific-fixes) · [Decision route](#decision-route) · [Tools](#toolkit) · [Procedure](#walkthrough) · [Verification](#acceptance) · [Sources](#sources) · [Interactive checklist](https://herbertofury.github.io/INTA/#job/client-handoff)

<a id="working-kit"></a>

## 🧰 Tools and links for this job

**[Open working view](https://herbertofury.github.io/INTA/#job/client-handoff)** · Keep the tool panel beside the instructions.

**First move:** Repeat the original requested task with the user before declaring the repair complete.

| Use this for | Official tool + exact action |
| --- | --- |
| Comparing a saved configuration with the changed version or checking a copied folder before accepting a repair or migration. | **[WinMerge](https://winmerge.org/)** · [Official page](https://winmerge.org/) · **[Download Windows x64 portable · ZIP](https://github.com/WinMerge/winmerge/releases/download/v2.16.60/winmerge-2.16.60-x64-exe.zip)** |

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

### 🎯 Go straight to a specific fix

- **[Prove the repair under the real user account](#fix-handoff-admin-works-user-denied)** — The technician can open a repaired or restored file, but the normal user gets Access denied.
- **[Make recovery information retrievable](#fix-handoff-recovery-record-cannot-be-found)** — Encryption works, but the owner cannot locate the recovery record during handoff.
- **[Remove access that was only for this repair](#fix-handoff-temporary-support-app-persists)** — A temporary support program or unattended setting still starts after the case ends.

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

**Route links:** [Continue this workflow](#walkthrough) · [Acceptance checks](#acceptance) · [Start here: triage any computer problem](https://github.com/Herbertofury/INTA/wiki/Job-intake-triage) · [Prove a backup can actually restore](https://github.com/Herbertofury/INTA/wiki/Job-backup-restore) · [Run a useful workstation maintenance visit](https://github.com/Herbertofury/INTA/wiki/Job-preventive-maintenance)

| Checkpoint | What to carry forward |
| --- | --- |
| **Before changing anything** | Return to the original acceptance test and retained rollback. |
| **Keep during the job** | Verified change summary, evidence, residual issues, and user response. |
| **Before handing back** | Acceptance recorded, temporary access closed, and follow-up owned. |

<a id="toolkit"></a>

<a id="-build-the-right-toolkit"></a>

## Recommended tools

Select a tool for the next diagnostic or repair step. Check its platform, access and licensing requirements before use.

| Tool / official page + download | Use it for | Platform & access | Documentation |
| --- | --- | --- | --- |
| **[WinMerge](https://winmerge.org/)** · [Official page](https://winmerge.org/) · **[Download Windows x64 portable · ZIP](https://github.com/WinMerge/winmerge/releases/download/v2.16.60/winmerge-2.16.60-x64-exe.zip)** | Comparing a saved configuration with the changed version or checking a copied folder before accepting a repair or migration. | [Windows](https://www.microsoft.com/windows); Free and open source under GPL-2.0-or-later | [Open guide](https://manual.winmerge.org/en/) |

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

<a id="specific-fixes"></a>

## 🎯 Specific fixes

Choose the case that matches the evidence. Each case gives a first action, a short procedure and a result to prove.

<a id="fix-handoff-admin-works-user-denied"></a>

### Prove the repair under the real user account

**Use when:** The technician can open a repaired or restored file, but the normal user gets Access denied.

**Start here:** Reproduce the exact task under the intended standard-user account.

**[PowerShell](https://learn.microsoft.com/en-us/powershell/)** · [Official page](https://learn.microsoft.com/en-us/powershell/) · **[Download Windows x64 portable · ZIP](https://github.com/PowerShell/PowerShell/releases/download/v7.7.0/PowerShell-7.7.0-win-x64.zip)**

1. Compare the intended path and file ownership/permissions with a working equivalent; use a read-only permissions view before changing ACLs.
2. Correct only the identified ownership or access issue through the approved application/share/file owner.
3. Have the user reopen, edit and save an agreed test item without elevation.

**Prove it worked:** The user’s actual workflow succeeds with the intended limited permissions.

**If this does not resolve it:** Do not grant Everyone full control or make the user an administrator to hide a permission fault.

**Official evidence:** [Source 1](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/icacls) · Reviewed 2026-11-26

[Back to specific fixes](#specific-fixes) · [Full procedure](#walkthrough)

<a id="fix-handoff-recovery-record-cannot-be-found"></a>

### Make recovery information retrievable

**Use when:** Encryption works, but the owner cannot locate the recovery record during handoff.

**Start here:** Identify the approved account or organization store that should hold the recovery key.

1. Use a separate trusted device to verify that the authorized owner can reach the relevant recovery record and match the device/key identity.
2. If the key is not backed up, follow the supported key-backup procedure using the approved destination.
3. Record the private retrieval reference and responsible owner without copying the secret into the public wiki or ordinary handoff text.

**Prove it worked:** The authorized owner can retrieve the correct recovery material independently of the repaired PC.

**If this does not resolve it:** If retrieval cannot be demonstrated, leave that recovery gap explicitly open rather than claiming the machine is fully recoverable.

**Official evidence:** [Source 1](https://support.microsoft.com/en-us/windows/security/encryption/back-up-your-bitlocker-recovery-key) · [Source 2](https://support.microsoft.com/en-us/windows/security/encryption/find-your-bitlocker-recovery-key) · Reviewed 2026-11-26

[Back to specific fixes](#specific-fixes) · [Full procedure](#walkthrough)

<a id="fix-handoff-temporary-support-app-persists"></a>

### Remove access that was only for this repair

**Use when:** A temporary support program or unattended setting still starts after the case ends.

**Start here:** Compare the current installation/access state with the agreed end state for this device.

**[RustDesk](https://rustdesk.com/)** · [Official page](https://rustdesk.com/) · **[Download Windows x64 · EXE](https://github.com/rustdesk/rustdesk/releases/download/1.5.3/rustdesk-1.5.3-x86_64.exe)**

**[MeshCentral](https://meshcentral.com/)** · [Official page](https://meshcentral.com/) · **[Open server installation instructions](https://github.com/Ylianst/MeshCentral#installation)**

1. End all case-only sessions and remove the temporary unattended credential or enrollment through that product’s supported controls.
2. Uninstall a case-only client when appropriate; keep the organization’s approved persistent management agent intact.
3. Restart if required and verify that the temporary access is gone while approved support and the repaired task still work.

**Prove it worked:** Remaining access has an explicit owner and purpose; one-case access no longer functions.

**If this does not resolve it:** If the temporary and managed deployments cannot be distinguished, involve the management owner before removing agents or services.

**Official evidence:** [Source 1](https://support.microsoft.com/en-us/windows/apps/uninstall-or-remove-apps-and-programs-in-windows) · [Source 2](https://rustdesk.com/docs/en/client/) · [Source 3](https://github.com/Ylianst/MeshCentral) · Reviewed 2026-11-26

[Back to specific fixes](#specific-fixes) · [Full procedure](#walkthrough)


<a id="-continue-with-the-evidence"></a>

## Related playbooks

| Next playbook | What it resolves |
| --- | --- |
| [Start here: triage any computer problem](https://github.com/Herbertofury/INTA/wiki/Job-intake-triage) | Turn a vague symptom into a clear next action, a protected data set, and a useful service record. |
| [Prove a backup can actually restore](https://github.com/Herbertofury/INTA/wiki/Job-backup-restore) | Validate recovery credentials, data integrity, application usability, and recovery time. |
| [Run a useful workstation maintenance visit](https://github.com/Herbertofury/INTA/wiki/Job-preventive-maintenance) | Check security, updates, hardware, backups, and usability without unnecessary cleanup rituals. |

<a id="sources"></a>

<a id="-source-trail"></a>

## Official sources

- [Microsoft: recovery options in Windows](https://support.microsoft.com/en-us/windows/experience/backup-recovery/recovery-options-in-windows)

Reviewed **2026-11-26**. Timing is a planning range. A successful command is evidence about that operation; acceptance belongs to the user’s original task.


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
