import json
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[1] / "data"


class MaintenanceRecommendations(unittest.TestCase):
    def load(self, name):
        return json.loads((ROOT / f"{name}.json").read_text(encoding="utf-8"))

    def test_specialist_tools_are_in_the_working_kit(self):
        tools = {row["id"]: row for row in self.load("tools")}
        kit = self.load("job-workspaces")["jobs"]["preventive-maintenance"]
        for identity in ("mangodisk", "optimizerduck"):
            self.assertEqual(tools[identity]["tier"], "specialist")
            self.assertIn(identity, {tool["toolId"] for tool in kit["tools"]})

    def test_acquisition_boundaries_and_platforms(self):
        access = self.load("tool-access")["tools"]
        mango = access["mangodisk"]
        routes = [mango["primary"], *mango["alternates"]]
        self.assertEqual(mango["primary"]["id"], "windows-portable")
        self.assertEqual(len([r for r in routes if r["kind"] == "file"]), 7)
        for route in routes:
            if route["kind"] == "file":
                self.assertRegex(route["sha256"], r"^[0-9a-f]{64}$")
                self.assertIn("/harry0703/MangoDisk/releases/download/", route["url"])
        duck = access["optimizerduck"]
        self.assertEqual(duck["primary"]["kind"], "release-page")
        self.assertIn("/itsfatduck/optimizerDuck/releases/", duck["primary"]["url"])
        self.assertFalse(any(r["kind"] == "file" for r in duck["alternates"]))
