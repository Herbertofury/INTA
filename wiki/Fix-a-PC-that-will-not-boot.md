[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Service map](https://github.com/Herbertofury/INTA/wiki/Service-Map) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

# 🚀 Fix a PC that will not boot

**Record the startup error and confirm disk health and [BitLocker](https://learn.microsoft.com/en-us/windows/security/operating-system-security/data-protection/bitlocker/) recovery access before attempting repairs.**

<a id="tool-choices"></a>

## 🧰 Choose the tool for this job

Choose by the task shown in the first column. Each option includes its acquisition route and access boundary.

| Need | Official tool + exact action | Why this choice / access |
| --- | --- | --- |
| **[Startup recovery](https://github.com/Herbertofury/INTA/wiki/Job-boot-recovery)** | **[Windows Recovery Environment](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference)** · **[Open recovery instructions](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference)** | Use the least disruptive applicable recovery option after preserving data and recovery keys. **Access:** Included with supported [Windows](https://www.microsoft.com/windows) |
| **Trusted repair media** | **[Windows 11 installation media](https://www.microsoft.com/software-download/windows11)** · **[Download media creation tool · EXE](https://go.microsoft.com/fwlink/?linkid=2156295)** | Create official matching-architecture media on a working PC when local recovery is unavailable. **Access:** Media is free; [Windows](https://www.microsoft.com/windows) licensing remains separate. |
| **Preserve unstable storage** | **[SystemRescue](https://www.system-rescue.org/)** · **[Download x64 rescue image · ISO](https://fastly-cdn.system-rescue.org/releases/13.03/systemrescue-13.03-amd64.iso)** | Use a separate recovery environment for image-first work when the disk may be failing. **Access:** Free and open source |
| **Eligible widespread failure** | **[Quick machine recovery](https://learn.microsoft.com/en-us/windows/configuration/quick-machine-recovery/?tabs=csp)** · **[Check supported recovery setup](https://learn.microsoft.com/en-us/windows/configuration/quick-machine-recovery/?tabs=csp)** | Check the configured cloud recovery option on a supported device when a matching widespread boot remedy exists. **Access:** Included on eligible, configured [Windows](https://www.microsoft.com/windows) devices. |

**[Current selection evidence](https://github.com/Herbertofury/INTA/wiki/Current-Tool-Choices)**

<a id="working-kit"></a>

## 🧰 Tools and links for this job

**[Open working view](https://herbertofury.github.io/INTA/#job/boot-recovery/quick/fix-boot-failure)** · Keep the tool panel beside the instructions.

**First move:** Record the startup error and confirm disk health and [BitLocker](https://learn.microsoft.com/en-us/windows/security/operating-system-security/data-protection/bitlocker/) recovery access before attempting repairs.

| Use this for | Official tool + exact action |
| --- | --- |
| Fix a PC that will not boot | **[Windows Recovery Environment](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference)** · [Official page](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference) · **[Open recovery instructions](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference)** |
| Create trusted repair media on a working PC. | **[Windows 11 installation media](https://www.microsoft.com/software-download/windows11)** · [Official page](https://www.microsoft.com/software-download/windows11) · **[Download media creation tool · EXE](https://go.microsoft.com/fwlink/?linkid=2156295)** |
| Preserve an unstable disk before boot-repair writes. | **[SystemRescue](https://www.system-rescue.org/)** · [Official page](https://www.system-rescue.org/) · **[Download x64 rescue image · ISO](https://fastly-cdn.system-rescue.org/releases/13.03/systemrescue-13.03-amd64.iso)** |
| Check for a matching cloud remedy on eligible, configured devices with a widespread startup failure. | **[Quick machine recovery](https://learn.microsoft.com/en-us/windows/configuration/quick-machine-recovery/?tabs=csp)** · [Official page](https://learn.microsoft.com/en-us/windows/configuration/quick-machine-recovery/?tabs=csp) · **[Check supported recovery setup](https://learn.microsoft.com/en-us/windows/configuration/quick-machine-recovery/?tabs=csp)** |

### [Startup repair](https://support.microsoft.com/en-us/windows/experience/startup-boot/startup-repair)

| Open the exact resource | What to do there |
| --- | --- |
| **[Enter Windows Recovery Environment](https://support.microsoft.com/en-us/windows/experience/backup-recovery/windows-recovery-environment)** | Use supported recovery media or the available advanced-startup path. |
| **[Diagnose the failed boot stage](https://learn.microsoft.com/en-us/windows/client-management/advanced-troubleshooting-boot-problems)** | Separate firmware, boot manager, loader and OS failures. |

### Unlock the intended disk

| Open the exact resource | What to do there |
| --- | --- |
| **[Find the matching BitLocker recovery key](https://support.microsoft.com/en-us/windows/security/encryption/find-your-bitlocker-recovery-key)** | Match the on-screen key ID with the approved personal or organization recovery record. |

### Recovery

| Open the exact resource | What to do there |
| --- | --- |
| **[Choose a recovery operation](https://support.microsoft.com/en-us/windows/experience/backup-recovery/recovery-options-in-windows)** | Check the data and application effects before reset or reinstall. |

### Current recovery

| Open the exact resource | What to do there |
| --- | --- |
| **[Quick machine recovery — conditional](https://learn.microsoft.com/en-us/windows/configuration/quick-machine-recovery/?tabs=csp)** | [Windows 11](https://www.microsoft.com/windows/windows-11) 24H2 build 26100.4700 or later; verify policy, network access and a matching remedy. Enterprise defaults can be off. |

## ⚡ First three actions

1. Check power, attached devices and the selected boot disk; stop if storage is failing.
2. Open [Windows Recovery Environment](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference) and run [Startup Repair](https://support.microsoft.com/en-us/windows/experience/startup-boot/startup-repair) for the identified installation when appropriate.
3. Follow the matching recovery branch, verify a normal boot and open priority files before closing the job.

**[Windows Recovery Environment](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference)** · **[Open recovery instructions](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference)**

**[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-boot-recovery)** · **[Specific fixes for this job](https://herbertofury.github.io/INTA/#job/boot-recovery/quick/fix-boot-failure?section=fixes)** · **[Selection evidence](https://github.com/Herbertofury/INTA/wiki/Free-Tool-Picks)**

<a id="next-action"></a>

## 🧭 Match the symptom

| You see this | Do this next | You should get |
| --- | --- | --- |
| **Firmware does not list the intended disk** | Check the correct storage device and connection using the hardware procedure; stop if the disk is unstable. | The disk is detected consistently or the job is routed to hardware/[data recovery](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive). |
| **[BitLocker](https://learn.microsoft.com/en-us/windows/security/operating-system-security/data-protection/bitlocker/) asks for a recovery key** | Match the displayed key ID to the approved recovery record; keep the key out of the public ticket or wiki. | The correct encrypted volume unlocks without a reset or format. |
| **The disk is stable but [Windows](https://www.microsoft.com/windows) cannot start** | Open [WinRE](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference) > Troubleshoot > Advanced options > [Startup Repair](https://support.microsoft.com/en-us/windows/experience/startup-boot/startup-repair) for the identified installation. | [Startup Repair](https://support.microsoft.com/en-us/windows/experience/startup-boot/startup-repair) reports its result and a normal boot can be tested. |
| **A broad known startup failure may have a cloud remedy** | Check supported [Quick machine recovery](https://learn.microsoft.com/en-us/windows/configuration/quick-machine-recovery/?tabs=csp) availability, network access and organization policy in the recovery interface. | A remedy is offered and tested, or ordinary recovery continues without assuming a cloud fix exists. |
| **A recent update or driver change is the established trigger** | Use the applicable uninstall or restore choice in [WinRE](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference) after confirming recovery access and data protection. | [Windows](https://www.microsoft.com/windows) boots without losing the required files and the triggering change is documented. |

### ✅ Finish the job

- [ ] The device boots normally twice and the user can open priority files.
- [ ] The intended boot disk and encryption recovery remain available.

**Change route when:** The disk fails, disconnects or contains the only unprotected data copy. · Recovery keys or the identity of the target installation are uncertain.


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
