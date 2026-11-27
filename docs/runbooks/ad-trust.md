[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

<a id="-repair-a-workstations-domain-trust-relationship"></a>

# [Repair a workstation’s domain trust relationship](https://github.com/Herbertofury/INTA/wiki/Job-ad-trust)

Check connectivity, time, and computer identity before repairing a member computer’s secure channel.

**Identity & business systems** · Advanced · 20–60 min · 6 steps

[Tools and useful links](#working-kit) · [Quick answer](#quick-answer) · [Next action](#next-action) · [Decision route](#decision-route) · [Tools](#toolkit) · [Procedure](#walkthrough) · [Verification](#acceptance) · [Sources](#sources) · [Interactive checklist](https://herbertofury.github.io/INTA/#job/ad-trust)

<a id="working-kit"></a>

## 🧰 Tools and links for this job

**[Open working view](https://herbertofury.github.io/INTA/#job/ad-trust)** · Keep the tool panel beside the instructions.

**First move:** Use the affected member PC. Check its domain DNS, time and domain-controller reachability before considering a trust repair.

| Use this for | Official tool + exact action |
| --- | --- |
| Use existing [Windows PowerShell 5.1](https://learn.microsoft.com/en-us/powershell/scripting/windows-powershell/starting-windows-powershell) on the member PC. | **[PowerShell](https://learn.microsoft.com/en-us/powershell/)** · [Official page](https://learn.microsoft.com/en-us/powershell/) · **[Use Windows PowerShell 5.1](https://learn.microsoft.com/en-us/powershell/scripting/windows-powershell/starting-windows-powershell)** |

### Diagnostic and repair

| Open the exact resource | What to do there |
| --- | --- |
| **[Test-ComputerSecureChannel](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.management/test-computersecurechannel?view=powershell-5.1)** | Read-only test first; repair only a confirmed member-computer fault. |
| **[Microsoft trust-relationship troubleshooting](https://learn.microsoft.com/en-us/troubleshoot/windows-server/windows-security/broken-trust-relationship-domain-joined-device-its-domain-secure-channel-issues)** | Check connectivity, identity and the cause before rejoining. |

### Read-only diagnostics

| Open the exact resource | What to do there |
| --- | --- |
| **[Read the member PC network configuration](https://learn.microsoft.com/en-us/powershell/module/nettcpip/get-netipconfiguration)** | Confirm the intended domain DNS path. |

<a id="quick-answer"></a>

## ⚡ Quick answer

<a id="quick-repair-domain-trust"></a>

### 🔐 Repair domain sign-in

**On the affected domain-member PC, check the domain DNS, time and domain-controller reachability first.**

1. Use existing [Windows PowerShell 5.1](https://learn.microsoft.com/en-us/powershell/scripting/windows-powershell/starting-windows-powershell) to run the guide’s diagnostic secure-channel check on the member PC.
2. Repair only a confirmed trust fault using an authorized account and the scoped procedure.
3. Verify domain sign-in, a required shared resource and the secure-channel result.

**[PowerShell](https://learn.microsoft.com/en-us/powershell/)** · [Official page](https://learn.microsoft.com/en-us/powershell/) · **[Use Windows PowerShell 5.1](https://learn.microsoft.com/en-us/powershell/scripting/windows-powershell/starting-windows-powershell)**

<a id="next-action"></a>

## 🧭 Match the symptom

| You see this | Do this next | You should get |
| --- | --- | --- |
| **Domain names or controllers cannot be reached** | Correct the approved network/VPN, DNS or time issue before touching the computer account. | The member can resolve and reach its expected domain services. |
| **Connectivity is correct but the trust error remains** | Open [Windows PowerShell 5.1](https://learn.microsoft.com/en-us/powershell/scripting/windows-powershell/starting-windows-powershell) as administrator on the member PC and run Test-ComputerSecureChannel -Verbose. | True confirms that this channel works; False supplies evidence for the scoped repair branch. |
| **A member secure-channel fault is confirmed** | With approved repair rights and local recovery access, run Test-ComputerSecureChannel -Repair -Credential (Get-Credential) -Verbose in that elevated member-PC session; enter delegated credentials only in the prompt. | The diagnostic returns True after repair. |
| **The device is a domain controller or many devices fail** | Use the domain-administration incident route instead of this member-only cmdlet. | The correct infrastructure owner investigates the wider fault. |

### ✅ Finish the job

- [ ] The member secure-channel test succeeds.
- [ ] A normal domain sign-in and required shared resource work.

**Change route when:** The target is a domain controller or the issue affects the wider domain. · Repair authority, local recovery access or the device identity is uncertain.

> [!NOTE]
> **Start here.** Use this guide for an [Active Directory](https://learn.microsoft.com/en-us/windows-server/identity/ad-ds/get-started/virtual-dc/active-directory-domain-services-overview) member workstation or member server. Domain-controller trust repair needs its own procedure.

<a id="decision-route"></a>

<a id="-choose-the-right-branch"></a>

## 🧭 Choose a route

```mermaid
flowchart TD
    A["Confirm membership and exact trust error"]:::action --> B{"Domain controller?"}:::decision
    B -->|"Yes"| C["Use the controller-specific procedure"]:::stop
    B -->|"No, member computer"| D{"Network, name resolution, and time healthy?"}:::decision
    D -->|"No"| E["Correct connectivity first"]:::action
    D -->|"Yes"| F["Test the secure channel"]:::action
    E --> F
    F --> G{"Trust fault confirmed?"}:::decision
    G -->|"No"| H["Investigate sign-in or resource access"]:::action
    G -->|"Yes"| I["Resolve duplicate identity and repair"]:::action
    I --> J["Restart and test fresh domain sign-in"]:::done
    H --> J
    classDef decision fill:#eaf0f8,stroke:#607fa7,color:#172b46
    classDef action fill:#f1f3f6,stroke:#8792a2,color:#263445
    classDef stop fill:#faf0f0,stroke:#a46b70,color:#5e3137
    classDef done fill:#e9edf3,stroke:#6c7d94,color:#253449
```

**Route links:** [Continue this workflow](#walkthrough) · [Acceptance checks](#acceptance) · [Fix internet, DNS, or local-network problems](network-dns.md) · [Set up a dependable new workstation](new-pc.md) · [Bring an office back after a power outage](outage-recovery.md)

| Checkpoint | What to carry forward |
| --- | --- |
| **Before changing anything** | Confirm a member computer, local fallback, and its unique identity. |
| **Keep during the job** | Connectivity, time, channel result, directory object, and repair evidence. |
| **Before handing back** | Fresh domain sign-in, policy, shares, and printers pass. |

<a id="toolkit"></a>

<a id="-build-the-right-toolkit"></a>

## Recommended tools

Select a tool for the next diagnostic or repair step. Check its platform, access and licensing requirements before use.

| Tool / official page + download | Use it for | Platform & access | Documentation |
| --- | --- | --- | --- |
| **[PowerShell](https://learn.microsoft.com/en-us/powershell/)** · [Official page](https://learn.microsoft.com/en-us/powershell/) · **[Use Windows PowerShell 5.1](https://learn.microsoft.com/en-us/powershell/scripting/windows-powershell/starting-windows-powershell)** | Use existing [Windows PowerShell 5.1](https://learn.microsoft.com/en-us/powershell/scripting/windows-powershell/starting-windows-powershell) on the member PC. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos); Free / open source | [Open guide](https://learn.microsoft.com/en-us/powershell/) |
| **[Microsoft Sysinternals Suite](https://learn.microsoft.com/en-us/sysinternals/)** · [Official page](https://learn.microsoft.com/en-us/sysinternals/) · **[Download Windows portable · ZIP](https://download.sysinternals.com/files/SysinternalsSuite.zip)** | The core portable [Windows](https://www.microsoft.com/windows) troubleshooting kit. | [Windows](https://www.microsoft.com/windows); Free Microsoft utilities | [Open guide](https://learn.microsoft.com/en-us/sysinternals/downloads/) |

<a id="walkthrough"></a>

<a id="-work-the-case"></a>

## Procedure

Work in order, record the result of each step, and change route when the evidence calls for it.

<a id="step-01"></a>

### 01 · Confirm the exact failure

- [ ] Record the trust error, hostname, domain membership, recent rename/reimage/restore, and whether another device has reused the same computer account. Use a known local administrator or approved recovery access rather than repeatedly trying cached domain credentials.

<a id="step-02"></a>

### 02 · Verify the domain path

- [ ] Confirm the device uses the intended internal DNS, can reach its domain controllers, and has correct time. Check VPN requirements for offsite devices. If connectivity is broken, fix that first; a channel test cannot establish trust across a missing network path.

<a id="step-03"></a>

### 03 · Test without repairing

- [ ] Open [Windows](https://www.microsoft.com/windows) [PowerShell](https://learn.microsoft.com/en-us/powershell/) as administrator on the member computer and run the diagnostic command. Record the result and relevant NETLOGON events. Do not apply this cmdlet to a domain controller, where the result can be misleading.

**Command for the environment described above:**

```text
Test-ComputerSecureChannel -Verbose
```

**Command legend:** [Test-ComputerSecureChannel](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.management/test-computersecurechannel?view=powershell-5.1) · [PowerShell](https://learn.microsoft.com/en-us/powershell/) · [Return to the step context](#step-03).

<a id="step-04"></a>

### 04 · Resolve conflicting computer identity

- [ ] Have the directory administrator inspect the computer object and recent changes. Give each active machine its own correct computer identity. Do not let two physical computers alternate using one cloned domain computer account, and do not delete the object until its dependencies are understood.

<a id="step-05"></a>

### 05 · Repair the confirmed secure-channel fault

- [ ] If the network is healthy and the correct computer object is established, the directory administrator can use the documented Test-ComputerSecureChannel repair flow with credentials that have the required rights. Keep credentials in the interactive prompt and plan the restart with the user.

<a id="step-06"></a>

### 06 · Validate from a fresh sign-in

- [ ] After the approved repair and restart, repeat the secure-channel test and perform a real domain sign-in. Check group policy, shares, printers, and business apps. If rejoin is needed, plan profile and local-access continuity before leaving the domain.

<a id="acceptance"></a>

<a id="-prove-the-result"></a>

## Verification

- [ ] The member computer’s secure-channel test succeeds.
- [ ] Fresh domain sign-in and required network resources work.
- [ ] No duplicate active computer identity remains.

> [!WARNING]
> **Pause this route and escalate when:**
> - The target is a domain controller.
> - DNS/time/connectivity is still broken or the computer account is ambiguous.
> - Local recovery access is not available before a membership change.

<a id="-continue-with-the-evidence"></a>

## Related playbooks

| Next playbook | What it resolves |
| --- | --- |
| [Fix internet, DNS, or local-network problems](network-dns.md) | Locate the failing layer before resetting adapters or changing the whole network. |
| [Set up a dependable new workstation](new-pc.md) | Build a usable, recoverable workstation around the employee’s actual job. |
| [Bring an office back after a power outage](outage-recovery.md) | Restore dependencies in order and verify services before releasing the office back to work. |

<a id="sources"></a>

<a id="-source-trail"></a>

## Official sources

- [Microsoft: Test-ComputerSecureChannel](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.management/test-computersecurechannel?view=powershell-5.1)
- [Microsoft: broken trust relationship](https://learn.microsoft.com/en-us/troubleshoot/windows-server/windows-security/broken-trust-relationship-domain-joined-device-its-domain-secure-channel-issues)

Reviewed **2026-11-26**. Timing is a planning range. A successful command is evidence about that operation; acceptance belongs to the user’s original task.


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)

[← All playbooks](README.md) · [Live Wiki](https://github.com/Herbertofury/INTA/wiki) · [Contribution guide](https://github.com/Herbertofury/INTA/blob/main/CONTRIBUTING.md)
