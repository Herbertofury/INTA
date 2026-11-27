#!/usr/bin/env python3
"""Build a small, public, read-only INTA news digest from fixed sources."""
from __future__ import annotations
import argparse
import concurrent.futures
import datetime as dt
from email.utils import parsedate_to_datetime
from hashlib import sha256
from html import unescape
from html.parser import HTMLParser
import json
import os
from pathlib import Path
import re
import sys
import tempfile
import time
from typing import NamedTuple
from urllib.error import URLError
from urllib.parse import parse_qs, parse_qsl, urlencode, urljoin, urlsplit, urlunsplit
from urllib.request import HTTPRedirectHandler, ProxyHandler, Request, build_opener
import xml.etree.ElementTree as ET

FEED_BYTES = 1024 * 1024
PAGE_BYTES = 384 * 1024
OUTPUT_BYTES = 512 * 1024
SOCKET_TIMEOUT = 7
SOURCE_SECONDS = 16
MAX_ITEMS = 16
MAX_SCANNED = 96
MAX_MEDIA = 3
PAGE_LIMIT = 2
WORKERS = 4

class Source(NamedTuple):
    id: str
    feed: str
    hosts: frozenset[str]
    article_prefix: tuple[str, ...] = ()
    media_hosts: frozenset[str] = frozenset()
    byte_limit: int = FEED_BYTES

SOURCES = (
    Source("bleeping", "https://www.bleepingcomputer.com/feed/", frozenset({"www.bleepingcomputer.com"}),
           ("/news/",), frozenset({"www.bleepstatic.com"})),
    Source("krebs", "https://krebsonsecurity.com/feed/", frozenset({"krebsonsecurity.com"}),
           (), frozenset()),
    Source("securityweek", "https://www.securityweek.com/feed/", frozenset({"www.securityweek.com"}),
           (), frozenset()),
    Source("github-status", "https://www.githubstatus.com/history.rss", frozenset({"www.githubstatus.com"})),
    Source("cert-fr", "https://cert.cyber.gouv.fr/feed/", frozenset({"cert.cyber.gouv.fr", "www.cert.cyber.gouv.fr"})),
    # Its first-party public RSS includes historic episodes. Only this fixed
    # feed gets the wider still-bounded budget; no MP3 body is fetched here.
    Source("sans-isc", "https://isc.sans.edu/dailypodcast.xml", frozenset({"isc.sans.edu"}),
           (), frozenset({"traffic.libsyn.com"}), 7 * 1024 * 1024),
)
SEED_SOURCES = {
    "windows-health": Source("windows-health", "", frozenset({
        "learn.microsoft.com", "techcommunity.microsoft.com", "blogs.windows.com",
    })),
}
MEDIA_TYPES = {
    "image": frozenset({"image/jpeg", "image/png", "image/webp", "image/gif", "image/avif"}),
    "audio": frozenset({"audio/mpeg", "audio/mp4", "audio/ogg", "audio/wav", "audio/x-wav", "audio/aac"}),
    "video": frozenset({"video/mp4", "video/webm", "video/ogg"}),
}
MEDIA_EXT = {
    "image": (".jpg", ".jpeg", ".png", ".webp", ".gif", ".avif"),
    "audio": (".mp3", ".m4a", ".wav", ".ogg", ".oga", ".aac"),
    "video": (".mp4", ".webm", ".ogv"),
}
ATOM = "{http://www.w3.org/2005/Atom}"
MEDIA = "{http://search.yahoo.com/mrss/}"
TRACKERS = {"fbclid", "gclid", "mc_cid", "mc_eid", "ref", "ref_src"}
EMBED_HOSTS = frozenset({"www.youtube.com", "youtube.com", "youtu.be", "www.youtube-nocookie.com"})

class Unsafe(ValueError):
    pass

def safe_url(raw, base, hosts, *, canonical=False):
    if (not isinstance(raw, str) or not raw or len(raw) > 2048
            or any(ord(c) <= 32 or ord(c) == 127 or c == "\\" for c in raw)):
        return None
    try:
        split = urlsplit(urljoin(base, raw))
        if (split.scheme != "https" or split.hostname not in hosts
                or split.netloc.lower() != split.hostname or split.username or split.password):
            return None
        path = split.path or "/"
        if re.search(r"%(?:00|0a|0d|2e|2f|5c)", path, re.I) or set(path.split("/")) & {".", ".."}:
            return None
        query = split.query
        if canonical:
            query = urlencode([(k, v) for k, v in parse_qsl(query, keep_blank_values=True, max_num_fields=32)
                               if not k.lower().startswith("utm_") and k.lower() not in TRACKERS])
        return urlunsplit((split.scheme, split.hostname, path, query, ""))
    except (ValueError, UnicodeError):
        return None

class GuardRedirect(HTTPRedirectHandler):
    max_redirections = 3
    max_repeats = 1

    def __init__(self, hosts):
        self.hosts = hosts

    def redirect_request(self, req, fp, code, msg, headers, newurl):
        if not safe_url(newurl, req.full_url, self.hosts):
            raise Unsafe("redirect outside the approved HTTPS hosts")
        return super().redirect_request(req, fp, code, msg, headers, newurl)

def fetch(url, hosts, limit, deadline, *, page=False):
    """Only fixed source-owned HTTPS hosts, no session cookies, tight size/time budgets."""
    if safe_url(url, url, hosts) != url:
        raise Unsafe("unapproved request URL")
    request = Request(url, headers={
        "User-Agent": "INTA-News/1.0 (public metadata reader)",
        "Accept": "text/html" if page else "application/atom+xml, application/rss+xml, application/xml, text/xml",
        "Accept-Encoding": "identity",
    })
    # Ambient system proxy configuration is retained for normal managed networks.
    # No cookie/auth handlers are installed and no credential header is submitted.
    opener = build_opener(ProxyHandler(), GuardRedirect(hosts))
    remaining = deadline - time.monotonic()
    if remaining <= 0:
        raise Unsafe("source time budget exhausted")
    with opener.open(request, timeout=min(SOCKET_TIMEOUT, remaining)) as response:
        end_url = response.geturl()
        if not safe_url(end_url, url, hosts) or response.status != 200:
            raise Unsafe("unexpected final response")
        mime = response.headers.get_content_type()
        allowed = ({"text/html", "application/xhtml+xml"} if page
                   else {"application/atom+xml", "application/rss+xml", "application/xml", "text/xml", "application/rdf+xml"})
        if mime not in allowed or response.headers.get("Content-Encoding", "identity").lower() != "identity":
            raise Unsafe("unexpected response type or encoding")
        length = response.headers.get("Content-Length")
        if length is not None and (not length.isdecimal() or int(length) > limit):
            raise Unsafe("declared response too large")
        data = bytearray()
        while len(data) <= limit:
            if time.monotonic() >= deadline:
                raise Unsafe("source time budget exhausted")
            chunk = response.read1(min(16 * 1024, limit + 1 - len(data)))
            if not chunk:
                break
            data.extend(chunk)
        if len(data) > limit:
            raise Unsafe("response too large")
        return bytes(data), end_url

class HeadMetadata(HTMLParser):
    keys = {
        "og:image": "image", "og:image:url": "image", "og:image:secure_url": "image", "twitter:image": "image",
        "og:video": "video", "og:video:url": "video", "og:video:secure_url": "video", "twitter:player": "embed",
        "og:audio": "audio", "og:audio:url": "audio", "og:audio:secure_url": "audio",
    }

    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.values = []
        self.closed_head = False

    def handle_starttag(self, tag, attrs):
        if tag == "body":
            self.closed_head = True
        if self.closed_head or tag != "meta" or len(self.values) >= 24:
            return
        values = dict(attrs)
        key = (values.get("property") or values.get("name") or "").lower()
        if key in self.keys:
            self.values.append((self.keys[key], values.get("content"), ""))

    def handle_endtag(self, tag):
        if tag == "head":
            self.closed_head = True

class PlainText(HTMLParser):
    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.parts = []
        self.ignored = 0

    def handle_starttag(self, tag, attrs):
        self.ignored += tag in {"script", "style"}

    def handle_endtag(self, tag):
        if tag in {"script", "style"} and self.ignored:
            self.ignored -= 1

    def handle_data(self, data):
        if not self.ignored:
            self.parts.append(data)

def plain_title(raw):
    if not raw:
        return ""
    parser = PlainText()
    parser.feed(unescape(raw[:4096]))
    value = " ".join(" ".join(parser.parts).split())
    value = "".join(c for c in value if c >= " " and c != chr(127))
    return value[:220].strip()

def item_date(raw):
    if not raw or len(raw) > 100:
        return None
    try:
        value = dt.datetime.fromisoformat(raw.strip().replace("Z", "+00:00"))
    except ValueError:
        try:
            value = parsedate_to_datetime(raw.strip())
        except (ValueError, TypeError, OverflowError):
            return None
    if value.tzinfo:
        value = value.astimezone(dt.timezone.utc)
    if value.year < 1990:
        return None
    return value.date().isoformat()

def embed_url(url):
    split = urlsplit(url)
    if split.hostname in {"www.youtube.com", "youtube.com", "youtu.be", "www.youtube-nocookie.com"}:
        try:
            ident = (parse_qs(split.query, max_num_fields=32).get("v", [""])[0]
                     if split.path == "/watch" else split.path.rsplit("/", 1)[-1])
        except ValueError:
            return None
        if (split.path.startswith(("/embed/", "/shorts/"))
                or split.path == "/watch" or split.hostname == "youtu.be") and re.fullmatch(r"[A-Za-z0-9_-]{11}", ident):
            return f"https://www.youtube.com/watch?v={ident}"
    return None

def pick_media(source, rows, base):
    final = []
    seen = set()
    for kind, href, mime in rows:
        if not isinstance(kind, str) or not isinstance(mime, str):
            continue
        # Normal asset URLs must stay on the publisher's explicit media origin list.
        # Provider embeds are link-out only and never fetched/embedded by this tool.
        url = safe_url(href, base, EMBED_HOSTS)
        outbound = embed_url(url) if url and kind in {"video", "audio", "embed", ""} else None
        if outbound:
            kind, url = "embed", outbound
        else:
            url = safe_url(href, base, source.hosts | source.media_hosts)
            if not url:
                continue
            mime = (mime or "").split(";", 1)[0].strip().lower()
            path = urlsplit(url).path.lower()
            if urlsplit(url).hostname == "traffic.libsyn.com" and not re.fullmatch(r"/securitypodcast/\d+\.mp3", path):
                continue
            # Extension required for type-unknown metadata. Explicit dangerous
            # extensions such as SVG/HTML are refused even with misleading MIME.
            if path.endswith((".svg", ".svgz", ".html", ".htm", ".php", ".js", ".xml")):
                continue
            if kind not in MEDIA_TYPES:
                kind = next((key for key in MEDIA_TYPES if mime in MEDIA_TYPES[key] or path.endswith(MEDIA_EXT[key])), "")
            if not kind or not (mime in MEDIA_TYPES[kind] or (not mime and path.endswith(MEDIA_EXT[kind]))):
                continue
        if url not in seen:
            final.append({"kind": kind, "url": url})
            seen.add(url)
        if len(final) == MAX_MEDIA:
            break
    return final

def child_text(node, tags):
    for tag in tags:
        child = node.find(tag)
        if child is not None:
            value = "".join(child.itertext()).strip()
            if value:
                return value
    return ""

def item_links(node, atom):
    if not atom:
        return child_text(node, ("link",))
    for child in node.findall(ATOM + "link"):
        if child.get("rel", "alternate") == "alternate" and child.get("type", "text/html") in {"text/html", "application/xhtml+xml"}:
            return child.get("href", "")
    return ""

def feed_media(node, atom):
    result = []
    for child in node.iter():
        if child.tag == MEDIA + "thumbnail":
            result.append(("image", child.get("url"), ""))
        elif child.tag == MEDIA + "content":
            result.append((child.get("medium", ""), child.get("url"), child.get("type", "")))
        elif child.tag == MEDIA + "player":
            result.append(("embed", child.get("url"), ""))
        elif child.tag == "enclosure":
            result.append(("", child.get("url"), child.get("type", "")))
        elif atom and child.tag == ATOM + "link" and child.get("rel") == "enclosure":
            result.append(("", child.get("href"), child.get("type", "")))
        if len(result) >= 24:
            break
    return result

def promotional(source, node, url, title):
    if source.id != "bleeping":
        return False
    path = urlsplit(url).path.lower()
    categories = {(c.text or "").strip().lower() for c in (node.findall("category") if node is not None else ())}
    return (path.startswith(("/news/deals/", "/deals/"))
            or bool(categories & {"deals", "sponsored", "sponsored content"})
            or (bool(re.search(r"\bblack friday\b", title, re.I))
                and bool(re.search(r"(?:\b(?:deals?|discount|saving|sale|shop)\b|\$\d)", title, re.I))))

def parse_feed(source, data, base):
    if len(data) > source.byte_limit:
        raise Unsafe("oversized feed")
    xml = data.decode("utf-8-sig", errors="strict")
    if re.search(r"<!\s*(?:DOCTYPE|ENTITY)", xml, re.I):
        raise Unsafe("DTD and entity declarations are not permitted")
    tree = ET.fromstring(xml)
    if tree.tag == ATOM + "feed":
        atom, nodes = True, tree.iterfind(ATOM + "entry")
    elif tree.tag == "rss":
        atom, nodes = False, tree.iterfind("channel/item")
    else:
        raise Unsafe("unrecognized feed root")
    seen, items = set(), []
    for count, node in enumerate(nodes, 1):
        if count > MAX_SCANNED or len(items) >= MAX_ITEMS:
            break
        url = safe_url(item_links(node, atom), base, source.hosts, canonical=True)
        title = plain_title(child_text(node, (ATOM + "title",) if atom else ("title",)))
        raw_date = child_text(node, (ATOM + "published", ATOM + "updated") if atom else (
            "pubDate", "{http://purl.org/dc/elements/1.1/}date", "date"))
        date = item_date(raw_date)
        if not url or not title or not date or url in seen or promotional(source, node, url, title):
            continue
        seen.add(url)
        items.append({"id": sha256((source.id + "|" + url).encode()).hexdigest()[:24],
                      "sourceId": source.id, "title": title, "url": url, "date": date,
                      "media": pick_media(source, feed_media(node, atom), url)})
    return items

REJECT = (Unsafe, URLError, OSError, ET.ParseError, UnicodeError, ValueError)

def refresh_source(source, *, page_media=False, fetcher=fetch):
    deadline = time.monotonic() + SOURCE_SECONDS
    try:
        data, end_url = fetcher(source.feed, source.hosts, source.byte_limit, deadline)
        items = parse_feed(source, data, end_url)
    except REJECT:
        return source.id, "unavailable", []
    # Only first-party article URL prefixes for specifically approved publishers.
    # No HTML discovery, scraping of page body, or auto-fetching media files.
    if page_media and source.article_prefix:
        eligible = [x for x in items if len(x["media"]) < MAX_MEDIA
                    and urlsplit(x["url"]).path.startswith(source.article_prefix)]
        for item in eligible[:PAGE_LIMIT]:
            if time.monotonic() > deadline - 0.25:
                break
            try:
                html, end = fetcher(item["url"], source.hosts, PAGE_BYTES, deadline, page=True)
                if not urlsplit(end).path.startswith(source.article_prefix):
                    continue
                parser = HeadMetadata()
                parser.feed(html.decode("utf-8", errors="replace"))
                found = pick_media(source, parser.values, end)
                seen = {m["url"] for m in item["media"]}
                item["media"] = (item["media"] + [m for m in found if m["url"] not in seen])[:MAX_MEDIA]
            except REJECT:
                continue  # Feed article remains usable if head metadata is unavailable.
    return source.id, "ok", items

def normalize_saved_items(rows, source):
    final, seen = [], set()
    for saved in rows[:max(1, len(SOURCES)) * MAX_ITEMS]:
        if not isinstance(saved, dict) or saved.get("sourceId") != source.id:
            continue
        url = safe_url(saved.get("url"), source.feed, source.hosts, canonical=True)
        title = plain_title(saved.get("title") if isinstance(saved.get("title"), str) else "")
        date = item_date(saved.get("date") if isinstance(saved.get("date"), str) else None)
        if not url or not title or not date or url in seen or promotional(source, None, url, title):
            continue
        seen.add(url)
        candidates = saved.get("media")
        candidates = candidates[:MAX_MEDIA] if isinstance(candidates, list) else []
        media = pick_media(source, [(m.get("kind"), m.get("url"), "") for m in candidates if isinstance(m, dict)], url)
        final.append({"id": sha256((source.id + "|" + url).encode()).hexdigest()[:24],
                      "sourceId": source.id, "title": title, "url": url, "date": date, "media": media})
        if len(final) >= MAX_ITEMS:
            break
    return final

def read_bounded_json(path):
    with path.open("rb") as stream:
        blob = stream.read(OUTPUT_BYTES + 1)
    if len(blob) > OUTPUT_BYTES:
        raise Unsafe("local snapshot exceeds read budget")
    return json.loads(blob)

def previous_items(path, source):
    try:
        if not path.is_file():
            return []
        snapshot = read_bounded_json(path)
        if not isinstance(snapshot, dict) or snapshot.get("schemaVersion") != 1 or not isinstance(snapshot.get("items"), list):
            return []
        return normalize_saved_items(snapshot["items"], source)
    except (OSError, ValueError):
        return []

def load_seed(path):
    try:
        data = read_bounded_json(path)
    except (OSError, ValueError) as exc:
        raise Unsafe("editorial seed could not be read as bounded JSON") from exc
    rows = data.get("items") if isinstance(data, dict) else data
    if not isinstance(rows, list) or len(rows) > MAX_ITEMS:
        raise Unsafe("editorial seed needs a short items list")
    if any(not isinstance(r, dict) or not isinstance(r.get("sourceId"), str) or r["sourceId"] not in SEED_SOURCES for r in rows):
        raise Unsafe("editorial seed has an unapproved publisher")
    out = {sid: normalize_saved_items(rows, source) for sid, source in SEED_SOURCES.items()}
    if sum(map(len, out.values())) != len(rows):
        raise Unsafe("editorial seed entries must have unique approved article URLs, dates and titles")
    return [(k, "ok", entries) for k, entries in out.items() if entries]

def refresh(output, *, sources=SOURCES, page_media=False, seed=None, fetcher=fetch, now=None):
    curated = load_seed(seed) if seed else []
    with concurrent.futures.ThreadPoolExecutor(max_workers=WORKERS) as pool:
        gathered = list(pool.map(lambda s: refresh_source(s, page_media=page_media, fetcher=fetcher), sources))
    if not any(status == "ok" for _, status, _ in gathered):
        raise Unsafe("all public sources unavailable; previous output left unchanged")
    indexed = {s.id: s for s in sources}
    gathered.extend(curated)
    items = [item for ident, status, entries in gathered
             for item in (entries if status == "ok" else previous_items(output, indexed[ident]))]
    items.sort(key=lambda item: (item["date"], item["id"]), reverse=True)
    now = now or dt.datetime.now(dt.timezone.utc)
    obj = {"schemaVersion": 1, "generatedAt": now.astimezone(dt.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
           "items": items, "sources": [{"id": ident, "status": status} for ident, status, _ in gathered]}
    payload = (json.dumps(obj, ensure_ascii=False, separators=(",", ":")) + "\n").encode()
    if len(payload) > OUTPUT_BYTES:
        raise Unsafe("digest exceeds output budget")
    output.parent.mkdir(parents=True, exist_ok=True)
    fd, name = tempfile.mkstemp(prefix=".news-", dir=output.parent)
    try:
        with os.fdopen(fd, "wb") as stream:
            stream.write(payload)
        os.replace(name, output)
        if os.name != "nt":
            os.chmod(output, 0o644)
    finally:
        if os.path.exists(name):
            os.unlink(name)
    return obj

def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=Path(__file__).resolve().parents[1]/"data/news-digest.json")
    parser.add_argument("--page-media", action="store_true", help="Enrich at most two approved article heads per source")
    parser.add_argument("--seed", type=Path, help="Public editorial metadata for approved first-party article links")
    args = parser.parse_args(argv)
    try:
        obj = refresh(args.output, page_media=args.page_media, seed=args.seed)
    except (Unsafe, OSError) as exc:
        print(f"INTA News: {exc}", file=sys.stderr)
        return 1
    counts = {state: sum(s["status"] == state for s in obj["sources"]) for state in ("ok", "unavailable")}
    print(f"INTA News: {len(obj['items'])} items; {counts['ok']} sources ok, {counts['unavailable']} unavailable.")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
