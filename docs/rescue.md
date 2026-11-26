# [INTA Rescue](https://github.com/Herbertofury/INTA/blob/main/docs/rescue.md)

A live x86-64 recovery desktop for INTA Computer Services. Boot from USB to diagnose hardware, recover files, image disks, inspect offline [Windows](https://www.microsoft.com/windows) data, and get AI assistance when connected.

[Latest verified release](https://github.com/Herbertofury/INTA/releases/latest) · [Build and boot checks](https://github.com/Herbertofury/INTA/actions) · [Tool catalog](tools.md)

## Make or refresh the USB

1. Start **INTA [Media Creator](https://github.com/Herbertofury/INTA/blob/main/docs/rescue.md)** with `python media/inta-media.py` on [Windows](https://www.microsoft.com/windows) or `python3 media/inta-media.py` on [Linux](https://www.kernel.org/). Use [Python](https://www.python.org/) 3.11 or later with Tk installed.
2. Choose **Download or update INTA ISO**. The app verifies the published SHA-256 checksum before replacing the local image.
3. For **[Ventoy](https://www.ventoy.net/)**, open the upstream [Ventoy](https://www.ventoy.net/) utility from the [Media Creator](https://github.com/Herbertofury/INTA/blob/main/docs/rescue.md) to install [Ventoy](https://www.ventoy.net/) on the USB once. Select **Install or update ISO on [Ventoy](https://www.ventoy.net/) USB** and choose the mounted data volume. The verified ISO replaces the existing INTA file and leaves other [Ventoy](https://www.ventoy.net/) images in place.
4. For **[Rufus](https://rufus.ie/)**, open the official [Windows](https://www.microsoft.com/windows) utility from the [Media Creator](https://github.com/Herbertofury/INTA/blob/main/docs/rescue.md), choose the verified ISO and select your USB. [Rufus](https://rufus.ie/) writes the disk and erases its current contents.

The Actions workflow is configured to rebuild every Tuesday and on demand. Each build resolves updated packages through the [Debian](https://www.debian.org/) archive and resolves both AI clients through the npm registry. It publishes a new ISO only after the desktop and tools pass BIOS and UEFI VM checks. The [Media Creator](https://github.com/Herbertofury/INTA/blob/main/docs/rescue.md) updates the local ISO or [Ventoy](https://www.ventoy.net/) copy on demand. The package list lives in the source tree, while the installed npm versions and integrity data live in `/opt/inta-ai/package-lock.json` inside each image.

## What opens when INTA boots

- **Disks & Recovery** includes disk inventory, SMART health, [GParted](https://gparted.org/), [Clonezilla](https://clonezilla.org/), [TestDisk](https://www.cgsecurity.org/wiki/TestDisk) and [PhotoRec](https://www.cgsecurity.org/wiki/PhotoRec).
- **System & Security** includes hardware inventory, manual [ClamAV](https://www.clamav.net/) signature refresh, folder scanning and access to Sleuth Kit, YARA, [Windows](https://www.microsoft.com/windows) hive and event-log tools, and local packet inspection.
- **AI Workspace** opens [ChatGPT](https://chatgpt.com/) in [Firefox](https://www.mozilla.org/firefox/) and launches the bundled Codex and [Claude Code](https://code.claude.com/docs/en/overview) CLIs on demand. The provider may require sign-in and internet access. Keep client files off the AI workspace unless approved for sharing.

The live session uses [XFCE](https://www.xfce.org/) and keeps its writable state in memory by default. Connect a separate destination before exporting files or disk images. Refresh [ClamAV](https://www.clamav.net/) signatures from the desktop for each new session if your ISO does not have a signature database. See the [tool catalog](tools.md) for [Windows](https://www.microsoft.com/windows)-native repair media and tools.

## Build from source

Use an amd64 [Debian](https://www.debian.org/) 13 host or disposable VM with network access and sufficient disk space.

```sh
sudo apt-get update
sudo apt-get install live-build debootstrap debian-archive-keyring ca-certificates
sudo ./scripts/build-iso
cd dist
sha256sum -c SHA256SUMS
```

The build targets BIOS and UEFI. Secure Boot and physical USB boot require separate hardware validation. The test service activates only on a QEMU guest explicitly marked by the smoke-test script.
