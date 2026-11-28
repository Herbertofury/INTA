"""Verify the public offline desktop and its SHA-addressed technician guide."""
import base64
import hashlib
import json
from pathlib import Path
import re
import runpy
import shutil
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]


class OfflineReleaseTests(unittest.TestCase):
    def test_reviewed_artwork_preserves_the_full_catalog_and_links(self):
        with tempfile.TemporaryDirectory() as name:
            dest = Path(name)
            shutil.copytree(ROOT / "site", dest / "site")
            (dest / "wiki").mkdir()
            (dest / "docs").mkdir()
            shutil.copyfile(ROOT / "wiki/Home.md", dest / "wiki/Home.md")
            shutil.copyfile(ROOT / "docs/rescue.md", dest / "docs/rescue.md")
            before = json.loads((dest / "site/catalog.json").read_bytes())
            guide = (dest / "docs/rescue.md").read_text(encoding="utf-8")
            runpy.run_path(str(ROOT / "native/release/offline.py"))["offline_guides"](dest)
            runpy.run_path(str(ROOT / "native/release/offline_artwork.py"))["apply_release_artwork"](dest)
            after = json.loads((dest / "site/catalog.json").read_bytes())
            self.assertEqual({k:v for k,v in before.items() if k!="brands"},
                             {k:v for k,v in after.items() if k!="brands"})
            def records(data):
                return [{k:v for k,v in b.items() if k!="assetId"} for b in data["brands"]["brands"]]
            self.assertEqual(records(before), records(after))
            assets = after["brands"]["assets"]
            self.assertEqual(set(assets), {"i70ad9f0981b05341142b", "inta-official-link-indicator"})
            self.assertFalse(after["brands"]["assetUrls"])
            self.assertTrue(all(b["assetId"] in assets for b in after["brands"]["brands"]))
            self.assertTrue(all(v.startswith(("data:image/png;base64,", "data:image/svg+xml;base64,")) for v in assets.values()))
            for value in assets.values():
                self.assertTrue(base64.b64decode(value.partition(",")[2], validate=True))
            page = (dest / "site/index.html").read_text(encoding="utf-8")
            match = re.search(r'<script id="inta-data">window.INTA_DATA = (.*?);</script>', page, re.S)
            self.assertEqual(json.loads(match[1]), after)
            rewritten = (dest / "docs/rescue.md").read_text(encoding="utf-8")
            self.assertEqual(re.findall(r'<a href="([^"]+)"', guide), re.findall(r'<a href="([^"]+)"', rewritten))
            self.assertNotRegex(rewritten, r'<img[^>]+src="https://[^"]*/(?:web-)?brand-icons/')
            self.assertIn("&#8599;", rewritten)
            self.assertIn("../site/inta-app-icon.svg", rewritten)
            manifest = json.loads((dest / "site/build-manifest.json").read_bytes())
            self.assertEqual(manifest["artworkProfile"], "first-party-release")
            for path, digest in manifest["files"].items():
                self.assertEqual(hashlib.sha256((dest / path).read_bytes()).hexdigest(), digest)


if __name__ == "__main__":
    unittest.main()
