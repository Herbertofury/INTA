"""Integrity and current Command Center coverage for web-only brand identities."""
import base64
import hashlib
import importlib.util
import json
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("web_brand_assets", ROOT / "scripts/brand_icons.py")
BRANDS = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(BRANDS)


class WebBrandIcons(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.base = BRANDS.load_registry(ROOT)
        cls.extension = json.loads((ROOT / "data/web-brand-icons.json").read_text())
        cls.merged = dict(cls.base, assets={**cls.base["assets"], **cls.extension["assets"]},
                          brands=[*cls.extension["brands"], *cls.base["brands"]])
        cls.center = json.loads((ROOT / "data/command-center.json").read_text())

    def test_all_cached_images_have_valid_bytes_hashes_and_provenance(self):
        self.assertEqual(BRANDS.validate_registry(self.extension, ROOT), [])
        for brand in self.extension["brands"]:
            with self.subTest(brand=brand["id"]):
                self.assertIn(brand["name"], brand["names"])
                data_url = BRANDS.asset_data_url(self.merged, brand["assetId"], ROOT)
                raw = base64.b64decode(data_url.split(",", 1)[1], validate=True)
                self.assertEqual(hashlib.sha256(raw).hexdigest(),
                                 self.extension["assets"][brand["assetId"]]["sha256"])

    def test_extension_preserves_native_assets_and_identity_ids(self):
        self.assertFalse({b["id"] for b in self.base["brands"]} &
                         {b["id"] for b in self.extension["brands"]})
        for asset_id, asset in self.extension["assets"].items():
            with self.subTest(asset=asset_id):
                if asset_id in self.base["assets"]:
                    self.assertEqual(asset, self.base["assets"][asset_id])
                else:
                    self.assertTrue(asset["path"].startswith("assets/web-brand-icons/"))

    def test_every_command_center_toolkit_resolves_to_its_identity(self):
        expected = {"sysinternals": "sysinternals", "winutil": "web-app-winutil",
                    "tron": "web-app-tron", "chocolatey": "web-app-chocolatey",
                    "winget": "web-app-winget", "scoop": "web-app-scoop",
                    "unigetui": "web-app-unigetui", "ninite-pro": "web-app-ninite-pro",
                    "patch-my-pc-home-updater": "web-app-patch-my-pc-home-updater"}
        self.assertEqual(set(expected), {item["id"] for item in self.center["toolkits"]})
        for item in self.center["toolkits"]:
            with self.subTest(toolkit=item["name"]):
                brand = BRANDS.resolve_brand(self.merged, name=item["name"],
                                             url=item["url"], tool_id=item["id"])
                self.assertIsNotNone(brand)
                self.assertEqual(brand["id"], expected[item["id"]])
                self.assertIn(brand["assetId"], self.merged["assets"])

    def test_every_current_command_source_has_a_genuine_cached_icon(self):
        for command in self.center["commands"]:
            with self.subTest(command=command["id"]):
                brand = BRANDS.resolve_brand(self.merged, name=command["sourceTitle"],
                                             url=command["sourceUrl"])
                self.assertIsNotNone(brand)
                self.assertIn(brand["assetId"], self.merged["assets"])

    def test_project_routes_and_web_override_keep_the_correct_identity(self):
        tron = BRANDS.resolve_brand(self.merged, name="Tron Script")
        self.assertEqual(tron["downloadPage"], "https://www.reddit.com/r/TronScript/")
        unigetui = BRANDS.resolve_brand(self.merged, name="UniGetUI", tool_id="unigetui")
        self.assertEqual(unigetui["downloadPage"], "https://github.com/Devolutions/UniGetUI/releases/latest")
        self.assertIn("/Devolutions/UniGetUI/main/media/icon.png",
                      self.merged["assets"][unigetui["assetId"]]["sourceUrl"])
        github = BRANDS.resolve_brand(self.base, name="github.com")
        self.assertNotEqual(unigetui["assetId"], github["assetId"])

    def test_current_package_manager_and_platform_names_resolve(self):
        for name in ("WinUtil", "Tron", "Chocolatey", "Scoop", "UniGetUI",
                     "Windows Package Manager (WinGet)", "APT", "DNF", "Pacman",
                     "pacman-contrib", "checkupdates", "Arch Linux", "Arch",
                     "Fedora", "RHEL", "Microsoft Store", "Ubuntu Manpages",
                     "ArchWiki", "Chocolatey Documentation", "DNF Documentation",
                     "Ninite Pro", "Ninite Pro Classic", "NinitePro",
                     "Patch My PC Home Updater", "Patch My PC"):
            with self.subTest(name=name):
                self.assertIsNotNone(BRANDS.resolve_brand(self.merged, name=name))

    def test_concurrent_toolkits_keep_their_exact_product_pages(self):
        ninite = BRANDS.resolve_brand(self.merged, name="Ninite Pro Classic")
        patchmypc = BRANDS.resolve_brand(self.merged, name="Patch My PC")
        self.assertEqual(ninite["downloadPage"].rstrip("/"), "https://ninite.com/pro")
        self.assertEqual(patchmypc["downloadPage"].rstrip("/"),
                         "https://patchmypc.com/product/home-updater")
        for name, source in (("NinitePro", "https://ninite.com/help/features/selection.html"),
                             ("Patch My PC Home Updater", "https://patchmypc.com/faq/category/home-updater/")):
            self.assertEqual(BRANDS.resolve_brand(self.merged, name=name)["id"],
                             BRANDS.resolve_brand(self.merged, url=source)["id"])


if __name__ == "__main__":
    unittest.main()
