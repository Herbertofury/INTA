[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Service map](https://github.com/Herbertofury/INTA/wiki/Service-Map) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

# 💻 Set up a new PC

**Confirm the owner, approved applications, identity setup and recovery requirements before installing software.**

<a id="tool-choices"></a>

## 🧰 Choose the tool for this job

Choose by the task shown in the first column. Each option includes its acquisition route and access boundary.

| Need | Official tool + exact action | Why this choice / access |
| --- | --- | --- |
| **Approved application setup** | **[WinGet](https://learn.microsoft.com/en-us/windows/package-manager/winget/)** · **[Download App Installer · MSIXBUNDLE](https://github.com/microsoft/winget-cli/releases/download/v1.29.380/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle)** | Identify exact packages and sources before installing the agreed applications. **Access:** Included App Installer; each app keeps its own license. |
| **Model-specific drivers** | **[Official OEM driver and firmware portals](https://support.microsoft.com/en-us/windows/hardware/drivers/update-drivers-through-device-manager-in-windows)** · **[Find the correct driver source](https://support.microsoft.com/en-us/windows/hardware/drivers/update-drivers-through-device-manager-in-windows)** | Use the device manufacturer’s supported drivers and firmware for the exact model and OS. **Access:** Vendor downloads; support terms vary. |
| **Move agreed user files** | **[Robocopy](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy)** · **[Open built-in copy instructions](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy)** | Copy the approved data with an inspectable log and verify it as the target user. **Access:** Included with supported [Windows](https://www.microsoft.com/windows) |

**[Current selection evidence](https://github.com/Herbertofury/INTA/wiki/Current-Tool-Choices)**

<a id="working-kit"></a>

## 🧰 Tools and links for this job

**[Open working view](https://herbertofury.github.io/INTA/#job/new-pc/quick/set-up-a-new-pc)** · Keep the tool panel beside the instructions.

**First move:** Confirm the owner, approved applications, identity setup and recovery requirements before installing software.

| Use this for | Official tool + exact action |
| --- | --- |
| Set up a new PC | **[WinGet](https://learn.microsoft.com/en-us/windows/package-manager/winget/)** · [Official page](https://learn.microsoft.com/en-us/windows/package-manager/winget/) · **[Download App Installer · MSIXBUNDLE](https://github.com/microsoft/winget-cli/releases/download/v1.29.380/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle)** |
| Use the exact device vendor’s supported drivers and firmware. | **[Official OEM driver and firmware portals](https://support.microsoft.com/en-us/windows/hardware/drivers/update-drivers-through-device-manager-in-windows)** · [Official page](https://support.microsoft.com/en-us/windows/hardware/drivers/update-drivers-through-device-manager-in-windows) · **[Find the correct driver source](https://support.microsoft.com/en-us/windows/hardware/drivers/update-drivers-through-device-manager-in-windows)** |
| Move only the agreed files with a reviewed copy plan. | **[Robocopy](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy)** · [Official page](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy) · **[Open built-in copy instructions](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy)** |
| Evaluate an approved package in a disposable desktop with network and shared-folder exposure explicitly configured. | **[Windows Sandbox](https://learn.microsoft.com/en-us/windows/security/application-security/application-isolation/windows-sandbox/)** · [Official page](https://learn.microsoft.com/en-us/windows/security/application-security/application-isolation/windows-sandbox/) · **[Enable Windows Sandbox](https://learn.microsoft.com/en-us/windows/security/application-security/application-isolation/windows-sandbox/windows-sandbox-install)** |
| Synchronize an explicitly chosen healthy-data folder between approved machines; keep an independent backup. | **[Syncthing](https://syncthing.net/)** · [Official page](https://syncthing.net/) · **[Download Windows x64 · ZIP](https://github.com/syncthing/syncthing/releases/download/v2.1.7/syncthing-windows-amd64-v2.1.7.zip)** |
| Starting an approved installer or rescue environment through network boot or remote virtual media when local media delivery is inconvenient. | **[netboot.xyz](https://netboot.xyz/)** · [Official page](https://netboot.xyz/) · **[Download PC network-boot image · ISO](https://github.com/netbootxyz/netboot.xyz/releases/download/3.0.3/netboot.xyz.iso)** |
| Establishing recoverable, per-device administrator access during managed workstation setup and support planning. | **[Windows LAPS](https://learn.microsoft.com/en-us/windows-server/identity/laps/laps-overview)** · [Official page](https://learn.microsoft.com/en-us/windows-server/identity/laps/laps-overview) · **[Configure built-in administrator password management](https://learn.microsoft.com/en-us/windows-server/identity/laps/laps-overview)** |

### Applications

| Open the exact resource | What to do there |
| --- | --- |
| **[WinGet package identification and install](https://learn.microsoft.com/en-us/windows/package-manager/winget/install)** | Review the exact package and source before installing. |
| **[WinGet import review](https://learn.microsoft.com/en-us/windows/package-manager/winget/import)** | Reapply an approved inventory; licenses and user data remain separate. |

### Device readiness

| Open the exact resource | What to do there |
| --- | --- |
| **[Supported Windows driver updates](https://support.microsoft.com/en-us/windows/hardware/drivers/update-drivers-through-device-manager-in-windows)** | Use [Windows Update](https://learn.microsoft.com/en-us/windows/deployment/update/) and the correct device/manufacturer route. |

### Handoff

| Open the exact resource | What to do there |
| --- | --- |
| **[Workstation acceptance sheet](https://github.com/Herbertofury/INTA/wiki/Technician-Handoff)** | Record identity, encryption recovery, backup and daily-task proof. |

## ⚡ First three actions

1. Complete the approved [Windows](https://www.microsoft.com/windows) and device-driver updates and check basic hardware operation.
2. Use [WinGet](https://learn.microsoft.com/en-us/windows/package-manager/winget/) for approved available packages, reviewing the package identity and each application’s license.
3. Verify sign-in, encryption recovery, backup and the user’s daily tasks before handoff.

**[WinGet](https://learn.microsoft.com/en-us/windows/package-manager/winget/)** · **[Download App Installer · MSIXBUNDLE](https://github.com/microsoft/winget-cli/releases/download/v1.29.380/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle)**

**[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-new-pc)** · **[Specific fixes for this job](https://herbertofury.github.io/INTA/#job/new-pc/quick/set-up-a-new-pc?section=fixes)** · **[Selection evidence](https://github.com/Herbertofury/INTA/wiki/Free-Tool-Picks)**

<a id="next-action"></a>

## 🧭 Match the symptom

| You see this | Do this next | You should get |
| --- | --- | --- |
| **The device is new or has a fresh installation** | Complete the approved enrollment and [Windows](https://www.microsoft.com/windows)/OEM updates, then check [Device Manager](https://support.microsoft.com/en-us/windows/hardware/drivers/update-drivers-through-device-manager-in-windows) for unresolved devices. | The intended identity and basic hardware work without unexplained device faults. |
| **Approved applications must be installed** | Use [WinGet](https://learn.microsoft.com/en-us/windows/package-manager/winget/) search/show to confirm package identity and review installation or the approved import list. | Each required app launches with the correct license and user context. |
| **Data comes from the previous device** | Copy the agreed data using the migration plan and test representative files in their real applications. | The user can access the expected files without relying on the old device. |
| **Setup is nearly complete** | Verify encryption recovery storage, a backup restore, printing and the user’s daily workflow before handoff. | The workstation is usable and recoverable, with acceptance evidence recorded. |

### ✅ Finish the job

- [ ] The owner can sign in and complete the agreed daily tasks.
- [ ] Recovery material and a tested backup are accessible through the approved private process.

**Change route when:** Enrollment, licensing or recovery ownership is unclear. · Migration would endanger the only copy of user data.


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
