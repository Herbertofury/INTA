#!/usr/bin/env python3
"""Package the complete portable INTA Media Creator, including genuine icons."""
from __future__ import annotations
import argparse
import hashlib
import io
import json
from pathlib import Path
import zipfile

ROOT = Path(__file__).resolve().parents[1]


def package(output: Path, root: Path = ROOT):
    registry = json.loads((root / "data/brand-icons.json").read_text())
    entries = {
        "__main__.py": (root / "media/inta-media.py").read_bytes(),
        "inta_brand_ui.py": (root / "media/inta_brand_ui.py").read_bytes(),
        "inta_optics_ui.py": (root / "media/inta_optics_ui.py").read_bytes(),
        "inta-resources/data/brand-icons.json": (root / "data/brand-icons.json").read_bytes(),
        "inta-resources/data/rescue-download-channel.json": (root / "data/rescue-download-channel.json").read_bytes(),
    }
    for asset in registry["assets"].values():
        data = (root / asset["path"]).read_bytes()
        if hashlib.sha256(data).hexdigest() != asset["sha256"]:
            raise ValueError("Icon hash does not match verified registry: " + asset["path"])
        entries["inta-resources/" + asset["path"]] = data
    buffer = io.BytesIO()
    buffer.write(b"#!/usr/bin/env python3\n")
    with zipfile.ZipFile(buffer, "w", compression=zipfile.ZIP_DEFLATED, compresslevel=9) as archive:
        for name, data in sorted(entries.items()):
            info = zipfile.ZipInfo(name, date_time=(2026, 11, 26, 0, 0, 0))
            info.compress_type = zipfile.ZIP_DEFLATED
            info.external_attr = 0o644 << 16
            archive.writestr(info, data)
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_bytes(buffer.getvalue())
    output.chmod(0o755)
    return {"path": str(output), "bytes": output.stat().st_size, "sha256": hashlib.sha256(output.read_bytes()).hexdigest(), "brands": len(registry["brands"]), "icons": len(registry["assets"]), "requires": "Python 3 with Tk; no icon network requests"}


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=ROOT / "dist/INTA-Media-Creator.pyz")
    args = parser.parse_args()
    print(json.dumps(package(args.output.resolve()), indent=2))
