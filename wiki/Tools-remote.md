[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Service map](https://github.com/Herbertofury/INTA/wiki/Service-Map) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

<a id="️-remote-support"></a>
<a id="-remote-support"></a>

# [Remote support](https://github.com/Herbertofury/INTA/wiki/Job-remote-support)

Match an [attended support session](https://github.com/Herbertofury/INTA/wiki/Job-remote-support) or a managed endpoint to the right access model.

**9 tools** · [All categories](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Selection standards](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards)

**Selection question:** One-time help or recurring managed access?

<a id="-start-with-a-workflow"></a>

## Related playbooks

- **[🖥️ Help remotely](https://github.com/Herbertofury/INTA/wiki/Job-remote-support)** · **[Open working view](https://herbertofury.github.io/INTA/#job/remote-support)**
- **[📋 Complete the handoff](https://github.com/Herbertofury/INTA/wiki/Job-client-handoff)** · **[Open working view](https://herbertofury.github.io/INTA/#job/client-handoff)**

<a id="compare-options"></a>

<a id="️-compare-the-options"></a>
<a id="-compare-the-options"></a>

## Tool comparison

| Tool | Choose it for | Platforms / role | Official page + download |
| --- | --- | --- | --- |
| [Chrome Remote Desktop](#chrome-remote-desktop) | Attended support or an explicitly configured personal remote-access endpoint. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) · Core | [Official page](https://remotedesktop.google.com/) · **[Open remote support](https://remotedesktop.google.com/support)** |
| [MeshCentral](#meshcentral) | INTA-managed unattended support with its own server and device groups. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) · Core | [Official page](https://meshcentral.com/) · **[Open server installation instructions](https://github.com/Ylianst/MeshCentral#installation)** |
| [Microsoft Quick Assist](#quick-assist) | A quick one-time [Windows](https://www.microsoft.com/windows) support session without deploying an RMM agent. | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://learn.microsoft.com/en-us/windows/client-management/client-tools/quick-assist) · **[Get Quick Assist in Microsoft Store](https://apps.microsoft.com/detail/9P7BP5VNWKX5)** |
| [RustDesk](#rustdesk) | A flexible remote-control option across mixed client platforms. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) · Core | [Official page](https://rustdesk.com/) · **[Download Windows x64 · EXE](https://github.com/rustdesk/rustdesk/releases/download/1.5.3/rustdesk-1.5.3-x86_64.exe)** |
| [Tailscale](#tailscale) | Remote administration of private services without exposing them directly to the internet. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) · Core | [Official page](https://tailscale.com/) · **[Download Windows x64 MSI](https://dl.tailscale.com/stable/tailscale-setup-1.104.7-amd64.msi)** |
| [AnyDesk](#anydesk) | Client support when the customer already standardizes on [AnyDesk](https://anydesk.com/). | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) · Specialist | [Official page](https://anydesk.com/) · **[Download Windows EXE](https://download.anydesk.com/AnyDesk.exe)** |
| [Remmina](#remmina) | Connecting a [Linux](https://www.kernel.org/) technician workstation to an already configured remote desktop or server. | [Linux](https://www.kernel.org/) · Specialist | [Official page](https://remmina.org/) · **[Install Linux package · Flatpak or distribution](https://www.remmina.org/how-to-install-remmina/)** |
| [ScreenConnect](#screenconnect) | A ticket-oriented remote-support workflow with technician administration. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) · Specialist | [Official page](https://www.screenconnect.com/) · **[Client installer setup instructions](https://www.screenconnect.com/pricing/unattended-access)** |
| [TeamViewer Remote](#teamviewer) | Supporting customers whose approved remote-access stack uses [TeamViewer](https://www.teamviewer.com/). | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) · Specialist | [Official page](https://www.teamviewer.com/) · **[Download Windows x64 EXE](https://download.teamviewer.com/download/TeamViewer_Setup_x64.exe)** |

Core means an everyday candidate; Specialist means a focused advanced use; Evaluate means prove it in a suitable test environment before promotion.

<a id="chrome-remote-desktop"></a>

## [Chrome Remote Desktop](https://remotedesktop.google.com/)

Browser-led remote access with separate temporary support and persistent access flows.

[Official page](https://remotedesktop.google.com/) · **[Open remote support](https://remotedesktop.google.com/support)** · [Documentation](https://support.google.com/chrome/answer/1649523)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Attended support or an explicitly configured personal remote-access endpoint. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | installed |
| **Access / pricing** | Free service |
| **Selection role** | Core |

> **Before using it:** Support codes are one-time. Do not confuse temporary sharing with persistent PIN-based access.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://support.google.com/chrome/answer/1649523)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="meshcentral"></a>

## [MeshCentral](https://meshcentral.com/)

Self-hosted remote management with browser-based desktop, terminal and file access.

[Official page](https://meshcentral.com/) · **[Open server installation instructions](https://github.com/Ylianst/MeshCentral#installation)** · [All download options](https://github.com/Ylianst/MeshCentral) · [Documentation](https://docs.meshcentral.com/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | INTA-managed unattended support with its own server and device groups. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | installed |
| **Access / pricing** | Free / open source; hosting has a cost |
| **Selection role** | Core |

> **Before using it:** Use the agent generated by the correct INTA server; configure account MFA and scoped device access.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://docs.meshcentral.com/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="quick-assist"></a>

## [Microsoft Quick Assist](https://learn.microsoft.com/en-us/windows/client-management/client-tools/quick-assist)

[Windows](https://www.microsoft.com/windows) attended screen sharing using a support code and user-approved control.

[Official page](https://learn.microsoft.com/en-us/windows/client-management/client-tools/quick-assist) · **[Get Quick Assist in Microsoft Store](https://apps.microsoft.com/detail/9P7BP5VNWKX5)** · [Documentation](https://learn.microsoft.com/en-us/windows/client-management/client-tools/quick-assist)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A quick one-time [Windows](https://www.microsoft.com/windows) support session without deploying an RMM agent. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | installed |
| **Access / pricing** | Free Microsoft app |
| **Selection role** | Core |

> **Before using it:** The helper signs in; the user approves the session. The [macOS](https://support.apple.com/macos) edition is restricted to Microsoft Support interactions.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://learn.microsoft.com/en-us/windows/client-management/client-tools/quick-assist)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="rustdesk"></a>

## [RustDesk](https://rustdesk.com/)

Cross-platform remote desktop with self-hosted server options.

[Official page](https://rustdesk.com/) · **[Download Windows x64 · EXE](https://github.com/rustdesk/rustdesk/releases/download/1.5.3/rustdesk-1.5.3-x86_64.exe)** · [All download options](https://github.com/rustdesk/rustdesk/releases/latest) · [Documentation](https://rustdesk.com/docs/en/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A flexible remote-control option across mixed client platforms. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | installed |
| **Access / pricing** | Open-source components; paid Server Pro features |
| **Selection role** | Core |

> **Before using it:** Match the client to the intended server and key. Persistent access must be a deliberate managed configuration.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://rustdesk.com/docs/en/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="tailscale"></a>

## [Tailscale](https://tailscale.com/)

Identity-based private connectivity for reaching approved devices and services.

[Official page](https://tailscale.com/) · **[Download Windows x64 MSI](https://dl.tailscale.com/stable/tailscale-setup-1.104.7-amd64.msi)** · [Download Windows ARM64 MSI](https://dl.tailscale.com/stable/tailscale-setup-1.104.7-arm64.msi) · [Other platforms and packages](https://tailscale.com/download) · [Documentation](https://tailscale.com/docs)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Remote administration of private services without exposing them directly to the internet. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | installed |
| **Access / pricing** | Personal and paid business plans |
| **Selection role** | Core |

> **Before using it:** It provides connectivity rather than a remote-desktop interface; apply device and access policies.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://tailscale.com/docs)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="anydesk"></a>

## [AnyDesk](https://anydesk.com/)

Remote desktop and support software with business plans and portable client options.

[Official page](https://anydesk.com/) · **[Download Windows EXE](https://download.anydesk.com/AnyDesk.exe)** · [Other platforms](https://anydesk.com/en/downloads) · [Documentation](https://support.anydesk.com/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Client support when the customer already standardizes on [AnyDesk](https://anydesk.com/). |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | portable |
| **Access / pricing** | Commercial license for professional use |
| **Selection role** | Specialist |

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://support.anydesk.com/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="remmina"></a>

## [Remmina](https://remmina.org/)

[Linux](https://www.kernel.org/) desktop client for RDP, VNC, SSH and other remote-access protocols through plugins.

[Official page](https://remmina.org/) · **[Install Linux package · Flatpak or distribution](https://www.remmina.org/how-to-install-remmina/)** · [Open upstream source and build guidance](https://gitlab.com/Remmina/Remmina) · [Documentation](https://www.remmina.org/how-to-install-remmina/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Connecting a [Linux](https://www.kernel.org/) technician workstation to an already configured remote desktop or server. |
| **Where it runs** | [Linux](https://www.kernel.org/) |
| **How you use it** | installed |
| **Access / pricing** | Free and open source under GPL-2.0-or-later |
| **Selection role** | Specialist |

> **Before using it:** This is a client: the target protocol, network access and authorized account must already be available. Package plugins and keyring integration vary. Verify host identity and certificate warnings; stored profiles may contain sensitive connection details.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://remmina.org/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Primary documentation reviewed; published file routes were checked with bounded response samples where supplied. No INTA installation, hardware, recovery or efficacy test is claimed.
- **Selection provenance:**
  - Current primary documentation and publisher acquisition sources
  - https://remmina.org/
  - https://www.remmina.org/how-to-install-remmina/
  - https://gitlab.com/Remmina/Remmina/blob/master/COPYING

</details>

[Back to comparison](#compare-options)

<a id="screenconnect"></a>

## [ScreenConnect](https://www.screenconnect.com/)

[Remote support](https://github.com/Herbertofury/INTA/wiki/Job-remote-support) sessions and managed access for service teams.

[Official page](https://www.screenconnect.com/) · **[Client installer setup instructions](https://www.screenconnect.com/pricing/unattended-access)** · [Server downloads · licence required](https://www.screenconnect.com/Download) · [Documentation](https://www.screenconnect.com/Download)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A ticket-oriented remote-support workflow with technician administration. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | installed |
| **Access / pricing** | Paid service or licensed deployment |
| **Selection role** | Specialist |

> **Before using it:** Keep the server and agents current and restrict administrative access.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://www.screenconnect.com/Download)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="teamviewer"></a>

## [TeamViewer Remote](https://www.teamviewer.com/)

Commercial [remote support](https://github.com/Herbertofury/INTA/wiki/Job-remote-support) and device-access platform.

[Official page](https://www.teamviewer.com/) · **[Download Windows x64 EXE](https://download.teamviewer.com/download/TeamViewer_Setup_x64.exe)** · [QuickSupport x64 EXE](https://download.teamviewer.com/download/TeamViewerQS_x64.exe) · [Download Windows ARM64 EXE](https://download.teamviewer.com/download/TeamViewer_Setup_ARM64.exe) · [Other platforms and editions](https://www.teamviewer.com/en/download/) · [Documentation](https://www.teamviewer.com/en/global/support/knowledge-base/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Supporting customers whose approved remote-access stack uses [TeamViewer](https://www.teamviewer.com/). |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | installed |
| **Access / pricing** | Commercial license for professional use |
| **Selection role** | Specialist |

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://www.teamviewer.com/en/global/support/knowledge-base/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
