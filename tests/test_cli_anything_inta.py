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

    def test_startup_folder_opt_in_and_private_field_refusal(self):
        sources = [dict(scope=scope, view=view, key=key, status="read", entryNames=[], truncated=False)
                   for scope, view in (("current-user", "shared"), ("machine", "default"))
                   for key in ("Run", "RunOnce")]
        folders = [dict(scope="current-user", status="read", entryNames=["Example tool.lnk"], truncated=False),
                   dict(scope="all-users", status="not-present", entryNames=[], truncated=False)]
        report = dict(schema=2, scope="windows-logon-registrations-and-local-startup-folders",
                      readOnly=True, enabledState="not-evaluated", argumentsCollected=False,
                      sources=sources, startupFolders=folders, startupSettingsUri="ms-settings:startupapps")
        def response():
            return subprocess.CompletedProcess([], 0, json.dumps(report).encode(), b"")
        with patch.object(harness.subprocess, "run", return_value=response()) as call:
            self.assertEqual(harness.startup_inventory(self.binary, self.pin, True)["status"], "ready")
        call.assert_called_once_with([str(self.binary), "--startup-inventory", "--include-folders"],
                                    capture_output=True, check=True, shell=False, timeout=12)
        folders[1]["status"] = "location-not-scanned"
        with patch.object(harness.subprocess, "run", return_value=response()):
            self.assertEqual(harness.startup_inventory(self.binary, self.pin, True)["status"], "incomplete")
            with self.assertRaises(ValueError):
                harness.startup_inventory(self.binary, self.pin)
        for replacement in (["C:/private/name"], ["two", "two"], [".."], ["unexpected:stream"], [None]):
            with self.subTest(replacement=replacement):
                folders[0]["entryNames"] = replacement
                with patch.object(harness.subprocess, "run", return_value=response()):
                    with self.assertRaises(ValueError):
                        harness.startup_inventory(self.binary, self.pin, True)
        folders[0]["entryNames"] = []
        folders[0]["path"] = "C:/private"
        with patch.object(harness.subprocess, "run", return_value=response()):
            with self.assertRaises(ValueError):
                harness.startup_inventory(self.binary, self.pin, True)

    def test_program_display_metadata_only(self):
        report = dict(schema=1, scope="windows-desktop-registrations", readOnly=True,
                      commandsCollected=False, skipped=0, errors=0, capped=False,
                      rows=[dict(name="Synthetic", version="1.0", publisher="Publisher", scope="Current user")])
        def run():
            return subprocess.CompletedProcess([], 0, json.dumps(report).encode(), b"")
        with patch.object(harness.subprocess, "run", return_value=run()) as call:
            self.assertEqual(harness.installed_programs(self.binary, self.pin)["status"], "ready")
        call.assert_called_once_with([str(self.binary), "--installed-programs"], capture_output=True,
                                    check=True, shell=False, timeout=12)
        report["errors"] = 1
        with patch.object(harness.subprocess, "run", return_value=run()):
            self.assertEqual(harness.installed_programs(self.binary, self.pin)["status"], "incomplete")
        report["rows"][0]["uninstallString"] = "not a display field"
        with patch.object(harness.subprocess, "run", return_value=run()):
            with self.assertRaises(ValueError):
                harness.installed_programs(self.binary, self.pin)

    def test_storage_reports_only_valid_aggregate_totals_and_exact_arguments(self):
        target = self.binary.parent / "folder with spaces & symbols"
        report = dict(schemaVersion=1, resultType="storage-preview", observedAt="2026-11-28T02:59:20.1234567+00:00",
                      minimumAgeDays=7, entryLimit=5, entriesScanned=2, filesScanned=2, logicalBytes=9,
                      olderFiles=1, olderLogicalBytes=5, specialEntriesSkipped=0, readErrors=0,
                      depthLimitedDirectories=0, entryLimitReached=False, partial=False)
        def output():
            return subprocess.CompletedProcess([], 0, json.dumps(report).encode(), b"")
        with patch.object(harness.subprocess, "run", return_value=output()) as call:
            self.assertEqual(harness.storage_preview(self.binary, self.pin, target, 7, 5)["status"], "ready")
        call.assert_called_once_with([str(self.binary), "--storage-preview", str(target), "7", "5"],
                                    capture_output=True, check=True, shell=False, timeout=30)
        report.update(entriesScanned=5, entryLimitReached=True, partial=True)
        with patch.object(harness.subprocess, "run", return_value=output()):
            self.assertEqual(harness.storage_preview(self.binary, self.pin, target, 7, 5)["status"], "incomplete")
        for key, invalid in (("filesScanned", True), ("olderLogicalBytes", 10), ("partial", False),
                             ("observedAt", "2026-02-30T02:59:20+00:00"), ("entryLimit", 6), ("rootPath", str(target))):
            present, saved = key in report, report.get(key)
            report[key] = invalid
            with patch.object(harness.subprocess, "run", return_value=output()):
                with self.assertRaises(ValueError):
                    harness.storage_preview(self.binary, self.pin, target, 7, 5)
            if present:
                report[key] = saved
            else:
                del report[key]
        with patch.object(harness.subprocess, "run") as call:
            for folder, days, limit in ((pathlib.Path("relative"), 7, 5),
                                        (pathlib.Path(self.binary.anchor), 7, 5), (target, 7, 0), (target, -1, 5)):
                with self.assertRaises(ValueError):
                    harness.storage_preview(self.binary, self.pin, folder, days, limit)
        call.assert_not_called()

    def test_rclone_receives_only_bounded_stdin_and_pinned_command(self):
        binary = self.binary.rename(self.binary.with_name("rclone.exe" if harness.os.name == "nt" else "rclone"))
        data = b"local fixture"
        selected = binary.parent.resolve() / "private client & file.bin"
        selected.write_bytes(data)
        digest = hashlib.sha256(data).hexdigest()
        result = subprocess.CompletedProcess([], 0, (digest+"  -\n").encode(), b"unprojected")
        with patch.dict(harness.os.environ, {"RCLONE_CONFIG": "secret", "HTTPS_PROXY": "private"}):
            with patch.object(harness.subprocess, "run", return_value=result) as call:
                binary = binary.resolve()
                self.assertEqual(harness.rclone_local_sha256(binary, self.pin, selected, digest)["status"], "ready")
            cmd = call.call_args.args[0]
            self.assertEqual(cmd, [str(binary), "--config", harness.os.devnull, "hashsum", "sha256", "-"])
            self.assertEqual(call.call_args.kwargs["input"], data)
            self.assertFalse(call.call_args.kwargs["shell"])
            self.assertFalse({"RCLONE_CONFIG", "HTTPS_PROXY"} & set(call.call_args.kwargs["env"]))
        with patch.object(harness.subprocess, "run", return_value=result):
            self.assertEqual(harness.rclone_local_sha256(binary, self.pin, selected, "0" * 64)["status"], "mismatch")
        with patch.object(harness.subprocess, "run") as call:
            for file in (selected.parent, pathlib.Path("relative.bin")):
                with self.assertRaises(ValueError):
                    harness.rclone_local_sha256(binary, self.pin, file, digest)
            with patch.object(harness, "MAX_LOCAL_HASH", 2), self.assertRaises(ValueError):
                harness.rclone_local_sha256(binary, self.pin, selected, digest)
        call.assert_not_called()
        self.assertEqual(selected.read_bytes(), data)

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
