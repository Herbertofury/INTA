[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

<a id="-technician-productivity"></a>

# Technician productivity

Make everyday technician work clearer without leaving unnecessary software on customer machines.

**9 tools** · [All categories](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Selection standards](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards)

**Selection question:** Inspect, document, transfer, or improve your own workspace?

<a id="-start-with-a-workflow"></a>

## Related playbooks

- [Set up a dependable new workstation](https://github.com/Herbertofury/INTA/wiki/Job-new-pc)
- [Run a useful workstation maintenance visit](https://github.com/Herbertofury/INTA/wiki/Job-preventive-maintenance)
- [Close a repair with a clear client handoff](https://github.com/Herbertofury/INTA/wiki/Job-client-handoff)

<a id="compare-options"></a>

<a id="️-compare-the-options"></a>
<a id="-compare-the-options"></a>

## Tool comparison

| Tool | Choose it for | Platforms / role | Official page + download |
| --- | --- | --- | --- |
| [7-Zip](#7zip) | Packaging logs or extracting a trusted portable tool. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/) · Core | [Official page](https://www.7-zip.org/) · **[Download Windows x64 EXE](https://github.com/ip7z/7zip/releases/download/26.06/7z2606-x64.exe)** |
| [Everything](#everything) | Locating files by name on a technician workstation. | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://www.voidtools.com/) · **[Download Windows x64 EXE](https://www.voidtools.com/Everything-1.4.1.1032.x64-Setup.exe)** |
| [KeePassXC](#keepassxc) | Keeping a protected technician vault with a deliberate sharing and backup process. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) · Core | [Official page](https://keepassxc.org/) · **[Download Windows x64 portable · ZIP](https://github.com/keepassxreboot/keepassxc/releases/download/2.8.0/KeePassXC-2.8.0-x64.zip)** |
| [LocalSend](#localsend) | Moving a selected log or tool between trusted devices on the same network. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos), Android, iOS · Core | [Official page](https://localsend.org/) · **[Download Windows x64 portable · ZIP](https://github.com/localsend/localsend/releases/download/v1.19.0/LocalSend-1.19.0-windows-x86-64.zip)** |
| [Microsoft PowerToys](#powertoys) | Making the technician workstation easier to navigate and operate. | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://learn.microsoft.com/en-us/windows/powertoys/) · **[Download Windows x64 setup · EXE](https://github.com/microsoft/PowerToys/releases/download/v0.102.3101/PowerToysUserSetup-0.102.3101.0-x64.exe)** |
| [Notepad++](#notepadplusplus) | Reading logs and making deliberate configuration-file changes. | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://notepad-plus-plus.org/) · **[Download Windows x64 portable · ZIP](https://github.com/notepad-plus-plus/notepad-plus-plus/releases/download/v8.9.9.1/npp.8.9.9.1.portable.x64.zip)** |
| [ShareX](#sharex) | Capturing a problem and producing clear technician documentation. | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://getsharex.com/) · **[Download Windows x64 portable · ZIP](https://github.com/ShareX/ShareX/releases/download/v22.4.0/ShareX-22.4.0-portable-x64.zip)** |
| [UniGetUI](#unigetui) | Reviewing available applications and updates in one technician interface. | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://github.com/Devolutions/UniGetUI) · **[Download Windows x64 portable · ZIP](https://github.com/Devolutions/UniGetUI/releases/download/v2026.3.4/UniGetUI.x64.zip)** |
| [WizFile](#wizfile) | Locating a known file, log or recently changed group of files without guessing its folder. | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://antibody-software.com/wizfile/) · **[Download Windows portable · ZIP](https://antibody-software.com/files/wizfile_3_15_portable.zip)** |

Core means an everyday candidate; Specialist means a focused advanced use; Evaluate means prove it in a suitable test environment before promotion.

<a id="7zip"></a>

## [7-Zip](https://www.7-zip.org/)

Archive creation, extraction and integrity testing for common archive formats.

[Official page](https://www.7-zip.org/) · **[Download Windows x64 EXE](https://github.com/ip7z/7zip/releases/download/26.06/7z2606-x64.exe)** · [Download Windows ARM64 EXE](https://github.com/ip7z/7zip/releases/download/26.06/7z2606-arm64.exe) · [Other platforms and packages](https://www.7-zip.org/download.html) · [Documentation](https://www.7-zip.org/faq.html)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Packaging logs or extracting a trusted portable tool. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/) |
| **How you use it** | installed |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** Extract untrusted archives to an isolated directory and review their contents before execution.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://www.7-zip.org/faq.html)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json

</details>

[Back to comparison](#compare-options)

<a id="everything"></a>

## [Everything](https://www.voidtools.com/)

Fast indexed filename search on supported [Windows](https://www.microsoft.com/windows) filesystems.

[Official page](https://www.voidtools.com/) · **[Download Windows x64 EXE](https://www.voidtools.com/Everything-1.4.1.1032.x64-Setup.exe)** · [Portable Windows x64 ZIP](https://www.voidtools.com/Everything-1.4.1.1032.x64.zip) · [Other Windows packages](https://www.voidtools.com/downloads/) · [Documentation](https://www.voidtools.com/support/everything/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Locating files by name on a technician workstation. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | installed |
| **Access / pricing** | Freeware |
| **Selection role** | Core |

> **Before using it:** The index and optional service are persistent components; avoid indexing sensitive client data unnecessarily.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://www.voidtools.com/support/everything/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json

</details>

[Back to comparison](#compare-options)

<a id="keepassxc"></a>

## [KeePassXC](https://keepassxc.org/)

Local encrypted password database with cross-platform clients.

[Official page](https://keepassxc.org/) · **[Download Windows x64 portable · ZIP](https://github.com/keepassxreboot/keepassxc/releases/download/2.8.0/KeePassXC-2.8.0-x64.zip)** · [All download options](https://keepassxc.org/download/) · [Documentation](https://keepassxc.org/docs/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Keeping a protected technician vault with a deliberate sharing and backup process. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | installed |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** Store the database and recovery material outside the public repository; control who receives a copy.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://keepassxc.org/docs/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json

</details>

[Back to comparison](#compare-options)

<a id="localsend"></a>

## [LocalSend](https://localsend.org/)

Cross-platform local-network file transfer without a cloud account.

[Official page](https://localsend.org/) · **[Download Windows x64 portable · ZIP](https://github.com/localsend/localsend/releases/download/v1.19.0/LocalSend-1.19.0-windows-x86-64.zip)** · [All download options](https://localsend.org/download) · [Documentation](https://github.com/localsend/localsend)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Moving a selected log or tool between trusted devices on the same network. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos), Android, iOS |
| **How you use it** | installed |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** Confirm the recipient device and network; local transfer is not backup.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://github.com/localsend/localsend)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json

</details>

[Back to comparison](#compare-options)

<a id="powertoys"></a>

## [Microsoft PowerToys](https://learn.microsoft.com/en-us/windows/powertoys/)

Modular [Windows](https://www.microsoft.com/windows) utilities including window layout, file naming and input helpers.

[Official page](https://learn.microsoft.com/en-us/windows/powertoys/) · **[Download Windows x64 setup · EXE](https://github.com/microsoft/PowerToys/releases/download/v0.102.3101/PowerToysUserSetup-0.102.3101.0-x64.exe)** · [All download options](https://github.com/microsoft/PowerToys/releases/latest) · [Documentation](https://learn.microsoft.com/en-us/windows/powertoys/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Making the technician workstation easier to navigate and operate. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | installed |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** Enable only the modules needed; client machines do not need a permanent utility suite for a one-time job.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://learn.microsoft.com/en-us/windows/powertoys/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json

</details>

[Back to comparison](#compare-options)

<a id="notepadplusplus"></a>

## [Notepad++](https://notepad-plus-plus.org/)

[Windows](https://www.microsoft.com/windows) text editor with searching, syntax support and large text-file workflows.

[Official page](https://notepad-plus-plus.org/) · **[Download Windows x64 portable · ZIP](https://github.com/notepad-plus-plus/notepad-plus-plus/releases/download/v8.9.9.1/npp.8.9.9.1.portable.x64.zip)** · [All download options](https://notepad-plus-plus.org/downloads/) · [Documentation](https://npp-user-manual.org/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Reading logs and making deliberate configuration-file changes. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | portable |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://github.com/notepad-plus-plus/notepad-plus-plus)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json

</details>

[Back to comparison](#compare-options)

<a id="sharex"></a>

## [ShareX](https://getsharex.com/)

Screenshots, recordings and configurable local output workflows.

[Official page](https://getsharex.com/) · **[Download Windows x64 portable · ZIP](https://github.com/ShareX/ShareX/releases/download/v22.4.0/ShareX-22.4.0-portable-x64.zip)** · [All download options](https://github.com/ShareX/ShareX/releases/latest) · [Documentation](https://github.com/ShareX/ShareX)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Capturing a problem and producing clear technician documentation. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | installed |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** Choose local saving and review upload actions before capturing client screens.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://github.com/ShareX/ShareX)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json

</details>

[Back to comparison](#compare-options)

<a id="unigetui"></a>

## [UniGetUI](https://github.com/Devolutions/UniGetUI)

Graphical management interface for multiple software package managers.

[Official page](https://github.com/Devolutions/UniGetUI) · **[Download Windows x64 portable · ZIP](https://github.com/Devolutions/UniGetUI/releases/download/v2026.3.4/UniGetUI.x64.zip)** · [All download options](https://github.com/Devolutions/UniGetUI/releases/latest) · [Documentation](https://github.com/Devolutions/UniGetUI)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Reviewing available applications and updates in one technician interface. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | installed |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** Package-source trust and application licensing still depend on each upstream package.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://github.com/Devolutions/UniGetUI)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json

</details>

[Back to comparison](#compare-options)

<a id="wizfile"></a>

## [WizFile](https://antibody-software.com/wizfile/)

Fast filename and path lookup with size, date and other filters on [Windows](https://www.microsoft.com/windows).

[Official page](https://antibody-software.com/wizfile/) · **[Download Windows portable · ZIP](https://antibody-software.com/files/wizfile_3_15_portable.zip)** · [All download options](https://antibody-software.com/wizfile/download) · [Documentation](https://antibody-software.com/wizfile/quick-start)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Locating a known file, log or recently changed group of files without guessing its folder. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | portable / installed |
| **Access / pricing** | Free for personal use; commercial supporter or enterprise license required by business site and staff size. |
| **Selection role** | Core |

> **Before using it:** Search results do not establish that a file is safe or disposable. The close button can leave monitoring active in the tray; use File > Exit for a finished one-time job and remove any case-only startup setting. Consult the current EULA for commercial scope; older FAQ wording differs.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://antibody-software.com/wizfile/eula)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - Explicit INTA user request
  - https://antibody-software.com/wizfile/download
  - https://antibody-software.com/wizfile/eula
  - https://antibody-software.com/wizfile/quick-start

</details>

[Back to comparison](#compare-options)


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
