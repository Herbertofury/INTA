# INTA Computer Services Wiki

> The field manual for diagnosing, repairing, securing, recovering and maintaining computers.

**[PowerShell cookbook](powershell-cookbook.md) · [Tool selector](tool-selection.md) · [Security response](security-response.md) · [Remote support](remote-support.md)**

## Choose the job

| Job | Start here | Typical tools |
|---|---|---|
| Slow or unstable PC | [Performance](field-guides/performance.md) | Task Manager, Resource Monitor, Reliability Monitor |
| Hardware fault | [Hardware](field-guides/hardware.md) | HWiNFO, CrystalDiskInfo, MemTest86, vendor diagnostics |
| Malware / security alert | [Security response](security-response.md) | Microsoft Defender, Defender Offline, MSERT, managed EDR |
| Windows corruption | [Windows repair](field-guides/windows-repair.md) | DISM, SFC, Windows Recovery Environment |
| Deleted files / failing disk | [Data recovery](field-guides/data-recovery.md) | ddrescue, Clonezilla, TestDisk, PhotoRec |
| Wi-Fi / LAN / DNS / VPN | [Network](field-guides/network.md) | ipconfig, ping, Resolve-DnsName, Wireshark |
| Remote user support | [Remote support](remote-support.md) | Quick Assist, approved RMM, RustDesk |
| Patch and maintain | [Maintenance](field-guides/maintenance.md) | Windows Update, WinGet, PowerShell |
| New PC / handover | [Deployment](field-guides/deployment.md) | Autopilot/Intune where licensed, approved image |

## Standard INTA job workflow

1. **Authorize:** verify requester, device ownership, scope, consent and change approval.
2. **Protect:** check backups and business impact. For suspected compromise, follow the security incident plan before cleanup.
3. **Identify:** record OS/build, symptom, error, timestamp, recent changes and reproduction steps.
4. **Baseline:** collect read-only diagnostics before making changes.
5. **Choose:** use the least invasive supported tool that answers the question.
6. **Change:** explain impact, obtain approval and make one logical change at a time.
7. **Verify:** repeat the original test and confirm the user's workflow.
8. **Document:** record tool/version, commands, results, changes, evidence location and follow-up owner.

## Principles

- Read-only first; destructive operations require explicit authorization and a verified recovery plan.
- Preserve evidence in security incidents; do not clear logs or delete suspicious files before the incident lead decides.
- Never request passwords, MFA codes, BitLocker keys or private keys in tickets or chat.
- Use company-approved remote access, MFA, consent, RBAC and audit logging. Never expose RDP directly to the internet.
- Download from canonical vendor/project sources; verify signatures and hashes when available.
- A successful command exit code is not proof the user's problem is fixed.
- Do not upload customer files/logs to public analysis services without approval.

## Tool recommendation labels

**Preferred** = organization-approved and supported. **Alternative** = reputable option with trade-offs. **Specialist** = trained-technician tool. **Evaluate** = requires security/licensing/privacy review. **Avoid** = unsupported or unsafe.

## Contribution standard

Every procedure should state owner, last verified date, supported OS/build, tool/version, prerequisites, rollback, verification and official sources. Review security-sensitive guidance quarterly and after vendor changes. Never commit customer names, serials, IPs, credentials, tokens, logs or screenshots containing private data.

This wiki is guidance, not authorization to access devices, bypass controls or make destructive changes. Follow company policy, customer consent and applicable law.
