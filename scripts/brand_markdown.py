"""Add locally preserved publisher icons without changing contextual links.

The icon has its own official acquisition-page link. Existing documentation,
installer, evidence and in-project destinations retain their exact meaning.
Code, diagrams, image labels and raw URLs are never rewritten.
"""
from __future__ import annotations

import html
import re
from urllib.parse import urlparse

RAW_ASSETS = "https://raw.githubusercontent.com/Herbertofury/INTA/main/"
GENERATED_ICON = re.compile(r'<a\b[^>]*\bdata-inta-brand="[^"]+"[^>]*>\s*<img\b[^>]*>\s*</a> ?', re.I)


def strip_icons(text: str) -> str:
    """Return canonical prose, making regeneration safe and idempotent."""
    return GENERATED_ICON.sub("", text)


class BrandMarkdown:
    def __init__(self, registry: dict, linker):
        self.registry, self.linker = registry, linker
        self.assets = registry.get("assets", {})
        self.by_name, self.by_url, self.by_host = {}, {}, {}
        self.brands = registry.get("brands", [])
        for brand in self.brands:
            for name in [brand.get("name", ""), *brand.get("names", [])]:
                if name:
                    self.by_name.setdefault(name.casefold(), brand)
            for url in brand.get("urls", []):
                self.by_url.setdefault(url.rstrip("/"), brand)
            for host in brand.get("hosts", []):
                host = host.lower().removeprefix("www.")
                previous = self.by_host.get(host)
                if not previous or brand.get("kind") == "website":
                    self.by_host[host] = brand
        names = sorted(self.by_name, key=lambda name: (-len(name), name))
        self.matcher = re.compile(r"(?<![\w])(?:" + "|".join(map(re.escape, names)) + r")(?![\w])", re.I) if names else None
        self.cache = {}

    def resolve(self, label: str, url: str):
        key = (label, url)
        if key in self.cache:
            return self.cache[key]
        clean = html.unescape(re.sub(r"<[^>]*>|[*_`]", "", label)).strip()
        ambiguous_lower = clean.casefold() in {"everything", "windows", "autopsy", "chainsaw", "terminal", "edge", "access", "claude"} and clean == clean.lower()
        brand = None if ambiguous_lower else self.by_name.get(clean.casefold())
        if not brand and self.matcher:
            match = self.matcher.search(clean)
            if match:
                candidate = self.by_name[match.group().casefold()]
                # Lower-case ordinary words are not product names.
                if match.group().casefold() not in {"everything", "windows", "autopsy", "chainsaw", "terminal", "edge", "access", "claude"} or match.group()[0].isupper():
                    brand = candidate
        internal = url.startswith(("https://github.com/Herbertofury/INTA/", "https://herbertofury.github.io/INTA/")) or not url.startswith("https://")
        if not brand and internal:
            self.cache[key] = None
            return None
        if not brand:
            brand = self.by_url.get(url.rstrip("/"))
        if not brand and url.startswith("https://"):
            # Longest publisher path wins before a website-level fallback.
            routes = [(route, item) for route, item in self.by_url.items()
                      if len(url) > len(route) and url.startswith(route) and url[len(route)] in "/?#"]
            if routes:
                brand = max(routes, key=lambda item: len(item[0]))[1]
            else:
                brand = self.by_host.get((urlparse(url).hostname or "").removeprefix("www."))
        self.cache[key] = brand
        return brand

    def icon(self, brand) -> str:
        if not brand or brand.get("inlineIcon") is False or brand.get("assetId") not in self.assets:
            return ""
        asset = self.assets[brand["assetId"]]
        source = RAW_ASSETS + asset["path"]
        page = brand["downloadPage"]
        if urlparse(page).scheme != "https":
            raise ValueError("Brand destination must be public HTTPS: " + brand["id"])
        title = "Open " + brand["name"] + " official download or service page"
        attr = lambda value: html.escape(str(value), quote=True)
        return (f'<a href="{attr(page)}" title="{attr(title)}" aria-label="{attr(title)}" '
                f'data-inta-brand="{attr(brand["id"])}"><img src="{attr(source)}" alt="{attr(brand["name"] + " official page")}" '
                'width="18" height="18" valign="middle" /></a> ')

    def resolve_all(self, label: str, url: str) -> list[dict]:
        """A combined installer/source label may name several real products."""
        clean = html.unescape(re.sub(r"<[^>]*>|[*_`]", "", label)).strip()
        found = {}
        if self.matcher:
            for match in self.matcher.finditer(clean):
                name = match.group()
                if name.casefold() in {"everything", "windows", "autopsy", "chainsaw", "terminal", "edge", "access", "claude"} and not name[0].isupper():
                    continue
                brand = self.by_name[name.casefold()]
                found.setdefault(brand["id"], brand)
        if found:
            return list(found.values())
        brand = self.resolve(label, url)
        return [brand] if brand else []

    def inline(self, text: str, references: dict) -> str:
        result, index = [], 0
        reference_ids = set(references)
        while index < len(text):
            end = self.linker._protected_end(text, index, reference_ids)
            if end is None:
                result.append(text[index])
                index += 1
                continue
            token = text[index:end]
            label, url = "", ""
            if token.startswith("[") and not token.startswith("[["):
                close = self.linker._balanced(token, 0, "[", "]")
                if close:
                    label = token[1:close - 1]
                    tail = token[close:]
                    if tail.startswith("("):
                        url = tail[1:-1].split(' "', 1)[0].strip().strip("<>")
                    elif tail.startswith("["):
                        ref = tail[1:-1].strip() or label
                        url = references.get(ref.casefold(), "")
                    else:
                        url = references.get(label.strip().casefold(), "")
            elif token.lower().startswith("<a ") and "data-inta-brand" not in token:
                match = re.search(r'\bhref=["\']([^"\']+)["\']', token, re.I)
                if match:
                    url, label = html.unescape(match[1]), re.sub(r"<[^>]*>", "", token)
            if label and url and not re.search(r"<img\b|!\[", token, re.I):
                result.extend(self.icon(brand) for brand in self.resolve_all(label, url))
            result.append(token)
            index = end
        return "".join(result)

    def decorate(self, text: str) -> str:
        text = strip_icons(text)
        references = {match[1].strip().casefold(): match[2].strip("<>") for match in
                      re.finditer(r"^ {0,3}\[([^\]]+)\]:\s*(\S+)", text, re.M)}
        result, prose = [], []
        fence_char, fence_length = None, 0

        def flush():
            if prose:
                result.append(self.inline("".join(prose), references))
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
