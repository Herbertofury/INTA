# INTA Rescue

A live x86-64 recovery desktop for INTA Computer Services. Boot from USB to diagnose hardware, recover files, image disks, inspect offline Windows data, and get AI assistance when connected.

[Latest verified release](https://github.com/Herbertofury/INTA/releases/latest) · [Build and boot checks](https://github.com/Herbertofury/INTA/actions) · [Tool catalog](docs/tools.md)

## Make or refresh the USB

1. Start **INTA Media Creator** with `python media/inta-media.py` on Windows or `python3 media/inta-media.py` on Linux. Use Python 3.11 or later with Tk installed.
2. Choose **Download or update INTA ISO**. The app verifies the published SHA-256 checksum before replacing the local image.
3. For **Ventoy**, open the upstream Ventoy utility from the Media Creator to install Ventoy on the USB once. Select **Install or update ISO on Ventoy USB** and choose the mounted data volume. The verified ISO replaces the existing INTA file and leaves other Ventoy images in place.
4. For **Rufus**, open the official Windows utility from the Media Creator, choose the verified ISO and select your USB. Rufus writes the disk and erases its current contents.

The Actions workflow is configured to rebuild every Tuesday and on demand. Each build resolves updated packages through the Debian archive and resolves both AI clients through the npm registry. It publishes a new ISO only after the desktop and tools pass BIOS and UEFI VM checks. The Media Creator updates the local ISO or Ventoy copy on demand. The package list lives in the source tree, while the installed npm versions and integrity data live in `/opt/inta-ai/package-lock.json` inside each image.

## What opens when INTA boots

- **Disks & Recovery** includes disk inventory, SMART health, GParted, Clonezilla, TestDisk and PhotoRec.
- **System & Security** includes hardware inventory, manual ClamAV signature refresh, folder scanning and access to Sleuth Kit, YARA, Windows hive and event-log tools, and local packet inspection.
- **AI Workspace** opens ChatGPT in Firefox and launches the bundled Codex and Claude Code CLIs on demand. The provider may require sign-in and internet access. Keep client files off the AI workspace unless approved for sharing.

The live session uses XFCE and keeps its writable state in memory by default. Connect a separate destination before exporting files or disk images. Refresh ClamAV signatures from the desktop for each new session if your ISO does not have a signature database. See the [tool catalog](docs/tools.md) for Windows-native repair media and tools.

## Build from source

Use an amd64 Debian 13 host or disposable VM with network access and sufficient disk space.

```sh
sudo apt-get update
sudo apt-get install live-build debootstrap debian-archive-keyring ca-certificates
sudo ./scripts/build-iso
cd dist
sha256sum -c SHA256SUMS
```

The build targets BIOS and UEFI. Secure Boot and physical USB boot require separate hardware validation. The test service activates only on a QEMU guest explicitly marked by the smoke-test script.
