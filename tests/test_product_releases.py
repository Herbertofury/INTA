import copy
import importlib.util
import json
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("build_hub", ROOT / "scripts/build-hub.py")
build = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(build)


class ProductReleases(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.source = json.loads((ROOT / "data/product-releases.json").read_text())
        cls.outputs = build.build_outputs()
        cls.products = json.loads(cls.outputs["site/catalog.json"])["productReleases"]

    def test_identical_pinned_routes_in_landing_pages_and_offline_catalog(self):
        self.assertEqual([p["id"] for p in self.products], ["desktop", "mobile", "rescue"])
        for product in self.products:
            release = product["releaseUrl"]
            tag = product["tag"]
            self.assertEqual(release, f"{build.REPO}/releases/tag/{tag}")
            self.assertTrue(any(a["label"] == "SHA-256 checksums" for a in product["actions"]))
            targets = ["README.md", "wiki/Home.md", "site/index.html"]
            if product["id"] == "rescue":
                targets += ["docs/rescue.md", "docs/rescue-media.md", "wiki/Rescue-Media.md"]
            for name in targets:
                with self.subTest(name=name, product=product["id"]):
                    body = self.outputs[name]
                    self.assertTrue(release in body, f"{name} lacks the current {product['id']} release")
                    self.assertFalse(f"{build.REPO}/releases/latest" in body, f"{name} uses generic latest")
                    self.assertFalse(f"{build.REPO}/releases/tag/mobile-0.2-preview.1" in body, f"{name} uses stale Mobile preview")
                    for action in product["actions"]:
                        self.assertTrue(action["url"].startswith(f"{build.REPO}/releases/download/{tag}/"))
                        self.assertTrue(action["url"] in body, f"{name} lacks {action['url']}")

    def test_release_channels_and_boundaries_are_distinct(self):
        desktop, mobile, rescue = self.products
        self.assertEqual([p["channel"] for p in self.products], ["preview", "preview", "stable"])
        self.assertIn("self-signed", desktop["summary"])
        self.assertIn("Android 8+", mobile["platform"])
        self.assertIn("does not control Desktop or Rescue live", mobile["summary"])
        self.assertIn("Recovery only", rescue["summary"])
        self.assertIn("No antivirus scanner", rescue["summary"])
        self.assertIn("Physical systems", rescue["tested"])

    def test_recovery_guides_name_the_published_ui_actions(self):
        for name in ["docs/rescue.md", "docs/rescue-media.md", "wiki/Rescue-Media.md"]:
            with self.subTest(name=name):
                body = self.outputs[name]
                self.assertTrue("**Download Recovery**" in body)
                self.assertTrue("**Copy checked ISO**" in body)
                self.assertTrue("historical" in body)

    def test_rejects_generic_tags_duplicate_and_injected_assets(self):
        cases = [
            ("tag", "latest", "exact release tag"),
            ("tag", "desktop-/../../other", "exact release tag"),
            ("downloads", [{"label": "Open", "file": "../SHA256SUMS"}], "asset filename"),
            ("downloads", [{"label": "Open", "file": "SHA256SUMS"}], "duplicate release asset"),
            ("checksums", None, "asset filename"),
        ]
        for key, value, error in cases:
            source = copy.deepcopy(self.source)
            source["products"][0][key] = value
            with self.subTest(key=key, value=value), self.assertRaisesRegex(ValueError, error):
                build.product_release_links(source)
