#!/usr/bin/env python3
"""Check every generated Wiki page for links, navigation, and diagram coverage."""
from __future__ import annotations

import argparse
import importlib.util
import json
from pathlib import Path
import re
import unicodedata
from urllib.parse import unquote, urlparse

ROOT = Path(__file__).resolve().parents[1]
WIKI = "https://github.com/Herbertofury/INTA/wiki"


def load_module(filename):
    spec = importlib.util.spec_from_file_location(filename.replace("-", "_"), ROOT / "scripts" / filename)
    result = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(result)
    return result


links = load_module("wiki_links.py")


def without_code(text):
    lines, marker, length = [], None, 0
    for line in text.splitlines(keepends=True):
        fence = re.match(r"^[ >\t]*(`{3,}|~{3,})(.*)$", line.rstrip("\r\n"))
        if marker:
            if fence and fence[1][0] == marker and len(fence[1]) >= length and not fence[2].strip():
                marker = None
            lines.append("\n")
        elif fence:
            marker, length = fence[1][0], len(fence[1])
            lines.append("\n")
        elif line.startswith(("    ", "\t")):
            lines.append("\n")
        else:
            lines.append(line)
    return re.sub(r"(`+).*?\1", "", "".join(lines), flags=re.S)


def destinations(text):
    """Extract inline destinations without misreading balanced URL parentheses."""
    result, index = [], 0
    while index < len(text):
        if text[index] == "[":
            end = links._balanced(text, index, "[", "]")
            if end and end < len(text) and text[end] == "(":
                finish = links._balanced(text, end, "(", ")")
                if finish:
                    result.append(text[end + 1:finish - 1].split(' "', 1)[0].strip().strip("<>"))
                    index = finish
                    continue
        index += 1
    result.extend(re.findall(r"^ {0,3}\[[^\]]+\]:\s*<?([^\s>]+)", text, flags=re.M))
    return result


def anchors(text):
    result = set(re.findall(r'<a\s+(?:id|name)=["\']([^"\']+)["\']', text))
    seen = {}
    for title in re.findall(r"^#{1,6}\s+(.+)$", without_code(text), flags=re.M):
        title = re.sub(r"\[([^\]]+)\]\([^)]*\)", r"\1", title).lower()
        title = re.sub(r"<[^>]+>", "", title)
        slug = "".join(c for c in title if unicodedata.category(c)[0] in "LNM" or c in "_ -")
        slug = slug.replace(" ", "-")
        count = seen.get(slug, 0)
        seen[slug] = count + 1
        result.add(slug if not count else f"{slug}-{count}")
    return result


def inspect(pages, registry, jobs):
    errors, count_links, count_diagrams = [], 0, 0
    page_names = {Path(path).stem: text for path, text in pages.items()}
    page_anchors = {name: anchors(text) for name, text in page_names.items()}
    for name, text in page_names.items():
        if links.link_markdown(text, registry) != text:
            errors.append(f"{name}: unlinked registered names: {', '.join(links.plain_mentions(text, registry)[:12])}")
        stripped = without_code(text)
        for url in destinations(stripped):
            count_links += 1
            parsed = urlparse(url)
            if parsed.scheme and parsed.scheme != "https":
                errors.append(f"{name}: non-HTTPS destination {url}")
                continue
            if any(x in parsed.query.lower() for x in ("utm_", "fbclid=", "gclid=")):
                errors.append(f"{name}: tracking destination {url}")
            target = None
            if url.startswith(WIKI):
                tail = parsed.path[len(urlparse(WIKI).path):].lstrip("/")
                target = unquote(tail) or "Home"
            elif not parsed.scheme and not parsed.netloc:
                if parsed.path and "/" not in parsed.path:
                    target = unquote(parsed.path).removesuffix(".md")
                elif not parsed.path and parsed.fragment:
                    target = name
            if target is not None:
                if target not in page_names:
                    errors.append(f"{name}: missing Wiki page {target}")
                elif parsed.fragment and unquote(parsed.fragment) not in page_anchors[target]:
                    errors.append(f"{name}: missing anchor {target}#{parsed.fragment}")
        diagrams = re.findall(r"^```mermaid\s*\n(.*?)^```\s*$", text, flags=re.M | re.S)
        count_diagrams += len(diagrams)
        for diagram in diagrams:
            if re.search(r"^\s*click\s|%%\{|<\/?(?:br|a)\b", diagram, flags=re.M | re.I):
                errors.append(f"{name}: unsupported interactive/configuration/HTML diagram syntax")
        for diagram in re.finditer(r"^```mermaid\s*\n.*?^```\s*$", text, flags=re.M | re.S):
            following = text[diagram.end():].lstrip().split("\n##", 1)[0]
            if not destinations(following[:1600]):
                errors.append(f"{name}: decision diagram needs an immediately following linked legend")
    for job in jobs:
        title = "Job-" + job["id"]
        text = page_names.get(title, "")
        if "```mermaid" not in text:
            errors.append(f"{title}: no decision diagram")
        if not re.search(r"\{[^\n]+\}", text):
            errors.append(f"{title}: decision diagram has no branch")
        if not re.search(r"^- \[ \]", text, flags=re.M):
            errors.append(f"{title}: no actionable checklist")
    return {"wikiFiles": len(pages), "wikiPages": len([x for x in page_names if not x.startswith("_")]),
            "registeredNames": len(registry), "inlineLinks": count_links,
            "mermaidDiagrams": count_diagrams, "playbooks": len(jobs), "errors": errors}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--report", type=Path)
    args = parser.parse_args()
    build = load_module("build-hub.py")
    tools, jobs = build.records(ROOT / "data/tools.json", "tools"), build.records(ROOT / "data/jobs.json", "jobs")
    registry = links.build_registry(tools, jobs)
    pages = {str(path.relative_to(ROOT)): path.read_text(encoding="utf-8") for path in sorted((ROOT / "wiki").glob("*.md"))}
    result = inspect(pages, registry, jobs)
    if args.report:
        args.report.parent.mkdir(parents=True, exist_ok=True)
        args.report.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(result, indent=2))
    raise SystemExit(bool(result["errors"]))


if __name__ == "__main__":
    main()
