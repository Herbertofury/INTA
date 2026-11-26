[🏠 Home](https://github.com/Herbertofury/INTA/wiki) · [🧭 Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [📚 Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [🧰 Tool directory](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [✨ Open interactive hub](https://herbertofury.github.io/INTA/)

# 📚 The playbook library

Find the symptom, follow the evidence, and leave a tested result. Every playbook includes a decision route, the right toolkit, a numbered walkthrough, and handoff checks.

**22 complete playbooks** · [Choose at the decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Browse methods](https://github.com/Herbertofury/INTA/wiki/Methods)

[🧭 First response & handoff](#first-response) · [🛡️ Security & incident response](#security) · [🪟 Windows & system stability](#windows) · [💾 Storage, backup & rescue](#recovery) · [🌐 Network & office continuity](#office) · [🖨️ Everyday workplace fixes](#workplace) · [🧰 Deployment & maintenance](#deployment) · [✨ AI-assisted investigation](#ai)

<a id="first-response"></a>

## 🧭 First response & handoff

Frame the case, connect to the right device, and close with evidence.

| Playbook | Outcome | Experience | Plan your time |
| --- | --- | --- | --- |
| [Start here: triage any computer problem](https://github.com/Herbertofury/INTA/wiki/Job-intake-triage) | Turn a vague symptom into a clear next action, a protected data set, and a useful service record. | Starter | 10–20 min |
| [Start and finish a clean remote-support session](https://github.com/Herbertofury/INTA/wiki/Job-remote-support) | Choose the right attended or managed tool and leave a clear, auditable handoff. | Starter | 5–15 min setup |
| [Close a repair with a clear client handoff](https://github.com/Herbertofury/INTA/wiki/Job-client-handoff) | Turn completed technical work into proof the customer can use, with clean access and an actionable service record. | Starter | 10–20 min |

<a id="security"></a>

## 🛡️ Security & [incident response](https://github.com/Herbertofury/INTA/wiki/Job-ransomware-response)

Separate a routine cleanup from a wider compromise.

| Playbook | Outcome | Experience | Plan your time |
| --- | --- | --- | --- |
| [Remove malware and browser hijackers](https://github.com/Herbertofury/INTA/wiki/Job-malware-removal) | A measured cleanup workflow for a single [Windows](https://www.microsoft.com/windows) endpoint, with a separate path for a wider compromise. | Intermediate | 45–120 min plus scanning |
| [Contain ransomware or a wider compromise](https://github.com/Herbertofury/INTA/wiki/Job-ransomware-response) | Stop spread, preserve useful evidence, and restore services through a clean recovery path. | Advanced | Immediate triage; recovery varies |

<a id="windows"></a>

## 🪟 [Windows](https://www.microsoft.com/windows) & system stability

Find the bottleneck, repair the cause, and verify the workload.

| Playbook | Outcome | Experience | Plan your time |
| --- | --- | --- | --- |
| [Diagnose a slow, freezing, or hitching PC](https://github.com/Herbertofury/INTA/wiki/Job-slow-pc) | Find the actual bottleneck and prove the improvement with the same workload. | Intermediate | 20–60 min |
| [Repair Windows component or system-file corruption](https://github.com/Herbertofury/INTA/wiki/Job-windows-repair) | Use [Windows](https://www.microsoft.com/windows) servicing tools in a sensible order and validate the specific broken feature. | Intermediate | 30–90 min |
| [Fix a failing Windows update](https://github.com/Herbertofury/INTA/wiki/Job-windows-update) | Diagnose the update code, servicing state, and available space before broad reset scripts. | Intermediate | 20–60 min plus installation |
| [Diagnose crashes, blue screens, and overheating](https://github.com/Herbertofury/INTA/wiki/Job-hardware-bsod) | Correlate crash evidence with memory, storage, temperature, power, and driver changes. | Advanced | 30–120 min plus extended tests |

<a id="recovery"></a>

## 💾 Storage, backup & rescue

Protect the source and prove the recovery path.

| Playbook | Outcome | Experience | Plan your time |
| --- | --- | --- | --- |
| [Recover a Windows PC that will not boot](https://github.com/Herbertofury/INTA/wiki/Job-boot-recovery) | Separate firmware, storage, encryption, and [Windows](https://www.microsoft.com/windows) startup faults before changing the boot configuration. | Advanced | 30–120 min |
| [Rescue data from a failing or unreadable drive](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive) | Preserve the source, create a recoverable image, and perform recovery on a copy. | Advanced | Triage 15–30 min; imaging varies |
| [Clone a healthy system or migrate to a new SSD](https://github.com/Herbertofury/INTA/wiki/Job-disk-migration) | Move a working system with clear source/destination control and a rollback copy. | Intermediate | 45–180 min |
| [Prove a backup can actually restore](https://github.com/Herbertofury/INTA/wiki/Job-backup-restore) | Validate recovery credentials, data integrity, application usability, and recovery time. | Intermediate | 30–90 min; full restore varies |
| [Build and refresh a technician rescue USB](https://github.com/Herbertofury/INTA/wiki/Job-boot-usb) | Keep verified recovery images, [Windows repair](https://github.com/Herbertofury/INTA/wiki/Job-windows-repair) media, and the INTA knowledge hub ready for offline use. | Intermediate | 20–60 min plus downloads |

<a id="office"></a>

## 🌐 Network & office continuity

Restore dependencies in the right order.

| Playbook | Outcome | Experience | Plan your time |
| --- | --- | --- | --- |
| [Fix internet, DNS, or local-network problems](https://github.com/Herbertofury/INTA/wiki/Job-network-dns) | Locate the failing layer before resetting adapters or changing the whole network. | Intermediate | 15–60 min |
| [Bring an office back after a power outage](https://github.com/Herbertofury/INTA/wiki/Job-outage-recovery) | Restore dependencies in order and verify services before releasing the office back to work. | Advanced | 30–120 min; infrastructure varies |
| [Repair a workstation’s domain trust relationship](https://github.com/Herbertofury/INTA/wiki/Job-ad-trust) | Check connectivity, time, and computer identity before repairing a member computer’s secure channel. | Advanced | 20–60 min |

<a id="workplace"></a>

## 🖨️ Everyday workplace fixes

Get the actual mail, document, and printing workflow working.

| Playbook | Outcome | Experience | Plan your time |
| --- | --- | --- | --- |
| [Fix Outlook or email send/receive problems](https://github.com/Herbertofury/INTA/wiki/Job-outlook-email) | Separate service, authentication, mailbox, network, and local-client faults. | Intermediate | 20–60 min |
| [Restore printing without breaking other queues](https://github.com/Herbertofury/INTA/wiki/Job-printer-repair) | Isolate printer hardware, network reachability, queue state, driver, and application behavior. | Starter | 15–45 min |

<a id="deployment"></a>

## 🧰 Deployment & maintenance

Build a dependable workstation and keep it dependable.

| Playbook | Outcome | Experience | Plan your time |
| --- | --- | --- | --- |
| [Set up a dependable new workstation](https://github.com/Herbertofury/INTA/wiki/Job-new-pc) | Build a usable, recoverable workstation around the employee’s actual job. | Intermediate | 60–180 min |
| [Run a useful workstation maintenance visit](https://github.com/Herbertofury/INTA/wiki/Job-preventive-maintenance) | Check security, updates, hardware, backups, and usability without unnecessary cleanup rituals. | Intermediate | 30–60 min |

<a id="ai"></a>

## ✨ AI-assisted investigation

Turn sanitized evidence into a reviewed next test.

| Playbook | Outcome | Experience | Plan your time |
| --- | --- | --- | --- |
| [Use AI as a careful troubleshooting copilot](https://github.com/Herbertofury/INTA/wiki/Job-ai-triage) | Use AI to organize evidence, compare hypotheses, and explain findings while the technician controls execution. | Intermediate | 5–20 min per analysis |

## 🤝 Make the next case easier

Use the [contribution template](https://github.com/Herbertofury/INTA/wiki/Templates) when a case reveals a better sequence, a changed tool, or a missing stop condition. Keep customer evidence in the private service record and add only the reusable method to the [shared knowledge process](https://github.com/Herbertofury/INTA/wiki/Continuous-Improvement).

---

**Source review: 2026-11-26.** Guidance review and tool release versions are separate. [Report a correction](https://github.com/Herbertofury/INTA/issues/new/choose) · [How we maintain this guide](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards)
