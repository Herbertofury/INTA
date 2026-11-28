"""One-shot, JSON CLI harness for the reviewed native INTA Local Agent inventory."""
import argparse
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


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--json", action="store_true", help="output a machine-readable JSON result (also the default)")
    sub = parser.add_subparsers(dest="operation", required=True)
    power = sub.add_parser("power-plans", help="read only: list Windows power plans and their active selection")
    power.add_argument("--agent-binary", type=pathlib.Path, required=True)
    power.add_argument("--agent-sha256", required=True)
    args = parser.parse_args(argv)
    try:
        result = inventory(args.agent_binary, args.agent_sha256)
    except (OSError, ValueError, TypeError, RecursionError, subprocess.SubprocessError):
        print(json.dumps({"operation": args.operation, "status": "refused"}))
        return 2
    print(json.dumps({"operation": args.operation, **result}, ensure_ascii=True))
    return 0 if result["status"] == "ready" else 1


if __name__ == "__main__":
    raise SystemExit(main())
