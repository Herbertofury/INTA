# Rescue media and the technician hub

**[INTA releases](https://github.com/Herbertofury/INTA/releases)** · [Build status](https://github.com/Herbertofury/INTA/actions) · [Rufus](https://rufus.ie/en/) · [Ventoy](https://www.ventoy.net/en/download.html)

## Choose the right environment

| Environment | Choose it for | Important boundary |
| --- | --- | --- |
| **INTA Rescue** | Offline disk inspection, SMART, imaging, file recovery, hardware diagnostics, Linux-based triage and AI access. | A Debian live desktop. Windows-native repair tools and desktop apps do not become Linux programs just because they appear in the hub. |
| **Windows Recovery Environment / Microsoft installation media** | Windows startup repair, Windows servicing and recovery options. | Match the installed Windows system, encryption state and intended operation. |
| **Hiren's BootCD PE** | A Windows PE utility environment when its included tools fit the job. | Check the bundled software list and release age. Boot media freshness differs from each bundled tool's freshness. |
| **SystemRescue / Rescuezilla** | A specialist Linux rescue environment or a graphical imaging workflow. | Keep as separately named, verified images. |
| **Running Windows** | Remote support, live process inspection, application fixes, endpoint protection and normal updates. | Capture live evidence before restarts if it matters to the incident. |

## Prepare a USB with INTA Media Creator

1. Launch `python media/inta-media.py` on Windows or `python3 media/inta-media.py` on Linux, with Python 3.11+ and Tk available.
2. Choose **Download or update INTA ISO**. The application checks the published SHA-256 checksum before replacing the local image.
3. **For a multiboot USB:** use the official Ventoy utility to initialize the intended USB once. Its first installation can erase that device. Then use **Install or update ISO on Ventoy USB** and select its mounted data volume. This copies the verified INTA image while keeping other images.
4. **For a dedicated INTA USB:** open Rufus from the Media Creator on Windows. Select the verified ISO and confirm the intended USB before writing it. Writing the device erases its existing contents.
5. Boot the USB on representative hardware before taking it to a job. Confirm firmware mode, keyboard, networking, storage visibility and a separate export destination.

The Media Creator coordinates official upstream utilities; it is not a rewritten Rufus or Ventoy bootloader. Physical boot and Secure Boot compatibility remain hardware-specific validation work. A VM boot check is useful evidence but cannot prove every physical machine will boot.

## Take the hub on your USB

Copy `site/index.html` to the Ventoy data volume or a separate technician drive and open it in a browser when needed. The full catalog and all checklists are embedded, so the guide remains usable without network access. Official downloads, external documentation and online AI require a connection. Updating the hub HTML is independent of replacing an ISO.

The wider catalog is a library of recommendations. For what is actually installed in a specific ISO, use its Debian package manifest and `/opt/inta-ai/package-lock.json`; consult `live/package-lists/inta.list.chroot` for requested base packages. A tool listed in the browser hub is not automatically bundled.

## At the job

- Confirm source and destination disks by model, size and serial, not drive letter alone.
- Keep recovery writes and recovered files on a separate destination.
- Treat a hibernated or dirty Windows filesystem carefully; use the recovery guide before attempting writes.
- Have the correct BitLocker recovery method available before changing boot or recovery behavior.
- Export important logs and recovered files before shutting down a RAM-backed live session.
- Sign out of web AI and remote services before handing the machine back or ending the session.

## References

- [Ventoy download](https://www.ventoy.net/en/download.html)
- [Rufus](https://rufus.ie/en/)
- [Microsoft Windows 11 media](https://www.microsoft.com/software-download/windows11)
- [Hiren's download and included software](https://www.hirensbootcd.org/download/)
- [SystemRescue](https://www.system-rescue.org/)
- [Rescuezilla downloads](https://rescuezilla.com/download)
