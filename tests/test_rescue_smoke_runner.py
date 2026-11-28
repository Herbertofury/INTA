"""Exercise the real host boot gate against deterministic emulator outcomes."""
import os
from pathlib import Path
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
PASS = b"INTA_SMOKE_START\nXFCE ready\nINTA_SMOKE_PASS\n"
EMULATOR = """#!/usr/bin/env python3
import os
from pathlib import Path
import sys
args = sys.argv[1:]
assert args[args.index('-nic') + 1] == 'none'
assert args[args.index('-smbios') + 1] == 'type=1,serial=INTA_CI_SMOKE'
serial = args[args.index('-serial') + 1]
assert serial.startswith('file:')
case = os.environ['INTA_TEST_OUTCOME']
if case == 'clean':
    Path(serial[5:]).write_bytes(b'INTA_SMOKE_START\\nXFCE ready\\nINTA_SMOKE_PASS\\n')
elif case == 'partial':
    Path(serial[5:]).write_bytes(b'INTA_SMOKE_START\\n')
elif case == 'failed':
    Path(serial[5:]).write_bytes(b'INTA_SMOKE_START\\nXFCE ready\\nINTA_SMOKE_PASS\\n')
    sys.exit(17)
"""


class RescueSmokeRunner(unittest.TestCase):
    def launch(self, outcome, *, stale=False, missing_iso=False, linked_log=False):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            bin_dir = root / "bin"
            bin_dir.mkdir()
            for name, body in (
                ("qemu-system-x86_64", EMULATOR),
                ("sleep", "#!/bin/sh\nexec /bin/sleep 0.04\n"),
            ):
                file = bin_dir / name
                file.write_text(body, encoding="utf-8")
                file.chmod(0o755)
            iso = root / "candidate.iso"
            if not missing_iso:
                iso.write_bytes(b"fixture, not an ISO image")
            diagnostics = root / "diagnostics"
            diagnostics.mkdir()
            serial = diagnostics / "bios-serial.log"
            if stale:
                serial.write_bytes(PASS)
            if linked_log:
                target = root / "do-not-write"
                target.write_bytes(b"leave this untouched")
                serial.symlink_to(target)
            env = {**os.environ, "PATH": str(bin_dir) + os.pathsep + os.environ["PATH"],
                   "INTA_TEST_OUTCOME": outcome}
            result = subprocess.run([str(ROOT / "scripts/smoke-iso"), str(iso), "bios", str(diagnostics)],
                                    env=env, capture_output=True, timeout=8)
            if linked_log:
                self.assertEqual(target.read_bytes(), b"leave this untouched")
                self.assertFalse(serial.is_symlink())
            return result, serial.read_bytes() if serial.exists() else None

    def test_current_clean_guest_output_is_required(self):
        result, serial = self.launch("clean")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(serial, PASS)

    def test_a_previous_pass_cannot_certify_a_guest_with_no_output(self):
        result, serial = self.launch("silent", stale=True)
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(serial, b"")

    def test_partial_output_and_nonzero_emulator_exit_never_pass(self):
        partial, serial = self.launch("partial", stale=True)
        self.assertNotEqual(partial.returncode, 0)
        self.assertEqual(serial, b"INTA_SMOKE_START\n")
        failed, serial = self.launch("failed")
        self.assertEqual(failed.returncode, 17)
        self.assertEqual(serial, PASS)

    def test_old_diagnostic_symlink_is_replaced_without_touching_its_target(self):
        result, serial = self.launch("clean", linked_log=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(serial, PASS)

    def test_missing_iso_is_refused_even_if_emulator_would_report_success(self):
        result, serial = self.launch("clean", missing_iso=True)
        self.assertNotEqual(result.returncode, 0)
        self.assertIsNone(serial)
