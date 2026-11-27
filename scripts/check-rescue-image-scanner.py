"""Read only the Debian package metadata embedded in an exact INTA Rescue ISO.

This check never mounts or boots an image, scans files, or runs bundled code.
Use it only on the trusted candidate bytes obtained from the build workflow.
"""
import argparse
import hashlib
from pathlib import Path
import resource
import subprocess
import sys
import tempfile

LIMIT_ISO = 16 * 1024**3
LIMIT_STATUS = 32 * 1024**2
LIMIT_RELEASE = 64 * 1024


def limited_file_size(max_size):
    return lambda: resource.setrlimit(resource.RLIMIT_FSIZE, (max_size, max_size))


def safe_process(argv, max_seconds, stdout, limit):
    # Output is sent only to controlled staging files; keep tools noninteractive.
    return subprocess.run(argv, stdin=subprocess.DEVNULL, stdout=stdout,
                          stderr=subprocess.DEVNULL, check=True, timeout=max_seconds,
                          preexec_fn=limited_file_size(limit))


def iso_stat(path):
    before = path.lstat()
    if path.is_symlink() or not path.is_file() or not 0 < before.st_size <= LIMIT_ISO:
        raise ValueError("A regular bounded ISO file is required")
    return (before.st_dev, before.st_ino, before.st_size, before.st_mtime_ns, before.st_ctime_ns)


def qualify(iso, expected, review=None):
    before = iso_stat(iso)
    if len(expected) != 64 or any(c not in "0123456789abcdef" for c in expected):
        raise ValueError("Invalid expected SHA-256")
    with iso.open("rb") as stream:
        if hashlib.file_digest(stream, "sha256").hexdigest() != expected:
            raise ValueError("ISO digest mismatch")
    if iso_stat(iso) != before:
        raise ValueError("ISO changed while hashing")

    with tempfile.TemporaryDirectory(prefix="inta-exact-iso-") as folder:
        temp = Path(folder)
        squash = temp / "filesystem.squashfs"
        safe_process(["xorriso", "-osirrox", "on", "-abort_on", "FAILURE", "-indev", str(iso),
                      "-extract", "/live/filesystem.squashfs", str(squash)],
                     180, subprocess.DEVNULL, min(before[2], LIMIT_ISO))
        if squash.is_symlink() or not squash.is_file() or not 0 < squash.stat().st_size <= before[2]:
            raise ValueError("Expected one regular live squashfs image")
        for name, bound in (("usr/lib/os-release", LIMIT_RELEASE), ("var/lib/dpkg/status", LIMIT_STATUS)):
            out = temp / "metadata" / name
            out.parent.mkdir(parents=True, exist_ok=True)
            with out.open("xb") as stream:
                safe_process(["unsquashfs", "-processors", "1", "-cat", str(squash), name], 40, stream, bound)
        if iso_stat(iso) != before:
            raise ValueError("ISO changed during extraction")

        checker = Path(__file__).with_name("check-rescue-scanner.py")
        # The policy checker hashes the ISO again before emitting a qualification.
        check = [sys.executable, str(checker), "--image-root", str(temp / "metadata"),
                 "--iso", str(iso), "--expected-sha256", expected]
        if review is not None:
            check += ["--review", str(review)]
        return subprocess.run(check, timeout=180, check=False).returncode


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--iso", type=Path, required=True)
    p.add_argument("--expected-sha256", required=True)
    p.add_argument("--review", type=Path)
    a = p.parse_args()
    try:
        return qualify(a.iso.absolute(), a.expected_sha256, a.review)
    except (OSError, ValueError, subprocess.SubprocessError):
        p.exit(2, "Scanner qualification refused: image identity or bounded extraction failed.\n")


if __name__ == "__main__":
    sys.exit(main())
