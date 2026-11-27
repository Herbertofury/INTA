"""Authentic image coverage and offline identity regressions."""
import base64
import hashlib
import json
from pathlib import Path
import struct
import sys
import unittest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'scripts'))
from brand_icons import load_registry, portal_icons, resolve_brand, validate_registry


class BrandIconTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.registry = load_registry()
        cls.tools = json.loads((ROOT / 'data/tools.json').read_text())
        cls.aliases = json.loads((ROOT / 'data/link-aliases.json').read_text())['links']

    def test_complete_tool_coverage_and_aliases(self):
        for tool in self.tools:
            with self.subTest(tool=tool['id']):
                brand = resolve_brand(self.registry, tool_id=tool['id'])
                self.assertIsNotNone(brand)
                self.assertIn(brand['assetId'], self.registry['assets'])
                self.assertEqual(resolve_brand(self.registry, name=tool['name'])['toolId'], tool['id'])
        for alias in self.aliases:
            if alias.get('kind') in ('method', 'reference'):
                continue
            for name in [alias['name'], *alias.get('aliases', [])]:
                with self.subTest(alias=name):
                    self.assertIsNotNone(resolve_brand(self.registry, name=name))

    def test_images_have_real_bytes_provenance_and_dimensions(self):
        self.assertEqual(validate_registry(self.registry), [])
        for asset in self.registry['assets'].values():
            with self.subTest(path=asset['path']):
                raw = (ROOT / asset['path']).read_bytes()
                self.assertEqual(raw[:8], b'\x89PNG\r\n\x1a\n')
                w, h = struct.unpack('>II', raw[16:24])
                self.assertEqual((w, h), (asset['width'], asset['height']))
                self.assertEqual(hashlib.sha256(raw).hexdigest(), asset['sha256'])
                self.assertTrue(asset['sourceUrl'].startswith('https://'))
                self.assertTrue(asset['sourceSha256'])

    def test_offline_assets_are_embedded_once_and_round_trip_exactly(self):
        portal = portal_icons(self.registry)
        self.assertEqual(set(portal['assets']), set(self.registry['assets']))
        for key, uri in portal['assets'].items():
            self.assertTrue(uri.startswith('data:image/png;base64,'))
            self.assertEqual(base64.b64decode(uri.split(',', 1)[1]), (ROOT / self.registry['assets'][key]['path']).read_bytes())
        self.assertEqual(len(portal['brands']), len(self.registry['brands']))

    def test_specific_name_beats_shared_host(self):
        pairs = [('PowerShell', 'powershell'), ('GitHub', 'app-github'), ('Rufus', 'rufus'), ('Malwarebytes AdwCleaner', 'adwcleaner')]
        for name, identity in pairs:
            self.assertEqual(resolve_brand(self.registry, name=name, url='https://github.com/')['id'], identity)
        self.assertEqual(resolve_brand(self.registry, tool_id='codex-cli', name='GitHub')['id'], 'codex-cli')

    def test_shared_release_hosts_keep_their_own_website_identity(self):
        github = resolve_brand(self.registry, url='https://github.com/an-unrelated-project/docs')
        self.assertEqual(github['kind'], 'website')
        asset = self.registry['assets'][github['assetId']]
        self.assertIn('github', asset['sourceUrl'])
        self.assertNotIn('meshcentral', asset['sourceUrl'])
        self.assertEqual(resolve_brand(self.registry, name='INTA Rescue')['assetId'], resolve_brand(self.registry, name='Media Creator')['assetId'])

    def test_inta_project_names_use_the_first_party_mark_and_correct_destinations(self):
        by_id = {b['id']: b for b in self.registry['brands']}
        for name in ('INTA', 'INTA Computer Services'):
            brand = resolve_brand(self.registry, name=name)
            self.assertEqual(brand['id'], 'app-inta')
            self.assertEqual(brand['assetId'], by_id['app-inta-rescue']['assetId'])
            self.assertEqual(brand['downloadPage'], 'https://herbertofury.github.io/INTA/')
        self.assertEqual(resolve_brand(self.registry, name='INTA Media Creator')['id'], 'app-media-creator')
        self.assertEqual(by_id['app-inta-rescue']['downloadPage'], 'https://github.com/Herbertofury/INTA/releases')
        self.assertEqual(by_id['site-herbertofury-github-io']['downloadPage'], 'https://herbertofury.github.io/INTA/')

    def test_native_forensics_names_remain_distinct_from_related_products(self):
        yara = resolve_brand(self.registry, name='YARA')
        yarax = resolve_brand(self.registry, name='YARA-X')
        self.assertEqual(yara['id'], 'app-yara')
        self.assertEqual(yara['downloadPage'], 'https://github.com/VirusTotal/yara/releases/latest')
        self.assertEqual(yarax['toolId'], 'yara-x')
        for name in ('Sleuth Kit', 'The Sleuth Kit'):
            brand = resolve_brand(self.registry, name=name)
            self.assertEqual(brand['id'], 'app-sleuth-kit')
            self.assertEqual(brand['downloadPage'], 'https://www.sleuthkit.org/sleuthkit/download.php')
        self.assertEqual(resolve_brand(self.registry, name='Autopsy')['toolId'], 'autopsy')

    def test_merged_service_tools_and_reputation_provider_names_have_exact_identities(self):
        expected = {
            'MangoDisk': 'https://mangodisk.app/download',
            'Kimai': 'https://www.kimai.org/en/download.html',
            'LibreOffice': 'https://www.libreoffice.org/download/',
        }
        for name, route in expected.items():
            brand = resolve_brand(self.registry, name=name)
            self.assertIsNotNone(brand)
            self.assertEqual(brand['downloadPage'], route)
            self.assertIn(brand['assetId'], self.registry['assets'])
        providers = json.loads((ROOT / 'data/reputation-sources.json').read_text())['providers']
        for provider in providers:
            with self.subTest(provider=provider['name']):
                self.assertIsNotNone(resolve_brand(self.registry, name=provider['name']))
        self.assertEqual(resolve_brand(self.registry, name='SourceForge')['assetId'], resolve_brand(self.registry, name='SourceForge Reviews')['assetId'])

    def test_online_tools_keep_their_hosted_launch_pages_and_full_coverage(self):
        access = json.loads((ROOT / 'data/tool-access.json').read_text())['tools']
        for tool in self.tools:
            primary = access[tool['id']].get('primary', {})
            if tool.get('category') != 'online':
                continue
            self.assertEqual(primary.get('kind'), 'web')
            with self.subTest(tool=tool['id']):
                brand = resolve_brand(self.registry, tool_id=tool['id'])
                self.assertEqual(brand['downloadPage'], primary['url'])
                self.assertEqual(resolve_brand(self.registry, name=tool['name'])['toolId'], tool['id'])

    def test_transport_hosts_route_to_a_meaningful_hosted_publisher_page(self):
        by_id = {b['id']: b for b in self.registry['brands']}
        expected = {
            'site-download-sysinternals-com': 'https://learn.microsoft.com/en-us/sysinternals/downloads/sysinternals-suite',
            'site-aka-ms': 'https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/',
            'site-go-microsoft-com': 'https://learn.microsoft.com/en-us/defender-endpoint/safety-scanner-download',
            'site-api-github-com': 'https://github.com/',
            'site-raw-githubusercontent-com': 'https://github.com/',
        }
        for identity, route in expected.items():
            self.assertEqual(by_id[identity]['downloadPage'], route)

    def test_icons_link_to_hosted_pages_without_invented_downloads(self):
        for brand in self.registry['brands']:
            self.assertTrue(brand['downloadPage'].startswith('https://'))
            self.assertNotIn('utm_', brand['downloadPage'])
        self.assertEqual(self.registry['coverage']['unresolved'], [])

if __name__ == '__main__':
    unittest.main()
