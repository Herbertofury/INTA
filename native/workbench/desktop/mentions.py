"""Compile tool names and aliases for the native directory from the public catalog."""

import json
from pathlib import Path
import sys

here = Path(__file__).resolve().parent
root = here.parents[2]
catalog = json.loads((root / "site/catalog.json").read_bytes())
extra = json.loads((here.parent / "data/additional-tools.json").read_bytes())
legacy = json.loads((here / "mentions.json").read_bytes())
tools = {r["id"]: {"id": r["id"], "name": r["name"]} for r in extra + catalog["tools"]}
links = {}
for record in [
    *catalog["links"],
    *legacy["links"],
    *({"name": r["name"], "kind": "tool", "id": r["id"]} for r in tools.values()),
]:
    if record["kind"] == "tool" and record["id"] in tools:
        links.setdefault(record["name"].casefold(), record)
data = {
    "tools": list(tools.values()),
    "links": sorted(links.values(), key=lambda r: -len(r["name"])),
}
Path(sys.argv[1]).write_text(json.dumps(data, ensure_ascii=False), encoding="utf8")
