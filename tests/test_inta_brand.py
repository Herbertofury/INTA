import base64
import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import unittest

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("inta_brand", ROOT / "scripts/refresh-inta-brand.py")
brand = importlib.util.module_from_spec(spec)
spec.loader.exec_module(brand)


class BrandContract(unittest.TestCase):
    def test_derived_markdown_and_windows_assets_are_current(self):
        subprocess.run([sys.executable, str(ROOT / "scripts/refresh-inta-brand.py"), "--check"], check=True)
        files = brand.canonical_inputs(ROOT)
        self.assertIn((256, 256), brand.ico_sizes(files[brand.ICO]))
        heritage = base64.b64encode(files[f"{brand.ASSETS}/heritage-192.png"])
        for hero in ("inta-banner.svg", "wiki-hero.svg"):
            data = (ROOT / "docs/assets" / hero).read_bytes()
            self.assertIn(heritage, data)
            self.assertNotIn(base64.b64encode(files[brand.icon_source()]), data)

    def test_original_archive_is_sealed_and_matches_working_inputs(self):
        source = ROOT / brand.ASSETS / "source"
        filename = "INTA-Complete-Icon-Pack.zip"
        expected, name = (source / "SHA256SUMS").read_text(encoding="ascii").split()
        self.assertEqual(name, filename)
        self.assertEqual(hashlib.sha256((source / filename).read_bytes()).hexdigest(), expected)
        self.assertEqual(brand.pack_inputs(source / filename), brand.canonical_inputs(ROOT))

    def test_route_and_native_chrome_use_application(self):
        files = brand.canonical_inputs(ROOT)
        inline = (ROOT / brand.THUMB).read_bytes()
        self.assertEqual(inline, files[brand.icon_source()])
        a = (ROOT / "native/workbench/desktop/build.ps1").read_text()
        self.assertIn("application-64.png", a)
        self.assertNotIn("heritage-192.png", a)
        for file in ("JobsActivity.java", "JobActivity.java"):
            src = (ROOT / "mobile/android/src/com/inta/mobile" / file).read_text()
            self.assertIn("mark.setImageResource(R.mipmap.inta_application)", src)
        self.assertIn("application-192.png", (ROOT / "scripts/build-rescue-wallpaper.py").read_text())

    def test_pwa_uses_application_art_and_verified_cache(self):
        site = ROOT / "site"
        manifest = json.loads((site / "manifest.webmanifest").read_text())
        service_worker = (site / "sw.js").read_text()
        self.assertIn("inta-touch-icon.png", service_worker)
        sources = brand.canonical_inputs(ROOT)
        self.assertIn("inta-shell-" + brand.brand_cache_version(sources), service_worker)
        for name in sources:
            self.assertNotEqual(brand.brand_cache_version(sources),
                                brand.brand_cache_version({**sources, name: b"different"}))
        self.assertTrue(any(item["src"] == "./inta-touch-icon.png" and item["sizes"] == "192x192"
                            for item in manifest["icons"]))
        self.assertEqual((site / "inta-touch-icon.png").read_bytes(),
                         (ROOT / brand.ASSETS / "application-192.png").read_bytes())
        self.assertIn(base64.b64encode((ROOT / brand.ASSETS / "application-512.png").read_bytes()),
                      (site / "inta-app-icon.svg").read_bytes())

    def test_generated_image_payloads_refuse_bad_headers(self):
        for invalid in (b"", b"<svg/>", b"\x89PNG\r\n\x1a\n"):
            with self.subTest(invalid=invalid), self.assertRaises(ValueError):
                brand.png_size(invalid)
        with self.assertRaises(ValueError):
            brand.ico_sizes((ROOT / brand.ICO).read_bytes()[:50])


if __name__ == "__main__":
    unittest.main()
