"""Check the standalone creator, its portable package and the real Recovery selector."""
import importlib.util
import json
from pathlib import Path
import runpy
import sys
import tempfile
import unittest
from unittest.mock import patch
import zipimport

ROOT = Path(__file__).resolve().parents[1]


def load_portable(archive):
    loader = zipimport.zipimporter(str(archive))
    namespace = {"__name__": "inta_portable_test", "__loader__": loader,
                 "__file__": str(archive / "__main__.py")}
    with patch.object(sys, "path", [str(archive), *sys.path]):
        exec(loader.get_code("__main__"), namespace)
    return namespace


class RecoveryMediaTests(unittest.TestCase):
    def setUp(self):
        spec = importlib.util.spec_from_file_location("standalone_media", ROOT / "media/inta-media.py")
        self.media = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(self.media)
        self.pin = json.loads((ROOT / "data/rescue-recovery-download-channel.json").read_bytes())

    def test_recovery_pin_and_shared_downloader(self):
        media = self.media
        self.assertEqual(media.download_channel(media.products.RECOVERY), self.pin)
        with tempfile.TemporaryDirectory() as work:
            dest = Path(work)
            with patch.object(media.products, "fetch_product", return_value=dest) as fetch:
                self.assertEqual(media.download_release(dest, self.pin, media.products.RECOVERY), dest)
                fetch.assert_called_once_with(dest, self.pin, media.products.RECOVERY)
                fetch.reset_mock()
                for changed in (dict(self.pin, sha256="1"*64), media.download_channel()):
                    with self.assertRaises(ValueError):
                        media.download_release(dest, changed, media.products.RECOVERY)
                fetch.assert_not_called()

    def test_pyz_is_reproducible_and_contains_both_pins(self):
        package = runpy.run_path(str(ROOT / "scripts/package-media.py"))["package"]
        with tempfile.TemporaryDirectory() as work:
            first = Path(work) / "creator.pyz"
            second = Path(work) / "repeat.pyz"
            self.assertEqual(package(first)["sha256"], package(second)["sha256"])
            app = load_portable(first)
            self.assertEqual(app["download_channel"]("recovery-only"), self.pin)
            self.assertEqual(app["download_channel"](), self.media.download_channel())
            self.assertEqual(app["products"].RECOVERY_ISO, self.pin["filename"])

    def test_ventoy_preserves_an_existing_recovery_image_on_failed_readback(self):
        import hashlib
        media = self.media
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            volume = root / "usb"
            volume.mkdir()
            source = root / self.pin["filename"]
            source.write_bytes(b"not an ISO")
            digest = hashlib.sha256(source.read_bytes()).hexdigest()
            (root / media.SUMS).write_text(f"{digest}  {source.name}\n")
            old = volume / source.name
            old.write_bytes(b"previous verified ISO")
            pin = dict(self.pin, bytes=source.stat().st_size, sha256=digest)
            with patch.object(media.os.path, "ismount", return_value=True), patch.object(media, "download_channel", return_value=pin):
                with self.assertRaises(ValueError):
                    media.copy_to_ventoy(source, volume)
            self.assertEqual(list(volume.iterdir()), [old])
            self.assertEqual(old.read_bytes(), b"previous verified ISO")


if __name__ == "__main__":
    unittest.main()
