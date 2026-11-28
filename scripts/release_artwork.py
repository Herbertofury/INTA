"""Artwork with verified distribution provenance for self-contained public packages."""
import hashlib
import json
from pathlib import Path

FIRST_PARTY_ID = "i70ad9f0981b05341142b"
FIRST_PARTY_PNG = "70ad9f0981b05341142b9502c62980fe7e69e8d8e4458f0cefbd5eb699709f78"
FIRST_PARTY_SVG = "7302bcd200791e670643ebc10558b15fddaa3b394268277294c2282a8946bcd4"
MEDIA_BRANDS = {"app-inta", "app-inta-rescue", "app-media-creator", "rufus", "ventoy"}


def approved_asset(root: Path, registry):
    asset = registry["assets"][FIRST_PARTY_ID]
    mark = root / "docs/assets/inta-mark.svg"
    path = "assets/brand-icons/" + FIRST_PARTY_PNG[:20] + ".png"
    if (asset["path"] != path or asset["sha256"] != FIRST_PARTY_PNG
            or asset["sourceSha256"] != FIRST_PARTY_SVG
            or hashlib.sha256(mark.read_bytes()).hexdigest() != FIRST_PARTY_SVG
            or hashlib.sha256((root / path).read_bytes()).hexdigest() != FIRST_PARTY_PNG):
        raise ValueError("The approved INTA mark changed; review its source and image before release.")
    return asset


def media_registry(root: Path):
    source = json.loads((root / "data/brand-icons.json").read_bytes())
    asset = approved_asset(root, source)
    brands = []
    for record in source["brands"]:
        if record["id"] not in MEDIA_BRANDS:
            continue
        brand = dict(record)
        if brand.get("assetId") != FIRST_PARTY_ID:
            brand.pop("assetId", None)
        brands.append(brand)
    if {b["id"] for b in brands} != MEDIA_BRANDS:
        raise ValueError("The portable creator is missing a required publisher link.")
    return {"schemaVersion": 1, "checkedAt": source["checkedAt"],
            "policy": "The bundled image is INTA's first-party mark. Tool names link to their official publisher sites.",
            "assets": {FIRST_PARTY_ID: asset}, "brands": brands}
