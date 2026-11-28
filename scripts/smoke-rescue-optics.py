#!/usr/bin/env python3
"""Exercise the installed optical launcher on a disposable, networkless QEMU display."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import runpy
import shutil
import socket
import subprocess
import sys
import tempfile
import time


def guest_test():
    import tkinter as tk
    from tkinter import ttk
    original = runpy.run_path("/usr/local/bin/inta-rescue")
    root = original["build_app"]()
    root.lift()
    root.update()
    header = root.inta_optics
    assert header.native is not None, "the installed Rust field did not load"
    tabs = next(w for w in root.winfo_children() if isinstance(w, ttk.Notebook))
    assert len(tabs.tabs()) == 3
    # Settings in a synthetic live account should not outlive this standalone test.
    with tempfile.TemporaryDirectory() as temp:
        header.settings = Path(temp) / "profile"
        for name in ("Full Prism", "Classic"):
            header.profile.set(name)
            header.choose()
            root.update()
            assert bool(header.cells) == (name == "Full Prism")
            assert header.settings.read_text().strip() == name
            print("\nINTA_OPTICAL_VISIBLE_" + name.split()[0].upper(), flush=True)
            root.after(2200, root.quit)
            root.mainloop()
        header.profile.set("Professional")
        header.choose()
        root.update()
        header.canvas.event_generate("<ButtonPress-1>", x=20, y=15)
        root.update()
        assert header.pending is not None and header.impulses
        root.after(1750, root.quit)
        root.mainloop()
        assert header.pending is None and not header.impulses
        tabs.select(1)
        root.update()
        assert tabs.tab(tabs.select(), "text") == "System & Security"
    root.destroy()
    print("INTA_OPTICAL_GUEST_PASS", flush=True)


def boot_files(image, work):
    source = f"stdio:{image}"
    listing = subprocess.check_output(["xorriso", "-indev", source, "-ls", "/live"],
                                      text=True, stderr=subprocess.DEVNULL)
    files = [line.strip("'") for line in listing.splitlines()]
    kernels = [name for name in files if re.fullmatch(r"vmlinuz(?:-[A-Za-z0-9._+-]+)?", name)]
    if len(kernels) != 1 or (initrd := kernels[0].replace("vmlinuz", "initrd.img", 1)) not in files:
        raise ValueError("Expected one matching Debian live kernel and initramfs")
    subprocess.run(["xorriso", "-osirrox", "on", "-indev", source, "-extract", "/live/"+kernels[0],
                    work/"kernel", "-extract", "/live/"+initrd, work/"initrd"], check=True,
                   stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)


def capture(monitor, target):
    with socket.socket(socket.AF_UNIX) as connection:
        connection.settimeout(5)
        connection.connect(str(monitor))
        def prompt():
            result = bytearray()
            while b"(qemu)" not in result:
                part = connection.recv(4096)
                if not part:
                    raise OSError("QEMU closed the monitor before acknowledging the screenshot")
                result.extend(part)
        prompt()
        connection.sendall(f'screendump "{target}"\n'.encode())
        prompt()
    if not target.is_file() or target.stat().st_size < 4096:
        raise ValueError("Missing guest screenshot")


def host(image, expected, dest, *, guest_script=None, verified_tabs=None):
    image = image.resolve(strict=True)
    if not image.is_file() or not re.fullmatch(r"[0-9a-f]{64}", expected):
        raise ValueError("Select a regular ISO and provide its SHA-256")
    with image.open("rb") as stream:
        if hashlib.file_digest(stream, "sha256").hexdigest() != expected:
            raise ValueError("ISO checksum mismatch")
    dest.mkdir(parents=True, exist_ok=False)
    with tempfile.TemporaryDirectory(prefix="inta-optical-") as folder:
        work = Path(folder)
        if any(ch in str(p) for p in (image, work) for ch in ',\n\r"'):
            raise ValueError("Unsupported QEMU path")
        boot_files(image, work)
        seed = work/"seed"
        seed.mkdir()
        shutil.copyfile(guest_script or __file__, seed/"test.py")
        (seed/"start.sh").write_text('''#!/bin/bash
set -euo pipefail
exec >/dev/ttyS0 2>&1
systemctl start live-config.service
systemctl --no-block start graphical.target
for i in $(seq 1 150); do
    if pgrep -u inta -x xfce4-session >/dev/null 2>&1 && test -e /home/inta/.Xauthority; then break; fi
    sleep 1
done
uid=$(id -u inta)
runuser -u inta -- env HOME=/home/inta DISPLAY=:0 XAUTHORITY=/home/inta/.Xauthority XDG_RUNTIME_DIR="/run/user/$uid" \
  DBUS_SESSION_BUS_ADDRESS="unix:path=/run/user/$uid/bus" python3 /run/optical-fixture/test.py --guest
''')
        control = work/"fixture.iso"
        subprocess.run(["xorriso", "-as", "mkisofs", "-quiet", "-R", "-V", "INTAOPTICAL", "-o", control, seed], check=True)
        cmdline = ('boot=live components username=inta hostname=inta quiet console=ttyS0 live-media=/dev/vda '
                   'systemd.mask=serial-getty@ttyS0.service systemd.mount-extra=LABEL=INTAOPTICAL:/run/optical-fixture:iso9660:ro '
                   'systemd.run="/bin/bash /run/optical-fixture/start.sh" systemd.run_success_action=poweroff '
                   'systemd.run_failure_action=poweroff')
        monitor = work/"monitor"
        serial = dest/"guest-serial.log"
        kvm = os.access("/dev/kvm", os.R_OK | os.W_OK)
        command = ["qemu-system-x86_64", "-machine", "q35,accel="+("kvm" if kvm else "tcg"),
                   "-cpu", "host" if kvm else "max", "-smp", "2", "-m", "3072", "-nic", "none",
                   "-display", "none", "-monitor", f"unix:{monitor},server=on,wait=off", "-no-reboot",
                   "-serial", f"file:{serial}", "-kernel", work/"kernel", "-initrd", work/"initrd", "-append", cmdline,
                   "-drive", f"if=virtio,format=raw,readonly=on,file={image}",
                   "-drive", f"if=virtio,format=raw,readonly=on,file={control}"]
        with subprocess.Popen(command) as process:
            try:
                timeout = time.monotonic() + (250 if kvm else 600)
                seen = set()
                while process.poll() is None:
                    if time.monotonic() > timeout:
                        raise TimeoutError("Guest graphical proof timed out")
                    lines = serial.read_text(errors="replace").splitlines() if serial.exists() else []
                    for name in ("FULL", "CLASSIC"):
                        if name not in seen and "INTA_OPTICAL_VISIBLE_"+name in lines:
                            frame = work/(name+".ppm")
                            capture(monitor, frame)
                            shutil.copyfile(frame, dest/frame.name)
                            seen.add(name)
                    time.sleep(.2)
            finally:
                if process.poll() is None:
                    process.terminate()
                    process.wait(timeout=10)
        if process.returncode or len(seen) != 2 or "INTA_OPTICAL_GUEST_PASS" not in serial.read_text(errors="replace").splitlines():
            raise RuntimeError("Guest UI failed; inspect "+str(serial))
        result = {"sha256":expected, "acceleration":"kvm" if kvm else "tcg", "profiles":["Full Prism","Classic","Professional"],
                  "nativeRenderer":True, "systemTab":guest_script is None, "settled":True, "networkDevices":0, "writableGuestDisks":0}
        if verified_tabs is not None:
            result["verifiedTabs"] = verified_tabs
        (dest/"evidence.json").write_text(json.dumps(result, indent=2)+"\n")
        print(json.dumps(result))


if __name__ == "__main__":
    if sys.argv[1:] == ["--guest"]:
        guest_test()
    else:
        parser = argparse.ArgumentParser(description=__doc__)
        parser.add_argument("iso", type=Path)
        parser.add_argument("sha256")
        parser.add_argument("new_output_directory", type=Path)
        args = parser.parse_args()
        host(args.iso, args.sha256, args.new_output_directory.resolve())
