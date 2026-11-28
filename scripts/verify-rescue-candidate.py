#!/usr/bin/env python3
"""Verify an unchanged Rescue ISO and its preserved, successful firmware proof."""
import argparse
import hashlib
import json
import re
import subprocess
import zipfile
from pathlib import Path
from rescue_boot_log import passed


# These files become part of the ISO. The standalone Media Creator is rebuilt
# from the publication source and has its own tests and checksum.
ISO_INPUTS = (
    "live", "media/inta_brand_ui.py", "data/brand-icons.json", "assets/brand-icons",
    "docs/assets/inta-mark.svg", "assets/inta-brand/heritage-192.png", "scripts/build-iso", "scripts/build-rescue-wallpaper.py",
    "media/inta_optics_ui.py", "native/optics",
)
MEDIA_INPUTS = ("media", "data/rescue-download-channel.json", "scripts/package-media.py", "scripts/package-rescue-release.py")
BUILD_RECIPE = "Build with Debian stable and current signed security updates"
BUILD_STEPS = (
    "Validate source", BUILD_RECIPE,
    "Package complete portable Media Creator", "Store candidate image",
)
BOOT_STEPS = (
    "Verify original build and unchanged native source",
    "Validate current source and boot harness",
    "Retrieve the exact existing candidate", "Verify candidate checksums",
    "Verify BIOS and UEFI boot to XFCE",
    "Preserve runtime proof and candidate provenance",
)


def valid_sha(value, digits=40):
    if not isinstance(value, str) or not re.fullmatch(r"[0-9a-f]{%d}" % digits, value):
        raise ValueError(f"Expected a complete lowercase {digits}-digit SHA")
    return value


def _validate_run(run, repository, run_id, source, workflow):
    valid_sha(source)
    if not isinstance(run_id, int) or isinstance(run_id, bool) or run_id <= 0:
        raise ValueError("Run ID must be positive")
    expected = {"id": run_id, "head_sha": source, "head_branch": "main",
                "status": "completed", "path": workflow}
    for key, value in expected.items():
        if run.get(key) != value:
            raise ValueError(f"Workflow run has unexpected {key}")
    if run.get("repository", {}).get("full_name") != repository:
        raise ValueError("Workflow run belongs to another repository")
    if run.get("event") not in {"push", "schedule", "workflow_dispatch"}:
        raise ValueError("Run must come from the trusted main workflow")


def _validate_artifact(artifact, artifact_id, name, run_id, source, digest):
    valid_sha(digest, 64)
    if not isinstance(artifact_id, int) or isinstance(artifact_id, bool) or artifact_id <= 0:
        raise ValueError("Artifact ID must be positive")
    if (artifact.get("id") != artifact_id or artifact.get("name") != name
            or artifact.get("expired") is not False
            or artifact.get("digest") != "sha256:" + digest
            or not isinstance(artifact.get("size_in_bytes"), int)
            or artifact["size_in_bytes"] <= 0):
        raise ValueError("Artifact identity, retention or checksum does not match")
    provenance = artifact.get("workflow_run", {})
    if provenance.get("id") != run_id or provenance.get("head_sha") != source:
        raise ValueError("Artifact does not belong to the selected source run")


def _require_steps(jobs, job_name, run_id, source, required):
    for job in jobs.get("jobs", []):
        if (job.get("name") != job_name or job.get("head_sha") != source
                or job.get("run_id") != run_id or job.get("status") != "completed"):
            continue
        passed = {step.get("name") for step in job.get("steps", [])
                  if step.get("status") == "completed" and step.get("conclusion") == "success"}
        if set(required) <= passed:
            return
    raise ValueError("The selected run did not pass every required verification step")


def validate_provenance(run, artifact, jobs, repository, run_id, artifact_id, source, digest):
    """The original build may have failed later, after creating a valid ISO."""
    _validate_run(run, repository, run_id, source, ".github/workflows/rescue.yml")
    _validate_artifact(artifact, artifact_id, "inta-rescue-amd64", run_id, source, digest)
    _require_steps(jobs, "build", run_id, source, BUILD_STEPS)


def validate_boot_provenance(run, artifact, jobs, repository, run_id, artifact_id, source, digest):
    """A publication failure does not invalidate completed firmware gates."""
    _validate_run(run, repository, run_id, source, ".github/workflows/rescue-candidate.yml")
    _validate_artifact(artifact, artifact_id, "inta-candidate-verification", run_id, source, digest)
    _require_steps(jobs, "retest", run_id, source, BOOT_STEPS)


def verify_boot_archive(archive, digest, repository, run_id, artifact_id, source, candidate_digest):
    """Read only exact proof members after checking the complete immutable ZIP."""
    valid_sha(digest, 64)
    hasher = hashlib.sha256()
    with Path(archive).open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            hasher.update(block)
    if hasher.hexdigest() != digest:
        raise ValueError("Downloaded boot proof archive checksum does not match")
    with zipfile.ZipFile(archive) as proof:
        names = proof.namelist()
        def read(name):
            if names.count(name) != 1:
                raise ValueError("Boot proof must contain exactly one " + name)
            # Fail closed on unexpectedly large metadata; never treat truncated
            # records as complete evidence.
            if proof.getinfo(name).file_size > 16 * 1024 * 1024:
                raise ValueError("Boot proof record is unexpectedly large: " + name)
            return proof.read(name)
        original = {name: json.loads(read("inta-candidate-proof/" + name + ".json"))
                    for name in ("run", "artifact", "jobs")}
        validate_provenance(original["run"], original["artifact"], original["jobs"],
                            repository, run_id, artifact_id, source, candidate_digest)
        serial_hashes = {}
        for firmware in ("bios", "uefi"):
            serial = read("inta-diagnostics/" + firmware + "-serial.log")
            if not passed(serial):
                raise ValueError(f"Preserved {firmware} serial proof did not pass")
            serial_hashes[firmware] = hashlib.sha256(serial).hexdigest()
        return serial_hashes


def iso_build_recipe(workflow):
    """Protect exact job setup and ISO commands while allowing release changes.

    Accept only the repository's simple block layout. Unfamiliar YAML or global
    environment/defaults require a new build/review rather than a guessed parse.
    """
    lines = workflow.replace("\r\n", "\n").splitlines(keepends=True)
    for line in lines:
        if (line.strip() and not line[0].isspace() and not line.startswith("#")
                and not re.match(r"^(?:name|on|permissions|concurrency|jobs):", line)):
            raise ValueError("Unknown global workflow settings need a fresh ISO build")
    starts = [i for i, line in enumerate(lines) if line.rstrip() == "  build:"]
    markers = [i for i, line in enumerate(lines)
               if line.rstrip() == "      - name: " + BUILD_RECIPE]
    if len(starts) != 1 or len(markers) != 1 or starts[0] >= markers[0]:
        raise ValueError("Cannot identify exactly one protected ISO build recipe")
    # YAML job settings can appear after the steps they affect. Never let a
    # later env/defaults/container/strategy declaration escape prefix checks.
    for index, line in enumerate(lines[starts[0] + 1:], start=starts[0] + 1):
        if line.strip() and len(line) - len(line.lstrip()) <= 2:
            break
        if re.match(r"^    (?:env|defaults|container|services|strategy|uses|with|secrets):", line):
            raise ValueError("Job execution settings require a fresh ISO build")
        if (index > markers[0] and line.strip() and not line.lstrip().startswith("#")
                and len(line) - len(line.lstrip()) == 4
                and not line.startswith("    permissions:")):
            raise ValueError("Late job execution settings require a fresh ISO build")
    end = markers[0] + 1
    while end < len(lines):
        line = lines[end]
        if line.strip() and len(line) - len(line.lstrip()) <= 6:
            break
        end += 1
    return "".join(lines[starts[0]:end])


def verify_native_inputs(source, target="HEAD"):
    valid_sha(source)
    subprocess.run(["git", "merge-base", "--is-ancestor", source, target], check=True)
    subprocess.run(["git", "diff", "--exit-code", source, target, "--", *ISO_INPUTS], check=True)
    recipe = ".github/workflows/rescue.yml"
    original = subprocess.check_output(["git", "show", source + ":" + recipe], text=True)
    current = subprocess.check_output(["git", "show", target + ":" + recipe], text=True)
    if iso_build_recipe(original) != iso_build_recipe(current):
        raise ValueError("ISO build recipe changed; a fresh ISO build and firmware proof are required")


def verify_media_inputs(source, target="HEAD"):
    valid_sha(source)
    subprocess.run(["git", "merge-base", "--is-ancestor", source, target], check=True)
    subprocess.run(["git", "diff", "--exit-code", source, target, "--", *MEDIA_INPUTS], check=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--metadata", type=Path)
    parser.add_argument("--repository", required=True)
    parser.add_argument("--run-id", required=True, type=int)
    parser.add_argument("--artifact-id", required=True, type=int)
    parser.add_argument("--source", required=True)
    parser.add_argument("--artifact-sha256")
    parser.add_argument("--target", default="HEAD")
    parser.add_argument("--media-source")
    parser.add_argument("--inputs-only", action="store_true")
    parser.add_argument("--proof-metadata", type=Path)
    parser.add_argument("--proof-archive", type=Path)
    parser.add_argument("--proof-run-id", type=int)
    parser.add_argument("--proof-artifact-id", type=int)
    parser.add_argument("--proof-source")
    parser.add_argument("--proof-sha256")
    parser.add_argument("--receipt", type=Path)
    args = parser.parse_args()
    if not args.inputs_only:
        required = (args.metadata, args.artifact_sha256, args.proof_metadata, args.proof_archive,
                    args.proof_run_id, args.proof_artifact_id, args.proof_source, args.proof_sha256)
        if not all(required):
            parser.error("Original build metadata and complete immutable boot-proof identity are required")
        def read(directory, name):
            return json.loads((directory / (name + ".json")).read_text())
        validate_provenance(*(read(args.metadata, name) for name in ("run", "artifact", "jobs")),
                            args.repository, args.run_id, args.artifact_id, args.source, args.artifact_sha256)
        validate_boot_provenance(*(read(args.proof_metadata, name) for name in ("run", "artifact", "jobs")),
                                 args.repository, args.proof_run_id, args.proof_artifact_id,
                                 args.proof_source, args.proof_sha256)
        serials = verify_boot_archive(args.proof_archive, args.proof_sha256, args.repository,
                                     args.run_id, args.artifact_id, args.source, args.artifact_sha256)
        verify_native_inputs(args.source, args.proof_source)
        subprocess.run(["git", "merge-base", "--is-ancestor", args.proof_source, args.target], check=True)
    verify_native_inputs(args.source, args.target)
    if args.media_source:
        verify_media_inputs(args.media_source, args.target)
    if args.receipt and not args.inputs_only:
        receipt = {"isoSource": args.source, "candidateRunId": args.run_id,
                   "candidateArtifactId": args.artifact_id, "candidateArchiveSHA256": args.artifact_sha256,
                   "firmwareProofSource": args.proof_source, "firmwareProofRunId": args.proof_run_id,
                   "firmwareProofArtifactId": args.proof_artifact_id, "firmwareProofSHA256": args.proof_sha256,
                   "serialSHA256": serials, "target": args.target, "unchangedISOInputs": True}
        args.receipt.parent.mkdir(parents=True, exist_ok=True)
        args.receipt.write_text(json.dumps(receipt, indent=2) + "\n")
    print(f"Verified unchanged ISO inputs and build recipe from {args.source} to {args.target}")
    if not args.inputs_only:
        print(f"Verified preserved BIOS and UEFI boot proof from run {args.proof_run_id}")


if __name__ == "__main__":
    main()
