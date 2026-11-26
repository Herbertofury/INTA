#!/usr/bin/env python3
"""Synchronize generated pages to the real Wiki using the maintainer's Git authentication."""
from __future__ import annotations
import argparse
from pathlib import Path
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[1]
REMOTE = "https://github.com/Herbertofury/INTA.wiki.git"


def git(path, *args):
    return subprocess.check_output(["git", "-C", str(path), *args], text=True).strip()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--checkout", type=Path, required=True)
    parser.add_argument("--prepare-only", action="store_true", help="Copy and commit the pages without pushing")
    args = parser.parse_args()
    checkout = args.checkout.resolve()
    if checkout == ROOT or ROOT in checkout.parents:
        parser.error("Use a separate checkout outside the source repository")
    subprocess.run(["python3", str(ROOT / "scripts/build-hub.py"), "--check"], check=True)
    if not checkout.exists():
        subprocess.run(["git", "clone", REMOTE, str(checkout)], check=True)
    remote = git(checkout, "remote", "get-url", "origin")
    if remote not in {REMOTE, "git@github.com:Herbertofury/INTA.wiki.git"}:
        parser.error("Checkout must point to the exact INTA Wiki remote")
    if git(checkout, "status", "--porcelain"):
        parser.error("Wiki checkout has local changes; preserve them before syncing")
    git(checkout, "pull", "--ff-only")
    pages = sorted((ROOT / "wiki").glob("*.md"))
    for page in pages:
        shutil.copyfile(page, checkout / page.name)
    git(checkout, "add", "--", *[p.name for p in pages])
    if not git(checkout, "diff", "--cached", "--name-only"):
        print("Wiki is already synchronized.")
        return
    source = git(ROOT, "rev-parse", "HEAD")
    git(checkout, "commit", "-m", f"Sync INTA technician guide from {source[:12]}")
    if args.prepare_only:
        print(f"Prepared {len(pages)} pages at {git(checkout, 'rev-parse', 'HEAD')}; not pushed.")
        return
    git(checkout, "push", "origin", "HEAD")
    print(f"Published {len(pages)} pages to {REMOTE} at {git(checkout, 'rev-parse', 'HEAD')}")


if __name__ == "__main__":
    main()
