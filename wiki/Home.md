![INTA Computer Services — technical knowledge base](https://github.com/Herbertofury/INTA/raw/main/docs/assets/wiki-hero.svg)

<a id="-your-next-good-fix-starts-here"></a>

# INTA knowledge base

Service procedures, tool references and troubleshooting methods for the INTA team.

[Open technician hub](https://herbertofury.github.io/INTA/) · [Decision desk](Decision-Desk) · [Playbooks](Playbooks) · [Tool directory](Tool-Directory) · [Methods](Methods)

**113 tools · 22 playbooks · 10 tool categories**

<a id="free-tools"></a>

## ⚡ Free tools for common jobs

Open a tool below, or select the scenario for its step-by-step guide. [How these picks were chosen](https://github.com/Herbertofury/INTA/wiki/Free-Tool-Picks).

| Scenario | Primary free tool |
| --- | --- |
| **[🛡️ Suspected malware](https://github.com/Herbertofury/INTA/wiki/Job-malware-removal)** | **[Microsoft Safety Scanner](https://learn.microsoft.com/en-us/defender-endpoint/safety-scanner-download)** |
| **[📁 Deleted files](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive)** | **[PhotoRec](https://www.cgsecurity.org/wiki/TestDisk_Download)** |
| **[💾 Failing drive](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive)** | **[GNU ddrescue](https://ftp.gnu.org/gnu/ddrescue/)** |
| **[🐢 Slow PC](https://github.com/Herbertofury/INTA/wiki/Job-slow-pc)** | **[Microsoft Sysinternals Suite](https://learn.microsoft.com/en-us/sysinternals/downloads/sysinternals-suite)** |
| **[📊 Disk space](https://github.com/Herbertofury/INTA/wiki/Job-slow-pc)** | **[WinDirStat](https://github.com/windirstat/windirstat/releases/latest)** |
| **[🔎 Find a file](https://github.com/Herbertofury/INTA/wiki/Job-preventive-maintenance)** | **[Everything](https://www.voidtools.com/downloads/)** |
| **[🖥️ Remote help](https://github.com/Herbertofury/INTA/wiki/Job-remote-support)** | **[Microsoft Quick Assist](https://apps.microsoft.com/detail/9P7BP5VNWKX5)** |
| **[🔌 Boot USB](https://github.com/Herbertofury/INTA/wiki/Job-boot-usb)** | **[Rufus](https://rufus.ie/#download)** |
| **[🧰 Windows repair](https://github.com/Herbertofury/INTA/wiki/Job-windows-repair)** | **[DISM](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image)** |
| **[🌐 Inspect network traffic](https://github.com/Herbertofury/INTA/wiki/Job-network-dns)** | **[Wireshark](https://www.wireshark.org/download.html)** |
| **[🧪 Suspected RAM fault](https://github.com/Herbertofury/INTA/wiki/Job-hardware-bsod)** | **[Memtest86+](https://www.memtest.org/#download)** |
| **[🗄️ File backup](https://github.com/Herbertofury/INTA/wiki/Job-backup-restore)** | **[restic](https://github.com/restic/restic/releases/latest)** |

Selection review: **2026-11-26** · [Complete tool directory](https://github.com/Herbertofury/INTA/wiki/Tool-Directory)

<a id="-first-choose-the-right-route"></a>

## 🧭 Start a job

<a id="playbooks-by-work-area"></a>
<a id="️-find-your-work-area"></a>
<a id="-find-your-work-area"></a>

| Scenario | Outcome |
| --- | --- |
| **[🦠 Remove viruses](Job-malware-removal)** | A measured cleanup workflow for a single [Windows](https://www.microsoft.com/windows) endpoint, with a separate path for a wider compromise. |
| **[💾 Recover data](Job-failing-drive)** | Preserve the source, create a recoverable image, and perform recovery on a copy. |
| **[🐢 Fix a slow PC](Job-slow-pc)** | Find the actual bottleneck and prove the improvement with the same workload. |
| **[🖥️ Help remotely](Job-remote-support)** | Choose the right attended or managed tool and leave a clear, auditable handoff. |
| **[🧭 Triage a new issue](Job-intake-triage)** | Turn a vague symptom into a clear next action, a protected data set, and a useful service record. |
| **[🌐 Fix internet & DNS](Job-network-dns)** | Locate the failing layer before resetting adapters or changing the whole network. |
| **[🚑 Fix boot failure](Job-boot-recovery)** | Separate firmware, storage, encryption, and [Windows](https://www.microsoft.com/windows) startup faults before changing the boot configuration. |
| **[🛠️ Repair Windows](Job-windows-repair)** | Use [Windows](https://www.microsoft.com/windows) servicing tools in a sensible order and validate the specific broken feature. |
| **[🔄 Fix Windows updates](Job-windows-update)** | Diagnose the update code, servicing state, and available space before broad reset scripts. |
| **[🌡️ Diagnose crashes / BSOD](Job-hardware-bsod)** | Correlate crash evidence with memory, storage, temperature, power, and driver changes. |
| **[📧 Fix email](Job-outlook-email)** | Separate service, authentication, mailbox, network, and local-client faults. |
| **[🖨️ Fix printing](Job-printer-repair)** | Isolate printer hardware, network reachability, queue state, driver, and application behavior. |
| **[🚨 Contain ransomware](Job-ransomware-response)** | Stop spread, preserve useful evidence, and restore services through a clean recovery path. |
| **[🗄️ Test a backup restore](Job-backup-restore)** | Validate recovery credentials, data integrity, application usability, and recovery time. |
| **[💽 Clone or migrate a disk](Job-disk-migration)** | Move a working system with clear source/destination control and a rollback copy. |
| **[🆕 Set up a new PC](Job-new-pc)** | Build a usable, recoverable workstation around the employee’s actual job. |
| **[🔌 Build a rescue USB](Job-boot-usb)** | Keep verified recovery images, [Windows repair](https://github.com/Herbertofury/INTA/wiki/Job-windows-repair) media, and the INTA knowledge hub ready for offline use. |
| **[⚡ Recover from an outage](Job-outage-recovery)** | Restore dependencies in order and verify services before releasing the office back to work. |
| **[🔑 Repair domain trust](Job-ad-trust)** | Check connectivity, time, and computer identity before repairing a member computer’s secure channel. |
| **[🧹 Maintain a workstation](Job-preventive-maintenance)** | Check security, updates, hardware, backups, and usability without unnecessary cleanup rituals. |
| **[🧠 Troubleshoot with AI](Job-ai-triage)** | Use AI to organize evidence, compare hypotheses, and explain findings while the technician controls execution. |
| **[📋 Complete the handoff](Job-client-handoff)** | Turn completed technical work into proof the customer can use, with clean access and an actionable service record. |

## Before you begin

| Observed situation | First route |
| --- | --- |
| Encryption spreading, unusual privileged access, or several affected machines | [Contain a wider compromise](Job-ransomware-response) |
| Clicking storage, disappearing volumes, or data with no other copy | [Protect and recover the source](Job-failing-drive) |
| The whole office or a shared service is down | [Restore office dependencies](Job-outage-recovery) |
| The cause is still unclear | [Scope and triage the problem](Job-intake-triage) |

<details>
<summary>Decision map: from first observation to verified handoff</summary>

```mermaid
flowchart TD
    A["Start with the observed problem"]:::action --> B{"Immediate risk?"}:::decision
    B -->|"Active compromise"| C["Contain and coordinate response"]:::stop
    B -->|"Failing storage or damage"| D["Protect data and choose recovery"]:::stop
    B -->|"No immediate risk"| E{"What is the scope?"}:::decision
    E -->|"Office or shared service"| F["Restore dependencies in order"]:::action
    E -->|"One device or application"| G["Choose a focused playbook"]:::action
    F --> H["Test the real business task"]:::done
    G --> H
    H --> I{"Evidence worth reusing?"}:::decision
    I -->|"New method or correction"| J["Improve the shared guide"]:::done
    I -->|"Case-specific details"| K["Complete the private handoff"]:::done
    classDef decision fill:#eaf0f8,stroke:#607fa7,color:#172b46
    classDef action fill:#f1f3f6,stroke:#8792a2,color:#263445
    classDef stop fill:#faf0f0,stroke:#a46b70,color:#5e3137
    classDef done fill:#e9edf3,stroke:#6c7d94,color:#253449
```

**Map routes:** [Containment](Job-ransomware-response) · [Data preservation](Job-failing-drive) · [Outage recovery](Job-outage-recovery) · [Triage](Job-intake-triage) · [Verification and handoff](Job-client-handoff) · [Contributions](Contribute)

</details>

<a id="-put-the-right-resource-within-reach"></a>

## Reference

| Resource | Contents |
| --- | --- |
| [Tool directory](Tool-Directory) | Use cases, supported platforms, access, official downloads and documentation. |
| [Method library](Methods) | Reusable diagnostic methods, selection criteria and expected evidence. |
| [Rescue media](Rescue-Media) | Boot environments, media preparation and readiness checks. |
| [Intake and handoff](Technician-Handoff) | A private service record template and technician handoff criteria. |
| [Service evidence](Service-Evidence) | Before-and-after measurements, validation records and useful evidence limits. |
| [Tool selection](Tool-Selection) | Match a utility to the job, operating environment and commercial-use requirements. |
| [Free Tool Picks](Free-Tool-Picks) | Business-use terms, practical limits and source evidence for the quick-start selections. |
| [Knowledge standards](Knowledge-Standards) | Authoring rules for instructions, links, diagrams, evidence and review. |
| [Interface design](Interface-Design) | Visual standards for the hub, Wiki and documentation. |
| [Contribution guide](Contribute) | Review and publication workflow, with a [submission template](Templates). |

<a id="-the-inta-working-standard"></a>

## Service checkpoints

| Before work | During work | Before closure |
| --- | --- | --- |
| Establish scope and protect unique data. | Record the result of each purposeful test. | Repeat the real task and document the handoff. |
| Choose recovery or [incident response](https://github.com/Herbertofury/INTA/wiki/Job-ransomware-response) early when indicated. | Preserve rollback and investigate unchanged failures. | Contribute a proven reusable improvement. |

<a id="-take-the-whole-bench-guide-offline"></a>

## Offline access

Download the [complete repository](https://github.com/Herbertofury/INTA/archive/refs/heads/main.zip) and open `site/index.html`. Search, filters, saved tools and checklists remain available without a connection. Official downloads and external references require internet access. Keep customer records outside this public guide.

<a id="-improve-it-after-the-next-real-case"></a>

## Contributing

Submit a corrected download, improved diagnostic branch, evaluated tool or missing verification step through the [contribution guide](Contribute). Follow the [review process](Continuous-Improvement), [record the evidence](Templates), and [publish the matching outputs](Maintaining-the-Hub).

---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
