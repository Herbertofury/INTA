[Home](https://github.com/Herbertofury/INTA/wiki) · [All playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Interactive checklist](https://herbertofury.github.io/INTA/#job/outage-recovery)

# Bring an office back after a power outage

> Restore dependencies in order and verify services before releasing the office back to work.

| Job profile | Details |
| --- | --- |
| Category | Backup & continuity |
| Working time | 30–120 min; infrastructure varies |
| Experience | Advanced |

## Start here

Confirm power is stable and establish which devices and services are down. Use the site’s private dependency diagram and recovery plan.

## Your toolkit

| Tool | Why it belongs here |
| --- | --- |
| [smartmontools](https://github.com/smartmontools/smartmontools/releases) | Reading SMART and NVMe health from a rescue environment or supported host OS. |
| [Microsoft Sysinternals Suite](https://learn.microsoft.com/en-us/sysinternals/downloads/sysinternals-suite) | The core portable Windows troubleshooting kit. |

## Follow the workflow

### 01. Check the physical environment

- [ ] Inspect UPS status, breakers, power strips, equipment alarms, and visible damage. Keep equipment with electrical damage, liquid exposure, or repeated power cycling out of service. Record the approximate outage and shutdown times from available logs.

### 02. Restore the network foundation

- [ ] Bring up the internet handoff, firewall/router, switches, and wireless infrastructure in the site’s documented dependency order. Wait for each to reach a healthy state, then verify local link, addressing, routing, and name resolution before starting application tests.

### 03. Recover storage and core services

- [ ] Check storage controllers and arrays for degraded state before normal workload resumes. Bring up required hypervisors, storage, identity/DNS/DHCP, and dependent services in their actual site order. Avoid repeated hard resets of a server performing recovery or consistency work.

### 04. Validate applications from one workstation

- [ ] Use one known-good client to test domain sign-in, shared folders, printing, email, and each critical line-of-business application. An open server console or a successful ping does not prove the business service is healthy.

### 05. Review shutdown and recovery evidence

- [ ] Inspect UPS logs and relevant event logs. Kernel-Power Event ID 41 indicates Windows did not shut down cleanly; it does not by itself identify the power supply or the original cause. Correlate the event with other evidence before replacing parts.

### 06. Resume operations and verify backups

- [ ] Release workstations in a controlled order, confirm time synchronization and remote management, and check that backup jobs resume. Record any disk repairs, degraded components, interrupted transactions, and remaining follow-up with the designated service owner.

## Verify before handoff

- [ ] Network, identity, storage, and business applications pass end-to-end tests.
- [ ] No degraded storage or critical recovery alarm is left unassigned.
- [ ] Backups and remote access have resumed.
- [ ] Users can complete their actual work from a client workstation.

## Stop and escalate when

- Power is unstable or equipment shows physical/electrical damage.
- Storage is degraded, rebuilding, or producing new errors without an approved recovery plan.

## Related playbooks

- [Network Dns](https://github.com/Herbertofury/INTA/wiki/Job-network-dns)
- [Hardware Bsod](https://github.com/Herbertofury/INTA/wiki/Job-hardware-bsod)
- [Backup Restore](https://github.com/Herbertofury/INTA/wiki/Job-backup-restore)
- [Client Handoff](https://github.com/Herbertofury/INTA/wiki/Job-client-handoff)

## Official references

- [Microsoft: interpreting Kernel-Power Event ID 41](https://learn.microsoft.com/en-us/troubleshoot/windows-client/performance/event-id-41-restart)
- [restic: repository consistency and integrity](https://restic.readthedocs.io/en/stable/045_working_with_repos.html)

---

INTA Computer Services · Editorial review: 2026-11-26. Timing is an estimate; stop conditions take priority.
