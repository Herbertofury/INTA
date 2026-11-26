[🏠 Home](https://github.com/Herbertofury/INTA/wiki) · [🧭 Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [📚 Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [🧰 Tool directory](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [✨ Open interactive hub](https://herbertofury.github.io/INTA/)

# 🥾 Bootable media

Keep independent rescue environments ready and test the exact hardware boot path.

**7 tools** · [All disciplines](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Selection standards](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards)

> [!TIP]
> **Choose by the job:** Create media, recover startup, or rescue data?

## 🧭 Start with a workflow

- [Build and refresh a technician rescue USB](https://github.com/Herbertofury/INTA/wiki/Job-boot-usb)
- [Recover a Windows PC that will not boot](https://github.com/Herbertofury/INTA/wiki/Job-boot-recovery)
- [Rescue data from a failing or unreadable drive](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive)

<a id="compare-options"></a>

## ⚖️ Compare the options

| Tool | Choose it for | Platforms | Role |
| --- | --- | --- | --- |
| [Debian Live](#debian-live) | A general [Linux](https://www.kernel.org/) session independent of the installed operating system. | Bootable PC | 🟢 Core |
| [Hiren's BootCD PE](#hirens-bootcd-pe) | A familiar [Windows](https://www.microsoft.com/windows)-oriented offline bench toolkit. | Bootable PC | 🟢 Core |
| [Rufus](#rufus) | Preparing one ISO on a dedicated USB drive. | [Windows](https://www.microsoft.com/windows) | 🟢 Core |
| [SystemRescue](#systemrescue) | A ready-made [Linux](https://www.kernel.org/) recovery environment for an unbootable or damaged machine. | Bootable PC | 🟢 Core |
| [Ubuntu Desktop live media](#ubuntu-live) | Hardware checks and file access from a familiar [Linux](https://www.kernel.org/) desktop. | Bootable PC | 🟢 Core |
| [Ventoy](#ventoy) | Maintaining a technician USB with several independently updated rescue images. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/) | 🟢 Core |
| [Windows 11 installation media](#microsoft-windows11-media) | A clean installation or supported repair using the correct [Windows](https://www.microsoft.com/windows) edition and architecture. | [Windows](https://www.microsoft.com/windows), Bootable PC | 🟢 Core |

Core means an everyday candidate; Specialist means a focused advanced use; Evaluate means prove it in a suitable test environment before promotion.

---

<a id="debian-live"></a>

## [Debian Live](https://www.debian.org/CD/live/)

Official live images for a standard [Debian](https://www.debian.org/) desktop and recovery starting point.

**[Debian Live: official download →](https://www.debian.org/CD/live/)** · [Documentation](https://live-team.pages.debian.net/live-manual/html/live-manual/index.en.html) · [Publisher](https://www.debian.org/CD/live/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A general [Linux](https://www.kernel.org/) session independent of the installed operating system. |
| **Where it runs** | Bootable PC |
| **How you use it** | boot |
| **Access / pricing** | Free / open source |
| **Selection role** | 🟢 Core |

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://live-team.pages.debian.net/live-manual/html/live-manual/index.en.html)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[↑ Compare the options](#compare-options)

---

<a id="hirens-bootcd-pe"></a>

## [Hiren's BootCD PE](https://www.hirensbootcd.org/)

[Windows PE](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/winpe-intro) recovery environment containing a broad collection of technician utilities.

**[Hiren's BootCD PE: official download →](https://www.hirensbootcd.org/download/)** · [Documentation](https://www.hirensbootcd.org/howtos/) · [Publisher](https://www.hirensbootcd.org/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A familiar [Windows](https://www.microsoft.com/windows)-oriented offline bench toolkit. |
| **Where it runs** | Bootable PC |
| **How you use it** | boot |
| **Access / pricing** | Free distribution; bundled tools have individual terms |
| **Selection role** | 🟢 Core |

> **Before using it:** The official bundle contains older tool versions. Treat its ISO and each included application as separate freshness checks.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://www.hirensbootcd.org/howtos/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[↑ Compare the options](#compare-options)

---

<a id="rufus"></a>

## [Rufus](https://rufus.ie/)

Writes bootable USB media from operating-system and recovery images.

**[Rufus: official download →](https://rufus.ie/#download)** · [Documentation](https://github.com/pbatard/rufus/wiki/FAQ) · [Publisher](https://rufus.ie/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Preparing one ISO on a dedicated USB drive. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | portable |
| **Access / pricing** | Free / open source |
| **Selection role** | 🟢 Core |

> **Before using it:** Writing media erases the selected target. Verify the drive identity before starting.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://github.com/pbatard/rufus/wiki/FAQ)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[↑ Compare the options](#compare-options)

---

<a id="systemrescue"></a>

## [SystemRescue](https://www.system-rescue.org/)

[Linux](https://www.kernel.org/) rescue environment for storage, filesystem and administrative recovery work.

**[SystemRescue: official download →](https://www.system-rescue.org/Download/)** · [Documentation](https://www.system-rescue.org/manual/) · [Publisher](https://www.system-rescue.org/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A ready-made [Linux](https://www.kernel.org/) recovery environment for an unbootable or damaged machine. |
| **Where it runs** | Bootable PC |
| **How you use it** | boot |
| **Access / pricing** | Free / open source |
| **Selection role** | 🟢 Core |

> **Before using it:** Verify checksums/signatures and choose the intended disk before mounting or modifying filesystems.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://www.system-rescue.org/manual/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[↑ Compare the options](#compare-options)

---

<a id="ubuntu-live"></a>

## [Ubuntu Desktop live media](https://ubuntu.com/desktop)

Official [Ubuntu](https://ubuntu.com/) desktop installer with a live try-before-install workflow.

**[Ubuntu Desktop live media: official download →](https://ubuntu.com/download/desktop)** · [Documentation](https://ubuntu.com/tutorials/try-ubuntu-before-you-install) · [Publisher](https://ubuntu.com/desktop)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Hardware checks and file access from a familiar [Linux](https://www.kernel.org/) desktop. |
| **Where it runs** | Bootable PC |
| **How you use it** | boot |
| **Access / pricing** | Free / open source |
| **Selection role** | 🟢 Core |

> **Before using it:** Choose the live session when diagnosing; installing is a separate disk-changing action.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://ubuntu.com/tutorials/try-ubuntu-before-you-install)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[↑ Compare the options](#compare-options)

---

<a id="ventoy"></a>

## [Ventoy](https://www.ventoy.net/)

Boot menu for multiple ISO and other image files copied onto prepared media.

**[Ventoy: official download →](https://www.ventoy.net/en/download.html)** · [Documentation](https://www.ventoy.net/en/doc_start.html) · [Publisher](https://www.ventoy.net/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Maintaining a technician USB with several independently updated rescue images. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/) |
| **How you use it** | portable |
| **Access / pricing** | Free / open source |
| **Selection role** | 🟢 Core |

> **Before using it:** Initial installation formats the target. Updating [Ventoy](https://www.ventoy.net/) and replacing an ISO are separate operations; test boot compatibility.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://www.ventoy.net/en/doc_start.html)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[↑ Compare the options](#compare-options)

---

<a id="microsoft-windows11-media"></a>

## [Windows 11 installation media](https://www.microsoft.com/software-download/windows11)

Official Microsoft installation assistant, media creation and ISO download source.

**[Windows 11 installation media: official download →](https://www.microsoft.com/software-download/windows11)** · [Documentation](https://www.microsoft.com/software-download/windows11) · [Publisher](https://www.microsoft.com/software-download/windows11)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A clean installation or supported repair using the correct [Windows](https://www.microsoft.com/windows) edition and architecture. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), Bootable PC |
| **How you use it** | boot |
| **Access / pricing** | Download available; [Windows](https://www.microsoft.com/windows) license required |
| **Selection role** | 🟢 Core |

> **Before using it:** An installation can erase data. Match edition, language and architecture and preserve the recovery key.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://www.microsoft.com/software-download/windows11)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[↑ Compare the options](#compare-options)


---

**Source review: 2026-11-26.** Guidance review and tool release versions are separate. [Report a correction](https://github.com/Herbertofury/INTA/issues/new/choose) · [How we maintain this guide](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards)
