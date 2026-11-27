import copy
from datetime import date
import importlib.util
import json
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('knowledge_health', ROOT / 'scripts/knowledge_health.py')
health = importlib.util.module_from_spec(spec)
spec.loader.exec_module(health)


class OrganizedKnowledge(unittest.TestCase):
    def setUp(self):
        self.policy = json.loads((ROOT / 'data/knowledge-policy.json').read_text())
        self.tools = [{'id': 'example-tool', 'name': 'Example Tool', 'homepage': 'https://example.org/tool',
                       'tier': 'evaluate', 'reviewedAt': '2026-11-26'}]
        self.jobs = [{'id': 'example-job', 'title': 'Restore a real task', 'reviewedAt': '2026-11-26'}]
        self.layout = {'groups': [{'id': 'service', 'jobs': ['example-job']}], 'categories': {'example': {}}}
        self.notes = {'schemaVersion': 1, 'reviewedAt': '2026-11-26', 'notes': [{
            'id': 'identify-stale-port', 'jobId': 'example-job', 'title': 'Identify the stale port',
            'symptom': 'The target changed address.', 'firstAction': 'Read the actual target address.',
            'steps': ['Compare the selected destination with the known target.'],
            'expected': 'The destination matches the target.', 'ifUnresolved': 'Inspect the next network hop.',
            'toolIds': ['example-tool'], 'sourceUrls': ['https://example.org/docs'], 'reviewedAt': '2026-11-26'}]}
        self.queue = {'schemaVersion': 1, 'items': [{
            'id': kind + '-review', 'kind': kind, 'targetId': target, 'priority': 2, 'status': 'queued',
            'question': 'Which condition changes the correct route?', 'deliverable': 'A tested branch.',
            'acceptance': 'The test distinguishes the two conditions.', 'sourceUrls': ['https://example.org/docs']
        } for kind, target in [('job', 'example-job'), ('category', 'example')]]}
        self.workspaces = {'jobs': {'example-job': {'resources': [{
            'id': 'official-guide', 'label': 'Official guide', 'sourceUrl': 'https://example.org/docs',
            'checkedAt': '2026-11-26'}], 'decisions': []}}}
        action = {'label': 'Open instructions', 'checkedAt': '2026-11-26', 'evidenceUrl': 'https://example.org/docs'}
        self.access = {'example-tool': {'primary': action, 'alternates': [dict(action, id='second-platform')]}}

    def validate(self):
        health.validate_organization(self.policy, self.queue, self.tools, self.jobs, self.layout, self.notes)

    def report(self, as_of):
        return health.knowledge_report(self.policy, self.queue, self.tools, self.jobs, self.layout,
                                       self.notes, self.workspaces, self.access, as_of)

    def test_duplicate_tool_with_spacing_and_case_cannot_enter_catalog(self):
        self.tools.append(dict(self.tools[0], id='duplicate', name='EXAMPLE—tool', homepage='https://example.org/other'))
        with self.assertRaisesRegex(ValueError, 'Duplicate normalized tool name'):
            self.validate()

    def test_scenario_choices_get_due_dates_without_rewriting_evidence(self):
        quick = {'items': [{'id': 'example-case', 'label': 'Fix the observed fault', 'choices': [{
            'toolId': 'example-tool', 'role': 'Confirm the cause', 'reviewedAt': '2026-11-26',
            'evidenceUrls': ['https://example.org/current-guide']}]}]}
        before = copy.deepcopy(quick)
        report = health.knowledge_report(self.policy, self.queue, self.tools, self.jobs, self.layout,
                                         self.notes, self.workspaces, self.access, '2026-12-27', quick)
        choices = [row for row in report['dueReviews'] if row['kind'] == 'tool-choice']
        self.assertEqual(report['counts']['scenarioToolChoices'], 1)
        self.assertEqual(len(choices), 1)
        self.assertEqual(choices[0]['dueAt'], '2026-12-26')
        self.assertEqual(choices[0]['daysOverdue'], 1)
        self.assertEqual(quick, before)

    def test_same_product_page_needs_an_explained_exception(self):
        self.tools.append(dict(self.tools[0], id='other-product', name='Other product', homepage='https://EXAMPLE.org/tool/'))
        with self.assertRaisesRegex(ValueError, 'Duplicate official tool page'):
            self.validate()
        self.policy['duplicateOfficialPageAllowlist'] = [{'toolIds': ['example-tool', 'other-product'],
                                                         'reason': 'Distinct supported products sharing a publisher page.'}]
        self.validate()
        self.policy['duplicateOfficialPageAllowlist'][0]['reason'] = ''
        with self.assertRaisesRegex(ValueError, 'known tools and a reason'):
            self.validate()

    def test_a_new_job_cannot_be_left_without_specific_fixes(self):
        self.jobs.append({'id': 'new-job', 'title': 'New task'})
        with self.assertRaisesRegex(ValueError, 'every job'):
            health.validate_field_notes(self.notes, self.jobs, self.tools)

    def test_duplicate_service_homes_are_rejected(self):
        self.layout['groups'].append({'id': 'second-home', 'jobs': ['example-job']})
        with self.assertRaisesRegex(ValueError, 'one service home'):
            self.validate()

    def test_fixes_need_real_outcomes_fallback_and_safe_sources(self):
        for field in ('symptom', 'firstAction', 'steps', 'expected', 'ifUnresolved', 'sourceUrls'):
            invalid = copy.deepcopy(self.notes)
            invalid['notes'][0][field] = [] if isinstance(invalid['notes'][0][field], list) else ''
            with self.subTest(field=field), self.assertRaises(ValueError):
                health.validate_field_notes(invalid, self.jobs, self.tools)
        for url in ('javascript:alert(1)', 'https://user:secret@example.org/docs', 'https://example.org/?utm_source=ad'):
            invalid = copy.deepcopy(self.notes)
            invalid['notes'][0]['sourceUrls'] = [url]
            with self.subTest(url=url), self.assertRaisesRegex(ValueError, 'public HTTPS'):
                health.validate_field_notes(invalid, self.jobs, self.tools)

    def test_duplicate_fix_and_unknown_tool_do_not_become_navigation_dead_ends(self):
        invalid = copy.deepcopy(self.notes)
        invalid['notes'].append(copy.deepcopy(invalid['notes'][0]))
        with self.assertRaisesRegex(ValueError, 'field-note ID'):
            health.validate_field_notes(invalid, self.jobs, self.tools)
        invalid['notes'].pop()
        invalid['notes'][0]['toolIds'] = ['does-not-exist']
        with self.assertRaisesRegex(ValueError, 'unknown or duplicate'):
            health.validate_field_notes(invalid, self.jobs, self.tools)

    def test_research_cannot_neglect_a_category_or_claim_unpublished_completion(self):
        self.queue['items'].pop()
        with self.assertRaisesRegex(ValueError, 'every category'):
            self.validate()
        self.setUp()
        self.queue['items'][0]['status'] = 'done'
        with self.assertRaisesRegex(ValueError, 'completed research needs a result'):
            self.validate()

    def test_freshness_is_complete_and_never_rewrites_source_dates(self):
        before = json.dumps([self.tools, self.jobs, self.notes, self.workspaces, self.access], sort_keys=True)
        report = self.report('2026-12-10')
        self.assertEqual(report['counts']['reviewedRecords'], 6)
        self.assertEqual(len(report['reviews']), 6)
        self.assertEqual(report['dueReviews'][0]['id'], 'example-tool')
        self.assertEqual(report['dueReviews'][0]['status'], 'due')
        self.assertEqual(report['dueReviews'][0]['daysOverdue'], 0)
        later = self.report('2026-12-11')
        self.assertEqual(later['dueReviews'][0]['status'], 'overdue')
        self.assertEqual(later['dueReviews'][0]['daysOverdue'], 1)
        self.assertEqual(before, json.dumps([self.tools, self.jobs, self.notes, self.workspaces, self.access], sort_keys=True))

    def test_future_review_dates_are_not_accepted_as_fresh_evidence(self):
        self.tools[0]['reviewedAt'] = '2026-11-27'
        with self.assertRaisesRegex(ValueError, 'Future review date'):
            self.report(date(2026, 11, 26))

    def test_actual_repository_has_one_complete_organization(self):
        read = lambda name: json.loads((ROOT / 'data' / (name + '.json')).read_text())
        health.validate_organization(read('knowledge-policy'), read('research-queue'), read('tools'),
                                     read('jobs'), read('wiki-layout'), read('field-notes'))

    def test_cross_page_case_links_open_the_exact_working_route(self):
        spec = importlib.util.spec_from_file_location('knowledge_builder_test', ROOT / 'scripts/build-hub.py')
        builder = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(builder)
        output = builder.build_outputs()
        notes = json.loads((ROOT / 'data/field-notes.json').read_text())['notes']
        for note in notes:
            route = f"https://herbertofury.github.io/INTA/#job/{note['jobId']}?section=fix-{note['id']}"
            self.assertIn('](' + route + ')', output['wiki/Service-Map.md'])
            self.assertNotIn('/Job-' + note['jobId'] + '#fix-', output['wiki/Service-Map.md'])
        quick = json.loads((ROOT / 'data/quick-start.json').read_text())['items']
        for item in quick:
            self.assertIn(f"#job/{item['jobId']}/quick/{item['id']}?section=fixes", output[f"wiki/{item['wikiPage']}.md"])


if __name__ == '__main__':
    unittest.main()
