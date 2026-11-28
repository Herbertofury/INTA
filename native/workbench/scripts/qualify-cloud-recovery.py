#!/usr/bin/env python3
"""Qualify a new offline recovery image before recording its Cloud Hypervisor proof."""
import argparse
import importlib
import re
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
lab = importlib.import_module("smoke-rescue-repair")

# The released x86_64 Linux static Cloud Hypervisor v55.0 binary in the Toolbox.
ENGINE_SHA256 = "ce7133bf535ed5a4e2a63ea1f26025445a5b1241f73ddfb81ff2fbcb2e803a59"


def qualify(args):
    if not re.fullmatch(r"[0-9a-f]{64}", args.sha256) or not 0 < args.bytes <= lab.media.products.MAX_IMAGE:
        raise ValueError("Supply the exact build receipt SHA-256 and byte count.")
    if args.iso.name != lab.media.products.RECOVERY_ISO or args.iso.is_symlink():
        raise ValueError("Select the regular recovery-only edition ISO.")
    engine = Path(args.executable).resolve(strict=True)
    if not engine.is_file() or lab.sha256(engine) != ENGINE_SHA256:
        raise ValueError("Select the qualified Linux x86_64 Cloud Hypervisor v55.0 executable.")
    args.executable = engine
    args.backend = "cloud-hypervisor"

    def inspect_candidate(image):
        return lab.media.products.readback(image, {"sha256": args.sha256, "bytes": args.bytes},
                                           lab.media.products.RECOVERY)

    # The regular application only accepts the release pin plus its recorded guest proof.
    # This isolated qualification exercises the same guest and writes only new evidence.
    lab.run(args, inspect_image=inspect_candidate)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("iso", type=Path)
    parser.add_argument("output", type=Path, help="New directory for the serial log and guest proof")
    parser.add_argument("--sha256", required=True, help="Independent build receipt SHA-256")
    parser.add_argument("--bytes", required=True, type=int, help="Independent build receipt byte count")
    parser.add_argument("--executable", required=True, help="Pinned Linux x86_64 Cloud Hypervisor v55.0 binary")
    try:
        qualify(parser.parse_args())
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        sys.exit(str(error))
