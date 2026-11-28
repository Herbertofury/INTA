"""Compare the actual release executable against independent Windows and Python readings."""
import argparse
import concurrent.futures
import ctypes
import datetime
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import winreg


def run(binary, output):
    project = Path(__file__).resolve().parents[1]
    root = project.parents[1]
    executable_hash = hashlib.file_digest(binary.open("rb"), "sha256").hexdigest()

    def call(*args):
        outcome = subprocess.run([binary, *map(str, args)], capture_output=True, timeout=20, check=False)
        body = json.loads(outcome.stdout)
        assert body.get("schemaVersion") == 1 and body.get("ok") == (outcome.returncode == 0), body
        if outcome.returncode:
            raise AssertionError(body)
        return body["result"]

    public = json.loads((root / "data/tools.json").read_bytes())
    extended = json.loads((root / "native/workbench/data/additional-tools.json").read_bytes())
    records = {r["id"]: r for r in extended}
    records.update({r["id"]: r for r in public})
    listing = call("discover")
    assert listing["catalogCount"] == len(records)
    found = call("tools", "search", "", "500")
    assert {r["id"] for r in found["items"]} == records.keys()

    def check_record(tool_id):
        assert call("tools", "inspect", tool_id)["tool"] == records[tool_id]

    with concurrent.futures.ThreadPoolExecutor(4) as pool:
        list(pool.map(check_record, records))
    for op in listing["operations"]:
        assert call("schema", op["id"]) == op

    class Memory(ctypes.Structure):
        _fields_ = [("size", ctypes.c_uint32), ("load", ctypes.c_uint32)] + [
            (n, ctypes.c_uint64) for n in ("total", "available", "totalPage", "availPage", "virtual", "availVirtual", "availExtended")]

    native = Memory()
    native.size = ctypes.sizeof(native)
    assert ctypes.windll.kernel32.GlobalMemoryStatusEx(ctypes.byref(native))
    memory = call("run", "system.memory")
    assert native.total == memory["totalPhysicalBytes"]
    assert 0 < memory["availablePhysicalBytes"] <= native.total and 0 <= memory["loadPercent"] <= 100
    pc = Path(os.environ["SystemRoot"]) / "System32/powercfg.exe"
    uuid_pattern = rb"[0-9a-fA-F]{8}-(?:[0-9a-fA-F]{4}-){3}[0-9a-fA-F]{12}"
    plans = subprocess.check_output([pc, "/list"], timeout=10)
    active = subprocess.check_output([pc, "/getactivescheme"], timeout=10)
    observed = call("run", "system.power")
    assert {v.decode().lower() for v in re.findall(uuid_pattern, plans)} == set(observed["planIds"])
    assert re.search(uuid_pattern, active).group().decode().lower() == observed["activeId"]
    startup = call("run", "startup.counts")
    for entry in startup["sources"]:
        base = winreg.HKEY_CURRENT_USER if entry["scope"] == "current-user" else winreg.HKEY_LOCAL_MACHINE
        view = {"shared": 0, "32-bit": winreg.KEY_WOW64_32KEY, "64-bit": winreg.KEY_WOW64_64KEY}[entry["view"]]
        try:
            with winreg.OpenKey(base, r"Software\Microsoft\Windows\CurrentVersion" + "\\" + entry["key"],
                                0, winreg.KEY_QUERY_VALUE | view) as key:
                assert winreg.QueryInfoKey(key)[1] == entry["registered"] and entry["state"] == "read"
        except FileNotFoundError:
            assert entry["state"] == "not-present" and entry["registered"] == 0
    assert len(startup["sources"]) == 6 and startup["registeredValues"] == sum(r["registered"] for r in startup["sources"])

    output.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(dir=output.parent) as temp:
        sample = Path(temp) / "non-executable archive fixture #Ω.bin"
        content = b"INTA capability test\x00\n" * 18001
        sample.write_bytes(content)
        inputs = Path(temp) / "selected-file.json"
        inputs.write_text(json.dumps({"path": str(sample)}), encoding="utf-8")
        digest = call("run", "file.sha256", "--input", inputs)
        assert digest == {"sha256": hashlib.sha256(content).hexdigest(), "bytes": len(content)} and sample.read_bytes() == content
        manifest = project / "examples/verify-workstation-basics.json"
        preview = call("forge", "verify", manifest)
        store = Path(temp) / "offline recipes"
        with concurrent.futures.ThreadPoolExecutor(2) as pool:
            imports = list(pool.map(lambda _: call("forge", "import", manifest, "--store", store)["state"], range(2)))
        assert sorted(imports) == ["already-imported", "imported"]
        receipt = call("forge", "run", preview["id"], "--store", store)
        assert preview["testsPassed"] == receipt["testsPassed"] == 4 and "observations" in receipt

    record = {
        "checkedUtc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "version": call("--version")["version"], "executableSha256": executable_hash,
        "host": {"platform": sys.platform, "build": sys.getwindowsversion().build, "machine": os.environ.get("PROCESSOR_ARCHITECTURE")},
        "catalog": {"canonical": len(public), "additional": len(extended), "unique": len(records), "exactReadbacks": len(records)},
        "schemaReadbacks": len(listing["operations"]), "operations": [r["id"] for r in listing["operations"]],
        "nativeComparisons": {"physicalMemoryMatches": True, "allPowerPlanIdsAndActiveSchemeMatch": True,
                              "sixRegistryViewsMatch": True, "powerPlanCount": observed["count"]},
        "localFile": digest, "sourcePreserved": True,
        "forge": {"recipeSha256": preview["manifestSha256"], "verifiedVectors": preview["testsPassed"],
                  "concurrentPublication": sorted(imports), "successfulRerun": True},
    }
    with output.open("x", encoding="utf-8") as dest:
        json.dump(record, dest, indent=2)
        dest.write("\n")
    print(output)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("binary", type=Path)
    parser.add_argument("new_receipt", type=Path)
    args = parser.parse_args()
    run(args.binary.resolve(), args.new_receipt.resolve())
