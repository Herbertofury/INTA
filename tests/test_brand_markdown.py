import importlib.util
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[1]


def module(name):
    spec = importlib.util.spec_from_file_location(name, ROOT / 'scripts' / (name + '.py'))
    result = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(result)
    return result


links = module('wiki_links')
icons = module('brand_markdown')


class PublisherIconMarkdown(unittest.TestCase):
    def setUp(self):
        self.registry = {
            'assets': {'publisher': {'path': 'assets/brand-icons/publisher.png'}},
            'brands': [{'id': 'sample', 'name': 'Example Tool', 'names': ['Example Tool'],
                        'assetId': 'publisher', 'kind': 'tool',
                        'downloadPage': 'https://publisher.example/downloads',
                        'urls': ['https://publisher.example/product'], 'hosts': ['publisher.example']}],
        }
        self.render = icons.BrandMarkdown(self.registry, links)

    def test_repeated_mentions_each_have_download_icon_without_losing_source_links(self):
        original = '[Example Tool](https://publisher.example/product) and [Example Tool](https://publisher.example/support/fix).'
        result = self.render.decorate(original)
        self.assertEqual(result.count('data-inta-brand="sample"'), 2)
        self.assertEqual(result.count('href="https://publisher.example/downloads"'), 2)
        self.assertEqual(icons.strip_icons(result), original)
        self.assertEqual(self.render.decorate(result), result)

    def test_commands_diagrams_images_and_raw_urls_are_exactly_preserved(self):
        original = '''`Example Tool` https://publisher.example/product
![Example Tool](https://publisher.example/logo.png)
```mermaid
A["Example Tool"] --> B["Done"]
```
    Example Tool command
<!-- Example Tool -->
'''
        self.assertEqual(self.render.decorate(original), original)

    def test_reference_links_and_balanced_destinations_are_supported(self):
        original = '[Example Tool][manual] and [Example Tool](https://publisher.example/docs_(current)).\n\n[manual]: https://publisher.example/product\n'
        result = self.render.decorate(original)
        self.assertEqual(result.count('data-inta-brand="sample"'), 2)
        self.assertEqual(icons.strip_icons(result), original)

    def test_existing_contextual_html_anchor_gets_separate_download_icon(self):
        original = '<a href="https://publisher.example/support">Example Tool</a>'
        result = self.render.decorate(original)
        self.assertEqual(result.count('data-inta-brand="sample"'), 1)
        self.assertTrue(result.endswith(original))
        self.assertNotIn('<a href="https://publisher.example/support"><a ', result)

    def test_internal_navigation_is_not_mistaken_for_a_publisher(self):
        original = '[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-printer-repair)'
        self.assertEqual(self.render.decorate(original), original)

    def test_existing_illustrated_link_does_not_gain_a_duplicate_image(self):
        original = '<a href="https://publisher.example/product"><img src="icon.png" alt="">Example Tool</a>'
        self.assertEqual(self.render.decorate(original), original)

    def test_lowercase_ordinary_link_label_does_not_become_a_brand(self):
        self.registry['brands'].append(dict(self.registry['brands'][0], id='everything', name='Everything', names=['Everything']))
        render = icons.BrandMarkdown(self.registry, links)
        self.assertEqual(render.decorate('[everything](#complete-catalog)'), '[everything](#complete-catalog)')

    def test_link_only_windows_drops_only_decoration_and_keeps_contextual_urls(self):
        self.registry['brands'].append(dict(
            self.registry['brands'][0], id='app-windows', name='Windows', names=['Windows'],
            inlineIcon=False, downloadPage='https://www.microsoft.com/windows',
            urls=['https://www.microsoft.com/windows']))
        render = icons.BrandMarkdown(self.registry, links)
        original = ('[Windows](https://www.microsoft.com/windows), '
                    '[Windows and Example Tool installer](https://publisher.example/tools.zip), '
                    '<a href="https://publisher.example/support">Download for Windows</a>, '
                    '[System guide](https://www.microsoft.com/windows).')
        old_icon = '<a href="https://www.microsoft.com/windows" data-inta-brand="app-windows"><img src="old.png"></a> '
        result = render.decorate(old_icon + original)
        self.assertNotIn('data-inta-brand="app-windows"', result)
        self.assertEqual(result.count('data-inta-brand="sample"'), 1)
        self.assertEqual(icons.strip_icons(result), original)
        self.assertEqual(render.decorate(result), result)

    def test_current_windows_products_keep_authentic_icons(self):
        from json import loads
        registry = loads((ROOT / 'data' / 'brand-icons.json').read_text(encoding='utf-8'))
        render = icons.BrandMarkdown(registry, links)
        original = ('[Windows 10](https://example.com/example) / '
                    '[Windows 11](https://example.com/example) / '
                    '[Windows Server](https://example.com/example) / '
                    '[Windows Terminal](https://example.com/example) / '
                    '[Windows Update](https://example.com/example) / '
                    '[Windows 11 installation media](https://example.com/example)')
        result = render.decorate(original)
        for generic in ('windows', 'windows10', 'windows11', 'windows-server'):
            self.assertNotIn(f'data-inta-brand="app-{generic}"', result)
        for product in ('windows-terminal', 'app-windows-update', 'microsoft-windows11-media'):
            self.assertIn(f'data-inta-brand="{product}"', result)
        self.assertEqual(icons.strip_icons(result), original)

    def test_executable_download_destination_is_rejected(self):
        self.registry['brands'][0]['downloadPage'] = 'javascript:alert(1)'
        with self.assertRaisesRegex(ValueError, 'public HTTPS'):
            self.render.decorate('[Example Tool](https://publisher.example/product)')

    def test_combined_download_label_preserves_every_product_icon(self):
        self.registry['brands'].append(dict(self.registry['brands'][0], id='second', name='Other Tool', names=['Other Tool'], downloadPage='https://other.example/downloads'))
        render = icons.BrandMarkdown(self.registry, links)
        original = '[Download Example Tool / Other Tool archive](https://publisher.example/tools.zip)'
        result = render.decorate(original)
        self.assertIn('data-inta-brand="sample"', result)
        self.assertIn('data-inta-brand="second"', result)
        self.assertEqual(icons.strip_icons(result), original)
        self.assertEqual(render.decorate(result), result)


if __name__ == '__main__':
    unittest.main()
