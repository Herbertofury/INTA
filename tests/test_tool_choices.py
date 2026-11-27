import copy
import html
import importlib.util
import json
import re
from pathlib import Path
import unittest


ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location('inta_choice_build', ROOT / 'scripts/build-hub.py')
build = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(build)


def visible_text(markdown):
    """Compare authored prose after links and decorative HTML are rendered."""
    without_tags = re.sub(r'<[^>]*>', '', markdown)
    without_links = re.sub(r'\[([^\]]+)\]\([^)]*\)', r'\1', without_tags)
    return re.sub(r'\s+', ' ', html.unescape(without_links).replace('**', ''))


class ScenarioToolChoices(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.tools = json.loads((ROOT / 'data/tools.json').read_text())
        cls.by_id = {tool['id']: tool for tool in cls.tools}
        cls.jobs = json.loads((ROOT / 'data/jobs.json').read_text())
        cls.quick = json.loads((ROOT / 'data/quick-start.json').read_text())
        cls.access = json.loads((ROOT / 'data/tool-access.json').read_text())['tools']

    def choice(self, tool_id, action_id=None):
        value = {
            'toolId': tool_id,
            'role': 'Investigate the observed fault',
            'why': 'Match this diagnostic to the affected device.',
            'freeUse': 'Use the supported edition for the device.',
            'evidenceUrls': [self.access[tool_id]['officialUrl']],
            'reviewedAt': '2026-11-26',
        }
        if action_id is not None:
            value['actionId'] = action_id
        return value

    def test_authored_order_and_original_primary_route_are_preserved(self):
        item = copy.deepcopy(next(row for row in self.quick['items'] if row['jobId'] == 'malware-removal'))
        item['choices'] = [self.choice(identity) for identity in ['malwarebytes', 'adwcleaner', 'bitdefender-free']]
        resolved = build.resolved_tool_choices(item, self.access)
        self.assertEqual([choice['toolId'] for choice in resolved[:3]], ['malwarebytes', 'adwcleaner', 'bitdefender-free'])
        expected = build.selected_tool_action(self.access[item['toolId']], item.get('toolAction'))['url']
        self.assertTrue(any(choice['toolId'] == item['toolId'] and
                            build.selected_tool_action(self.access[choice['toolId']], choice.get('actionId'))['url'] == expected
                            for choice in resolved))

    def test_named_primary_action_is_not_duplicated(self):
        item = copy.deepcopy(next(row for row in self.quick['items'] if row.get('toolId') == 'safety-scanner'))
        action = build.selected_tool_action(self.access[item['toolId']], item.get('toolAction'))
        item['choices'] = [self.choice(item['toolId'], action.get('id'))]
        self.assertEqual(len(build.resolved_tool_choices(item, self.access)), 1)

    def test_case_specific_action_survives_a_different_default_choice(self):
        item = copy.deepcopy(next(row for row in self.quick['items'] if row.get('toolAction') and
            build.selected_tool_action(self.access[row['toolId']], row['toolAction'])['url'] != self.access[row['toolId']]['primary']['url']))
        item['choices'] = [self.choice(item['toolId'])]
        resolved = build.resolved_tool_choices(item, self.access)
        self.assertEqual(len(resolved), 2)
        self.assertEqual(build.selected_tool_action(self.access[resolved[-1]['toolId']], resolved[-1].get('actionId')),
                         build.selected_tool_action(self.access[item['toolId']], item['toolAction']))

    def test_invalid_choice_fails_before_publication(self):
        cases = [
            ('unknown-tool', {'toolId': 'not-a-real-tool'}),
            ('missing-role', {'role': ''}),
            ('markup-role', {'role': '<hidden>Scan</hidden>'}),
            ('missing-reason', {'why': ''}),
            ('missing-access', {'freeUse': ''}),
            ('future-review', {'reviewedAt': '2999-01-01'}),
            ('unknown-action', {'actionId': 'not-a-real-acquisition'}),
            ('empty-action', {'actionId': ''}),
            ('missing-source', {'evidenceUrls': []}),
            ('tracking-source', {'evidenceUrls': ['https://example.com/?utm_source=tracking']}),
            ('unsafe-source', {'evidenceUrls': ['javascript:alert(1)']}),
        ]
        for name, changed in cases:
            with self.subTest(name=name):
                data = copy.deepcopy(self.quick)
                data['items'][0]['choices'] = [dict(self.choice('adwcleaner'), **changed)]
                with self.assertRaises(ValueError):
                    build.validate_quick_start(data, self.tools, self.jobs, self.access)
        data = copy.deepcopy(self.quick)
        data['items'][0]['choices'] = [self.choice('adwcleaner'), self.choice('adwcleaner')]
        with self.assertRaisesRegex(ValueError, 'duplicate tool choice'):
            build.validate_quick_start(data, self.tools, self.jobs, self.access)

    def test_choices_have_no_quantity_cap(self):
        data = copy.deepcopy(self.quick)
        selected = self.tools[:9]
        data['items'][0]['choices'] = [self.choice(tool['id']) for tool in selected]
        build.validate_quick_start(data, self.tools, self.jobs, self.access)
        rendered = '\n'.join(build.tool_choices_markdown(data['items'][0], self.by_id, self.access))
        for tool in selected:
            self.assertIn('](' + self.access[tool['id']]['officialUrl'] + ')', rendered)

    def test_all_reviewed_choices_reach_front_pages_and_answer_pages(self):
        outputs = build.build_outputs()
        visible = {path: visible_text(page) for path, page in outputs.items()
                   if path.endswith('.md')}
        for row in self.quick['items']:
            self.assertIn('choices', row, row['id'] + ' needs an explicit reviewed choice disposition')
            choices = build.resolved_tool_choices(row, self.access)
            if not row.get('guideOnly'):
                self.assertTrue(row['choices'], row['id'])
            for choice in choices:
                official = self.access[choice['toolId']]['officialUrl']
                download = build.selected_tool_action(self.access[choice['toolId']], choice.get('actionId'))['url']
                for path in ['README.md', 'docs/start-here.md', 'wiki/Home.md', 'wiki/Start-Here.md', 'wiki/' + row['wikiPage'] + '.md']:
                    with self.subTest(scenario=row['id'], tool=choice['toolId'], page=path):
                        self.assertIn('](' + official + ')', outputs[path])
                        self.assertIn('](' + download + ')', outputs[path])
                        self.assertTrue(choice['role'] in visible[path],
                                        'Missing visible role: ' + choice['role'])

    def test_named_malware_tools_precede_the_long_toolkit(self):
        outputs = build.build_outputs()
        for path in ['wiki/Job-malware-removal.md', 'wiki/Remove-viruses.md']:
            page = outputs[path]
            immediate = page.split('<a id="working-kit"></a>', 1)[0]
            for identity in ['malwarebytes', 'adwcleaner', 'bitdefender-free']:
                self.assertIn('](' + self.access[identity]['officialUrl'] + ')', immediate, path + ': ' + identity)
            self.assertNotRegex(immediate.lower(), r'<(?:details|summary)\b')

    def test_guide_only_cases_keep_their_checklist(self):
        for item in self.quick['items']:
            if item.get('guideOnly'):
                self.assertIsNone(item['toolId'])
                page = build.quick_answer_markdown(item, self.by_id, self.access, self.quick['reviewedAt'])
                self.assertIn('**No extra download needed.**', page)
                self.assertEqual(len(item['quickSteps']), 3)


if __name__ == '__main__':
    unittest.main()
