"""Resumable, verified toolbox copy. The source and active selection are preserved."""

import argparse
import json
import os
import shutil
import stat
import tempfile
from functools import lru_cache
from pathlib import Path

import manage


@lru_cache(maxsize=1)
def stream_api():
    import ctypes
    from ctypes import wintypes

    kernel = ctypes.WinDLL("kernel32", use_last_error=True)

    class Stream(ctypes.Structure):
        _fields_ = [("size", ctypes.c_longlong), ("name", wintypes.WCHAR * 296)]

    first = kernel.FindFirstStreamW
    first.argtypes = [
        wintypes.LPCWSTR,
        ctypes.c_int,
        ctypes.POINTER(Stream),
        wintypes.DWORD,
    ]
    first.restype = wintypes.HANDLE
    following = kernel.FindNextStreamW
    following.argtypes = [wintypes.HANDLE, ctypes.POINTER(Stream)]
    following.restype = wintypes.BOOL
    close = kernel.FindClose
    close.argtypes = [wintypes.HANDLE]
    return ctypes, Stream, first, following, close


def reject_extra_streams(path):
    if os.name != "nt":
        return
    ctypes, Stream, first, following, close = stream_api()
    info = Stream()
    handle = first(str(path), 0, ctypes.byref(info), 0)
    if handle == ctypes.c_void_p(-1).value:
        code = ctypes.get_last_error()
        if code in (
            1,
            38,
            50,
        ):  # Filesystem lacks streams, or no streams on this entry.
            return
        raise ctypes.WinError(code)
    try:
        while True:
            if info.name != "::$DATA":
                raise manage.Failure(
                    "A named stream requires a filesystem-aware migration; source preserved"
                )
            if not following(handle, ctypes.byref(info)):
                code = ctypes.get_last_error()
                if code != 38:
                    raise ctypes.WinError(code)
                return
    finally:
        close(handle)


def inventory(root, skip_lock=False):
    files = {}
    directories = set()

    def fail(exc):
        raise exc

    for dirname, dirs, names in os.walk(root, onerror=fail):
        parent = Path(dirname)
        for name in dirs + names:
            path = parent / name
            manage.check_link(path)
            rel = path.relative_to(root).as_posix()
            if skip_lock and rel == ".inta-maintenance.lock":
                continue
            kind = path.lstat()
            reject_extra_streams(path)
            if stat.S_ISDIR(kind.st_mode):
                directories.add(rel)
            elif stat.S_ISREG(kind.st_mode):
                if kind.st_nlink > 1:
                    raise manage.Failure(
                        "Hardlinked file requires a filesystem-aware migration; source preserved"
                    )
                files[rel] = (kind.st_size, manage.digest(path))
            else:
                raise manage.Failure("Unexpected device or special file in toolbox")
    return directories, files


def copy(source, dest):
    src = manage.safe_root(source)
    dst = manage.safe_root(dest)
    if dst == src or src in dst.parents or dst in src.parents or dst.exists():
        raise manage.Failure("Choose a new destination outside the current toolbox")
    if not dst.parent.is_dir():
        raise manage.Failure("Select an existing destination parent directory")
    marker = manage.identity(src)
    key = marker["toolboxId"]
    staging = manage.within(dst.parent, f".{dst.name}.{key}.inta-copy")
    state = manage.within(dst.parent, staging.name + ".json")
    with manage.locked(src):
        dirs, files = inventory(src, skip_lock=True)
        if staging.exists():
            if not state.is_file() or manage.data(state) != {
                "schemaVersion": 1,
                "toolboxId": key,
                "destination": dst.name,
            }:
                raise manage.Failure("Staging destination has no matching copy receipt")
        else:
            if state.exists():
                raise manage.Failure("An unmatched copy receipt is present")
            staging.mkdir()
            manage.dump(
                state, {"schemaVersion": 1, "toolboxId": key, "destination": dst.name}
            )
        staged_dirs, staged_files = inventory(staging)
        if staged_dirs - dirs or staged_files.keys() - files.keys():
            raise manage.Failure(
                "Unexpected files in staging; both trees have been preserved"
            )
        pending = [rel for rel in files if staged_files.get(rel) != files[rel]]
        if shutil.disk_usage(dst.parent).free < sum(files[f][0] for f in pending):
            raise manage.Failure("Insufficient free space for a complete verified copy")
        for rel in sorted(dirs):
            manage.within(staging, rel).mkdir(parents=True, exist_ok=True)
        for rel in pending:
            source_file = manage.within(src, rel)
            target = manage.within(staging, rel)
            with tempfile.NamedTemporaryFile(
                dir=dst.parent, prefix=".inta-copy-", delete=False
            ) as f:
                tmp = Path(f.name)
            try:
                shutil.copy2(source_file, tmp)
                if manage.digest(tmp) != files[rel][1]:
                    raise manage.Failure(
                        "Source changed while copying; active toolbox preserved"
                    )
                tmp.replace(target)
            finally:
                tmp.unlink(missing_ok=True)
        # Check both trees again, including deleted, added, or concurrently modified files.
        if inventory(src, skip_lock=True) != (dirs, files) or inventory(staging) != (
            dirs,
            files,
        ):
            raise manage.Failure("Source or destination changed during validation")
        idx = manage.index(staging)
        for pin in manage.PINS:
            rel = idx["packages"].get(pin["id"])
            if rel and manage.digest(manage.within(staging, rel)) != pin["exeSha256"]:
                raise manage.Failure(
                    "A copied active package differs from its pinned SHA-256"
                )
        if dst.exists():
            raise manage.Failure("Destination was created concurrently")
        staging.rename(dst)
        state.unlink()
    return {
        "toolboxId": key,
        "files": len(files),
        "bytes": sum(size for size, _ in files.values()),
        "state": "verified",
        "destination": str(dst),
        "selection": "explicit reconnect required",
    }


if __name__ == "__main__":
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("source", type=Path)
    p.add_argument("destination", type=Path)
    a = p.parse_args()
    try:
        print(json.dumps(copy(a.source, a.destination), indent=2))
    except (ValueError, OSError) as exc:
        p.exit(1, str(exc) + "\n")
