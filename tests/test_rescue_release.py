import hashlib
import importlib.util
import json
import os
from pathlib import Path
import stat
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("rescue_release", ROOT / "scripts/package-rescue-release.py")
RELEASE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(RELEASE)


class RescueReleasePackaging(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.source = self.root / "input"
        self.source.mkdir()
        self.output = self.root / "release"
        self.image = bytes(range(256)) * 5 + b"original ISO tail"
        self.portable = b"#!/usr/bin/env python3\nverified portable program\n"
        self.packages = b"example-package 1.0 amd64\n"
        (self.source / RELEASE.ISO).write_bytes(self.image)
        (self.source / RELEASE.PORTABLE).write_bytes(self.portable)
        (self.source / RELEASE.PORTABLE).chmod(0o755)
        os.utime(self.source / RELEASE.PORTABLE, (1700000000, 1700000000))
        (self.source / RELEASE.PACKAGES).write_bytes(self.packages)
        self.write_checksums()

    def write_checksums(self):
        (self.source / RELEASE.SUMS).write_text("".join(
            f"{hashlib.sha256((self.source / name).read_bytes()).hexdigest()}  {name}\n"
            for name in (RELEASE.ISO, RELEASE.PORTABLE)), encoding="ascii")

    def package_split(self, **kwargs):
        return RELEASE.package_release(self.source, self.output, part_size=512,
                                       asset_limit=1024, **kwargs)

    def assert_output_checksums(self):
        hashes = {}
        for line in (self.output / RELEASE.SUMS).read_text().splitlines():
            digest, name = line.split("  ", 1)
            self.assertNotIn(name, hashes)
            hashes[name] = digest
        self.assertEqual(hashes[RELEASE.ISO], hashlib.sha256(self.image).hexdigest())
        for path in self.output.iterdir():
            if path.name != RELEASE.SUMS:
                self.assertEqual(hashes[path.name], hashlib.sha256(path.read_bytes()).hexdigest())

    def assert_no_publication(self):
        self.assertFalse(self.output.exists())
        self.assertEqual(list(self.root.iterdir()), [self.source])

    def test_ordinary_iso_keeps_existing_download_assets_and_executable_mode(self):
        report = RELEASE.package_release(self.source, self.output)
        self.assertFalse(report["split"])
        self.assertEqual((self.output / RELEASE.ISO).read_bytes(), self.image)
        self.assertEqual((self.output / RELEASE.PORTABLE).read_bytes(), self.portable)
        self.assertEqual((self.output / RELEASE.PACKAGES).read_bytes(), self.packages)
        self.assertEqual({path.name for path in self.output.iterdir()},
                         {RELEASE.ISO, RELEASE.PORTABLE, RELEASE.PACKAGES, RELEASE.SUMS})
        self.assertEqual(stat.S_IMODE((self.output / RELEASE.PORTABLE).stat().st_mode), 0o755)
        self.assertEqual((self.output / RELEASE.PORTABLE).stat().st_mtime_ns,
                         (self.source / RELEASE.PORTABLE).stat().st_mtime_ns)
        self.assert_output_checksums()

    def test_split_manifest_reassembles_every_original_byte_and_verifies_all_assets(self):
        report = self.package_split()
        self.assertTrue(report["split"])
        manifest = json.loads((self.output / RELEASE.PARTS).read_text())
        self.assertEqual(set(manifest), {"schemaVersion", "filename", "size", "sha256", "parts"})
        self.assertEqual(manifest["schemaVersion"], 1)
        self.assertEqual(manifest["filename"], RELEASE.ISO)
        self.assertEqual(manifest["size"], len(self.image))
        self.assertEqual(manifest["sha256"], hashlib.sha256(self.image).hexdigest())
        self.assertFalse((self.output / RELEASE.ISO).exists())
        joined = bytearray()
        for index, part in enumerate(manifest["parts"], 1):
            self.assertEqual(set(part), {"name", "size", "sha256"})
            self.assertEqual(part["name"], f"{RELEASE.ISO}.part{index:03d}")
            data = (self.output / part["name"]).read_bytes()
            self.assertEqual(part["size"], len(data))
            self.assertTrue(0 < len(data) <= 512)
            self.assertEqual(part["sha256"], hashlib.sha256(data).hexdigest())
            joined.extend(data)
        self.assertEqual(joined, self.image)
        self.assertEqual(report["part_count"], len(manifest["parts"]))
        self.assertTrue(all(path.stat().st_size < 1024 for path in self.output.iterdir()))
        self.assert_output_checksums()

    def test_exact_asset_limit_splits_without_an_empty_trailing_part(self):
        self.image = self.image[:1024]
        (self.source / RELEASE.ISO).write_bytes(self.image)
        self.write_checksums()
        report = self.package_split()
        manifest = json.loads((self.output / RELEASE.PARTS).read_text())
        self.assertEqual(report["part_count"], 2)
        self.assertEqual([part["size"] for part in manifest["parts"]], [512, 512])

    def test_below_asset_limit_keeps_normal_iso(self):
        self.image = self.image[:1023]
        (self.source / RELEASE.ISO).write_bytes(self.image)
        self.write_checksums()
        self.assertFalse(self.package_split()["split"])
        self.assertEqual((self.output / RELEASE.ISO).read_bytes(), self.image)

    def test_same_inputs_produce_identical_release_asset_bytes(self):
        self.package_split()
        other = self.root / "second-release"
        RELEASE.package_release(self.source, other, part_size=512, asset_limit=1024)
        self.assertEqual({p.name: p.read_bytes() for p in self.output.iterdir()},
                         {p.name: p.read_bytes() for p in other.iterdir()})

    def test_wrong_original_iso_or_portable_checksum_never_publishes_partial_output(self):
        for name in (RELEASE.ISO, RELEASE.PORTABLE):
            with self.subTest(name=name):
                self.write_checksums()
                path = self.source / name
                original = path.read_bytes()
                path.write_bytes(original + b"corruption")
                with self.assertRaises(ValueError):
                    self.package_split()
                self.assert_no_publication()
                self.assertEqual(path.read_bytes(), original + b"corruption")
                path.write_bytes(original)

    def test_existing_output_directory_file_or_symlink_is_preserved(self):
        for kind in ("directory", "file", "symlink"):
            with self.subTest(kind=kind):
                if kind == "directory":
                    self.output.mkdir()
                elif kind == "file":
                    self.output.write_bytes(b"keep this output")
                else:
                    self.output.symlink_to(self.root / "missing")
                with self.assertRaises(FileExistsError):
                    self.package_split()
                if kind == "directory":
                    self.assertEqual(list(self.output.iterdir()), [])
                    self.output.rmdir()
                elif kind == "file":
                    self.assertEqual(self.output.read_bytes(), b"keep this output")
                    self.output.unlink()
                else:
                    self.assertTrue(self.output.is_symlink())
                    self.output.unlink()

    def test_oversized_companion_asset_is_rejected_without_publication(self):
        (self.source / RELEASE.PORTABLE).write_bytes(b"x" * 1024)
        self.write_checksums()
        with self.assertRaisesRegex(ValueError, "Release asset must be smaller"):
            self.package_split()
        self.assert_no_publication()

    def test_invalid_checksums_and_symlink_inputs_are_rejected(self):
        original = (self.source / RELEASE.SUMS).read_text()
        for invalid in (original + original, original.replace(RELEASE.ISO, "../" + RELEASE.ISO),
                        "", original.splitlines()[0] + "\n"):
            with self.subTest(invalid=invalid):
                (self.source / RELEASE.SUMS).write_text(invalid)
                with self.assertRaises(ValueError):
                    self.package_split()
                self.assert_no_publication()
        (self.source / RELEASE.SUMS).write_text(original)
        (self.source / RELEASE.ISO).unlink()
        (self.source / RELEASE.ISO).symlink_to(self.source / RELEASE.PORTABLE)
        with self.assertRaises(ValueError):
            self.package_split()
        self.assert_no_publication()

    def test_absent_optional_package_manifest_is_not_invented(self):
        (self.source / RELEASE.PACKAGES).unlink()
        report = self.package_split()
        self.assertIsNone(report["package_manifest"])
        self.assertFalse((self.output / RELEASE.PACKAGES).exists())

    def test_invalid_part_limits_cannot_relax_the_real_release_limit(self):
        for value in (0, -1, True, RELEASE.MAX_PART_SIZE + 1):
            with self.subTest(value=value):
                with self.assertRaises(ValueError):
                    RELEASE.package_release(self.source, self.output, part_size=value)
                self.assert_no_publication()
        with self.assertRaises(ValueError):
            RELEASE.package_release(self.source, self.output, asset_limit=RELEASE.ASSET_LIMIT + 1)
        self.assert_no_publication()


if __name__ == "__main__":
    unittest.main()
