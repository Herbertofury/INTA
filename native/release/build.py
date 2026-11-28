"""Build and validate the Windows x64 Desktop payload from this checkout."""

import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import tomllib
import zipfile

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
TARGET = ROOT / "target"
WORKBENCH = ROOT / "native/workbench"
POWERSHELL = (
    Path(os.environ["SystemRoot"]) / "System32/WindowsPowerShell/v1.0/powershell.exe"
)
SOURCES = [
    "native",
    "desktop",
    "agent",
    "data",
    "docs",
    "site",
    "wiki",
    "media",
    "assets",
]
CRATES = [
    ("workbench", "inta"),
    ("capabilities", "inta-capabilities"),
    ("diagnostic-handoff", "inta-diagnostic-handoff"),
    ("desktop", "inta-desktop"),
]


def digest(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def output(*args, cwd=ROOT, env=None):
    return subprocess.check_output([str(arg) for arg in args], cwd=cwd, env=env)


def fingerprint():
    hashes = hashlib.sha256(output("git", "rev-parse", "HEAD").strip())
    names = output("git", "ls-files", "-coz", "--exclude-standard", "--", *SOURCES)
    for raw in sorted(set(names.split(b"\0")) - {b""}):
        path = ROOT / raw.decode("utf8")
        hashes.update(raw + b"\0")
        hashes.update(bytes.fromhex(digest(path)) if path.is_file() else b"removed")
    return hashes.hexdigest()


def write_json(path, obj):
    path.write_text(
        json.dumps(obj, indent=2, ensure_ascii=False) + "\n", encoding="utf8"
    )


def run(*args, cwd=ROOT, env=None):
    subprocess.run([str(arg) for arg in args], check=True, cwd=cwd, env=env)


def crate_env():
    env = os.environ.copy()
    registry = Path(os.environ.get("CARGO_HOME", Path.home() / ".cargo")) / "registry"
    # Source locations can appear in release panic messages. Keep build-user paths private.
    mappings = [
        f"--remap-path-prefix={ROOT}=INTA",
        f"--remap-path-prefix={registry}=cargo-registry",
    ]
    env["CARGO_ENCODED_RUSTFLAGS"] = "\x1f".join(mappings)
    env["CARGO_TARGET_DIR"] = str(TARGET / "cargo-release")
    return env


def cargo_payload(out, third_party):
    env = crate_env()
    licenses = {}
    for crate, name in CRATES:
        project = ROOT / "native" / crate
        manifest = project / "Cargo.toml"
        cmd = ("--manifest-path", manifest, "--locked", "--workspace")
        run("cargo", "test", *cmd, env=env)
        run("cargo", "build", *cmd, "--release", env=env)
        shutil.copyfile(
            Path(env["CARGO_TARGET_DIR"]) / "release" / (name + ".exe"),
            out / ("inta-views.exe" if crate == "desktop" else name + ".exe"),
        )
        metadata = json.loads(
            output(
                "cargo",
                "metadata",
                "--manifest-path",
                manifest,
                "--locked",
                "--format-version",
                "1",
                "--filter-platform",
                "x86_64-pc-windows-msvc",
                env=env,
            )
        )
        lock = tomllib.loads((project / "Cargo.lock").read_text(encoding="utf8"))
        pins = {(p["name"], p["version"]): p.get("checksum") for p in lock["package"]}
        for package in metadata["packages"]:
            if not package["source"]:
                continue
            key = (package["name"], package["version"])
            if key in licenses:
                continue
            pkg = Path(package["manifest_path"]).parent
            record = {
                "type": "library",
                "name": key[0],
                "version": key[1],
                "purl": f"pkg:cargo/{key[0]}@{key[1]}",
            }
            if package["license"]:
                record["licenses"] = [
                    {"expression": package["license"].replace("/", " OR ")}
                ]
            if pins.get(key):
                record["hashes"] = [{"alg": "SHA-256", "content": pins[key]}]
            licenses[key] = record
            texts = [
                f
                for f in pkg.iterdir()
                if f.is_file()
                and f.name.upper().startswith(
                    ("LICENSE", "COPYING", "NOTICE", "UNLICENSE", "COPYRIGHT")
                )
            ]
            for sub in ("licenses", "LICENSES", "fonts"):
                if (pkg / sub).is_dir():
                    texts.extend(
                        f
                        for f in (pkg / sub).rglob("*")
                        if f.is_file() and f.suffix.lower() in ("", ".md", ".txt")
                    )
            for f in sorted(set(texts)):
                third_party.write(
                    f, f"cargo/{key[0]}-{key[1]}/{f.relative_to(pkg).as_posix()}"
                )
    return list(licenses.values())


def build(out, thumbprint):
    before = fingerprint()
    out.mkdir(parents=True, exist_ok=False)
    TARGET.mkdir(exist_ok=True)
    run(
        "cargo",
        "rustc",
        "--release",
        "--locked",
        "--manifest-path",
        ROOT / "native/optics/Cargo.toml",
        "--lib",
        "--crate-type",
        "cdylib",
        env=crate_env(),
    )
    shutil.copyfile(
        TARGET / "cargo-release/release/inta_optics.dll", out / "inta_optics.dll"
    )
    dotnet = Path(shutil.which("dotnet") or "dotnet")
    if not output(dotnet, "--version", cwd=HERE).decode().strip().startswith("10."):
        raise RuntimeError(
            "Add the Microsoft .NET 10 SDK directory to PATH to build Windows system tools."
        )
    with zipfile.ZipFile(
        out / "THIRD-PARTY-NOTICES.zip",
        "x",
        compression=zipfile.ZIP_DEFLATED,
        strict_timestamps=False,
    ) as notices:
        components = cargo_payload(out, notices)
        for name in ("LICENSE.txt", "ThirdPartyNotices.txt"):
            notices.write(dotnet.parent / name, "dotnet/" + name)
    write_json(
        out / "sbom.cdx.json",
        {
            "bomFormat": "CycloneDX",
            "specVersion": "1.5",
            "version": 1,
            "metadata": {
                "component": {
                    "type": "application",
                    "name": "INTA Desktop",
                    "version": "0.2",
                }
            },
            "components": sorted(components, key=lambda p: (p["name"], p["version"])),
        },
    )
    run(
        POWERSHELL,
        "-NoProfile",
        "-NonInteractive",
        "-File",
        WORKBENCH / "desktop/build.ps1",
        "-OutputDirectory",
        out,
    )
    system = TARGET / "system-tools"
    run(
        dotnet,
        "publish",
        ROOT / "desktop/INTA.Desktop/INTA.Desktop.csproj",
        "-c",
        "Release",
        "-r",
        "win-x64",
        "--self-contained",
        "-warnaserror",
        "-p:PublishSingleFile=true",
        "-p:IncludeNativeLibrariesForSelfExtract=true",
        "-p:DebugType=None",
        "-o",
        system,
        cwd=HERE,
    )
    shutil.copyfile(system / "INTA.Desktop.exe", out / "INTA System Tools.exe")
    run(
        POWERSHELL,
        "-NoProfile",
        "-NonInteractive",
        "-File",
        HERE / "sign-windows.ps1",
        "-Thumbprint",
        thumbprint,
        "-Payload",
        out,
    )
    for directory in ("site", "wiki"):
        shutil.copytree(ROOT / directory, out / directory)
    shutil.copytree(ROOT / "docs/runbooks", out / "docs/runbooks")
    shutil.copyfile(ROOT / "docs/rescue.md", out / "docs/rescue.md")
    shutil.copytree(
        WORKBENCH / "media",
        out / "media",
        ignore=shutil.ignore_patterns("__pycache__", "*.pyc"),
    )
    shutil.copyfile(
        ROOT / "data/rescue-download-channel.json",
        out / "media/rescue-download-channel.json",
    )
    run(
        sys.executable, "-I", "-B", WORKBENCH / "tests/test_pinned_media.py",
        "--payload", out,
    )
    shutil.copyfile(ROOT / "media/inta_optics_ui.py", out / "media/inta_optics_ui.py")
    shutil.copyfile(out / "inta_optics.dll", out / "media/inta_optics.dll")
    examples = out / "examples"
    shutil.copytree(ROOT / "native/capabilities/examples", examples)
    shutil.copyfile(
        ROOT / "native/capabilities/README.md", examples / "capabilities.md"
    )
    loader = importlib.util.spec_from_file_location("offline", HERE / "offline.py")
    offline = importlib.util.module_from_spec(loader)
    loader.loader.exec_module(offline)
    offline.offline_guides(out)
    with tempfile.TemporaryDirectory(dir=TARGET) as state:
        env = dict(os.environ, INTA_DATA_DIR=state)
        catalog = json.loads(output(out / "inta.exe", "catalog", env=env))
        tools = {r["id"] for r in catalog if r["kind"] == "tool"}
        inputs = [ROOT / "data/tools.json", WORKBENCH / "data/additional-tools.json"]
        expected = {r["id"] for p in inputs for r in json.loads(p.read_bytes())}
        if tools != expected:
            raise ValueError("Desktop catalog is incomplete")
        routes = json.loads(output(out / "inta.exe", "tool", "actions", env=env))
        if {r["id"] for r in routes} != tools or any(
            not all(isinstance(r.get(k), dict) for k in ("install", "launch"))
            for r in routes
        ):
            raise ValueError("Incomplete tool action coverage")
        scenarios = json.loads(output(out / "inta.exe", "scenario", "list", env=env))
        if not scenarios or any(s.get("mode") != "inspect-only" for s in scenarios):
            raise ValueError("Inspection playbooks are missing or invalid")
        discovery = json.loads(
            output(out / "inta-capabilities.exe", "discover", env=env)
        )
        if discovery.get("ok") is not True:
            raise ValueError("Capability discovery failed")
    if fingerprint() != before:
        raise ValueError(
            "Checkout changed during validation. Retaining the last passing build."
        )
    write_json(
        out / "manifest.json",
        {
            "schemaVersion": 1,
            "channel": "desktop-preview",
            "sourceDigest": before,
            "revision": output("git", "rev-parse", "HEAD").decode().strip(),
            "generation": int(output("git", "rev-list", "--count", "HEAD")),
            "workingTreeChanges": bool(
                output("git", "status", "--porcelain", "--", *SOURCES).strip()
            ),
            "files": [
                {"path": p.relative_to(out).as_posix(), "sha256": digest(p)}
                for p in sorted(out.rglob("*"))
                if p.is_file()
            ],
        },
    )
    run(
        POWERSHELL,
        "-NoProfile",
        "-NonInteractive",
        "-File",
        HERE / "sign.ps1",
        "-Directory",
        out,
        "-Thumbprint",
        thumbprint,
    )


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--fingerprint", action="store_true")
    parser.add_argument("--output", type=Path)
    parser.add_argument("--thumbprint")
    opts = parser.parse_args()
    if opts.fingerprint:
        print(fingerprint())
    elif opts.output and opts.thumbprint:
        build(opts.output.resolve(), opts.thumbprint)
    else:
        parser.error("supply --fingerprint or both --output and --thumbprint")
