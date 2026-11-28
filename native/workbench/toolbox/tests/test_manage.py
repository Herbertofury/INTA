import sys
import tempfile
import unittest
import zipfile
from pathlib import Path
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
import manage
import relocate


class PackageSafety(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.base = Path(self.temp.name)
        self.root = self.base / "source"
        manage.init(self.root)

    def test_unsafe_names_and_redirects(self):
        for bad in (
            "..",
            "folder/../file",
            "C:/path",
            "/path",
            "a//b",
            "x:ads",
            "z./q",
            "\\\\share",
        ):
            with self.subTest(bad=bad), self.assertRaises(manage.Failure):
                manage.within(self.root, bad)
        for url in (
            "http://github.com/x",
            "https://github.com.evil.test/a",
            "https://github.com@evil.test/x",
            "https://github.com:123/x",
        ):
            with self.subTest(url=url), self.assertRaises(manage.Failure):
                manage.validate_url(url)

    def test_unmarked_data_preserved(self):
        other = self.base / "legacy"
        other.mkdir()
        (other / "owner-note.txt").write_text("keep")
        with self.assertRaises(manage.Failure):
            manage.init(other)
        self.assertEqual((other / "owner-note.txt").read_text(), "keep")

    def test_zip_path_entries_cannot_decide_destination(self):
        archive = self.base / "fixture.zip"
        with zipfile.ZipFile(archive, "w") as z:
            z.writestr("../escape", b"bad")
            z.writestr("good", b"ok")
        target = self.base / "copy"
        import hashlib

        manage.unpack(
            archive,
            {"member": "good", "exeSha256": hashlib.sha256(b"ok").hexdigest()},
            target,
        )
        self.assertEqual(target.read_bytes(), b"ok")
        self.assertFalse((self.base / "escape").exists())

    def test_duplicate_zip_member_blocked(self):
        archive = self.base / "fixture.zip"
        with zipfile.ZipFile(archive, "w") as z:
            z.writestr("good", b"ok")
            with self.assertWarns(UserWarning):
                z.writestr("good", b"other")
        with self.assertRaises(manage.Failure):
            manage.unpack(
                archive, {"member": "good", "exeSha256": "0" * 64}, self.base / "copy"
            )

    def test_interrupted_relocation_resumes_without_data_loss(self):
        (self.root / "My technician notes.txt").write_text("custom", encoding="utf-8")
        dest = self.base / "@工具 portable"
        original = relocate.shutil.copy2
        calls = []

        def interrupt(*args, **kwargs):
            calls.append(True)
            if len(calls) == 2:
                raise OSError("interrupted fixture")
            return original(*args, **kwargs)

        with (
            patch.object(relocate.shutil, "copy2", interrupt),
            self.assertRaises(OSError),
        ):
            relocate.copy(self.root, dest)
        self.assertFalse(dest.exists())
        self.assertTrue((self.root / "My technician notes.txt").is_file())
        result = relocate.copy(self.root, dest)
        self.assertEqual(result["toolboxId"], manage.identity(self.root)["toolboxId"])
        self.assertEqual((dest / "My technician notes.txt").read_text(), "custom")
        self.assertEqual(manage.identity(dest), manage.identity(self.root))
        with self.assertRaises(manage.Failure):
            relocate.copy(self.root, dest)

    def test_reorganized_package_and_duplicate_collision(self):
        import hashlib

        pin = {
            "id": "fixture",
            "version": "1",
            "exeSha256": hashlib.sha256(b"ok").hexdigest(),
            "executable": "demo.exe",
        }
        for label in ["@Tést", "Second copy"]:
            version = self.root / label
            (version / "PORTABLE").mkdir(parents=True)
            (version / "PORTABLE/demo.exe").write_bytes(b"ok")
            manage.dump(version / "tool.json", pin)
            with patch.object(manage, "PINS", [pin]):
                if label == "@Tést":
                    idx = manage.reindex(self.root)
                    self.assertEqual(
                        idx["packages"]["fixture"], "@Tést/PORTABLE/demo.exe"
                    )
                else:
                    with self.assertRaises(manage.Failure):
                        manage.reindex(self.root)
                    self.assertEqual(manage.index(self.root), idx)


if __name__ == "__main__":
    unittest.main()
