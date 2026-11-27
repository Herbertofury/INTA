#!/usr/bin/env python3
"""Merge current main into the online toolbox branch without dropping content."""
import json
import os
import subprocess
from pathlib import Path

def git(*args, check=True):
    return subprocess.run(["git", *args], check=check, text=True, capture_output=True)

MISSING = object()
def merge(base, left, right, label):
    if left == right:
        return left
    if left == base:
        return right
    if right == base:
        return left
    if isinstance(left, dict) and isinstance(right, dict):
        base = base if isinstance(base, dict) else {}
        out = {}
        for key in dict.fromkeys([*base, *left, *right]):
            value = merge(base.get(key, MISSING), left.get(key, MISSING), right.get(key, MISSING), label + "." + key)
            if value is not MISSING:
                out[key] = value
        return out
    if isinstance(left, list) and isinstance(right, list):
        base = base if isinstance(base, list) else []
        rows = base + left + right
        key = next((candidate for candidate in ("id", "toolId")
                    if rows and all(isinstance(row, dict) and candidate in row for row in rows)
                    and all(len({row[candidate] for row in seq}) == len(seq) for seq in (base, left, right))), None)
        if key:
            maps = [{row[key]: row for row in seq} for seq in (base, left, right)]
            ids = list(dict.fromkeys([row[key] for row in left + right]))
            if any(identity not in ids for identity in maps[0]):
                raise ValueError(label + ": original record deletion requires review")
            return [merge(maps[0].get(identity, MISSING), maps[1].get(identity, MISSING), maps[2].get(identity, MISSING), label + "[" + str(identity) + "]") for identity in ids]
        if all(isinstance(row, (str, int, float, bool)) for row in rows):
            if not all(row in left and row in right for row in base):
                raise ValueError(label + ": non-additive list edit requires review")
            return list(dict.fromkeys(left + right))
        if left[:len(base)] == base and right[:len(base)] == base:
            out = list(left)
            for row in right[len(base):]:
                if row not in out:
                    out.append(row)
            return out
    raise ValueError(label + ": both branches changed the same value")

def stage_json(path):
    stages = [json.loads(git("show", ":" + str(stage) + ":" + path).stdout) for stage in (1, 2, 3)]
    result = merge(*stages, path)
    Path(path).write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n")
    git("add", "--", path)

if os.environ.get("GITHUB_REF") != "refs/heads/online-toolbox-2026-11-26":
    raise SystemExit("Recovery is restricted to the owned toolbox branch")
git("fetch", "origin", "main")
main_sha = git("rev-parse", "origin/main").stdout.strip()
result = git("merge", "--no-commit", "--no-ff", "origin/main", check=False)
print(result.stdout)
print(result.stderr)
conflicts = git("diff", "--name-only", "--diff-filter=U").stdout.splitlines()
unresolved = []
for path in conflicts:
    try:
        if path.startswith(("site/", "wiki/", "docs/runbooks/")):
            git("checkout", "--ours", "--", path)
            git("add", "--", path)
        elif path.startswith("data/") and path.endswith(".json"):
            stage_json(path)
        else:
            unresolved.append(path)
    except (ValueError, json.JSONDecodeError, subprocess.CalledProcessError) as error:
        print("REVIEW REQUIRED:", path, str(error))
        unresolved.append(path)
if unresolved:
    report = Path("reports/recovery")
    report.mkdir(parents=True, exist_ok=True)
    (report / "unresolved.json").write_text(json.dumps({"main": main_sha, "files": unresolved}, indent=2))
    for path in unresolved:
        print("CONFLICT", path)
        print(git("diff", "--cc", "--", path).stdout)
    raise SystemExit("Unresolved authored conflicts were preserved; no branch was pushed")

app = Path("portal/app.js")
text = app.read_text()
old = "if(updateCategory){$('.updates-category-tabs [data-update-category]').forEach"
new = "if(updateCategory){$$('.updates-category-tabs [data-update-category]').forEach"
if old in text:
    assert text.count(old) == 1
    text = text.replace(old, new)
elif new not in text:
    raise SystemExit("Updates handler changed; inspect before applying the verified fix")
app.write_text(text)
catalog_path = Path("data/updates-center.json")
catalog = json.loads(catalog_path.read_text())
for row in catalog["commands"]:
    if row["id"] == "windows-store-cache" and ("2daed4e5" in row["sourceUrl"] or row["sourceUrl"].endswith("/launch-store-app")):
        row["sourceUrl"] = "https://support.microsoft.com/en-us/accounts-billing/i-can-t-find-or-install-an-app-from-microsoft-store"
        row["sourceTitle"] = "Microsoft Store reset and installation troubleshooting"
    if row["id"] == "store-open-updates" and "2daed4e5" in row["sourceUrl"]:
        row["sourceUrl"] = "https://learn.microsoft.com/en-us/windows/apps/develop/launch/launch-store-app"
        row["sourceTitle"] = "Microsoft Store URI reference"
catalog_path.write_text(json.dumps(catalog, ensure_ascii=False, indent=2) + "\n")
tools = json.loads(Path("data/tools.json").read_text())
online = json.loads(Path("data/online-tools.json").read_text())
if len(tools) < 258 or len(online["tools"]) < 125 or len(online["workflows"]) < 13:
    raise SystemExit("Complete online toolbox coverage was not retained")
print(json.dumps({"mergedMain": main_sha, "tools": len(tools), "onlineTools": len(online["tools"]), "onlineWorkflows": len(online["workflows"])}))
