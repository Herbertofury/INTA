#!/usr/bin/env python3
"""Package the exact validated source commit, offline app, and Wiki/runtime evidence."""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import zipfile

def require(condition, message):
    if not condition:
        raise RuntimeError(message)

def sha(data):
    return hashlib.sha256(data).hexdigest()

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=Path("dist/release"))
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[1]
    def git(*parts):
        return subprocess.check_output(["git", "-C", str(root), *parts])
    commit = git("rev-parse", "HEAD").decode().strip()
    tree = git("rev-parse", "HEAD^{tree}").decode().strip()
    require(commit == os.environ.get("SOURCE_COMMIT", commit), "Checkout differs from the validated commit")
    require(not git("diff", "--name-only", commit, "--", ".", ":(exclude)reports/**", ":(exclude)site/validation/**").strip(),
            "Tracked source changed after validation")
    runtime = json.loads((root / "reports/online-runtime/online-runtime.json").read_text())
    wiki = json.loads((root / "reports/wiki-publish.json").read_text())
    build = json.loads((root / "site/build-manifest.json").read_text())
    html = (root / "site/index.html").read_bytes()
    require(runtime.get("status") == "passed" and runtime.get("sha256") == sha(html),
            "Browser evidence does not cover this exact HTML")
    require(not runtime.get("javascriptErrors") and not runtime.get("externalRequests"),
            "Runtime evidence reports an error or unexpected request")
    expected = {p.name: p.read_bytes() for p in sorted((root / "wiki").glob("*.md"))}
    require(wiki.get("status") == "success" and wiki.get("source_head") == commit,
            "Wiki readback does not cover this source commit")
    require(wiki.get("expected_pages") == len(expected) == wiki.get("verified_pages"),
            "Wiki readback is incomplete")
    reported = {row["name"]: row for row in wiki["files"]}
    require(set(reported) == set(expected), "Wiki report file inventory differs")
    require(all(reported[name]["sha256"] == sha(data) for name, data in expected.items()),
            "Wiki report hashes differ from source")
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    stem = "INTA-online-toolbox-2026-11-26"
    source_zip = output / (stem + "-source.zip")
    offline = output / (stem + "-offline.html")
    manifest_path = output / (stem + "-manifest.json")
    checksum_path = output / (stem + "-SHA256SUMS.txt")
    require(not any(p.exists() for p in (source_zip, offline, manifest_path, checksum_path)), "Release files already exist")
    archive_root = "INTA-" + commit[:12] + "/"
    git("archive", "--format=zip", "-9", "--prefix=" + archive_root, "--output=" + str(source_zip), commit)
    tracked = {}
    for row in git("ls-tree", "-rz", "--full-tree", commit).split(b"\0"):
        if row:
            metadata, path = row.split(b"\t", 1)
            mode, kind, identity = metadata.decode().split()
            require(kind == "blob", "Unresolved submodule cannot enter a complete source release")
            tracked[path.decode()] = {"mode": mode, "gitBlob": identity}
    inventory = []
    with zipfile.ZipFile(source_zip, "a", compression=zipfile.ZIP_DEFLATED, compresslevel=9) as archive:
        actual = {}
        for item in list(archive.infolist()):
            if item.is_dir():
                continue
            require(item.filename.startswith(archive_root), "Unexpected source archive prefix")
            name = item.filename[len(archive_root):]
            require(name in tracked and name not in actual, "Unexpected or duplicate source member")
            data = archive.read(item)
            require(hashlib.sha1(b"blob " + str(len(data)).encode() + b"\0" + data).hexdigest() == tracked[name]["gitBlob"],
                    "Archived source differs from Git blob: " + name)
            actual[name] = sha(data)
            inventory.append({"path": name, "bytes": len(data), "sha256": actual[name], **tracked[name]})
        require(set(actual) == set(tracked), "Archive omitted committed source")
        require(all(actual.get(name) == digest for name, digest in build["files"].items()),
                "Generated build manifest differs from committed source")
        evidence_paths = [root / "reports/wiki-publish.json", *sorted((root / "reports/online-runtime").glob("*"))]
        evidence = []
        for path in evidence_paths:
            require(path.is_file() and not path.is_symlink(), "Evidence must be a regular file")
            name = path.relative_to(root).as_posix()
            data = path.read_bytes()
            archive.writestr("verification/" + name, data)
            evidence.append({"path": name, "bytes": len(data), "sha256": sha(data)})
        manifest = {
            "schemaVersion": 1, "sourceCommit": commit, "sourceTree": tree,
            "sourceUrl": "https://github.com/Herbertofury/INTA/tree/" + commit,
            "sourceFiles": len(tracked), "wikiFiles": len(expected), "htmlSha256": sha(html),
            "browser": runtime.get("browser"), "runtimeStatus": runtime["status"],
            "wikiVerifiedHead": wiki["verified_head"], "wikiReadbackStatus": wiki["status"],
            "catalogCounts": {k: v for k, v in build.items() if isinstance(v, int)},
            "files": inventory, "evidence": evidence,
            "validationRun": os.environ.get("VALIDATION_RUN_URL", ""),
            "publicationRun": os.environ.get("GITHUB_SERVER_URL", "https://github.com") + "/" + os.environ.get("GITHUB_REPOSITORY", "Herbertofury/INTA") + "/actions/runs/" + os.environ.get("GITHUB_RUN_ID", ""),
        }
        archive.writestr("verification/package-manifest.json", json.dumps(manifest, indent=2) + "\n")
    with zipfile.ZipFile(source_zip) as archive:
        require(archive.testzip() is None, "ZIP integrity check failed")
        require(len(archive.namelist()) == len(set(archive.namelist())), "ZIP contains duplicate entries")
        require(archive.read(archive_root + "site/index.html") == html, "Archived offline app differs")
    offline.write_bytes(html)
    manifest["artifacts"] = {p.name: {"bytes": p.stat().st_size, "sha256": sha(p.read_bytes())} for p in (source_zip, offline)}
    manifest_path.write_text(json.dumps(manifest, indent=2) + "\n")
    checksum_path.write_text("".join(sha(p.read_bytes()) + "  " + p.name + "\n" for p in (source_zip, offline, manifest_path)))
    notes = (
        "# INTA online toolbox — source review 2026-11-26\n\n"
        "Complete app, source and Wiki checkpoint: " + commit + ".\n\n"
        + str(build["onlineTools"]) + " online services across " + str(build["onlineGroups"]) + " groups and "
        + str(build["onlineWorkflows"]) + " guided workflows, within the full " + str(build["tools"]) + "-tool catalog.\n\n"
        "Includes all source, assets, generated app and Wiki, tests, runtime screenshots and full Wiki readback evidence. "
        "The standalone HTML is the exact validated offline app. Existing Command Center, maintenance and Rescue work is retained.\n\n"
        "Research records state official launch routes, account/access boundaries, submission behavior and primary sources. "
        "The review date does not imply that every service launched in 2026. No customer samples or indicators were submitted.\n"
    )
    (output / "release-notes.md").write_text(notes)
    print(json.dumps({"sourceCommit": commit, "sourceTree": tree, "sourceFiles": len(tracked),
                      "wikiFiles": len(expected), "htmlSha256": sha(html), "artifacts": manifest["artifacts"]}, indent=2))

if __name__ == "__main__":
    main()
