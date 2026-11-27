import copy
import importlib.util
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("rescue_candidate", ROOT / "scripts/verify-rescue-candidate.py")
CANDIDATE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CANDIDATE)


class RescueCandidateIntegrity(unittest.TestCase):
    def setUp(self):
        self.source = "a" * 40
        self.run = {"id": 123, "head_sha": self.source, "head_branch": "main",
                    "status": "completed", "path": ".github/workflows/rescue.yml",
                    "repository": {"full_name": "example/INTA"}, "event": "push"}
        self.artifact = {"id": 456, "name": "inta-rescue-amd64", "expired": False,
                         "workflow_run": {"id": 123, "head_sha": self.source}}
        self.jobs = {"jobs": [{"name": "build", "head_sha": self.source, "run_id": 123, "steps": [
            {"name": name, "conclusion": "success"} for name in CANDIDATE.BUILD_STEPS]}]}

    def validate(self):
        CANDIDATE.validate_provenance(self.run, self.artifact, self.jobs,
                                      "example/INTA", 123, 456, self.source)

    def test_built_candidate_can_be_retested_after_boot_failure(self):
        self.run["conclusion"] = "failure"
        self.validate()

    def test_rejects_wrong_source_or_repository_or_untrusted_event(self):
        original = copy.deepcopy(self.run)
        for key, value in (("head_sha", "b" * 40), ("repository", {"full_name": "other/INTA"}),
                           ("event", "pull_request"), ("head_branch", "feature")):
            with self.subTest(key=key):
                self.run = copy.deepcopy(original)
                self.run[key] = value
                with self.assertRaises(ValueError):
                    self.validate()

    def test_rejects_wrong_or_expired_artifact(self):
        original = copy.deepcopy(self.artifact)
        for key, value in (("id", 999), ("expired", True),
                           ("workflow_run", {"id": 999, "head_sha": self.source})):
            with self.subTest(key=key):
                self.artifact = copy.deepcopy(original)
                self.artifact[key] = value
                with self.assertRaises(ValueError):
                    self.validate()

    def test_all_pre_boot_gates_are_required(self):
        for index in range(len(CANDIDATE.BUILD_STEPS)):
            with self.subTest(index=index):
                self.jobs["jobs"][0]["steps"][index]["conclusion"] = "failure"
                with self.assertRaises(ValueError):
                    self.validate()
                self.jobs["jobs"][0]["steps"][index]["conclusion"] = "success"

    def test_native_input_changes_are_detected_by_git(self):
        with tempfile.TemporaryDirectory() as directory:
            checkout = Path(directory)
            def git(*args):
                return subprocess.check_output(["git", "-C", str(checkout), *args], stderr=subprocess.DEVNULL)
            git("init")
            git("config", "user.email", "test@example.invalid")
            git("config", "user.name", "Fixture")
            (checkout / "live").mkdir()
            (checkout / "live/fixture").write_text("original")
            git("add", ".")
            git("commit", "-m", "candidate")
            source = git("rev-parse", "HEAD").decode().strip()
            command = [sys.executable, str(ROOT / "scripts/verify-rescue-candidate.py"),
                       "--inputs-only", "--repository", "example/INTA", "--run-id", "123",
                       "--artifact-id", "456", "--source", source]
            unchanged = subprocess.run(command, cwd=checkout, capture_output=True)
            self.assertEqual(unchanged.returncode, 0, unchanged.stderr.decode())
            (checkout / "live/fixture").write_text("changed")
            git("add", ".")
            git("commit", "-m", "native change")
            result = subprocess.run(command, cwd=checkout, capture_output=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("live/fixture", result.stdout.decode())


if __name__ == "__main__":
    unittest.main()
