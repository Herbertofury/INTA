[Open the technician hub](https://herbertofury.github.io/INTA/) · [Wiki home](https://github.com/Herbertofury/INTA/wiki) · [Source repository](https://github.com/Herbertofury/INTA)

# Storage, recovery & backup

20 purpose-selected tools. Use the official download link for the current publisher offering; check architecture and release notes before installing.

| Tool | Best use | Platform | Kit level |
| --- | --- | --- | --- |
| [CrystalDiskInfo](#crystaldiskinfo) | First-pass HDD and SSD health inspection in Windows. | Windows | Core |
| [smartmontools](#smartmontools) | Reading SMART and NVMe health from a rescue environment or supported host OS. | Linux, Windows, macOS | Core |
| [GNU ddrescue](#ddrescue) | Creating an image of a failing disk before recovery or repair attempts. | Linux, macOS | Specialist |
| [TestDisk](#testdisk) | Investigating missing partitions on an image or safely copied disk. | Windows, Linux, macOS | Specialist |
| [PhotoRec](#photorec) | Recovering files from an image when directory-based recovery is unsuccessful. | Windows, Linux, macOS | Specialist |
| [DMDE](#dmde) | Complex partition and file recovery where a specialist needs detailed filesystem views. | Windows, Linux, macOS | Specialist |
| [GParted](#gparted) | A planned partition resize or layout change after backup and health checks. | Linux, Bootable PC | Specialist |
| [Rescuezilla](#rescuezilla) | A visual bare-metal backup or restore workflow at the bench. | Bootable PC | Core |
| [Clonezilla](#clonezilla) | Capturing or restoring a supported disk image with a repeatable procedure. | Bootable PC, Linux | Core |
| [restic](#restic) | Repeatable file backups with explicit integrity and restore checks. | Windows, Linux, macOS | Core |
| [Kopia](#kopia) | A visual backup workflow with repository policies and restore browsing. | Windows, Linux, macOS | Core |
| [rclone](#rclone) | Cloud migrations, verifiable copies and controlled provider-to-provider transfers. | Windows, Linux, macOS | Core |
| [Veeam Agent for Microsoft Windows](#veeam-agent) | Image backup and recovery media for a supported Windows endpoint. | Windows | Core |
| [FreeFileSync](#freefilesync) | Reviewed file migration when technicians need to see proposed differences. | Windows, Linux, macOS | Core |
| [Robocopy](#robocopy) | A documented migration between accessible Windows folders or shares. | Windows | Core |
| [nvme-cli](#nvme-cli) | Reading NVMe identity and health information from INTA Rescue. | Linux | Specialist |
| [GSmartControl](#gsmartcontrol) | A visual SMART investigation in the rescue desktop. | Linux, Windows | Core |
| [cryptsetup](#cryptsetup) | Opening a supported encrypted volume using the correct recovery material. | Linux | Specialist |
| [Dislocker](#dislocker) | Reading an encrypted client volume with its valid key or recovery material. | Linux, macOS | Specialist |
| [rsync](#rsync) | A logged file copy from Linux rescue to a separate destination. | Linux, macOS, BSD | Core |

<a name="crystaldiskinfo"></a>

## CrystalDiskInfo

Displays supported drive health indicators, temperatures and SMART attributes.

**Choose it for:** First-pass HDD and SSD health inspection in Windows.

**Platforms:** Windows · **Delivery:** portable · **Kit level:** Core

**Access / pricing:** Free / open source

[Official download](https://crystalmark.info/en/download/) · [Documentation](https://crystalmark.info/en/software/crystaldiskinfo/) · [Publisher](https://crystalmark.info/en/software/crystaldiskinfo/)

> **Before you use it:** SMART is incomplete through some USB bridges and cannot rule out impending failure.

**Source:** https://crystalmark.info/en/software/crystaldiskinfo/  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json


<a name="smartmontools"></a>

## smartmontools

smartctl and smartd tools for drive health information and supported self-tests.

**Choose it for:** Reading SMART and NVMe health from a rescue environment or supported host OS.

**Platforms:** Linux, Windows, macOS · **Delivery:** installed · **Kit level:** Core

**Access / pricing:** Free / open source

[Official download](https://github.com/smartmontools/smartmontools/releases) · [Documentation](https://github.com/smartmontools/smartmontools) · [Publisher](https://www.smartmontools.org/)

> **Before you use it:** Read health before long tests; if a drive is failing, preserve data before stressing it.

**Source:** https://github.com/smartmontools/smartmontools  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/INTA/blob/1ecba1129aac275a02f40b9c4f5bea438c9e2688/live/package-lists/inta.list.chroot


<a name="ddrescue"></a>

## GNU ddrescue

Copies readable areas from damaged media and records progress in a resumable mapfile.

**Choose it for:** Creating an image of a failing disk before recovery or repair attempts.

**Platforms:** Linux, macOS · **Delivery:** installed · **Kit level:** Specialist

**Access / pricing:** Free / open source

[Official download](https://ftp.gnu.org/gnu/ddrescue/) · [Documentation](https://www.gnu.org/software/ddrescue/manual/ddrescue_manual.html) · [Publisher](https://www.gnu.org/software/ddrescue/)

> **Before you use it:** Source and destination direction is critical. Keep the mapfile and never recover files back onto the failing source.

**Source:** https://www.gnu.org/software/ddrescue/manual/ddrescue_manual.html  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/INTA/blob/1ecba1129aac275a02f40b9c4f5bea438c9e2688/live/package-lists/inta.list.chroot


<a name="testdisk"></a>

## TestDisk

Partition and filesystem recovery tool for diagnosing lost partitions and recovering some files.

**Choose it for:** Investigating missing partitions on an image or safely copied disk.

**Platforms:** Windows, Linux, macOS · **Delivery:** portable · **Kit level:** Specialist

**Access / pricing:** Free / open source

[Official download](https://www.cgsecurity.org/wiki/TestDisk_Download) · [Documentation](https://www.cgsecurity.org/wiki/TestDisk_Step_By_Step) · [Publisher](https://www.cgsecurity.org/wiki/TestDisk)

> **Before you use it:** Image first when data matters; writing a partition table changes the evidence and can worsen an incorrect diagnosis.

**Source:** https://www.cgsecurity.org/wiki/TestDisk_Step_By_Step  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/INTA/blob/1ecba1129aac275a02f40b9c4f5bea438c9e2688/live/package-lists/inta.list.chroot


<a name="photorec"></a>

## PhotoRec

File carving that can recover supported file content when filesystem metadata is unavailable.

**Choose it for:** Recovering files from an image when directory-based recovery is unsuccessful.

**Platforms:** Windows, Linux, macOS · **Delivery:** portable · **Kit level:** Specialist

**Access / pricing:** Free / open source

[Official download](https://www.cgsecurity.org/wiki/TestDisk_Download) · [Documentation](https://www.cgsecurity.org/wiki/PhotoRec_Step_By_Step) · [Publisher](https://www.cgsecurity.org/wiki/PhotoRec)

> **Before you use it:** Recovered filenames and folder structure may be lost; write results to a different disk.

**Source:** https://www.cgsecurity.org/wiki/PhotoRec_Step_By_Step  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/INTA/blob/1ecba1129aac275a02f40b9c4f5bea438c9e2688/live/package-lists/inta.list.chroot


<a name="dmde"></a>

## DMDE

Disk inspection, filesystem reconstruction and data recovery software.

**Choose it for:** Complex partition and file recovery where a specialist needs detailed filesystem views.

**Platforms:** Windows, Linux, macOS · **Delivery:** portable · **Kit level:** Specialist

**Access / pricing:** Free edition with limits; paid editions

[Official download](https://dmde.com/download.html) · [Documentation](https://dmde.com/manual.html) · [Publisher](https://dmde.com/)

> **Before you use it:** Prefer read-only investigation on an image. Disk-editor writes require a verified plan.

**Source:** https://dmde.com/manual.html  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="gparted"></a>

## GParted

Graphical partition editor available as a Linux application and bootable live image.

**Choose it for:** A planned partition resize or layout change after backup and health checks.

**Platforms:** Linux, Bootable PC · **Delivery:** boot · **Kit level:** Specialist

**Access / pricing:** Free / open source

[Official download](https://gparted.org/download.php) · [Documentation](https://gparted.org/documentation.php) · [Publisher](https://gparted.org/)

> **Before you use it:** Partition moves and resizing can lose data if interrupted; confirm a recoverable backup and stable power.

**Source:** https://gparted.org/documentation.php  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/INTA/blob/1ecba1129aac275a02f40b9c4f5bea438c9e2688/live/package-lists/inta.list.chroot


<a name="rescuezilla"></a>

## Rescuezilla

Bootable graphical disk-imaging and recovery environment compatible with Clonezilla images.

**Choose it for:** A visual bare-metal backup or restore workflow at the bench.

**Platforms:** Bootable PC · **Delivery:** boot · **Kit level:** Core

**Access / pricing:** Free / open source

[Official download](https://rescuezilla.com/download) · [Documentation](https://rescuezilla.com/help) · [Publisher](https://rescuezilla.com/)

> **Before you use it:** Verify the destination model, serial and capacity before a restore overwrites it.

**Source:** https://rescuezilla.com/help  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="clonezilla"></a>

## Clonezilla

Bootable disk and partition imaging with multiple deployment modes.

**Choose it for:** Capturing or restoring a supported disk image with a repeatable procedure.

**Platforms:** Bootable PC, Linux · **Delivery:** boot · **Kit level:** Core

**Access / pricing:** Free / open source

[Official download](https://clonezilla.org/downloads.php) · [Documentation](https://clonezilla.org/clonezilla-live-doc.php) · [Publisher](https://clonezilla.org/)

> **Before you use it:** Source and target selection must be verified; check filesystem and target-size constraints for the chosen operation.

**Source:** https://clonezilla.org/clonezilla-live-doc.php  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/INTA/blob/1ecba1129aac275a02f40b9c4f5bea438c9e2688/live/package-lists/inta.list.chroot


<a name="restic"></a>

## restic

Encrypted, deduplicating file-backup snapshots across local and remote storage.

**Choose it for:** Repeatable file backups with explicit integrity and restore checks.

**Platforms:** Windows, Linux, macOS · **Delivery:** portable · **Kit level:** Core

**Access / pricing:** Free / open source

[Official download](https://github.com/restic/restic/releases/latest) · [Documentation](https://restic.readthedocs.io/en/stable/) · [Publisher](https://restic.net/)

> **Before you use it:** Keep the repository password recoverable and test restoration of real files. A snapshot is not a full bare-metal image.

**Source:** https://restic.readthedocs.io/en/stable/  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json


<a name="kopia"></a>

## Kopia

Encrypted snapshot backups with both desktop and command-line interfaces.

**Choose it for:** A visual backup workflow with repository policies and restore browsing.

**Platforms:** Windows, Linux, macOS · **Delivery:** installed · **Kit level:** Core

**Access / pricing:** Free / open source

[Official download](https://kopia.io/docs/installation/) · [Documentation](https://kopia.io/docs/) · [Publisher](https://kopia.io/)

> **Before you use it:** Record credentials and verify a restore before relying on the repository.

**Source:** https://kopia.io/docs/  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json


<a name="rclone"></a>

## rclone

Transfers, checks and synchronizes files across many storage providers.

**Choose it for:** Cloud migrations, verifiable copies and controlled provider-to-provider transfers.

**Platforms:** Windows, Linux, macOS · **Delivery:** portable · **Kit level:** Core

**Access / pricing:** Free / open source

[Official download](https://rclone.org/downloads/) · [Documentation](https://rclone.org/docs/) · [Publisher](https://rclone.org/)

> **Before you use it:** Start with copy and dry-run where applicable. sync can remove destination files and is not versioned backup by itself.

**Source:** https://rclone.org/docs/  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json


<a name="veeam-agent"></a>

## Veeam Agent for Microsoft Windows

Windows backup with file, volume and full-machine recovery options.

**Choose it for:** Image backup and recovery media for a supported Windows endpoint.

**Platforms:** Windows · **Delivery:** installed · **Kit level:** Core

**Access / pricing:** Free and paid editions; service-provider terms differ

[Official download](https://www.veeam.com/products/free/microsoft-windows) · [Documentation](https://helpcenter.veeam.com/docs/agentforwindows/userguide/overview.html) · [Publisher](https://www.veeam.com/products/free/microsoft-windows)

> **Before you use it:** Use the licensing appropriate to managed client services; test recovery media and a restore path.

**Source:** https://helpcenter.veeam.com/docs/agentforwindows/userguide/overview.html  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="freefilesync"></a>

## FreeFileSync

Visual folder comparison and synchronization with configurable directions.

**Choose it for:** Reviewed file migration when technicians need to see proposed differences.

**Platforms:** Windows, Linux, macOS · **Delivery:** installed · **Kit level:** Core

**Access / pricing:** Business Edition for commercial use

[Official download](https://freefilesync.org/download.php) · [Documentation](https://freefilesync.org/faq.php) · [Publisher](https://freefilesync.org/)

> **Before you use it:** Choose the intended direction and preview deletions. Synchronization alone does not retain independent history.

**Source:** https://freefilesync.org/faq.php  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json


<a name="robocopy"></a>

## Robocopy

Windows file-copy utility with restart, retry, metadata and logging controls.

**Choose it for:** A documented migration between accessible Windows folders or shares.

**Platforms:** Windows · **Delivery:** built-in · **Kit level:** Core

**Access / pricing:** Included with Windows

[Official download](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy) · [Documentation](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy) · [Publisher](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy)

> **Before you use it:** Mirror and purge options delete destination content. Check exit-code semantics and verify the copied data.

**Source:** https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="nvme-cli"></a>

## nvme-cli

Linux NVMe controller and namespace inspection utilities.

**Choose it for:** Reading NVMe identity and health information from INTA Rescue.

**Platforms:** Linux · **Delivery:** installed · **Kit level:** Specialist

**Access / pricing:** Free / open source

[Official download](https://github.com/linux-nvme/nvme-cli/releases/latest) · [Documentation](https://github.com/linux-nvme/nvme-cli) · [Publisher](https://github.com/linux-nvme/nvme-cli)

> **Before you use it:** Inventory and health commands differ from format, sanitize and firmware commands; keep those write operations separate.

**Source:** https://github.com/linux-nvme/nvme-cli  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/INTA/blob/1ecba1129aac275a02f40b9c4f5bea438c9e2688/live/package-lists/inta.list.chroot


<a name="gsmartcontrol"></a>

## GSmartControl

Graphical frontend for drive health inspection and smartctl self-tests.

**Choose it for:** A visual SMART investigation in the rescue desktop.

**Platforms:** Linux, Windows · **Delivery:** installed · **Kit level:** Core

**Access / pricing:** Free / open source

[Official download](https://gsmartcontrol.shaduri.dev/downloads) · [Documentation](https://gsmartcontrol.shaduri.dev/) · [Publisher](https://gsmartcontrol.shaduri.dev/)

> **Before you use it:** Health reads may not pass through every USB adapter. Avoid lengthy self-tests on a failing drive.

**Source:** https://gsmartcontrol.shaduri.dev/  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/INTA/blob/1ecba1129aac275a02f40b9c4f5bea438c9e2688/live/package-lists/inta.list.chroot


<a name="cryptsetup"></a>

## cryptsetup

Linux tooling for encrypted-volume management and supported unlock workflows.

**Choose it for:** Opening a supported encrypted volume using the correct recovery material.

**Platforms:** Linux · **Delivery:** installed · **Kit level:** Specialist

**Access / pricing:** Free / open source

[Official download](https://gitlab.com/cryptsetup/cryptsetup/-/releases) · [Documentation](https://gitlab.com/cryptsetup/cryptsetup) · [Publisher](https://gitlab.com/cryptsetup/cryptsetup)

> **Before you use it:** Unlocking and formatting are different operations. Never format or alter headers as a troubleshooting shortcut.

**Source:** https://gitlab.com/cryptsetup/cryptsetup  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/INTA/blob/1ecba1129aac275a02f40b9c4f5bea438c9e2688/live/package-lists/inta.list.chroot


<a name="dislocker"></a>

## Dislocker

Access to supported BitLocker-encrypted volumes from a non-Windows environment.

**Choose it for:** Reading an encrypted client volume with its valid key or recovery material.

**Platforms:** Linux, macOS · **Delivery:** installed · **Kit level:** Specialist

**Access / pricing:** Free / open source

[Official download](https://github.com/Aorimn/dislocker/releases) · [Documentation](https://github.com/Aorimn/dislocker) · [Publisher](https://github.com/Aorimn/dislocker)

> **Before you use it:** Use read-only access for recovery when possible. It does not bypass the need for authorized key material.

**Source:** https://github.com/Aorimn/dislocker  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/INTA/blob/1ecba1129aac275a02f40b9c4f5bea438c9e2688/live/package-lists/inta.list.chroot


<a name="rsync"></a>

## rsync

Incremental file transfer with explicit preservation and comparison controls.

**Choose it for:** A logged file copy from Linux rescue to a separate destination.

**Platforms:** Linux, macOS, BSD · **Delivery:** installed · **Kit level:** Core

**Access / pricing:** Free / open source

[Official download](https://rsync.samba.org/download.html) · [Documentation](https://download.samba.org/pub/rsync/rsync.1) · [Publisher](https://rsync.samba.org/)

> **Before you use it:** Check trailing-slash semantics and avoid delete options unless deletion is part of the reviewed plan.

**Source:** https://download.samba.org/pub/rsync/rsync.1  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/INTA/blob/1ecba1129aac275a02f40b9c4f5bea438c9e2688/live/package-lists/inta.list.chroot
