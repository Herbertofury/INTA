"""One authentic brand identity registry shared by the app, Markdown and exports.

Runtime needs only the standard library. Icon fetching/normalization is an
explicit maintenance action in refresh-brand-icons.py, never part of a build.
"""
from __future__ import annotations
import base64
import hashlib
import html
import json
from pathlib import Path
from urllib.parse import urlsplit, urlunsplit

ROOT = Path(__file__).resolve().parents[1]
RAW_ASSETS = 'https://raw.githubusercontent.com/Herbertofury/INTA/main/'


def load_registry(root: Path = ROOT) -> dict:
    return json.loads((Path(root) / 'data/brand-icons.json').read_text(encoding='utf-8'))


def _url_key(value: str) -> str:
    parsed = urlsplit(str(value))
    return urlunsplit((parsed.scheme.lower(), parsed.netloc.lower(), parsed.path.rstrip('/'), parsed.query, ''))


_INDEX_CACHE = []


def _indexes(registry: dict):
    # The build loads one immutable registry. Reuse its indexes for every
    # repeated mention; keep at most two distinct build inputs resident.
    for original, indexes in _INDEX_CACHE:
        if original is registry:
            return indexes
    tools, names, urls, hosts = {}, {}, {}, {}
    for brand in registry.get('brands', []):
        if brand.get('toolId'):
            tools.setdefault(brand['toolId'], brand)
        for name in brand.get('names', []):
            names.setdefault(name.casefold().strip(), brand)
        for url in brand.get('urls', []):
            key = _url_key(url)
            if key not in urls or brand.get('kind') == 'tool' and urls[key].get('kind') != 'tool':
                urls[key] = brand
        for host in brand.get('hosts', []):
            if host not in hosts or brand.get('kind') == 'website' and hosts[host].get('kind') != 'website':
                hosts[host] = brand
    indexes = tools, names, urls, hosts
    _INDEX_CACHE.append((registry, indexes))
    del _INDEX_CACHE[:-2]
    return indexes


def resolve_brand(registry: dict, name: str | None = None, url: str | None = None,
                  tool_id: str | None = None) -> dict | None:
    """Prefer exact identities; a shared hosting domain is never a product name."""
    tools, names, urls, hosts = _indexes(registry)
    if tool_id and tool_id in tools:
        return tools[tool_id]
    if name and name.casefold().strip() in names:
        return names[name.casefold().strip()]
    if url:
        exact = urls.get(_url_key(url))
        if exact:
            return exact
        return hosts.get((urlsplit(url).hostname or '').lower())
    return None


def asset_data_url(registry: dict, asset_id: str, root: Path = ROOT) -> str:
    item = registry['assets'][asset_id]
    content = (Path(root) / item['path']).read_bytes()
    return 'data:' + item['mimeType'] + ';base64,' + base64.b64encode(content).decode('ascii')


def portal_icons(registry: dict, root: Path = ROOT) -> dict:
    """Embed each image once; repeated mentions reference its stable asset ID."""
    return {
        'assets': {key: asset_data_url(registry, key, root) for key in registry['assets']},
        'assetUrls': {key: RAW_ASSETS + asset['path'] for key, asset in registry['assets'].items()},
        'brands': registry['brands'],
        'checkedAt': registry['checkedAt'],
    }


def icon_html(brand: dict | None, registry: dict, *, embedded: bool = False,
              size: int = 18, root: Path = ROOT, css_class: str = 'brand-icon') -> str:
    if not brand or not brand.get('assetId'):
        return ''
    asset_id = brand['assetId']
    asset = registry['assets'][asset_id]
    src = asset_data_url(registry, asset_id, root) if embedded else RAW_ASSETS + asset['path']
    return (f'<img class="{html.escape(css_class, quote=True)}" '
            f'src="{html.escape(src, quote=True)}" width="{int(size)}" height="{int(size)}" '
            f'alt="" aria-hidden="true" data-brand-id="{html.escape(brand["id"], quote=True)}">')


def validate_registry(registry: dict, root: Path = ROOT) -> list[str]:
    errors = []
    brands = registry.get('brands', [])
    ids = [b.get('id') for b in brands]
    if len(ids) != len(set(ids)):
        errors.append('Duplicate brand identity')
    for b in brands:
        if b.get('assetId') not in registry.get('assets', {}):
            errors.append(f'{b.get("id")}: missing authentic cached icon')
        for u in [b.get('downloadPage', ''), *b.get('urls', [])]:
            p = urlsplit(u)
            if p.scheme != 'https' or not p.hostname or p.username or p.password:
                errors.append(f'{b.get("id")}: unsafe brand URL')
    for key, a in registry.get('assets', {}).items():
        p = Path(root) / a['path']
        if not p.is_file():
            errors.append(f'{key}: missing asset file')
            continue
        raw = p.read_bytes()
        if hashlib.sha256(raw).hexdigest() != a['sha256']:
            errors.append(f'{key}: asset hash mismatch')
        if not raw.startswith(b'\x89PNG\r\n\x1a\n') or a['mimeType'] != 'image/png':
            errors.append(f'{key}: non-PNG cached image')
        if not (8 <= a.get('width', 0) <= 64 and 8 <= a.get('height', 0) <= 64):
            errors.append(f'{key}: invalid thumbnail dimensions')
        if not a.get('sourceUrl', '').startswith('https://'):
            errors.append(f'{key}: missing acquisition provenance')
    return errors
