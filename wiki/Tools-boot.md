[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

<a id="-bootable-media"></a>

# Bootable media

Keep independent rescue environments ready and test the exact hardware boot path.

**7 tools** · [All categories](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Selection standards](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards)

**Selection question:** Create media, recover startup, or rescue data?

<a id="-start-with-a-workflow"></a>

## Related playbooks

- **[🔌 Build a rescue USB](https://github.com/Herbertofury/INTA/wiki/Job-boot-usb)** · **[Open working view](https://herbertofury.github.io/INTA/#job/boot-usb)**
- **[🚑 Fix boot failure](https://github.com/Herbertofury/INTA/wiki/Job-boot-recovery)** · **[Open working view](https://herbertofury.github.io/INTA/#job/boot-recovery)**
- **[💾 Recover data](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive)** · **[Open working view](https://herbertofury.github.io/INTA/#job/failing-drive)**

<a id="compare-options"></a>

<a id="️-compare-the-options"></a>
<a id="-compare-the-options"></a>

## Tool comparison

| Tool | Choose it for | Platforms / role | Official page + download |
| --- | --- | --- | --- |
| [Debian Live](#debian-live) | A general [Linux](https://www.kernel.org/) session independent of the installed operating system. | Bootable PC · Core | [Official page](https://www.debian.org/CD/live/) · **[Download x64 Xfce live image · ISO](https://cdimage.debian.org/debian-cd/current-live/amd64/iso-hybrid/debian-live-13.8.0-amd64-xfce.iso)** |
| [Hiren's BootCD PE](#hirens-bootcd-pe) | A familiar [Windows](https://www.microsoft.com/windows)-oriented offline bench toolkit. | Bootable PC · Core | [Official page](https://www.hirensbootcd.org/) · **[Download x64 rescue image · ISO](https://www.hirensbootcd.org/files/HBCD_PE_x64.iso)** |
| [Rufus](#rufus) | Preparing one ISO on a dedicated USB drive. | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://rufus.ie/) · **[Download Windows x64 portable · EXE](https://github.com/pbatard/rufus/releases/download/v4.16/rufus-4.16p.exe)** |
| [SystemRescue](#systemrescue) | A ready-made [Linux](https://www.kernel.org/) recovery environment for an unbootable or damaged machine. | Bootable PC · Core | [Official page](https://www.system-rescue.org/) · **[Download x64 rescue image · ISO](https://fastly-cdn.system-rescue.org/releases/13.03/systemrescue-13.03-amd64.iso)** |
| [Ubuntu Desktop live media](#ubuntu-live) | Hardware checks and file access from a familiar [Linux](https://www.kernel.org/) desktop. | Bootable PC · Core | [Official page](https://ubuntu.com/desktop) · **[Download x64 desktop image · ISO](https://releases.ubuntu.com/26.04.1/ubuntu-26.04.1-desktop-amd64.iso)** |
| [Ventoy](#ventoy) | Maintaining a technician USB with several independently updated rescue images. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/) · Core | [Official page](https://www.ventoy.net/) · **[Download Windows package · ZIP](https://github.com/ventoy/Ventoy/releases/download/v1.1.18/ventoy-1.1.18-windows.zip)** |
| [Windows 11 installation media](#microsoft-windows11-media) | A clean installation or supported repair using the correct [Windows](https://www.microsoft.com/windows) edition and architecture. | [Windows](https://www.microsoft.com/windows), Bootable PC · Core | [Official page](https://www.microsoft.com/software-download/windows11) · **[Download media creation tool · EXE](https://go.microsoft.com/fwlink/?linkid=2156295)** |

Core means an everyday candidate; Specialist means a focused advanced use; Evaluate means prove it in a suitable test environment before promotion.

<a id="debian-live"></a>

## [Debian Live](https://www.debian.org/CD/live/)

Official live images for a standard [Debian](https://www.debian.org/) desktop and recovery starting point.

[Official page](https://www.debian.org/CD/live/) · **[Download x64 Xfce live image · ISO](https://cdimage.debian.org/debian-cd/current-live/amd64/iso-hybrid/debian-live-13.8.0-amd64-xfce.iso)** · [Documentation](https://live-team.pages.debian.net/live-manual/html/live-manual/index.en.html)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A general [Linux](https://www.kernel.org/) session independent of the installed operating system. |
| **Where it runs** | Bootable PC |
| **How you use it** | boot |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://live-team.pages.debian.net/live-manual/html/live-manual/index.en.html)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="hirens-bootcd-pe"></a>

## [Hiren's BootCD PE](https://www.hirensbootcd.org/)

[Windows PE](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/winpe-intro) recovery environment containing a broad collection of technician utilities.

[Official page](https://www.hirensbootcd.org/) · **[Download x64 rescue image · ISO](https://www.hirensbootcd.org/files/HBCD_PE_x64.iso)** · [All download options](https://www.hirensbootcd.org/download/) · [Documentation](https://www.hirensbootcd.org/howtos/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A familiar [Windows](https://www.microsoft.com/windows)-oriented offline bench toolkit. |
| **Where it runs** | Bootable PC |
| **How you use it** | boot |
| **Access / pricing** | Free distribution; bundled tools have individual terms |
| **Selection role** | Core |

> **Before using it:** The official bundle contains older tool versions. Treat its ISO and each included application as separate freshness checks.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://www.hirensbootcd.org/howtos/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="rufus"></a>

## [Rufus](https://rufus.ie/)

Writes bootable USB media from operating-system and recovery images.

[Official page](https://rufus.ie/) · **[Download Windows x64 portable · EXE](https://github.com/pbatard/rufus/releases/download/v4.16/rufus-4.16p.exe)** · [All download options](https://rufus.ie/#download) · [Documentation](https://github.com/pbatard/rufus/wiki/FAQ)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Preparing one ISO on a dedicated USB drive. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | portable |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** Writing media erases the selected target. Verify the drive identity before starting.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://github.com/pbatard/rufus/wiki/FAQ)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="systemrescue"></a>

## [SystemRescue](https://www.system-rescue.org/)

[Linux](https://www.kernel.org/) rescue environment for storage, filesystem and administrative recovery work.

[Official page](https://www.system-rescue.org/) · **[Download x64 rescue image · ISO](https://fastly-cdn.system-rescue.org/releases/13.03/systemrescue-13.03-amd64.iso)** · [All download options](https://www.system-rescue.org/Download/) · [Documentation](https://www.system-rescue.org/manual/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A ready-made [Linux](https://www.kernel.org/) recovery environment for an unbootable or damaged machine. |
| **Where it runs** | Bootable PC |
| **How you use it** | boot |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** Verify checksums/signatures and choose the intended disk before mounting or modifying filesystems.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://www.system-rescue.org/manual/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="ubuntu-live"></a>

## [Ubuntu Desktop live media](https://ubuntu.com/desktop)

Official [Ubuntu](https://ubuntu.com/) desktop installer with a live try-before-install workflow.

[Official page](https://ubuntu.com/desktop) · **[Download x64 desktop image · ISO](https://releases.ubuntu.com/26.04.1/ubuntu-26.04.1-desktop-amd64.iso)** · [All download options](https://ubuntu.com/download/desktop) · [Documentation](https://ubuntu.com/tutorials/try-ubuntu-before-you-install)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Hardware checks and file access from a familiar [Linux](https://www.kernel.org/) desktop. |
| **Where it runs** | Bootable PC |
| **How you use it** | boot |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** Choose the live session when diagnosing; installing is a separate disk-changing action.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://ubuntu.com/tutorials/try-ubuntu-before-you-install)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="ventoy"></a>

## [Ventoy](https://www.ventoy.net/)

Boot menu for multiple ISO and other image files copied onto prepared media.

[Official page](https://www.ventoy.net/) · **[Download Windows package · ZIP](https://github.com/ventoy/Ventoy/releases/download/v1.1.18/ventoy-1.1.18-windows.zip)** · [All download options](https://www.ventoy.net/en/download.html) · [Documentation](https://www.ventoy.net/en/doc_start.html)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Maintaining a technician USB with several independently updated rescue images. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/) |
| **How you use it** | portable |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** Initial installation formats the target. Updating [Ventoy](https://www.ventoy.net/) and replacing an ISO are separate operations; test boot compatibility.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://www.ventoy.net/en/doc_start.html)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="microsoft-windows11-media"></a>

## [Windows 11 installation media](https://www.microsoft.com/software-download/windows11)

Official Microsoft installation assistant, media creation and ISO download source.

[Official page](https://www.microsoft.com/software-download/windows11) · **[Download media creation tool · EXE](https://go.microsoft.com/fwlink/?linkid=2156295)** · [Documentation](https://www.microsoft.com/software-download/windows11)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A clean installation or supported repair using the correct [Windows](https://www.microsoft.com/windows) edition and architecture. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), Bootable PC |
| **How you use it** | boot |
| **Access / pricing** | Download available; [Windows](https://www.microsoft.com/windows) license required |
| **Selection role** | Core |

> **Before using it:** An installation can erase data. Match edition, language and architecture and preserve the recovery key.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://www.microsoft.com/software-download/windows11)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
