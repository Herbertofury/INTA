[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Service map](https://github.com/Herbertofury/INTA/wiki/Service-Map) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

# 🔐 Repair domain sign-in

**On the affected domain-member PC, check the domain DNS, time and domain-controller reachability first.**

<a id="tool-choices"></a>

## 🧰 Choose the tool for this job

Choose by the task shown in the first column. Each option includes its acquisition route and access boundary.

| Need | Official tool + exact action | Why this choice / access |
| --- | --- | --- |
| **Member secure-channel test** | **[PowerShell](https://learn.microsoft.com/en-us/powershell/)** · **[Use Windows PowerShell 5.1](https://learn.microsoft.com/en-us/powershell/scripting/windows-powershell/starting-windows-powershell)** | Use existing [Windows PowerShell 5.1](https://learn.microsoft.com/en-us/powershell/scripting/windows-powershell/starting-windows-powershell) on the domain member after DNS, time and connectivity checks. **Access:** Included with supported [Windows](https://www.microsoft.com/windows) |

**[Current selection evidence](https://github.com/Herbertofury/INTA/wiki/Current-Tool-Choices)**

<a id="working-kit"></a>

## 🧰 Tools and links for this job

**[Open working view](https://herbertofury.github.io/INTA/#job/ad-trust/quick/repair-domain-trust)** · Keep the tool panel beside the instructions.

**First move:** On the affected domain-member PC, check the domain DNS, time and domain-controller reachability first.

| Use this for | Official tool + exact action |
| --- | --- |
| Repair domain sign-in | **[PowerShell](https://learn.microsoft.com/en-us/powershell/)** · [Official page](https://learn.microsoft.com/en-us/powershell/) · **[Use Windows PowerShell 5.1](https://learn.microsoft.com/en-us/powershell/scripting/windows-powershell/starting-windows-powershell)** |
| Establishing recoverable, per-device administrator access during managed workstation setup and support planning. | **[Windows LAPS](https://learn.microsoft.com/en-us/windows-server/identity/laps/laps-overview)** · [Official page](https://learn.microsoft.com/en-us/windows-server/identity/laps/laps-overview) · **[Configure Active Directory backup](https://learn.microsoft.com/en-us/windows-server/identity/laps/laps-scenarios-windows-server-active-directory)** |

### Diagnostic and repair

| Open the exact resource | What to do there |
| --- | --- |
| **[Test-ComputerSecureChannel](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.management/test-computersecurechannel?view=powershell-5.1)** | Read-only test first; repair only a confirmed member-computer fault. |
| **[Microsoft trust-relationship troubleshooting](https://learn.microsoft.com/en-us/troubleshoot/windows-server/windows-security/broken-trust-relationship-domain-joined-device-its-domain-secure-channel-issues)** | Check connectivity, identity and the cause before rejoining. |

### Read-only diagnostics

| Open the exact resource | What to do there |
| --- | --- |
| **[Read the member PC network configuration](https://learn.microsoft.com/en-us/powershell/module/nettcpip/get-netipconfiguration)** | Confirm the intended domain DNS path. |

## ⚡ First three actions

1. Use existing [Windows PowerShell 5.1](https://learn.microsoft.com/en-us/powershell/scripting/windows-powershell/starting-windows-powershell) to run the guide’s diagnostic secure-channel check on the member PC.
2. Repair only a confirmed trust fault using an authorized account and the scoped procedure.
3. Verify domain sign-in, a required shared resource and the secure-channel result.

**[PowerShell](https://learn.microsoft.com/en-us/powershell/)** · **[Use Windows PowerShell 5.1](https://learn.microsoft.com/en-us/powershell/scripting/windows-powershell/starting-windows-powershell)**

**[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-ad-trust)** · **[Specific fixes for this job](https://herbertofury.github.io/INTA/#job/ad-trust/quick/repair-domain-trust?section=fixes)** · **[Selection evidence](https://github.com/Herbertofury/INTA/wiki/Free-Tool-Picks)**

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


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
