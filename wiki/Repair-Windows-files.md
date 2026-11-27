[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

# 🧰 Repair [Windows](https://www.microsoft.com/windows) files

**Confirm storage health, a recovery route and the intended [Windows](https://www.microsoft.com/windows) installation before repair writes.**

<a id="working-kit"></a>

## 🧰 Tools and links for this job

**[Open working view](https://herbertofury.github.io/INTA/#job/windows-repair/quick/windows-repair)** · Keep the tool panel beside the instructions.

**First move:** Confirm storage health, a recovery route and the intended [Windows](https://www.microsoft.com/windows) installation before repair writes.

| Use this for | Official tool + exact action |
| --- | --- |
| Repair [Windows](https://www.microsoft.com/windows) files | **[DISM](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image)** · [Official page](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image) · **[Open built-in repair instructions](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image)** |
| Verify protected files after [component-store repair](https://github.com/Herbertofury/INTA/wiki/Job-windows-repair). | **[System File Checker](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc)** · [Official page](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc) · **[Open built-in system-file checks](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc)** |
| Reinstall the current [Windows](https://www.microsoft.com/windows) version while keeping apps, files and settings, when available. | **[Windows Recovery Environment](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference)** · [Official page](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference) · **[Reinstall current Windows via Windows Update](https://support.microsoft.com/en-us/windows/deployment/install-upgrade/fix-issues-by-reinstalling-the-current-version-of-windows)** |

### Repair

| Open the exact resource | What to do there |
| --- | --- |
| **[Microsoft DISM and SFC sequence](https://support.microsoft.com/en-us/windows/experience/backup-recovery/using-system-file-checker-in-windows)** | Run the documented repair order and read the final result. |
| **[Repair an image and choose a source](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image)** | Match the installed version and correct online/offline target. |

### Recovery

| Open the exact resource | What to do there |
| --- | --- |
| **[Recovery options](https://support.microsoft.com/en-us/windows/experience/backup-recovery/recovery-options-in-windows)** | Choose a supported next step if servicing cannot repair the installation. |
| **[Repair reinstall through Windows Update](https://support.microsoft.com/en-us/windows/deployment/install-upgrade/fix-issues-by-reinstalling-the-current-version-of-windows)** | Use the supported same-version repair; managed update policies can make it unavailable. |

## ⚡ First three actions

1. Run the guide’s [DISM](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image) checks and repair against the correct online or offline installation.
2. Follow with [System File Checker](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc) and record whether corruption was repaired or remains unresolved.
3. Restart if required, repeat the original task and retain the relevant repair logs.

**[DISM](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image)** · **[Open built-in repair instructions](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image)**

**[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-windows-repair)** · **[Selection evidence](https://github.com/Herbertofury/INTA/wiki/Free-Tool-Picks)**

<a id="next-action"></a>

## 🧭 Match the symptom

| You see this | Do this next | You should get |
| --- | --- | --- |
| **[Windows](https://www.microsoft.com/windows) runs but a component or system file is corrupt** | Open Terminal or [Command Prompt](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/cmd) as administrator; run [DISM](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image).exe /Online /Cleanup-Image /RestoreHealth and wait for the final result. | The operation completes successfully or supplies a specific error for the next branch. |
| **[DISM](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image) completes successfully** | Run [sfc](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc) /scannow in that elevated online session and retain its final message. | [SFC](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc) reports no corruption, repaired corruption or an explicit unresolved result. |
| **[DISM](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image) cannot find repair files** | Record the error and OS build; follow Microsoft’s matching-source instructions instead of obtaining DLLs from third-party sites. | A compatible source is identified and the repair can complete. |
| **Repair reports success but the symptom remains** | Restart if required, repeat the original feature test and distinguish an application-only fault from recurring system corruption. | The actual failure is resolved or has an evidenced next owner. |
| **[Windows](https://www.microsoft.com/windows) starts but system/update corruption persists** | With saved work, recovery access, stable power and internet, open Settings > System > Recovery > Fix problems using [Windows Update](https://learn.microsoft.com/en-us/windows/deployment/update/) > Reinstall now if available. If managed policy hides it, use the organization’s repair route. | The same [Windows](https://www.microsoft.com/windows) version is repaired, apps/files/settings remain, and the original failed task or update succeeds after restart. |

### ✅ Finish the job

- [ ] The final [DISM](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image)/[SFC](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc) results and relevant logs are retained privately.
- [ ] The original failing feature works after the required restart.

**Change route when:** Corruption returns with I/O errors or other storage symptoms. · The installation is nonrepairable or the correct target/source is uncertain.


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
