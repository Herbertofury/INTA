"""Prepare and validate a local, relocatable INTA toolbox. Python 3.11+."""

from __future__ import annotations

import argparse
import contextlib
import datetime as dt
import hashlib
import html
import json
import os
import platform
import shutil
import stat
import subprocess
import sys
import tempfile
import urllib.request
import uuid
import zipfile
from pathlib import Path, PureWindowsPath
from urllib.parse import urlsplit

HERE = Path(__file__).resolve().parent
CACHE = "93 - Offline Package Cache"
SRC = "90 - Upstream Source Repositories"
START = "00 - START HERE"
MARKER = ".inta-toolbox.json"
INDEX = ".inta-packages.json"
PINS = json.loads((HERE / "pins.json").read_text(encoding="utf-8"))
HOSTS = {
    "github.com",
    "api.github.com",
    "release-assets.githubusercontent.com",
    "objects.githubusercontent.com",
    "restic.net",
}
MAX_BINARY = 256 * 1024 * 1024
MANGO = {
    "id": "MangoDisk",
    "repo": "harry0703/MangoDisk",
    "tag": "v2.1.1",
    "commit": "d2a4fd6597524eb77b0687cfe1c9e6f533fae7c9",
}


class Failure(ValueError):
    pass


def date():
    return dt.datetime.now(dt.UTC).isoformat(timespec="seconds")


def digest(path):
    with path.open("rb") as f:
        return hashlib.file_digest(f, "sha256").hexdigest()


def safe_root(path):
    path = Path(path).absolute()
    if os.name == "nt" and (len(path.drive) != 2 or path.drive[1] != ":"):
        raise Failure("Select a local drive path")
    for item in [*reversed(path.parents), path]:
        if item.name == "..":
            raise Failure("Use a normalized root path")
        check_link(item)
    return path


def check_link(path):
    try:
        s = path.lstat()
    except FileNotFoundError:
        return
    if stat.S_ISLNK(s.st_mode) or getattr(s, "st_file_attributes", 0) & 0x400:
        raise Failure(
            f"Link or reparse point is outside the package policy: {path.name}"
        )


def within(root, rel):
    if not isinstance(rel, str) or not rel:
        raise Failure("Missing relative path")
    path = root
    for part in rel.replace("\\", "/").split("/"):
        if (
            part in ("", ".", "..")
            or ":" in part
            or part.endswith((" ", "."))
            or (
                os.path.isreserved(part)
                if hasattr(os.path, "isreserved")
                else PureWindowsPath(part).is_reserved()
            )
        ):
            raise Failure(f"Invalid relative path: {rel!r}")
        path = path / part
        check_link(path)
    return path


def data(path):
    if path.stat().st_size > 128 * 1024:
        raise Failure(f"Oversized metadata: {path.name}")
    return json.loads(path.read_text(encoding="utf-8"))


def write(path, content):
    if not isinstance(content, bytes):
        content = content.encode("utf-8")
    path.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.NamedTemporaryFile(dir=path.parent, delete=False) as f:
        temp = Path(f.name)
        try:
            f.write(content)
            f.flush()
            os.fsync(f.fileno())
        except BaseException:
            f.close()
            temp.unlink(missing_ok=True)
            raise
    try:
        temp.replace(path)
    finally:
        temp.unlink(missing_ok=True)


def dump(path, obj):
    write(path, json.dumps(obj, indent=2, ensure_ascii=False) + "\n")


def identity(root):
    m = data(within(root, MARKER))
    if m.get("schemaVersion") != 1 or str(uuid.UUID(m["toolboxId"])) != m["toolboxId"]:
        raise Failure("Unsupported or malformed toolbox marker")
    return m


def init(root):
    if root.exists() and any(root.iterdir()) and not (root / MARKER).is_file():
        raise Failure("Initialize an empty destination or select a marked toolbox")
    root.mkdir(parents=True, exist_ok=True)
    if not (root / MARKER).exists():
        with within(root, MARKER).open("x", encoding="utf-8") as f:
            json.dump(
                {
                    "schemaVersion": 1,
                    "toolboxId": str(uuid.uuid4()),
                    "displayName": "INTA Toolbox",
                    "layoutProfile": "technician-v1",
                    "createdAt": date(),
                    "compatibleSchema": [1],
                },
                f,
                indent=2,
            )
    m = identity(root)
    for rel in json.loads((HERE / "layout.json").read_text(encoding="utf-8")):
        within(root, rel).mkdir(exist_ok=True, parents=True)
    if not within(root, INDEX).exists():
        dump(
            root / INDEX,
            {"schemaVersion": 1, "toolboxId": m["toolboxId"], "packages": {}},
        )
    return m


def index(root):
    idx = data(within(root, INDEX))
    if (
        idx["schemaVersion"] != 1
        or idx["toolboxId"] != identity(root)["toolboxId"]
        or not isinstance(idx["packages"], dict)
    ):
        raise Failure("Index has a different toolbox identity or invalid format")
    return idx


@contextlib.contextmanager
def locked(root):
    with within(root, ".inta-maintenance.lock").open("a+b") as f:
        if not f.tell():
            f.write(b"\0")
            f.flush()
        f.seek(0)
        if os.name == "nt":
            import msvcrt

            msvcrt.locking(f.fileno(), msvcrt.LK_NBLCK, 1)
        else:
            import fcntl

            fcntl.flock(f, fcntl.LOCK_EX | fcntl.LOCK_NB)
        yield


def validate_url(url):
    u = urlsplit(url)
    if (
        u.scheme != "https"
        or u.hostname not in HOSTS
        or u.port not in (None, 443)
        or u.username
        or u.password
    ):
        raise Failure("Only pinned upstream HTTPS origins are permitted")


class SafeRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        validate_url(newurl)
        return super().redirect_request(req, fp, code, msg, headers, newurl)


def fetch(url, f, limit):
    validate_url(url)
    req = urllib.request.Request(url, headers={"User-Agent": "INTA-Toolbox/1"})
    total = 0
    with urllib.request.build_opener(SafeRedirect).open(req, timeout=45) as resp:
        while chunk := resp.read(1024 * 1024):
            total += len(chunk)
            if total > limit:
                raise Failure("Download exceeded pinned size")
            f.write(chunk)
    return total


def url(pin):
    return f"https://github.com/{pin['repo']}/releases/download/{pin['tag']}/{pin['asset']}"


def cache(root, pin, offline):
    output = within(root, CACHE + "/" + pin["assetSha256"])
    if not output.exists():
        if offline:
            raise Failure("Pinned bytes are absent from the offline cache")
        with tempfile.TemporaryDirectory(dir=output.parent) as temp:
            part = Path(temp) / "artifact"
            with part.open("xb") as f:
                actual = fetch(url(pin), f, pin["assetBytes"])
            if actual != pin["assetBytes"] or digest(part) != pin["assetSha256"]:
                raise Failure("Release digest mismatch")
            part.rename(output)
    if (
        output.stat().st_size != pin["assetBytes"]
        or digest(output) != pin["assetSha256"]
    ):
        raise Failure("Cache integrity failed; existing bytes preserved")
    return output


def source(root, pin, offline):
    repo = within(root, SRC + "/" + pin["id"])
    if not repo.exists():
        if offline:
            raise Failure("Pinned source is absent")
        with tempfile.TemporaryDirectory(dir=repo.parent) as temp:
            stage = Path(temp) / "checkout"
            subprocess.run(
                [
                    "git",
                    "-c",
                    "core.autocrlf=false",
                    "clone",
                    "--branch",
                    pin["tag"],
                    "--",
                    "https://github.com/" + pin["repo"] + ".git",
                    str(stage),
                ],
                check=True,
                timeout=600,
            )
            if git(stage, "rev-parse", "HEAD").decode().strip() != pin["commit"]:
                raise Failure("Tagged source commit differs from the reviewed pin")
            stage.rename(repo)
    if (
        git(repo, "remote", "get-url", "origin").decode().strip().removesuffix(".git")
        != "https://github.com/" + pin["repo"]
    ):
        raise Failure("Existing source checkout has a different upstream")
    if git(repo, "rev-parse", "HEAD").decode().strip() != pin["commit"]:
        raise Failure(
            "Pinned source differs; preserve local checkout and review changes"
        )
    return repo


def git(repo, *args):
    return subprocess.run(
        ["git", "-C", str(repo), *args], check=True, capture_output=True, timeout=60
    ).stdout


def unpack(artifact, pin, destination):
    member = pin["member"]
    with destination.open("xb") as f:
        if member is None:
            with artifact.open("rb") as src:
                shutil.copyfileobj(src, f)
        else:
            with zipfile.ZipFile(artifact) as archive:
                entries = [
                    entry for entry in archive.infolist() if entry.filename == member
                ]
                if (
                    len(entries) != 1
                    or entries[0].file_size > MAX_BINARY
                    or entries[0].is_dir()
                ):
                    raise Failure("Required archive member failed validation")
                # Fixed output name. No archive paths are passed to extraction APIs.
                with archive.open(entries[0]) as src:
                    shutil.copyfileobj(src, f)
    if digest(destination) != pin["exeSha256"]:
        raise Failure("Executable digest differs from the reviewed build")


def fixture(pin, exe, work):
    if os.name != "nt" or platform.machine().lower() not in ("amd64", "x86_64"):
        raise Failure("Fixture execution requires x64 Windows")
    text = b"INTA sandbox fixture\n"
    payload = work / "sample.txt"
    payload.write_bytes(text)
    env = {
        "SystemRoot": os.environ["SystemRoot"],
        "PATH": os.path.join(os.environ["SystemRoot"], "System32"),
        "TEMP": str(work),
        "TMP": str(work),
        "USERPROFILE": str(work),
        "LOCALAPPDATA": str(work),
    }
    evidence = []

    def run(args, expected=None):
        result = subprocess.run(
            [str(exe), *args],
            cwd=work,
            env=env,
            check=False,
            stdin=subprocess.DEVNULL,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            timeout=90,
        )
        out = result.stdout.decode("utf-8", errors="replace")
        if result.returncode != 0 or (expected is not None and expected not in out):
            raise Failure(
                f"{pin['id']} fixture failed at {args[0]} (code {result.returncode}): {out[-400:]}"
            )
        evidence.append({"operation": args[0], "exitCode": result.returncode})
        return out

    version = run(pin["probe"], pin["version"]).splitlines()
    if pin["id"] == "7zip":
        run(["a", "example.7z", "sample.txt"])
        run(["t", "example.7z"], "Everything is Ok")
        run(["x", "-orestored", "example.7z"])
        restored = (work / "restored/sample.txt").read_bytes()
    elif pin["id"] == "restic":
        env["RESTIC_PASSWORD"] = str(uuid.uuid4())
        env["RESTIC_REPOSITORY"] = str(work / "repo")
        env["RESTIC_CACHE_DIR"] = str(work / "cache")
        run(["init"])
        run(["backup", "sample.txt"])
        run(["check", "--read-data"])
        run(["restore", "latest", "--target", "restored"])
        restored = next((work / "restored").rglob("sample.txt")).read_bytes()
    elif pin["id"] == "rclone":
        config = work / "empty.conf"
        config.write_text("", encoding="utf-8")
        input_dir = work / "input"
        input_dir.mkdir()
        payload.rename(input_dir / "sample.txt")
        opts = ["--config", str(config)]
        run(["copy", "input", "copied", *opts])
        run(["check", "--download", "input", "copied", *opts])
        restored = (work / "copied/sample.txt").read_bytes()
    elif pin["id"] == "yara-x":
        (work / "sample.yar").write_text(
            'rule INTA_fixture { strings: $s = "INTA sandbox fixture" condition: $s }',
            encoding="ascii",
        )
        run(["scan", "sample.yar", "sample.txt"], "INTA_fixture")
        (work / "negative.txt").write_text("ordinary text", encoding="ascii")
        if "INTA_fixture" in run(["scan", "sample.yar", "negative.txt"]):
            raise Failure("Unexpected fixture match")
        restored = payload.read_bytes()
    else:
        raise Failure("No approved fixture defined")
    if restored != text:
        raise Failure("Fixture content did not survive the operation")
    return {
        "testedAt": date(),
        "host": platform.platform(),
        "mode": "temporary local fixture",
        "versionOutput": next(line for line in version if pin["version"] in line),
        "operations": evidence,
        "outcome": "passed",
    }


def activate(root, pin, offline, current):
    artifact = cache(root, pin, offline)
    checkout = source(root, pin, offline)
    name = pin["id"]
    relative = (
        current
        or f"{pin['folder']}/{name}/{pin['version']}/PORTABLE/{pin['executable']}"
    )
    destination = within(root, relative)
    if destination.name != pin["executable"] or destination.parent.name != "PORTABLE":
        raise Failure(
            "Reindexed packages must retain the PORTABLE executable structure"
        )
    release = destination.parent.parent
    if release.exists():
        if digest(destination) != pin["exeSha256"]:
            raise Failure(
                "Existing package failed verification; preserved for inspection"
            )
        record = data(
            within(root, (release.relative_to(root) / "tool.json").as_posix())
        )
        if (
            record.get("id") != name
            or record.get("exeSha256") != pin["exeSha256"]
            or record.get("version") != pin["version"]
        ):
            raise Failure("Existing package receipt differs from the reviewed pin")
        return relative
    release.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix=".inta-stage-", dir=release.parent) as temp:
        stage = Path(temp) / "version"
        binary = stage / "PORTABLE" / pin["executable"]
        binary.parent.mkdir(parents=True)
        unpack(artifact, pin, binary)
        notice = git(checkout, "cat-file", "blob", pin["commit"] + ":" + pin["notice"])
        write(stage / "LICENSES" / Path(pin["notice"]).name, notice)
        if name == "7zip":
            write(
                stage / "LICENSES/unRarLicense.txt",
                git(
                    checkout,
                    "cat-file",
                    "blob",
                    pin["commit"] + ":DOC/unRarLicense.txt",
                ),
            )
        work = Path(temp) / "fixture"
        work.mkdir()
        result = fixture(pin, binary, work)
        dump(stage / "TEST-RESULTS" / "fixture.json", result)
        receipt = dict(
            pin,
            schemaVersion=1,
            url=url(pin),
            acquiredAt=date(),
            testedAt=result["testedAt"],
            integrity="Pinned GitHub release SHA-256 and pinned extracted executable SHA-256",
            publisherSignature="See the separate publisher attestation evidence when available",
            licensePolicy="Private local acquisition. Preserve upstream notices and resolve redistribution terms before packaging.",
            launchMode="Portable fixed-argument probe via INTA",
            privileges="Standard user",
            reboot=False,
            supportedTestHost=result["host"],
            sourcePath=f"{SRC}/{name}",
        )
        dump(stage / "tool.json", receipt)
        write(
            stage / "README.md",
            f"# {pin['name']} {pin['version']}\n\n{pin['rationale']}\n\nRelease, license, source revision, tested operations and executable hashes are recorded in tool.json, LICENSES and TEST-RESULTS.\n",
        )
        stage.rename(release)
    return relative


def reindex(root):
    idx = index(root)
    packages = {}
    pins = {x["id"]: x for x in PINS}
    budget = 20000

    def scan_error(exc):
        raise exc

    for here, dirs, files in os.walk(root, onerror=scan_error):
        here = Path(here)
        if here == root:
            dirs[:] = [
                d
                for d in dirs
                if d not in {SRC, CACHE, "99 - Logs, Reports & Quarantine"}
            ]
        dirs[:] = [d for d in dirs if not d.startswith(".inta-stage-")]
        budget -= len(dirs) + len(files)
        if budget < 0:
            raise Failure("Reindex limit reached. Select a smaller managed root")
        for d in dirs:
            check_link(here / d)
        if "tool.json" not in files:
            continue
        receipt = within(root, (here.relative_to(root) / "tool.json").as_posix())
        p = data(receipt)
        pin = pins.get(p.get("id"))
        if not pin or p.get("version") != pin["version"]:
            continue
        rel = (here.relative_to(root) / "PORTABLE" / pin["executable"]).as_posix()
        exe = within(root, rel)
        if digest(exe) != pin["exeSha256"]:
            raise Failure("Reindex failed executable integrity")
        if pin["id"] in packages:
            raise Failure(
                "Two copies of an active package found; choose one before reindexing"
            )
        packages[pin["id"]] = rel
    idx["packages"] = packages
    dump(within(root, INDEX), idx)
    return idx


def offline_guide(root):
    from urllib.parse import quote

    site = HERE.parent / "site"
    manifest = data(site / "build-manifest.json")
    expected = manifest["files"]["site/index.html"]
    content = (site / "index.html").read_bytes()
    if hashlib.sha256(content).hexdigest() != expected:
        raise Failure("Offline workspace differs from its source build manifest")
    name = f"Technician Guide/INTA workspace {expected[:16]}.html"
    target = within(root, START + "/" + name)
    if target.exists() and digest(target) != expected:
        raise Failure("Existing versioned offline guide differs from its recorded hash")
    if not target.exists():
        write(target, content)
    return f'<p><a href="{quote(name)}">Offline technician workspace</a> · {manifest["tools"]} tools, {manifest["jobs"]} playbooks, {manifest["methods"]} methods</p>'


def render(root):
    idx = index(root)
    guide = offline_guide(root)
    items = []
    summary = []
    for p in PINS:
        rel = idx["packages"].get(p["id"])
        if not rel:
            continue
        exe = within(root, rel)
        receipt = (exe.parent.parent / "tool.json").relative_to(root).as_posix()
        from urllib.parse import quote

        items.append(
            f'<li><h2>{html.escape(p["name"])} {html.escape(p["version"])}</h2><p>{html.escape(p["rationale"])}</p><p><a href="../{quote(receipt)}">Package manifest and provenance</a> · <a href="{html.escape(p["homepage"])}">Official documentation</a></p></li>'
        )
        summary.append(
            {
                "id": p["id"],
                "version": p["version"],
                "source": p["repo"],
                "commit": p["commit"],
                "license": p["license"],
                "sha256": p["exeSha256"],
                "path": rel,
            }
        )
    write(
        within(root, f"{START}/INTA Toolbox Index.html"),
        '<!doctype html><html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>INTA Toolbox</title><style>:root{color-scheme:dark light}body{max-width:54rem;margin:3rem auto;padding:0 1.5rem;font:1rem/1.55 system-ui}li{padding:.5rem 0;border-top:1px solid #8885}ul{list-style:none;padding:0}a{color:LinkText}</style><h1>INTA Toolbox</h1><p>Locally acquired tools. Verify a package in INTA before use.</p>'
        + guide
        + "<ul>"
        + "".join(items)
        + "</ul></html>",
    )
    dump(
        within(root, "91 - Licenses, Notices & Provenance/acquired-packages.json"),
        summary,
    )
    write(
        within(root, START + "/Quick Start.md"),
        "# INTA Toolbox\n\nOpen INTA Toolbox Index.html to browse acquired packages. Use the desktop Toolbox view to verify integrity and run version probes. Keep case evidence elsewhere. After moving a category or package folder, run the reindex command described in the development repository's toolbox/README.md.\n",
    )
    return summary


def check_updates(pins):
    import io

    result = []
    for pin in pins:
        buf = io.BytesIO()
        fetch(
            "https://api.github.com/repos/" + pin["repo"] + "/releases/latest",
            buf,
            2 * 1024 * 1024,
        )
        candidate = json.loads(buf.getvalue())["tag_name"]
        result.append(
            {
                "id": pin["id"],
                "installedPin": pin["tag"],
                "upstreamTag": candidate,
                "reviewRequired": candidate != pin["tag"],
            }
        )
    return result


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument(
        "action",
        choices=[
            "init",
            "acquire",
            "reindex",
            "verify",
            "check-updates",
            "index",
            "reference",
        ],
    )
    p.add_argument("--root", type=Path, required=True)
    p.add_argument("--tool", choices=[x["id"] for x in PINS], action="append")
    p.add_argument("--offline", action="store_true")
    args = p.parse_args()
    root = safe_root(args.root)
    picks = [x for x in PINS if not args.tool or x["id"] in args.tool]
    if args.action == "init":
        result = init(root)
    elif args.action == "reference":
        identity(root)
        with locked(root):
            checkout = source(root, MANGO, args.offline)
            result = dict(
                MANGO,
                sourcePath=checkout.relative_to(root).as_posix(),
                state="source-reference",
                license="GPL-3.0-only",
                runtimeIntegration=False,
            )
            dump(
                within(
                    root, "91 - Licenses, Notices & Provenance/mangodisk-reference.json"
                ),
                result,
            )
    elif args.action == "check-updates":
        if args.offline:
            raise Failure("Update checks require the upstream connection")
        result = check_updates(picks)
    elif args.action == "verify":
        idx = index(root)
        for pin in picks:
            if digest(within(root, idx["packages"][pin["id"]])) != pin["exeSha256"]:
                raise Failure(f"{pin['id']} failed SHA-256")
        result = [x["id"] for x in picks]
    else:
        identity(root)
        with locked(root):
            if args.action == "acquire":
                idx = index(root)
                for pin in picks:
                    current = idx["packages"].get(pin["id"])
                    # New pins always stage a distinct version. Keep previous bytes available.
                    if current:
                        previous = within(root, current).parent.parent
                        previous_receipt = within(
                            root, (previous.relative_to(root) / "tool.json").as_posix()
                        )
                        if data(previous_receipt)["version"] != pin["version"]:
                            current = None
                    idx["packages"][pin["id"]] = activate(
                        root, pin, args.offline, current
                    )
                dump(within(root, INDEX), idx)
            elif args.action == "reindex":
                reindex(root)
            result = render(root)
    print(json.dumps(result, indent=2, ensure_ascii=True))


if __name__ == "__main__":
    try:
        main()
    except (
        OSError,
        ValueError,
        KeyError,
        subprocess.SubprocessError,
        zipfile.BadZipFile,
    ) as exc:
        print(str(exc), file=sys.stderr)
        sys.exit(1)
