"""Verified entry point staged by the Rust CLI inside its private scratch directory."""

import hashlib
import os
from pathlib import Path
import runpy
import sys
import tempfile

engine = Path("/usr/local/bin/inta-cloud-hypervisor")
pin = "ce7133bf535ed5a4e2a63ea1f26025445a5b1241f73ddfb81ff2fbcb2e803a59"
if os.geteuid() == 0 or not os.access("/dev/kvm", os.R_OK | os.W_OK):
    sys.exit(
        "The prepared lab requires a dedicated nonroot Linux account with KVM access."
    )
with engine.open("rb") as stream:
    if hashlib.file_digest(stream, "sha256").hexdigest() != pin:
        sys.exit("The installed Cloud Hypervisor failed its release checksum.")
root = Path(__file__).resolve().parent
mode, iso, *base = sys.argv[1:]
if mode == "console":
    script = root / "scripts/cloud-rescue-console.py"
    sys.argv = [str(script), "--iso", iso, "--executable", str(engine)]
    if base:
        sys.argv.extend(["--base", base[0]])
    runpy.run_path(str(script), run_name="__main__")
elif mode == "repair":
    script = root / "scripts/smoke-rescue-repair.py"
    with tempfile.TemporaryDirectory(prefix="inta-lab-proof-") as folder:
        sys.argv = [
            str(script),
            iso,
            str(Path(folder) / "proof"),
            "--backend",
            "cloud-hypervisor",
            "--executable",
            str(engine),
        ]
        runpy.run_path(str(script), run_name="__main__")
else:
    sys.exit("Unknown Rust lab profile.")
