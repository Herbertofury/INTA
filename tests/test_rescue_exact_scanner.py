"""Inert ISO/SquashFS fixtures, with real host extraction and Debian package tools."""
import datetime as dt
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest


SCRIPT = Path(__file__).resolve().parents[1] / "scripts/check-rescue-image-scanner.py"
KNOWN_OLD = "1.4.6+dfsg-1~deb13u1"
SYNTHETIC = "1.4.7+dfsg-1~deb13u99"
HOST_TOOLS = ("xorriso", "mksquashfs", "unsquashfs", "dpkg-query")


@unittest.skipUnless(all(map(shutil.which, HOST_TOOLS)), "Install trusted ISO/SquashFS fixture tooling")
class ExactImageTests(unittest.TestCase):
    def setUp(self):
        f = tempfile.TemporaryDirectory()
        self.addCleanup(f.cleanup)
        self.folder = Path(f.name)

    def pack(self, version):
        root = self.folder / "inert"
        (root / "var/lib/dpkg").mkdir(parents=True)
        (root / "usr/lib").mkdir(parents=True)
        (root / "usr/lib/os-release").write_text("VERSION_CODENAME=trixie\n")
        lines = []
        for package in ("clamav", "libclamav12"):
            lines.append(f"Package: {package}\nStatus: install ok installed\nArchitecture: amd64\n"
                         f"Maintainer: Fixture <fixture@example.invalid>\nDescription: Inert fixture\n"
                         f"Version: {version}\nSource: clamav ({version})\n")
        (root / "var/lib/dpkg/status").write_text("\n".join(lines))
        image_folder = self.folder / "disc"
        (image_folder / "live").mkdir(parents=True)
        squash = image_folder / "live/filesystem.squashfs"
        subprocess.run(["mksquashfs", str(root), str(squash), "-noappend", "-processors",
                        "1", "-no-progress", "-quiet"], check=True, capture_output=True, timeout=15)
        self.iso = self.folder / "synthetic.iso"
        subprocess.run(["xorriso", "-as", "mkisofs", "-quiet", "-o", str(self.iso), str(image_folder)],
                       check=True, capture_output=True, timeout=15)
        self.digest = hashlib.sha256(self.iso.read_bytes()).hexdigest()

    def run_check(self, sha=None, review=None):
        args = [sys.executable, "-B", str(SCRIPT), "--iso", str(self.iso),
                "--expected-sha256", self.digest if sha is None else sha]
        if review is not None:
            args += ["--review", str(review)]
        return subprocess.run(args, capture_output=True, text=True, timeout=25)

    def review(self, version):
        review = self.folder / "review.json"
        review.write_text(json.dumps(dict(suite="trixie", architecture="amd64", source="clamav",
            reviewed_on=dt.datetime.now(dt.timezone.utc).date().isoformat(),
            evidence=["https://security-tracker.debian.org/tracker/source-package/clamav",
                      "https://tracker.debian.org/news/1/inert-test-only/"],
            packages=[dict(package=p, architecture="amd64", source="clamav", source_version=version,
                           binary_version=version) for p in ("clamav", "libclamav12")])))
        return review

    def test_known_old_refused_from_actual_iso_even_with_review(self):
        self.pack(KNOWN_OLD)
        result = self.run_check(review=self.review(KNOWN_OLD))
        self.assertEqual(result.returncode, 2, result.stderr)
        record = json.loads(result.stdout)
        self.assertEqual(record["iso_sha256"], self.digest)
        self.assertEqual(record["qualification"], "known-vulnerable")

    def test_exact_synthetic_reviewed_tuple_only(self):
        self.pack(SYNTHETIC)
        no_review = self.run_check()
        self.assertEqual(no_review.returncode, 2, no_review.stderr)
        self.assertEqual(json.loads(no_review.stdout)["qualification"], "unverified")
        reviewed = self.run_check(review=self.review(SYNTHETIC))
        self.assertEqual(reviewed.returncode, 0, reviewed.stderr)
        self.assertEqual(json.loads(reviewed.stdout)["qualification"], "reviewed-exact-match")

    def test_wrong_iso_hash_fails_before_extraction(self):
        self.pack(SYNTHETIC)
        wrong = self.run_check("0" * 64, self.review(SYNTHETIC))
        self.assertEqual(wrong.returncode, 2)
        self.assertFalse(wrong.stdout)
        self.assertIn("refused", wrong.stderr)

    def test_digest_valid_non_iso_refused(self):
        self.iso = self.folder / "fake.iso"
        self.iso.write_bytes(b"inert bytes")
        self.digest = hashlib.sha256(self.iso.read_bytes()).hexdigest()
        no_squash = self.run_check(self.digest)
        self.assertEqual(no_squash.returncode, 2)
        self.assertFalse(no_squash.stdout)
