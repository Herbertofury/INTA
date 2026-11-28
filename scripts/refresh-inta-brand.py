#!/usr/bin/env python3
"""Install explicit artwork from an INTA icon pack and regenerate stable assets."""
import argparse
import base64
import hashlib
import io
import json
import struct
import subprocess
import sys
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
ASSETS = "assets/inta-brand"
ICO = "native/workbench/desktop/inta.ico"
THUMB = "assets/brand-icons/70ad9f0981b05341142b.png"  # stable URL in published Markdown
REGISTRY = "data/brand-icons.json"
SOURCE_PACK = f"{ASSETS}/source/INTA-Complete-Icon-Pack.zip"
SOURCE_SUM = f"{ASSETS}/source/SHA256SUMS"
PNG_MAGIC = b"\x89PNG\r\n\x1a\n"
APP_SIZES = (64, 192, 512)
HERITAGE_SIZE = 192
MSIX_SIZES = (44, 50, 150)
MSIX_PATH = "desktop/INTA.Desktop/msix"
MSIX_LOCK = f"{ASSETS}/msix-icons.json"


def icon_source():
    return f"{ASSETS}/application-64.png"


def png_size(blob):
    if len(blob) < 33 or not blob.startswith(PNG_MAGIC) or blob[12:16] != b"IHDR":
        raise ValueError("Expected PNG logo bytes")
    return struct.unpack_from(">II", blob, 16)


def ico_sizes(blob):
    if len(blob) < 6 or blob[:4] != b"\0\0\x01\0":
        raise ValueError("Expected a Windows ICO")
    count, = struct.unpack_from("<H", blob, 4)
    if not 1 <= count <= 32 or len(blob) < 6 + count * 16:
        raise ValueError("Invalid ICO frame table")
    sizes = set()
    for frame in range(count):
        w, h, _, _, _, _, size, offset = struct.unpack_from("<BBBBHHII", blob, 6 + 16 * frame)
        if size < 12 or offset < 6 + count * 16 or offset + size > len(blob):
            raise ValueError("Invalid ICO frame bounds")
        sizes.add((w or 256, h or 256))
    return sizes


def png_uri(data):
    return "data:image/png;base64," + base64.b64encode(data).decode("ascii")


def app_svg(app, size):
    return (f'<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" '
            f'width="{size}" height="{size}" viewBox="0 0 {size} {size}"><title>INTA</title>'
            f'<image width="{size}" height="{size}" xlink:href="{png_uri(app)}"/></svg>\n').encode("ascii")


def banner(heritage, subtitle, height):
    return (f'<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" '
            f'width="1280" height="{height}" viewBox="0 0 1280 {height}" role="img" aria-labelledby="title">\n'
            '<title id="title">INTA Computer Services</title>\n'
            f'<rect width="1280" height="{height}" rx="12" fill="#111820"/>\n'
            f'<image x="28" y="{(height - 112)//2}" width="112" height="112" xlink:href="{png_uri(heritage)}"/>\n'
            '<g font-family="Arial,Helvetica,sans-serif">\n'
            f'<text x="166" y="{height//2 - 1}" fill="#f4f6fb" font-weight="700" font-size="38">INTA Computer Services</text>\n'
            f'<text x="168" y="{height//2 + 35}" fill="#b8c6d7" font-size="20">{subtitle}</text></g></svg>\n').encode("ascii")


def canonical_inputs(root):
    files = {f"{ASSETS}/application-{size}.png": (root / f"{ASSETS}/application-{size}.png").read_bytes()
             for size in APP_SIZES}
    files[f"{ASSETS}/heritage-{HERITAGE_SIZE}.png"] = (root / f"{ASSETS}/heritage-{HERITAGE_SIZE}.png").read_bytes()
    files[ICO] = (root / ICO).read_bytes()
    return files


def brand_cache_version(files):
    seal = hashlib.sha256(b"inta-brand-surfaces-v2\0")
    for name, data in sorted(files.items()):
        seal.update(name.encode("ascii") + b"\0" + hashlib.sha256(data).digest())
    return seal.hexdigest()[:16]


def source_checksum(data):
    return (hashlib.sha256(data).hexdigest() + "  " + Path(SOURCE_PACK).name + "\n").encode("ascii")


def pack_inputs(path):
    paths = {f"{ASSETS}/application-{size}.png": f"Application/PNG/INTA-Application-{size}x{size}.png" for size in APP_SIZES}
    paths[f"{ASSETS}/heritage-{HERITAGE_SIZE}.png"] = f"Heritage/PNG/INTA-Heritage-{HERITAGE_SIZE}x{HERITAGE_SIZE}.png"
    paths[ICO] = "Application/INTA-Application.ico"
    with zipfile.ZipFile(path) as archive:
        result = {}
        for destination, source in paths.items():
            entries = [item for item in archive.infolist() if item.filename == source]
            if len(entries) != 1 or entries[0].file_size > 512_000:
                raise ValueError(f"Pack needs exactly one bounded {source}")
            result[destination] = archive.read(entries[0])
    return result


def generated(root, files):
    for name, data in files.items():
        if name.endswith(".png"):
            size = int(Path(name).stem.rsplit("-", 1)[-1])
            if png_size(data) != (size, size):
                raise ValueError(f"Wrong size for {name}")
    if not {(16, 16), (32, 32), (48, 48), (256, 256)} <= ico_sizes(files[ICO]):
        raise ValueError("ICO is missing required Windows sizes")
    small = files[icon_source()]
    heritage = files[f"{ASSETS}/heritage-{HERITAGE_SIZE}.png"]
    registry = json.loads((root / REGISTRY).read_text(encoding="utf-8"))
    row = registry["assets"]["i70ad9f0981b05341142b"]
    digest = hashlib.sha256(small).hexdigest()
    source = "https://raw.githubusercontent.com/Herbertofury/INTA/main/" + icon_source()
    row.update(sourceUrl=source, resolvedSourceUrl=source, sourceKind="User-supplied first-party INTA Application logo",
               sha256=digest, sourceSha256=digest)
    return {
        THUMB: small,
        REGISTRY: (json.dumps(registry, ensure_ascii=False, indent=2) + "\n").encode("utf-8"),
        "docs/assets/inta-mark.svg": app_svg(small, 64),
        "docs/assets/inta-banner.svg": banner(heritage, "Technician workspace  /  Repair, maintenance and support", 160),
        "docs/assets/wiki-hero.svg": banner(heritage, "Field knowledge base  /  Procedures, tools and recovery", 144),
    }


def validate_transparency(files):
    from PIL import Image
    for name, data in files.items():
        image = Image.open(io.BytesIO(data))
        frames = (image.ico.getimage(size) for size in image.ico.sizes()) if name == ICO else (image,)
        for frame in frames:
            alpha = frame.convert("RGBA").getchannel("A")
            w, h = alpha.size
            if any(alpha.getpixel(point) for point in ((0, 0), (0, h-1), (w-1, 0), (w-1, h-1))) or not alpha.getbbox():
                raise ValueError(f"{name}: artwork must be visible on a transparent canvas, without a solid backing")


def msix_assets(files, root, check):
    source = files[f"{ASSETS}/application-192.png"]
    if check:
        logos = {f"{MSIX_PATH}/logo{size}.png": (root / f"{MSIX_PATH}/logo{size}.png").read_bytes()
                 for size in MSIX_SIZES}
    else:
        from PIL import Image
        image = Image.open(io.BytesIO(source)).convert("RGBA")
        logos = {}
        for size in MSIX_SIZES:
            out = io.BytesIO()
            image.resize((size, size), resample=Image.Resampling.LANCZOS).save(out, format="PNG")
            logos[f"{MSIX_PATH}/logo{size}.png"] = out.getvalue()
    for size, blob in zip(MSIX_SIZES, logos.values()):
        if png_size(blob) != (size, size):
            raise ValueError(f"Incorrect {size}px MSIX icon")
    seal = {name: hashlib.sha256(blob).hexdigest() for name, blob in logos.items()}
    seal["sourceSha256"] = hashlib.sha256(source).hexdigest()
    return {**logos, MSIX_LOCK: (json.dumps(seal, indent=2, sort_keys=True) + "\n").encode("ascii")}


def web_brand_errors(root, files):
    site = root / "site"
    app192 = files[f"{ASSETS}/application-192.png"]
    app512 = files[f"{ASSETS}/application-512.png"]
    heritage = files[f"{ASSETS}/heritage-{HERITAGE_SIZE}.png"]
    ico = b"data:image/x-icon;base64," + base64.b64encode(files[ICO])
    wanted = {
        "inta-touch-icon.png": app192,
        "inta-app-icon.svg": app_svg(app512, 512),
    }
    bad = [name for name, data in wanted.items() if not (site / name).is_file()
           or (site / name).read_bytes() != data]
    html = site / "index.html"
    expected_html = (
        b'<link rel="icon" type="image/x-icon" href="' + ico + b'">',
        b'<img class="brand-mark" src="' + png_uri(files[icon_source()]).encode("ascii") + b'"',
        b'<template id="inta-home-brand"><img class="home-heritage-mark" src="' + png_uri(heritage).encode("ascii") + b'"',
    )
    page = html.read_bytes() if html.is_file() else b""
    if not all(token in page for token in expected_html):
        bad.append(html.name)
    worker = site / "sw.js"
    cache = f'const CACHE = "inta-shell-{brand_cache_version(files)}";'.encode("ascii")
    if not worker.is_file() or cache not in worker.read_bytes():
        bad.append(worker.name)
    return ["site/" + name for name in bad]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--pack", type=Path, help="Import the supplied Application and Heritage pack")
    mode.add_argument("--check", action="store_true", help="Verify existing derived assets without writing")
    mode.add_argument("--sync", action="store_true", help="Regenerate from the committed source pair and ICO")
    args = parser.parse_args()
    archive = args.pack.read_bytes() if args.pack else None
    files = pack_inputs(io.BytesIO(archive)) if archive is not None else canonical_inputs(ROOT)
    if not args.check:
        validate_transparency(files)
    outputs = {**generated(ROOT, files), **msix_assets(files, ROOT, args.check)}
    if args.check:
        errors = [name for name, data in outputs.items() if not (ROOT / name).exists() or (ROOT / name).read_bytes() != data]
        errors.extend(web_brand_errors(ROOT, files))
        original = (ROOT / SOURCE_PACK).read_bytes()
        if (ROOT / SOURCE_SUM).read_bytes() != source_checksum(original) or pack_inputs(io.BytesIO(original)) != files:
            errors.append(SOURCE_PACK)
        if errors:
            parser.exit(1, "Stale INTA branding: " + ", ".join(errors) + "\n")
    else:
        if archive is not None:
            outputs = {**files, **outputs, SOURCE_PACK: archive, SOURCE_SUM: source_checksum(archive)}
        for name, data in outputs.items():
            path = ROOT / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data)
        # Native resources are built independently, but the committed site
        # and offline icons must refresh in the same authoring operation.
        subprocess.run([sys.executable, "-B", str(ROOT / "scripts/build-hub.py")],
                       cwd=ROOT, check=True)
    print("INTA Application and Heritage branding verified.")


if __name__ == "__main__":
    main()
