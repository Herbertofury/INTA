[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

<a id="️-remote-support"></a>
<a id="-remote-support"></a>

# [Remote support](https://github.com/Herbertofury/INTA/wiki/Job-remote-support)

Match an [attended support session](https://github.com/Herbertofury/INTA/wiki/Job-remote-support) or a managed endpoint to the right access model.

**8 tools** · [All categories](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Selection standards](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards)

**Selection question:** One-time help or recurring managed access?

<a id="-start-with-a-workflow"></a>

## Related playbooks

- [Start and finish a clean remote-support session](https://github.com/Herbertofury/INTA/wiki/Job-remote-support)
- [Close a repair with a clear client handoff](https://github.com/Herbertofury/INTA/wiki/Job-client-handoff)

<a id="compare-options"></a>

<a id="️-compare-the-options"></a>
<a id="-compare-the-options"></a>

## Tool comparison

| Tool | Choose it for | Platforms | Role |
| --- | --- | --- | --- |
| [Chrome Remote Desktop](#chrome-remote-desktop) | Attended support or an explicitly configured personal remote-access endpoint. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) | Core |
| [MeshCentral](#meshcentral) | INTA-managed unattended support with its own server and device groups. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) | Core |
| [Microsoft Quick Assist](#quick-assist) | A quick one-time [Windows](https://www.microsoft.com/windows) support session without deploying an RMM agent. | [Windows](https://www.microsoft.com/windows) | Core |
| [RustDesk](#rustdesk) | A flexible remote-control option across mixed client platforms. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) | Core |
| [Tailscale](#tailscale) | Remote administration of private services without exposing them directly to the internet. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) | Core |
| [AnyDesk](#anydesk) | Client support when the customer already standardizes on [AnyDesk](https://anydesk.com/). | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) | Specialist |
| [ScreenConnect](#screenconnect) | A ticket-oriented remote-support workflow with technician administration. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) | Specialist |
| [TeamViewer Remote](#teamviewer) | Supporting customers whose approved remote-access stack uses [TeamViewer](https://www.teamviewer.com/). | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) | Specialist |

Core means an everyday candidate; Specialist means a focused advanced use; Evaluate means prove it in a suitable test environment before promotion.

<a id="chrome-remote-desktop"></a>

## [Chrome Remote Desktop](https://remotedesktop.google.com/)

Browser-led remote access with separate temporary support and persistent access flows.

[Official download](https://remotedesktop.google.com/support) · [Documentation](https://support.google.com/chrome/answer/1649523) · [Publisher](https://remotedesktop.google.com/)

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

[Official download](https://github.com/Ylianst/MeshCentral) · [Documentation](https://docs.meshcentral.com/) · [Publisher](https://meshcentral.com/)

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

[Official download](https://apps.microsoft.com/detail/9P7BP5VNWKX5) · [Documentation](https://learn.microsoft.com/en-us/windows/client-management/client-tools/quick-assist) · [Publisher](https://learn.microsoft.com/en-us/windows/client-management/client-tools/quick-assist)

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

[Official download](https://github.com/rustdesk/rustdesk/releases/latest) · [Documentation](https://rustdesk.com/docs/en/) · [Publisher](https://rustdesk.com/)

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

[Official download](https://tailscale.com/download) · [Documentation](https://tailscale.com/docs) · [Publisher](https://tailscale.com/)

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

[Official download](https://anydesk.com/en/downloads) · [Documentation](https://support.anydesk.com/) · [Publisher](https://anydesk.com/)

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

<a id="screenconnect"></a>

## [ScreenConnect](https://www.screenconnect.com/)

[Remote support](https://github.com/Herbertofury/INTA/wiki/Job-remote-support) sessions and managed access for service teams.

[Official download](https://www.screenconnect.com/Download) · [Documentation](https://www.screenconnect.com/Download) · [Publisher](https://www.screenconnect.com/)

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

[Official download](https://www.teamviewer.com/en/download/) · [Documentation](https://www.teamviewer.com/en/global/support/knowledge-base/) · [Publisher](https://www.teamviewer.com/)

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
