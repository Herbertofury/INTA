import pathlib
import unittest


ROOT = pathlib.Path(__file__).resolve().parents[1]


class ProgressionContractTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.app = (ROOT / "portal" / "app.js").read_text(encoding="utf-8")
        cls.template = (ROOT / "portal" / "index.template.html").read_text(encoding="utf-8")
        cls.styles = (ROOT / "portal" / "styles.css").read_text(encoding="utf-8")

    def test_progress_route_is_reachable(self):
        self.assertIn('href="#progress" data-nav="progress"', self.template)
        self.assertIn("route.name === 'progress'", self.app)
        self.assertIn("function progressionPage()", self.app)

    def test_awards_require_every_step_and_verification(self):
        self.assertIn("function fullWorkflowProgress(job)", self.app)
        self.assertIn("array(job.steps)", self.app)
        self.assertIn("array(job.verify)", self.app)
        self.assertIn("items.length > 0 && done === items.length", self.app)
        self.assertIn("awardWorkflowCompletion(job)", self.app)

    def test_reward_events_are_idempotent_per_playbook_rule_version(self):
        self.assertIn("workflow-complete:${job.id}:v1", self.app)
        self.assertIn("xpLedger.some(event => event.id === id)", self.app)
        self.assertIn("ruleVersion: 1", self.app)

    def test_progress_is_explicitly_browser_local_until_sync_exists(self):
        self.assertIn("This first release stores progression in this browser only", self.app)
        self.assertIn("does not yet sync across accounts or devices", self.app)

    def test_audio_is_opt_in_and_visual_state_is_always_available(self):
        self.assertIn("sound: false", self.app)
        self.assertIn("if (!feedbackPreferences.sound", self.app)
        self.assertIn("data-feedback-sound", self.app)
        self.assertIn(".feedback-toggle", self.styles)

    def test_rewards_do_not_replace_customer_service_evidence(self):
        self.assertIn("Record the verified result in your service ticket", self.app)
        self.assertIn("not independently audited certifications", self.app)
        self.assertIn("Do not use this local profile as an employee performance record", self.app)


if __name__ == "__main__":
    unittest.main()
