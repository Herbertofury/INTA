# Data Recovery & Disk Imaging

## Golden rule
If a source drive may be failing, minimize writes and unnecessary reads. Do not install recovery software on the source, run filesystem repair, initialize/format a partition or save recovered files back to the same device.

## Triage
1. Confirm ownership, authorization, data priority and scope.
2. Establish what happened: deletion, format, corruption, encryption, clicking, drop/liquid or intermittent disconnect.
3. Identify source and destination by model, serial, capacity and connection path.
4. Check physical condition and SMART where possible.
5. Clicking, severe media errors or irreplaceable data: stop and escalate to a professional recovery provider.
6. If appropriate, image to healthy storage and work from the image/copy.

## Tool choices
- [GNU ddrescue](https://www.gnu.org/software/ddrescue/manual/ddrescue_manual.html): controlled imaging of failing media; mapfile supports resumption.
- [Clonezilla](https://clonezilla.org/) / [Partclone](https://partclone.org/): disk/partition imaging.
- [TestDisk / PhotoRec](https://www.cgsecurity.org/): partition recovery and file carving; names/structure may not be preserved.
- [Sleuth Kit](https://www.sleuthkit.org/) / [Autopsy](https://www.autopsy.com/): forensic analysis.

## ddrescue pattern — verify devices before use
```bash
lsblk -o NAME,SIZE,MODEL,SERIAL,RO,TYPE,MOUNTPOINTS
sudo smartctl -a /dev/sdX
# Pattern only: source device to an image file on a separate healthy destination.
sudo ddrescue -n /dev/sdX /mnt/recovery/source.img /mnt/recovery/source.map
```

Replace /dev/sdX only after independent verification. Reversing source and destination can destroy data. Do not mount a failing source read-write. For forensic cases, hardware write blocking and chain of custody may be required.

Use a separate physical destination with sufficient capacity. Record tool versions, hashes, mapfiles and logs. Recover files to a different disk and validate representative files with the requester. Stop if abnormal mechanical sounds, overheating, liquid/impact damage or repeated disconnects occur.
