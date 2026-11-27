[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

<a id="-storage-recovery--backup"></a>

# Storage, recovery & backup

Keep the source safe, select the right copy or recovery operation, and test the restored result.

**20 tools** · [All categories](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Selection standards](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards)

**Selection question:** Healthy migration, missing files, or failing media?

<a id="-start-with-a-workflow"></a>

## Related playbooks

- **[💾 Recover data](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive)** · **[Open working view](https://herbertofury.github.io/INTA/#job/failing-drive)**
- **[💽 Clone or migrate a disk](https://github.com/Herbertofury/INTA/wiki/Job-disk-migration)** · **[Open working view](https://herbertofury.github.io/INTA/#job/disk-migration)**
- **[🗄️ Test a backup restore](https://github.com/Herbertofury/INTA/wiki/Job-backup-restore)** · **[Open working view](https://herbertofury.github.io/INTA/#job/backup-restore)**

<a id="compare-options"></a>

<a id="️-compare-the-options"></a>
<a id="-compare-the-options"></a>

## Tool comparison

| Tool | Choose it for | Platforms / role | Official page + download |
| --- | --- | --- | --- |
| [Clonezilla](#clonezilla) | Capturing or restoring a supported disk image with a repeatable procedure. | Bootable PC, [Linux](https://www.kernel.org/) · Core | [Official page](https://clonezilla.org/) · **[Get x64 rescue image · ISO](https://downloads.sourceforge.net/clonezilla/clonezilla-live-3.3.5-3-amd64.iso)** |
| [CrystalDiskInfo](#crystaldiskinfo) | First-pass HDD and SSD health inspection in [Windows](https://www.microsoft.com/windows). | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://crystalmark.info/en/software/crystaldiskinfo/) · **[Get Windows ZIP · 9.9.5](https://sourceforge.net/projects/crystaldiskinfo/files/9.9.5/CrystalDiskInfo9_9_5.zip/download)** |
| [FreeFileSync](#freefilesync) | Reviewed file migration when technicians need to see proposed differences. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) · Core | [Official page](https://freefilesync.org/) · **[Download Windows EXE · 14.13](https://freefilesync.org/download/FreeFileSync_14.13_Windows_Setup.exe)** |
| [GSmartControl](#gsmartcontrol) | A visual SMART investigation in the rescue desktop. | [Linux](https://www.kernel.org/), [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://gsmartcontrol.shaduri.dev/) · **[Download Windows x64 portable · ZIP](https://github.com/ashaduri/gsmartcontrol/releases/download/v2.0.2/gsmartcontrol-2.0.2-win64.zip)** |
| [Kopia](#kopia) | A visual backup workflow with repository policies and restore browsing. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) · Core | [Official page](https://kopia.io/) · **[Download Windows desktop · ZIP](https://github.com/kopia/kopia/releases/download/v0.23.1/KopiaUI-0.23.1-win.zip)** |
| [rclone](#rclone) | Cloud migrations, verifiable copies and controlled provider-to-provider transfers. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) · Core | [Official page](https://rclone.org/) · **[Download Windows x64 · ZIP](https://downloads.rclone.org/v1.76.0/rclone-v1.76.0-windows-amd64.zip)** |
| [Rescuezilla](#rescuezilla) | A visual bare-metal backup or restore workflow at the bench. | Bootable PC · Core | [Official page](https://rescuezilla.com/) · **[Download x64 rescue image · ISO](https://github.com/rescuezilla/rescuezilla/releases/download/2.6.2/rescuezilla-2.6.2-64bit.resolute.iso)** |
| [restic](#restic) | Repeatable file backups with explicit integrity and restore checks. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) · Core | [Official page](https://restic.net/) · **[Download Windows x64 · ZIP](https://github.com/restic/restic/releases/download/v0.20.0/restic_0.20.0_windows_amd64.zip)** |
| [Robocopy](#robocopy) | A documented migration between accessible [Windows](https://www.microsoft.com/windows) folders or shares. | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy) · **[Open built-in copy instructions](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy)** |
| [rsync](#rsync) | A logged file copy from [Linux](https://www.kernel.org/) rescue to a separate destination. | [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos), BSD · Core | [Official page](https://rsync.samba.org/) · **[Download source · TAR.GZ](https://download.samba.org/pub/rsync/rsync-3.5.2.tar.gz)** |
| [smartmontools](#smartmontools) | Reading SMART and NVMe health from a rescue environment or supported host OS. | [Linux](https://www.kernel.org/), [Windows](https://www.microsoft.com/windows), [macOS](https://support.apple.com/macos) · Core | [Official page](https://www.smartmontools.org/) · **[Download Windows setup · EXE](https://github.com/smartmontools/smartmontools/releases/download/RELEASE_7_5/smartmontools-7.5.win32-setup.exe)** |
| [Veeam Agent for Microsoft Windows](#veeam-agent) | Image backup and recovery media for a supported [Windows](https://www.microsoft.com/windows) endpoint. | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://www.veeam.com/products/free/microsoft-windows) · **[Sign in for the Windows download](https://www.veeam.com/products/physical/windows-backup-recovery-download.html)** |
| [cryptsetup](#cryptsetup) | Opening a supported encrypted volume using the correct recovery material. | [Linux](https://www.kernel.org/) · Specialist | [Official page](https://gitlab.com/cryptsetup/cryptsetup) · **[Download source · TAR.XZ](https://www.kernel.org/pub/linux/utils/cryptsetup/v2.8/cryptsetup-2.8.9.tar.xz)** |
| [Dislocker](#dislocker) | Reading an encrypted client volume with its valid key or recovery material. | [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) · Specialist | [Official page](https://github.com/Aorimn/dislocker) · **[Download source · ZIP](https://api.github.com/repos/Aorimn/dislocker/zipball/refs/tags/v0.7.4)** |
| [DMDE](#dmde) | Complex partition and file recovery where a specialist needs detailed filesystem views. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) · Specialist | [Official page](https://dmde.com/) · **[Download Windows x64 GUI · ZIP](https://dmde.com/download/dmde-4-4-4-842-win64-gui.zip)** |
| [GNU ddrescue](#ddrescue) | Creating an image of a failing disk before recovery or repair attempts. | [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) · Specialist | [Official page](https://www.gnu.org/software/ddrescue/) · **[Download source · TAR.LZ](https://ftp.gnu.org/gnu/ddrescue/ddrescue-1.30.tar.lz)** |
| [GParted](#gparted) | A planned partition resize or layout change after backup and health checks. | [Linux](https://www.kernel.org/), Bootable PC · Specialist | [Official page](https://gparted.org/) · **[Get x64 live ISO · download page](https://sourceforge.net/projects/gparted/files/gparted-live-stable/1.8.1-6/gparted-live-1.8.1-6-amd64.iso/download)** |
| [nvme-cli](#nvme-cli) | Reading NVMe identity and health information from [INTA Rescue](https://github.com/Herbertofury/INTA/blob/main/docs/rescue.md). | [Linux](https://www.kernel.org/) · Specialist | [Official page](https://github.com/linux-nvme/nvme-cli) · **[Download Linux x64 binary](https://github.com/linux-nvme/nvme-cli/releases/download/v3.3/nvme-cli-v3.3-x86_64)** |
| [PhotoRec](#photorec) | Recovering files from an image when directory-based recovery is unsuccessful. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) · Specialist | [Official page](https://www.cgsecurity.org/wiki/PhotoRec) · **[Download PhotoRec / TestDisk · ZIP](https://www.cgsecurity.org/testdisk-7.2.win64.zip)** |
| [TestDisk](#testdisk) | Investigating missing partitions on an image or safely copied disk. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) · Specialist | [Official page](https://www.cgsecurity.org/wiki/TestDisk) · **[Download Windows x64 tools · ZIP](https://www.cgsecurity.org/testdisk-7.2.win64.zip)** |

Core means an everyday candidate; Specialist means a focused advanced use; Evaluate means prove it in a suitable test environment before promotion.

<a id="clonezilla"></a>

## [Clonezilla](https://clonezilla.org/)

Bootable disk and partition imaging with multiple deployment modes.

[Official page](https://clonezilla.org/) · **[Get x64 rescue image · ISO](https://downloads.sourceforge.net/clonezilla/clonezilla-live-3.3.5-3-amd64.iso)** · [All download options](https://clonezilla.org/downloads.php) · [Documentation](https://clonezilla.org/clonezilla-live-doc.php)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Capturing or restoring a supported disk image with a repeatable procedure. |
| **Where it runs** | Bootable PC, [Linux](https://www.kernel.org/) |
| **How you use it** | boot |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** Source and target selection must be verified; check filesystem and target-size constraints for the chosen operation.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://clonezilla.org/clonezilla-live-doc.php)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/INTA/blob/1ecba1129aac275a02f40b9c4f5bea438c9e2688/live/package-lists/inta.list.chroot

</details>

[Back to comparison](#compare-options)

<a id="crystaldiskinfo"></a>

## [CrystalDiskInfo](https://crystalmark.info/en/software/crystaldiskinfo/)

Displays supported drive health indicators, temperatures and SMART attributes.

[Official page](https://crystalmark.info/en/software/crystaldiskinfo/) · **[Get Windows ZIP · 9.9.5](https://sourceforge.net/projects/crystaldiskinfo/files/9.9.5/CrystalDiskInfo9_9_5.zip/download)** · [All download options](https://crystalmark.info/en/download/) · [Documentation](https://crystalmark.info/en/software/crystaldiskinfo/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | First-pass HDD and SSD health inspection in [Windows](https://www.microsoft.com/windows). |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | portable |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** SMART is incomplete through some USB bridges and cannot rule out impending failure.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://crystalmark.info/en/software/crystaldiskinfo/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json

</details>

[Back to comparison](#compare-options)

<a id="freefilesync"></a>

## [FreeFileSync](https://freefilesync.org/)

Visual folder comparison and synchronization with configurable directions.

[Official page](https://freefilesync.org/) · **[Download Windows EXE · 14.13](https://freefilesync.org/download/FreeFileSync_14.13_Windows_Setup.exe)** · [Download macOS ZIP · 14.13](https://freefilesync.org/download/FreeFileSync_14.13_macOS.zip) · [Download Linux x64 TAR.GZ · 14.13](https://freefilesync.org/download/FreeFileSync_14.13_Linux_x86_64.tar.gz) · [All download options](https://freefilesync.org/download.php) · [Documentation](https://freefilesync.org/faq.php)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Reviewed file migration when technicians need to see proposed differences. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | installed |
| **Access / pricing** | Business Edition for commercial use |
| **Selection role** | Core |

> **Before using it:** Choose the intended direction and preview deletions. Synchronization alone does not retain independent history.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://freefilesync.org/faq.php)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json

</details>

[Back to comparison](#compare-options)

<a id="gsmartcontrol"></a>

## [GSmartControl](https://gsmartcontrol.shaduri.dev/)

Graphical frontend for drive health inspection and smartctl self-tests.

[Official page](https://gsmartcontrol.shaduri.dev/) · **[Download Windows x64 portable · ZIP](https://github.com/ashaduri/gsmartcontrol/releases/download/v2.0.2/gsmartcontrol-2.0.2-win64.zip)** · [All download options](https://gsmartcontrol.shaduri.dev/downloads) · [Documentation](https://gsmartcontrol.shaduri.dev/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A visual SMART investigation in the rescue desktop. |
| **Where it runs** | [Linux](https://www.kernel.org/), [Windows](https://www.microsoft.com/windows) |
| **How you use it** | installed |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** Health reads may not pass through every USB adapter. Avoid lengthy self-tests on a failing drive.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://gsmartcontrol.shaduri.dev/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/INTA/blob/1ecba1129aac275a02f40b9c4f5bea438c9e2688/live/package-lists/inta.list.chroot

</details>

[Back to comparison](#compare-options)

<a id="kopia"></a>

## [Kopia](https://kopia.io/)

Encrypted snapshot backups with both desktop and command-line interfaces.

[Official page](https://kopia.io/) · **[Download Windows desktop · ZIP](https://github.com/kopia/kopia/releases/download/v0.23.1/KopiaUI-0.23.1-win.zip)** · [All download options](https://kopia.io/docs/installation/) · [Documentation](https://kopia.io/docs/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A visual backup workflow with repository policies and restore browsing. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | installed |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** Record credentials and verify a restore before relying on the repository.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://kopia.io/docs/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json

</details>

[Back to comparison](#compare-options)

<a id="rclone"></a>

## [rclone](https://rclone.org/)

Transfers, checks and synchronizes files across many storage providers.

[Official page](https://rclone.org/) · **[Download Windows x64 · ZIP](https://downloads.rclone.org/v1.76.0/rclone-v1.76.0-windows-amd64.zip)** · [All download options](https://rclone.org/downloads/) · [Documentation](https://rclone.org/docs/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Cloud migrations, verifiable copies and controlled provider-to-provider transfers. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | portable |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** Start with copy and dry-run where applicable. sync can remove destination files and is not versioned backup by itself.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://rclone.org/docs/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json

</details>

[Back to comparison](#compare-options)

<a id="rescuezilla"></a>

## [Rescuezilla](https://rescuezilla.com/)

Bootable graphical disk-imaging and recovery environment compatible with [Clonezilla](https://clonezilla.org/) images.

[Official page](https://rescuezilla.com/) · **[Download x64 rescue image · ISO](https://github.com/rescuezilla/rescuezilla/releases/download/2.6.2/rescuezilla-2.6.2-64bit.resolute.iso)** · [All download options](https://rescuezilla.com/download) · [Documentation](https://rescuezilla.com/help)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A visual bare-metal backup or restore workflow at the bench. |
| **Where it runs** | Bootable PC |
| **How you use it** | boot |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** Verify the destination model, serial and capacity before a restore overwrites it.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://rescuezilla.com/help)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="restic"></a>

## [restic](https://restic.net/)

Encrypted, deduplicating file-backup snapshots across local and remote storage.

[Official page](https://restic.net/) · **[Download Windows x64 · ZIP](https://github.com/restic/restic/releases/download/v0.20.0/restic_0.20.0_windows_amd64.zip)** · [All download options](https://github.com/restic/restic/releases/latest) · [Documentation](https://restic.readthedocs.io/en/stable/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Repeatable file backups with explicit integrity and restore checks. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | portable |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** Keep the repository password recoverable and test restoration of real files. A snapshot is not a full bare-metal image.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://restic.readthedocs.io/en/stable/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json

</details>

[Back to comparison](#compare-options)

<a id="robocopy"></a>

## [Robocopy](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy)

[Windows](https://www.microsoft.com/windows) file-copy utility with restart, retry, metadata and logging controls.

[Official page](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy) · **[Open built-in copy instructions](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy)** · [Documentation](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A documented migration between accessible [Windows](https://www.microsoft.com/windows) folders or shares. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | built-in |
| **Access / pricing** | Included with [Windows](https://www.microsoft.com/windows) |
| **Selection role** | Core |

> **Before using it:** Mirror and purge options delete destination content. Check exit-code semantics and verify the copied data.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="rsync"></a>

## [rsync](https://rsync.samba.org/)

Incremental file transfer with explicit preservation and comparison controls.

[Official page](https://rsync.samba.org/) · **[Download source · TAR.GZ](https://download.samba.org/pub/rsync/rsync-3.5.2.tar.gz)** · [All download options](https://rsync.samba.org/download.html) · [Documentation](https://download.samba.org/pub/rsync/rsync.1)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A logged file copy from [Linux](https://www.kernel.org/) rescue to a separate destination. |
| **Where it runs** | [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos), BSD |
| **How you use it** | installed |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** Check trailing-slash semantics and avoid delete options unless deletion is part of the reviewed plan.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://download.samba.org/pub/rsync/rsync.1)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/INTA/blob/1ecba1129aac275a02f40b9c4f5bea438c9e2688/live/package-lists/inta.list.chroot

</details>

[Back to comparison](#compare-options)

<a id="smartmontools"></a>

## [smartmontools](https://www.smartmontools.org/)

smartctl and smartd tools for drive health information and supported self-tests.

[Official page](https://www.smartmontools.org/) · **[Download Windows setup · EXE](https://github.com/smartmontools/smartmontools/releases/download/RELEASE_7_5/smartmontools-7.5.win32-setup.exe)** · [All download options](https://github.com/smartmontools/smartmontools/releases) · [Documentation](https://github.com/smartmontools/smartmontools)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Reading SMART and NVMe health from a rescue environment or supported host OS. |
| **Where it runs** | [Linux](https://www.kernel.org/), [Windows](https://www.microsoft.com/windows), [macOS](https://support.apple.com/macos) |
| **How you use it** | installed |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** Read health before long tests; if a drive is failing, preserve data before stressing it.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://github.com/smartmontools/smartmontools)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/INTA/blob/1ecba1129aac275a02f40b9c4f5bea438c9e2688/live/package-lists/inta.list.chroot

</details>

[Back to comparison](#compare-options)

<a id="veeam-agent"></a>

## [Veeam Agent for Microsoft Windows](https://www.veeam.com/products/free/microsoft-windows)

[Windows](https://www.microsoft.com/windows) backup with file, volume and full-machine recovery options.

[Official page](https://www.veeam.com/products/free/microsoft-windows) · **[Sign in for the Windows download](https://www.veeam.com/products/physical/windows-backup-recovery-download.html)** · [Documentation](https://helpcenter.veeam.com/docs/agentforwindows/userguide/overview.html)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Image backup and recovery media for a supported [Windows](https://www.microsoft.com/windows) endpoint. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | installed |
| **Access / pricing** | Free and paid editions; service-provider terms differ |
| **Selection role** | Core |

> **Before using it:** Use the licensing appropriate to managed client services; test recovery media and a restore path.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://helpcenter.veeam.com/docs/agentforwindows/userguide/overview.html)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="cryptsetup"></a>

## [cryptsetup](https://gitlab.com/cryptsetup/cryptsetup)

[Linux](https://www.kernel.org/) tooling for encrypted-volume management and supported unlock workflows.

[Official page](https://gitlab.com/cryptsetup/cryptsetup) · **[Download source · TAR.XZ](https://www.kernel.org/pub/linux/utils/cryptsetup/v2.8/cryptsetup-2.8.9.tar.xz)** · [All download options](https://gitlab.com/cryptsetup/cryptsetup/-/releases) · [Documentation](https://gitlab.com/cryptsetup/cryptsetup)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Opening a supported encrypted volume using the correct recovery material. |
| **Where it runs** | [Linux](https://www.kernel.org/) |
| **How you use it** | installed |
| **Access / pricing** | Free / open source |
| **Selection role** | Specialist |

> **Before using it:** Unlocking and formatting are different operations. Never format or alter headers as a troubleshooting shortcut.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://gitlab.com/cryptsetup/cryptsetup)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/INTA/blob/1ecba1129aac275a02f40b9c4f5bea438c9e2688/live/package-lists/inta.list.chroot

</details>

[Back to comparison](#compare-options)

<a id="dislocker"></a>

## [Dislocker](https://github.com/Aorimn/dislocker)

Access to supported [BitLocker](https://learn.microsoft.com/en-us/windows/security/operating-system-security/data-protection/bitlocker/)-encrypted volumes from a non-[Windows](https://www.microsoft.com/windows) environment.

[Official page](https://github.com/Aorimn/dislocker) · **[Download source · ZIP](https://api.github.com/repos/Aorimn/dislocker/zipball/refs/tags/v0.7.4)** · [All downloads / setup](https://github.com/Aorimn/dislocker/releases) · [Documentation](https://github.com/Aorimn/dislocker)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Reading an encrypted client volume with its valid key or recovery material. |
| **Where it runs** | [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | installed |
| **Access / pricing** | Free / open source |
| **Selection role** | Specialist |

> **Before using it:** Use read-only access for recovery when possible. It does not bypass the need for authorized key material.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://github.com/Aorimn/dislocker)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/INTA/blob/1ecba1129aac275a02f40b9c4f5bea438c9e2688/live/package-lists/inta.list.chroot

</details>

[Back to comparison](#compare-options)

<a id="dmde"></a>

## [DMDE](https://dmde.com/)

Disk inspection, filesystem reconstruction and [data recovery](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive) software.

[Official page](https://dmde.com/) · **[Download Windows x64 GUI · ZIP](https://dmde.com/download/dmde-4-4-4-842-win64-gui.zip)** · [All download options](https://dmde.com/download.html) · [Documentation](https://dmde.com/manual.html)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Complex partition and file recovery where a specialist needs detailed filesystem views. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | portable |
| **Access / pricing** | Free edition with limits; paid editions |
| **Selection role** | Specialist |

> **Before using it:** Prefer read-only investigation on an image. Disk-editor writes require a verified plan.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://dmde.com/manual.html)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="ddrescue"></a>

## [GNU ddrescue](https://www.gnu.org/software/ddrescue/)

Copies readable areas from damaged media and records progress in a resumable mapfile.

[Official page](https://www.gnu.org/software/ddrescue/) · **[Download source · TAR.LZ](https://ftp.gnu.org/gnu/ddrescue/ddrescue-1.30.tar.lz)** · [All download options](https://ftp.gnu.org/gnu/ddrescue/) · [Documentation](https://www.gnu.org/software/ddrescue/manual/ddrescue_manual.html)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Creating an image of a failing disk before recovery or repair attempts. |
| **Where it runs** | [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | installed |
| **Access / pricing** | Free / open source |
| **Selection role** | Specialist |

> **Before using it:** Source and destination direction is critical. Keep the mapfile and never recover files back onto the failing source.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://www.gnu.org/software/ddrescue/manual/ddrescue_manual.html)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/INTA/blob/1ecba1129aac275a02f40b9c4f5bea438c9e2688/live/package-lists/inta.list.chroot

</details>

[Back to comparison](#compare-options)

<a id="gparted"></a>

## [GParted](https://gparted.org/)

Graphical partition editor available as a [Linux](https://www.kernel.org/) application and bootable live image.

[Official page](https://gparted.org/) · **[Get x64 live ISO · download page](https://sourceforge.net/projects/gparted/files/gparted-live-stable/1.8.1-6/gparted-live-1.8.1-6-amd64.iso/download)** · [All download options](https://gparted.org/download.php) · [Documentation](https://gparted.org/documentation.php)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A planned partition resize or layout change after backup and health checks. |
| **Where it runs** | [Linux](https://www.kernel.org/), Bootable PC |
| **How you use it** | boot |
| **Access / pricing** | Free / open source |
| **Selection role** | Specialist |

> **Before using it:** Partition moves and resizing can lose data if interrupted; confirm a recoverable backup and stable power.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://gparted.org/documentation.php)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/INTA/blob/1ecba1129aac275a02f40b9c4f5bea438c9e2688/live/package-lists/inta.list.chroot

</details>

[Back to comparison](#compare-options)

<a id="nvme-cli"></a>

## [nvme-cli](https://github.com/linux-nvme/nvme-cli)

[Linux](https://www.kernel.org/) NVMe controller and namespace inspection utilities.

[Official page](https://github.com/linux-nvme/nvme-cli) · **[Download Linux x64 binary](https://github.com/linux-nvme/nvme-cli/releases/download/v3.3/nvme-cli-v3.3-x86_64)** · [All download options](https://github.com/linux-nvme/nvme-cli/releases/latest) · [Documentation](https://github.com/linux-nvme/nvme-cli)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Reading NVMe identity and health information from [INTA Rescue](https://github.com/Herbertofury/INTA/blob/main/docs/rescue.md). |
| **Where it runs** | [Linux](https://www.kernel.org/) |
| **How you use it** | installed |
| **Access / pricing** | Free / open source |
| **Selection role** | Specialist |

> **Before using it:** Inventory and health commands differ from format, sanitize and firmware commands; keep those write operations separate.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://github.com/linux-nvme/nvme-cli)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/INTA/blob/1ecba1129aac275a02f40b9c4f5bea438c9e2688/live/package-lists/inta.list.chroot

</details>

[Back to comparison](#compare-options)

<a id="photorec"></a>

## [PhotoRec](https://www.cgsecurity.org/wiki/PhotoRec)

File carving that can recover supported file content when filesystem metadata is unavailable.

[Official page](https://www.cgsecurity.org/wiki/PhotoRec) · **[Download PhotoRec / TestDisk · ZIP](https://www.cgsecurity.org/testdisk-7.2.win64.zip)** · [All download options](https://www.cgsecurity.org/wiki/TestDisk_Download) · [Documentation](https://www.cgsecurity.org/wiki/PhotoRec_Step_By_Step)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Recovering files from an image when directory-based recovery is unsuccessful. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | portable |
| **Access / pricing** | Free / open source |
| **Selection role** | Specialist |

> **Before using it:** Recovered filenames and folder structure may be lost; write results to a different disk.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://www.cgsecurity.org/wiki/PhotoRec_Step_By_Step)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/INTA/blob/1ecba1129aac275a02f40b9c4f5bea438c9e2688/live/package-lists/inta.list.chroot

</details>

[Back to comparison](#compare-options)

<a id="testdisk"></a>

## [TestDisk](https://www.cgsecurity.org/wiki/TestDisk)

Partition and filesystem recovery tool for diagnosing lost partitions and recovering some files.

[Official page](https://www.cgsecurity.org/wiki/TestDisk) · **[Download Windows x64 tools · ZIP](https://www.cgsecurity.org/testdisk-7.2.win64.zip)** · [All download options](https://www.cgsecurity.org/wiki/TestDisk_Download) · [Documentation](https://www.cgsecurity.org/wiki/TestDisk_Step_By_Step)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Investigating missing partitions on an image or safely copied disk. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | portable |
| **Access / pricing** | Free / open source |
| **Selection role** | Specialist |

> **Before using it:** Image first when data matters; writing a partition table changes the evidence and can worsen an incorrect diagnosis.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://www.cgsecurity.org/wiki/TestDisk_Step_By_Step)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/INTA/blob/1ecba1129aac275a02f40b9c4f5bea438c9e2688/live/package-lists/inta.list.chroot

</details>

[Back to comparison](#compare-options)


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
