#!/usr/bin/env python3
"""Verify the provenance and unchanged native inputs of a reusable Rescue image."""
import argparse
import json
import re
import subprocess
from pathlib import Path


NATIVE_INPUTS = (
    "live", "media", "data/brand-icons.json", "assets/brand-icons",
    "docs/assets/inta-mark.svg", "scripts/build-iso", "scripts/package-media.py",
    ".github/workflows/rescue.yml",
)
BUILD_STEPS = (
    "Validate source",
    "Build with Debian stable and current signed security updates",
    "Package complete portable Media Creator",
    "Store candidate image",
)


def validate_provenance(run, artifact, jobs, repository, run_id, artifact_id, source):
    """Accept a completed main-branch build whose four pre-boot gates passed."""
    if not re.fullmatch(r"[0-9a-f]{40}", source):
        raise ValueError("Candidate source must be a complete lowercase Git commit SHA")
    if run_id <= 0 or artifact_id <= 0:
        raise ValueError("Candidate run and artifact IDs must be positive")
    expected_run = {
        "id": run_id, "head_sha": source, "head_branch": "main",
        "status": "completed", "path": ".github/workflows/rescue.yml",
    }
    for key, value in expected_run.items():
        if run.get(key) != value:
            raise ValueError(f"Candidate run has unexpected {key}")
    if run.get("repository", {}).get("full_name") != repository:
        raise ValueError("Candidate run belongs to another repository")
    if run.get("event") not in {"push", "schedule", "workflow_dispatch"}:
        raise ValueError("Candidate must come from the trusted main Rescue workflow")
    if (artifact.get("id") != artifact_id or artifact.get("name") != "inta-rescue-amd64"
            or artifact.get("expired") is not False):
        raise ValueError("Candidate artifact identity or retention does not match")
    provenance = artifact.get("workflow_run", {})
    if provenance.get("id") != run_id or provenance.get("head_sha") != source:
        raise ValueError("Candidate artifact does not belong to the selected source run")
    for job in jobs.get("jobs", []):
        if (job.get("name") != "build" or job.get("head_sha") != source
                or job.get("run_id") != run_id):
            continue
        passed = {step.get("name") for step in job.get("steps", [])
                  if step.get("conclusion") == "success"}
        if set(BUILD_STEPS) <= passed:
            return
    raise ValueError("Candidate did not pass source, image, packaging and upload gates")


def verify_native_inputs(source, target="HEAD"):
    subprocess.run(["git", "merge-base", "--is-ancestor", source, target], check=True)
    subprocess.run(["git", "diff", "--exit-code", source, target, "--", *NATIVE_INPUTS],
                   check=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--metadata", type=Path)
    parser.add_argument("--repository", required=True)
    parser.add_argument("--run-id", required=True, type=int)
    parser.add_argument("--artifact-id", required=True, type=int)
    parser.add_argument("--source", required=True)
    parser.add_argument("--target", default="HEAD")
    parser.add_argument("--inputs-only", action="store_true")
    args = parser.parse_args()
    if not re.fullmatch(r"[0-9a-f]{40}", args.source):
        parser.error("--source must be a complete lowercase Git commit SHA")
    if not args.inputs_only:
        if args.metadata is None:
            parser.error("--metadata is required for provenance validation")
        read = lambda name: json.loads((args.metadata / name).read_text())
        validate_provenance(read("run.json"), read("artifact.json"), read("jobs.json"),
                            args.repository, args.run_id, args.artifact_id, args.source)
    verify_native_inputs(args.source, args.target)
    print(f"Verified unchanged native inputs from {args.source} to {args.target}")


if __name__ == "__main__":
    main()
