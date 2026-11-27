"""Protect the complete online directory, its review evidence and launch routes."""
import copy
from datetime import date, timedelta
from html.parser import HTMLParser
import importlib.util
import json
from pathlib import Path
import unittest
from urllib.parse import parse_qs, urlparse


ROOT = Path(__file__).resolve().parents[1]


def module(filename):
    spec = importlib.util.spec_from_file_location(
        'inta_online_test_' + filename.replace('-', '_'), ROOT / 'scripts' / filename)
    result = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(result)
    return result


online = module('online_tools.py')
build = module('build-hub.py')

# These are the 133 tool identities present before the online expansion. Keeping
# a separate acceptance baseline catches a tool removed from both joined files.
PREEXISTING_TOOL_IDS = frozenset('''
defender-offline safety-scanner adwcleaner malwarebytes hitmanpro
eset-online-scanner clamav bitdefender-gravityzone defender-business
sophos-endpoint bitdefender-free emsisoft-emergency-kit meshcentral
rustdesk quick-assist chrome-remote-desktop anydesk teamviewer
screenconnect tailscale sysinternals autoruns process-explorer procmon
tcpview rammap procdump sigcheck dism sfc windows-recovery winget
powershell windows-terminal system-informer windirstat wiztree
bcuninstaller windows-performance-toolkit windbg winsat crystaldiskinfo
smartmontools ddrescue testdisk photorec dmde gparted rescuezilla
clonezilla restic kopia rclone veeam-agent freefilesync robocopy hwinfo
libre-hardware-monitor memtest86plus memtest86 cpu-z occt oem-drivers
wireshark iperf3 nmap angry-ip-scanner putty winscp openssh pktmon rufus
ventoy systemrescue hirens-bootcd-pe microsoft-windows11-media
debian-live ubuntu-live 7zip notepadplusplus powertoys everything wizfile
sharex keepassxc localsend unigetui chatgpt codex-cli claude-code
gemini-cli ollama yara-x pe-sieve hollows-hunter fibratus rustinel prx-sd
sentinella velociraptor hayabusa chainsaw sigma osquery wazuh volatility
autopsy nvme-cli gsmartcontrol tcpdump cryptsetup dislocker rsync
get-help naps2 advanced-ip-scanner quick-machine-recovery
windows-point-in-time-restore setupdiag windows-sandbox
usb-device-tree-view powercfg curl mtr remmina syncthing diskspd
winmerge sumatrapdf netbootxyz promptfoo windows-laps sysmon
'''.split())

# Every accepted product must survive subsequent presentation or performance
# changes. Additions are welcome; the assertion is deliberately not a count cap.
ACCEPTED_ONLINE_IDS = frozenset('''
virustotal metadefender-cloud jotti hybrid-analysis anyrun triage joe-sandbox
intezer-analyze polyswarm filescan-io unpacme kaspersky-opentip urlscan urlhaus
malwarebazaar threatfox yaraify no-more-ransom id-ransomware checkphish
sophos-intelix drweb-online ssl-labs mdn-observatory securityheaders internet-nl
hardenize dmarcian-tools mxtoolbox google-admin-toolbox dnsviz zonemaster
verisign-dnssec-debugger dnschecker whatsmydns nslookup-io learndmarc mail-tester
globalping ripestat cloudflare-radar cloudflare-speed fast-com
waveform-bufferbloat microsoft-remote-connectivity microsoft365-connectivity
microsoft-service-health google-workspace-status test-ipv6 webpagetest
pagespeed-insights wave-accessibility shodan censys greynoise abuseipdb
talos-intelligence otx pulsedive ipinfo spamhaus google-safe-browsing
have-i-been-pwned icann-lookup urlvoid openphish urlquery first-epss cisa-kev nvd
osv github-advisories snyk-vulnerability-db cyberchef regex101 jwt-debugger
browserleaks cover-your-tracks deps-dev first-cvss attack-navigator squoosh
drawio excalidraw hoppscotch
'''.split())


def valid_fixture():
    tools = [
        {'id': 'sample-online', 'category': 'online', 'mode': 'web'},
        {'id': 'sample-local', 'category': 'utility', 'mode': 'native'},
    ]
    jobs = [{'id': 'investigate-file'}]
    data = {
        'schemaVersion': 1,
        'reviewedAt': '2025-01-01',
        'groups': [{'id': 'file-analysis', 'label': 'File analysis',
                    'summary': 'Read the available file evidence.'}],
        'featured': ['sample-online'],
        'workflows': [{
            'id': 'check-file', 'title': 'Check a suspicious file',
            'summary': 'Compare evidence before deciding the next action.',
            'jobId': 'investigate-file', 'toolIds': ['sample-online'],
            'steps': ['Calculate the file hash.', 'Look up the existing report.',
                      'Record the dated findings.'],
        }],
        'tools': [{
            'toolId': 'sample-online', 'group': 'file-analysis',
            'account': 'Public lookup does not need an account.',
            'sharing': 'The provider receives the searched hash.',
            'firstAction': 'Search for the SHA-256 hash.',
            'expectedResult': 'An existing report or an explicit no-match result.',
            'reviewedAt': '2025-01-01',
            'sources': [{'url': 'https://vendor.example/docs',
                         'title': 'Official lookup documentation',
                         'evidence': 'Explains public hash lookup and retention.'}],
        }],
    }
    return data, tools, jobs


class OnlineValidationTests(unittest.TestCase):
    def test_valid_directory_is_accepted(self):
        data, tools, jobs = valid_fixture()
        online.validate_online(data, tools, jobs)

    def test_metadata_requires_a_complete_one_to_one_join(self):
        for case in ('missing', 'duplicate', 'unknown', 'non-online'):
            with self.subTest(case=case):
                data, tools, jobs = valid_fixture()
                if case == 'missing':
                    data['tools'].clear()
                elif case == 'duplicate':
                    data['tools'].append(copy.deepcopy(data['tools'][0]))
                else:
                    data['tools'][0]['toolId'] = 'missing-tool' if case == 'unknown' else 'sample-local'
                with self.assertRaises(ValueError):
                    online.validate_online(data, tools, jobs)

    def test_groups_reject_orphans_duplicates_and_unsafe_page_paths(self):
        for case in ('unknown', 'duplicate', 'empty-group', '../outside', 'two words', 'UPPER', ''):
            with self.subTest(case=case):
                data, tools, jobs = valid_fixture()
                if case == 'unknown':
                    data['tools'][0]['group'] = 'missing-group'
                elif case == 'duplicate':
                    data['groups'].append(copy.deepcopy(data['groups'][0]))
                elif case == 'empty-group':
                    data['groups'].append({'id': 'unused-group', 'label': 'Unused', 'summary': 'No tools.'})
                else:
                    data['groups'][0]['id'] = case
                    data['tools'][0]['group'] = case
                with self.assertRaises(ValueError):
                    online.validate_online(data, tools, jobs)

    def test_group_labels_and_summaries_cannot_be_blank(self):
        for field in ('label', 'summary'):
            for value in ('', '   ', None):
                with self.subTest(field=field, value=value):
                    data, tools, jobs = valid_fixture()
                    data['groups'][0][field] = value
                    with self.assertRaises(ValueError):
                        online.validate_online(data, tools, jobs)

    def test_online_catalog_entries_cannot_be_downloads(self):
        data, tools, jobs = valid_fixture()
        tools[0]['mode'] = 'native'
        with self.assertRaises(ValueError):
            online.validate_online(data, tools, jobs)

    def test_missing_or_blank_operational_notes_are_rejected(self):
        for field in ('account', 'sharing', 'firstAction', 'expectedResult'):
            for value in ('', '   ', None):
                with self.subTest(field=field, value=value):
                    data, tools, jobs = valid_fixture()
                    data['tools'][0][field] = value
                    with self.assertRaises(ValueError):
                        online.validate_online(data, tools, jobs)

    def test_sources_require_titles_evidence_and_at_least_one_source(self):
        for field in ('title', 'evidence'):
            for value in ('', '   ', None):
                with self.subTest(field=field, value=value):
                    data, tools, jobs = valid_fixture()
                    data['tools'][0]['sources'][0][field] = value
                    with self.assertRaises(ValueError):
                        online.validate_online(data, tools, jobs)
            data, tools, jobs = valid_fixture()
            del data['tools'][0]['sources'][0][field]
            with self.assertRaises(ValueError):
                online.validate_online(data, tools, jobs)
        data, tools, jobs = valid_fixture()
        data['tools'][0]['sources'] = []
        with self.assertRaises(ValueError):
            online.validate_online(data, tools, jobs)

    def test_evidence_urls_reject_executable_insecure_credential_and_tracking_routes(self):
        for url in ('javascript:alert(1)', 'http://vendor.example/docs',
                    'https://user:password@vendor.example/docs',
                    'https://:password@vendor.example/docs',
                    'https:///missing-host', 'https://vendor.example/?utm_source=tracker',
                    'https://vendor.example/?fbclid=tracker', 'https://vendor.example/?gclid=tracker'):
            with self.subTest(url=url):
                data, tools, jobs = valid_fixture()
                data['tools'][0]['sources'][0]['url'] = url
                with self.assertRaises(ValueError):
                    online.validate_online(data, tools, jobs)

    def test_review_dates_are_real_and_not_newer_than_the_publication_review(self):
        for value in ('2025-02-30', '2025-99-99', '2025-1-1', '2025-01-02'):
            with self.subTest(entry_date=value):
                data, tools, jobs = valid_fixture()
                data['tools'][0]['reviewedAt'] = value
                with self.assertRaises(ValueError):
                    online.validate_online(data, tools, jobs)
        for value in ('2025-02-30', '2025-1-1', (date.today() + timedelta(days=1)).isoformat()):
            with self.subTest(directory_date=value):
                data, tools, jobs = valid_fixture()
                data['reviewedAt'] = value
                with self.assertRaises(ValueError):
                    online.validate_online(data, tools, jobs)

    def test_workflows_reject_unknown_references_and_duplicate_ids(self):
        for field, value in (('jobId', 'unknown-job'), ('toolIds', ['unknown-tool']),
                             ('toolIds', ['sample-local']), ('toolIds', []),
                             ('toolIds', ['sample-online', 'sample-online'])):
            with self.subTest(field=field, value=value):
                data, tools, jobs = valid_fixture()
                data['workflows'][0][field] = value
                with self.assertRaises(ValueError):
                    online.validate_online(data, tools, jobs)
        data, tools, jobs = valid_fixture()
        data['workflows'].append(copy.deepcopy(data['workflows'][0]))
        with self.assertRaises(ValueError):
            online.validate_online(data, tools, jobs)

    def test_workflows_require_safe_ids_and_nonblank_titles_and_summaries(self):
        for field, values in (('id', ('../outside', 'two words', '', 'UPPER')),
                              ('title', ('', '   ', None)), ('summary', ('', '   ', None))):
            for value in values:
                with self.subTest(field=field, value=value):
                    data, tools, jobs = valid_fixture()
                    data['workflows'][0][field] = value
                    with self.assertRaises(ValueError):
                        online.validate_online(data, tools, jobs)

    def test_workflows_require_exactly_three_concrete_steps(self):
        for steps in ([], ['One'], ['One', 'Two'], ['One', 'Two', 'Three', 'Four'],
                      ['One', '', 'Three'], ['One', '   ', 'Three'], ['One', None, 'Three']):
            with self.subTest(steps=steps):
                data, tools, jobs = valid_fixture()
                data['workflows'][0]['steps'] = steps
                with self.assertRaises(ValueError):
                    online.validate_online(data, tools, jobs)

    def test_featured_tools_are_unique_online_entries(self):
        for featured in (['unknown-tool'], ['sample-local'], ['sample-online', 'sample-online']):
            with self.subTest(featured=featured):
                data, tools, jobs = valid_fixture()
                data['featured'] = featured
                with self.assertRaises(ValueError):
                    online.validate_online(data, tools, jobs)


class OnlineDataFixture:
    @classmethod
    def setUpClass(cls):
        cls.data = json.loads((ROOT / 'data/online-tools.json').read_text())
        cls.tools = json.loads((ROOT / 'data/tools.json').read_text())
        cls.by_id = {tool['id']: tool for tool in cls.tools}
        cls.jobs = json.loads((ROOT / 'data/jobs.json').read_text())
        cls.access = json.loads((ROOT / 'data/tool-access.json').read_text())['tools']


class OnlineCatalogTests(OnlineDataFixture, unittest.TestCase):
    def test_all_online_records_and_relationships_validate(self):
        online.validate_online(self.data, self.tools, self.jobs)
        identities = [entry['toolId'] for entry in self.data['tools']]
        self.assertEqual(len(identities), len(set(identities)))
        self.assertEqual(set(identities), {tool['id'] for tool in self.tools if tool['category'] == 'online'})
        self.assertEqual({entry['group'] for entry in self.data['tools']},
                         {group['id'] for group in self.data['groups']})

    def test_existing_and_accepted_catalog_identities_are_preserved(self):
        self.assertTrue(PREEXISTING_TOOL_IDS <= set(self.by_id),
                        sorted(PREEXISTING_TOOL_IDS - set(self.by_id)))
        accepted = {entry['toolId'] for entry in self.data['tools']}
        self.assertTrue(ACCEPTED_ONLINE_IDS <= accepted, sorted(ACCEPTED_ONLINE_IDS - accepted))

    def test_online_launches_are_official_web_routes_with_honest_labels(self):
        for entry in self.data['tools']:
            identity = entry['toolId']
            tool, access = self.by_id[identity], self.access[identity]
            with self.subTest(tool=identity):
                action = access['primary']
                self.assertEqual(tool['mode'], 'web')
                self.assertEqual(action['kind'], 'web')
                self.assertEqual(action['url'], tool['downloadUrl'])
                self.assertEqual(access['officialUrl'], tool['homepage'])
                self.assertNotRegex(action['label'].lower(), r'\b(download|install|installer)\b')
                self.assertRegex(action['label'].lower(), r'\b(open|use|launch|visit)\b')
                self.assertEqual(urlparse(action['url']).scheme, 'https')
                self.assertTrue(urlparse(action['url']).hostname)
                self.assertIsNone(urlparse(action['url']).username)
                self.assertIsNone(urlparse(action['url']).password)
                self.assertNotRegex(urlparse(action['url']).path.lower(), r'\.(exe|msi|dmg|pkg|apk|zip|tar|gz)$')
                self.assertFalse(any(key.lower().startswith('utm_') or key.lower() in {'fbclid', 'gclid'}
                                     for key in parse_qs(urlparse(action['url']).query)))
                self.assertIn(action['evidenceUrl'], {source['url'] for source in entry['sources']})

    def test_unavailable_or_superseded_candidates_are_not_accepted_launches(self):
        # The primary-source review held these candidates or replaced their old
        # app route. A future reintroduction requires a fresh documented review.
        excluded = {'regexr.com', 'amiunique.org', 'crt.sh', 'euvd.enisa.europa.eu',
                    'observatory.mozilla.org', 'search.censys.io'}
        for entry in self.data['tools']:
            identity = entry['toolId']
            hostname = urlparse(self.access[identity]['primary']['url']).hostname.removeprefix('www.')
            self.assertNotIn(hostname, excluded, identity)
            self.assertNotIn(hostname, {'localhost', 'example.com', 'example.org', 'example.net'}, identity)
        self.assertEqual(urlparse(self.access['censys']['primary']['url']).hostname, 'platform.censys.io')
        self.assertEqual(urlparse(self.access['mdn-observatory']['primary']['url']).hostname, 'developer.mozilla.org')

    def test_review_evidence_covers_every_entry_without_future_or_missing_notes(self):
        reviewed = date.fromisoformat(self.data['reviewedAt'])
        self.assertLessEqual(reviewed, date.today())
        for entry in self.data['tools']:
            with self.subTest(tool=entry['toolId']):
                self.assertLessEqual(date.fromisoformat(entry['reviewedAt']), reviewed)
                for field in ('account', 'sharing', 'firstAction', 'expectedResult'):
                    self.assertTrue(entry[field].strip(), field)
                self.assertTrue(entry['sources'])
                for source in entry['sources']:
                    self.assertTrue(source['title'].strip())
                    self.assertTrue(source['evidence'].strip())
                    self.assertEqual(urlparse(source['url']).scheme, 'https')

    def test_every_task_has_a_real_guide_three_steps_and_launchable_tools(self):
        jobs = {job['id']: job for job in self.jobs}
        for task in self.data['workflows']:
            with self.subTest(task=task['id']):
                self.assertIn(task['jobId'], jobs)
                self.assertEqual(len(task['steps']), 3)
                self.assertTrue(all(step.strip() for step in task['steps']))
                self.assertTrue(task['toolIds'])
                for identity in task['toolIds']:
                    self.assertIn(identity, self.by_id)
                    self.assertIn(identity, self.access)
                    self.assertIn(identity, jobs[task['jobId']]['tools'])


class NavigationLinks(HTMLParser):
    def __init__(self):
        super().__init__()
        self.links = []

    def handle_starttag(self, tag, attrs):
        if tag == 'a':
            self.links.append(dict(attrs))


class OnlineGeneratedOutputTests(OnlineDataFixture, unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        super().setUpClass()
        cls.outputs = build.build_outputs()
        cls.catalog = json.loads(cls.outputs['site/catalog.json'])
        html = cls.outputs['site/index.html']
        start = html.index('window.INTA_DATA = ') + len('window.INTA_DATA = ')
        cls.embedded, _ = json.JSONDecoder().raw_decode(html[start:])

    def test_complete_catalog_and_online_metadata_reach_both_payloads(self):
        reputation = json.loads((ROOT / 'data/tool-reputation.json').read_text())['tools']
        expected_tools = [{**tool, 'reputation': reputation[tool['id']]} for tool in self.tools]
        for payload in (self.catalog, self.embedded):
            self.assertEqual(payload['onlineTools'], self.data)
            self.assertEqual(payload['tools'], expected_tools)
            self.assertEqual(payload['toolAccess'], self.access)
            self.assertTrue(PREEXISTING_TOOL_IDS <= {tool['id'] for tool in payload['tools']})
            self.assertTrue(ACCEPTED_ONLINE_IDS <= {entry['toolId'] for entry in payload['onlineTools']['tools']})

    def test_shell_has_a_real_keyboard_accessible_online_navigation_link(self):
        parser = NavigationLinks()
        parser.feed(self.outputs['site/index.html'])
        links = [link for link in parser.links if link.get('data-nav') == 'online']
        self.assertEqual(len(links), 1)
        self.assertEqual(links[0].get('href'), '#online')
        self.assertNotEqual(links[0].get('tabindex'), '-1')
        self.assertNotEqual(links[0].get('aria-hidden'), 'true')

    def test_every_group_and_entry_reaches_the_wiki_with_its_exact_routes(self):
        sidebar = self.outputs['wiki/_Sidebar.md']
        home = self.outputs['wiki/Online-Tools.md']
        for group in self.data['groups']:
            path = 'wiki/Online-' + group['id'] + '.md'
            self.assertIn(path, self.outputs)
            self.assertIn('](Online-' + group['id'] + ')', sidebar)
            self.assertIn('](Online-' + group['id'] + ')', home)
            self.assertIn(online.HUB + '#online?group=' + group['id'], self.outputs[path])
        for entry in self.data['tools']:
            with self.subTest(tool=entry['toolId']):
                identity = entry['toolId']
                page = self.outputs['wiki/Online-' + entry['group'] + '.md']
                self.assertEqual(page.count('<a id="' + identity + '"></a>'), 1)
                for url in (self.access[identity]['primary']['url'], self.access[identity]['officialUrl'],
                            self.by_id[identity]['docsUrl'], *[source['url'] for source in entry['sources']]):
                    self.assertIn('](' + url + ')', page)
                self.assertIn('Reviewed ' + entry['reviewedAt'], page)
                for heading in ('**Start here:**', '**Read the result:**', '**Access:**', '**Submission behavior:**'):
                    self.assertIn(heading, page)

    def test_all_task_links_and_first_actions_reach_the_front_pages(self):
        paths = ('README.md', 'docs/start-here.md', 'docs/free-tool-picks.md', 'wiki/Home.md', 'wiki/Start-Here.md')
        for path in paths:
            page = self.outputs[path]
            self.assertIn(online.HUB + '#online', page, path)
            for task in self.data['workflows']:
                with self.subTest(page=path, task=task['id']):
                    self.assertIn(online.HUB + '#online?task=' + task['id'], page)
                    for identity in task['toolIds']:
                        self.assertIn('](' + self.access[identity]['primary']['url'] + ')', page)
        page = self.outputs['wiki/Online-Tools.md']
        self.assertNotRegex(page, r'<(?:details|summary)\b')
        for task in self.data['workflows']:
            self.assertEqual(page.count('<a id="' + task['id'] + '"></a>'), 1)
            self.assertIn(online.WIKI + 'Job-' + task['jobId'], page)
            self.assertIn(online.HUB + '#job/' + task['jobId'], page)
            self.assertIn('wiki/Job-' + task['jobId'] + '.md', self.outputs)

    def test_manifest_reports_complete_online_content(self):
        manifest = json.loads(self.outputs['site/build-manifest.json'])
        for field, key in (('onlineTools', 'tools'), ('onlineGroups', 'groups'), ('onlineWorkflows', 'workflows')):
            self.assertEqual(manifest[field], len(self.data[key]))
        self.assertEqual(manifest['tools'], len(self.tools))


if __name__ == '__main__':
    unittest.main()
