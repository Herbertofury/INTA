#!/usr/bin/env python3
"""Package a self-contained INTA Local Agent publish directory."""
from pathlib import Path
import sys
import zipfile

if len(sys.argv) != 3:
    raise SystemExit("usage: package.py <publish-directory> <output.zip>")
source = Path(sys.argv[1]).resolve()
output = Path(sys.argv[2]).resolve()
if not source.is_dir() or not any(source.iterdir()):
    raise SystemExit("publish directory is empty or missing")
output.parent.mkdir(parents=True, exist_ok=True)
with zipfile.ZipFile(output, "w", compression=zipfile.ZIP_DEFLATED, compresslevel=9) as archive:
    for path in sorted(source.rglob("*")):
        if path.is_file():
            archive.write(path, path.relative_to(source))
print(f"Packaged {sum(1 for p in source.rglob('*') if p.is_file())} files into {output}")
