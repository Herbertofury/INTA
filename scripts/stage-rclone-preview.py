"""Stage a publisher-checksummed rclone v1.76.0 binary for disposable CI only."""
import hashlib
import io
from pathlib import Path
import sys
import urllib.request
import zipfile

PLATFORMS = {"linux": "b00f00b3266c3764413afe599d3ec316f6f43b3e0ea8a0bc1d186d2133e46df7",
             "win32": "8b4fd2737ade4e70c057ef4a49deb7cffe94b0fb9d7587c189d149823f21a1dc"}
platform = {"win32": "windows", "linux": "linux"}[sys.platform]
stem = f"rclone-v1.76.0-{platform}-amd64"
with urllib.request.urlopen(f"https://downloads.rclone.org/v1.76.0/{stem}.zip", timeout=60) as source:
    archive = source.read(64 * 1024 * 1024 + 1)
if len(archive) > 64 * 1024 * 1024 or hashlib.sha256(archive).hexdigest() != PLATFORMS[sys.platform]:
    raise SystemExit("Publisher archive differs from its reviewed digest")
dest = Path(sys.argv[1])
dest.mkdir()
name = "rclone.exe" if platform == "windows" else "rclone"
with zipfile.ZipFile(io.BytesIO(archive)) as z:
    item = z.getinfo(f"{stem}/{name}")
    if item.file_size > 96 * 1024 * 1024:
        raise SystemExit("Unexpected executable size")
    data = z.read(item)
p = dest / name
with p.open("xb") as handle:
    handle.write(data)
p.chmod(0o700)
print(p.resolve())
