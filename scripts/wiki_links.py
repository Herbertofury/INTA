"""Canonical, repeatable links for the INTA Wiki and offline hub.

Only prose is rewritten. Existing links, URLs, code, and Mermaid source remain
byte-for-byte intact. The same registry is embedded in the browser build.
"""
from __future__ import annotations

from functools import lru_cache
import json
from pathlib import Path
import re
from urllib.parse import urlparse

ROOT = Path(__file__).resolve().parents[1]
WIKI = "https://github.com/Herbertofury/INTA/wiki"
AMBIGUOUS = {"Everything", "Autopsy", "Chainsaw"}
BARE_URL = re.compile(r"https?://[^\s<>]+")


def _records(path: Path, key: str) -> list[dict]:
    if not path.exists():
        return []
    content = json.loads(path.read_text(encoding="utf-8"))
    return content.get(key, []) if isinstance(content, dict) else content


def build_registry(tools: list[dict], jobs: list[dict]) -> list[dict]:
    """Resolve each spelling to one deliberate destination, never a search page."""
    entries: dict[str, dict] = {}

    def add(name, url, kind, identity, case_sensitive=False):
        name = name.strip()
        if not name:
            return
        parsed = urlparse(url)
        if parsed.scheme != "https" or not parsed.hostname or parsed.username:
            raise ValueError(f"Unsafe registry URL for {name}: {url}")
        if any(x in parsed.query.lower() for x in ("utm_", "fbclid=", "gclid=")):
            raise ValueError(f"Tracking registry URL for {name}")
        key = name.casefold()
        previous = entries.get(key)
        if previous and previous["url"] != url:
            raise ValueError(f"Ambiguous link alias: {name}")
        item = {"name": name, "url": url, "kind": kind, "id": identity}
        if case_sensitive:
            item["caseSensitive"] = True
        entries[key] = previous or item

    for tool in tools:
        add(tool["name"], tool["homepage"], "tool", tool["id"], tool["name"] in AMBIGUOUS)
        for alias in tool.get("aliases", []):
            add(alias, tool["homepage"], "tool", tool["id"])
    for job in jobs:
        add(job["title"], WIKI + "/Job-" + job["id"], "playbook", job["id"])
    for method in _records(ROOT / "data/methods.json", "methods"):
        for name in [method["name"], *method.get("aliases", [])]:
            add(name, method["url"], "method", method["id"], method.get("caseSensitive", False))
    by_id = {tool["id"]: tool for tool in tools}
    for item in _records(ROOT / "data/link-aliases.json", "links"):
        if "toolId" in item:
            tool = by_id[item["toolId"]]
            url, kind, identity = tool["homepage"], "tool", tool["id"]
        else:
            url, kind, identity = item["url"], item.get("kind", "application"), item["id"]
        for name in [item["name"], *item.get("aliases", [])]:
            add(name, url, kind, identity, item.get("caseSensitive", False))
    # Publisher/site identities share the same repeated-mention linker as tools.
    # Existing explicit aliases and precise method destinations keep precedence.
    brand_path = ROOT / "data/brand-icons.json"
    if brand_path.is_file():
        brands = json.loads(brand_path.read_text(encoding="utf-8")).get("brands", [])
        ordinary = {"home", "support", "download", "downloads", "documentation", "source", "help", "free", "tools", "media"}
        for brand in brands:
            for name in [brand.get("name", ""), *brand.get("names", [])]:
                if not name or name.casefold() in entries or name.casefold() in ordinary or len(name) < 2:
                    continue
                add(name, brand["downloadPage"], brand.get("kind", "website"), brand["id"], True)
    return sorted(entries.values(), key=lambda x: (-len(x["name"]), x["name"].casefold()))


def portal_links(registry: list[dict]) -> list[dict]:
    """Public, non-secret link metadata shared with the offline browser app."""
    return [dict(item) for item in registry]


@lru_cache(maxsize=8)
def _matcher(keys: tuple[str, ...]):
    if not keys:
        return re.compile(r"(?!x)x")
    return re.compile(r"(?<![\w])(?:" + "|".join(re.escape(x) for x in keys) + r")(?![\w])", re.IGNORECASE)


def _balanced(text: str, start: int, opening: str, closing: str) -> int | None:
    depth, index = 0, start
    while index < len(text):
        char = text[index]
        if char == "\\":
            index += 2
            continue
        if char == opening:
            depth += 1
        elif char == closing:
            depth -= 1
            if depth == 0:
                return index + 1
        index += 1
    return None


def _protected_end(text: str, index: int, reference_ids: set[str]) -> int | None:
    """Find a Markdown/HTML token to preserve; this is not an HTML renderer."""
    if text.startswith("<!--", index):
        finish = text.find("-->", index + 4)
        return len(text) if finish < 0 else finish + 3
    if text[index] == "`":
        run = re.match(r"`+", text[index:]).group()
        end = text.find(run, index + len(run))
        if end >= 0:
            return end + len(run)
    if text[index] == "<":
        anchor = re.match(r"<a\b[^>]*>.*?</a\s*>", text[index:], flags=re.I | re.S)
        if anchor:
            return index + anchor.end()
        end = text.find(">", index + 1)
        if end >= 0:
            return end + 1
    bare_url = BARE_URL.match(text, index) if text[index] == "h" else None
    if bare_url:
        return bare_url.end()
    bracket = index + 1 if text.startswith("![", index) else index
    if bracket < len(text) and text[bracket] == "[":
        end = _balanced(text, bracket, "[", "]")
        if end is not None:
            if end < len(text) and text[end] == "(":
                return _balanced(text, end, "(", ")")
            if end < len(text) and text[end] == "[":
                return _balanced(text, end, "[", "]")
            label = text[bracket + 1:end - 1].strip().casefold()
            if label in reference_ids or text.startswith("[[", bracket):
                return end
    return None


def _inline(text: str, registry: list[dict], reference_ids: set[str]) -> str:
    by_name = {x["name"].casefold(): x for x in registry}
    matcher = _matcher(tuple(x["name"] for x in registry))

    def replace(prose):
        def link(match):
            item = by_name[match.group().casefold()]
            if item.get("caseSensitive") and match.group() != item["name"]:
                return match.group()
            return f"[{match.group()}]({item['url']})"
        return matcher.sub(link, prose)

    result, start, index = [], 0, 0
    while index < len(text):
        end = _protected_end(text, index, reference_ids)
        if end is not None:
            result.extend((replace(text[start:index]), text[index:end]))
            index = start = end
        else:
            index += 1
    result.append(replace(text[start:]))
    return "".join(result)


def link_markdown(text: str, registry: list[dict]) -> str:
    """Link every matching prose occurrence; applying it twice is a no-op."""
    reference_ids = set(x.casefold() for x in re.findall(r"^ {0,3}\[([^\]]+)\]:", text, flags=re.M))
    result, prose = [], []
    fence_char, fence_length = None, 0

    def flush():
        if prose:
            result.append(_inline("".join(prose), registry, reference_ids))
            prose.clear()

    for line in text.splitlines(keepends=True):
        marker = re.match(r"^[ >\t]*(`{3,}|~{3,})(.*)$", line.rstrip("\r\n"))
        if fence_char:
            result.append(line)
            if marker and marker[1][0] == fence_char and len(marker[1]) >= fence_length and not marker[2].strip():
                fence_char, fence_length = None, 0
        elif marker:
            flush()
            fence_char, fence_length = marker[1][0], len(marker[1])
            result.append(line)
        elif re.match(r"^(?: {4}|\t)", line) or re.match(r"^ {0,3}\[[^\]]+\]:", line):
            flush()
            result.append(line)
        else:
            prose.append(line)
    flush()
    return "".join(result)


def plain_mentions(text: str, registry: list[dict]) -> list[str]:
    """Return names still missing their link in rendered prose."""
    transformed = link_markdown(text, registry)
    if transformed == text:
        return []
    # The caller can show a diff; names alone keep CI diagnostics compact.
    before = {x["name"]: text.count("](" + x["url"] + ")") for x in registry}
    return sorted({x["name"] for x in registry if transformed.count("](" + x["url"] + ")") > before[x["name"]]})
