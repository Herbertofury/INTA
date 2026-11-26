#!/usr/bin/env python3
"""Download a verified INTA release and copy it onto an existing Ventoy volume."""
import hashlib
import json
import os
import queue
import re
import shutil
import subprocess
import tempfile
import threading
import tkinter as tk
import urllib.request
import webbrowser
from pathlib import Path
from tkinter import filedialog, messagebox, ttk

REPO = "Herbertofury/INTA"
RELEASE_API = f"https://api.github.com/repos/{REPO}/releases/latest"
ASSET_ROOT = f"https://github.com/{REPO}/releases/download/"
ISO = "inta-rescue-amd64.iso"
SUMS = "SHA256SUMS"


def expected_digest(data):
    pattern = rf"([0-9a-f]{{64}}) [ *]{re.escape(ISO)}"
    matches = [match.group(1) for line in data.splitlines()
               if (match := re.fullmatch(pattern, line))]
    if len(matches) != 1:
        raise ValueError(f"Expected exactly one checksum for {ISO}.")
    return matches[0]


def stream_verified(source, target, expected, expected_size=None):
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
            partial.replace(target)
        finally:
            if partial.exists():
                output.close()
                partial.unlink()


def release_asset(assets, name):
    asset = assets[name]
    url = asset["browser_download_url"]
    if not url.startswith(ASSET_ROOT) or not url.endswith("/" + name):
        raise ValueError("Unexpected release asset URL.")
    return url, asset["size"]


def download_release(folder):
    with urllib.request.urlopen(RELEASE_API, timeout=30) as reply:
        release = json.load(reply)
    assets = {asset["name"]: asset for asset in release["assets"]}
    sums_url, _ = release_asset(assets, SUMS)
    image_url, image_size = release_asset(assets, ISO)
    with urllib.request.urlopen(sums_url, timeout=30) as reply:
        expected = expected_digest(reply.read(65_536).decode("ascii"))
    if shutil.disk_usage(folder).free < image_size:
        raise ValueError("The destination has insufficient free space for a safe replacement.")
    with urllib.request.urlopen(image_url, timeout=45) as reply:
        stream_verified(reply, folder / ISO, expected, image_size)
    (folder / SUMS).write_text(f"{expected}  {ISO}\n", encoding="ascii")


def copy_to_ventoy(source, volume):
    expected = expected_digest((source.parent / SUMS).read_text(encoding="ascii"))
    if not os.path.ismount(volume):
        raise ValueError("Select the root of the mounted Ventoy data volume.")
    if shutil.disk_usage(volume).free < source.stat().st_size:
        raise ValueError("The Ventoy volume has insufficient free space for a safe replacement.")
    with source.open("rb") as image:
        stream_verified(image, volume / ISO, expected, source.stat().st_size)


def main():
    root = tk.Tk()
    root.title("INTA Media Creator")
    root.minsize(630, 420)
    root.configure(background="#102734")
    status = tk.StringVar(value="Ready")
    panel = ttk.Frame(root, padding=24)
    panel.pack(fill="both", expand=True, padx=16, pady=16)
    ttk.Label(panel, text="INTA Media Creator", font=("TkDefaultFont", 22, "bold")).pack(anchor="w")
    ttk.Label(panel, text="Download a refreshed rescue ISO and prepare your boot USB.").pack(anchor="w", pady=(3, 16))
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
                function(*arguments)
            except Exception as error:
                responses.put(("", str(error)))
            else:
                responses.put(("Verified ISO ready", None))

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
        if target and messagebox.askokcancel("Copy to Ventoy", f"Install or replace {ISO} on {target}?"):
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
        ("Download or update INTA ISO", download),
        ("Install or update ISO on Ventoy USB", ventoy),
        ("Open or download Ventoy", lambda: official_tool("Ventoy")),
        ("Open or download Rufus", lambda: official_tool("Rufus")),
    ]
    for label, callback in actions:
        button = ttk.Button(panel, text=label, command=callback)
        button.pack(fill="x", pady=5, ipady=4)
        buttons.append(button)
    ttk.Label(panel, textvariable=status, wraplength=540).pack(anchor="w", pady=(12, 0))
    root.protocol("WM_DELETE_WINDOW", close)
    root.mainloop()


if __name__ == "__main__":
    main()
