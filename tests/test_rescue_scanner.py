"""Tests use an inert dpkg status fixture, never load a scanner or use the network."""
import datetime as dt
import hashlib
import importlib.util
import json
import subprocess
import sys
from pathlib import Path
import tempfile
import unittest

p = Path(__file__).resolve().parents[1] / "scripts/check-rescue-scanner.py"
spec = importlib.util.spec_from_file_location("scanner", p)
scan = importlib.util.module_from_spec(spec)
spec.loader.exec_module(scan)

class Tests(unittest.TestCase):
    def inventory(self, version, library_version=None, arch="amd64"):
        root = tempfile.TemporaryDirectory()
        self.addCleanup(root.cleanup)
        path = Path(root.name)
        status = path / "var/lib/dpkg/status"
        status.parent.mkdir(parents=True)
        (path / "usr/lib").mkdir(parents=True)
        (path / "usr/lib/os-release").write_text("VERSION_CODENAME=trixie\n")
        entries = []
        for package,v in (("clamav",version),("libclamav12",library_version or version)):
            entries.append(f"Package: {package}\nStatus: install ok installed\nArchitecture: {arch}\n"
                f"Maintainer: Fixture <fixture@example.invalid>\nDescription: Inert fixture\n"
                f"Version: {v}\nSource: clamav ({version})\n")
        status.write_text("\n".join(entries))
        self.fixture_root = path
        return scan.packages(path)

    def review(self, rows, **overrides):
        return dict(dict(suite="trixie", architecture="amd64", source="clamav", reviewed_on="2026-11-27",
                    evidence=[scan.TRACKER, "https://tracker.debian.org/news/1/synthetic-fixture/"], packages=rows), **overrides)

    def test_public_release_refused_even_with_erroneous_review(self):
        rows = self.inventory(scan.OLD)
        state = scan.qualify(rows, self.review(rows), dt.date(2026,11,27))
        self.assertEqual(state, "known-vulnerable")

    def test_only_exact_current_stable_tuple_qualifies(self):
        # Synthetic future stable package identifier, not a claimed Debian release.
        rows = self.inventory("1.4.7+dfsg-1~deb13u99")
        review = self.review(rows)
        today = dt.date(2026,11,27)
        self.assertEqual(scan.qualify(rows, review, today), "reviewed-exact-match")
        self.assertEqual(scan.qualify(rows, None, today), "unverified")
        self.assertEqual(scan.qualify(rows, self.review(rows, suite="testing"), today), "unverified")
        self.assertEqual(scan.qualify(rows, review, dt.date(2026,12,5)), "unverified")
        different = self.inventory("1.4.7+dfsg-1~deb13u99", "1.4.7+dfsg-1~deb13u99+b1")
        self.assertEqual(scan.qualify(different, review, today), "unverified")

    def test_wrong_architecture_refused(self):
        with self.assertRaisesRegex(ValueError, "identity"):
            self.inventory(scan.OLD, arch="arm64")

    def test_cli_binds_report_to_iso_digest_and_exits_nonzero(self):
        self.inventory(scan.OLD)
        iso = self.fixture_root / "fixture.iso"
        data = b"not bootable, inert test bytes"
        iso.write_bytes(data)
        digest = hashlib.sha256(data).hexdigest()
        argv = [sys.executable, "-B", str(p), "--image-root", str(self.fixture_root), "--iso", str(iso)]
        def run(expected):
            return subprocess.run([*argv, "--expected-sha256", expected],
                                  timeout=10, capture_output=True, text=True)
        result = run(digest)
        self.assertEqual(result.returncode, 2, result.stderr)
        self.assertEqual(json.loads(result.stdout)["iso_sha256"], digest)
        self.assertEqual(json.loads(result.stdout)["qualification"], "known-vulnerable")
        corrupt = run("0"*64)
        self.assertEqual(corrupt.returncode, 2)
        self.assertIn("ISO digest mismatch", corrupt.stderr)
        self.assertFalse(corrupt.stdout)
