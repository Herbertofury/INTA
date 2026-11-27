import copy
import json
import unittest

from test_hub import ROOT, build

class NewsCatalogTest(unittest.TestCase):
    def setUp(self):
        self.catalog = json.loads((ROOT / 'data/news-sources.json').read_text())
    def test_real_catalog_and_embedded_client_snapshot(self):
        build.validate_news(self.catalog)
        self.assertEqual({s['category'] for s in self.catalog['sources']},
                         {'advisory','compliance','vendor','reporting','community','status'})
        data = json.loads(build.build_outputs()['site/catalog.json'])
        self.assertEqual(data['news'], self.catalog)
    def test_rejects_bad_links_unknown_source_and_future_headlines(self):
        for kind, value in [('url', 'javascript:alert(1)'), ('feed', 'https://fake@evil.example/'), ('category', 'unknown')]:
            modified = copy.deepcopy(self.catalog)
            modified['sources'][0][kind] = value
            with self.subTest(kind=kind), self.assertRaises(ValueError):
                build.validate_news(modified)
        for kind,value in [('sourceId','nonexistent'), ('date','2099-01-01')]:
            modified = copy.deepcopy(self.catalog)
            modified['briefing'][0][kind] = value
            with self.subTest(kind=kind), self.assertRaises(ValueError):
                build.validate_news(modified)
