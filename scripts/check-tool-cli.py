"""Catalog-level audit of INTA's reviewed command-line control coverage."""
import argparse
import collections
from datetime import datetime, timezone
import json
import pathlib
import re
import sys
from urllib.parse import urlparse

ROOT = pathlib.Path(__file__).resolve().parents[1]
ID = re.compile(r"[a-z0-9]+(?:-[a-z0-9]+)*\Z")
HASH = re.compile(r"[a-f0-9]{64}\Z")
MODES = {"candidate", "tested", "tested-preview", "manual-only", "blocked"}
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


def as_operation(entry, now):
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
    if state in {"tested", "tested-preview"}:
        version, digest = entry.get("version"), entry.get("artifactSha256")
        if not isinstance(version, str) or not version.strip() or len(version) > 96:
            raise ValueError("tested operation requires exact version")
        if not isinstance(digest, str) or not HASH.fullmatch(digest) or not https(entry.get("runtimeEvidence")):
            raise ValueError("tested operation requires exact artifact and runtime proof")
    if state == "tested-preview":
        expires = entry.get("expiresAt")
        archive = entry.get("archiveSha256")
        if (entry["effect"] != "read-only" or not isinstance(expires, str)
                or not re.fullmatch(r"[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z", expires)
                or not isinstance(archive, str) or not HASH.fullmatch(archive) or not https(entry.get("artifactUrl"))):
            raise ValueError("nonproduction preview requires read-only scope, archive pin and UTC expiry")
        if datetime.fromisoformat(expires[:-1] + "+00:00") <= now:
            return "expired-preview"
    return state


def catalog_ids(catalog):
    if not isinstance(catalog, list) or not all(isinstance(t, dict) and slug(t.get("id")) for t in catalog):
        raise ValueError("invalid tool catalog")
    ids = {t["id"] for t in catalog}
    if len(ids) != len(catalog):
        raise ValueError("duplicate catalog tool")
    return ids


def audit(catalog, ledger, now=None):
    now = datetime.now(timezone.utc) if now is None else now
    ids = catalog_ids(catalog)
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
        state = {key: as_operation(value, now) for key, value in operations.items()}
        tested = sorted(k for k in required if state.get(k) == "tested")
        missing = sorted(set(required) - set(tested))
        candidates = sorted(k for k in required if state.get(k) == "candidate")
        status = "ready" if not missing and entry["scope"] == "complete" else "partial"
        def binding(k, fields):
            return {field: operations[k][field] for field in ("platform", "version", "artifactSha256", "effect", *fields)}
        metadata = {k: binding(k, ()) for k in tested}
        preview = {k: binding(k, ("expiresAt", "archiveSha256", "artifactUrl"))
                   for k in required if state.get(k) == "tested-preview"}
        rows[tool_id] = {"status": status, "tested": tested, "candidate": candidates, "bindings": metadata,
                         "previews": preview, "expiredPreviews": sorted(k for k in required if state.get(k) == "expired-preview"),
                         "missing": missing, "manual": sorted(k for k in required if state.get(k) in {"manual-only", "blocked"})}
    return {"toolCount": len(ids), "summary": dict(collections.Counter(row["status"] for row in rows.values())), "tools": rows}


OFFER_PLATFORM_LABELS = {
    "windows-x64": {"Windows", "Windows 11"},
    "windows-arm64": {"Windows", "Windows 11"},
    "linux-x64": {"Linux"},
    "linux-arm64": {"Linux"},
    "macos-x64": {"macOS"},
    "macos-arm64": {"macOS"},
    "android-arm64": {"Android"},
    "ios-arm64": {"iOS"},
    "web": {"Web", "Web browser"},
}


def new_offer_gaps(report, base_catalog, catalog):
    """Do not expand the legacy catalog beyond complete verified control."""
    original = catalog_ids(base_catalog)
    result = {}
    current = {tool["id"]: tool for tool in catalog}
    for tool_id, row in report["tools"].items():
        if tool_id in original:
            continue
        # Production claims must not combine tests from different editions,
        # platforms or artifact builds into a single tool-wide release claim.
        fingerprints = {(entry["platform"], entry["version"], entry["artifactSha256"])
                        for entry in row.get("bindings", {}).values()}
        if row["status"] != "ready":
            result[tool_id] = row["status"]
        elif len(fingerprints) != 1:
            result[tool_id] = "mixed-tested-builds"
        else:
            advertised = current[tool_id].get("platforms")
            platform = next(iter(fingerprints))[0]
            if (not isinstance(advertised, list) or not advertised
                    or not all(isinstance(value, str) and value in OFFER_PLATFORM_LABELS.get(platform, ())
                               for value in advertised)):
                result[tool_id] = "unverified-advertised-platform"
    return result


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--catalog", type=pathlib.Path, default=ROOT / "data/tools.json")
    parser.add_argument("--coverage", type=pathlib.Path, default=ROOT / "data/tool-cli-coverage.json")
    parser.add_argument("--json", action="store_true")
    gate = parser.add_mutually_exclusive_group()
    gate.add_argument("--require-full", metavar="TOOL")
    gate.add_argument("--require-operation", nargs=2, metavar=("TOOL", "OPERATION"))
    gate.add_argument("--require-new-offers", type=pathlib.Path, metavar="BASE_CATALOG")
    parser.add_argument("--platform")
    parser.add_argument("--version")
    parser.add_argument("--artifact-sha256")
    parser.add_argument("--preview", action="store_true", help="explicitly allow an unexpired local read-only test preview")
    args = parser.parse_args(argv)
    if args.preview and not args.require_operation:
        parser.error("--preview is valid only for an exact operation")
    if (args.require_full or args.require_operation) and not slug(args.platform):
        parser.error("a gate requires a platform slug")
    if args.require_operation and (not args.version or not args.artifact_sha256 or not HASH.fullmatch(args.artifact_sha256)):
        parser.error("operation admission requires exact version and reviewed artifact SHA-256")
    try:
        catalog = load(args.catalog)
        report = audit(catalog, load(args.coverage))
        new_gaps = new_offer_gaps(report, load(args.require_new_offers), catalog) if args.require_new_offers else None
    except (OSError, ValueError, TypeError, RecursionError) as exc:
        print(f"Invalid CLI coverage: {exc}", file=sys.stderr)
        return 2
    if new_gaps is not None:
        payload = {"accepted": not new_gaps, "blockedNewOffers": new_gaps}
        print(json.dumps(payload, sort_keys=True))
        return int(bool(new_gaps))
    tool = args.require_full or (args.require_operation[0] if args.require_operation else None)
    if tool:
        row = report["tools"].get(tool, {"status": "unknown-tool", "tested": []})
        bindings = row.get("bindings", {})
        if args.require_full:
            ready = row["status"] == "ready" and all(item["platform"] == args.platform for item in bindings.values())
        else:
            item = bindings.get(args.require_operation[1])
            if item is None and args.preview:
                item = row.get("previews", {}).get(args.require_operation[1])
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
