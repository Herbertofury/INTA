"""Verify an extracted Windows x64 Desktop preview before running it."""

import argparse
import hashlib
import json
from pathlib import Path
import re
import stat
import struct
import sys


EXE = "INTA.Desktop.exe"
SUMS = "SHA256SUMS"


def require_regular(path):
    info = path.lstat()
    if not stat.S_ISREG(info.st_mode) or getattr(info, "st_file_attributes", 0) & 0x400:
        raise ValueError(f"Expected a regular file: {path.name}")
    return info.st_size


def verify(directory, pin):
    if not re.fullmatch(r"[a-fA-F0-9]{64}", pin):
        raise ValueError("Supply the full executable SHA-256 from the build summary")
    if {item.name for item in directory.iterdir()} != {EXE, SUMS}:
        raise ValueError(f"Expected exactly {EXE} and {SUMS} in the extracted preview")
    expected = f"{pin.lower()}  {EXE}".encode("ascii")
    sums = directory / SUMS
    if require_regular(sums) not in (len(expected) + 1, len(expected) + 2):
        raise ValueError("Checksum receipt has an unexpected length")
    if sums.read_bytes() not in (expected + b"\n", expected + b"\r\n"):
        raise ValueError("Checksum receipt does not match the pinned executable")

    exe = directory / EXE
    length = require_regular(exe)
    with exe.open("rb") as binary:
        dos = binary.read(64)
        if len(dos) != 64 or dos[:2] != b"MZ":
            raise ValueError("Executable has no DOS header")
        offset = int.from_bytes(dos[60:64], "little")
        if not 64 <= offset <= length - (24 + 70):
            raise ValueError("Invalid PE header offset")
        binary.seek(offset)
        header = binary.read(24)
        signature, machine, sections, _, _, _, optional_size, flags = struct.unpack("<4sHHIIIHH", header)
        if (signature != b"PE\0\0" or machine != 0x8664 or not sections
                or not flags & 0x0002 or flags & 0x2000):
            raise ValueError("Expected an AMD64 PE executable")
        if optional_size < 70 or offset + 24 + optional_size + sections * 40 > length:
            raise ValueError("Truncated PE optional header or section table")
        optional = binary.read(70)
        if int.from_bytes(optional[:2], "little") != 0x020B or int.from_bytes(optional[68:70], "little") != 2:
            raise ValueError("Expected a PE32+ Windows GUI app")
        binary.seek(0)
        actual = hashlib.file_digest(binary, "sha256").hexdigest()
    if actual != pin.lower():
        raise ValueError("Downloaded executable does not match the pinned build")
    return {"file": EXE, "bytes": length, "sha256": actual, "target": "win-x64"}


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("directory", type=Path)
    parser.add_argument("--sha256", required=True, help="Independent executable SHA-256 from the workflow summary")
    args = parser.parse_args()
    try:
        print(json.dumps(verify(args.directory, args.sha256)))
    except (OSError, ValueError) as error:
        sys.exit(f"Preview verification failed: {error}")
