import copy
import hashlib
import importlib.util
import json
import subprocess
import sys
import tempfile
import unittest
import zipfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))
SPEC = importlib.util.spec_from_file_location("rescue_candidate", ROOT / "scripts/verify-rescue-candidate.py")
CANDIDATE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CANDIDATE)
WORKFLOW = """name: Build fixture
on: workflow_dispatch
jobs:
  build:
    runs-on: ubuntu-24.04
    timeout-minutes: 55
    steps:
      - uses: actions/checkout@fixture
      - name: Validate source
        run: python3 -m unittest discover -s tests
      - name: Build with Debian stable and current signed security updates
        run: |
          docker run debian:13-slim /src/scripts/build-iso /out/inta-dist
      - name: Package complete portable Media Creator
        run: python3 scripts/package-media.py
    permissions:
      contents: write
"""


class RescueCandidateIntegrity(unittest.TestCase):
    def setUp(self):
        self.source = "a" * 40
        self.digest = "c" * 64
        self.run = {"id": 123, "head_sha": self.source, "head_branch": "main",
                    "status": "completed", "path": ".github/workflows/rescue.yml",
                    "repository": {"full_name": "example/INTA"}, "event": "push"}
        self.artifact = {"id": 456, "name": "inta-rescue-amd64", "expired": False,
                         "digest": "sha256:" + self.digest, "size_in_bytes": 200,
                         "workflow_run": {"id": 123, "head_sha": self.source}}
        self.jobs = {"jobs": [{"name": "build", "head_sha": self.source, "run_id": 123,
                              "status": "completed", "steps": [
            {"name": name, "status": "completed", "conclusion": "success"}
            for name in CANDIDATE.BUILD_STEPS]}]}
        self.proof_source = "b" * 40
        self.proof_digest = "d" * 64
        self.proof_run = dict(self.run, id=789, head_sha=self.proof_source,
                              path=".github/workflows/rescue-candidate.yml", conclusion="failure")
        self.proof_artifact = dict(self.artifact, id=101, name="inta-candidate-verification",
                                   digest="sha256:" + self.proof_digest,
                                   workflow_run={"id": 789, "head_sha": self.proof_source})
        self.proof_jobs = {"jobs": [{"name": "retest", "head_sha": self.proof_source, "run_id": 789,
                                    "status": "completed", "steps": [
            {"name": name, "status": "completed", "conclusion": "success"}
            for name in CANDIDATE.BOOT_STEPS]}]}

    def validate(self):
        CANDIDATE.validate_provenance(self.run, self.artifact, self.jobs,
                                      "example/INTA", 123, 456, self.source, self.digest)

    def validate_proof(self):
        CANDIDATE.validate_boot_provenance(self.proof_run, self.proof_artifact, self.proof_jobs,
                                           "example/INTA", 789, 101, self.proof_source, self.proof_digest)

    def test_built_candidate_is_valid_despite_later_boot_failure(self):
        self.run["conclusion"] = "failure"
        self.validate()

    def test_successful_boot_proof_survives_later_publication_failure(self):
        self.validate_proof()

    def test_rejects_wrong_source_repository_event_or_workflow(self):
        original = copy.deepcopy(self.run)
        for key, value in (("head_sha", "b" * 40), ("repository", {"full_name": "other/INTA"}),
                           ("event", "pull_request"), ("head_branch", "feature"),
                           ("path", ".github/workflows/untrusted.yml"), ("status", "in_progress")):
            with self.subTest(key=key):
                self.run = copy.deepcopy(original)
                self.run[key] = value
                with self.assertRaises(ValueError):
                    self.validate()

    def test_rejects_wrong_expired_or_different_digest_artifact(self):
        original = copy.deepcopy(self.artifact)
        for key, value in (("id", 999), ("expired", True), ("digest", "sha256:" + "e" * 64),
                           ("workflow_run", {"id": 999, "head_sha": self.source})):
            with self.subTest(key=key):
                self.artifact = copy.deepcopy(original)
                self.artifact[key] = value
                with self.assertRaises(ValueError):
                    self.validate()

    def test_all_build_and_boot_gates_are_required(self):
        for jobs, validate, names in ((self.jobs, self.validate, CANDIDATE.BUILD_STEPS),
                                     (self.proof_jobs, self.validate_proof, CANDIDATE.BOOT_STEPS)):
            for index, name in enumerate(names):
                with self.subTest(step=name):
                    jobs["jobs"][0]["steps"][index]["conclusion"] = "failure"
                    with self.assertRaises(ValueError):
                        validate()
                    jobs["jobs"][0]["steps"][index]["conclusion"] = "success"

    def make_archive(self, directory, missing=None, altered_source=False, duplicate=None, bios_log=None):
        archive = Path(directory) / "proof.zip"
        records = {"run": copy.deepcopy(self.run), "artifact": self.artifact, "jobs": self.jobs}
        if altered_source:
            records["run"]["head_sha"] = "f" * 40
        with zipfile.ZipFile(archive, "w") as output:
            for name, record in records.items():
                output.writestr("inta-candidate-proof/" + name + ".json", json.dumps(record))
            for firmware in ("bios", "uefi"):
                serial = "INTA_SMOKE_START\nVerified 303 bundled native icon files\n"
                serial += "INTA_SMOKE_FAIL\n" if firmware == missing else "INTA_SMOKE_PASS\n"
                log = bios_log if firmware == "bios" and bios_log is not None else serial
                output.writestr("inta-diagnostics/" + firmware + "-serial.log", log)
            if duplicate:
                output.writestr("inta-diagnostics/" + duplicate + "-serial.log", "INTA_SMOKE_PASS\n")
        return archive, hashlib.sha256(archive.read_bytes()).hexdigest()

    def check_archive(self, archive, digest):
        return CANDIDATE.verify_boot_archive(archive, digest, "example/INTA",
                                             123, 456, self.source, self.digest)

    def test_actual_archive_requires_both_firmware_markers(self):
        with tempfile.TemporaryDirectory() as directory:
            archive, digest = self.make_archive(directory)
            result = self.check_archive(archive, digest)
            self.assertEqual(set(result), {"bios", "uefi"})
            for firmware in ("bios", "uefi"):
                archive, digest = self.make_archive(directory, missing=firmware)
                with self.subTest(firmware=firmware), self.assertRaisesRegex(ValueError, firmware):
                    self.check_archive(archive, digest)

    def test_archive_bytes_and_original_candidate_identity_are_bound(self):
        with tempfile.TemporaryDirectory() as directory:
            archive, digest = self.make_archive(directory)
            with self.assertRaisesRegex(ValueError, "checksum"):
                self.check_archive(archive, "0" * 64)
            archive, digest = self.make_archive(directory, altered_source=True)
            with self.assertRaisesRegex(ValueError, "head_sha"):
                self.check_archive(archive, digest)

    def test_current_boot_and_archived_proof_require_one_ordered_attempt(self):
        start = "INTA_SMOKE_START"
        passed = "INTA_SMOKE_PASS"
        fail = "INTA_SMOKE_FAIL (1)"
        cases = [
            (["firmware banner", start, "XFCE ready", passed, "powered off"], True),
            ([passed], False),
            ([passed, start], False),
            ([start, passed, fail], False),
            ([start, fail, start, passed], False),
            ([start, passed, passed], False),
        ]
        with tempfile.TemporaryDirectory() as directory:
            serial = Path(directory) / "bios-serial.log"
            for lines, expected in cases:
                with self.subTest(lines=lines):
                    contents = ("\r\n".join(lines) + "\r\n").encode()
                    serial.write_bytes(contents)
                    check = subprocess.run([sys.executable, str(ROOT / "scripts/rescue_boot_log.py"), str(serial)],
                                           capture_output=True)
                    self.assertEqual(check.returncode == 0, expected)
                    archive, digest = self.make_archive(directory, bios_log=contents)
                    if expected:
                        self.assertEqual(self.check_archive(archive, digest)["bios"], hashlib.sha256(contents).hexdigest())
                    else:
                        with self.assertRaisesRegex(ValueError, "bios"):
                            self.check_archive(archive, digest)


class RescueInputCompatibility(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.checkout = Path(self.temporary.name)
        self.git("init")
        self.git("config", "user.email", "test@example.invalid")
        self.git("config", "user.name", "Fixture")
        for name in ("live/fixture", "media/inta_brand_ui.py", "media/inta-media.py",
                     "native/optics/src/lib.rs", "scripts/build-rescue-wallpaper.py", "media/inta_optics_ui.py",
                     "data/brand-icons.json", "assets/brand-icons/fixture.png",
                     "docs/assets/inta-mark.svg", "scripts/build-iso", "scripts/package-media.py"):
            self.write(name, "original\n")
        self.write(".github/workflows/rescue.yml", WORKFLOW)
        self.commit()
        self.source = self.git("rev-parse", "HEAD").strip()

    def git(self, *args):
        return subprocess.check_output(["git", "-C", str(self.checkout), *args],
                                       stderr=subprocess.DEVNULL, text=True)

    def write(self, name, text):
        path = self.checkout / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text)

    def commit(self):
        self.git("add", ".")
        self.git("commit", "-m", "fixture")

    def verify(self):
        command = [sys.executable, str(ROOT / "scripts/verify-rescue-candidate.py"),
                   "--inputs-only", "--repository", "example/INTA", "--run-id", "123",
                   "--artifact-id", "456", "--source", self.source]
        return subprocess.run(command, cwd=self.checkout, capture_output=True, text=True)

    def test_current_media_creator_and_distribution_changes_are_allowed(self):
        self.write("media/inta-media.py", "current multipart downloader\n")
        self.write("scripts/package-media.py", "current portable package\n")
        self.write(".github/workflows/rescue.yml",
                   WORKFLOW.replace("run: python3 scripts/package-media.py",
                                    "run: python3 scripts/package-rescue-release.py"))
        self.commit()
        result = self.verify()
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_all_iso_inputs_remain_protected(self):
        for name in ("live/fixture", "media/inta_brand_ui.py", "media/inta_optics_ui.py",
                     "native/optics/src/lib.rs", "scripts/build-rescue-wallpaper.py", "data/brand-icons.json",
                     "assets/brand-icons/fixture.png", "docs/assets/inta-mark.svg", "scripts/build-iso"):
            with self.subTest(path=name):
                self.git("reset", "--hard", self.source)
                self.write(name, "changed\n")
                self.commit()
                result = self.verify()
                self.assertNotEqual(result.returncode, 0)
                self.assertIn(name, result.stdout)

    def test_changed_original_build_recipe_is_rejected(self):
        self.write(".github/workflows/rescue.yml", WORKFLOW.replace("debian:13-slim", "debian:12-slim"))
        self.commit()
        result = self.verify()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("ISO build recipe changed", result.stderr)

    def test_late_yaml_execution_settings_cannot_bypass_recipe_protection(self):
        for key, value in (("env", "      BUILD_MODE: different"),
                           ("defaults", "      run: {shell: sh}"),
                           ("container", "      image: other"),
                           ("strategy", "      matrix: {arch: [other]}"),
                           ("'env'", "      BUILD_MODE: quoted"),
                           ("runs-on", " ubuntu-22.04")):
            with self.subTest(key=key):
                changed = WORKFLOW + f"    {key}:\n{value}\n"
                with self.assertRaisesRegex(ValueError, "execution settings"):
                    CANDIDATE.iso_build_recipe(changed)

    def test_missing_recipe_or_global_environment_fails_closed(self):
        for value in (WORKFLOW.replace(CANDIDATE.BUILD_RECIPE, "Other build"),
                      "env:\n  BUILD: changed\n" + WORKFLOW):
            with self.assertRaises(ValueError):
                CANDIDATE.iso_build_recipe(value)

    def test_publication_workflow_reuses_proof_and_rebuilds_media_before_release(self):
        workflow = (ROOT / ".github/workflows/rescue-candidate.yml").read_text()
        sequence = [
            "Verify original build and authoritative firmware proof",
            "Validate current source and Media Creator",
            "Retrieve the exact existing candidate",
            "Verify original candidate checksums and exact ISO identity",
            "Rebuild and verify the current portable Media Creator",
            "Package hostable release assets and prove complete ISO reconstruction",
            "Publish the verified prepared release with exact provenance",
        ]
        positions = [workflow.index("- name: " + name) for name in sequence]
        self.assertEqual(positions, sorted(positions))
        self.assertNotIn('scripts/smoke-iso "$iso"', workflow)
        self.assertIn('"$RUNNER_TEMP"/inta-release/*', workflow)
        self.assertNotIn('"$RUNNER_TEMP"/inta-dist/*', workflow)


if __name__ == "__main__":
    unittest.main()
