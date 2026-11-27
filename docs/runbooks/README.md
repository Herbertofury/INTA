[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

<a id="-the-playbook-library"></a>

# Playbooks

Procedures for common service jobs, with decision diagrams, [tool selection](https://github.com/Herbertofury/INTA/wiki/Tool-Selection), ordered steps, verification and escalation criteria.

**22 playbooks** · [Identify the right route](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Troubleshooting methods](https://github.com/Herbertofury/INTA/wiki/Methods)

[First response & handoff](#first-response) · [Security & incident response](#security) · [Windows & system stability](#windows) · [Storage, backup & rescue](#recovery) · [Network & office continuity](#office) · [Everyday workplace fixes](#workplace) · [Deployment & maintenance](#deployment) · [AI-assisted investigation](#ai)

<a id="first-response"></a>

## 🧭 First response & handoff

Frame the case, connect to the right device, and close with evidence.

| Playbook | Outcome | Experience | Plan your time |
| --- | --- | --- | --- |
| **[🧭 Triage a new issue](intake-triage.md)** · [Working view](https://herbertofury.github.io/INTA/#job/intake-triage) | Turn a vague symptom into a clear next action, a protected data set, and a useful service record. | Starter | 10–20 min |
| **[🖥️ Help remotely](remote-support.md)** · [Working view](https://herbertofury.github.io/INTA/#job/remote-support) | Choose the right attended or managed tool and leave a clear, auditable handoff. | Starter | 5–15 min setup |
| **[📋 Complete the handoff](client-handoff.md)** · [Working view](https://herbertofury.github.io/INTA/#job/client-handoff) | Turn completed technical work into proof the customer can use, with clean access and an actionable service record. | Starter | 10–20 min |

<a id="security"></a>

## 🛡️ Security & [incident response](https://github.com/Herbertofury/INTA/wiki/Job-ransomware-response)

Separate a routine cleanup from a wider compromise.

| Playbook | Outcome | Experience | Plan your time |
| --- | --- | --- | --- |
| **[🦠 Remove viruses](malware-removal.md)** · [Working view](https://herbertofury.github.io/INTA/#job/malware-removal) | A measured cleanup workflow for a single [Windows](https://www.microsoft.com/windows) endpoint, with a separate path for a wider compromise. | Intermediate | 45–120 min plus scanning |
| **[🚨 Contain ransomware](ransomware-response.md)** · [Working view](https://herbertofury.github.io/INTA/#job/ransomware-response) | Stop spread, preserve useful evidence, and restore services through a clean recovery path. | Advanced | Immediate triage; recovery varies |

<a id="windows"></a>

## 🪟 [Windows](https://www.microsoft.com/windows) & system stability

Find the bottleneck, repair the cause, and verify the workload.

| Playbook | Outcome | Experience | Plan your time |
| --- | --- | --- | --- |
| **[🐢 Fix a slow PC](slow-pc.md)** · [Working view](https://herbertofury.github.io/INTA/#job/slow-pc) | Find the actual bottleneck and prove the improvement with the same workload. | Intermediate | 20–60 min |
| **[🛠️ Repair Windows](windows-repair.md)** · [Working view](https://herbertofury.github.io/INTA/#job/windows-repair) | Use [Windows](https://www.microsoft.com/windows) servicing tools in a sensible order and validate the specific broken feature. | Intermediate | 30–90 min |
| **[🔄 Fix Windows updates](windows-update.md)** · [Working view](https://herbertofury.github.io/INTA/#job/windows-update) | Diagnose the update code, servicing state, and available space before broad reset scripts. | Intermediate | 20–60 min plus installation |
| **[🌡️ Diagnose crashes / BSOD](hardware-bsod.md)** · [Working view](https://herbertofury.github.io/INTA/#job/hardware-bsod) | Correlate crash evidence with memory, storage, temperature, power, and driver changes. | Advanced | 30–120 min plus extended tests |

<a id="recovery"></a>

## 💾 Storage, backup & rescue

Protect the source and prove the recovery path.

| Playbook | Outcome | Experience | Plan your time |
| --- | --- | --- | --- |
| **[🚑 Fix boot failure](boot-recovery.md)** · [Working view](https://herbertofury.github.io/INTA/#job/boot-recovery) | Separate firmware, storage, encryption, and [Windows](https://www.microsoft.com/windows) startup faults before changing the boot configuration. | Advanced | 30–120 min |
| **[💾 Recover data](failing-drive.md)** · [Working view](https://herbertofury.github.io/INTA/#job/failing-drive) | Preserve the source, create a recoverable image, and perform recovery on a copy. | Advanced | Triage 15–30 min; imaging varies |
| **[💽 Clone or migrate a disk](disk-migration.md)** · [Working view](https://herbertofury.github.io/INTA/#job/disk-migration) | Move a working system with clear source/destination control and a rollback copy. | Intermediate | 45–180 min |
| **[🗄️ Test a backup restore](backup-restore.md)** · [Working view](https://herbertofury.github.io/INTA/#job/backup-restore) | Validate recovery credentials, data integrity, application usability, and recovery time. | Intermediate | 30–90 min; full restore varies |
| **[🔌 Build a rescue USB](boot-usb.md)** · [Working view](https://herbertofury.github.io/INTA/#job/boot-usb) | Keep verified recovery images, [Windows repair](https://github.com/Herbertofury/INTA/wiki/Job-windows-repair) media, and the INTA knowledge hub ready for offline use. | Intermediate | 20–60 min plus downloads |

<a id="office"></a>

## 🌐 Network & office continuity

Restore dependencies in the right order.

| Playbook | Outcome | Experience | Plan your time |
| --- | --- | --- | --- |
| **[🌐 Fix internet & DNS](network-dns.md)** · [Working view](https://herbertofury.github.io/INTA/#job/network-dns) | Locate the failing layer before resetting adapters or changing the whole network. | Intermediate | 15–60 min |
| **[⚡ Recover from an outage](outage-recovery.md)** · [Working view](https://herbertofury.github.io/INTA/#job/outage-recovery) | Restore dependencies in order and verify services before releasing the office back to work. | Advanced | 30–120 min; infrastructure varies |
| **[🔑 Repair domain trust](ad-trust.md)** · [Working view](https://herbertofury.github.io/INTA/#job/ad-trust) | Check connectivity, time, and computer identity before repairing a member computer’s secure channel. | Advanced | 20–60 min |

<a id="workplace"></a>

## 🖨️ Everyday workplace fixes

Get the actual mail, document, and printing workflow working.

| Playbook | Outcome | Experience | Plan your time |
| --- | --- | --- | --- |
| **[📧 Fix email](outlook-email.md)** · [Working view](https://herbertofury.github.io/INTA/#job/outlook-email) | Separate service, authentication, mailbox, network, and local-client faults. | Intermediate | 20–60 min |
| **[🖨️ Fix a printer](printer-repair.md)** · [Working view](https://herbertofury.github.io/INTA/#job/printer-repair) | Isolate printer hardware, network reachability, queue state, driver, and application behavior. | Starter | 15–45 min |

<a id="deployment"></a>

## 🧰 Deployment & maintenance

Build a dependable workstation and keep it dependable.

| Playbook | Outcome | Experience | Plan your time |
| --- | --- | --- | --- |
| **[🆕 Set up a new PC](new-pc.md)** · [Working view](https://herbertofury.github.io/INTA/#job/new-pc) | Build a usable, recoverable workstation around the employee’s actual job. | Intermediate | 60–180 min |
| **[🧹 Maintain a workstation](preventive-maintenance.md)** · [Working view](https://herbertofury.github.io/INTA/#job/preventive-maintenance) | Check security, updates, hardware, backups, and usability without unnecessary cleanup rituals. | Intermediate | 30–60 min |

<a id="ai"></a>

## 🧠 AI-assisted investigation

Turn sanitized evidence into a reviewed next test.

| Playbook | Outcome | Experience | Plan your time |
| --- | --- | --- | --- |
| **[🧠 Troubleshoot with AI](ai-triage.md)** · [Working view](https://herbertofury.github.io/INTA/#job/ai-triage) | Use AI to organize evidence, compare hypotheses, and explain findings while the technician controls execution. | Intermediate | 5–20 min per analysis |

<a id="-make-the-next-case-easier"></a>

## Corrections and additions

Use the [contribution template](https://github.com/Herbertofury/INTA/wiki/Templates) when a case reveals a better sequence, a changed tool, or a missing stop condition. Keep customer evidence in the private service record and add only the reusable method to the [shared knowledge process](https://github.com/Herbertofury/INTA/wiki/Continuous-Improvement).

---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)

[← All playbooks](README.md) · [Live Wiki](https://github.com/Herbertofury/INTA/wiki) · [Contribution guide](https://github.com/Herbertofury/INTA/blob/main/CONTRIBUTING.md)
