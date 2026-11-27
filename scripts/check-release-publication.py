#!/usr/bin/env python3
"""Check publication freshness, tag identity, and complete release readback."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import urllib.request

def require(value, message):
    if not value:
        raise RuntimeError(message)

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("phase", choices=("freshness", "prepare", "before", "after"))
    args = parser.parse_args()
    repository = os.environ["GITHUB_REPOSITORY"]
    source = os.environ["SOURCE_COMMIT"]
    tag = os.environ["RELEASE_TAG"]
    require(re.fullmatch(r"[0-9a-f]{40}", source), "A complete source commit is required")
    def api(path, missing=False):
        result = subprocess.run(["gh", "api", "repos/" + repository + "/" + path],
                                stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        if result.returncode:
            if missing and "(HTTP 404)" in result.stderr:
                return None
            raise RuntimeError("GitHub API read failed for " + path)
        return json.loads(result.stdout)
    def output(key, value):
        with open(os.environ["GITHUB_OUTPUT"], "a") as stream:
            stream.write(key + "=" + str(value).lower() + "\n")
    current = api("git/ref/heads/main")["object"]["sha"]
    if args.phase == "freshness":
        output("current", current == source)
        print(json.dumps({"sourceCommit": source, "currentMain": current, "current": current == source}))
        return
    if args.phase in ("prepare", "before"):
        require(current == source, "A newer main revision exists; this publication was stopped")
    release = api("releases/tags/" + tag, missing=True)
    reference = api("git/ref/tags/" + tag, missing=True)
    target = reference["object"] if reference else None
    for _ in range(8):
        if not target or target["type"] == "commit":
            break
        require(target["type"] == "tag", "Release tag points to an unsupported Git object")
        target = api("git/tags/" + target["sha"])["object"]
    require(not target or target["type"] == "commit", "Annotated tag chain is too deep")
    tagged_commit = target["sha"] if target else None
    if args.phase == "prepare" and release and not release["draft"]:
        require(tagged_commit is not None, "Published release has no Git tag")
        output("pending", False)
        print(json.dumps({"pending": False, "releaseUrl": release["html_url"],
                          "publishedCommit": tagged_commit, "reason": "Dated release is already published"}))
        return
    if args.phase in ("prepare", "before"):
        require(tagged_commit in (None, source), "Existing tag points to a different source commit")
        if release:
            require(release["draft"], "Release became public before this publication step")
            require(release["target_commitish"] == source, "Existing draft belongs to a different source commit")
        if args.phase == "prepare":
            output("pending", True)
        print(json.dumps({"pending": True, "sourceCommit": source, "tag": tag, "existingTag": tagged_commit}))
        return
    require(release and not release["draft"] and tagged_commit == source,
            "Published release/tag identity does not match the verified source")
    stem = "INTA-online-toolbox-2026-11-26"
    names = [stem + suffix for suffix in ("-source.zip", "-offline.html", "-manifest.json", "-SHA256SUMS.txt")]
    assets = {item["name"]: item for item in release["assets"]}
    report = {"status": "success", "sourceCommit": source, "tag": tag, "releaseUrl": release["html_url"], "assets": []}
    for name in names:
        require(name in assets, "A complete release asset is missing: " + name)
        local = Path("dist/release") / name
        expected = hashlib.sha256(local.read_bytes()).hexdigest()
        url = assets[name]["browser_download_url"]
        require(url.startswith("https://github.com/" + repository + "/releases/download/" + tag + "/"),
                "Release download URL is outside the expected repository")
        digest, size = hashlib.sha256(), 0
        with urllib.request.urlopen(url, timeout=60) as response:
            while chunk := response.read(1024 * 1024):
                digest.update(chunk)
                size += len(chunk)
        require(digest.hexdigest() == expected and size == local.stat().st_size,
                "Downloaded release bytes differ: " + name)
        report["assets"].append({"name": name, "bytes": size, "sha256": expected, "url": url})
    Path("reports").mkdir(exist_ok=True)
    Path("reports/release-publish.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))

if __name__ == "__main__":
    main()
