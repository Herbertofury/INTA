import importlib.util
import io
import hashlib
from pathlib import Path
import tempfile
import unittest

spec = importlib.util.spec_from_file_location(
    "media", Path(__file__).resolve().parents[1] / "media" / "inta-media.py")
media = importlib.util.module_from_spec(spec)
spec.loader.exec_module(media)


class MediaTests(unittest.TestCase):
    def test_checksum_identifies_exactly_one_iso(self):
        digest = "a" * 64
        entry = f"{digest}  {media.ISO}"
        self.assertEqual(media.expected_digest(entry), digest)
        for invalid in (entry + "\n" + entry, entry + ".bad", "none"):
            with self.assertRaises(ValueError):
                media.expected_digest(invalid)

    def test_failed_verification_preserves_previous_image(self):
        with tempfile.TemporaryDirectory() as folder:
            target = Path(folder) / media.ISO
            target.write_bytes(b"previous")
            with self.assertRaises(ValueError):
                media.stream_verified(io.BytesIO(b"wrong"), target, "a" * 64)
            self.assertEqual(target.read_bytes(), b"previous")
            self.assertEqual(list(Path(folder).iterdir()), [target])

    def test_valid_image_replaces_previous_image(self):
        with tempfile.TemporaryDirectory() as folder:
            target = Path(folder) / media.ISO
            image = b"new image"
            target.write_bytes(b"old image")
            media.stream_verified(io.BytesIO(image), target, hashlib.sha256(image).hexdigest(), len(image))
            self.assertEqual(target.read_bytes(), image)
