import importlib.util
import pathlib
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("cli_coverage", ROOT / "scripts/check-tool-cli.py")
control = importlib.util.module_from_spec(spec)
spec.loader.exec_module(control)


class Coverage(unittest.TestCase):
    def setUp(self):
        self.catalog = [{"id": "fake-tool"}, {"id": "unpinned"}]
        self.operation = {"state": "tested", "backend": "official-cli", "effect": "read-only", "platform": "windows",
                          "source": "https://example.com/official-docs", "version": "1.2.3",
                          "artifactSha256": "a" * 64, "runtimeEvidence": "https://example.com/window-test"}
        self.tool = {"scope": "complete", "requiredOperations": ["inventory"],
                     "operations": {"inventory": self.operation}}
        self.ledger = {"schemaVersion": 1, "tools": {"fake-tool": self.tool}}

    def test_full_explicit_coverage(self):
        rows = control.audit(self.catalog, self.ledger)["tools"]
        self.assertEqual(rows["fake-tool"]["status"], "ready")
        self.assertEqual(rows["unpinned"]["status"], "needs-tool-review")

    def test_partial_is_not_full_even_with_one_tested_operation(self):
        self.tool["scope"] = "partial"
        row = control.audit(self.catalog, self.ledger)["tools"]["fake-tool"]
        self.assertEqual(row["status"], "partial")
        self.assertEqual(row["tested"], ["inventory"])

    def test_unverified_not_admitted(self):
        self.operation["state"] = "candidate"
        row = control.audit(self.catalog, self.ledger)["tools"]["fake-tool"]
        self.assertEqual(row["status"], "partial")
        self.assertEqual(row["tested"], [])
        self.assertEqual(row["missing"], ["inventory"])

    def test_claim_without_evidence_refused(self):
        del self.operation["runtimeEvidence"]
        with self.assertRaises(ValueError):
            control.audit(self.catalog, self.ledger)

    def test_removed_tool_and_scope_mismatch_refused(self):
        with self.assertRaises(ValueError):
            control.audit(self.catalog[1:], self.ledger)
        self.tool["requiredOperations"] = ["not-inventory"]
        with self.assertRaises(ValueError):
            control.audit(self.catalog, self.ledger)


if __name__ == "__main__":
    unittest.main()
