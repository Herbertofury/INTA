"""Stage the selected Windows executable's locked Cargo dependency inventory and original notices."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import shutil
import subprocess
import tempfile
import tomllib


def digest(path):
    with path.open("rb") as source:
        return hashlib.file_digest(source, "sha256").hexdigest()


def build(binary, output):
    project = Path(__file__).resolve().parent
    if not binary.is_file() or output.exists() or not output.parent.is_dir():
        raise ValueError("Select a built executable and a new output directory under an existing parent")
    meta = json.loads(subprocess.check_output(
        ["cargo", "metadata", "--format-version", "1", "--locked", "--offline",
         "--filter-platform", "x86_64-pc-windows-msvc", "--manifest-path", str(project / "Cargo.toml")],
        cwd=project, timeout=90))
    project_id = meta["resolve"]["root"]
    packages = {p["id"]: p for p in meta["packages"]}
    lock = {(v["name"], v["version"]): v.get("checksum")
            for v in tomllib.loads((project / "Cargo.lock").read_text(encoding="utf-8"))["package"]}
    refs = {i: f"pkg:cargo/{p['name']}@{p['version']}" for i, p in packages.items()}
    components, notices = [], []
    with tempfile.TemporaryDirectory(prefix=".inta-notices-", dir=output.parent) as stage:
        dest = Path(stage) / "bundle"
        dest.mkdir()
        for key, p in sorted(packages.items(), key=lambda kv: kv[1]["name"]):
            if key == project_id:
                continue
            checksum = lock[p["name"], p["version"]]
            if not checksum or not p["source"].startswith("registry+"):
                raise ValueError(f"Dependency has no pinned registry digest: {p['name']}")
            source = Path(p["manifest_path"]).parent
            files = sorted(f for f in source.iterdir() if f.is_file() and re.match(
                r"^(?:LICENSE|LICENCE|COPYING|NOTICE)(?:$|[._-])", f.name, re.I))
            if p["license_file"]:
                candidate = (source / p["license_file"]).resolve()
                if not candidate.is_relative_to(source.resolve()) or not candidate.is_file():
                    raise ValueError(f"Invalid declared license file: {p['name']}")
                if candidate not in files:
                    files.append(candidate)
            if not files:
                raise ValueError(f"Review dependency license materials: {p['name']}")
            folder = dest / "licenses" / f"{p['name']}-{p['version']}"
            folder.mkdir(parents=True)
            evidence = []
            for f in files:
                target = folder / f.name
                shutil.copyfile(f, target)
                evidence.append({"path": target.relative_to(dest).as_posix(), "sha256": digest(target)})
            notices.append({"name": p["name"], "version": p["version"], "manifestLicense": p["license"], "files": evidence})
            c = {"type": "library", "name": p["name"], "version": p["version"], "purl": refs[key],
                 "bom-ref": refs[key], "hashes": [{"alg": "SHA-256", "content": checksum}]}
            if p["license"]:
                c["licenses"] = [{"expression": p["license"].replace("MIT/Apache-2.0", "MIT OR Apache-2.0")}]
            components.append(c)
        root = packages[project_id]
        sbom = {"bomFormat": "CycloneDX", "specVersion": "1.5", "version": 1,
                "metadata": {"component": {"type": "application", "name": root["name"], "version": root["version"],
                                          "bom-ref": refs[project_id], "hashes": [{"alg": "SHA-256", "content": digest(binary)}]}},
                "components": components, "dependencies": [
                    {"ref": refs[n["id"]], "dependsOn": sorted(refs[d["pkg"]] for d in n["deps"])}
                    for n in meta["resolve"]["nodes"]]}
        for filename, data in [("capabilities.cdx.json", sbom), ("license-inventory.json", notices)]:
            (dest / filename).write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")
        dest.rename(output)
    print(f"{len(components)} dependencies and {sum(len(r['files']) for r in notices)} original license documents in {output}")


if __name__ == "__main__":
    cli = argparse.ArgumentParser(description=__doc__)
    cli.add_argument("executable", type=Path)
    cli.add_argument("new_directory", type=Path)
    args = cli.parse_args()
    build(args.executable.resolve(), args.new_directory.resolve())
