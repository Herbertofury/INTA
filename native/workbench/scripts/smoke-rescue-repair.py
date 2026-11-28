#!/usr/bin/env python3
"""Boot INTA offline, clone a damaged FAT fixture, repair its copy and prove recovery."""
import argparse
import hashlib
import importlib
import json
from pathlib import Path
import re
import shutil
import struct
import subprocess
import sys
import tempfile
import uuid

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))
media = importlib.import_module("media.inta-media")


def sha256(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def new_volume(path, size, label):
    with path.open("xb") as stream:
        stream.truncate(size)
    subprocess.run(["mformat", "-i", path, "-v", label, "::"], check=True)


def prepare_fixture(work):
    source = work / "source.raw"
    destination = work / "destination.raw"
    new_volume(source, 32 * 1024**2, "INTASRC")
    new_volume(destination, 96 * 1024**2, "INTAOUT")
    with source.open("rb") as stream:
        bpb = stream.read(512)
    sector = struct.unpack_from("<H", bpb, 11)[0]
    cluster = sector * bpb[13]
    root = (struct.unpack_from("<H", bpb, 14)[0] + bpb[16] * struct.unpack_from("<H", bpb, 22)[0]) * sector
    payload = work / "RECOVER.TXT"
    payload.write_bytes((b"INTA reproducible recovery fixture\n" * cluster)[:cluster])
    subprocess.run(["mcopy", "-i", source, payload, "::RECOVER.TXT"], check=True)
    # The fixture has a one-cluster file with an impossible two-cluster size.
    with source.open("r+b") as stream:
        stream.seek(root)
        directory = stream.read(struct.unpack_from("<H", bpb, 17)[0] * 32)
        index = next(i for i in range(0, len(directory), 32) if directory[i:i + 11] == b"RECOVER TXT")
        stream.seek(root + index + 28)
        stream.write(struct.pack("<I", 2 * cluster))
    nonce = uuid.uuid4().hex
    marker = work / "QA.ID"
    marker.write_text(nonce, encoding="ascii")
    subprocess.run(["mcopy", "-i", destination, marker, "::QA.ID"], check=True)
    seed = work / "seed"
    seed.mkdir()
    original, expected = sha256(source), sha256(payload)
    (seed / "fixture.env").write_text(f"SOURCE_HASH={original}\nFILE_HASH={expected}\nFIXTURE_ID={nonce}\n", encoding="ascii")
    shutil.copyfile(ROOT / "tests/rescue-lab/repair.sh", seed / "repair.sh")
    control = work / "lab.iso"
    subprocess.run(["xorriso", "-as", "mkisofs", "-quiet", "-R", "-V", "INTALAB", "-o", control, seed], check=True)
    return source, destination, control, original, expected


def extract_boot_files(image, work):
    address = f"stdio:{image}"
    rows = subprocess.check_output(["xorriso", "-indev", address, "-ls", "/live"], text=True, stderr=subprocess.DEVNULL)
    names = [line.strip("'") for line in rows.splitlines()]
    kernels = [name for name in names if re.fullmatch(r"vmlinuz(?:-[a-zA-Z0-9._+-]+)?", name)]
    if len(kernels) != 1:
        raise ValueError("Expected one Debian live kernel in the verified ISO.")
    initrd = kernels[0].replace("vmlinuz", "initrd.img", 1)
    if initrd not in names:
        raise ValueError("The matching initramfs is missing.")
    kernel, ram = work / "vmlinuz", work / "initrd"
    subprocess.run(["xorriso", "-osirrox", "on", "-indev", address, "-extract",
                    f"/live/{kernels[0]}", kernel, "-extract", f"/live/{initrd}", ram], check=True,
                   stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    return kernel, ram


def verified_lab_iso(image):
    if image.name == media.products.RECOVERY_ISO:
        proof = json.loads((ROOT / "vm-lab/cloud/recovery-proof.json").read_bytes())
        info = media.inspect_product(image)
        if proof["passed"] is not True or proof["rescueISO"] != info["sha256"]:
            raise ValueError("This recovery-only image has no matching Cloud Hypervisor qualification.")
        return info
    if image.name != media.ISO:
        raise ValueError("Choose a supported INTA Debian amd64 lab ISO.")
    info = media.inspect_product(image)
    if info["volumeLabel"] != "INTA_RESCUE":
        raise ValueError("The image does not identify itself as INTA Rescue.")
    return info


def run(args):
    image = args.iso.resolve(strict=True)
    if not image.is_file():
        raise ValueError("Select a regular ISO file.")
    paths = [image, args.output.resolve(), Path(tempfile.gettempdir())]
    if any(any(char in str(path) for char in ",\r\n") for path in paths):
        raise ValueError("VMM device paths must be representable as single option values.")
    info = verified_lab_iso(image)
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    serial = output / "guest-serial.log"
    with tempfile.TemporaryDirectory(prefix="inta-repair-") as temporary:
        work = Path(temporary)
        source, destination, control, original, expected = prepare_fixture(work)
        kernel, initrd = extract_boot_files(image, work)
        kernel_args = ('boot=live components username=inta hostname=inta quiet console=ttyS0 live-media=/dev/vda '
                       'systemd.mount-extra=LABEL=INTALAB:/run/inta-lab:iso9660:ro '
                       'systemd.run="/bin/bash /run/inta-lab/repair.sh" '
                       'systemd.run_success_action=poweroff systemd.run_failure_action=poweroff')
        disks = ((image, True), (control, True), (source, True), (destination, False))
        if args.backend == "cloud-hypervisor":
            binary = args.executable or "cloud-hypervisor"
            command = [binary, "--cpus", "boot=2", "--memory", "size=2G", "--kernel", kernel, "--initramfs", initrd,
                       "--cmdline", kernel_args, "--console", "off", "--serial", f"file={serial}", "--disk"]
            command.extend(f"path={p},image_type=raw,readonly={'on' if ro else 'off'}" for p, ro in disks)
        else:
            binary = args.executable or "qemu-system-x86_64"
            command = [binary, "-machine", "q35,accel=kvm", "-cpu", "host", "-smp", "2", "-m", "2048", "-nic", "none",
                       "-display", "none", "-monitor", "none", "-serial", f"file:{serial}", "-kernel", kernel,
                       "-initrd", initrd, "-append", kernel_args, "-no-reboot"]
            for p, ro in disks:
                command.extend(["-drive", f"if=virtio,format=raw,readonly={'on' if ro else 'off'},file={p}"])
        version = subprocess.check_output([binary, "--version"], text=True).splitlines()[0]
        with (output / "vmm.log").open("wb") as log:
            subprocess.run(command, stdout=log, stderr=subprocess.STDOUT, check=True, timeout=300)
        log_lines = serial.read_bytes().splitlines()
        if b"INTA_REPAIR_PASS" not in log_lines or f"INTA_RECOVERED_SHA256={expected}".encode() not in log_lines:
            raise ValueError(f"Guest recovery verification failed; inspect {serial}.")
        if sha256(source) != original:
            raise ValueError("The read-only source changed.")
        proof = {"schemaVersion": 1, "vmm": version, "rescueISO": info["sha256"], "kernelSHA256": sha256(kernel),
                 "initrdSHA256": sha256(initrd), "sourceSHA256BeforeAndAfter": original, "recoveredFileSHA256": expected,
                 "workload": "ddrescue clone, FAT repair on copy, clean check and recovered-file readback",
                 "networkDevices": 0, "passed": True}
        (output / "evidence.json").write_text(json.dumps(proof, indent=2) + "\n", encoding="utf-8")
        print(json.dumps(proof))


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("iso", type=Path)
    parser.add_argument("output", type=Path, help="New evidence directory")
    parser.add_argument("--backend", choices=("cloud-hypervisor", "qemu"), required=True)
    parser.add_argument("--executable", help="Path to the trusted host VMM")
    try:
        run(parser.parse_args())
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        sys.exit(str(error))
