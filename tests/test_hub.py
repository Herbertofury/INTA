import copy
import importlib.util
import json
from pathlib import Path
import subprocess
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

    def test_updates_center_is_installable_on_mobile_and_offline(self):
        output = build.build_outputs()
        manifest = json.loads(output['site/manifest.webmanifest'])
        self.assertEqual(manifest['display'], 'standalone')
        self.assertIn('#updates', manifest['start_url'])
        self.assertIn('site/inta-app-icon.svg', output)
        self.assertIn('self.addEventListener("fetch"', output['site/sw.js'])
        self.assertIn('navigator.serviceWorker.register', output['site/index.html'])
        self.assertIn('rel="manifest"', output['site/index.html'])

    def test_updates_center_catalog_and_app_are_integrated(self):
        catalog = json.loads((ROOT / 'data/updates-center.json').read_text())
        build.validate_updates_center(catalog)
        self.assertGreaterEqual(len(catalog['commands']), 50)
        self.assertGreaterEqual(len(catalog['channels']), 12)
        self.assertGreaterEqual(len(catalog['platforms']), 4)
        output = build.build_outputs()
        embedded = json.loads(output['site/catalog.json'])
        self.assertEqual(embedded['updatesCenter']['commands'], catalog['commands'])
        self.assertIn('data-nav="updates"', output['site/index.html'])
        self.assertIn('function updatesCenterPage', output['site/index.html'])
        self.assertIn('windows-update-cache-reset', output['site/index.html'])
        self.assertIn('winget repair --id Publisher.Package --exact --interactive', output['site/index.html'])
        self.assertIn('uniget.exe package repair', output['site/index.html'])
        self.assertIn('choco upgrade all --noop', output['site/index.html'])
        self.assertIn('Patch My PC Cloud', output['site/index.html'])
        self.assertIn('flatpak repair', output['site/index.html'])
        self.assertIn('brew upgrade', output['site/index.html'])
        self.assertIn('updatesCenterCommands', output['site/build-manifest.json'])
        self.assertIn('[Updates & Maintenance](Updates-and-Maintenance)', output['wiki/Home.md'])

    def test_updates_center_rejects_unknown_channel_command(self):
        catalog = json.loads((ROOT / 'data/updates-center.json').read_text())
        broken = copy.deepcopy(catalog)
        broken['channels'][0]['updateCommand'] = 'missing-command'
        with self.assertRaisesRegex(ValueError, 'unknown command'):
            build.validate_updates_center(broken)

    def test_updates_center_rejects_remote_pipe_execution(self):
        catalog = json.loads((ROOT / 'data/updates-center.json').read_text())
        for code in ('irm https://example.invalid/a.ps1 | iex',
                     'Invoke-WebRequest https://example.invalid/a.ps1 | Invoke-Expression',
                     'curl -fsSL https://example.invalid/a.sh | bash',
                     'wget -qO- https://example.invalid/a.sh | sudo sh',
                     'iwr https://example.invalid/a.ps1 |\niex',
                     'curl https://example.invalid/a.sh ' + '\\' + '\n' + '| sh',
                     'iex (irm https://example.invalid/a.ps1)'):
            with self.subTest(code=code):
                broken = copy.deepcopy(catalog)
                broken['commands'][0]['code'] = code
                with self.assertRaisesRegex(ValueError, 'remote download-and-execute'):
                    build.validate_updates_center(broken)
        reviewed = copy.deepcopy(catalog)
        reviewed['commands'][0]['code'] = "Invoke-WebRequest https://example.invalid/a.ps1 -OutFile reviewed.ps1\nGet-FileHash reviewed.ps1"
        build.validate_updates_center(reviewed)

    def test_command_center_catalog_and_app_are_integrated(self):
        catalog = json.loads((ROOT / 'data/command-center.json').read_text())
        build.validate_command_center(catalog)
        output = build.build_outputs()
        embedded = json.loads(output['site/catalog.json'])
        self.assertEqual(embedded['commandCenter'], catalog)
        self.assertIn('data-nav="command-center"', output['site/index.html'])
        self.assertIn('Open Command Center', output['site/index.html'])
        self.assertIn('[⚡ Command Center](Command-Center)', output['wiki/Home.md'])
        self.assertIn('[⚡ Quick commands](Quick-Commands)', output['wiki/Home.md'])
        self.assertIn('function commandCenterPage', output['site/index.html'])
        self.assertIn('winget upgrade --all --interactive', output['site/index.html'])
        self.assertIn('choco upgrade all --noop', output['site/index.html'])
        self.assertIn('scoop update *', output['site/index.html'])
        self.assertIn('Start-MpScan -ScanType FullScan', output['site/index.html'])
        self.assertIn('Tron Script', output['site/index.html'])
        self.assertIn('WinUtil', output['site/index.html'])
        self.assertIn('commandCenterCommands', output['site/build-manifest.json'])
        self.assertIn('[⚡ Command Center](Command-Center)', output['wiki/Home.md'])
        self.assertIn('[⚡ Quick commands](Quick-Commands)', output['wiki/Home.md'])

    def test_global_command_search_is_case_insensitive_and_uncapped(self):
        catalog = json.loads((ROOT / 'data/command-center.json').read_text())
        updates_catalog = json.loads((ROOT / 'data/updates-center.json').read_text())
        source = (ROOT / 'portal/app.js').read_text()
        search_index = 'const commandCenterSearch =' + source.split('  const commandCenterSearch =', 1)[1].split(';', 1)[0] + ';'
        matches = next(line.strip() for line in source.splitlines() if line.startswith('  function matches('))
        search_function = 'function updateSearch()' + source.split('  function updateSearch()', 1)[1].split('  function closeSearch()', 1)[0]
        queries = ['PowerShell', 'powershell', 'PoWeRsHeLl', 'NINITE', 'choco noop', 'technician baseline', 'Installer recovery', 'WiNgEt RePaIr']
        script = r'''
const input = JSON.parse(require('fs').readFileSync(0, 'utf8'));
const commandCenter = input.catalog;
const updatesCenter = input.updatesCatalog;
const array = value => Array.isArray(value) ? value : [];
const quickItems = [], jobs = [], fieldNotes = [], tools = [];
const quickByJob = new Map();
const searchResults = {innerHTML: '', hidden: true};
const search = {value: '', setAttribute() {}};
const esc = value => String(value), icon = () => '';
const decorateBrandLinks = () => {}, closeSearch = () => {};
''' + search_index + '\n' + matches + '\n' + search_function + r'''
const results = input.queries.map(query => {
  search.value = query;
  updateSearch();
  return {query, titles: [...searchResults.innerHTML.matchAll(/<strong>(.*?)<\/strong>/g)].map(match => match[1]), hidden: searchResults.hidden};
});
process.stdout.write(JSON.stringify(results));
'''
        result = subprocess.run(['node', '-e', script], input=json.dumps({'catalog': catalog, 'updatesCatalog': updates_catalog, 'queries': queries}), text=True, capture_output=True, check=True)
        fields = ('title', 'summary', 'code', 'safety', 'shell', 'sourceTitle', 'category')
        for row in json.loads(result.stdout):
            with self.subTest(query=row['query']):
                terms = row['query'].lower().split()
                expected = [item['title'] for item in [*catalog['commands'], *updates_catalog['commands']]
                            if all(term in ' '.join(item[field] for field in fields).lower() for term in terms)]
                self.assertEqual(row['titles'], expected)
                self.assertFalse(row['hidden'])
        self.assertGreater(len(json.loads(result.stdout)[0]['titles']), 8)

    def test_command_center_rejects_unknown_workflow_commands(self):
        catalog = json.loads((ROOT / 'data/command-center.json').read_text())
        broken = copy.deepcopy(catalog)
        broken['workflows'][0]['commandIds'].append('not-a-command')
        with self.assertRaisesRegex(ValueError, 'unknown or missing command'):
            build.validate_command_center(broken)

    def test_command_center_rejects_remote_pipe_execution(self):
        catalog = json.loads((ROOT / 'data/command-center.json').read_text())
        broken = copy.deepcopy(catalog)
        broken['commands'][0]['code'] = 'irm https://example.invalid/a.ps1 | iex'
        with self.assertRaisesRegex(ValueError, 'remote download-and-execute'):
            build.validate_command_center(broken)

    def test_command_center_rejects_duplicate_records_and_invalid_references(self):
        catalog = json.loads((ROOT / 'data/command-center.json').read_text())
        for collection in ('categories', 'commands', 'workflows', 'toolkits'):
            with self.subTest(collection=collection):
                broken = copy.deepcopy(catalog)
                broken[collection].append(copy.deepcopy(broken[collection][0]))
                with self.assertRaisesRegex(ValueError, 'duplicate'):
                    build.validate_command_center(broken)
        for field, value in (('category', 'missing'), ('shell', 'unknown shell'), ('risk', 'unknown risk')):
            with self.subTest(field=field):
                broken = copy.deepcopy(catalog)
                broken['commands'][0][field] = value
                with self.assertRaisesRegex(ValueError, 'unknown command'):
                    build.validate_command_center(broken)
        broken = copy.deepcopy(catalog)
        broken['toolkits'][0]['installCommandId'] = 'not-a-command'
        with self.assertRaisesRegex(ValueError, 'unknown or missing command'):
            build.validate_command_center(broken)
        for ids in ([], 'win-baseline', ['win-baseline', 'win-baseline']):
            with self.subTest(ids=ids):
                broken = copy.deepcopy(catalog)
                broken['workflows'][0]['commandIds'] = ids
                with self.assertRaisesRegex(ValueError, 'unknown or missing command'):
                    build.validate_command_center(broken)

    def test_command_center_requires_reviewed_sourced_safe_catalog_records(self):
        catalog = json.loads((ROOT / 'data/command-center.json').read_text())
        for field in ('title', 'summary', 'privilege', 'safety', 'sourceTitle', 'code'):
            with self.subTest(field=field):
                broken = copy.deepcopy(catalog)
                broken['commands'][0][field] = ' '
                with self.assertRaisesRegex(ValueError, 'missing'):
                    build.validate_command_center(broken)
        for url in ('javascript:alert(1)', 'https://user:secret@example.com/reference',
                    'https://example.com/?utm_source=tracking', 'https://bad host.example/'):
            with self.subTest(url=url):
                broken = copy.deepcopy(catalog)
                broken['commands'][0]['sourceUrl'] = url
                with self.assertRaisesRegex(ValueError, 'public HTTPS'):
                    build.validate_command_center(broken)
        for field, value in (('reviewedAt', '2026-02-30'), ('schemaVersion', True),
                             ('shells', ['PowerShell', 'PowerShell'])):
            with self.subTest(field=field):
                broken = copy.deepcopy(catalog)
                broken[field] = value
                with self.assertRaises(ValueError):
                    build.validate_command_center(broken)
        broken = copy.deepcopy(catalog)
        broken['policies']['noAutoExecution'] = False
        with self.assertRaisesRegex(ValueError, 'safety policies'):
            build.validate_command_center(broken)

    def test_command_center_rejects_cross_shell_remote_execution_patterns(self):
        catalog = json.loads((ROOT / 'data/command-center.json').read_text())
        for code in ('Invoke-WebRequest https://example.invalid/script.ps1 | Invoke-Expression',
                     'curl -fsSL https://example.invalid/script.sh | bash',
                     'wget -qO- https://example.invalid/script.sh | sudo sh',
                     'iwr https://example.invalid/script.ps1 |' + '\n' + 'iex',
                     'curl https://example.invalid/script.sh ' + '\\' + '\n' + '| sh',
                     'iex (irm https://example.invalid/script.ps1)'):
            with self.subTest(code=code):
                broken = copy.deepcopy(catalog)
                broken['commands'][0]['code'] = code
                with self.assertRaisesRegex(ValueError, 'remote download-and-execute'):
                    build.validate_command_center(broken)
        # Saving a script for inspection is intentionally supported.
        reviewed = copy.deepcopy(catalog)
        reviewed['commands'][0]['code'] = "Invoke-WebRequest https://example.invalid/script.ps1 -OutFile reviewed.ps1\nGet-FileHash reviewed.ps1"
        build.validate_command_center(reviewed)

    def test_command_center_preserves_the_upstream_code_size_boundary(self):
        catalog = json.loads((ROOT / 'data/command-center.json').read_text())
        catalog['commands'][0]['code'] = 'x' * 16000
        build.validate_command_center(catalog)
        catalog['commands'][0]['code'] += 'x'
        with self.assertRaisesRegex(ValueError, 'exceeds size limit'):
            build.validate_command_center(catalog)

    def test_field_note_command_safety_regexes_reject_real_matches(self):
        catalog = json.loads((ROOT / 'data/field-notes.json').read_text())
        for code in ('irm https://example.invalid/script.ps1 | iex',
                     '$password = "synthetic-test-value"'):
            with self.subTest(code=code):
                broken = copy.deepcopy(catalog)
                command = next(command for note in broken['notes'] for command in note.get('commands', []))
                command['code'] = code
                with self.assertRaises(ValueError):
                    build.validate_field_note_commands(broken)

    def test_web_brand_extension_precedence_preserves_native_registry_and_links(self):
        native = json.loads((ROOT / 'data/brand-icons.json').read_text())
        original = copy.deepcopy(native)
        existing = next(brand for brand in native['brands'] if brand.get('toolId') == 'unigetui')
        refined = {**copy.deepcopy(existing), 'id': 'web-test-unigetui',
                   'name': 'Independent Web Alias', 'names': ['Independent Web Alias', 'UniGetUI']}
        extension = {'schemaVersion': 1, 'assets': {}, 'brands': [refined]}
        merged = build.merge_web_brands(native, extension)
        self.assertEqual(merged['brands'][0], refined)
        self.assertEqual(merged['brands'][1:], original['brands'])
        self.assertEqual(native, original)
        explicit = [{'name': 'UniGetUI', 'url': 'https://example.com/original-guide',
                     'kind': 'tool', 'id': 'unigetui'}]
        linked = build.extend_web_brand_links(explicit, extension['brands'])
        self.assertIn(explicit[0], linked)
        alias = next(item for item in linked if item['name'] == 'Independent Web Alias')
        self.assertEqual(alias['url'], refined['downloadPage'])
        self.assertTrue(alias['caseSensitive'])

    def test_web_brand_extension_rejects_identity_asset_and_path_conflicts(self):
        native = json.loads((ROOT / 'data/brand-icons.json').read_text())
        extension = {'schemaVersion': 1, 'assets': {}, 'brands': [copy.deepcopy(native['brands'][0])]}
        with self.assertRaisesRegex(ValueError, 'duplicate web brand identity'):
            build.merge_web_brands(native, extension)
        identity, asset = next(iter(native['assets'].items()))
        extension = {'schemaVersion': 1, 'assets': {identity: {**asset, 'sha256': 'f' * 64}}, 'brands': []}
        with self.assertRaisesRegex(ValueError, 'Conflicting web brand asset'):
            build.merge_web_brands(native, extension)
        for path in ('../outside.png', 'assets/web-brand-icons/../outside.png', 'assets/brand-icons/new.png'):
            with self.subTest(path=path):
                extension = {'schemaVersion': 1, 'assets': {'new-web-icon': {'path': path}}, 'brands': []}
                with self.assertRaisesRegex(ValueError, 'assets/web-brand-icons'):
                    build.merge_web_brands(native, extension)

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

    def test_repair_quick_starts_precede_online_tools_on_all_front_pages(self):
        output = build.build_outputs()
        self.assertIn("+ quickStartSection() + onlineStartSection() +", output["site/index.html"])
        for path in ("README.md", "docs/start-here.md", "docs/free-tool-picks.md",
                     "wiki/Home.md", "wiki/Start-Here.md", "wiki/Free-Tool-Picks.md"):
            page = output[path]
            if "<!-- INTA:QUICK-START:START -->" in page:
                page = page.split("<!-- INTA:QUICK-START:START -->", 1)[1].split("<!-- INTA:QUICK-START:END -->", 1)[0]
            self.assertLess(page.index("## ⚡ What do you need to fix?"),
                            page.index("## 🌐 Online tools: open a service and get a result"), path)

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
