import base64
import hashlib
import json
import os
import re
import runpy
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def apply_release_artwork(out):
    """Keep the published hub and guides usable offline with source-reviewed art."""
    artwork = runpy.run_path(str(ROOT / "scripts/release_artwork.py"))
    mark_id = artwork["FIRST_PARTY_ID"]
    registry = json.loads((ROOT / "data/brand-icons.json").read_bytes())
    approved = artwork["approved_asset"](ROOT, registry)
    site = out / "site"
    catalog_path = site / "catalog.json"
    page_path = site / "index.html"
    catalog = json.loads(catalog_path.read_bytes())
    page = page_path.read_text(encoding="utf-8")
    start_marker = '<script id="inta-data">window.INTA_DATA = '
    end_marker = ';</script>'
    if page.count(start_marker) != 1:
        raise ValueError("Expected one embedded hub catalog")
    start = page.index(start_marker) + len(start_marker)
    end = page.index(end_marker, start)
    if json.loads(page[start:end]) != catalog:
        raise ValueError("Standalone hub and catalog data differ before packaging")
    brands = catalog["brands"]
    mark_uri = brands["assets"][mark_id]
    prefix = "data:image/png;base64,"
    if not mark_uri.startswith(prefix) or hashlib.sha256(base64.b64decode(mark_uri[len(prefix):], validate=True)).hexdigest() != approved["sha256"]:
        raise ValueError("The embedded INTA mark differs from its reviewed source")
    # Existing portal links support a decorative image. The same original arrow
    # retains the separate, keyboard-focusable official link beside a source link.
    arrow = b'<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 20 20"><path d="M5 15 15 5M7 5h8v8" fill="none" stroke="#8bb8ff" stroke-width="2"/></svg>'
    link_id = "inta-official-link-indicator"
    brands["assets"] = {mark_id: mark_uri, link_id: "data:image/svg+xml;base64," + base64.b64encode(arrow).decode("ascii")}
    brands["assetUrls"] = {}
    for brand in brands["brands"]:
        if brand.get("assetId") != mark_id:
            brand["assetId"] = link_id
    serialized = json.dumps(catalog, ensure_ascii=False, separators=(",", ":"))
    safely_embedded = serialized.replace("<", "\\u003c").replace("\u2028", "\\u2028").replace("\u2029", "\\u2029")
    page_path.write_text(page[:start] + safely_embedded + page[end:], encoding="utf-8", newline="\n")
    catalog_path.write_text(serialized + "\n", encoding="utf-8", newline="\n")

    generated_icon = runpy.run_path(str(ROOT / "scripts/brand_markdown.py"))["GENERATED_ICON"]
    image = re.compile(r'<img\b[^>]*>', re.IGNORECASE)
    mentions = 0
    for directory in (out / "wiki", out / "docs"):
        for guide in directory.rglob("*.md"):
            content = guide.read_text(encoding="utf-8")
            # Preserve the publisher anchor, title and existing contextual links.
            def offline_link(match):
                nonlocal mentions
                mentions += 1
                tag = match.group()
                if approved["path"] in tag:
                    local = Path(os.path.relpath(site / "inta-app-icon.svg", guide.parent)).as_posix()
                    return re.sub(r'(<img\b[^>]*\bsrc=")[^"]+"', lambda m: m[1] + local + '"', tag, count=1)
                return image.sub("&#8599;", tag, count=1)
            guide.write_text(generated_icon.sub(offline_link, content), encoding="utf-8", newline="\n")

    manifest_path = site / "build-manifest.json"
    manifest = json.loads(manifest_path.read_bytes())
    shipped = {}
    for relative in manifest["files"]:
        path = (out / relative).resolve()
        if not path.is_relative_to(out.resolve()):
            raise ValueError("Hub manifest contains a path outside the release directory")
        if path.is_file():
            shipped[relative] = hashlib.sha256(path.read_bytes()).hexdigest()
    manifest.update(files=shipped, contentSha256=hashlib.sha256(serialized.encode()).hexdigest(),
                    brandAssets=len(brands["assets"]), brandIdentities=len(brands["brands"]),
                    brandedMentions=mentions, toolsWithIcons=len({b["toolId"] for b in brands["brands"]
                                                                  if b.get("toolId") and b.get("assetId") == mark_id}),
                    brandRegistrySha256=hashlib.sha256(json.dumps(brands, sort_keys=True, ensure_ascii=False).encode()).hexdigest(),
                    artworkProfile="first-party-release")
    manifest_path.write_text(json.dumps(manifest, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")
