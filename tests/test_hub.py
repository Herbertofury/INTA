import copy
import importlib.util
import json
from pathlib import Path
import unittest
from urllib.error import HTTPError

ROOT = Path(__file__).resolve().parents[1]


def module(filename):
    spec = importlib.util.spec_from_file_location(filename.replace('-', '_'), ROOT / 'scripts' / filename)
    result = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(result)
    return result


build = module('build-hub.py')
upstream = module('check-upstreams.py')


class HubIntegrity(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.tools = build.records(ROOT / 'data/tools.json', 'tools')
        cls.jobs = build.records(ROOT / 'data/jobs.json', 'jobs')

    def test_all_references_resolve(self):
        build.validate(self.tools, self.jobs)

    def test_bad_relationship_is_rejected(self):
        jobs = copy.deepcopy(self.jobs)
        jobs[0]['tools'].append('missing-tool')
        with self.assertRaisesRegex(ValueError, 'unknown tools'):
            build.validate(self.tools, jobs)

    def test_executable_or_tracking_urls_are_rejected(self):
        for url in ('javascript:alert(1)', 'https://vendor.example/?utm_source=tracker'):
            tools = copy.deepcopy(self.tools)
            tools[0]['downloadUrl'] = url
            with self.assertRaises(ValueError):
                build.validate(tools, self.jobs)

    def test_every_guide_and_tool_reaches_generated_output(self):
        output = build.build_outputs()
        catalog = json.loads(output['site/catalog.json'])
        self.assertEqual(len(catalog['tools']), len(self.tools))
        self.assertEqual(len(catalog['jobs']), len(self.jobs))
        for job in self.jobs:
            self.assertIn('wiki/Job-' + job['id'] + '.md', output)
            self.assertIn('docs/runbooks/' + job['id'] + '.md', output)
        for tool in self.tools:
            self.assertIn(tool['name'], output['wiki/Tool-Directory.md'])
        self.assertIn('window.INTA_DATA = ', output['site/index.html'])
        self.assertNotIn('{{DATA}}', output['site/index.html'])


class FakeResponse:
    status = 200
    def __init__(self, value):
        self.value = value
    def __enter__(self):
        return self
    def __exit__(self, *args):
        pass
    def read(self):
        return json.dumps(self.value).encode()
    def geturl(self):
        return 'https://api.github.com/repos/example/tool/releases/latest'


class UpstreamReport(unittest.TestCase):
    def setUp(self):
        self.tool = {'id': 'test', 'name': 'Test', 'reviewedAt': '2026-11-26',
                     'sourceUrl': 'https://github.com/example/tool',
                     'homepage': 'https://example.com', 'downloadUrl': 'https://example.com/download'}

    def test_release_keeps_observed_version(self):
        result = upstream.inspect(self.tool, opener=lambda *a, **kw: FakeResponse({'tag_name': 'v2.0', 'html_url': 'https://github.com/example/tool/releases/tag/v2.0'}))
        self.assertEqual(result['status'], 'release-observed')
        self.assertEqual(result['version'], 'v2.0')

    def test_no_release_never_invents_a_version(self):
        def missing(req, **kw):
            raise HTTPError(req.full_url, 404, 'Not Found', {}, None)
        result = upstream.inspect(self.tool, opener=missing)
        self.assertEqual(result['status'], 'manual-version-review')
        self.assertNotIn('version', result)

    def test_vendor_reachability_is_not_version_verification(self):
        self.tool['sourceUrl'] = 'https://example.com'
        result = upstream.inspect(self.tool, opener=lambda *a, **kw: FakeResponse({}))
        self.assertEqual(result['status'], 'manual-version-review')
        self.assertNotIn('version', result)

    def test_network_failure_remains_unresolved(self):
        def unavailable(*a, **kw):
            raise TimeoutError('Connection timed out')
        self.assertEqual(upstream.inspect(self.tool, opener=unavailable)['status'], 'unresolved')


if __name__ == '__main__':
    unittest.main()
