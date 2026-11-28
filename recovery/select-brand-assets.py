"""Copy only the recovery edition's verified publisher icons and registry entries."""
import hashlib
import json
from pathlib import Path
import sys

source, output = map(Path, sys.argv[1:])
registry = json.loads((source / "data/brand-icons.json").read_bytes())
selected = {"app-inta", "app-inta-rescue", "gsmartcontrol", "gparted", "clonezilla", "testdisk", "photorec"}
brands = [b for b in registry["brands"] if b["id"] in selected]
if {b["id"] for b in brands} != selected:
    raise ValueError("Missing recovery brand")
assets = {b["assetId"]: registry["assets"][b["assetId"]] for b in brands}
for asset in assets.values():
    path = Path(asset["path"])
    assert path.parent.as_posix() == "assets/brand-icons"
    data = (source / path).read_bytes()
    if hashlib.sha256(data).hexdigest() != asset["sha256"]:
        raise ValueError("Recovery icon differs from its registry digest")
    (output / path).parent.mkdir(parents=True, exist_ok=True)
    (output / path).write_bytes(data)
(output / "data").mkdir(parents=True, exist_ok=True)
(output / "data/brand-icons.json").write_text(json.dumps({"schemaVersion":1, "brands":brands, "assets":assets})+"\n")
