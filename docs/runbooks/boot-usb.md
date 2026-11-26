[Home](https://github.com/Herbertofury/INTA/wiki) · [All playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Interactive checklist](https://herbertofury.github.io/INTA/#job/boot-usb)

# Build and refresh a technician rescue USB

> Keep verified recovery images, Windows repair media, and the INTA knowledge hub ready for offline use.

| Job profile | Details |
| --- | --- |
| Category | Recovery & boot |
| Working time | 20–60 min plus downloads |
| Experience | Intermediate |

## Start here

Choose a known USB drive and copy off anything that must be kept. Confirm its model, capacity, and contents before a media-writing operation.

## Your toolkit

| Tool | Why it belongs here |
| --- | --- |
| [Ventoy](https://www.ventoy.net/en/download.html) | Maintaining a technician USB with several independently updated rescue images. |
| [Rufus](https://rufus.ie/#download) | Preparing one ISO on a dedicated USB drive. |
| [Windows 11 installation media](https://www.microsoft.com/software-download/windows11) | A clean installation or supported repair using the correct Windows edition and architecture. |
| [SystemRescue](https://www.system-rescue.org/Download/) | A ready-made Linux recovery environment for an unbootable or damaged machine. |
| [Hiren's BootCD PE](https://www.hirensbootcd.org/download/) | A familiar Windows-oriented offline bench toolkit. |
| [Rescuezilla](https://rescuezilla.com/download) | A visual bare-metal backup or restore workflow at the bench. |

## Follow the workflow

### 01. Choose the media strategy

- [ ] Use Ventoy for a multiboot collection that can be refreshed by copying ISO files, or Rufus for a dedicated image. Keep official Windows media for WinRE and Windows-native repair alongside Linux-based rescue tools. An ISO’s platform determines which tools run inside it.

### 02. Download from the official source

- [ ] Use the catalog’s official download links and keep release notes and vendor checksums with each image. Use the INTA Media Creator for its verified INTA ISO download/update flow. A current boot environment does not guarantee every bundled third-party tool is current.

### 03. Verify the bytes before writing

- [ ] Compare the local SHA-256 hash with the release’s published checksum using the exact file path. In PowerShell, select the downloaded ISO and run Get-FileHash for that file. The hash proves a match to the published value; also verify you obtained that value from the genuine release.

### 04. Write or update the chosen device

- [ ] Installing Ventoy or writing with Rufus can erase the selected USB. Review the device identity before confirming. Use Ventoy’s Update operation for an existing Ventoy installation when intended, and copy refreshed verified ISOs to its data partition. Keep a rollback image until the new one boots.

### 05. Test the actual boot path

- [ ] Boot the USB on a representative spare machine and confirm keyboard, display, storage, network, and recovery tools. Test the intended firmware mode. Read Ventoy’s current Secure Boot documentation before enrolling keys or changing firmware settings; firmware and image support must be proven on the target hardware.

### 06. Keep an offline manifest and refresh on demand

- [ ] Record image name, source URL, release, hash, download date, and successful boot test. Include the INTA offline hub and keep client evidence on a separate encrypted destination. Refresh before a service visit and re-verify changed images instead of assuming the old USB is current.

## Verify before handoff

- [ ] Each image matches its official checksum where one is provided.
- [ ] The intended USB boots and can see required hardware.
- [ ] The offline hub and recovery destination are accessible.
- [ ] The manifest distinguishes downloaded, verified, and hardware-boot-tested images.

## Stop and escalate when

- The target USB is ambiguous or contains the only copy of important data.
- Secure Boot or encryption changes are proposed without a recovery plan.

## Related playbooks

- [Boot Recovery](boot-recovery.md)
- [Failing Drive](failing-drive.md)
- [Backup Restore](backup-restore.md)

## Official references

- [Ventoy: install and update](https://www.ventoy.net/en/doc_start.html)
- [Ventoy: verify image checksums](https://www.ventoy.net/en/doc_checksum.html)
- [Ventoy: Secure Boot behavior](https://www.ventoy.net/en/doc_secure.html)
- [INTA Rescue source and build workflow](https://github.com/Herbertofury/INTA)

---

INTA Computer Services · Editorial review: 2026-11-26. Timing is an estimate; stop conditions take priority.
