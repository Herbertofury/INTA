import hashlib
import importlib.util
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
