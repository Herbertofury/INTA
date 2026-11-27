"""Qualification check for scanner packages from a candidate ISO's extracted root.

The caller is responsible for deriving the root directly from the exact ISO and
independently reviewing fixed source/binary tuples in Debian's stable suite.
This check does not obtain updates, validate signatures, or run the scanner.
"""
import argparse
import datetime as dt
import hashlib
import json
from pathlib import Path
import re
import stat
import subprocess
import sys

SUITE = "trixie"
ARCH = "amd64"
TRACKER = "https://security-tracker.debian.org/tracker/source-package/clamav"
OLD = "1.4.6+dfsg-1~deb13u1"
VERSION = re.compile(r"[0-9][A-Za-z0-9.+:~\-]{0,99}")
HEX = re.compile(r"[0-9a-f]{64}")
FORMAT = r"${Package}\t${Architecture}\t${db:Status-Status}\t${db:Status-Eflag}\t${source:Package}\t${source:Version}\t${Version}\n"

def regular(path, bound):
    if not stat.S_ISREG(path.lstat().st_mode) or not 0 < path.stat().st_size <= bound:
        raise ValueError("Ordinary nonempty input of bounded size required")
    return path

def packages(root):
    directory = root / "var/lib/dpkg"
    regular(directory / "status", 32 * 1024 * 1024)
    # A suite label by itself does not establish the origin of the installed packages.
    # Match the official reviewed exact source AND binary tuples below.
    os_release = regular(root / "usr/lib/os-release", 65536).read_text(encoding="utf-8")
    if not re.search(r'^VERSION_CODENAME=(?:"trixie"|trixie)$', os_release, re.M):
        raise ValueError("Expected a Debian Trixie image root")
    data = subprocess.run(["dpkg-query", f"--admindir={directory}", "-W", f"-f={FORMAT}",
                           "clamav", "libclamav[0-9]*"],
                          check=True, capture_output=True, text=True, timeout=10).stdout
    if len(data) > 4096:
        raise ValueError("Unexpected scanner package inventory size")
    rows = []
    for line in data.splitlines():
        package, arch, state, error, source, source_version, version = line.split("\t")
        if (not re.fullmatch(r"clamav|libclamav[0-9]+", package) or arch != ARCH
                or state != "installed" or error != "ok" or source != "clamav"
                or not VERSION.fullmatch(source_version) or not VERSION.fullmatch(version)):
            raise ValueError("Unexpected installed scanner package identity")
        rows.append(dict(package=package, architecture=arch, source=source,
                         source_version=source_version, binary_version=version))
    if len(rows) != 2 or sorted(r["package"].startswith("libclamav") for r in rows) != [False, True]:
        raise ValueError("Exactly one installed scanner and one installed engine library required")
    return sorted(rows, key=lambda r: r["package"])

def qualify(rows, review, today):
    if any(OLD in (row["source_version"], row["binary_version"]) for row in rows):
        return "known-vulnerable"
    if not review:
        return "unverified"
    if not isinstance(review, dict) or set(review) != {"suite", "architecture", "source", "reviewed_on", "evidence", "packages"}:
        raise ValueError("Invalid scanner review record")
    date = dt.date.fromisoformat(review["reviewed_on"])
    evidence = review["evidence"]
    if (review["suite"] != SUITE or review["architecture"] != ARCH or review["source"] != "clamav"
            or date.isoformat() != review["reviewed_on"] or not 0 <= (today-date).days <= 7
            or not isinstance(evidence, list) or not 2 <= len(evidence) <= 10 or TRACKER not in evidence
            or not any(isinstance(url,str) and re.fullmatch(r"https://tracker\.debian\.org/news/[0-9]+/[a-z0-9-]+/?", url) for url in evidence)):
        return "unverified"
    return "reviewed-exact-match" if review["packages"] == rows else "unverified"

def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--image-root", type=Path, required=True, help="root independently extracted from this exact ISO")
    p.add_argument("--iso", type=Path, required=True)
    p.add_argument("--expected-sha256", required=True)
    p.add_argument("--review", type=Path, help="maintainer-reviewed exact Debian stable evidence, not a downloaded advisory")
    a = p.parse_args()
    try:
        iso = regular(a.iso, 16 * 1024**3)
        with iso.open("rb") as f:
            digest = hashlib.file_digest(f, "sha256").hexdigest()
        if not HEX.fullmatch(a.expected_sha256) or digest != a.expected_sha256:
            raise ValueError("ISO digest mismatch")
        rows = packages(a.image_root)
        review = json.loads(regular(a.review, 65536).read_text(encoding="utf-8")) if a.review else None
        state = qualify(rows, review, dt.datetime.now(dt.timezone.utc).date())
        print(json.dumps(dict(iso_sha256=digest, suite=SUITE, architecture=ARCH, packages=rows,
                              qualification=state, evidence=review["evidence"] if state == "reviewed-exact-match" else [TRACKER],
                              reviewed_on=review["reviewed_on"] if state == "reviewed-exact-match" else None)))
        return 0 if state == "reviewed-exact-match" else 2
    except (OSError, subprocess.SubprocessError, ValueError, TypeError) as e:
        p.exit(2, f"Scanner qualification refused: {e}\n")

if __name__ == "__main__":
    sys.exit(main())
