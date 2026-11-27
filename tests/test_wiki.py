import copy
import importlib.util
import json
import re
from pathlib import Path
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]


def module(filename):
    spec = importlib.util.spec_from_file_location(filename.replace('-', '_'), ROOT / 'scripts' / filename)
    value = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(value)
    return value


links = module('wiki_links.py')
audit = module('check-wiki.py')
build = module('build-hub.py')
brand_markup = module('brand_markdown.py')


class LinkedKnowledge(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.tools = build.records(ROOT / 'data/tools.json', 'tools')
        cls.jobs = build.records(ROOT / 'data/jobs.json', 'jobs')
        cls.registry = links.build_registry(cls.tools, cls.jobs)

    def test_repeated_mentions_and_longest_alias(self):
        text = 'Windows Recovery Environment uses SFC. SFC is distinct from SFClone.'
        output = links.link_markdown(text, self.registry)
        self.assertEqual(output.count('[SFC]('), 2)
        self.assertIn('[Windows Recovery Environment](', output)
        self.assertNotIn('[Windows](', output)
        self.assertIn('SFClone', output)
        self.assertEqual(links.link_markdown(output, self.registry), output)

    def test_existing_links_code_urls_and_commands_are_preserved(self):
        text = '''[PowerShell](https://example.com/docs_(current)) and ![Windows](https://example.com/image.svg)
[SFC][manual]
[manual]: https://example.com/SFC
<a href="https://example.com">PowerShell</a>
`SFC` and ``PowerShell ` SFC``
https://example.com/Windows
```powershell
sfc /scannow
```
```mermaid
flowchart TD
A["Windows"] --> B["PowerShell"]
```
    Windows command sample
<!-- PowerShell comment -->
'''
        self.assertEqual(links.link_markdown(text, self.registry), text)

    def test_prose_after_code_is_still_linked(self):
        text = '~~~text\nPowerShell\n~~~\nPowerShell\n'
        output = links.link_markdown(text, self.registry)
        self.assertEqual(output.count('[PowerShell]('), 1)

    def test_ambiguous_common_nouns_are_not_linked(self):
        text = 'Do everything with care. Everything search is available. Open the windows. Windows is a platform.'
        output = links.link_markdown(text, self.registry)
        self.assertIn('Do everything with care.', output)
        self.assertIn('Open the windows.', output)
        self.assertEqual(output.count('[Everything]('), 1)
        self.assertEqual(output.count('[Windows]('), 1)

    def test_registry_rejects_conflicts_and_executable_links(self):
        tools = copy.deepcopy(self.tools)
        tools.append(dict(tools[0], id='conflicting-alias', homepage='https://different.example/'))
        with self.assertRaisesRegex(ValueError, 'Ambiguous link alias'):
            links.build_registry(tools, self.jobs)
        tools = copy.deepcopy(self.tools)
        tools[0]['homepage'] = 'javascript:alert(1)'
        with self.assertRaisesRegex(ValueError, 'Unsafe registry URL'):
            links.build_registry(tools, self.jobs)

    def test_every_generated_wiki_page_passes_the_rules(self):
        pages = {p: text for p, text in build.build_outputs().items() if p.startswith('wiki/')}
        result = audit.inspect(pages, self.registry, self.jobs)
        self.assertEqual(result['errors'], [], '\n'.join(result['errors']))
        self.assertGreaterEqual(result['mermaidDiagrams'], len(self.jobs))

    def test_missing_page_and_unlinked_name_fail_the_gate(self):
        result = audit.inspect({'wiki/Home.md': 'PowerShell\n\n[Missing](Missing-Page)\n'}, self.registry, [])
        self.assertTrue(any('unlinked' in x for x in result['errors']))
        self.assertTrue(any('missing Wiki page' in x for x in result['errors']))

    def test_sidebar_exposes_every_scenario_and_tool_category_without_expansion(self):
        output = build.build_outputs()
        sidebar = brand_markup.strip_icons(output['wiki/_Sidebar.md'])
        defaults = {}
        for row in json.loads((ROOT / 'data/quick-start.json').read_text())['items']:
            defaults.setdefault(row['jobId'], row['wikiPage'])
        self.assertNotRegex(sidebar.lower(), r'<(?:details|summary)\b')
        for job in self.jobs:
            destination = re.escape(defaults[job['id']])
            self.assertRegex(sidebar, r'\*\*\[[^\]]+\]\((?:' + re.escape(build.WIKI) + r'/)?' + destination + r'\)\*\*')
            self.assertIn('wiki/Job-' + job['id'] + '.md', output)
        self.assertIn('[All playbooks](Playbooks)', sidebar)
        for category in {tool['category'] for tool in self.tools}:
            self.assertIn('(Tools-' + build.slug(category) + ')', sidebar)

    def test_complete_directory_exposes_official_tool_routes(self):
        directory = build.build_outputs()['wiki/Tool-Directory.md']
        access = json.loads((ROOT / 'data/tool-access.json').read_text())['tools']
        for tool in self.tools:
            self.assertIn('](' + tool['downloadUrl'] + ')', directory, tool['id'])
            self.assertIn('](' + access[tool['id']]['officialUrl'] + ')', directory, tool['id'])
            self.assertIn('](' + access[tool['id']]['primary']['url'] + ')', directory, tool['id'])

    def test_front_pages_share_direct_free_tool_and_scenario_routes(self):
        quick_start = json.loads((ROOT / 'data/quick-start.json').read_text())
        tools = {tool['id']: tool for tool in self.tools}
        access = json.loads((ROOT / 'data/tool-access.json').read_text())['tools']
        jobs = {job['id'] for job in self.jobs}
        output = build.build_outputs()
        rows = quick_start['items']
        self.assertTrue(rows)
        self.assertEqual(len({row['id'] for row in rows}), len(rows))
        self.assertEqual({row['jobId'] for row in rows}, jobs)
        self.assertIn('Remove viruses', {row['label'] for row in rows})
        self.assertIn('Fix a printer', {row['label'] for row in rows})
        for row in rows:
            self.assertIn(row['jobId'], jobs)
            if not row.get('guideOnly'):
                self.assertIn(row['toolId'], tools)
            self.assertTrue(row['freeUse'])
            self.assertTrue(row['qualityBasis'])
            self.assertTrue(row['evidenceUrls'])
            self.assertTrue(row['firstStep'])
            self.assertEqual(len(row['quickSteps']), 3)
            anchor = 'quick-' + row['id']
            guide_page = output['wiki/Job-' + row['jobId'] + '.md']
            self.assertIn('<a id="' + anchor + '"></a>', guide_page)
            self.assertLess(guide_page.index('<a id="' + anchor + '"></a>'), guide_page.index('<a id="walkthrough"></a>'))
            for path in ['README.md', 'docs/start-here.md', 'wiki/Home.md', 'wiki/Start-Here.md']:
                page = output[path]
                if not row.get('guideOnly'):
                    route = access[row['toolId']]
                    action = build.selected_tool_action(route, row.get('toolAction'))
                    for url in [route['officialUrl'], action['url']]:
                        self.assertIn('](' + url + ')', page, path + ': ' + row['id'])
                self.assertIn('](' + build.WIKI + '/' + row['wikiPage'] + ')', page, path + ': ' + row['id'])
                self.assertNotIn('](' + build.WIKI + '/Job-' + row['jobId'] + '#' + anchor + ')', page, path + ': ' + row['id'])
        for path in ['README.md', 'docs/start-here.md']:
            self.assertEqual(output[path].count('<!-- INTA:QUICK-START:START -->'), 1)
            self.assertEqual(output[path].count('<!-- INTA:QUICK-START:END -->'), 1)

    def test_each_quick_answer_has_a_complete_page_with_exact_steps_and_tool_action(self):
        rows = json.loads((ROOT / 'data/quick-start.json').read_text())['items']
        access = json.loads((ROOT / 'data/tool-access.json').read_text())['tools']
        output = build.build_outputs()
        manifest = json.loads(output['site/build-manifest.json'])
        self.assertEqual(manifest['quickAnswerPages'], len(rows))

        def plain(value):
            value = brand_markup.strip_icons(value)
            return re.sub(r'\[([^\]]+)\]\([^)]*\)', r'\1', value).replace('**', '')

        for row in rows:
            page_path = 'wiki/' + row['wikiPage'] + '.md'
            self.assertIn(page_path, output, row['id'])
            page = output[page_path]
            self.assertIn(plain(row['firstStep']), plain(page), row['id'])
            numbered = re.findall(r'^\d+\. (.+)$', page, flags=re.M)
            self.assertEqual([plain(line) for line in numbered], row['quickSteps'], row['id'])
            self.assertIn('**[Complete procedure](' + build.WIKI + '/Job-' + row['jobId'] + ')**', page)
            self.assertIn('**[Selection evidence](' + build.WIKI + '/Free-Tool-Picks)**', page)
            self.assertNotIn('#quick-', page)
            self.assertNotIn('```mermaid', page)
            self.assertNotRegex(page.lower(), r'<(?:details|summary)\b')
            if row.get('guideOnly'):
                self.assertIn('**No extra download needed.**', page)
            else:
                route = access[row['toolId']]
                selected = build.selected_tool_action(route, row.get('toolAction'))
                self.assertIn('](' + route['officialUrl'] + ')', page)
                self.assertIn('](' + selected['url'] + ')', page)
            for path in ['docs/free-tool-picks.md', 'wiki/Free-Tool-Picks.md']:
                evidence = output[path] if path in output else (ROOT / path).read_text()
                self.assertIn('[Quick answer](' + build.WIKI + '/' + row['wikiPage'] + ')', evidence)
                self.assertIn('[Complete procedure](' + build.WIKI + '/Job-' + row['jobId'] + ')', evidence)

    def test_quick_answer_slugs_are_stable_public_destinations(self):
        # Keep these public page names stable when display labels are edited.
        published = {
            'suspected-malware': 'Remove-viruses',
            'fix-a-printer': 'Fix-a-printer',
            'fix-internet': 'Fix-internet',
            'fix-email': 'Fix-email',
            'failing-drive': 'Recover-a-failing-drive',
            'deleted-files': 'Recover-deleted-files',
            'slow-pc': 'Fix-a-slow-PC',
            'remote-help': 'Help-remotely',
            'fix-windows-updates': 'Fix-Windows-updates',
            'windows-repair': 'Repair-Windows-files',
            'fix-boot-failure': 'Fix-a-PC-that-will-not-boot',
            'diagnose-crashes': 'Diagnose-crashes',
            'disk-space': 'Free-disk-space',
            'clone-a-drive': 'Clone-or-move-a-drive',
            'file-backup': 'Restore-a-file-backup',
            'boot-usb': 'Make-a-boot-USB',
            'set-up-a-new-pc': 'Set-up-a-new-PC',
            'maintain-a-pc': 'Maintain-a-PC',
            'start-a-repair': 'Start-a-repair',
            'contain-ransomware': 'Contain-ransomware',
            'recover-an-outage': 'Recover-an-office-outage',
            'repair-domain-trust': 'Repair-domain-sign-in',
            'troubleshoot-with-ai': 'Troubleshoot-with-AI',
            'complete-a-handoff': 'Finish-the-job-and-hand-over',
            'find-a-file': 'Find-a-file',
            'inspect-network-traffic': 'Inspect-network-traffic',
            'suspected-ram-fault': 'Test-memory',
        }
        rows = {row['id']: row for row in json.loads((ROOT / 'data/quick-start.json').read_text())['items']}
        for identity, page in published.items():
            self.assertEqual(rows[identity]['wikiPage'], page, identity)
        self.assertEqual(len({row['wikiPage'].casefold() for row in rows.values()}), len(rows))

    def test_quick_answer_slugs_reject_missing_unsafe_duplicate_and_reserved_names(self):
        quick = json.loads((ROOT / 'data/quick-start.json').read_text())
        access = json.loads((ROOT / 'data/tool-access.json').read_text())['tools']
        build.validate_quick_start(quick, self.tools, self.jobs, access)
        for invalid in ['', '../outside', 'Page#case', 'Page with spaces', 'Réparer',
                        'https://example.com/page', 'Home', 'start-here', 'Job-intake-triage', 'Tools-malware']:
            with self.subTest(slug=invalid):
                changed = copy.deepcopy(quick)
                changed['items'][0]['wikiPage'] = invalid
                with self.assertRaises(ValueError):
                    build.validate_quick_start(changed, self.tools, self.jobs, access)
        missing = copy.deepcopy(quick)
        del missing['items'][0]['wikiPage']
        with self.assertRaisesRegex(ValueError, 'requires wikiPage'):
            build.validate_quick_start(missing, self.tools, self.jobs, access)
        duplicate = copy.deepcopy(quick)
        duplicate['items'][1]['wikiPage'] = duplicate['items'][0]['wikiPage'].lower()
        with self.assertRaisesRegex(ValueError, 'duplicate quick-answer Wiki page'):
            build.validate_quick_start(duplicate, self.tools, self.jobs, access)

    def test_methods_have_sources_and_complete_operating_guidance(self):
        methods = json.loads((ROOT / 'data/methods.json').read_text())['methods']
        self.assertEqual(len({m['id'] for m in methods}), len(methods))
        for method in methods:
            for field in ['id', 'name', 'url', 'summary', 'when', 'proof', 'sourceUrls']:
                self.assertTrue(method.get(field), f"{method.get('id')}: missing {field}")
            self.assertTrue(all(url.startswith('https://') for url in method['sourceUrls']))

    def test_linked_guides_preserve_every_canonical_command(self):
        output = build.build_outputs()
        for job in self.jobs:
            for step in job['steps']:
                if step.get('command'):
                    for path in ['wiki/Job-' + job['id'] + '.md', 'docs/runbooks/' + job['id'] + '.md']:
                        self.assertIn('\n' + step['command'] + '\n```', output[path], path)

    def test_published_step_destinations_survive_new_steps(self):
        # These destinations were published before the procedures grew.
        # Numbering may change; a saved link must still reach the same action.
        published = {
            'malware-removal': {
                'Prepare trustworthy scanners': 'step-02',
                'Close the entry point and retest': 'step-06',
            },
            'failing-drive': {
                'Recover from a working copy': 'step-05',
                'Validate what was actually recovered': 'step-06',
            },
            'remote-support': {
                'Connect with the user present': 'step-03',
                'Close and verify': 'step-06',
            },
        }
        output = build.build_outputs()
        for job in self.jobs:
            if job['id'] not in published:
                continue
            page = output['wiki/Job-' + job['id'] + '.md']
            for title, destination in published[job['id']].items():
                step = next(s for s in job['steps'] if s['title'] == title)
                self.assertEqual(step.get('anchor'), destination)
                section = page.split(f'<a id="{destination}"></a>', 1)[1]
                section = section.split('\n### ', 1)[1].split('\n', 1)[0]
                self.assertIn(title, section)

    def test_step_destinations_reject_collisions(self):
        jobs = copy.deepcopy(self.jobs)
        jobs[0]['steps'][1]['anchor'] = jobs[0]['steps'][0]['anchor']
        with self.assertRaisesRegex(ValueError, 'duplicate step anchors'):
            build.validate(self.tools, jobs)


if __name__ == '__main__':
    unittest.main()
