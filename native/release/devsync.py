"""Per-user Windows development channel. Directory notifications wake an idle worker."""

import argparse
import contextlib
import ctypes
from ctypes import wintypes
import hashlib
import json
import msvcrt
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
import threading
import time

HOME = Path(os.environ["LOCALAPPDATA"]) / "INTA/desktop-development"
CONFIG = HOME / "source.json"
KERNEL = ctypes.WinDLL("kernel32", use_last_error=True)
KERNEL.CreateFileW.argtypes = [
    wintypes.LPCWSTR,
    wintypes.DWORD,
    wintypes.DWORD,
    ctypes.c_void_p,
    wintypes.DWORD,
    wintypes.DWORD,
    wintypes.HANDLE,
]
KERNEL.CreateFileW.restype = wintypes.HANDLE
KERNEL.CloseHandle.argtypes = [wintypes.HANDLE]
KERNEL.ReadDirectoryChangesW.argtypes = [
    wintypes.HANDLE,
    ctypes.c_void_p,
    wintypes.DWORD,
    wintypes.BOOL,
    wintypes.DWORD,
    ctypes.POINTER(wintypes.DWORD),
    ctypes.c_void_p,
    ctypes.c_void_p,
]
WATCH_DIRS = ("native", "desktop", "agent", "data", "docs", "site", "wiki", "media", "assets")


def atomic(path, content):
    with tempfile.NamedTemporaryFile(
        mode="w", dir=path.parent, encoding="utf8", delete=False
    ) as f:
        f.write(content)
        f.flush()
        os.fsync(f.fileno())
    os.replace(f.name, path)


@contextlib.contextmanager
def handle(path, *, watch=False):
    h = KERNEL.CreateFileW(
        str(path), 1 if watch else 0x80, 3, None, 3, 0x02000000 | 0x00200000, None
    )
    if h in (None, ctypes.c_void_p(-1).value):
        raise ctypes.WinError(ctypes.get_last_error())
    try:
        yield h
    finally:
        KERNEL.CloseHandle(h)


@contextlib.contextmanager
def lock(name):
    with (HOME / name).open("a+b") as f:
        if f.tell() == 0:
            f.write(b"1")
            f.flush()
        f.seek(0)
        msvcrt.locking(f.fileno(), msvcrt.LK_NBLCK, 1)
        try:
            yield
        finally:
            f.seek(0)
            msvcrt.locking(f.fileno(), msvcrt.LK_UNLCK, 1)


def checked_config():
    cfg = json.loads(CONFIG.read_text(encoding="utf-8-sig"))
    root = Path(cfg["source"])
    if not root.is_absolute() or root.drive.startswith("\\\\"):
        raise ValueError("A local protected checkout is required")
    st = root.stat(follow_symlinks=False)
    if (
        st.st_dev != int(cfg["volume"])
        or st.st_ino != int(cfg["fileId"])
        or st.st_file_attributes & 0x400
    ):
        raise ValueError("Source directory identity changed")
    if not re.fullmatch("[a-fA-F0-9]{40}", cfg["thumbprint"]):
        raise ValueError("Invalid signing identity")
    return cfg, root


def guard_inputs(root):
    for p in root.parents:
        if p.is_junction() or p.is_symlink():
            raise ValueError("Source has a redirected ancestor")


def sync():
    with lock("build.lock"), contextlib.ExitStack() as stack:
        cfg, root = checked_config()
        for p in [*reversed(root.parents), root]:
            if p != Path(root.anchor):
                stack.enter_context(handle(p))
        checked_config()
        guard_inputs(root)
        # Both project path and its parents are held against replacement for the whole build.
        python = Path(sys.executable).with_name("python.exe")
        build = [str(python), "-I", str(root / "native/release/build.py")]
        env = {
            **os.environ,
            "CARGO_TARGET_DIR": str(root / "target"),
            "PYTHONDONTWRITEBYTECODE": "1",
        }
        env["PATH"] = cfg["buildPath"]
        opts = dict(cwd=root, env=env, creationflags=subprocess.CREATE_NO_WINDOW)
        source = (
            subprocess.check_output(build + ["--fingerprint"], **opts).decode().strip()
        )
        if not re.fullmatch("[a-f0-9]{64}", source):
            raise ValueError("Invalid fingerprint")
        pointer = HOME / "active.txt"
        if pointer.exists() and pointer.read_text().strip() == source:
            return
        revisions = HOME / "revisions"
        revisions.mkdir(exist_ok=True)
        if shutil.disk_usage(HOME).free < 512 * 1024**2:
            raise OSError("Less than 512 MiB available on installation volume")
        old = pointer.read_text().strip() if pointer.exists() else ""
        with tempfile.TemporaryDirectory(dir=root / "target", prefix="sync-") as temp:
            payload = Path(temp) / "payload"
            with (HOME / "last-build.log").open("w", encoding="utf8") as log:
                subprocess.run(
                    build
                    + ["--output", str(payload), "--thumbprint", cfg["thumbprint"]],
                    stdout=log,
                    stderr=subprocess.STDOUT,
                    check=True,
                    **opts,
                )
            manifest = json.loads((payload / "manifest.json").read_bytes())
            if manifest["sourceDigest"] != source:
                raise ValueError("Source changed while the build was queued")
            final = revisions / source
            if not final.is_dir():
                with tempfile.TemporaryDirectory(
                    dir=revisions, prefix="incoming-"
                ) as incoming:
                    candidate = Path(incoming) / "payload"
                    shutil.copytree(payload, candidate)
                    for entry in manifest["files"]:
                        with (candidate / entry["path"]).open("rb") as f:
                            if (
                                hashlib.file_digest(f, "sha256").hexdigest()
                                != entry["sha256"]
                            ):
                                raise ValueError("Copy verification failed")
                    os.rename(candidate, final)
            # Pinned public key and every installed payload file are checked before activation.
            verify = Path(__file__).resolve().parent / "INTA Verify.exe"
            subprocess.run([str(verify), str(final), source], check=True, **opts)
            if old and old != source:
                atomic(HOME / "previous.txt", old + "\n")
            atomic(pointer, source + "\n")
        atomic(
            HOME / "status.json",
            json.dumps(
                {
                    "state": "current",
                    "sourceDigest": source,
                    "updated": int(time.time()),
                }
            ),
        )


def notifications(path, signal, subtree, exit_event, host=False, names_filter=None):
    try:
        with handle(path, watch=True) as h:
            buffer = ctypes.create_string_buffer(64 * 1024)
            count = wintypes.DWORD()
            while not exit_event.is_set():
                if not KERNEL.ReadDirectoryChangesW(
                    h,
                    buffer,
                    len(buffer),
                    subtree,
                    0x13,
                    ctypes.byref(count),
                    None,
                    None,
                ):
                    raise ctypes.WinError(ctypes.get_last_error())
                if host:
                    if not Path(__file__).is_file() or not CONFIG.exists():
                        exit_event.set()
                        signal.set()
                    continue
                if not subtree and count.value:
                    pos = 0
                    names = []
                    while pos + 12 <= count.value:
                        offset = int.from_bytes(buffer[pos : pos + 4], "little")
                        size = int.from_bytes(buffer[pos + 8 : pos + 12], "little")
                        names.append(
                            buffer[pos + 12 : pos + 12 + size].decode("utf-16-le")
                        )
                        if not offset:
                            break
                        pos += offset
                    if not any(n in names_filter for n in names):
                        continue
                signal.set()
    except OSError:
        exit_event.set()
        signal.set()


def daemon():
    with lock("watch.lock"):
        cfg, root = checked_config()
        event = threading.Event()
        stop = threading.Event()

        def watch(path, sub, host=False, names=None):
            threading.Thread(
                target=notifications,
                args=(path, event, sub, stop, host, names),
                daemon=True,
            ).start()

        watch(root, False, names=("Cargo.toml", "Cargo.lock", *WATCH_DIRS))
        for name in WATCH_DIRS:
            watch(root / name, True)
        for query in ("--absolute-git-dir", "--git-common-dir"):
            folder = Path(
                subprocess.check_output(
                    ["git", "-C", root, "rev-parse", "--path-format=absolute", query],
                    text=True,
                    creationflags=subprocess.CREATE_NO_WINDOW,
                ).strip()
            )
            if query == "--absolute-git-dir":
                watch(folder, False, names=("HEAD", "packed-refs"))
            elif (folder / "refs/heads").is_dir():
                watch(folder / "refs/heads", True)
        watch(Path(__file__).parent, False, True)
        event.set()
        while True:
            event.wait()
            if stop.is_set():
                return
            event.clear()
            while event.wait(1.5):
                event.clear()
                if stop.is_set():
                    return
            try:
                sync()
            except Exception as exc:
                atomic(
                    HOME / "status.json",
                    json.dumps(
                        {
                            "state": "last-passing-build-retained",
                            "error": str(exc),
                            "updated": int(time.time()),
                        }
                    ),
                )


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("action", choices=["watch", "sync", "rollback"])
    args = parser.parse_args()
    if args.action == "watch":
        daemon()
    elif args.action == "sync":
        sync()
    else:
        with lock("build.lock"):
            previous = (HOME / "previous.txt").read_text().strip()
            if not re.fullmatch("[a-f0-9]{64}", previous):
                raise ValueError("Invalid previous revision")
            subprocess.run(
                [
                    str(Path(__file__).parent / "INTA Verify.exe"),
                    str(HOME / "revisions" / previous),
                    previous,
                ],
                check=True,
            )
            atomic(HOME / "active.txt", previous + "\n")
