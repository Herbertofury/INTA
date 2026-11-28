import copy
import json
import unittest

from test_hub import ROOT, build


class PublicBusinessProfiles(unittest.TestCase):
    def setUp(self):
        self.profiles = json.loads((ROOT / 'data/meta.json').read_text())['businessProfiles']

    def test_exact_reviewed_profiles_are_embedded_in_actual_hub(self):
        self.assertEqual(self.profiles, [
            {'id': 'company', 'url': 'https://intacomputerservices.com/'},
            {'id': 'nextdoor', 'url': 'https://nextdoor.com/pages/inta-computer-services-northglenn-co/'},
        ])
        build.validate_business_profiles(self.profiles)
        outputs = build.build_outputs()
        self.assertEqual(json.loads(outputs['site/catalog.json'])['meta']['businessProfiles'], self.profiles)
        html = outputs['site/index.html']
        for profile in self.profiles:
            self.assertEqual(html.count(profile['url']), 1)
        self.assertIn('#resources?section=find-inta', html)
        self.assertIn('Find INTA Computer Services', html)

    def test_unreviewed_and_noncanonical_data_never_enters_the_public_hub(self):
        for sample in [
            None,
            self.profiles[:1],
            self.profiles + [{'id': 'unreviewed', 'url': 'https://example.com/'}],
            [self.profiles[0], self.profiles[0]],
            [self.profiles[0], {**self.profiles[1], 'url': self.profiles[1]['url'] + '?tracking=1'}],
            [self.profiles[0], {**self.profiles[1], 'url': 'https://account@example.com/'}],
            [self.profiles[0], {**self.profiles[1], 'note': 'not approved for publication'}],
        ]:
            with self.subTest(sample=copy.deepcopy(sample)), self.assertRaises(ValueError):
                build.validate_business_profiles(sample)
