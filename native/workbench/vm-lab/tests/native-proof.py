"""Reproduce the native UEFI repair proof in a fresh test directory on Windows."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import time

HERE = Path(__file__).resolve().parent


def run(args, **kwargs):
    return subprocess.run(
        [str(x) for x in args],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        timeout=kwargs.pop("timeout", 180),
        check=True,
        **kwargs,
    ).stdout


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--cli", required=True, type=Path)
    p.add_argument("--toolbox", required=True, type=Path)
    p.add_argument(
        "--work",
        required=True,
        type=Path,
        help="new evidence directory on a local disk",
    )
    p.add_argument("--wsl", default="INTA-Rescue-Builder")
    args = p.parse_args()
    work = args.work.absolute()
    work.mkdir(exist_ok=False, parents=False)
    cli = args.cli.resolve(strict=True)
    toolbox = args.toolbox.resolve(strict=True)
    env = dict(
        os.environ,
        INTA_DATA_DIR=str(work / "app-state"),
        INTA_VM_LAB_DIR=str(work / "sessions"),
        INTA_TOOLBOX=str(toolbox),
    )

    def api(*argv):
        return json.loads(run([cli, "vm", *argv], env=env, timeout=300))

    def linux(*argv, check=True):
        proc = subprocess.run(
            ["wsl", "-d", args.wsl, "--cd", str(work), "--", *argv],
            capture_output=True,
            timeout=60,
            check=check,
        )
        return proc

    vswhere = (
        Path(os.environ["ProgramFiles(x86)"])
        / "Microsoft Visual Studio/Installer/vswhere.exe"
    )
    installation = Path(
        run(
            [
                vswhere,
                "-latest",
                "-requires",
                "Microsoft.VisualStudio.Component.VC.Tools.x86.x64",
                "-property",
                "installationPath",
            ]
        )
        .decode()
        .strip()
    )
    compiler = max(
        (installation / "VC/Tools/MSVC").glob("*/bin/Hostx64/x64/cl.exe"),
        key=lambda s: tuple(map(int, s.parents[3].name.split("."))),
    )
    run(
        [
            compiler,
            "/nologo",
            "/GS-",
            "/Zl",
            "/O1",
            "/c",
            "/Fofixture.obj",
            HERE / "repair-fixture.c",
        ],
        cwd=work,
    )
    run(
        [
            compiler.parent / "link.exe",
            "/nologo",
            "/nodefaultlib",
            "/subsystem:efi_application",
            "/machine:x64",
            "/entry:efi_main",
            "/out:BOOTX64.EFI",
            "fixture.obj",
        ],
        cwd=work,
    )
    optical = work / "optical"
    optical.mkdir()
    with (optical / "boot.img").open("xb") as f:
        f.truncate(4 * 1024 * 1024)
    linux("mformat", "-i", "optical/boot.img", "::")
    linux("mmd", "-i", "optical/boot.img", "::EFI", "::EFI/BOOT")
    linux("mcopy", "-i", "optical/boot.img", "BOOTX64.EFI", "::EFI/BOOT/BOOTX64.EFI")
    iso = work / "repair-fixture.iso"
    linux(
        "xorriso",
        "-as",
        "mkisofs",
        "-o",
        iso.name,
        "-V",
        "INTA_LAB",
        "-e",
        "boot.img",
        "-no-emul-boot",
        "optical",
    )
    src = work / "source.img"
    with src.open("xb") as f:
        f.truncate(1474560)
    linux("mformat", "-i", src.name, "-f", "1440", "-v", "INTALABCHK", "::")
    payload = b"INTA verified isolated repair\n"
    (work / "ORIGINAL.TXT").write_bytes(payload)
    linux("mcopy", "-i", src.name, "ORIGINAL.TXT", "::ORIGINAL.TXT")
    good = src.read_bytes()
    if good[43:54] != b"INTALABCHK " or good[11:13] != b"\0\x02":
        raise ValueError("Unexpected FAT fixture geometry")
    with src.open("r+b") as f:
        f.seek(11)
        f.write(b"\0\0")
        f.seek(400)
        f.write(b"INTA-OFFLINE-FIXTURE-V1")
    broken = src.read_bytes()
    if linux("mtype", "-i", src.name, "::ORIGINAL.TXT", check=False).returncode == 0:
        raise ValueError("Negative control was readable")
    status = api("status")
    if not status["ready"]:
        raise ValueError(status)
    session = api("start", iso, "--base", src, "--approve")
    deadline = time.monotonic() + 90
    while session["state"] in ("preparing", "running") and time.monotonic() < deadline:
        time.sleep(0.5)
        session = next(
            s for s in api("sessions")["sessions"] if s["id"] == session["id"]
        )
    if session["state"] != "exited":
        if session["state"] == "running":
            api("stop", session["id"], "--approve")
        raise ValueError("The fixture did not exit cleanly")
    folder = Path(session["directory"])
    engine = Path(status["runtimePath"]) / "bin/qemu-img.exe"
    run(
        [
            engine,
            "convert",
            "-f",
            "qcow2",
            "-O",
            "raw",
            folder / "disk.qcow2",
            work / "repaired.img",
        ]
    )
    disk = (work / "repaired.img").read_bytes()
    digest = lambda b: hashlib.sha256(b).hexdigest()
    if src.read_bytes() != broken or (folder / "base.raw").read_bytes() != broken:
        raise ValueError("Source base changed")
    if disk != broken[:12] + b"\x02" + broken[13:]:
        raise ValueError("Repair must change exactly the intended FAT BPB byte")
    if linux("mtype", "-i", "repaired.img", "::ORIGINAL.TXT").stdout != payload:
        raise ValueError("Recovered content differs")
    report = {
        "state": "passed",
        "backend": "QEMU " + status["version"],
        "accelerator": session["accelerator"],
        "network": session["network"],
        "sourceSha256": digest(broken),
        "repairedSha256": digest(disk),
        "payloadSha256": digest(payload),
        "onlyChangedByteOffset": 12,
        "sourcePreserved": True,
        "guestExitedNormally": True,
    }
    (work / "proof.json").write_text(
        json.dumps(report, indent=2) + "\n", encoding="utf8"
    )
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
