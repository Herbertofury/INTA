"""Integrity and current Command Center coverage for web-only brand identities."""
import base64
import copy
import hashlib
import importlib.util
import json
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("web_brand_assets", ROOT / "scripts/brand_icons.py")
BRANDS = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(BRANDS)
BUILD_SPEC = importlib.util.spec_from_file_location("web_brand_builder", ROOT / "scripts/build-hub.py")
BUILD = importlib.util.module_from_spec(BUILD_SPEC)
BUILD_SPEC.loader.exec_module(BUILD)


class WebBrandIcons(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.base = BRANDS.load_registry(ROOT)
        cls.extension = json.loads((ROOT / "data/web-brand-icons.json").read_text())
        cls.merged = BUILD.merge_web_brands(cls.base, cls.extension)
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
                    for key, value in self.base["assets"][asset_id].items():
                        if key != "alternateSources":
                            self.assertEqual(self.merged["assets"][asset_id][key], value)
                else:
                    self.assertTrue(asset["path"].startswith("assets/web-brand-icons/"))

    def test_identical_content_keeps_provenance_and_rejects_core_conflicts(self):
        identity = "i4538691198fe0ea7526e"
        base = copy.deepcopy(self.base)
        extension = copy.deepcopy(self.extension)
        original_base = copy.deepcopy(base)
        original_extension = copy.deepcopy(extension)
        source = {key: value for key, value in extension["assets"][identity].items()
                  if key not in {"path", "sha256", "mimeType", "width", "height"}}
        base["assets"][identity]["alternateSources"] = [{**source, "checkedAt": "2026-11-25"}]
        extension["assets"][identity]["alternateSources"] = [source, {**source, "checkedAt": "2026-11-24"}]
        expected_base = copy.deepcopy(base)
        expected_extension = copy.deepcopy(extension)
        merged = BUILD.merge_web_brands(base, extension)
        combined = merged["assets"][identity]
        self.assertEqual(combined["sourceUrl"], original_base["assets"][identity]["sourceUrl"])
        self.assertNotEqual(combined["sourceUrl"], source["sourceUrl"])
        self.assertEqual(combined["alternateSources"],
                         [base["assets"][identity]["alternateSources"][0], source,
                          extension["assets"][identity]["alternateSources"][1]])
        self.assertEqual(base, expected_base)
        self.assertEqual(extension, expected_extension)
        self.assertEqual(BRANDS.validate_registry(merged, ROOT), [])
        for key, value in (("sha256", "f" * 64), ("path", "assets/web-brand-icons/other.png"),
                           ("path", "../outside.png"), ("mimeType", "image/jpeg"),
                           ("width", 32), ("height", 32)):
            with self.subTest(conflict=key, value=value):
                broken = copy.deepcopy(original_extension)
                broken["assets"][identity][key] = value
                with self.assertRaisesRegex(ValueError, "Conflicting web brand asset"):
                    BUILD.merge_web_brands(original_base, broken)
        for source_url in ("http://www.microsoft.com/favicon.ico", "https://user:secret@example.com/icon.png"):
            with self.subTest(unsafe_source=source_url):
                broken = copy.deepcopy(original_extension)
                broken["assets"][identity]["sourceUrl"] = source_url
                with self.assertRaisesRegex(ValueError, "Invalid web brand asset provenance"):
                    BUILD.merge_web_brands(original_base, broken)

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
