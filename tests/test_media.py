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


class RescueChannelTests(unittest.TestCase):
    def test_release_channel_ignores_other_drafts_prereleases_and_metadata_order(self):
        records = [
            self.release("rescue-21.1", published="2026-11-25T13:00:00Z"),
            self.release("online-toolbox-2026-11-26", published="2026-11-27T13:00:00Z"),
            self.release("rescue-20.3", published="2026-11-26T13:00:00Z"),
            self.release("rescue-22.1", published="2026-11-27T13:00:00Z", prerelease=True),
            self.release("rescue-23.1", published="2026-11-27T13:00:00Z", draft=True),
        ]
        self.assertIs(media.select_rescue_release(records), records[2])
        for invalid in (records[1:2], records + [records[2]], [{}] * 41):
            with self.assertRaises(ValueError):
                media.select_rescue_release(invalid)

    @staticmethod
    def release(tag, published="2026-11-27T11:00:00Z", **extra):
        return dict(tag_name=tag, published_at=published, draft=False, prerelease=False, assets=[]) | extra

    def test_download_binds_release_and_keeps_checksums_and_atomic_write(self):
        import json
        from unittest.mock import patch
        tag = "rescue-40.1"
        body = b"owned test image"
        digest = hashlib.sha256(body).hexdigest()
        sums = f"{digest}  {media.ISO}\n".encode()
        base = media.ASSET_ROOT + tag + "/"
        assets = [{"name": n, "browser_download_url": base + n, "size": len(buf)}
                  for n, buf in [(media.ISO, body), (media.SUMS, sums)]]
        records = [self.release("online-toolbox-42"), self.release(tag, assets=assets)]
        responses = {base + media.SUMS: sums, base + media.ISO: body}
        def open_(url, timeout):
            return io.BytesIO(json.dumps(records).encode() if url == media.RELEASE_API else responses[url])
        with tempfile.TemporaryDirectory() as folder, patch.object(media.urllib.request, "urlopen", side_effect=open_) as call:
            target = Path(folder) / media.ISO
            media.download_release(Path(folder))
            self.assertEqual(target.read_bytes(), body)
            self.assertEqual([args.args[0] for args in call.call_args_list],
                             [media.RELEASE_API, base + media.SUMS, base + media.ISO])
            assets[0]["browser_download_url"] = media.ASSET_ROOT + "rescue-39.1/" + media.ISO
            with self.assertRaisesRegex(ValueError, "Unexpected release asset URL"):
                media.download_release(Path(folder))
            self.assertEqual(target.read_bytes(), body)
            records.insert(1, self.release("rescue-41.1"))
            call.reset_mock()
            with self.assertRaisesRegex(ValueError, "missing SHA256SUMS"):
                media.download_release(Path(folder))
            self.assertEqual([args.args[0] for args in call.call_args_list], [media.RELEASE_API])
            self.assertEqual(target.read_bytes(), body)

    def test_wrong_tag_multipart_part_is_refused_before_fetch(self):
        tag = "rescue-41.1"
        digest = hashlib.sha256(b"part").hexdigest()
        name = media.ISO + ".part001"
        manifest = {"schemaVersion": 1, "filename": media.ISO, "size": 4,
                    "sha256": digest, "parts": [{"name": name, "size": 4, "sha256": digest}]}
        assets = {name: {"size": 4, "browser_download_url": media.ASSET_ROOT + "rescue-40.1/" + name}}
        with self.assertRaisesRegex(ValueError, "Unexpected release asset URL"):
            media.verified_parts(manifest, assets, digest, tag)
        assets[name]["browser_download_url"] = media.ASSET_ROOT + tag + "/" + name
        self.assertEqual(media.verified_parts(manifest, assets, digest, tag),
                         (4, [(name, assets[name]["browser_download_url"], 4, digest)]))
