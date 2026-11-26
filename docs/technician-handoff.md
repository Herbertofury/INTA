# Technician intake and handoff

> **INTA service record template** · Copy into the approved private ticket system before entering customer information.

This template keeps the next action, the data-protection plan, and the proof of repair together. Leave passwords, recovery keys, access tokens, and other secrets in the approved credential store; record a reference to that store instead of copying the secret here.

## 1. Intake

| Field | Case entry |
| --- | --- |
| Ticket and technician | |
| Date, local time, and timezone | |
| Customer contact and preferred contact method | |
| Device / asset reference | |
| OS edition and build | |
| Exact failing task | |
| Expected result / observed result | |
| First occurrence / last known good state | |
| Recent software, account, hardware, or network changes | |
| Business impact and affected users | |
| Maintenance window and interruption constraints | |
| Recovery contact or on-site fallback | |

### Initial decision

- [ ] The user demonstrated the symptom or supplied the exact error.
- [ ] Scope is recorded: one app, one user, one device, a site, or a wider service.
- [ ] Active compromise, unsafe hardware, and failing storage were considered first.
- [ ] A specific job guide and success test have been selected.

## 2. Protect the data and recovery path

| Field | Case entry |
| --- | --- |
| Important or unique data that could be affected | |
| Last verified backup / restore evidence | |
| Encryption and recovery-key reference | |
| Source and destination device identities, if copying storage | |
| Configuration export or rollback method | |
| Required restart or loss of remote access | |

- [ ] The next action’s target is confirmed.
- [ ] A usable rollback or recovery route exists for the planned change.
- [ ] Any required user or service-owner coordination is recorded.

## 3. Evidence and working hypothesis

**Observed facts:**

**Current hypothesis and confidence:**

**Smallest next test that can distinguish likely causes:**

**Private evidence locations:**

Record only the evidence needed for this case. Logs and dumps may contain usernames, file contents, tokens, or other sensitive information. Keep them in the case’s approved private location.

## 4. Change and test log

| Time | Observation or exact change | Why this step | Result / evidence | Rollback |
| --- | --- | --- | --- | --- |
| | | | | |
| | | | | |
| | | | | |

**Failed routes not to repeat without new evidence:**

**Current configuration/version after the last change:**

## 5. Acceptance

- [ ] The original task works with representative customer data.
- [ ] The device restarts and signs in normally, if affected.
- [ ] Network, email, printing, and required peripherals work, if affected.
- [ ] The intended security product is healthy.
- [ ] Backup operation and a suitable restore check are recorded.
- [ ] Restored or migrated data has been opened in the intended applications.
- [ ] Temporary access, logging, test accounts, and files are resolved.
- [ ] The customer understands the result and any remaining work.

**Measured or observed before/after result:**

**Acceptance performed by / date:**

## 6. Handoff to the customer

**Problem:**

**Cause, distinguishing confirmed evidence from remaining uncertainty:**

**What changed:**

**What we tested:**

**What you need to do next:**

**How to contact INTA if the symptom returns:**

## 7. Continuation for the next technician

| Remaining item | Owner | Exact next action | Evidence or dependency | Target date |
| --- | --- | --- | --- | --- |
| | | | | |

**Current blocker:**

**Latest working checkpoint:**

**Rollback or retained source location:**

**Next action, written so another technician can perform it without rediscovery:**

---

[All field guides](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Start with triage](https://github.com/Herbertofury/INTA/wiki/Job-intake-triage) · [Client handoff guide](https://github.com/Herbertofury/INTA/wiki/Job-client-handoff)
