"""One-shot, JSON CLI harness for the reviewed native INTA Local Agent inventory."""
import argparse
import datetime
import hashlib
import json
import pathlib
import re
import subprocess
import sys
import uuid

MAX_BACKEND = 180 * 1024 * 1024
STATES = {"ready", "incomplete", "unsupported", "unavailable", "unstable"}


def executable(path, expected_sha):
    if (not path.is_absolute() or path.name not in {"inta-agent", "inta-agent.exe"}
            or path.is_symlink() or not path.is_file() or not re.fullmatch(r"[a-f0-9]{64}", expected_sha)):
        raise ValueError("select the absolute path to an already verified INTA agent")
    remaining = MAX_BACKEND
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        while chunk := stream.read(min(1024 * 1024, remaining + 1)):
            remaining -= len(chunk)
            if remaining < 0:
                raise ValueError("INTA binary exceeds configured size bound")
            digest.update(chunk)
    if digest.hexdigest() != expected_sha:
        raise ValueError("INTA agent does not match the approved release pin")
    return path


def unique(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError("ambiguous backend response")
        result[key] = value
    return result


def valid_guid(value):
    try:
        return isinstance(value, str) and str(uuid.UUID(value)) == value
    except ValueError:
        return False


def inventory(path, expected_sha):
    # Exactly one backend entry point. Local administrator-controlled path/pin,
    # not input from a Mobile request or an untrusted catalog record.
    binary = executable(path, expected_sha)
    run = subprocess.run([str(binary), "--power-plans"], capture_output=True, check=True,
                         shell=False, timeout=12)
    if len(run.stdout) > 3 * 1024 * 1024:
        raise ValueError("native response is too large")
    report = json.loads(run.stdout.decode("utf-8"), object_pairs_hook=unique)
    if (not isinstance(report, dict) or set(report) != {"status", "activeId", "plans", "errorCode"}
            or not isinstance(report["status"], str) or report["status"] not in STATES
            or not isinstance(report["plans"], list) or len(report["plans"]) > 256):
        raise ValueError("unexpected native response")
    seen = set()
    for row in report["plans"]:
        if not isinstance(row, dict) or set(row) != {"id", "name"} or not valid_guid(row["id"]) or row["id"] in seen:
            raise ValueError("invalid or duplicate power plan")
        if row["name"] is not None and (not isinstance(row["name"], str) or len(row["name"]) > 2048):
            raise ValueError("invalid power plan label")
        seen.add(row["id"])
    active = report["activeId"]
    if active is not None and not valid_guid(active):
        raise ValueError("invalid active scheme")
    if report["status"] == "ready" and (active is None or active not in seen):
        raise ValueError("ready response lacks an enumerated active scheme")
    error = report["errorCode"]
    if error is not None and (type(error) is not int or not 0 <= error <= 0xFFFFFFFF):
        raise ValueError("invalid native error code")
    return report


def startup_inventory(path, expected_sha, include_folders=False):
    """List documented logon registration names; startup folder names require explicit opt-in."""
    binary = executable(path, expected_sha)
    command = [str(binary), "--startup-inventory"]
    if include_folders:
        command.append("--include-folders")
    run = subprocess.run(command, capture_output=True,
                         check=True, shell=False, timeout=12)
    if len(run.stdout) > 3 * 1024 * 1024:
        raise ValueError("native response is too large")
    report = json.loads(run.stdout.decode("utf-8"), object_pairs_hook=unique)
    keys = {"schema", "scope", "readOnly", "enabledState", "argumentsCollected", "sources"}
    schema = 2 if include_folders else 1
    scope = "windows-logon-registrations-and-local-startup-folders" if include_folders else "windows-logon-run-keys-only"
    if include_folders:
        keys |= {"startupFolders", "startupSettingsUri"}
    if (not isinstance(report, dict) or set(report) != keys or type(report["schema"]) is not int
            or report["schema"] != schema or report["scope"] != scope
            or report["readOnly"] is not True or report["argumentsCollected"] is not False
            or report["enabledState"] != "not-evaluated" or not isinstance(report["sources"], list)):
        raise ValueError("unexpected startup response")
    rows = report["sources"]
    expected_keys = {"scope", "view", "key", "status", "entryNames", "truncated"}
    if len(rows) not in {4, 6}:
        raise ValueError("unexpected number of native startup sources")
    views = {"default"} if len(rows) == 4 else {"32-bit", "64-bit"}
    expected_sources = {(scope, view, key)
                        for scope, choices in (("current-user", {"shared"}), ("machine", views))
                        for view in choices for key in ("Run", "RunOnce")}
    seen = set()
    for row in rows:
        if not isinstance(row, dict) or set(row) != expected_keys:
            raise ValueError("unexpected startup row")
        source = row["scope"], row["view"], row["key"]
        status = row["status"]
        names = row["entryNames"]
        truncated = row["truncated"]
        if (not all(isinstance(value, str) for value in source) or source not in expected_sources or source in seen
                or not isinstance(status, str) or status not in {"read", "not-present", "unavailable"}
                or not isinstance(names, list) or len(names) > 1024 or type(truncated) is not bool
                or (truncated and len(names) != 1024)
                or (status != "read" and (names or truncated))
                or not all(isinstance(name, str) and len(name) <= 16383 and "\0" not in name for name in names)):
            raise ValueError("invalid native startup row")
        if len(set(names)) != len(names):
            raise ValueError("duplicate startup value")
        seen.add(source)
    incomplete = any(r["truncated"] or r["status"] == "unavailable" for r in rows)
    if include_folders:
        folders = report["startupFolders"]
        if (report["startupSettingsUri"] != "ms-settings:startupapps"
                or not isinstance(folders, list) or len(folders) != 2):
            raise ValueError("unexpected startup folder coverage")
        scopes = set()
        for folder in folders:
            if not isinstance(folder, dict) or set(folder) != {"scope", "status", "entryNames", "truncated"}:
                raise ValueError("unexpected startup folder row")
            key, status, names, truncated = (folder[k] for k in ("scope", "status", "entryNames", "truncated"))
            if (not isinstance(key, str) or key not in {"current-user", "all-users"} or key in scopes
                    or not isinstance(status, str) or status not in {"read", "not-present", "unavailable", "location-not-scanned"}
                    or not isinstance(names, list) or len(names) > 1024 or type(truncated) is not bool
                    or (truncated and len(names) != 1024) or (status != "read" and (names or truncated))
                    or not all(isinstance(n, str) and 0 < len(n) <= 255 and n not in {".", ".."}
                               and not any(separator in n for separator in "/\\:\0") for n in names)):
                raise ValueError("invalid startup folder display data")
            if len(set(names)) != len(names):
                raise ValueError("duplicate startup folder name")
            scopes.add(key)
            incomplete |= truncated or status in {"unavailable", "location-not-scanned"}
    return {"status": "incomplete" if incomplete else "ready", **report}


def installed_programs(path, expected_sha):
    binary = executable(path, expected_sha)
    run = subprocess.run([str(binary), "--installed-programs"], capture_output=True,
                         check=True, shell=False, timeout=12)
    if len(run.stdout) > 16 * 1024 * 1024:
        raise ValueError("native response is too large")
    report = json.loads(run.stdout.decode("utf-8"), object_pairs_hook=unique)
    required = {"schema", "scope", "readOnly", "commandsCollected", "rows", "skipped", "errors", "capped"}
    if (not isinstance(report, dict) or set(report) != required
            or type(report["schema"]) is not int or report["schema"] != 1
            or report["scope"] != "windows-desktop-registrations"
            or report["readOnly"] is not True or report["commandsCollected"] is not False
            or not isinstance(report["rows"], list) or len(report["rows"]) > 3 * 4096
            or type(report["capped"]) is not bool
            or any(type(report[k]) is not int or not 0 <= report[k] <= 3 * 4097 for k in ("skipped", "errors"))):
        raise ValueError("unexpected installed program report")
    scopes = {"Current user", "Machine", "Machine · 32-bit", "Machine · 64-bit"}
    for row in report["rows"]:
        if (not isinstance(row, dict) or set(row) != {"name", "version", "publisher", "scope"}
                or not all(isinstance(v, str) and len(v) <= 385 and "\0" not in v for v in row.values())
                or not row["name"].strip() or row["scope"] not in scopes):
            raise ValueError("invalid registered program display data")
    return {"status": "incomplete" if report["capped"] or report["errors"] else "ready", **report}


def storage_preview(path, expected_sha, folder, days, entry_limit):
    if not folder.is_absolute() or folder.parent == folder or type(days) is not int or not 1 <= days <= 3650 \
            or type(entry_limit) is not int or not 1 <= entry_limit <= 1_000_000:
        raise ValueError("select a bounded absolute local child folder")
    binary = executable(path, expected_sha)
    run = subprocess.run([str(binary), "--storage-preview", str(folder), str(days), str(entry_limit)],
                         capture_output=True, check=True, shell=False, timeout=30)
    if len(run.stdout) > 4096:
        raise ValueError("native response is too large")
    report = json.loads(run.stdout.decode("utf-8"), object_pairs_hook=unique)
    quantities = {"entriesScanned", "filesScanned", "logicalBytes", "olderFiles", "olderLogicalBytes",
                  "specialEntriesSkipped", "readErrors", "depthLimitedDirectories"}
    expected = quantities | {"schemaVersion", "resultType", "observedAt", "minimumAgeDays", "entryLimit",
                             "entryLimitReached", "partial"}
    if (not isinstance(report, dict) or set(report) != expected
            or type(report["schemaVersion"]) is not int or report["schemaVersion"] != 1
            or report["resultType"] != "storage-preview"
            or type(report["minimumAgeDays"]) is not int or report["minimumAgeDays"] != days
            or type(report["entryLimit"]) is not int or report["entryLimit"] != entry_limit
            or not isinstance(report["observedAt"], str)
            or not re.fullmatch(r"[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}(?:\.[0-9]{1,7})?\+00:00",
                                report["observedAt"])
            or any(type(report[key]) is not int or not 0 <= report[key] <= 2**63 - 1 for key in quantities)
            or type(report["entryLimitReached"]) is not bool or type(report["partial"]) is not bool):
        raise ValueError("unexpected native storage result")
    datetime.datetime.fromisoformat(report["observedAt"])
    n = report["entriesScanned"]
    if (n > entry_limit or report["filesScanned"] + report["specialEntriesSkipped"] + report["depthLimitedDirectories"] > n
            or report["olderFiles"] > report["filesScanned"] or report["olderLogicalBytes"] > report["logicalBytes"]
            or (report["filesScanned"] == 0 and report["logicalBytes"] != 0)
            or (report["olderFiles"] == 0 and report["olderLogicalBytes"] != 0)
            or report["readErrors"] > n + 1
            or (report["entryLimitReached"] and n != entry_limit)
            or report["partial"] != bool(report["entryLimitReached"] or report["readErrors"] or report["depthLimitedDirectories"])):
        raise ValueError("inconsistent native storage totals")
    return {"status": "incomplete" if report["partial"] else "ready", **report}


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--json", action="store_true", help="output a machine-readable JSON result (also the default)")
    sub = parser.add_subparsers(dest="operation", required=True)
    power = sub.add_parser("power-plans", help="read only: list Windows power plans and their active selection")
    startup = sub.add_parser("startup-inventory", help="read only: Windows Run and RunOnce names, no commands or enable state")
    programs = sub.add_parser("installed-programs", help="read only: Windows desktop registration display metadata")
    startup.add_argument("--include-folders", action="store_true",
                         help="explicitly include base names in the current-user and all-users local startup folders")
    storage = sub.add_parser("storage-preview", help="read only: aggregate metadata for a selected local folder")
    storage.add_argument("--folder", type=pathlib.Path, required=True)
    storage.add_argument("--minimum-age-days", type=int, default=7)
    storage.add_argument("--entry-limit", type=int, default=100000)
    for operation in (power, startup, programs, storage):
        operation.add_argument("--agent-binary", type=pathlib.Path, required=True)
        operation.add_argument("--agent-sha256", required=True)
    args = parser.parse_args(argv)
    try:
        if args.operation == "storage-preview":
            result = storage_preview(args.agent_binary, args.agent_sha256, args.folder,
                                     args.minimum_age_days, args.entry_limit)
        elif args.operation == "startup-inventory":
            result = startup_inventory(args.agent_binary, args.agent_sha256, args.include_folders)
        else:
            read = {"power-plans": inventory, "installed-programs": installed_programs}[args.operation]
            result = read(args.agent_binary, args.agent_sha256)
    except (OSError, ValueError, TypeError, RecursionError, subprocess.SubprocessError):
        print(json.dumps({"operation": args.operation, "status": "refused"}))
        return 2
    print(json.dumps({"operation": args.operation, **result}, ensure_ascii=True))
    return 0 if result["status"] == "ready" else 1


if __name__ == "__main__":
    raise SystemExit(main())
