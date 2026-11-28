"""One-shot, JSON CLI harness for the reviewed native INTA Local Agent inventory."""
import argparse
import datetime
import hashlib
import json
import os
import pathlib
import re
import subprocess
import stat
import sys
import uuid

MAX_BACKEND = 180 * 1024 * 1024
STATES = {"ready", "incomplete", "unsupported", "unavailable", "unstable"}


def executable(path, expected_sha, names=frozenset({"inta-agent", "inta-agent.exe"})):
    if (not path.is_absolute() or path.name not in names
            or path.is_symlink() or not path.is_file() or not re.fullmatch(r"[a-f0-9]{64}", expected_sha)):
        raise ValueError("select the absolute path to an already verified executable")
    remaining = MAX_BACKEND
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        while chunk := stream.read(min(1024 * 1024, remaining + 1)):
            remaining -= len(chunk)
            if remaining < 0:
                raise ValueError("executable exceeds configured size bound")
            digest.update(chunk)
    if digest.hexdigest() != expected_sha:
        raise ValueError("executable does not match the approved release pin")
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


def sleep_states(path, expected_sha):
    """Read a bounded native ACPI S1–S4 capability snapshot, not a policy action."""
    binary = executable(path, expected_sha)
    run = subprocess.run([str(binary), "--sleep-states"], capture_output=True, check=True,
                         shell=False, timeout=12, stdin=subprocess.DEVNULL)
    if len(run.stdout) > 4096:
        raise ValueError("native response is too large")
    report = json.loads(run.stdout.decode("utf-8"), object_pairs_hook=unique)
    fields = {"standbyS1", "standbyS2", "standbyS3", "hibernateS4", "hibernationFilePresent"}
    if (not isinstance(report, dict)
            or set(report) != {"schema", "status", "readOnly", "source", "states"}
            or type(report["schema"]) is not int or report["schema"] != 1
            or report["readOnly"] is not True or report["source"] != "system-power-capabilities-prefix"
            or not isinstance(report["status"], str) or report["status"] not in {"ready", "unsupported", "unavailable"}):
        raise ValueError("unexpected sleep capability response")
    states = report["states"]
    if report["status"] == "ready":
        if not isinstance(states, dict) or set(states) != fields or any(type(value) is not bool for value in states.values()):
            raise ValueError("invalid sleep capability flags")
    elif states is not None:
        raise ValueError("non-ready capability response must not contain flags")
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


MAX_LOCAL_HASH = 32 * 1024 * 1024


def open_windows_readonly(path):
    import ctypes
    from ctypes import wintypes
    import msvcrt
    api = ctypes.WinDLL("kernel32", use_last_error=True)
    open_file = api.CreateFileW
    open_file.argtypes = [wintypes.LPCWSTR, wintypes.DWORD, wintypes.DWORD, wintypes.LPVOID,
                          wintypes.DWORD, wintypes.DWORD, wintypes.HANDLE]
    open_file.restype = wintypes.HANDLE
    query = api.GetFileInformationByHandleEx
    query.argtypes = [wintypes.HANDLE, ctypes.c_int, wintypes.LPVOID, wintypes.DWORD]
    query.restype = wintypes.BOOL
    api.CloseHandle.argtypes = [wintypes.HANDLE]
    api.CloseHandle.restype = wintypes.BOOL
    # GENERIC_READ, FILE_SHARE_READ, OPEN_EXISTING, FILE_FLAG_OPEN_REPARSE_POINT.
    # Refuse concurrent writers/deletion and do not traverse a final reparse point.
    handle = open_file(str(path), 0x80000000, 1, None, 3, 0x00200000, None)
    if handle == ctypes.c_void_p(-1).value:
        raise ctypes.WinError(ctypes.get_last_error())
    try:
        attributes = (wintypes.DWORD * 2)()
        if not query(handle, 9, ctypes.byref(attributes), ctypes.sizeof(attributes)):  # FileAttributeTagInfo
            raise ctypes.WinError(ctypes.get_last_error())
        if attributes[0] & (0x10 | 0x40 | 0x400):  # directory, device or reparse point
            raise ValueError("selected Windows handle is not an ordinary file")
        fd = msvcrt.open_osfhandle(handle, os.O_RDONLY | os.O_BINARY)
    except BaseException:
        api.CloseHandle(handle)
        raise
    return fd


def regular_snapshot(path):
    if not path.is_absolute() or path.parent == path or ".." in path.parts:
        raise ValueError("choose an absolute file path on a local volume")
    if os.name == "nt":
        if re.fullmatch(r"[a-zA-Z]:", path.drive) is None or any(":" in p for p in path.parts[1:]):
            raise ValueError("Windows network paths, device paths and streams are out of scope")
        import ctypes
        drive_type = ctypes.windll.kernel32.GetDriveTypeW
        drive_type.argtypes, drive_type.restype = [ctypes.c_wchar_p], ctypes.c_uint
        if drive_type(path.anchor) not in {2, 3, 5, 6}:
            raise ValueError("choose a file on a known local Windows volume")
    for directory in reversed(path.parents):
        parent = directory.lstat()
        if not stat.S_ISDIR(parent.st_mode) or getattr(parent, "st_file_attributes", 0) & 0x400:
            raise ValueError("symlink or reparse folder is out of scope")
    item = path.lstat()
    if not stat.S_ISREG(item.st_mode) or getattr(item, "st_file_attributes", 0) & 0x400:
        raise ValueError("choose a regular file, not a symlink or reparse point")
    if item.st_size > MAX_LOCAL_HASH:
        raise ValueError("file exceeds the local hash preview budget")
    # O_NONBLOCK keeps a racing FIFO from blocking a POSIX opener; the opened
    # handle must still be a regular file and match the original identity.
    flags = os.O_RDONLY | getattr(os, "O_BINARY", 0) | getattr(os, "O_NOFOLLOW", 0) | getattr(os, "O_NONBLOCK", 0)
    fd = open_windows_readonly(path) if os.name == "nt" else os.open(path, flags)
    with os.fdopen(fd, "rb") as stream:
        opened = os.fstat(stream.fileno())
        snapshot_key = lambda st: (st.st_dev, st.st_ino, st.st_size, st.st_mtime_ns, st.st_ctime_ns)
        # Windows path and descriptor metadata use different system APIs;
        # compare only within each source, and bind the actual read to a
        # no-follow handle that refuses concurrent writer/delete sharing.
        path_matches = (snapshot_key(path.lstat()) == snapshot_key(item) and opened.st_size == item.st_size
                        if os.name == "nt" else snapshot_key(opened) == snapshot_key(item))
        if not stat.S_ISREG(opened.st_mode) or not path_matches:
            raise ValueError("file changed before it was opened")
        data = stream.read(MAX_LOCAL_HASH + 1)
        if (len(data) != item.st_size or snapshot_key(os.fstat(stream.fileno())) != snapshot_key(opened)
                or (os.name == "nt" and snapshot_key(path.lstat()) != snapshot_key(item))):
            raise ValueError("file changed while it was read")
    return data


def rclone_local_sha256(binary_path, expected_pin, file_path, expected_digest=None):
    if expected_digest is not None and not re.fullmatch(r"[a-f0-9]{64}", expected_digest):
        raise ValueError("invalid expected digest")
    binary = executable(binary_path, expected_pin, frozenset({"rclone", "rclone.exe"}))
    data = regular_snapshot(file_path)
    # No name/path/remote arguments and no inherited rclone/proxy settings.
    # stdin is the only backend; the OS null config avoids loading user accounts.
    env = {k: os.environ[k] for k in ("SystemRoot", "WINDIR") if k in os.environ}
    cmd = [str(binary), "--config=", "hashsum", "sha256", "-"]
    run = subprocess.run(cmd, input=data, capture_output=True, check=True, shell=False,
                         timeout=30, cwd=binary.parent, env=env)
    match = re.fullmatch(rb"([a-f0-9]{64})[ \t]+-?\r?\n", run.stdout)
    if not match:
        raise ValueError("unexpected rclone output")
    digest = match[1].decode("ascii")
    return {"status": "ready" if expected_digest in {None,digest} else "mismatch",
            "scope": "rclone-local-stdin", "sha256": digest, "bytes": len(data), "readOnly": True,
            "matchesExpected": None if expected_digest is None else expected_digest == digest}


def windows_system_directory():
    from ctypes import WinDLL, c_uint, c_wchar_p, create_unicode_buffer
    result = create_unicode_buffer(32768)
    locate = WinDLL("kernel32", use_last_error=True).GetSystemDirectoryW
    locate.argtypes = [c_wchar_p, c_uint]
    locate.restype = c_uint
    size = locate(result, len(result))
    if not 1 <= size < len(result):
        raise ValueError("system tool location unavailable")
    return pathlib.Path(result.value)


def windows_compare_directory(path):
    from ctypes import WinDLL, c_uint, c_wchar_p
    if (not path.is_absolute() or not re.fullmatch(r"[A-Za-z]:", path.drive)
            or not path.name or path.is_reserved()
            or any(p.endswith((".", " ")) or any(ch in p for ch in ':*?"<>|') for p in path.parts[1:])):
        raise ValueError("choose an existing ordinary drive-local folder, not a drive root")
    drive_type = WinDLL("kernel32", use_last_error=True).GetDriveTypeW
    drive_type.argtypes = [c_wchar_p]
    drive_type.restype = c_uint
    if drive_type(path.anchor) != 3:
        raise ValueError("comparison supports fixed local drives only")
    # Evaluate every ancestor without following an NTFS junction or mount point.
    for part in [*reversed(path.parents), path]:
        info = part.lstat()
        if not stat.S_ISDIR(info.st_mode) or info.st_file_attributes & stat.FILE_ATTRIBUTE_REPARSE_POINT:
            raise ValueError("reparse points or non-directories are not accepted")
    return path


def robocopy_metadata_preview(source, destination, expected_sha):
    if os.name != "nt":
        return {"status": "unsupported", "writesRequested": False, "recursive": False}
    source, destination = (windows_compare_directory(p) for p in (source, destination))
    if source == destination or source in destination.parents or destination in source.parents:
        raise ValueError("choose separate existing folders")
    binary = executable(windows_system_directory() / "robocopy.exe", expected_sha, frozenset({"robocopy.exe"}))
    # /L asks the installed tool to list only. No copy, mirror, exclude-lists
    # supplied by users, inherited jobs, recursive scope or output files.
    run = subprocess.run([str(binary), str(source), str(destination), "*",
                          "/L", "/XJ", "/R:0", "/W:0", "/NP", "/NJH", "/NJS", "/NFL", "/NDL"],
                         stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
                         shell=False, timeout=15)
    code = run.returncode
    if not 0 <= code <= 7:
        return {"status": "unavailable", "writesRequested": False, "recursive": False}
    return {"status": "ready", "writesRequested": False, "recursive": False,
            "exitCode": code, "copyWouldBeConsidered": bool(code & 1),
            "extrasReported": bool(code & 2), "mismatchReported": bool(code & 4),
            "contentCompared": False}


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--json", action="store_true", help="output a machine-readable JSON result (also the default)")
    sub = parser.add_subparsers(dest="operation", required=True)
    power = sub.add_parser("power-plans", help="read only: list Windows power plans and their active selection")
    sleep = sub.add_parser("sleep-states", help="read only: documented legacy Windows ACPI sleep capability flags")
    startup = sub.add_parser("startup-inventory", help="read only: Windows Run and RunOnce names, no commands or enable state")
    programs = sub.add_parser("installed-programs", help="read only: Windows desktop registration display metadata")
    startup.add_argument("--include-folders", action="store_true",
                         help="explicitly include base names in the current-user and all-users local startup folders")
    storage = sub.add_parser("storage-preview", help="read only: aggregate metadata for a selected local folder")
    storage.add_argument("--folder", type=pathlib.Path, required=True)
    storage.add_argument("--minimum-age-days", type=int, default=7)
    storage.add_argument("--entry-limit", type=int, default=100000)
    for operation in (power, sleep, startup, programs, storage):
        operation.add_argument("--agent-binary", type=pathlib.Path, required=True)
        operation.add_argument("--agent-sha256", required=True)
    rclone = sub.add_parser("rclone-sha256", help="read only: use pinned rclone to hash a single local file")
    rclone.add_argument("--rclone-binary", type=pathlib.Path, required=True)
    rclone.add_argument("--rclone-sha256", required=True)
    rclone.add_argument("--file", type=pathlib.Path, required=True)
    rclone.add_argument("--expect-sha256")
    robocopy = sub.add_parser("robocopy-preview", help="read only: compare immediate folder entries with the pinned Windows tool")
    robocopy.add_argument("--source", type=pathlib.Path, required=True)
    robocopy.add_argument("--destination", type=pathlib.Path, required=True)
    robocopy.add_argument("--robocopy-sha256", required=True)
    args = parser.parse_args(argv)
    try:
        if args.operation == "robocopy-preview":
            result = robocopy_metadata_preview(args.source, args.destination, args.robocopy_sha256)
        elif args.operation == "rclone-sha256":
            result = rclone_local_sha256(args.rclone_binary, args.rclone_sha256, args.file, args.expect_sha256)
        elif args.operation == "storage-preview":
            result = storage_preview(args.agent_binary, args.agent_sha256, args.folder,
                                     args.minimum_age_days, args.entry_limit)
        elif args.operation == "startup-inventory":
            result = startup_inventory(args.agent_binary, args.agent_sha256, args.include_folders)
        else:
            read = {"power-plans": inventory, "sleep-states": sleep_states, "installed-programs": installed_programs}[args.operation]
            result = read(args.agent_binary, args.agent_sha256)
    except (OSError, ValueError, TypeError, RecursionError, subprocess.SubprocessError):
        print(json.dumps({"operation": args.operation, "status": "refused"}))
        return 2
    print(json.dumps({"operation": args.operation, **result}, ensure_ascii=True))
    return 0 if result["status"] == "ready" else 1


if __name__ == "__main__":
    raise SystemExit(main())
