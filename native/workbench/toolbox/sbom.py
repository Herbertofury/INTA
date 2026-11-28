"""Record package and embedded dependency identities for the private toolbox."""

import argparse
import json
import subprocess
import tomllib
from pathlib import Path
from urllib.parse import quote

import manage


def generate(root):
    idx = manage.index(root)
    components = []
    dependencies = []
    for pin in manage.PINS:
        rel = idx["packages"].get(pin["id"])
        if not rel:
            continue
        binary = manage.within(root, rel)
        if manage.digest(binary) != pin["exeSha256"]:
            raise manage.Failure("Package integrity failed before SBOM generation")
        key = pin["id"] + "@" + pin["version"]
        package = {
            "type": "application",
            "name": pin["name"],
            "version": pin["version"],
            "hashes": [{"alg": "SHA-256", "content": pin["exeSha256"]}],
            "licenses": [{"license": {"name": pin["license"]}}],
            "externalReferences": [
                {
                    "type": "vcs",
                    "url": f"https://github.com/{pin['repo']}/tree/{pin['commit']}",
                },
                {"type": "distribution", "url": manage.url(pin)},
            ],
        }
        package["bom-ref"] = key
        components.append(package)
        items = []
        if pin["id"] in ("restic", "rclone"):
            build = json.loads(
                subprocess.check_output(
                    ["go", "version", "-m", "-json", str(binary)], timeout=30
                )
            )
            for dep in build.get("Deps", []):
                actual = dep.get("Replace", dep)
                items.append(
                    (
                        actual["Path"],
                        actual.get("Version", "unspecified"),
                        "golang",
                        "embedded-go-buildinfo",
                    )
                )
        elif pin["id"] == "yara-x":
            repo = manage.source(root, pin, True)
            lock = tomllib.loads(
                manage.git(
                    repo, "cat-file", "blob", pin["commit"] + ":Cargo.lock"
                ).decode()
            )
            for dep in lock["package"]:
                if dep.get("source", "").startswith("registry+"):
                    items.append(
                        (dep["name"], dep["version"], "cargo", "source-lock-candidate")
                    )
        refs = []
        for name, version, system, origin in items:
            uid = f"pkg:{system}/{quote(name, safe='/')}@{quote(version, safe='.-')}"
            ref = f"{key}/{uid}"
            components.append(
                {
                    "type": "library",
                    "bom-ref": ref,
                    "name": name,
                    "version": version,
                    "purl": uid,
                    "properties": [{"name": "inta:evidence", "value": origin}],
                }
            )
            if origin == "embedded-go-buildinfo":
                refs.append(ref)
        dependencies.append({"ref": key, "dependsOn": refs})
    report = {
        "bomFormat": "CycloneDX",
        "specVersion": "1.5",
        "version": 1,
        "metadata": {
            "timestamp": manage.date(),
            "properties": [
                {
                    "name": "inta:scope",
                    "value": "Local executable pins, embedded Go build metadata, and YARA-X source lock candidates. Source candidates are not evidence of binary inclusion. File-level 7-Zip notices stay with its source.",
                }
            ],
        },
        "components": components,
        "dependencies": dependencies,
    }
    target = manage.within(root, "91 - Licenses, Notices & Provenance/toolbox.cdx.json")
    manage.dump(target, report)
    return len(components)


if __name__ == "__main__":
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--root", type=Path, required=True)
    a = p.parse_args()
    print(f"{generate(manage.safe_root(a.root))} components recorded")
