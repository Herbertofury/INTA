# Choose the right tool for the job

[Home](https://github.com/Herbertofury/INTA/wiki) › [Decision Desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) › **[Tool selection](https://github.com/Herbertofury/INTA/wiki/Tool-Selection)**

**Start with the question you need to answer.** Choose a diagnostic tool below, or use [remote capability preflight](https://github.com/Herbertofury/INTA/wiki/Job-remote-support) to select the connection for the work, the person at the device and the access that should remain afterward.

Documentation reviewed **November 26, 2026**. These are documented capabilities and INTA selection rules; this page does not claim a new lab comparison or that every client version behaves identically.

## Match the diagnostic question

| Question | Tool and official download | Useful first observation | Decision boundary |
| --- | --- | --- | --- |
| Which process, startup entry or I/O operation explains the symptom? | [Microsoft Sysinternals Suite](https://learn.microsoft.com/en-us/sysinternals/downloads/sysinternals-suite) | Use [Process Explorer](https://learn.microsoft.com/en-us/sysinternals/downloads/process-explorer/) for process detail, [Process Monitor](https://learn.microsoft.com/en-us/sysinternals/downloads/procmon) for a short filtered trace, or [Autoruns](https://learn.microsoft.com/en-us/sysinternals/downloads/autoruns) for startup inspection. | Collect evidence for a specific question before stopping processes or changing startup entries. |
| Which folders and files consume this volume? | [WizTree](https://diskanalyzer.com/download) | Inspect the folder totals, treemap and largest-file view; identify the owning application or person. | A large item or apparent duplicate is not permission to delete it. Follow supported cleanup and retention decisions. |
| Where is this named file, log or recently changed group of files? | [WizFile](https://antibody-software.com/wizfile/download) | Search the filename or full path, then narrow by size or date using the [quick-start guide](https://antibody-software.com/wizfile/quick-start). | A matching filename does not prove contents, safety or disposability. Exit the application when a one-time lookup is finished. |

Use this distinction in [slow-PC diagnosis](https://github.com/Herbertofury/INTA/wiki/Job-slow-pc) and [preventive maintenance](https://github.com/Herbertofury/INTA/wiki/Job-preventive-maintenance). If storage shows physical-failure symptoms, use [image-first recovery](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive) before broad disk inspection.

Both [WizTree](https://diskanalyzer.com/eula) and [WizFile](https://antibody-software.com/wizfile/eula) are free for personal use; their current EULAs require commercial coverage based on the business site and staff scope. Their technician provisions distinguish service work from the client's own continuing use. Choose the current official installer or portable package for the actual platform; [WizTree's download page](https://diskanalyzer.com/download) separates its desktop editions. [WizFile](https://antibody-software.com/wizfile/quick-start) can remain active in the system tray after its window closes: use **File → Exit** and remove any startup option created only for the case.

## Establish the remote-session requirements

| Requirement | Record before selecting a client |
| --- | --- |
| Presence | A person stays at the device, can return for a reboot, or has explicitly authorized unattended management. |
| Platform and privilege | Both operating systems, the target architecture, the account level and who can handle a protected prompt. |
| Work | View or control, required transfers and their direction, expected reboot, and any network change that could break the connection. |
| Trust boundary | The approved support server or service, the correct customer/device and the technician identity. |
| End state | Access ends with this case, or continuing access has a named owner, scope and review date. |

A convenient connection is only one prerequisite. If the task needs a reboot or an elevation path, prove that path before beginning a change that depends on it.

## Compare the remote-support choices

| Tool | Useful session fit | Presence and approval | Transfer route to plan |
| --- | --- | --- | --- |
| [Microsoft Quick Assist](https://learn.microsoft.com/en-us/windows/client-management/client-tools/quick-assist) | A straightforward attended session between supported [Windows](https://www.microsoft.com/windows) devices. | The recipient permits sharing; control is a separate request. Keep someone available for protected prompts. | Plan an approved separate transfer channel or download directly from the publisher on the target. The cited guide does not establish a general file-transfer workflow. |
| [RustDesk](https://rustdesk.com/) | Approved cross-platform support using the organization's selected server and client configuration. | Incoming connections can require a click, a password, or either; temporary and permanent password modes differ. Check the actual policy before calling the session attended. | Incoming file-transfer permission is configurable. Test the required direction with a harmless file before the repair. |
| [MeshCentral](https://meshcentral.com/) | Recurring managed support for an enrolled endpoint with an assigned owner and access scope. | Consent and privacy-bar options are available; management enrollment alone does not promise that a person will approve each session. | A separate **Files** channel supports transfers. Desktop, terminal and file access should match the assigned rights and the job. |
| [Chrome Remote Desktop](https://remotedesktop.google.com/) | Approved temporary sharing, or a separately authorized persistent-access setup. | Support uses a one-use code and confirmation every 30 minutes. Persistent access uses a different PIN-based path. | Preflight the actual clients and use an approved separate channel if needed; the cited support guide does not establish feature parity for file transfer across platforms. |
| [AnyDesk](https://anydesk.com/) | An existing, appropriately licensed commercial support deployment with the needed client features. | Interactive approval and unattended credentials are different settings. Inspect the effective permission profile. | The documented file manager supports desktop platforms; clipboard transfers have their own permissions. Confirm the exact required path. |

Sources: [Microsoft support workflow](https://support.microsoft.com/en-us/windows/apps/solve-pc-problems-remotely-using-quick-assist), [Microsoft deployment guidance](https://learn.microsoft.com/en-us/windows/client-management/client-tools/quick-assist), [RustDesk permissions](https://rustdesk.com/docs/en/self-host/client-configuration/advanced-settings/), [MeshCentral channels](https://docs.meshcentral.com/meshcentral/devicetabs/), [Google support/access workflow](https://support.google.com/chrome/answer/1649523), [AnyDesk access settings](https://support.anydesk.com/docs/settings) and [AnyDesk file management](https://support.anydesk.com/file-manager-and-file-transfer).

## Apply the business and deployment boundary

| Choice | Decision for INTA |
| --- | --- |
| [Microsoft Quick Assist](https://learn.microsoft.com/en-us/windows/client-management/client-tools/quick-assist) | It does not supply the same tenant controls and session audit features as a managed enterprise support service. Keep the private ticket record. Its [macOS](https://www.apple.com/macos/) and browser assistance paths are reserved for Microsoft Support, so they are not general INTA cross-platform routes. |
| [RustDesk](https://rustdesk.com/) | The [client repository](https://github.com/rustdesk/rustdesk) identifies an AGPL-3.0 license; [Server Pro](https://rustdesk.com/docs/en/self-host/rustdesk-server-pro/license/) is a separate licensed product. Record the actual deployed edition, service and applicable terms instead of treating every feature as part of the open-source client. |
| [MeshCentral](https://meshcentral.com/) | The [project repository](https://github.com/Ylianst/MeshCentral) identifies an Apache-2.0 license. The organization still needs an accountable server operator, updates, backups, access administration and retained evidence; those operating responsibilities are part of the service choice. |
| [Chrome Remote Desktop](https://remotedesktop.google.com/) | Use only within the customer's approved account and service policy. The cited end-user documentation establishes the access workflow; it does not establish an MSP-specific contract, retention plan or service guarantee. |
| [AnyDesk](https://anydesk.com/) | The vendor requires a license for clients initiating commercial sessions; receiving clients alone do not require that outgoing-client license. Check the [current plan scope](https://support.anydesk.com/docs/how-does-the-anydesk-licensing-model-work) for the technicians and concurrent sessions involved. |

Review the exact edition and agreement when procurement or deployment scope changes.

## Run a two-minute preflight

1. Confirm the customer, exact device and service request through the established channel. Check the support identity shown on the recipient's screen.
2. Explain the requested capability. Start with viewing when control is unnecessary; request the additional capability when the task needs it.
3. Test the required control and transfer route with non-sensitive input. Confirm where transferred files land and how they will be retained or removed.
4. Agree on the reconnect and local recovery plan before a reboot, adapter change or security-product restart.
5. Record the access end state. Do not create a permanent credential or second management agent merely to get past an inconvenient prompt.

For transfers, record **purpose → direction → approved destination → retention decision**. Download installers from the publisher on the target where practical; put diagnostic records in the private case system. Desktop control is not permission to copy unrelated folders.

## End the session according to its purpose

| Session purpose | Closure proof |
| --- | --- |
| One-time help | The user completes the original task; all support channels close; any case-only credential or enrollment is removed. |
| Continuing management | The session closes, while approved persistent access retains the correct customer, owner and rights. Record the next review date. |
| Work interrupted | Preserve the tested state, the reason for stopping and the exact next action. State whether an active connection or approved agent remains. |

Use [temporary access revocation](https://github.com/Herbertofury/INTA/wiki/Job-remote-support) and the [remote-session acceptance record](https://github.com/Herbertofury/INTA/wiki/Service-Evidence#remote-session-acceptance). A closed desktop window does not prove that another file or terminal channel, installed agent, or saved credential has gone away.

**Continue:** [Remote-support playbook](https://github.com/Herbertofury/INTA/wiki/Job-remote-support) · [Service Evidence](https://github.com/Herbertofury/INTA/wiki/Service-Evidence) · [Technician handoff](https://github.com/Herbertofury/INTA/wiki/Technician-Handoff)
