#!/usr/bin/env python3
"""Start an interactive, offline INTA Rescue guest with a disposable working disk."""
import argparse
import hashlib
import importlib
import os
from pathlib import Path
import runpy
import shutil
import signal
import stat
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))
media = importlib.import_module("media.inta-media")
VMM_SHA256 = "ce7133bf535ed5a4e2a63ea1f26025445a5b1241f73ddfb81ff2fbcb2e803a59"


def private_copy(base, working):
    if base is None:
        with working.open("xb") as stream:
            stream.truncate(256 * 1024**2)
        return
    if base.suffix.lower() != ".img":
        raise ValueError("Choose a regular raw .img file.")
    fd = os.open(base, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
    with os.fdopen(fd, "rb") as source:
        before = os.fstat(fd)
        if not stat.S_ISREG(before.st_mode) or before.st_size == 0 or before.st_size % 512:
            raise ValueError("Use a nonempty regular raw image with whole 512-byte sectors.")
        header = source.read(512)
        if header.startswith((b"QFI\xfb", b"KDMV", b"vhdxfile", b"conectix", b"<<< Oracle VM VirtualBox")):
            raise ValueError("Convert the source to a raw image before starting the lab.")
        source.seek(-512, os.SEEK_END)
        if source.read(8) == b"conectix":
            raise ValueError("Convert the VHD to a raw image before starting the lab.")
        source.seek(0)
        subprocess.run(["cp", "--reflink=auto", "--sparse=always", "--", f"/proc/self/fd/{fd}", working],
                       pass_fds=(fd,), check=True)
        after = os.fstat(fd)
        if (before.st_size, before.st_mtime_ns, before.st_ctime_ns) != (after.st_size, after.st_mtime_ns, after.st_ctime_ns):
            raise ValueError("The base image changed while the private working copy was being created.")


def launch(args):
    if not sys.stdin.isatty() or not sys.stdout.isatty():
        raise ValueError("An interactive terminal is required for the serial console.")
    if not os.access("/dev/kvm", os.R_OK | os.W_OK):
        raise ValueError("Linux KVM access is required for the lab account.")
    binary = args.executable.resolve(strict=True)
    if not binary.is_file():
        raise ValueError("Select the pinned Cloud Hypervisor v55.0 executable.")
    with binary.open("rb") as stream:
        if hashlib.file_digest(stream, "sha256").hexdigest() != VMM_SHA256:
            raise ValueError("The Cloud Hypervisor executable does not match the approved release.")
    image = args.iso.resolve(strict=True)
    if image.name != media.ISO or not image.is_file():
        raise ValueError("Choose the INTA Rescue AMD64 ISO with its checksum manifest.")
    if any(ch in str(path) for ch in ",\r\n" for path in (image, Path(tempfile.gettempdir()))):
        raise ValueError("Use ISO and temporary paths representable as single VMM disk values.")
    print("Checking the rescue ISO and preparing a private working disk...", flush=True)
    info = media.verify_iso(image, media.image_digest(image))
    if info["volumeLabel"] != "INTA_RESCUE":
        raise ValueError("The ISO does not identify itself as INTA Rescue.")
    with tempfile.TemporaryDirectory(prefix="inta-cloud-console-") as temporary:
        work = Path(temporary)
        scratch = work / "working.raw"
        private_copy(args.base, scratch)
        helpers = runpy.run_path(str(ROOT / "scripts/smoke-rescue-repair.py"))
        kernel, initrd = helpers["extract_boot_files"](image, work)
        seed = work / "control"
        seed.mkdir()
        shutil.copyfile(ROOT / "tests/rescue-lab/console.sh", seed / "console.sh")
        control = work / "control.iso"
        subprocess.run(["xorriso", "-as", "mkisofs", "-quiet", "-R", "-V", "INTACTL", "-o", control, seed], check=True)
        cmdline = ('boot=live components username=inta hostname=inta quiet console=ttyS0 live-media=/dev/vda '
                   'systemd.mask=serial-getty@ttyS0.service '
                   'systemd.mount-extra=LABEL=INTACTL:/run/inta-control:iso9660:ro '
                   'systemd.run="/bin/bash /run/inta-control/console.sh" '
                   'systemd.run_success_action=poweroff systemd.run_failure_action=poweroff')
        command = [binary, "--cpus", "boot=2", "--memory", "size=2G", "--kernel", kernel,
                   "--initramfs", initrd, "--cmdline", cmdline, "--console", "off", "--serial", "tty", "--disk"]
        command.extend(f"path={path},image_type=raw,readonly={access}" for path, access in
                       [(image, "on"), (control, "on"), (scratch, "off")])
        with (work / "vmm.log").open("wb") as log:
            guest = subprocess.Popen(command, stderr=log)
            try:
                status = guest.wait()
            finally:
                if guest.poll() is None:
                    guest.terminate()
                    try:
                        guest.wait(timeout=8)
                    except subprocess.TimeoutExpired:
                        guest.kill()
                        guest.wait()
        if status:
            print((work / "vmm.log").read_text(errors="replace")[-2048:], file=sys.stderr)
            raise ValueError(f"Cloud Hypervisor exited with status {status}.")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--iso", type=Path, required=True)
    parser.add_argument("--base", type=Path, help="optional raw .img to copy privately; default is a blank 256 MiB disk")
    parser.add_argument("--executable", type=Path, default=Path("/usr/local/bin/inta-cloud-hypervisor"))
    def disconnected(signum, frame):
        raise SystemExit(128 + signum)
    signal.signal(signal.SIGHUP, disconnected)
    signal.signal(signal.SIGTERM, disconnected)
    try:
        launch(parser.parse_args())
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        print(error, file=sys.stderr)
        sys.exit(1)
