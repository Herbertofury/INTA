[Home](https://github.com/Herbertofury/INTA/wiki) · [All playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Interactive checklist](https://herbertofury.github.io/INTA/#job/windows-repair)

# Repair Windows component or system-file corruption

> Use Windows servicing tools in a sensible order and validate the specific broken feature.

| Job profile | Details |
| --- | --- |
| Category | Windows & performance |
| Working time | 30–90 min |
| Experience | Intermediate |

## Start here

Confirm the drive is stable and the affected data is backed up. Repeated corruption or I/O errors need a hardware investigation first.

## Your toolkit

| Tool | Why it belongs here |
| --- | --- |
| [DISM](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image) | Component-store corruption and controlled image servicing. |
| [System File Checker](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc) | System-file corruption after establishing disk health and servicing status. |
| [Windows Recovery Environment](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference) | A Windows machine that will not boot or needs supported recovery options. |

## Follow the workflow

### 01. Record the exact failure

- [ ] Capture the app or Windows feature that fails, error code, OS build, recent changes, and available disk space. Save relevant event entries before repair. Confirm this is a Windows component problem rather than an application-only fault.

### 02. Prepare the online Windows session

- [ ] Save work, finish a pending restart if practical, connect stable power, and open Terminal or Command Prompt as administrator. These commands target the currently running Windows installation. Do not paste them into WinRE expecting them to repair an offline Windows volume.

### 03. Repair the component store

- [ ] Run DISM and wait for its final result. If the repair source cannot be found, retain the error code and use Microsoft’s matching-source guidance; do not download replacement DLLs from a third-party site.

```text
DISM.exe /Online /Cleanup-Image /RestoreHealth
```

### 04. Check protected system files

- [ ] After DISM completes successfully, run System File Checker. Record whether corruption was absent, repaired, or unresolved. Keep the relevant CBS and DISM log excerpts privately if escalation is needed.

```text
sfc /scannow
```

### 05. Reboot and retest the original feature

- [ ] Reproduce the exact failure from intake. Check Windows Update and Reliability Monitor for new errors. A successful command exit is evidence about servicing, not proof that the user’s problem is solved.

### 06. Escalate from the remaining evidence

- [ ] If a stable system still has unresolved servicing problems, evaluate a supported repair reinstall that preserves apps and files. Confirm backup, edition, language, recovery keys, available space, and the actual preservation options before proceeding. Repeated corruption returns to hardware diagnostics.

## Verify before handoff

- [ ] DISM and SFC outcomes are recorded.
- [ ] The originally broken Windows function works after restart.
- [ ] No new storage or recurring corruption evidence is present.

## Stop and escalate when

- Drive failure, unstable RAM, or overheating is suspected.
- The proposed reinstall does not clearly preserve the required data and applications.

## Related playbooks

- [Windows Update](windows-update.md)
- [Boot Recovery](boot-recovery.md)
- [Hardware Bsod](hardware-bsod.md)
- [Backup Restore](backup-restore.md)

## Official references

- [Microsoft: using System File Checker](https://support.microsoft.com/en-us/windows/experience/backup-recovery/using-system-file-checker-in-windows)
- [Microsoft: recovery options in Windows](https://support.microsoft.com/en-us/windows/experience/backup-recovery/recovery-options-in-windows)

---

INTA Computer Services · Editorial review: 2026-11-26. Timing is an estimate; stop conditions take priority.
