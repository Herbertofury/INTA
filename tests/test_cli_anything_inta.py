import hashlib
import importlib.util
import json
import pathlib
import subprocess
import tempfile
import unittest
from unittest.mock import patch

ROOT = pathlib.Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("inta_harness", ROOT / "scripts/cli-anything-inta.py")
harness = importlib.util.module_from_spec(spec)
spec.loader.exec_module(harness)


class NativeHarness(unittest.TestCase):
    def setUp(self):
        fixture = tempfile.TemporaryDirectory()
        self.addCleanup(fixture.cleanup)
        self.binary = pathlib.Path(fixture.name) / "inta-agent"
        self.binary.write_bytes(b"mocked native fixture")
        self.pin = hashlib.sha256(self.binary.read_bytes()).hexdigest()

    def test_exact_read_only_backend_command(self):
        payload = b'{"status":"ready","activeId":"12345678-1234-4321-9876-123456789abc","plans":[{"id":"12345678-1234-4321-9876-123456789abc","name":"Balanced"}],"errorCode":null}'
        with patch.object(harness.subprocess, "run", return_value=subprocess.CompletedProcess([], 0, payload, b"")) as run:
            self.assertEqual(harness.inventory(self.binary, self.pin)["plans"][0]["name"], "Balanced")
        run.assert_called_once_with([str(self.binary), "--power-plans"], capture_output=True, check=True, shell=False, timeout=12)

    def test_startup_validates_source_coverage_and_never_invokes_shell(self):
        def source(scope, view, key, names=()):
            return dict(scope=scope, view=view, key=key, status="read", entryNames=list(names), truncated=False)
        rows = [source("current-user", "shared", "Run", ("Synthetic",)),
                source("current-user", "shared", "RunOnce"),
                source("machine", "default", "Run"),
                source("machine", "default", "RunOnce")]
        report = dict(schema=1, scope="windows-logon-run-keys-only", readOnly=True,
                      enabledState="not-evaluated", argumentsCollected=False, sources=rows)
        def result():
            return subprocess.CompletedProcess([], 0, json.dumps(report).encode(), b"")
        with patch.object(harness.subprocess, "run", return_value=result()) as run:
            self.assertEqual(harness.startup_inventory(self.binary, self.pin)["status"], "ready")
        run.assert_called_once_with([str(self.binary), "--startup-inventory"], capture_output=True,
                                    check=True, shell=False, timeout=12)
        rows[1]["status"] = "unavailable"
        with patch.object(harness.subprocess, "run", return_value=result()):
            self.assertEqual(harness.startup_inventory(self.binary, self.pin)["status"], "incomplete")
        rows[1]["entryNames"] = ["Unexpected"]
        with patch.object(harness.subprocess, "run", return_value=result()):
            with self.assertRaises(ValueError):
                harness.startup_inventory(self.binary, self.pin)
        rows[1] = rows[0]
        with patch.object(harness.subprocess, "run", return_value=result()):
            with self.assertRaises(ValueError):
                harness.startup_inventory(self.binary, self.pin)

    def test_bad_pin_never_launches(self):
        with patch.object(harness.subprocess, "run") as run:
            with self.assertRaises(ValueError):
                harness.inventory(self.binary, "c" * 64)
        run.assert_not_called()

    def test_ambiguous_or_empty_ready_response_refused(self):
        for payload in (b'{"status":"ready","status":"unsupported","activeId":null,"plans":[],"errorCode":null}',
                        b'{"status":"ready","activeId":null,"plans":[],"errorCode":null}'):
            with patch.object(harness.subprocess, "run", return_value=subprocess.CompletedProcess([], 0, payload, b"")):
                with self.assertRaises(ValueError):
                    harness.inventory(self.binary, self.pin)


if __name__ == "__main__":
    unittest.main()
