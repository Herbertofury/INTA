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

    def test_specific_fix_commands_are_valid_and_rendered_with_copy_controls(self):
        field_notes = json.loads((ROOT / 'data/field-notes.json').read_text())
        build.validate_field_note_commands(field_notes)
        commands = [command for note in field_notes['notes'] for command in note.get('commands', [])]
        self.assertGreaterEqual(len(commands), 25)
        self.assertTrue(all(command['sourceUrl'].startswith('https://') for command in commands))
        output = build.build_outputs()
        catalog = json.loads(output['site/catalog.json'])
        self.assertTrue(any(note.get('commands') for note in catalog['fieldNotes']['notes']))
        self.assertIn('data-copy-field-command', output['site/index.html'])
        for note in field_notes['notes']:
            if note.get('commands'):
                guide = output[f"wiki/Job-{note['jobId']}.md"]
                self.assertIn(note['title'], guide)
                self.assertIn(note['commands'][0]['code'], guide)
                self.assertIn('#### ⚡ Quick commands', guide)

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

    def test_publisher_page_and_exact_acquisition_are_separate(self):
        access = json.loads((ROOT / 'data/tool-access.json').read_text())
        build.validate_tool_access(access, self.tools)
        self.assertEqual(set(access['tools']), {t['id'] for t in self.tools})
        for identity, entry in access['tools'].items():
            if entry['primary']['kind'] == 'file':
                self.assertTrue(entry['primary']['platform'], identity)
                self.assertTrue(entry['primary']['format'], identity)
                self.assertNotEqual(entry['officialUrl'], entry['primary']['url'], identity)

    def test_acquisition_rejects_missing_tools_and_misleading_direct_downloads(self):
        action = {'kind': 'release-page', 'label': 'Direct download',
                  'url': 'https://example.com/releases', 'evidenceUrl': 'https://example.com/',
                  'checkedAt': '2026-11-26'}
        access = {'schemaVersion': 1, 'tools': {'sample': {
            'officialUrl': 'https://example.com/', 'primary': action, 'alternates': []}}}
        with self.assertRaisesRegex(ValueError, 'non-file route'):
            build.validate_tool_access(access, [{'id': 'sample'}])
        action['label'] = 'Choose a release'
        with self.assertRaisesRegex(ValueError, 'every catalog tool'):
            build.validate_tool_access(access, [{'id': 'sample'}, {'id': 'missing'}])
        action['url'] = 'javascript:alert(1)'
        with self.assertRaisesRegex(ValueError, 'public HTTPS'):
            build.validate_tool_access(access, [{'id': 'sample'}])

    def test_scenario_acquisition_selects_the_matching_troubleshooter(self):
        entry = {'primary': {'url': 'https://example.com/get-help'}, 'alternates': [
            {'id': 'printer', 'url': 'https://example.com/printer'},
            {'id': 'network', 'url': 'https://example.com/network'}]}
        self.assertEqual(build.selected_tool_action(entry)['url'], 'https://example.com/get-help')
        self.assertEqual(build.selected_tool_action(entry, 'network')['url'], 'https://example.com/network')
        with self.assertRaisesRegex(ValueError, 'Unknown scenario acquisition'):
            build.selected_tool_action(entry, 'missing')

    def test_every_job_has_a_complete_working_kit(self):
        workspaces = json.loads((ROOT / 'data/job-workspaces.json').read_text())
        access = json.loads((ROOT / 'data/tool-access.json').read_text())['tools']
        build.validate_job_workspaces(workspaces, self.jobs, self.tools, access)
        output = build.build_outputs()
        self.assertEqual(json.loads(output['site/catalog.json'])['jobWorkspaces'], workspaces)
        quick = json.loads((ROOT / 'data/quick-start.json').read_text())['items']
        jobs_by_id = {job['id']: job for job in self.jobs}
        for identity, workspace in workspaces['jobs'].items():
            full = output[f'wiki/Job-{identity}.md']
            self.assertLess(full.index('## 🧰 Tools and links for this job'), full.index('## ⚡ Quick answer'))
            self.assertIn('## 🧭 Match the symptom', full)
            self.assertNotIn('<details>', full)
            for resource in workspace['resources']:
                self.assertIn(f"]({resource['url']})", full)
            working_kit = full.split('<a id="working-kit"></a>', 1)[1].split('<a id="quick-answer"></a>', 1)[0]
            recommended = full.split('<a id="toolkit"></a>', 1)[1].split('<a id="walkthrough"></a>', 1)[0]
            for selection in workspace['tools']:
                route = access[selection['toolId']]
                action = build.selected_tool_action(route, selection.get('actionId'))
                for url in [route['officialUrl'], action['url']]:
                    self.assertIn(f"]({url})", working_kit, identity + ': ' + selection['toolId'])
                # The working kit may add useful actions beyond the job's
                # catalog list. Its scenario action must also survive when
                # that tool appears in the later recommended-tools section.
                if selection['toolId'] in jobs_by_id[identity]['tools']:
                    self.assertIn(f"]({action['url']})", recommended, identity + ': ' + selection['toolId'])
            for item in [item for item in quick if item['jobId'] == identity]:
                answer = output[f"wiki/{item['wikiPage']}.md"]
                self.assertIn(f"#job/{identity}/quick/{item['id']}", answer)
                self.assertLess(answer.index('## 🧰 Tools and links for this job'), answer.index('## ⚡ First three actions'))
                for resource in workspace['resources']:
                    self.assertIn(f"]({resource['url']})", answer)

    def test_working_kits_reject_missing_jobs_and_unknown_tool_actions(self):
        workspaces = json.loads((ROOT / 'data/job-workspaces.json').read_text())
        access = json.loads((ROOT / 'data/tool-access.json').read_text())['tools']
        missing = copy.deepcopy(workspaces)
        missing['jobs'].pop(next(iter(missing['jobs'])))
        with self.assertRaisesRegex(ValueError, 'every playbook exactly'):
            build.validate_job_workspaces(missing, self.jobs, self.tools, access)
        wrong_action = copy.deepcopy(workspaces)
        toolkit = next(entry['tools'] for entry in wrong_action['jobs'].values() if entry['tools'])
        toolkit[0]['actionId'] = 'unrelated-troubleshooter'
        with self.assertRaisesRegex(ValueError, 'Unknown scenario acquisition'):
            build.validate_job_workspaces(wrong_action, self.jobs, self.tools, access)

    def test_working_kits_reject_unsafe_resources_and_empty_decisions(self):
        workspaces = json.loads((ROOT / 'data/job-workspaces.json').read_text())
        access = json.loads((ROOT / 'data/tool-access.json').read_text())['tools']
        for url in ('javascript:alert(1)', 'https://user:password@example.com/', 'https://example.com/?utm_source=tracking'):
            invalid = copy.deepcopy(workspaces)
            resources = next(entry['resources'] for entry in invalid['jobs'].values() if entry['resources'])
            resources[0]['url'] = url
            with self.assertRaisesRegex(ValueError, 'public HTTPS resource'):
                build.validate_job_workspaces(invalid, self.jobs, self.tools, access)
        missing = copy.deepcopy(workspaces)
        next(iter(missing['jobs'].values()))['decisions'] = []
        with self.assertRaisesRegex(ValueError, 'decision, completion and escalation'):
            build.validate_job_workspaces(missing, self.jobs, self.tools, access)


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
