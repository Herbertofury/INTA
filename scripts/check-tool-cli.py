"""Catalog-level audit of INTA's reviewed command-line control coverage."""
import argparse
import collections
import json
import pathlib
import re
import sys
from urllib.parse import urlparse

ROOT = pathlib.Path(__file__).resolve().parents[1]
ID = re.compile(r"[a-z0-9]+(?:-[a-z0-9]+)*\Z")
HASH = re.compile(r"[a-f0-9]{64}\Z")
MODES = {"candidate", "tested", "manual-only", "blocked"}
BACKENDS = {"official-cli", "official-api", "cli-anything"}
EFFECTS = {"read-only", "changes-device"}


def load(path):
    def unique(members):
        result = {}
        for key, value in members:
            if key in result:
                raise ValueError("duplicate JSON member: " + key)
            result[key] = value
        return result
    return json.loads(path.read_text(encoding="utf-8"), object_pairs_hook=unique)


def slug(value):
    return isinstance(value, str) and ID.fullmatch(value) is not None


def https(value):
    if not isinstance(value, str) or any(c.isspace() for c in value):
        return False
    parsed = urlparse(value)
    return parsed.scheme == "https" and bool(parsed.hostname) and not (parsed.username or parsed.password)


def as_operation(entry):
    if not isinstance(entry, dict) or entry.get("state") not in MODES:
        raise ValueError("invalid operation entry")
    state = entry["state"]
    if not https(entry.get("source")):
        raise ValueError("each operation requires a primary HTTPS source")
    if state in {"blocked", "manual-only"}:
        if not isinstance(entry.get("note"), str) or not entry["note"].strip():
            raise ValueError("manual or blocked operations need a reason")
        return state
    if entry.get("backend") not in BACKENDS or entry.get("effect") not in EFFECTS or not slug(entry.get("platform")):
        raise ValueError("structured operation lacks backend, platform or effect")
    if state == "tested":
        version, digest = entry.get("version"), entry.get("artifactSha256")
        if not isinstance(version, str) or not version.strip() or len(version) > 96:
            raise ValueError("tested operation requires exact version")
        if not isinstance(digest, str) or not HASH.fullmatch(digest) or not https(entry.get("runtimeEvidence")):
            raise ValueError("tested operation requires exact artifact and runtime proof")
    return state


def audit(catalog, ledger):
    if not isinstance(catalog, list) or not all(isinstance(t, dict) and slug(t.get("id")) for t in catalog):
        raise ValueError("invalid tool catalog")
    ids = {t["id"] for t in catalog}
    if len(ids) != len(catalog):
        raise ValueError("duplicate catalog tool")
    if not isinstance(ledger, dict) or type(ledger.get("schemaVersion")) is not int or ledger["schemaVersion"] != 1 or not isinstance(ledger.get("tools"), dict):
        raise ValueError("invalid coverage ledger")
    entries = ledger["tools"]
    if entries.keys() - ids:
        raise ValueError("coverage refers to a tool that isn't in the catalog")
    rows = {}
    for tool_id in sorted(ids):
        entry = entries.get(tool_id)
        if entry is None:
            rows[tool_id] = {"status": "needs-tool-review", "tested": [], "candidate": [], "missing": []}
            continue
        if not isinstance(entry, dict) or entry.get("scope") not in {"partial", "complete"}:
            raise ValueError(f"{tool_id}: invalid reviewed scope")
        required = entry.get("requiredOperations")
        operations = entry.get("operations")
        if (not isinstance(required, list) or not required or not all(slug(i) for i in required)
                or len(required) != len(set(required)) or not isinstance(operations, dict)
                or operations.keys() - set(required)):
            raise ValueError(f"{tool_id}: invalid operation inventory")
        state = {key: as_operation(value) for key, value in operations.items()}
        tested = sorted(k for k in required if state.get(k) == "tested")
        missing = sorted(set(required) - set(tested))
        candidates = sorted(k for k in required if state.get(k) == "candidate")
        status = "ready" if not missing and entry["scope"] == "complete" else "partial"
        metadata = {k: {field: operations[k][field] for field in ("platform", "version", "artifactSha256", "effect")}
                    for k in tested}
        rows[tool_id] = {"status": status, "tested": tested, "candidate": candidates, "bindings": metadata,
                         "missing": missing, "manual": sorted(k for k in required if state.get(k) in {"manual-only", "blocked"})}
    return {"toolCount": len(ids), "summary": dict(collections.Counter(row["status"] for row in rows.values())), "tools": rows}


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--catalog", type=pathlib.Path, default=ROOT / "data/tools.json")
    parser.add_argument("--coverage", type=pathlib.Path, default=ROOT / "data/tool-cli-coverage.json")
    parser.add_argument("--json", action="store_true")
    gate = parser.add_mutually_exclusive_group()
    gate.add_argument("--require-full", metavar="TOOL")
    gate.add_argument("--require-operation", nargs=2, metavar=("TOOL", "OPERATION"))
    parser.add_argument("--platform")
    parser.add_argument("--version")
    parser.add_argument("--artifact-sha256")
    args = parser.parse_args(argv)
    if (args.require_full or args.require_operation) and not slug(args.platform):
        parser.error("a gate requires a platform slug")
    if args.require_operation and (not args.version or not args.artifact_sha256 or not HASH.fullmatch(args.artifact_sha256)):
        parser.error("operation admission requires exact version and reviewed artifact SHA-256")
    try:
        report = audit(load(args.catalog), load(args.coverage))
    except (OSError, ValueError, TypeError, RecursionError) as exc:
        print(f"Invalid CLI coverage: {exc}", file=sys.stderr)
        return 2
    tool = args.require_full or (args.require_operation[0] if args.require_operation else None)
    if tool:
        row = report["tools"].get(tool, {"status": "unknown-tool", "tested": []})
        bindings = row.get("bindings", {})
        if args.require_full:
            ready = row["status"] == "ready" and all(item["platform"] == args.platform for item in bindings.values())
        else:
            item = bindings.get(args.require_operation[1])
            ready = bool(item and (item["platform"], item["version"], item["artifactSha256"])
                         == (args.platform, args.version, args.artifact_sha256))
        if args.json:
            print(json.dumps({"tool": tool, "accepted": ready, **row}))
        elif not ready:
            print(f"{tool}: requested control is not verified ({row['status']})", file=sys.stderr)
        return 0 if ready else 1
    payload = report if args.json else {"toolCount": report["toolCount"], "summary": report["summary"]}
    print(json.dumps(payload, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
