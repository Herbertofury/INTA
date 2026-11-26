[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

<a id="-contain-ransomware-or-a-wider-compromise"></a>

# [Contain ransomware or a wider compromise](https://github.com/Herbertofury/INTA/wiki/Job-ransomware-response)

Stop spread, preserve useful evidence, and restore services through a clean recovery path.

**Security** · Advanced · Immediate triage; recovery varies · 6 steps

[Decision route](#decision-route) · [Tools](#toolkit) · [Procedure](#walkthrough) · [Verification](#acceptance) · [Sources](#sources) · [Interactive checklist](https://herbertofury.github.io/INTA/#job/ransomware-response)

> [!NOTE]
> **Start here.** Treat live encryption, ransom notes, or several compromised systems as an incident. Start a private incident timeline and contact the designated incident lead.

<a id="decision-route"></a>

<a id="-choose-the-right-branch"></a>

## 🧭 Choose a route

<details>
<summary>View the decision map and escalation routes</summary>

```mermaid
flowchart TD
    A["Confirm incident and start timeline"]:::action --> B{"Can network access be isolated?"}:::decision
    B -->|"Yes"| C["Isolate while preserving volatile evidence"]:::action
    B -->|"No, harm continues"| D["Contain spread with incident lead"]:::stop
    C --> E["Map affected systems and identities"]:::action
    D --> E
    E --> F{"Trusted recovery path available?"}:::decision
    F -->|"No"| G["Protect remaining copies and escalate"]:::stop
    F -->|"Yes"| H["Rebuild on a clean network"]:::action
    H --> I{"Identity and business tests pass?"}:::decision
    I -->|"No"| E
    I -->|"Yes"| J["Incident lead authorizes reconnection"]:::done
    classDef decision fill:#eaf0f8,stroke:#607fa7,color:#172b46
    classDef action fill:#f1f3f6,stroke:#8792a2,color:#263445
    classDef stop fill:#faf0f0,stroke:#a46b70,color:#5e3137
    classDef done fill:#e9edf3,stroke:#6c7d94,color:#253449
```

**Route links:** [Continue this workflow](#walkthrough) · [Acceptance checks](#acceptance) · [Remove malware and browser hijackers](malware-removal.md) · [Prove a backup can actually restore](backup-restore.md) · [Bring an office back after a power outage](outage-recovery.md) · [Close a repair with a clear client handoff](client-handoff.md)

</details>

| Checkpoint | What to carry forward |
| --- | --- |
| **Before changing anything** | Isolate spread and protect evidence and recovery copies. |
| **Keep during the job** | Private incident timeline, affected assets, identities, and preserved logs. |
| **Before handing back** | Validated clean services and incident-lead approval to reconnect. |

<a id="toolkit"></a>

<a id="-build-the-right-toolkit"></a>

## Recommended tools

Select a tool for the next diagnostic or repair step. Check its platform, access and licensing requirements before use.

| Tool / official download | Use it for | Platform & access | Documentation |
| --- | --- | --- | --- |
| [Microsoft Sysinternals Suite](https://learn.microsoft.com/en-us/sysinternals/downloads/sysinternals-suite) | The core portable [Windows](https://www.microsoft.com/windows) troubleshooting kit. | [Windows](https://www.microsoft.com/windows); Free Microsoft utilities | [Open guide](https://learn.microsoft.com/en-us/sysinternals/downloads/) |
| [Wireshark](https://www.wireshark.org/download.html) | Explaining DNS, TCP and application-protocol failures from a bounded capture. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos); Free / open source | [Open guide](https://www.wireshark.org/docs/) |

<a id="walkthrough"></a>

<a id="-work-the-case"></a>

## Procedure

Work in order, record the result of each step, and change route when the evidence calls for it.

<a id="step-01"></a>

### 01 · Isolate affected systems

- [ ] Disconnect affected network links or isolate the relevant switch ports and wireless access. Coordinate the action with the incident lead so essential services are understood. Do not connect backup media or use the compromised endpoint for routine communications.

<a id="step-02"></a>

### 02 · Preserve evidence without allowing spread

- [ ] Keep the device powered on after network isolation when feasible so trained responders can capture volatile evidence. If isolation is impossible and harm is continuing, containment may require shutdown. Record the decision, visible symptoms, time, and who acted.

<a id="step-03"></a>

### 03 · Map the impact

- [ ] List affected devices, identities, remote-access services, shares, cloud accounts, and backup systems. Preserve relevant logs before retention expires. Keep an untouched copy of original evidence and work from copies; do not run a broad cleanup that destroys the investigation trail.

<a id="step-04"></a>

### 04 · Protect the recovery environment

- [ ] Use a known-clean administrative device and a separate recovery network. Verify backup availability without exposing backups to the affected environment. Coordinate credential resets, session revocation, and remote-access restrictions with the incident lead so the attacker does not retain access.

<a id="step-05"></a>

### 05 · Rebuild in dependency order

- [ ] Restore identity, network, storage, and critical business services according to the agreed recovery plan. Rebuild affected systems from trusted images where required. Scan and validate restored data, close the identified entry point, and prevent old vulnerable configurations from being reintroduced.

<a id="step-06"></a>

### 06 · Rejoin only after explicit acceptance

- [ ] Test critical workflows in the clean recovery environment and monitor for recurrence. The incident lead decides when systems may reconnect and when the incident is closed. Record remaining unknowns, recovery gaps, owners, and follow-up actions.

<a id="acceptance"></a>

<a id="-prove-the-result"></a>

## Verification

- [ ] Affected systems are isolated and the incident timeline is complete.
- [ ] Recovery uses known-clean systems and validated backups.
- [ ] Compromised identities and the initial entry point have been addressed.
- [ ] Critical workflows pass and the incident lead authorizes reconnection.

> [!WARNING]
> **Pause this route and escalate when:**
> - You are about to destroy evidence or overwrite the only recovery copy.
> - The scope or identity compromise is beyond the current technician’s response capability.

<a id="-continue-with-the-evidence"></a>

## Related playbooks

| Next playbook | What it resolves |
| --- | --- |
| [Remove malware and browser hijackers](malware-removal.md) | A measured cleanup workflow for a single [Windows](https://www.microsoft.com/windows) endpoint, with a separate path for a wider compromise. |
| [Prove a backup can actually restore](backup-restore.md) | Validate recovery credentials, data integrity, application usability, and recovery time. |
| [Bring an office back after a power outage](outage-recovery.md) | Restore dependencies in order and verify services before releasing the office back to work. |
| [Close a repair with a clear client handoff](client-handoff.md) | Turn completed technical work into proof the customer can use, with clean access and an actionable service record. |

<a id="sources"></a>

<a id="-source-trail"></a>

## Official sources

<details>
<summary>References and review notes</summary>

- [CISA and MS-ISAC: ransomware response guide](https://www.cisa.gov/sites/default/files/publications/CISA_MS-ISAC_Ransomware%20Guide_S508C_.pdf)
- [Microsoft: incident response ransomware approach](https://learn.microsoft.com/en-us/security/compass/incident-response-playbook-dart-ransomware-approach)
- [Microsoft: ransomware investigation and recovery](https://learn.microsoft.com/en-us/defender-xdr/playbook-responding-ransomware-m365-defender)

Reviewed **2026-11-26**. Timing is a planning range. A successful command is evidence about that operation; acceptance belongs to the user’s original task.

</details>

---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)

[← All playbooks](README.md) · [Live Wiki](https://github.com/Herbertofury/INTA/wiki) · [Contribution guide](https://github.com/Herbertofury/INTA/blob/main/CONTRIBUTING.md)
