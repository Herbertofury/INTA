#!/usr/bin/env python3
"""Download a verified INTA release and copy it onto an existing Ventoy volume."""
import argparse
import hashlib
import json
import os
import queue
import re
import shutil
import subprocess
import sys
import tempfile
import threading
import urllib.error
import urllib.parse
import urllib.request
import webbrowser
from pathlib import Path

if __name__ == "__main__" and not __package__:
    from rescue_devices import candidates, selected_volume, writable_root
    from rescue_iso import verify_iso
else:
    from media.rescue_devices import candidates, selected_volume, writable_root
    from media.rescue_iso import verify_iso

REPO = "Herbertofury/INTA"
RELEASE_API = f"https://api.github.com/repos/{REPO}/releases/latest"
ASSET_ROOT = f"https://github.com/{REPO}/releases/download/"
ISO = "inta-rescue-amd64.iso"
SUMS = "SHA256SUMS"


def expected_digest(data, filename=ISO):
    pattern = rf"([0-9a-f]{{64}}) [ *]{re.escape(filename)}"
    matches = [match.group(1) for line in data.splitlines()
               if (match := re.fullmatch(pattern, line))]
    if len(matches) != 1:
        raise ValueError(f"Expected exactly one checksum for {filename}.")
    return matches[0]


def stream_verified(source, target, expected, expected_size=None, before_replace=None):
    digest = hashlib.sha256()
    total = 0
    with tempfile.NamedTemporaryFile(dir=target.parent, prefix=".inta-", delete=False) as output:
        partial = Path(output.name)
        try:
            while chunk := source.read(1024 * 1024):
                total += len(chunk)
                if expected_size is not None and total > expected_size:
                    raise ValueError("The download exceeds the release asset size.")
                digest.update(chunk)
                output.write(chunk)
            if digest.hexdigest() != expected or (expected_size is not None and total != expected_size):
                raise ValueError("Verification failed. Any previous ISO is intact.")
            output.flush()
            os.fsync(output.fileno())
            output.close()
            if before_replace:
                before_replace(partial)
            partial.replace(target)
        finally:
            if partial.exists():
                output.close()
                partial.unlink()


class ReleaseRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        url = urllib.parse.urlsplit(newurl)
        hosts = {"github.com", "api.github.com", "release-assets.githubusercontent.com", "objects.githubusercontent.com"}
        if url.scheme != "https" or url.hostname not in hosts or url.username or url.password or url.port not in (443, None):
            raise ValueError("Unexpected release download redirect.")
        return super().redirect_request(req, fp, code, msg, headers, newurl)


def release_asset(assets, name, tag):
    asset = assets[name]
    url = asset["browser_download_url"]
    if url != f"{ASSET_ROOT}{tag}/{name}" or type(asset["size"]) is not int or asset["size"] <= 0:
        raise ValueError("Unexpected release asset URL.")
    return url, asset["size"]


def download_release(folder):
    opener = urllib.request.build_opener(ReleaseRedirect)
    try:
        with opener.open(RELEASE_API, timeout=30) as reply:
            release = json.loads(reply.read(1024 * 1024))
    except urllib.error.HTTPError as error:
        if error.code == 404:
            raise ValueError("The INTA repository has no published rescue release. Choose Verify a local INTA ISO for a local build.") from error
        raise
    tag = release["tag_name"]
    if not re.fullmatch(r"rescue-[A-Za-z0-9._-]{1,96}", tag):
        raise ValueError("Unexpected rescue release tag.")
    assets = {asset["name"]: asset for asset in release["assets"]}
    sums_url, sums_size = release_asset(assets, SUMS, tag)
    image_url, image_size = release_asset(assets, ISO, tag)
    if sums_size > 65536:
        raise ValueError("Release checksum manifest exceeds 64 KiB.")
    with opener.open(sums_url, timeout=30) as reply:
        checksums = reply.read(sums_size + 1)
        if len(checksums) != sums_size:
            raise ValueError("Incomplete release checksum manifest.")
        expected = expected_digest(checksums.decode("ascii"))
    api_digest = assets[ISO].get("digest")
    if api_digest and api_digest != f"sha256:{expected}":
        raise ValueError("The release asset digest disagrees with SHA256SUMS.")
    # Version both files together. Publishing one directory keeps prior releases usable.
    product = folder / f"{tag}-{expected[:12]}"
    if product.exists():
        if image_digest(product / ISO) != expected:
            raise ValueError("The existing release directory has a different checksum manifest.")
        verify_iso(product / ISO, expected)
        return f"Verified ISO ready at {product / ISO}"
    if shutil.disk_usage(folder).free < image_size:
        raise ValueError("The destination has insufficient free space for a safe replacement.")
    with tempfile.TemporaryDirectory(dir=folder, prefix=".inta-release-") as temp:
        staging = Path(temp) / "image"
        staging.mkdir()
        with opener.open(image_url, timeout=45) as reply:
            stream_verified(reply, staging / ISO, expected, image_size)
        verify_iso(staging / ISO, expected)
        (staging / SUMS).write_bytes(checksums)
        staging.rename(product)
    return f"Verified ISO ready at {product / ISO}"


def image_digest(source):
    if source.name not in (ISO, "inta-winpe-amd64.iso", "inta-winpe-arm64.iso"):
        raise ValueError("Choose a supported INTA rescue ISO with its original filename.")
    checksums = source.parent / SUMS
    if not source.is_file() or not checksums.is_file():
        raise ValueError("Select a regular ISO file alongside its SHA256SUMS file.")
    with checksums.open("rb") as manifest:
        data = manifest.read(65_537)
    if len(data) > 65_536:
        raise ValueError("Checksum manifest exceeds 64 KiB.")
    return expected_digest(data.decode("ascii"), source.name)


def copy_to_ventoy(source, volume, expected_id):
    expected = image_digest(source)
    identity = selected_volume(volume, expected_id)
    if source.resolve() == (volume / source.name).resolve():
        raise ValueError("Source and destination are the same ISO.")
    def verify_target(partial):
        # Read back the bytes written to the target before replacing an older image.
        verify_iso(partial, expected)
        selected_volume(volume, identity["id"])
    with writable_root(volume, identity) as destination, source.open("rb") as image:
        size = os.fstat(image.fileno()).st_size
        if identity["filesystem"].lower() in ("fat32", "vfat") and size >= 2**32:
            raise ValueError("This ISO exceeds the FAT32 file size limit.")
        if shutil.disk_usage(destination).free < size:
            raise ValueError("The Ventoy volume has insufficient free space for a safe replacement.")
        stream_verified(image, destination / source.name, expected, size, verify_target)


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
    ttk.Label(panel, text="Download a refreshed rescue ISO and prepare your boot USB.").pack(anchor="w", pady=(3, 16))
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

    def download():
        folder = filedialog.askdirectory(title="Choose where to save the updated ISO")
        if folder:
            run("Downloading and verifying release", download_release, Path(folder))

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
                entry = verify_iso(Path(source), image_digest(Path(source)))
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
        ("Download or update INTA ISO", download),
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
            result = verify_iso(args.iso, image_digest(args.iso)) if args.action == "verify" else candidates()
            print(json.dumps(result))
        except (OSError, ValueError, subprocess.SubprocessError) as error:
            print(json.dumps({"error": str(error)}), file=sys.stderr)
            sys.exit(1)
