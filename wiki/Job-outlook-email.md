[Home](https://github.com/Herbertofury/INTA/wiki) · [All playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Interactive checklist](https://herbertofury.github.io/INTA/#job/outlook-email)

# Fix Outlook or email send/receive problems

> Separate service, authentication, mailbox, network, and local-client faults.

| Job profile | Details |
| --- | --- |
| Category | Identity & business systems |
| Working time | 20–60 min |
| Experience | Intermediate |

## Start here

Identify the mail provider and whether the user is in classic Outlook, new Outlook, or a browser. Their repair paths differ.

## Your toolkit

| Tool | Why it belongs here |
| --- | --- |
| [Microsoft Sysinternals Suite](https://learn.microsoft.com/en-us/sysinternals/downloads/sysinternals-suite) | The core portable Windows troubleshooting kit. |

## Follow the workflow

### 01. Locate the failing layer

- [ ] Check the provider’s service status and test the same mailbox in its official webmail. Record the exact send/receive error, whether one or all accounts are affected, mailbox quota, and a specific message that fails. Do not expose credentials or message contents in public notes.

### 02. Verify identity and addressing

- [ ] Confirm the account is signed in through the intended provider and that password/MFA changes have completed. For manually configured mail, compare SMTP/IMAP hostnames, ports, and encryption with the provider’s current settings. An internal directory domain is not automatically the outbound email domain.

### 03. Check the message and network path

- [ ] Inspect Outbox for a stuck or oversized message and verify whether attachment size, recipient address, or a provider rejection explains the failure. Compare a small test message through webmail. Check VPN/proxy and DNS only when the symptom points there.

### 04. Use the correct client-specific repair

- [ ] For classic Outlook, a safe-mode launch can isolate add-in problems; disable only the implicated add-in after testing. New Outlook uses a different architecture and does not use the classic safe-mode/profile repair workflow. Follow the current support path for the actual client.

### 05. Preserve local-only data before rebuilding

- [ ] Before removing an account, recreating a classic profile, or repairing data files, identify local PST archives, POP mail, unsent items, and anything not present on the server. Make a verified backup and retain the old profile until the replacement is proven.

### 06. Prove both directions

- [ ] Send a small message to a test recipient and receive a reply, then test the user’s normal attachment and shared-mailbox workflow. Record whether the final fix was service-side, account configuration, add-in, profile, or network, and remove only temporary diagnostic changes.

## Verify before handoff

- [ ] A sent message arrives and a reply returns.
- [ ] Local-only mail and archives remain accessible.
- [ ] The normal attachment/shared-mailbox workflow passes.

## Stop and escalate when

- A proposed profile or account deletion would remove unbacked-up local mail.
- There are signs of account takeover, unexpected rules, or malicious forwarding.

## Related playbooks

- [Network Dns](https://github.com/Herbertofury/INTA/wiki/Job-network-dns)
- [Malware Removal](https://github.com/Herbertofury/INTA/wiki/Job-malware-removal)
- [Backup Restore](https://github.com/Herbertofury/INTA/wiki/Job-backup-restore)

## Official references

- [Microsoft: classic Outlook not responding](https://support.microsoft.com/en-us/outlook/classic-outlook-not-responding-stuck-at-processing-stopped-working-or-freezes)
- [Microsoft: architecture changes in new Outlook](https://learn.microsoft.com/en-us/microsoft-365-apps/outlook/administration/architecture-changes-new-outlook)

---

INTA Computer Services · Editorial review: 2026-11-26. Timing is an estimate; stop conditions take priority.
