[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Service map](https://github.com/Herbertofury/INTA/wiki/Service-Map) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

# 🔄 Fix [Windows](https://www.microsoft.com/windows) updates

**Record the failing update or KB number, error code and pending restart before retrying it.**

<a id="working-kit"></a>

## 🧰 Tools and links for this job

**[Open working view](https://herbertofury.github.io/INTA/#job/windows-update/quick/fix-windows-updates)** · Keep the tool panel beside the instructions.

**First move:** Record the failing update or KB number, error code and pending restart before retrying it.

| Use this for | Official tool + exact action |
| --- | --- |
| Fix [Windows](https://www.microsoft.com/windows) updates | **[Windows Get Help](https://support.microsoft.com/en-us/support/get-help/about-get-help)** · [Official page](https://support.microsoft.com/en-us/support/get-help/about-get-help) · **[Open Windows Update troubleshooting](https://support.microsoft.com/en-us/windows/experience/windows-update-troubleshooter)** |
| Investigate component-store damage when the error points there. | **[DISM](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image)** · [Official page](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image) · **[Open built-in repair instructions](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image)** |
| Check protected files after successful servicing repair. | **[System File Checker](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc)** · [Official page](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc) · **[Open built-in system-file checks](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc)** |
| Reinstall the current [Windows](https://www.microsoft.com/windows) version while keeping apps, files and settings, when available. | **[Windows Recovery Environment](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference)** · [Official page](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference) · **[Reinstall current Windows via Windows Update](https://support.microsoft.com/en-us/windows/deployment/install-upgrade/fix-issues-by-reinstalling-the-current-version-of-windows)** |
| Finding the failure rule and related log evidence before retrying an unsuccessful [Windows](https://www.microsoft.com/windows) upgrade. | **[SetupDiag](https://learn.microsoft.com/en-us/windows/deployment/upgrade/setupdiag)** · [Official page](https://learn.microsoft.com/en-us/windows/deployment/upgrade/setupdiag) · **[Download Windows upgrade analyzer · EXE](https://go.microsoft.com/fwlink/?linkid=870142)** |

### First repair

| Open the exact resource | What to do there |
| --- | --- |
| **[Windows Update troubleshooter](https://support.microsoft.com/en-us/windows/experience/windows-update-troubleshooter)** | Run the diagnostic, restart when required and check updates again. |
| **[Troubleshoot update failures](https://support.microsoft.com/en-us/windows/deployment/updates-lifecycle/troubleshoot-problems-updating-windows)** | Use the exact error and installation stage. |

### Current issues

| Open the exact resource | What to do there |
| --- | --- |
| **[Windows release health](https://learn.microsoft.com/en-us/windows/release-health/)** | Check the affected [Windows](https://www.microsoft.com/windows) version for known issues or holds. |

### Recovery

| Open the exact resource | What to do there |
| --- | --- |
| **[Recovery and uninstall choices](https://support.microsoft.com/en-us/windows/experience/backup-recovery/recovery-options-in-windows)** | Use the appropriate rollback route if the update prevents startup. |
| **[Repair reinstall through Windows Update](https://support.microsoft.com/en-us/windows/deployment/install-upgrade/fix-issues-by-reinstalling-the-current-version-of-windows)** | Use the supported same-version repair; managed update policies can make it unavailable. |

## ⚡ First three actions

1. Confirm free space, a stable connection and a planned restart window.
2. Run the [Windows Update](https://learn.microsoft.com/en-us/windows/deployment/update/) troubleshooter in [Windows Get Help](https://support.microsoft.com/en-us/support/get-help/about-get-help) and review its findings.
3. Restart when required, check for updates again and verify the affected update and original task.

**[Windows Get Help](https://support.microsoft.com/en-us/support/get-help/about-get-help)** · **[Open Windows Update troubleshooting](https://support.microsoft.com/en-us/windows/experience/windows-update-troubleshooter)**

**[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-windows-update)** · **[Specific fixes for this job](https://herbertofury.github.io/INTA/#job/windows-update/quick/fix-windows-updates?section=fixes)** · **[Selection evidence](https://github.com/Herbertofury/INTA/wiki/Free-Tool-Picks)**

<a id="next-action"></a>

## 🧭 Match the symptom

| You see this | Do this next | You should get |
| --- | --- | --- |
| **One update repeatedly fails** | Record its KB/error, check free space and connection, then run the [Windows Update](https://learn.microsoft.com/en-us/windows/deployment/update/) troubleshooter in [Windows Get Help](https://support.microsoft.com/en-us/support/get-help/about-get-help). | The diagnostic gives a finding or a reproducible update error. |
| **A restart is pending** | Save work and restart in the agreed window, then return to [Windows Update](https://learn.microsoft.com/en-us/windows/deployment/update/) and Update history. | The update completes or the same failing KB is identified after restart. |
| **The device is managed or a compatibility hold is present** | Check the organization’s update owner and the applicable release-health notice; do not bypass the hold or policy. | The approved next update or remediation is recorded. |
| **The update prevents [Windows](https://www.microsoft.com/windows) from starting** | With recovery access ready, use [WinRE](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference) > Troubleshoot > Advanced options > Uninstall Updates when the affected update is the established cause. | The device returns to a bootable state for a scoped recovery plan. |
| **[Windows](https://www.microsoft.com/windows) starts but system/update corruption persists** | With saved work, recovery access, stable power and internet, open Settings > System > Recovery > Fix problems using [Windows Update](https://learn.microsoft.com/en-us/windows/deployment/update/) > Reinstall now if available. If managed policy hides it, use the organization’s repair route. | The same [Windows](https://www.microsoft.com/windows) version is repaired, apps/files/settings remain, and the original failed task or update succeeds after restart. |

### ✅ Finish the job

- [ ] Update history shows the intended result and no required restart remains.
- [ ] The original application and a normal reboot succeed.

**Change route when:** The next step would bypass a managed policy or documented safeguard without an approved plan. · Boot or storage failure changes the job into recovery.


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
