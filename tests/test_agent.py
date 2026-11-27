import hashlib
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
import zipfile

ROOT = Path(__file__).resolve().parents[1]
AGENT = ROOT / "agent" / "INTA.LocalAgent"


class LocalAgentContract(unittest.TestCase):
    def test_agent_binds_only_to_loopback_and_checks_browser_origin(self):
        source = (AGENT / "Program.cs").read_text()
        self.assertIn("IPAddress.Loopback", source)
        self.assertIn("IPAddress.IPv6Loopback", source)
        self.assertIn("IPAddress.IsLoopback(remote)", source)
        self.assertIn("allowedOrigins.Contains(origin)", source)
        self.assertNotIn("IPAddress.Any", source)
        self.assertNotIn("IPAddress.IPv6Any", source)

    def test_agent_uses_typed_operations_and_one_confirmation(self):
        source = (AGENT / "Program.cs").read_text()
        self.assertIn('request.Confirmed', source)
        self.assertIn('"update-all"', source)
        self.assertIn('request.Action is not ("update" or "update-all" or "repair" or "uninstall")', source)
        self.assertIn("PackageManagers.ValidateId", source)
        self.assertNotIn("ProcessStartInfo", source)

    def test_process_runner_never_invokes_a_shell_or_kills_installers(self):
        source = (AGENT / "ProcessRunner.cs").read_text()
        self.assertIn("UseShellExecute = false", source)
        self.assertIn("start.ArgumentList.Add(arg)", source)
        self.assertIn("WaitForExitAsync(CancellationToken.None)", source)
        self.assertNotIn("process.Kill", source)
        self.assertNotIn("cmd.exe", source.lower())
        self.assertNotIn("powershell.exe", source.lower())

    def test_manager_operations_are_allowlisted(self):
        source = (AGENT / "PackageManagers.cs").read_text()
        for manager in ("winget", "choco", "scoop", "apt", "dnf", "pacman", "brew", "flatpak", "snap", "uniget", "windows-update"):
            self.assertIn(f'"{manager}"', source)
        self.assertIn("public static string ValidateId", source)
        self.assertIn("VerifyAsync", source)
        self.assertIn('("snap", "rollback")', source)
        self.assertIn("windows-update", source)
        self.assertIn("NormalizeHotFixId", source)

    def test_agent_artifact_packaging_creates_sha256_sidecar(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            publish = root / "publish"
            publish.mkdir()
            (publish / "inta-agent.exe").write_bytes(b"test-binary")
            archive = root / "inta-agent-win-x64.zip"
            subprocess.run([sys.executable, str(ROOT / "agent" / "package.py"), str(publish), str(archive)], check=True, capture_output=True, text=True)
            sidecar = Path(str(archive) + ".sha256")
            self.assertTrue(archive.exists())
            self.assertTrue(sidecar.exists())
            expected = hashlib.sha256(archive.read_bytes()).hexdigest()
            self.assertTrue(sidecar.read_text().startswith(expected))
            with zipfile.ZipFile(archive) as zipped:
                self.assertEqual(zipped.namelist(), ["inta-agent.exe"])

    def test_local_agent_workflow_builds_supported_runtime_archives(self):
        source = (ROOT / ".github" / "workflows" / "local-agent.yml").read_text()
        for rid in ("win-x64", "linux-x64", "osx-arm64", "osx-x64"):
            self.assertIn(rid, source)
        self.assertIn("dotnet-version: '10.0.x'", source)
        self.assertIn("PublishSingleFile=true", source)
        self.assertIn("sha256", (ROOT / "agent" / "package.py").read_text().lower())


if __name__ == "__main__":
    unittest.main()
