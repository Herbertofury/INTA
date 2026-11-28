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
import urllib.parse
import urllib.request
import webbrowser
from contextlib import closing
from pathlib import Path

if __name__ == "__main__" and not __package__:
    from rescue_devices import candidates, selected_volume, writable_root
    from rescue_iso import verify_iso
else:
    from media.rescue_devices import candidates, selected_volume, writable_root
    from media.rescue_iso import verify_iso

REPO = "Herbertofury/INTA"
RELEASE_API = f"https://api.github.com/repos/{REPO}/releases/tags/"
ASSET_ROOT = f"https://github.com/{REPO}/releases/download/"
ISO = "inta-rescue-amd64.iso"
SUMS = "SHA256SUMS"
PARTS = ISO + ".parts.json"
CHANNEL_FILE = "rescue-download-channel.json"
MAX_MANIFEST = 1024 * 1024
MAX_PART = 1024 ** 3
MAX_IMAGE = 32 * 1024 ** 3


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


def unique_fields(items):
    result = {}
    for key, value in items:
        if key in result:
            raise ValueError(f"Duplicate JSON field {key}.")
        result[key] = value
    return result


def download_channel():
    media = Path(__file__).resolve().parent
    path = media / CHANNEL_FILE
    if not path.exists() and media.parts[-3:] == ("native", "workbench", "media"):
        path = media.parents[2] / "data" / CHANNEL_FILE
    with path.open("rb") as stream:
        content = stream.read(8193)
    if len(content) > 8192:
        raise ValueError("Rescue download channel exceeds 8 KiB.")
    channel = json.loads(content, object_pairs_hook=unique_fields)
    if (not isinstance(channel, dict) or type(channel.get("schemaVersion")) is not int
            or channel["schemaVersion"] != 1
            or not re.fullmatch(r"rescue-[0-9]{1,24}\.[0-9]{1,8}", str(channel.get("tag", "")))
            or not re.fullmatch(r"[a-f0-9]{64}", str(channel.get("sha256", "")))
            or channel.get("qualification") not in ("qualified", "historical-recovery")):
        raise ValueError("Invalid pinned Rescue download channel.")
    if channel["qualification"] == "historical-recovery" and (
        not isinstance(channel.get("scannerWarning"), str) or not channel["scannerWarning"].strip()
    ):
        raise ValueError("Historical recovery requires a scanner warning.")
    return channel


def release_asset(assets, name, tag, limit):
    asset = assets.get(name)
    if not isinstance(asset, dict):
        raise ValueError(f"The release is missing {name}.")
    size = asset.get("size")
    if (asset.get("browser_download_url") != f"{ASSET_ROOT}{tag}/{name}"
            or type(size) is not int or not 0 < size <= limit):
        raise ValueError(f"Unexpected URL or size for {name}.")
    return asset["browser_download_url"], size


def metadata(opener, assets, name, tag, limit):
    url, size = release_asset(assets, name, tag, limit)
    with opener.open(url, timeout=30) as reply:
        content = reply.read(size + 1)
    if len(content) != size:
        raise ValueError(f"Incomplete or oversized {name}.")
    check_asset_digest(assets[name], hashlib.sha256(content).hexdigest())
    return content


def check_asset_digest(asset, expected):
    provided = asset.get("digest")
    if provided is not None and provided != "sha256:" + expected:
        raise ValueError(f"The release asset digest disagrees for {asset['name']}.")


def verified_parts(manifest, assets, tag, expected):
    if (not isinstance(manifest, dict) or set(manifest) != {"schemaVersion", "filename", "size", "sha256", "parts"}
            or type(manifest["schemaVersion"]) is not int or manifest["schemaVersion"] != 1
            or manifest["filename"] != ISO or type(manifest["size"]) is not int
            or not 0 < manifest["size"] <= MAX_IMAGE or manifest["sha256"] != expected
            or not isinstance(manifest["parts"], list) or not 1 <= len(manifest["parts"]) <= 999):
        raise ValueError("Invalid ISO parts manifest or pinned checksum mismatch.")
    parts = []
    for index, part in enumerate(manifest["parts"], 1):
        name = f"{ISO}.part{index:03d}"
        if (not isinstance(part, dict) or set(part) != {"name", "size", "sha256"}
                or part["name"] != name or type(part["size"]) is not int
                or not isinstance(part["sha256"], str) or not re.fullmatch(r"[0-9a-f]{64}", part["sha256"])):
            raise ValueError(f"Invalid manifest entry for {name}.")
        url, size = release_asset(assets, name, tag, MAX_PART)
        if size != part["size"]:
            raise ValueError(f"The manifest and release sizes differ for {name}.")
        check_asset_digest(assets[name], part["sha256"])
        parts.append((name, url, size, part["sha256"]))
    if sum(p[2] for p in parts) != manifest["size"]:
        raise ValueError("The ordered parts do not add up to the declared ISO size.")
    return manifest["size"], parts


class VerifiedPartsReader:
    """Provide one bounded stream while independently checking each part."""

    def __init__(self, opener, parts):
        self.opener = opener
        self.parts = iter(parts)
        self.reply = None

    def close(self):
        if self.reply is not None:
            self.reply.close()
            self.reply = None

    def read(self, size):
        while True:
            if self.reply is None:
                part = next(self.parts, None)
                if part is None:
                    return b""
                self.name, url, self.remaining, self.expected = part
                self.digest = hashlib.sha256()
                self.reply = self.opener.open(url, timeout=45)
            chunk = self.reply.read(min(size, self.remaining + 1))
            if chunk:
                self.remaining -= len(chunk)
                if self.remaining < 0:
                    raise ValueError(f"Oversized part {self.name}.")
                self.digest.update(chunk)
                return chunk
            self.close()
            if self.remaining or self.digest.hexdigest() != self.expected:
                raise ValueError(f"ISO part {self.name} failed verification.")


def download_release(folder, reviewed_channel=None):
    channel = download_channel()
    if channel["qualification"] == "historical-recovery" and channel != reviewed_channel:
        raise ValueError("Review the historical scanner warning before downloading.")
    tag, expected = channel["tag"], channel["sha256"]
    product = folder / f"{tag}-{expected[:12]}"
    if product.exists():
        if image_digest(product / ISO) != expected:
            raise ValueError("The existing release directory has a different checksum manifest.")
        verify_iso(product / ISO, expected)
        return f"Verified ISO ready at {product / ISO}"
    opener = urllib.request.build_opener(ReleaseRedirect)
    with opener.open(RELEASE_API + tag, timeout=30) as reply:
        body = reply.read(MAX_MANIFEST + 1)
    if len(body) > MAX_MANIFEST:
        raise ValueError("Release metadata exceeds 1 MiB.")
    release = json.loads(body, object_pairs_hook=unique_fields)
    if (not isinstance(release, dict) or release.get("tag_name") != tag
            or release.get("draft") is not False or release.get("prerelease") is not False
            or not isinstance(release.get("assets"), list)):
        raise ValueError("The published release differs from the pinned Rescue channel.")
    assets = {}
    for asset in release["assets"]:
        if not isinstance(asset, dict) or not isinstance(asset.get("name"), str) or asset["name"] in assets:
            raise ValueError("Invalid or duplicate release asset.")
        assets[asset["name"]] = asset
    checksums = metadata(opener, assets, SUMS, tag, 65536)
    if expected_digest(checksums.decode("ascii")) != expected:
        raise ValueError("The release checksum differs from the pinned Rescue channel.")
    parts = None
    if ISO in assets:
        image_url, image_size = release_asset(assets, ISO, tag, MAX_IMAGE)
        check_asset_digest(assets[ISO], expected)
    else:
        manifest = json.loads(metadata(opener, assets, PARTS, tag, MAX_MANIFEST), object_pairs_hook=unique_fields)
        image_size, parts = verified_parts(manifest, assets, tag, expected)
    if shutil.disk_usage(folder).free < image_size:
        raise ValueError("The destination has insufficient free space for a safe replacement.")
    with tempfile.TemporaryDirectory(dir=folder, prefix=".inta-release-") as temp:
        staging = Path(temp) / "image"
        staging.mkdir()
        with (closing(VerifiedPartsReader(opener, parts)) if parts else opener.open(image_url, timeout=45)) as reply:
            stream_verified(reply, staging / ISO, expected, image_size)
        verify_iso(staging / ISO, expected)
        (staging / SUMS).write_bytes(checksums)
        staging.rename(product)
    return f"Verified ISO ready at {product / ISO}"


def choose_download(parent, messagebox, filedialog, run):
    try:
        channel = download_channel()
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
        run("Downloading and verifying the pinned release", download_release, Path(folder), channel)


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
        ("Download pinned rescue ISO", lambda: choose_download(root, messagebox, filedialog, run)),
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
