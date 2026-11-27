#!/usr/bin/env python3
"""Exercise the actual read-only Local Agent power-plan command."""
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import unittest
import uuid

AGENT = sys.argv.pop(1)
GUID = re.compile(r"\b[0-9a-f]{8}(?:-[0-9a-f]{4}){3}-[0-9a-f]{12}\b", re.I)

class PowerPlanTests(unittest.TestCase):
    def run_agent(self, *args):
        return subprocess.run(["dotnet", AGENT, "--power-plans", *args],
                              capture_output=True, text=True, encoding="utf-8", timeout=15)

    def test_rejects_arguments_without_starting_the_agent_server(self):
        result = self.run_agent("--activate", "not-a-plan")
        self.assertEqual(result.returncode, 2, result)
        self.assertEqual(result.stdout, "")
        self.assertIn("Usage:", result.stderr)

    def test_readonly_host_inventory(self):
        def active():
            executable = shutil.which("powercfg.exe")
            if not executable:
                executable = str(Path(os.environ["SystemRoot"]) / "System32" / "powercfg.exe")
            result = subprocess.run([executable, "/getactivescheme"], capture_output=True,
                                    check=True, timeout=15)
            text = result.stdout.decode("ascii", errors="ignore")
            return str(uuid.UUID(GUID.search(text).group()))
        before = active() if os.name == "nt" else None
        result = self.run_agent()
        self.assertEqual(result.returncode, 0, result)
        self.assertEqual(result.stderr, "")
        snapshot = json.loads(result.stdout)
        if os.name != "nt":
            self.assertEqual(snapshot, {"status": "unsupported", "activeId": None,
                                        "plans": [], "errorCode": None})
            return
        self.assertEqual(snapshot["status"], "ready", snapshot)
        expected = active()
        self.assertEqual(before, expected)
        self.assertEqual(snapshot["activeId"], expected)
        ids = [str(uuid.UUID(row["id"])) for row in snapshot["plans"]]
        self.assertEqual(len(ids), len(set(ids)))
        self.assertIn(expected, ids)
        self.assertGreaterEqual(len(ids), 1)
        self.assertLessEqual(len(ids), 256)
        self.assertTrue(all(row["name"] is None or isinstance(row["name"], str)
                            for row in snapshot["plans"]))

if __name__ == "__main__":
    unittest.main()
