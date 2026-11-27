[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Service map](https://github.com/Herbertofury/INTA/wiki/Service-Map) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

# 🖥️ Help remotely

**Confirm the user and support technician through the existing support channel before sharing a session code.**

<a id="tool-choices"></a>

## 🧰 Choose the tool for this job

Choose by the task shown in the first column. Each option includes its acquisition route and access boundary.

| Need | Official tool + exact action | Why this choice / access |
| --- | --- | --- |
| **Attended [Windows](https://www.microsoft.com/windows) help** | **[Microsoft Quick Assist](https://learn.microsoft.com/en-us/windows/client-management/client-tools/quick-assist)** · **[Get Quick Assist in Microsoft Store](https://apps.microsoft.com/detail/9P7BP5VNWKX5)** | Use a short-lived approved session with a user present to share and authorize control. **Access:** Included app; helper Microsoft account required. |
| **Cross-platform support** | **[RustDesk](https://rustdesk.com/)** · **[Download Windows x64 · EXE](https://github.com/rustdesk/rustdesk/releases/download/1.5.3/rustdesk-1.5.3-x86_64.exe)** | Use the approved client/server configuration for cross-platform desktop support. **Access:** Open-source components; Pro and hosting costs vary. |
| **Existing managed fleet** | **[MeshCentral](https://meshcentral.com/)** · **[Open server installation instructions](https://github.com/Ylianst/MeshCentral#installation)** | Use the organization’s existing server and enrolled device for managed access. **Access:** Open source; hosting and operation are separate. |
| **[Linux](https://www.kernel.org/) remote client** | **[Remmina](https://remmina.org/)** · **[Install Linux package · Flatpak or distribution](https://www.remmina.org/how-to-install-remmina/)** | Connect a [Linux](https://www.kernel.org/) technician workstation to an already configured RDP, VNC or SSH service. **Access:** Free and open source |

**[Current selection evidence](https://github.com/Herbertofury/INTA/wiki/Current-Tool-Choices)**

<a id="working-kit"></a>

## 🧰 Tools and links for this job

**[Open working view](https://herbertofury.github.io/INTA/#job/remote-support/quick/remote-help)** · Keep the tool panel beside the instructions.

**First move:** Confirm the user and support technician through the existing support channel before sharing a session code.

| Use this for | Official tool + exact action |
| --- | --- |
| Help remotely | **[Microsoft Quick Assist](https://learn.microsoft.com/en-us/windows/client-management/client-tools/quick-assist)** · [Official page](https://learn.microsoft.com/en-us/windows/client-management/client-tools/quick-assist) · **[Get Quick Assist in Microsoft Store](https://apps.microsoft.com/detail/9P7BP5VNWKX5)** |
| Approved cross-platform or self-hosted support. | **[RustDesk](https://rustdesk.com/)** · [Official page](https://rustdesk.com/) · **[Download Windows x64 · EXE](https://github.com/rustdesk/rustdesk/releases/download/1.5.3/rustdesk-1.5.3-x86_64.exe)** |
| Existing managed-device access and service oversight. | **[MeshCentral](https://meshcentral.com/)** · [Official page](https://meshcentral.com/) · **[Open server installation instructions](https://github.com/Ylianst/MeshCentral#installation)** |
| Connecting a [Linux](https://www.kernel.org/) technician workstation to an already configured remote desktop or server. | **[Remmina](https://remmina.org/)** · [Official page](https://remmina.org/) · **[Install Linux package · Flatpak or distribution](https://www.remmina.org/how-to-install-remmina/)** |

### Attended session

| Open the exact resource | What to do there |
| --- | --- |
| **[Quick Assist support-staff workflow](https://learn.microsoft.com/en-us/windows/client-management/client-tools/quick-assist)** | Code, helper sign-in, user approval and audit-log limits. |

### Approved alternative

| Open the exact resource | What to do there |
| --- | --- |
| **[RustDesk platform instructions](https://rustdesk.com/docs/en/)** | Use the approved server and required platform permissions. |

### Managed access

| Open the exact resource | What to do there |
| --- | --- |
| **[MeshCentral administration](https://docs.meshcentral.com/)** | Use the organization’s existing device group and access controls. |

## ⚡ First three actions

1. Open [Microsoft Quick Assist](https://learn.microsoft.com/en-us/windows/client-management/client-tools/quick-assist) on both [Windows](https://www.microsoft.com/windows) PCs; the helper signs in and supplies the temporary code.
2. Have the user approve screen sharing and any separately requested control before making the agreed change.
3. Test the original task with the user, end the session and record the work in the service ticket.

**[Microsoft Quick Assist](https://learn.microsoft.com/en-us/windows/client-management/client-tools/quick-assist)** · **[Get Quick Assist in Microsoft Store](https://apps.microsoft.com/detail/9P7BP5VNWKX5)**

**[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-remote-support)** · **[Specific fixes for this job](https://herbertofury.github.io/INTA/#job/remote-support/quick/remote-help?section=fixes)** · **[Selection evidence](https://github.com/Herbertofury/INTA/wiki/Free-Tool-Picks)**

<a id="next-action"></a>

## 🧭 Match the symptom

| You see this | Do this next | You should get |
| --- | --- | --- |
| **A [Windows](https://www.microsoft.com/windows) user is present and can approve a session** | Open [Microsoft Quick Assist](https://learn.microsoft.com/en-us/windows/client-management/client-tools/quick-assist) on both PCs, exchange the temporary code and have the user approve sharing and any separate control request. | The correct screen is visible and the user understands the control state. |
| **The task requires unattended access or durable audit records** | Use the approved managed tool and existing device identity; verify the technician’s role before connecting. | The session is tied to the intended asset and required audit record. |
| **Elevation or a secure-desktop prompt blocks work** | Have the authorized local user respond or use the approved managed elevation workflow; do not ask for credentials in chat. | The required action is approved without disclosing credentials or bypassing consent. |
| **The repair appears complete** | Repeat the original task with the user, end the session and remove any case-only access setting. | The user confirms the outcome and temporary control has ended. |

### ✅ Finish the job

- [ ] The original task works in the user’s session.
- [ ] The session is closed and authorized persistent access is documented.

**Change route when:** The user/helper identity or device target cannot be verified. · Requested access exceeds the approved session or exposes an active compromise.


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
