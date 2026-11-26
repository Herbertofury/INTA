#!/usr/bin/env python3
"""Produce an honest, on-demand freshness report; never install or auto-promote tools."""
from __future__ import annotations
import argparse
from concurrent.futures import ThreadPoolExecutor
from datetime import datetime, timezone
import json
from pathlib import Path
import re
from urllib.error import HTTPError, URLError
from urllib.parse import urlparse
from urllib.request import Request, urlopen

ROOT = Path(__file__).resolve().parents[1]


def github_repo(tool):
    # Use only explicit publisher/repository URLs, never infer an unrelated owner from a docs URL.
    for key in ("sourceUrl", "homepage", "downloadUrl"):
        parsed = urlparse(tool[key])
        parts = parsed.path.strip("/").split("/")
        if parsed.hostname == "github.com" and len(parts) >= 2:
            owner, repo = parts[:2]
            if re.fullmatch(r"[\w.-]+", owner) and re.fullmatch(r"[\w.-]+", repo):
                return owner + "/" + repo.removesuffix(".git")
    return None


def inspect(tool, timeout=15, opener=urlopen):
    result = {"id": tool["id"], "name": tool["name"], "editorialReview": tool["reviewedAt"],
              "downloadUrl": tool["downloadUrl"]}
    repo = github_repo(tool)
    url = f"https://api.github.com/repos/{repo}/releases/latest" if repo else tool["downloadUrl"]
    request = Request(url, headers={"User-Agent": "INTA-Technician-Hub/1.0", "Accept": "application/vnd.github+json" if repo else "text/html,*/*;q=0.5"}, method="GET" if repo else "HEAD")
    try:
        try:
            response = opener(request, timeout=timeout)
        except HTTPError as error:
            if not repo and error.code in {405, 501}:
                response = opener(Request(url, headers={"User-Agent": "INTA-Technician-Hub/1.0"}), timeout=timeout)
            else:
                raise
        with response:
            result["httpStatus"] = response.status
            result["resolvedUrl"] = response.geturl()
            if repo:
                release = json.load(response)
                if release.get("draft") or release.get("prerelease"):
                    result.update(status="review-required", detail="The returned release is not a stable public release.")
                else:
                    result.update(status="release-observed", version=release.get("tag_name"),
                                  publishedAt=release.get("published_at"), releaseUrl=release.get("html_url"),
                                  detail="Upstream metadata only. Review, test and verify the download before adopting.")
            else:
                result.update(status="manual-version-review", detail="Official link responded. A version has not been inferred from reachability.")
    except HTTPError as error:
        if repo and error.code == 404:
            result.update(status="manual-version-review", httpStatus=404, detail="No latest stable GitHub release is visible. Check the official download source manually.")
        else:
            result.update(status="unresolved", httpStatus=error.code, detail=str(error))
    except (URLError, TimeoutError, OSError, ValueError) as error:
        result.update(status="unresolved", detail=str(error))
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--catalog", type=Path, default=ROOT / "data/tools.json")
    parser.add_argument("--output", type=Path, default=ROOT / "reports/upstreams.json")
    parser.add_argument("--workers", type=int, default=6)
    parser.add_argument("--timeout", type=int, default=15)
    args = parser.parse_args()
    if not 1 <= args.workers <= 16 or not 1 <= args.timeout <= 60:
        parser.error("Use 1-16 workers and a 1-60 second request timeout")
    tools = json.loads(args.catalog.read_text(encoding="utf-8"))
    if isinstance(tools, dict):
        tools = tools["tools"]
    with ThreadPoolExecutor(max_workers=args.workers) as pool:
        results = list(pool.map(lambda t: inspect(t, args.timeout), tools))
    unresolved = sum(x["status"] == "unresolved" for x in results)
    report = {"checkedAt": datetime.now(timezone.utc).isoformat(), "expected": len(tools), "reported": len(results),
              "unresolved": unresolved, "installed": False, "results": results}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(f"Reported {len(results)}/{len(tools)} tools; {unresolved} unresolved. No software installed or catalog entries changed.")
    print(args.output)
    if unresolved:
        raise SystemExit(2)


if __name__ == "__main__":
    main()
