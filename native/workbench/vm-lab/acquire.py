"""Acquire the locked native Windows QEMU runtime from official MSYS2 packages."""

import argparse
import contextlib
import json
import shutil
import sys
import tarfile
import tempfile
import urllib.request
from pathlib import Path
from urllib.parse import urlsplit

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent / "toolbox"))
import manage


# Python 3.14+ includes a zstd reader. On Python 3.11-3.13 install zstandard.
@contextlib.contextmanager
def tar_reader(path):
    try:
        from compression import zstd

        source = zstd.open(path, "rb")
    except ImportError:
        import zstandard

        source = zstandard.ZstdDecompressor().stream_reader(path.open("rb"))
    with source, tarfile.open(fileobj=source, mode="r|") as archive:
        yield archive


def package_bytes(cache, pin, offline):
    name = pin["file"]
    if Path(name).name != name or not name.endswith(".pkg.tar.zst"):
        raise ValueError("Invalid package identity")
    target = manage.within(cache, name)
    if not target.exists():
        if offline:
            raise ValueError(f"Missing cached package {name}")
        url = "https://repo.msys2.org/mingw/ucrt64/" + name
        with tempfile.TemporaryDirectory(dir=cache) as d:
            partial = Path(d) / "package"
            with (
                urllib.request.urlopen(url, timeout=90) as response,
                partial.open("xb") as out,
            ):
                actual = urlsplit(response.geturl())
                if actual.scheme != "https" or actual.hostname != "repo.msys2.org":
                    raise ValueError("Unexpected package origin")
                count = 0
                while chunk := response.read(1024 * 1024):
                    count += len(chunk)
                    if count > pin["bytes"]:
                        raise ValueError("Package exceeds reviewed size")
                    out.write(chunk)
            if count != pin["bytes"] or manage.digest(partial) != pin["sha256"]:
                raise ValueError("Package SHA-256 failed")
            partial.rename(target)
    if target.stat().st_size != pin["bytes"] or manage.digest(target) != pin["sha256"]:
        raise ValueError("Cached package SHA-256 failed")
    return target


def acquire(root, offline):
    manage.identity(root)
    runtime = json.loads((HERE / "runtime-lock.json").read_text(encoding="utf8"))
    upstream = json.loads((HERE / "msys2-lock.json").read_text(encoding="utf8"))
    cache = manage.within(root, "93 - Offline Package Cache/QEMU Windows MSYS2")
    cache.mkdir(parents=True, exist_ok=True)
    marker = manage.within(root, ".inta-vm-runtime.json")
    rel = runtime["defaultLocation"]
    if marker.is_file():
        profile = manage.data(marker)
        if profile["schemaVersion"] != 1:
            raise ValueError("Unrecognized VM runtime marker")
        rel = profile["location"]
    target = manage.within(root, rel)
    expected = runtime["files"] + upstream["notices"]
    target.parent.mkdir(parents=True, exist_ok=True)
    with manage.locked(root):
        if target.exists():
            for item in expected:
                if manage.digest(manage.within(target, item["path"])) != item["sha256"]:
                    raise ValueError(
                        "Existing runtime content differs; preserve it for review"
                    )
        else:
            with tempfile.TemporaryDirectory(dir=target.parent) as d:
                stage = Path(d) / "verified"
                stage.mkdir()
                for pkg in upstream["packages"]:
                    members = {
                        item["member"]: item
                        for item in expected
                        if item["package"] == pkg["name"]
                    }
                    with tar_reader(package_bytes(cache, pkg, offline)) as t:
                        for member in t:
                            item = members.pop(member.name, None)
                            if not item:
                                continue
                            if not member.isfile() or member.size > 256 * 1024 * 1024:
                                raise ValueError("Unexpected package member")
                            out = manage.within(stage, item["path"])
                            out.parent.mkdir(parents=True, exist_ok=True)
                            with (
                                t.extractfile(member) as source,
                                out.open("xb") as dest,
                            ):
                                shutil.copyfileobj(source, dest)
                            if manage.digest(out) != item["sha256"]:
                                raise ValueError("Runtime file integrity check failed")
                    if members:
                        raise ValueError("Required package member missing")
                stage.rename(target)
        manage.dump(marker, {"schemaVersion": 1, "location": rel})
    return {
        "version": runtime["version"],
        "runtime": str(target),
        "files": len(runtime["files"]),
        "packages": len(upstream["packages"]),
    }


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", required=True, type=Path)
    parser.add_argument("--offline", action="store_true")
    args = parser.parse_args()
    try:
        print(json.dumps(acquire(manage.safe_root(args.root), args.offline), indent=2))
    except (OSError, ValueError, KeyError, ImportError, tarfile.TarError) as exc:
        parser.exit(1, str(exc) + "\n")
