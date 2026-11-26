[Home](https://github.com/Herbertofury/INTA/wiki) · [All playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Interactive checklist](https://herbertofury.github.io/INTA/#job/failing-drive)

# Rescue data from a failing or unreadable drive

> Preserve the source, create a recoverable image, and perform recovery on a copy.

| Job profile | Details |
| --- | --- |
| Category | Storage & recovery |
| Working time | Triage 15–30 min; imaging varies |
| Experience | Advanced |

## Start here

Stop ordinary use of the source drive. Ask whether the data is replaceable and whether a verified backup exists before applying power again.

## Your toolkit

| Tool | Why it belongs here |
| --- | --- |
| [smartmontools](https://github.com/smartmontools/smartmontools/releases) | Reading SMART and NVMe health from a rescue environment or supported host OS. |
| [CrystalDiskInfo](https://crystalmark.info/en/download/) | First-pass HDD and SSD health inspection in Windows. |
| [GNU ddrescue](https://ftp.gnu.org/gnu/ddrescue/) | Creating an image of a failing disk before recovery or repair attempts. |
| [TestDisk](https://www.cgsecurity.org/wiki/TestDisk_Download) | Investigating missing partitions on an image or safely copied disk. |
| [PhotoRec](https://www.cgsecurity.org/wiki/TestDisk_Download) | Recovering files from an image when directory-based recovery is unsuccessful. |

## Follow the workflow

### 01. Choose professional recovery when needed

- [ ] Clicking, repeated spin-up, liquid or impact damage, unstable detection, and irreplaceable data justify a specialist recovery decision before further attempts. Repeated scans can consume the remaining readable life. Do not open a drive or repeatedly power-cycle it.

### 02. Identify every device

- [ ] In the Linux rescue session, record source and destination model, serial, capacity, interface, and mount state. Use a known-good cable and stable power when appropriate. Disable automount and keep the source unmounted or read-only; do not initialize, format, or run filesystem repair on it.

```text
lsblk -o NAME,SIZE,MODEL,SERIAL,TYPE,MOUNTPOINTS
```

### 03. Prepare a separate healthy destination

- [ ] Choose a destination with sufficient capacity for the image and recovery output. Record the exact source, output file or device, and persistent mapfile path. A second technician should verify those identities before any imaging command that can overwrite a destination.

### 04. Image with a resumable recovery tool

- [ ] Use GNU ddrescue following its manual, with a mapfile saved on healthy storage. Start with a low-stress pass that retrieves easy data before any deliberate retry phase. Preserve the mapfile and image together, track errors and temperature, and stop if the device deteriorates.

### 05. Recover from a working copy

- [ ] Keep the original recovery image unchanged and create a working copy if capacity permits. Use TestDisk for partition/filesystem investigation or PhotoRec for file carving as appropriate. Save recovered files to a different healthy location and expect carving to lose some names or folder structure.

### 06. Validate what was actually recovered

- [ ] Open representative documents, photos, databases, and archives with the user’s applications. Compare expected folders and counts where known, record unreadable regions and missing files, and retain the recovery image until the client accepts the result.

## Verify before handoff

- [ ] The original source has not been repaired or overwritten.
- [ ] The image, mapfile, and device identities are recorded.
- [ ] Recovered files are stored separately and checked for usability.
- [ ] Unreadable data and incomplete recovery are explicitly reported.

## Stop and escalate when

- The source develops new noises, unstable detection, or worsening read failures.
- Source and destination cannot be distinguished with certainty.
- The data value calls for a specialist rather than further bench attempts.

## Related playbooks

- [Disk Migration](disk-migration.md)
- [Backup Restore](backup-restore.md)
- [Boot Usb](boot-usb.md)
- [Client Handoff](client-handoff.md)

## Official references

- [GNU ddrescue manual](https://www.gnu.org/software/ddrescue/manual/ddrescue_manual.html)
- [CGSecurity: PhotoRec data recovery](https://www.cgsecurity.org/wiki/PhotoRec)

---

INTA Computer Services · Editorial review: 2026-11-26. Timing is an estimate; stop conditions take priority.
