import copy
import json
import unittest

from test_hub import ROOT, build


class ReputationEvidence(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.snapshot = json.loads((ROOT / 'data/tool-reputation.json').read_text())
        cls.tools = build.records(ROOT / 'data/tools.json', 'tools')
        cls.checked_at = cls.snapshot['reviewedAt']
        cls.gh = dict(status='observed', url='https://github.com/project/test', checkedAt=cls.checked_at,
                      identitySource='https://owner.example/test', stars=0, forks=0)

    def test_every_record_and_observation_validates(self):
        build.validate_reputation(self.snapshot, self.tools)
        output = json.loads(build.build_outputs()['site/catalog.json'])['tools']
        self.assertEqual(self.snapshot['tools'], {tool['id']: tool['reputation'] for tool in output})

    def test_missing_tool_does_not_pass(self):
        snapshot = copy.deepcopy(self.snapshot)
        del snapshot['tools'][self.tools[0]['id']]
        with self.assertRaisesRegex(ValueError, 'every catalog tool'):
            build.validate_reputation(snapshot, self.tools)

    def test_noncanonical_and_unverified_cannot_enter_sort(self):
        snapshot = copy.deepcopy(self.snapshot)
        entry = snapshot['tools'][self.tools[0]['id']]
        entry['github'] = dict(self.gh)
        build.validate_reputation(snapshot, self.tools)  # A measured zero is valid.
        for bad in (
            dict(self.gh, status='unverified'),
            dict(self.gh, stars=True),
            dict(self.gh, url='https://github.com/owner/project/issues'),
            dict(self.gh, url='https://github.com@evil.example/owner/project'),
            dict(self.gh, checkedAt='2026-02-29'),
        ):
            entry['github'] = bad
            with self.subTest(bad=bad), self.assertRaises(ValueError):
                build.validate_reputation(snapshot, self.tools)

    def test_rating_requires_native_scale_and_review_count(self):
        snapshot = copy.deepcopy(self.snapshot)
        entry = snapshot['tools'][self.tools[0]['id']]
        entry['sourceforge'] = dict(
            status='observed', url='https://sourceforge.net/projects/test/', checkedAt=self.checked_at,
            identitySource='https://owner.example/test', rating=dict(value=4.3, scale=5, reviews=1)
        )
        build.validate_reputation(snapshot, self.tools)
        for invalid in (dict(value=97, scale=100, reviews=1), dict(value=4.9, scale=5, reviews=0)):
            entry['sourceforge']['rating'] = invalid
            with self.subTest(invalid=invalid), self.assertRaisesRegex(ValueError, 'native rating'):
                build.validate_reputation(snapshot, self.tools)


    def test_browser_store_ratings_keep_provider_identity_and_native_scale(self):
        snapshot = copy.deepcopy(self.snapshot)
        entry = snapshot['tools'][self.tools[0]['id']]
        entry['browserStores'] = [dict(
            provider='mozilla-addons', name='Mozilla Add-ons',
            url='https://addons.mozilla.org/en-US/firefox/addon/test/',
            checkedAt=self.checked_at, status='observed',
            rating=dict(value=4.8, scale=5, reviews=100), users=1000
        )]
        build.validate_reputation(snapshot, self.tools)
        for invalid in (
            dict(provider='unknown-store', name='Wrong provider',
                 url='https://addons.mozilla.org/en-US/firefox/addon/test/',
                 checkedAt=self.checked_at, status='observed',
                 rating=dict(value=4.8, scale=5, reviews=100)),
            dict(provider='mozilla-addons', name='Wrong host',
                 url='https://example.com/addon/test/',
                 checkedAt=self.checked_at, status='observed',
                 rating=dict(value=4.8, scale=5, reviews=100)),
            dict(provider='mozilla-addons', name='Invalid rating',
                 url='https://addons.mozilla.org/en-US/firefox/addon/test/',
                 checkedAt=self.checked_at, status='observed',
                 rating=dict(value=4.8, scale=10, reviews=100)),
            dict(provider='mozilla-addons', name='Missing sample',
                 url='https://addons.mozilla.org/en-US/firefox/addon/test/',
                 checkedAt=self.checked_at, status='observed',
                 rating=dict(value=4.8, scale=5, reviews=0)),
            dict(provider='mozilla-addons', name='Future observation',
                 url='https://addons.mozilla.org/en-US/firefox/addon/test/',
                 checkedAt='2027-01-01', status='observed',
                 rating=dict(value=4.8, scale=5, reviews=100)),
        ):
            entry['browserStores'] = [invalid]
            with self.subTest(invalid=invalid), self.assertRaises(ValueError):
                build.validate_reputation(snapshot, self.tools)

    def test_unreported_browser_store_rating_is_explicit_not_inferred(self):
        snapshot = copy.deepcopy(self.snapshot)
        entry = snapshot['tools'][self.tools[0]['id']]
        entry['browserStores'] = [dict(
            provider='apple-app-store', name='Apple App Store',
            url='https://apps.apple.com/us/app/test/id123456789',
            checkedAt=self.checked_at, status='unreported',
            reason='No rating was verified.'
        )]
        build.validate_reputation(snapshot, self.tools)

if __name__ == '__main__':
    unittest.main()
