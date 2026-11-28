"""Inspect the ISO9660 primary descriptor and the El Torito boot catalog."""
import hashlib
import os
import struct


def inspect_iso(stream):
    size = os.fstat(stream.fileno()).st_size
    sector = 2048
    def block(lba):
        if lba < 0 or (lba + 1) * sector > size:
            raise ValueError("ISO references a sector outside the image.")
        stream.seek(lba * sector)
        data = stream.read(sector)
        if len(data) != sector:
            raise ValueError("Incomplete ISO sector.")
        return data

    volume, catalog = None, None
    for lba in range(16, 80):
        descriptor = block(lba)
        if descriptor[1:7] != b"CD001\x01":
            raise ValueError("Invalid ISO9660 volume descriptor.")
        if descriptor[0] == 255:
            break
        if descriptor[0] == 1:
            volume = descriptor[40:72].decode("ascii").strip(" \0")
        if descriptor[0] == 0 and descriptor[7:39].rstrip(b" \0") == b"EL TORITO SPECIFICATION":
            catalog = struct.unpack_from("<I", descriptor, 71)[0]
    else:
        raise ValueError("Incomplete ISO9660 volume descriptor sequence.")
    if not volume or not catalog:
        raise ValueError("Missing volume label or El Torito boot catalog.")
    entries = block(catalog)
    header = entries[:32]
    if header[0] != 1 or header[30:] != b"\x55\xaa" or sum(struct.unpack("<16H", header)) & 0xffff:
        raise ValueError("Invalid El Torito validation entry.")
    modes = []
    def record(entry, platform):
        if len(entry) != 32 or entry[0] not in (0, 0x88):
            raise ValueError("Invalid El Torito boot entry.")
        if entry[0] != 0x88:
            return
        address = struct.unpack_from("<I", entry, 8)[0]
        count = struct.unpack_from("<H", entry, 6)[0]
        if address == 0 or count == 0 or address * sector + count * 512 > size:
            raise ValueError("Invalid boot image extent.")
        mode = {0: "bios", 0xef: "uefi"}.get(platform)
        if mode and mode not in modes:
            modes.append(mode)
    record(entries[32:64], header[1])
    offset = 64
    while offset + 32 <= sector and entries[offset] in (0x90, 0x91):
        final, platform, count = struct.unpack_from("<BBH", entries, offset)
        offset += 32
        if not count or offset + count * 32 > sector:
            raise ValueError("Unsupported or invalid El Torito boot catalog section.")
        for i in range(count):
            record(entries[offset:offset + 32], platform)
            offset += 32
        if final == 0x91:
            break
    if not modes:
        raise ValueError("No supported firmware boot entry found.")
    return {"bytes": size, "volumeLabel": volume, "bootEntries": modes}


def verify_iso(path, expected):
    with path.open("rb") as stream:
        before = os.fstat(stream.fileno())
        info = inspect_iso(stream)
        stream.seek(0)
        digest = hashlib.file_digest(stream, "sha256").hexdigest()
        after = os.fstat(stream.fileno())
    if (before.st_size, before.st_mtime_ns) != (after.st_size, after.st_mtime_ns) or digest != expected:
        raise ValueError("ISO checksum mismatch or the file changed during verification.")
    return dict(info, name=path.name, sha256=digest, checksumMatched=True)
