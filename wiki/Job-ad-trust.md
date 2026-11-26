[Home](https://github.com/Herbertofury/INTA/wiki) · [All playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Interactive checklist](https://herbertofury.github.io/INTA/#job/ad-trust)

# Repair a workstation’s domain trust relationship

> Check connectivity, time, and computer identity before repairing a member computer’s secure channel.

| Job profile | Details |
| --- | --- |
| Category | Identity & business systems |
| Working time | 20–60 min |
| Experience | Advanced |

## Start here

Use this guide for an Active Directory member workstation or member server. Domain-controller trust repair needs its own procedure.

## Your toolkit

| Tool | Why it belongs here |
| --- | --- |
| [Microsoft Sysinternals Suite](https://learn.microsoft.com/en-us/sysinternals/downloads/sysinternals-suite) | The core portable Windows troubleshooting kit. |

## Follow the workflow

### 01. Confirm the exact failure

- [ ] Record the trust error, hostname, domain membership, recent rename/reimage/restore, and whether another device has reused the same computer account. Use a known local administrator or approved recovery access rather than repeatedly trying cached domain credentials.

### 02. Verify the domain path

- [ ] Confirm the device uses the intended internal DNS, can reach its domain controllers, and has correct time. Check VPN requirements for offsite devices. If connectivity is broken, fix that first; a channel test cannot establish trust across a missing network path.

### 03. Test without repairing

- [ ] Open Windows PowerShell as administrator on the member computer and run the diagnostic command. Record the result and relevant NETLOGON events. Do not apply this cmdlet to a domain controller, where the result can be misleading.

```text
Test-ComputerSecureChannel -Verbose
```

### 04. Resolve conflicting computer identity

- [ ] Have the directory administrator inspect the computer object and recent changes. Give each active machine its own correct computer identity. Do not let two physical computers alternate using one cloned domain computer account, and do not delete the object until its dependencies are understood.

### 05. Repair the confirmed secure-channel fault

- [ ] If the network is healthy and the correct computer object is established, the directory administrator can use the documented Test-ComputerSecureChannel repair flow with credentials that have the required rights. Keep credentials in the interactive prompt and plan the restart with the user.

### 06. Validate from a fresh sign-in

- [ ] After the approved repair and restart, repeat the secure-channel test and perform a real domain sign-in. Check group policy, shares, printers, and business apps. If rejoin is needed, plan profile and local-access continuity before leaving the domain.

## Verify before handoff

- [ ] The member computer’s secure-channel test succeeds.
- [ ] Fresh domain sign-in and required network resources work.
- [ ] No duplicate active computer identity remains.

## Stop and escalate when

- The target is a domain controller.
- DNS/time/connectivity is still broken or the computer account is ambiguous.
- Local recovery access is not available before a membership change.

## Related playbooks

- [Network Dns](https://github.com/Herbertofury/INTA/wiki/Job-network-dns)
- [New Pc](https://github.com/Herbertofury/INTA/wiki/Job-new-pc)
- [Outage Recovery](https://github.com/Herbertofury/INTA/wiki/Job-outage-recovery)

## Official references

- [Microsoft: Test-ComputerSecureChannel](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.management/test-computersecurechannel?view=powershell-5.1)
- [Microsoft: broken trust relationship](https://learn.microsoft.com/en-us/troubleshoot/windows-server/windows-security/broken-trust-relationship-domain-joined-device-its-domain-secure-channel-issues)

---

INTA Computer Services · Editorial review: 2026-11-26. Timing is an estimate; stop conditions take priority.
