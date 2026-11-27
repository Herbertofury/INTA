#!/usr/bin/env python3
"""Prepare verified Rescue release assets without changing the original ISO."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import stat
import tempfile


ISO = "inta-rescue-amd64.iso"
PORTABLE = "INTA-Media-Creator.pyz"
PACKAGES = "inta-rescue-amd64.packages"
SUMS = "SHA256SUMS"
PARTS = ISO + ".parts.json"
HELP = "REASSEMBLE.txt"
ASSET_LIMIT = 2 * 1024**3  # GitHub requires every release asset to be smaller.
MAX_PART_SIZE = 1024**3
MAX_ASSETS = 1000
BUFFER_SIZE = 1024**2
REASSEMBLY_HELP = """This release preserves the original verified ISO in numbered pieces.

Download the included portable program, then choose Download / update ISO.
It retrieves and verifies the pieces and creates inta-rescue-amd64.iso.
Use that completed ISO for boot media; individual pieces are not bootable.
Keep SHA256SUMS beside the completed ISO for later verification and copying.

inta-rescue-amd64.iso.parts.json records the exact piece order, sizes and
SHA-256 checksums. SHA256SUMS retains the original complete ISO checksum.
"""


def _regular_file(path: Path):
    info = path.lstat()
    if not stat.S_ISREG(info.st_mode) or info.st_size <= 0:
        raise ValueError(f"Expected a nonempty regular file: {path.name}")
    return info


def _snapshot(info):
    return info.st_dev, info.st_ino, info.st_size, info.st_mtime_ns


def _checksums(path: Path):
    if _regular_file(path).st_size > 1024**2:
        raise ValueError("Input SHA256SUMS is unexpectedly large")
    result = {}
    for line in path.read_text(encoding="ascii").splitlines():
        if not line.strip():
            continue
        match = re.fullmatch(r"([0-9a-fA-F]{64}) [ *]([A-Za-z0-9][A-Za-z0-9._-]*)", line)
        if not match or match[2] in result:
            raise ValueError("Input SHA256SUMS has an invalid or duplicate entry")
        result[match[2]] = match[1].lower()
    for name in (ISO, PORTABLE):
        if name not in result:
            raise ValueError(f"Input SHA256SUMS must contain exactly one checksum for {name}")
    return result


def _digest_file(path: Path):
    digest = hashlib.sha256()
    size = 0
    with path.open("rb") as source:
        while block := source.read(BUFFER_SIZE):
            digest.update(block)
            size += len(block)
    return digest.hexdigest(), size


def _finish_file(path: Path, digest: str, size: int, source: Path | None = None):
    # Read the written bytes back before the staging directory can be published.
    if _digest_file(path) != (digest, size):
        raise ValueError(f"Written release asset failed verification: {path.name}")
    if source is not None:
        shutil.copystat(source, path, follow_symlinks=False)
    else:
        path.chmod(0o644)
    return {"name": path.name, "size": size, "sha256": digest}


def _copy_verified(source: Path, target: Path, expected: str | None = None):
    original = _regular_file(source)
    digest = hashlib.sha256()
    size = 0
    with source.open("rb") as incoming, target.open("xb") as outgoing:
        while block := incoming.read(BUFFER_SIZE):
            outgoing.write(block)
            digest.update(block)
            size += len(block)
        outgoing.flush()
        os.fsync(outgoing.fileno())
    value = digest.hexdigest()
    if (_snapshot(source.stat()) != _snapshot(original) or size != original.st_size
            or (expected is not None and value != expected)):
        raise ValueError(f"Source checksum or size changed: {source.name}")
    return _finish_file(target, value, size, source)


def _write_verified(target: Path, contents: str):
    data = contents.encode("utf-8")
    with target.open("xb") as outgoing:
        outgoing.write(data)
        outgoing.flush()
        os.fsync(outgoing.fileno())
    return _finish_file(target, hashlib.sha256(data).hexdigest(), len(data))


def _split_verified(source: Path, staging: Path, expected: str, part_size: int):
    original = _regular_file(source)
    full_digest = hashlib.sha256()
    total = 0
    parts = []
    with source.open("rb") as incoming:
        while total < original.st_size:
            name = f"{ISO}.part{len(parts) + 1:03d}"
            target = staging / name
            digest = hashlib.sha256()
            size = 0
            remaining = min(part_size, original.st_size - total)
            with target.open("xb") as outgoing:
                while remaining:
                    block = incoming.read(min(BUFFER_SIZE, remaining))
                    if not block:
                        raise ValueError("ISO ended before its recorded size")
                    outgoing.write(block)
                    digest.update(block)
                    full_digest.update(block)
                    size += len(block)
                    total += len(block)
                    remaining -= len(block)
                outgoing.flush()
                os.fsync(outgoing.fileno())
            parts.append(_finish_file(target, digest.hexdigest(), size, source))
        if incoming.read(1):
            raise ValueError("ISO grew while packaging")
    if (_snapshot(source.stat()) != _snapshot(original) or total != original.st_size
            or full_digest.hexdigest() != expected):
        raise ValueError("The complete ISO does not match its original SHA256SUMS entry")
    return {"schemaVersion": 1, "filename": ISO, "size": total,
            "sha256": expected, "parts": parts}


def package_release(input_dir: Path, output_dir: Path, *, part_size=MAX_PART_SIZE,
                    asset_limit=ASSET_LIMIT):
    """Create a new complete directory; lower limits support small test fixtures."""
    if (type(asset_limit) is not int or not 0 < asset_limit <= ASSET_LIMIT
            or type(part_size) is not int or not 0 < part_size <= MAX_PART_SIZE
            or part_size >= asset_limit):
        raise ValueError("Part size must be positive, at most 1 GiB and below the asset limit")
    source = Path(input_dir).resolve(strict=True)
    if not source.is_dir():
        raise ValueError("Input must be a directory")
    destination = Path(output_dir).absolute()
    if os.path.lexists(destination):
        raise FileExistsError(f"Output must not already exist: {destination}")
    destination.parent.mkdir(parents=True, exist_ok=True)
    sums = _checksums(source / SUMS)
    iso_info = _regular_file(source / ISO)
    companion_names = [PORTABLE]
    if os.path.lexists(source / PACKAGES):
        companion_names.append(PACKAGES)
    for name in companion_names:
        if _regular_file(source / name).st_size >= asset_limit:
            raise ValueError(f"Release asset must be smaller than {asset_limit} bytes: {name}")
    split = iso_info.st_size >= asset_limit
    part_count = (iso_info.st_size + part_size - 1) // part_size if split else 0
    asset_count = len(companion_names) + (part_count + 2 if split else 1) + 1
    if asset_count > MAX_ASSETS:
        raise ValueError("Packaging would exceed the release asset count limit")
    with tempfile.TemporaryDirectory(prefix=f".{destination.name}-", dir=destination.parent) as temporary:
        staging = Path(temporary)
        assets = []
        for name in companion_names:
            assets.append(_copy_verified(source / name, staging / name, sums.get(name)))
        if split:
            manifest = _split_verified(source / ISO, staging, sums[ISO], part_size)
            assets.extend(manifest["parts"])
            assets.append(_write_verified(staging / PARTS, json.dumps(manifest, indent=2) + "\n"))
            assets.append(_write_verified(staging / HELP, REASSEMBLY_HELP))
        else:
            assets.append(_copy_verified(source / ISO, staging / ISO, sums[ISO]))
        hashes = {asset["name"]: asset["sha256"] for asset in assets}
        hashes[ISO] = sums[ISO]  # Retain the complete-image hash even when its assets are split.
        checksum_text = "".join(f"{digest}  {name}\n" for name, digest in sorted(hashes.items()))
        assets.append(_write_verified(staging / SUMS, checksum_text))
        for asset in assets:
            if not 0 < asset["size"] < asset_limit:
                raise ValueError(f"Release asset exceeds the size limit: {asset['name']}")
        if os.path.lexists(destination):
            raise FileExistsError(f"Output appeared while packaging; preserved it: {destination}")
        staging.chmod(0o755)
        staging.rename(destination)
    return {"output_dir": str(destination), "iso": {"name": ISO, "size": iso_info.st_size,
            "sha256": sums[ISO]}, "split": split, "part_count": part_count,
            "package_manifest": PACKAGES if PACKAGES in companion_names else None,
            "assets": sorted(assets, key=lambda asset: asset["name"])}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input-dir", required=True, type=Path)
    parser.add_argument("--output-dir", required=True, type=Path,
                        help="New directory, created only after all checks pass")
    parser.add_argument("--part-size", type=int, default=MAX_PART_SIZE,
                        help="Maximum bytes per split part (default and maximum: 1 GiB)")
    args = parser.parse_args()
    try:
        report = package_release(args.input_dir, args.output_dir, part_size=args.part_size)
    except (OSError, ValueError) as error:
        parser.exit(1, f"Release packaging failed: {error}\n")
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
