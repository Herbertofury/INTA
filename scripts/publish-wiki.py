#!/usr/bin/env python3
"""Publish canonical wiki/*.md to this repository's existing GitHub Wiki."""
from __future__ import annotations
import argparse
import base64
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile

class PublishError(Exception):
    pass

def sha256(data):
    return hashlib.sha256(data).hexdigest()

def publish(report):
    root = Path(__file__).resolve().parents[1]
    source = root / "wiki"
    repository = os.environ.get("GITHUB_REPOSITORY", "")
    token = os.environ.get("GITHUB_TOKEN", "")
    if not re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9-]*/[A-Za-z0-9_.-]+", repository):
        raise PublishError("GITHUB_REPOSITORY must contain owner/repository")
    if repository.split("/", 1)[1] in {".", ".."} or not token or any(c.isspace() for c in token):
        raise PublishError("A valid repository and nonempty GITHUB_TOKEN are required")
    if source.is_symlink() or not source.is_dir():
        raise PublishError("The canonical wiki directory is missing or unsafe")
    report.update(repository=repository, phase="snapshot-source")
    expected = {}
    for path in sorted(source.glob("*.md"), key=lambda p: p.name):
        if path.is_symlink() or not path.is_file():
            raise PublishError("Canonical Markdown entries must be regular files")
        expected[path.name] = path.read_bytes()
    if not expected:
        raise PublishError("No canonical Wiki Markdown files were found")
    report["files"] = [{"name": n, "bytes": len(b), "sha256": sha256(b)} for n, b in expected.items()]
    report["expected_pages"] = len(expected)
    url = f"https://github.com/{repository}.wiki.git"
    authorization = base64.b64encode(f"x-access-token:{token}".encode()).decode("ascii")
    env = {k: v for k, v in os.environ.items() if not k.startswith("GIT_") and k not in {"GITHUB_TOKEN", "GH_TOKEN"}}
    env.update(GIT_TERMINAL_PROMPT="0", GIT_CONFIG_NOSYSTEM="1", GIT_CONFIG_GLOBAL=os.devnull, GIT_LITERAL_PATHSPECS="1")
    config = [
        (f"http.{url}.extraheader", f"Authorization: Basic {authorization}"),
        ("http.followRedirects", "false"), ("credential.helper", ""),
        ("core.hooksPath", os.devnull), ("core.autocrlf", "false"),
        ("core.safecrlf", "true"), ("commit.gpgsign", "false"),
        ("protocol.allow", "never"), ("protocol.https.allow", "always"),
    ]
    env["GIT_CONFIG_COUNT"] = str(len(config))
    for i, (key, value) in enumerate(config):
        env[f"GIT_CONFIG_KEY_{i}"] = key
        env[f"GIT_CONFIG_VALUE_{i}"] = value

    def git(phase, cwd, *args):
        report["phase"] = phase
        try:
            result = subprocess.run(["git", *args], cwd=cwd, env=env, stdin=subprocess.DEVNULL,
                                    stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=240, check=False)
        except subprocess.TimeoutExpired:
            raise PublishError("Git timed out; inspect phase and push_status before retrying") from None
        except OSError:
            raise PublishError("Git could not be started") from None
        if result.returncode:
            raise PublishError(f"Git exited with status {result.returncode}; captured output withheld")
        return result.stdout

    def head(cwd, phase):
        value = git(phase, cwd, "rev-parse", "--verify", "HEAD").decode("ascii").strip()
        if not re.fullmatch(r"(?:[0-9a-f]{40}|[0-9a-f]{64})", value):
            raise PublishError("Invalid Git commit identity")
        return value

    report.update(source_head=head(root, "read-source-head"), changed_pages=[], changes=[], push_status="not-needed")
    with tempfile.TemporaryDirectory(prefix="inta-wiki-") as temporary:
        directory = Path(temporary)
        work, readback = directory / "publish", directory / "readback"
        git("clone-wiki", directory, "clone", "--quiet", "--no-tags", "--depth=1", "--single-branch", url, str(work))
        report["base_head"] = head(work, "read-wiki-head")
        branch_ref = git("read-wiki-branch", work, "symbolic-ref", "--quiet", "HEAD").decode().strip()
        if not branch_ref.startswith("refs/heads/"):
            raise PublishError("Wiki checkout has no ordinary branch")
        git("validate-wiki-branch", work, "check-ref-format", branch_ref)
        branch = branch_ref[len("refs/heads/"):]
        report["branch"] = branch
        report["preserved_live_pages"] = sorted(p.name for p in work.glob("*.md") if p.name not in expected)
        report["phase"] = "copy-canonical-pages"
        for name, content in expected.items():
            destination = work / name
            if destination.is_symlink() or (destination.exists() and not destination.is_file()):
                raise PublishError("Canonical destination is not a regular file")
            previous = destination.read_bytes() if destination.exists() else None
            if previous != content:
                destination.write_bytes(content)
                report["changed_pages"].append(name)
                report["changes"].append({"name": name, "before_sha256": sha256(previous) if previous is not None else None,
                                          "after_sha256": sha256(content)})
        if report["changed_pages"]:
            git("stage-canonical-pages", work, "add", "--", *report["changed_pages"])
        for name, content in expected.items():
            staged = git("verify-staged-bytes", work, "cat-file", "blob", f":0:{name}")
            if staged != content:
                report["staged_mismatch"] = {"name": name, "expected_sha256": sha256(content), "actual_sha256": sha256(staged)}
                raise PublishError("Staged Markdown differs from canonical bytes")
        if report["changed_pages"]:
            git("commit-wiki", work, "-c", "user.name=github-actions[bot]",
                "-c", "user.email=41898282+github-actions[bot]@users.noreply.github.com",
                "commit", "--quiet", "-m", f"Sync canonical Wiki from {report['source_head'][:12]}")
            report["published_head"] = head(work, "read-publish-head")
            report["push_status"] = "attempted"
            git("push-wiki", work, "push", "--porcelain", "origin", f"HEAD:{branch_ref}")
            report["push_status"] = "confirmed"
        else:
            report["published_head"] = report["base_head"]
        git("clone-readback", directory, "clone", "--quiet", "--no-tags", "--depth=1", "--single-branch",
            f"--branch={branch}", url, str(readback))
        report["verified_head"] = head(readback, "read-verified-head")
        report["phase"] = "verify-live-bytes"
        mismatches = []
        for name, content in expected.items():
            path = readback / name
            actual = path.read_bytes() if path.is_file() and not path.is_symlink() else None
            if actual != content:
                mismatches.append({"name": name, "expected_sha256": sha256(content),
                                   "actual_sha256": sha256(actual) if actual is not None else None})
        if mismatches:
            report["mismatches"] = mismatches
            raise PublishError("Fresh Wiki clone differs from canonical bytes")
        report.update(verified_pages=len(expected), status="success", phase="complete")

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--report", type=Path)
    args = parser.parse_args()
    report = {"schemaVersion": 1, "status": "failed", "phase": "initialize"}
    try:
        publish(report)
    except PublishError as error:
        report["error"] = str(error)
    except Exception as error:
        report["error"] = f"Unexpected {type(error).__name__}; details withheld"
    if args.report:
        try:
            args.report.parent.mkdir(parents=True, exist_ok=True)
            args.report.write_text(json.dumps(report, indent=2) + "\n")
        except OSError:
            report.update(status="failed", error="Could not write JSON report")
    print(json.dumps(report, indent=2))
    return 0 if report["status"] == "success" else 1

if __name__ == "__main__":
    sys.exit(main())
