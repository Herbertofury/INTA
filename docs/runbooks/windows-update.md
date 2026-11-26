[Home](https://github.com/Herbertofury/INTA/wiki) · [All playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Interactive checklist](https://herbertofury.github.io/INTA/#job/windows-update)

# Fix a failing Windows update

> Diagnose the update code, servicing state, and available space before broad reset scripts.

| Job profile | Details |
| --- | --- |
| Category | Windows & performance |
| Working time | 20–60 min plus installation |
| Experience | Intermediate |

## Start here

Write down the exact KB or feature update, error code, Windows edition/build, and whether the device is centrally managed.

## Your toolkit

| Tool | Why it belongs here |
| --- | --- |
| [Microsoft Sysinternals Suite](https://learn.microsoft.com/en-us/sysinternals/downloads/sysinternals-suite) | The core portable Windows troubleshooting kit. |

## Follow the workflow

### 01. Confirm the failure and update policy

- [ ] Read Settings > Windows Update > Update history. Determine whether the failure is download, installation, or reboot rollback. On managed devices, check the assigned update policy and maintenance window before overriding settings.

### 02. Check the simple prerequisites

- [ ] Verify stable power, free space, correct time, functioning network/proxy, and whether a restart is pending. Compare with another device on the same network to distinguish local servicing from a broader service issue.

### 03. Run the supported troubleshooter

- [ ] Open Get Help and run the Windows Update troubleshooter. Capture its findings and restart when appropriate, then retry the specific update once. If the same failure persists, use the new evidence rather than looping identical retries.

### 04. Follow the exact error path

- [ ] Use Microsoft’s troubleshooting reference for the captured code. Inspect relevant logs and service state. A cache reset or security-product change should be tied to a specific diagnosis and have a recovery plan; avoid one-click scripts that reset unrelated settings.

### 05. Repair servicing if indicated

- [ ] For component-store or protected-file errors on a healthy drive, follow the Windows repair guide. If install rollback names a driver, check the device manufacturer’s supported update or rollback path instead of layering more repair commands.

### 06. Validate the installed result

- [ ] Confirm the intended update appears in history and the resulting OS build is correct. Test the user’s business applications, network, printing, and restart. Record any deferred update with its cause, owner, and next review.

## Verify before handoff

- [ ] The target update is installed or the remaining blocker is explicitly documented.
- [ ] Update history and the OS build agree.
- [ ] The original applications and devices work after reboot.

## Stop and escalate when

- The device is in an organization-managed change freeze.
- Repeated rollback or storage symptoms point to a deeper problem.

## Related playbooks

- [Windows Repair](windows-repair.md)
- [Hardware Bsod](hardware-bsod.md)
- [Preventive Maintenance](preventive-maintenance.md)

## Official references

- [Microsoft: Windows Update troubleshooter](https://support.microsoft.com/en-us/windows/experience/windows-update-troubleshooter)
- [Microsoft: troubleshooting Windows updates](https://support.microsoft.com/en-us/windows/deployment/updates-lifecycle/troubleshoot-problems-updating-windows)

---

INTA Computer Services · Editorial review: 2026-11-26. Timing is an estimate; stop conditions take priority.
