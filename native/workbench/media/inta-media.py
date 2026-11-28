#!/usr/bin/env python3
"""Download a verified INTA release and copy it onto an existing Ventoy volume."""
import argparse
import json
import os
import queue
import re
import shutil
import subprocess
import sys
import threading
import urllib.request
import webbrowser
from pathlib import Path

if __name__ == "__main__" and not __package__:
    import rescue_products as products
    from rescue_devices import candidates, selected_volume, writable_root
    from rescue_iso import verify_iso
else:
    from media import rescue_products as products
    from media.rescue_devices import candidates, selected_volume, writable_root
    from media.rescue_iso import verify_iso

ISO = products.ISO
SUMS = products.SUMS
PARTS = ISO + ".parts.json"
RELEASE_API = products.RELEASE_API
ASSET_ROOT = products.ASSET_ROOT
MAX_PART = products.MAX_PART
expected_digest = products.expected_digest
stream_verified = products.stream_verified


def download_channel(edition=products.FULL):
    name = products.CHANNELS[edition]
    media = Path(__file__).resolve().parent
    path = media / name
    if not path.exists() and media.parts[-3:] == ("native", "workbench", "media"):
        path = media.parents[2] / "data" / name
    with path.open("rb") as stream:
        return products.decode(stream.read(8193), edition)


def download_release(folder, reviewed_channel=None, edition=products.FULL):
    channel = download_channel(edition)
    if channel["qualification"] == "historical-recovery" and channel != reviewed_channel:
        raise ValueError("Review the historical scanner warning before downloading.")
    return f"Verified ISO ready at {products.fetch_product(folder, channel, edition)}"


def choose_download(parent, messagebox, filedialog, run, edition=products.FULL):
    try:
        channel = download_channel(edition)
    except (ValueError, OSError) as error:
        messagebox.showerror("Rescue download channel", str(error), parent=parent)
        return
    if channel["qualification"] == "historical-recovery" and not messagebox.askokcancel(
        "Historical rescue image",
        f"{channel['tag']}\n\n{channel['scannerWarning']}\n\nContinue to choose a download folder?",
        icon="warning", default="cancel", parent=parent
    ):
        return
    folder = filedialog.askdirectory(title="Choose where to save the verified ISO", parent=parent)
    if folder:
        if edition == products.FULL:
            run("Downloading and verifying the pinned release", download_release, Path(folder), channel)
        else:
            run("Downloading and verifying Rescue Recovery", download_release, Path(folder), channel, edition)


def image_digest(source):
    if source.name not in (products.RECOVERY_ISO, ISO, "inta-winpe-amd64.iso", "inta-winpe-arm64.iso"):
        raise ValueError("Choose a supported INTA rescue ISO with its original filename.")
    checksums = source.parent / SUMS
    if not source.is_file() or not checksums.is_file():
        raise ValueError("Select a regular ISO file alongside its SHA256SUMS file.")
    with checksums.open("rb") as manifest:
        data = manifest.read(65_537)
    if len(data) > 65_536:
        raise ValueError("Checksum manifest exceeds 64 KiB.")
    expected = expected_digest(data.decode("ascii"), source.name)
    if source.name == products.RECOVERY_ISO and expected != download_channel(products.RECOVERY)["sha256"]:
        raise ValueError("The checksum differs from the recovery-only product pin.")
    return expected


def copy_to_ventoy(source, volume, expected_id):
    expected = image_digest(source)
    identity = selected_volume(volume, expected_id)
    if source.resolve() == (volume / source.name).resolve():
        raise ValueError("Source and destination are the same ISO.")
    def verify_target(partial):
        # Read back the bytes written to the target before replacing an older image.
        if source.name == products.RECOVERY_ISO:
            products.readback(partial, download_channel(products.RECOVERY), products.RECOVERY)
        else:
            verify_iso(partial, expected)
        selected_volume(volume, identity["id"])
    with writable_root(volume, identity) as destination, source.open("rb") as image:
        size = os.fstat(image.fileno()).st_size
        if identity["filesystem"].lower() in ("fat32", "vfat") and size >= 2**32:
            raise ValueError("This ISO exceeds the FAT32 file size limit.")
        if shutil.disk_usage(destination).free < size:
            raise ValueError("The Ventoy volume has insufficient free space for a safe replacement.")
        stream_verified(image, destination / source.name, expected, size, verify_target)


def inspect_product(source):
    expected = image_digest(source)
    if source.name == products.RECOVERY_ISO:
        return products.readback(source, download_channel(products.RECOVERY), products.RECOVERY)
    return verify_iso(source, expected)


def launch_rufus(binary):
    if not re.fullmatch(r"rufus(?:-[0-9.]+(?:p|_x86|_arm64)?)?\.exe", binary.name, re.IGNORECASE):
        raise ValueError("Choose the Rufus executable supplied by its publisher.")
    env = dict(os.environ, INTA_SELECTED_RUFUS=str(binary.resolve(strict=True)))
    shell = Path(env["SystemRoot"]) / "System32/WindowsPowerShell/v1.0/powershell.exe"
    script = r"""
$ErrorActionPreference = 'Stop'
$file = $env:INTA_SELECTED_RUFUS
$handle = [IO.File]::Open($file, 'Open', 'Read', 'Read')
try {
    $signature = Get-AuthenticodeSignature -LiteralPath $file
    if ($signature.Status -ne 'Valid' -or $signature.SignerCertificate.GetNameInfo('SimpleName',$false) -ne 'Akeo Consulting') { exit 3 }
    Start-Process -FilePath $file -WorkingDirectory "$env:SystemRoot\System32" -ErrorAction Stop
} finally { $handle.Dispose() }
"""
    result = subprocess.run([str(shell), "-NoProfile", "-NonInteractive", "-Command", script],
                            env=env, capture_output=True, timeout=60)
    if result.returncode:
        raise ValueError("Rufus requires a valid Akeo Consulting signature and permission to launch.")
    return "Signed Rufus opened."


def main():
    import tkinter as tk
    from tkinter import filedialog, messagebox, ttk

    root = tk.Tk()
    root.title("INTA Media Creator")
    root.minsize(630, 420)
    root.configure(background="#21252a")
    status = tk.StringVar(value="Ready")
    panel = ttk.Frame(root, padding=24)
    panel.pack(fill="both", expand=True, padx=16, pady=16)
    try:
        from inta_optics_ui import OpticalHeader
    except ImportError:
        OpticalHeader = None
    if OpticalHeader is not None:
        OpticalHeader(panel).pack(fill="x", pady=(0, 8))
    ttk.Label(panel, text="INTA Media Creator", font=("TkDefaultFont", 22, "bold")).pack(anchor="w")
    ttk.Label(panel, text="Verify a rescue ISO or prepare your boot USB.").pack(anchor="w", pady=(3, 16))
    responses = queue.Queue()
    buttons = []
    busy = False

    def collect():
        nonlocal busy
        try:
            result, error, completed = responses.get_nowait()
        except queue.Empty:
            root.after(150, collect)
            return
        busy = False
        for button in buttons:
            button.state(["!disabled"])
        if error is not None:
            status.set("Action failed")
            messagebox.showerror("INTA Media Creator", error)
        elif completed:
            status.set("USB identified")
            completed(result)
        else:
            status.set(result if isinstance(result, str) else "Verified ISO ready")

    def run(label, function, *arguments, completed=None):
        nonlocal busy
        busy = True
        status.set(label + "…")
        for button in buttons:
            button.state(["disabled"])

        def worker():
            try:
                result = function(*arguments)
            except Exception as error:
                responses.put((None, str(error), None))
            else:
                responses.put((result, None, completed))

        threading.Thread(target=worker, daemon=True).start()
        root.after(150, collect)

    def ventoy():
        source = filedialog.askopenfilename(
            title="Select the downloaded INTA ISO next to SHA256SUMS", filetypes=[("ISO", "*.iso")]
        )
        if not source:
            return
        target = filedialog.askdirectory(title="Choose the mounted Ventoy data volume")
        if not target:
            return
        def confirm(device):
            summary = (f"{device['model']}   |   {device['capacity'] / 2**30:.1f} GiB   |   USB\n"
                       f"Disk {device['disk']}, partition 1 at {device['root']}\n{device['id']}\n\n"
                       f"Install or replace {Path(source).name} on this device?")
            if messagebox.askokcancel("Confirm Ventoy USB", summary):
                run("Copying and reading back the ISO", copy_to_ventoy, Path(source), Path(target), device["id"])
        run("Identifying the USB disk", selected_volume, Path(target), completed=confirm)

    def verify():
        source = filedialog.askopenfilename(title="Select the INTA ISO next to SHA256SUMS", filetypes=[("ISO", "*.iso")])
        if source:
            def check():
                entry = inspect_product(Path(source))
                return f"SHA-256 matched. Boot entries: {', '.join(entry['bootEntries'])}."
            run("Checking ISO", check)

    def rufus():
        if os.name != "nt":
            webbrowser.open("https://rufus.ie/en/")
            return
        choice = messagebox.askyesnocancel("Rufus", "Yes: open a signed Rufus executable.\nNo: visit the official download page.")
        if choice is None:
            return
        if not choice:
            webbrowser.open("https://rufus.ie/en/")
            return
        selected = filedialog.askopenfilename(title="Select the official signed Rufus executable", filetypes=[("Executable", "*.exe")])
        if selected:
            run("Checking the Rufus publisher", launch_rufus, Path(selected))

    def close():
        if busy:
            messagebox.showinfo("Transfer in progress", "Wait for the verification and transfer to finish.")
        else:
            root.destroy()

    actions = [
        ("Verify a local INTA ISO", verify),
        ("Download Rescue Recovery · offline file repair (no malware scanner)", lambda: choose_download(root, messagebox, filedialog, run, products.RECOVERY)),
        ("Download full Rescue ISO", lambda: choose_download(root, messagebox, filedialog, run)),
        ("Install or update ISO on Ventoy USB", ventoy),
        ("Get Ventoy from its publisher", lambda: webbrowser.open("https://www.ventoy.net/en/download.html")),
        ("Open or download Rufus", rufus),
    ]
    for label, callback in actions:
        button = ttk.Button(panel, text=label, command=callback)
        button.pack(fill="x", pady=5, ipady=4)
        buttons.append(button)
    ttk.Label(panel, textvariable=status, wraplength=540).pack(anchor="w", pady=(12, 0))
    root.protocol("WM_DELETE_WINDOW", close)
    root.mainloop()


if __name__ == "__main__":
    if len(sys.argv) == 1:
        main()
    else:
        parser = argparse.ArgumentParser(description=__doc__)
        subs = parser.add_subparsers(dest="action", required=True)
        subs.add_parser("verify", help="Match SHA256SUMS and inspect firmware boot entries").add_argument("iso", type=Path)
        subs.add_parser("volumes", help="List eligible mounted Ventoy USB targets")
        try:
            args = parser.parse_args()
            result = inspect_product(args.iso) if args.action == "verify" else candidates()
            print(json.dumps(result))
        except (OSError, ValueError, subprocess.SubprocessError) as error:
            print(json.dumps({"error": str(error)}), file=sys.stderr)
            sys.exit(1)
