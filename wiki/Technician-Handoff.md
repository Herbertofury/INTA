<a href="https://herbertofury.github.io/INTA/" title="Open INTA official download or service page" aria-label="Open INTA official download or service page" data-inta-brand="app-inta"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA official page" width="18" height="18" valign="middle" /></a> [INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Service map](https://github.com/Herbertofury/INTA/wiki/Service-Map) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

# 🤝 A service record the next technician can use

[Home](https://github.com/Herbertofury/INTA/wiki) › **Technician handoff**

> **Copy this template into the approved private ticket system before entering customer information.** Keep passwords, recovery keys and tokens in the approved credential store; record a reference here.

This record keeps the problem, protected data, exact change and evidence of repair together. Use [structured intake and triage](https://github.com/Herbertofury/INTA/wiki/Job-intake-triage) at arrival and [verified client handoff](https://github.com/Herbertofury/INTA/wiki/Job-client-handoff) at closure.

## 🧭 1 · Understand the job

| Field | Case entry |
| --- | --- |
| Ticket and technician | |
| Date, local time and timezone | |
| Customer contact and preferred contact method | |
| Device / asset reference | |
| Operating-system edition and build | |
| Exact failing task | |
| Expected result / observed result | |
| First occurrence / last known good state | |
| Recent software, account, hardware or network changes | |
| Business impact and affected users | |
| Maintenance window and interruption constraints | |
| Recovery contact or on-site fallback | |

- [ ] The symptom was demonstrated or the exact error supplied.
- [ ] Affected scope is recorded: application, user, device, site or service.
- [ ] Active compromise, unsafe hardware and failing storage were considered first.
- [ ] The selected guide and measurable success test match the observed evidence.

**Selected job:**

**Agreed success test:**

## 🛟 2 · Preserve data and a way back

| Field | Case entry |
| --- | --- |
| Important or unique data that could be affected | |
| Last verified [backup and restore](https://github.com/Herbertofury/INTA/wiki/Job-backup-restore) evidence | |
| Encryption and recovery-key reference | |
| Source and destination storage identities, if copying | |
| Configuration export or rollback method | |
| Required restart or possible loss of remote access | |
| Agreed recovery contact and fallback access | |

- [ ] The next action's target is confirmed.
- [ ] A usable recovery or rollback route exists for the planned change.
- [ ] Required service-owner coordination is recorded.

Useful methods: [Source and destination verification](https://github.com/Herbertofury/INTA/wiki/Job-disk-migration), [restore verification](https://github.com/Herbertofury/INTA/wiki/Job-backup-restore), <a href="https://learn.microsoft.com/en-us/windows/security/operating-system-security/data-protection/bitlocker/" title="Open BitLocker official download or service page" aria-label="Open BitLocker official download or service page" data-inta-brand="app-bitlocker"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="BitLocker official page" width="18" height="18" valign="middle" /></a> [BitLocker recovery readiness](https://github.com/Herbertofury/INTA/wiki/Job-boot-recovery), and [rollback rehearsal](https://github.com/Herbertofury/INTA/wiki/Job-new-pc).

## 🔎 3 · Keep facts separate from the hypothesis

**Observed facts, with relevant time and scope:**

**Current hypothesis and confidence:**

**Smallest next test that distinguishes likely causes:**

**Expected observation if the hypothesis is correct:**

**Private evidence locations and coverage limits:**

Use [read-only evidence collection](https://github.com/Herbertofury/INTA/wiki/Job-intake-triage) and [timestamp correlation](https://github.com/Herbertofury/INTA/wiki/Job-hardware-bsod) where useful. Record the evidence needed for the case. Logs and dumps may contain private data; preserve them in the approved private location.

## 🛠 4 · Record each meaningful change and test

| Time | Observation or exact change | Why this step | Result / evidence | Rollback |
| --- | --- | --- | --- | --- |
| | | | | |
| | | | | |
| | | | | |

**Failed routes not to repeat without new evidence:**

**Current configuration and exact version after the last change:**

**Latest working checkpoint:**

A [single-change experiment](https://github.com/Herbertofury/INTA/wiki/Job-slow-pc) is easier to interpret when the prediction and actual result are both retained. If several changes were necessary, identify their order and the evidence each contributed.

## 🧪 5 · Prove the result

- [ ] The original task works with representative customer data.
- [ ] Restart and fresh sign-in work where affected.
- [ ] Network, mail, printing and required peripherals work where affected.
- [ ] The intended security product is healthy.
- [ ] Backup operation and a suitable restore check are recorded.
- [ ] Restored or migrated data opens in the intended applications.
- [ ] Temporary access, logging, test accounts and files are resolved.
- [ ] The customer understands the result and remaining work.

| Acceptance item | Actual observation | Evidence reference | Performed by / date |
| --- | --- | --- | --- |
| Original task | | | |
| Relevant neighboring functions | | | |
| Restart or reconnection | | | |
| Recovery or backup | | | |

**Measured or observed before/after result:**

**Unverified limit, unresolved item or deferred decision:**

Use [post-change acceptance](https://github.com/Herbertofury/INTA/wiki/Job-client-handoff). Keep an unchecked item open rather than interpreting a successful command as complete proof.

## 💬 6 · Explain the outcome to the customer

**Problem:**

**Confirmed cause, with remaining uncertainty identified:**

**What changed:**

**What we tested:**

**What you need to do next:**

**How to contact <a href="https://herbertofury.github.io/INTA/" title="Open INTA official download or service page" aria-label="Open INTA official download or service page" data-inta-brand="app-inta"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA official page" width="18" height="18" valign="middle" /></a> [INTA](https://herbertofury.github.io/INTA/) if the symptom returns:**

Keep the explanation short enough to use and specific enough to distinguish the fix from a guess. Link any reusable instructions to the relevant [field guide](https://github.com/Herbertofury/INTA/wiki/Playbooks); retain customer-specific details in the private case.

## ➜ 7 · Leave an executable continuation

| Remaining item | Owner | Exact next action | Evidence or dependency | Target date |
| --- | --- | --- | --- | --- |
| | | | | |

**Current blocker:**

**Latest verified checkpoint and retained source location:**

**Running job identity, if work is still in progress:**

**Recovery or rollback location:**

**Rejected routes that should not be repeated unchanged:**

**Literal next action another technician can perform without rediscovery:**

## 🌱 8 · Turn a useful lesson into shared knowledge

**Reusable lesson, sanitized of customer details:**

**Guide, tool or named method that needs improvement:**

**Official source or generic reproduction that supports the change:**

Use [Contribute](https://github.com/Herbertofury/INTA/wiki/Contribute) and [the contribution template](https://github.com/Herbertofury/INTA/wiki/Templates) to improve the public guidance. Keep the private service record as the evidence owner.

**➜ Continue:** [Decision Desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Start with triage](https://github.com/Herbertofury/INTA/wiki/Job-intake-triage) · [Client closure guide](https://github.com/Herbertofury/INTA/wiki/Job-client-handoff)

---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
