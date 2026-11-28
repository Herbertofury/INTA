#!/usr/bin/env python3
"""Download a verified INTA release and copy it onto an existing Ventoy volume."""
import hashlib
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
import zipimport
from contextlib import closing
from pathlib import Path

if isinstance(__loader__, zipimport.zipimporter):
    import rescue_products as products
else:
    sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
    try:
        from native.workbench.media import rescue_products as products
    finally:
        sys.path.pop(0)

REPO = "Herbertofury/INTA"
RELEASE_API = f"https://api.github.com/repos/{REPO}/releases/tags/"
ASSET_ROOT = f"https://github.com/{REPO}/releases/download/"
ISO = "inta-rescue-amd64.iso"
SUMS = "SHA256SUMS"
PARTS = ISO + ".parts.json"
MAX_PART_SIZE = 1024 * 1024 * 1024
MAX_MANIFEST_SIZE = 1024 * 1024


expected_digest = products.expected_digest
stream_verified = products.stream_verified


def release_asset(assets, name):
    if name not in assets:
        raise ValueError(f"The release is missing {name}.")
    asset = assets[name]
    url = asset["browser_download_url"]
    if not isinstance(url, str) or not url.startswith(ASSET_ROOT) or not url.endswith("/" + name):
        raise ValueError("Unexpected release asset URL.")
    if type(asset["size"]) is not int or asset["size"] < 0:
        raise ValueError("Invalid release asset size.")
    return url, asset["size"]


def _unique_manifest_object(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError(f"Duplicate parts manifest field: {key}.")
        result[key] = value
    return result


def verified_parts(manifest, assets, expected):
    """Resolve an ordered manifest exclusively through this release's asset map."""
    fields = {"schemaVersion", "filename", "size", "sha256", "parts"}
    if not isinstance(manifest, dict) or set(manifest) != fields:
        raise ValueError("Invalid ISO parts manifest fields.")
    if type(manifest["schemaVersion"]) is not int or manifest["schemaVersion"] != 1:
        raise ValueError("Unsupported ISO parts manifest version.")
    if manifest["filename"] != ISO or type(manifest["size"]) is not int or manifest["size"] <= 0:
        raise ValueError("Invalid original ISO name or size in parts manifest.")
    digest = manifest["sha256"]
    if not isinstance(digest, str) or not re.fullmatch(r"[0-9a-fA-F]{64}", digest):
        raise ValueError("Invalid original ISO checksum in parts manifest.")
    if digest.lower() != expected:
        raise ValueError("The parts manifest disagrees with SHA256SUMS.")
    if not isinstance(manifest["parts"], list) or not manifest["parts"]:
        raise ValueError("The ISO parts manifest has no parts.")
    resolved = []
    total = 0
    for index, part in enumerate(manifest["parts"], 1):
        if not isinstance(part, dict) or set(part) != {"name", "size", "sha256"}:
            raise ValueError("Invalid ISO part fields; URLs and paths are not accepted.")
        name = f"{ISO}.part{index:03d}"
        if part["name"] != name:
            raise ValueError(f"Expected the next ISO part to be {name}.")
        size = part["size"]
        if type(size) is not int or not 0 < size <= MAX_PART_SIZE:
            raise ValueError(f"Invalid size for {name}; each part must be at most 1 GiB.")
        digest = part["sha256"]
        if not isinstance(digest, str) or not re.fullmatch(r"[0-9a-fA-F]{64}", digest):
            raise ValueError(f"Invalid checksum for {name}.")
        url, asset_size = release_asset(assets, name)
        if asset_size != size:
            raise ValueError(f"The release asset size disagrees with the manifest for {name}.")
        resolved.append((name, url, size, digest.lower()))
        total += size
    if total != manifest["size"]:
        raise ValueError("ISO part sizes do not add up to the original ISO size.")
    return total, resolved


class _VerifiedPartsReader:
    """Read one response at a time without buffering or saving whole parts."""

    def __init__(self, parts):
        self.parts = iter(parts)
        self.response = None

    def close(self):
        if self.response is not None:
            self.response.close()
            self.response = None

    def read(self, size):
        while True:
            if self.response is None:
                part = next(self.parts, None)
                if part is None:
                    return b""
                self.name, url, self.expected_size, self.expected_digest = part
                self.total = 0
                self.digest = hashlib.sha256()
                self.response = urllib.request.urlopen(url, timeout=45)
            chunk = self.response.read(size)
            if chunk:
                self.total += len(chunk)
                if self.total > self.expected_size:
                    raise ValueError(f"The download exceeds the size of {self.name}.")
                self.digest.update(chunk)
                return chunk
            self.close()
            if self.total != self.expected_size or self.digest.hexdigest() != self.expected_digest:
                raise ValueError(f"Verification failed for {self.name}. Any previous ISO is intact.")


def download_channel(edition=products.FULL):
    name = "data/" + products.CHANNELS[edition]
    if isinstance(__loader__, zipimport.zipimporter):
        data = __loader__.get_data("inta-resources/" + name)
    else:
        with (Path(__file__).resolve().parents[1] / name).open("rb") as stream:
            data = stream.read(8193)
    return products.decode(data, edition)


validated_channel = products.validate


def download_release(folder, channel=None, edition=products.FULL):
    channel = download_channel(edition) if channel is None else validated_channel(channel, edition)
    if edition == products.RECOVERY:
        if channel != download_channel(edition):
            raise ValueError("The recovery-only selection differs from the bundled product pin.")
        return products.fetch_product(folder, channel, edition)
    tag = channel["tag"]
    with urllib.request.urlopen(RELEASE_API + tag, timeout=30) as reply:
        release = json.load(reply)
    if release.get("tag_name") != tag or release.get("draft") is not False or release.get("prerelease") is not False:
        raise ValueError("The published release differs from the bundled Rescue channel.")
    assets = {}
    for asset in release["assets"]:
        name = asset["name"]
        if name in assets:
            raise ValueError(f"The release contains duplicate assets named {name}.")
        if (name in (ISO, SUMS, PARTS) or name.startswith(ISO + ".part")) and asset.get("browser_download_url") != ASSET_ROOT + tag + "/" + name:
            raise ValueError("Unexpected release asset URL.")
        assets[name] = asset
    sums_url, _ = release_asset(assets, SUMS)
    with urllib.request.urlopen(sums_url, timeout=30) as reply:
        expected = expected_digest(reply.read(65_536).decode("ascii"))
    if expected != channel["sha256"]:
        raise ValueError("The ISO checksum differs from the bundled Rescue channel.")
    parts = None
    if ISO in assets:
        image_url, image_size = release_asset(assets, ISO)
    else:
        manifest_url, manifest_size = release_asset(assets, PARTS)
        if not 0 < manifest_size <= MAX_MANIFEST_SIZE:
            raise ValueError("The ISO parts manifest must be nonempty and at most 1 MiB.")
        with urllib.request.urlopen(manifest_url, timeout=30) as reply:
            content = reply.read(manifest_size + 1)
        if len(content) != manifest_size:
            raise ValueError("The parts manifest size differs from its release asset.")
        manifest = json.loads(content, object_pairs_hook=_unique_manifest_object)
        image_size, parts = verified_parts(manifest, assets, expected)
    if shutil.disk_usage(folder).free < image_size:
        raise ValueError("The destination has insufficient free space for a safe replacement.")
    if parts is None:
        with urllib.request.urlopen(image_url, timeout=45) as reply:
            stream_verified(reply, folder / ISO, expected, image_size)
    else:
        with closing(_VerifiedPartsReader(parts)) as reply:
            stream_verified(reply, folder / ISO, expected, image_size)
    (folder / SUMS).write_text(f"{expected}  {ISO}\n", encoding="ascii")


def copy_to_ventoy(source, volume):
    filename = products.RECOVERY_ISO if source.name == products.RECOVERY_ISO else ISO
    with (source.parent / SUMS).open("rb") as manifest:
        data = manifest.read(65537)
    if len(data) > 65536:
        raise ValueError("Checksum manifest exceeds 64 KiB.")
    expected = expected_digest(data.decode("ascii"), filename)
    check = None
    if filename == products.RECOVERY_ISO:
        channel = download_channel(products.RECOVERY)
        if expected != channel["sha256"]:
            raise ValueError("The checksum differs from the recovery-only product pin.")
        def check(path):
            products.readback(path, channel, products.RECOVERY)
    if not os.path.ismount(volume):
        raise ValueError("Select the root of the mounted Ventoy data volume.")
    if shutil.disk_usage(volume).free < source.stat().st_size:
        raise ValueError("The Ventoy volume has insufficient free space for a safe replacement.")
    with source.open("rb") as image:
        stream_verified(image, volume / filename, expected, source.stat().st_size, check)


def build_app():
    import tkinter as tk
    from tkinter import filedialog, ttk
    # Resolve alongside the source/portable script, independent of current cwd.
    source_folder = Path(__file__).resolve().parent
    if str(source_folder) not in sys.path:
        sys.path.insert(0, str(source_folder))
    from inta_brand_ui import BrandUI, ScrollableFrame
    from inta_optics_ui import OpticalHeader

    root = tk.Tk()
    root.title("INTA Media Creator")
    root.geometry(f"{min(870, max(630, root.winfo_screenwidth() - 40))}x{min(650, max(420, root.winfo_screenheight() - 50))}")
    root.minsize(630, 420)
    brand_ui = BrandUI(root, script_path=__file__)
    root.inta_brands = brand_ui
    messagebox = brand_ui
    status = tk.StringVar(value="Ready")
    viewport = ScrollableFrame(root, padding=24)
    viewport.pack(fill="both", expand=True, padx=16, pady=16)
    panel = viewport.content
    header = ttk.Frame(panel)
    header.pack(fill="x", pady=(0, 10))
    header.columnconfigure(0, weight=3, minsize=155)
    header.columnconfigure(1, weight=2, minsize=160)
    appearance = OpticalHeader(header)
    appearance.grid(row=0, column=1, sticky="nsew", padx=(12, 0))
    root.inta_optics = appearance
    headings = ttk.Frame(header)
    headings.grid(row=0, column=0, sticky="new")
    brand_ui.text(headings, "INTA Media Creator", font=("TkDefaultFont", 22, "bold"), foreground="#e7e9ee", icon_size=36, fill="x")
    brand_ui.text(panel, "Download the selected INTA Rescue image or prepare an existing Ventoy volume.", fill="x", pady=(3, 12))
    brand_ui.text(panel, "Open a tool name or icon for its official download page. The action buttons keep installation and launch separate.", fill="x", pady=(0, 16))
    responses = queue.Queue()
    buttons = []
    busy = False

    def collect():
        nonlocal busy
        try:
            title, error = responses.get_nowait()
        except queue.Empty:
            root.after(150, collect)
            return
        busy = False
        for button in buttons:
            button.state(["!disabled"])
        status.set(title if error is None else "Action failed")
        if error is not None:
            messagebox.showerror("INTA Media Creator", error)

    def run(label, function, *arguments):
        nonlocal busy
        busy = True
        status.set(label + "…")
        for button in buttons:
            button.state(["disabled"])

        def worker():
            try:
                result = function(*arguments)
            except Exception as error:
                responses.put(("", str(error)))
            else:
                location = f"Verified ISO ready at {result}" if isinstance(result, Path) else "ISO ready · SHA-256 checksum matched"
                responses.put((location, None))

        threading.Thread(target=worker, daemon=True).start()
        root.after(150, collect)

    def download(edition=products.FULL):
        try:
            channel = download_channel(edition)
        except (OSError, ValueError, KeyError) as error:
            messagebox.showerror("Rescue download unavailable", str(error))
            return
        if channel["qualification"] == "historical-recovery" and not messagebox.askokcancel(
            "Historical recovery image", channel["scannerWarning"] + "\n\nContinue with the historical file-recovery download?"):
            return
        folder = filedialog.askdirectory(title="Choose where to save the Rescue ISO")
        if folder:
            run("Downloading and checking the release checksum", download_release, Path(folder), channel, edition)

    def ventoy():
        source = filedialog.askopenfilename(
            title="Select the downloaded INTA ISO next to SHA256SUMS", filetypes=[("ISO", "*.iso")]
        )
        if not source:
            return
        target = filedialog.askdirectory(title="Choose the mounted Ventoy data volume")
        filename = products.RECOVERY_ISO if Path(source).name == products.RECOVERY_ISO else ISO
        if target and messagebox.askokcancel("Copy to Ventoy", f"Install or replace {filename} on {target}?"):
            run("Verifying and copying ISO", copy_to_ventoy, Path(source), Path(target))

    def official_tool(tool):
        if tool == "Ventoy":
            names = ("Ventoy2Disk.exe",) if os.name == "nt" else ("VentoyGUI.x86_64",)
            url = "https://www.ventoy.net/en/download.html"
        else:
            if os.name != "nt":
                webbrowser.open("https://rufus.ie/en/")
                return
            names = ()
            url = "https://rufus.ie/en/"
        choice = messagebox.askyesnocancel(
            f"Open {tool}", f"Yes: choose a downloaded official {tool} executable.\n"
            f"No: open the official {tool} download page."
        )
        if choice is None:
            return
        if not choice:
            webbrowser.open(url)
            return
        selected = filedialog.askopenfilename(title=f"Select your official {tool} executable")
        if not selected:
            return
        binary = Path(selected)
        was_expected = binary.name in names if names else bool(
            re.fullmatch(r"rufus(?:-[0-9.]+p?)?\.exe", binary.name, re.IGNORECASE)
        )
        if not was_expected:
            messagebox.showerror(tool, f"Choose an official {tool} executable.")
            return
        try:
            if os.name == "nt":
                os.startfile(str(binary), cwd=str(binary.parent))
            else:
                subprocess.Popen([str(binary)], cwd=binary.parent, start_new_session=True)
        except OSError as error:
            messagebox.showerror(tool, str(error))

    def close():
        if busy:
            messagebox.showinfo("Transfer in progress", "Wait for the verification and transfer to finish.")
        else:
            root.destroy()

    actions = [
        ("INTA Rescue Recovery", "Recover files, clone disks and repair filesystems offline. This edition has no malware scanner.", "Download Recovery", lambda: download(products.RECOVERY)),
        ("INTA Rescue", "Download the image selected by the bundled Rescue channel and check its SHA-256.", "Download / update ISO", download),
        ("Ventoy", "Check the selected ISO against its adjacent SHA256SUMS and copy it to an existing Ventoy volume.", "Copy checked ISO", ventoy),
        ("Ventoy", "Open a downloaded executable, or get the official Ventoy download.", "Open / download", lambda: official_tool("Ventoy")),
        ("Rufus", "Open a downloaded executable, or get the official Rufus download.", "Open / download", lambda: official_tool("Rufus")),
    ]
    for name, description, label, callback in actions:
        row = ttk.Frame(panel, padding=(0, 11))
        row.pack(fill="x")
        row.columnconfigure(0, weight=1)
        brand_ui.text(row, name, font=("TkDefaultFont", 12, "bold"), foreground="#e7e9ee", icon_size=26).grid(row=0, column=0, sticky="ew", padx=(0, 18))
        brand_ui.text(row, description).grid(row=1, column=0, sticky="ew", padx=(0, 18))
        button = ttk.Button(row, text=label, command=callback, width=22)
        button.grid(row=0, column=1, rowspan=2, sticky="e")
        buttons.append(button)
        ttk.Separator(panel).pack(fill="x")
    status_view = brand_ui.text(panel, status.get(), foreground="#e7e9ee", fill="x", pady=(15, 0))

    def update_status(*_args):
        nonlocal status_view
        status_view.destroy()
        status_view = brand_ui.text(panel, status.get(), foreground="#e7e9ee", fill="x", pady=(15, 0))

    status.trace_add("write", update_status)
    root.protocol("WM_DELETE_WINDOW", close)
    return root


def main():
    build_app().mainloop()


if __name__ == "__main__":
    main()
