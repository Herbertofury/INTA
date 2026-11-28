"""Run the real pinned CLI over a disposable fixture, without machine inventory."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time

ROOT = Path(__file__).resolve().parents[1]
AGENT = Path(sys.argv[1]).resolve(strict=True)
with AGENT.open("rb") as stream:
    PIN = hashlib.file_digest(stream, "sha256").hexdigest()


def run(folder, *args, pin=PIN):
    return subprocess.run([sys.executable, "-B", str(ROOT / "scripts/cli-anything-inta.py"), "--json", "storage-preview",
                           "--folder", str(folder), "--agent-binary", str(AGENT), "--agent-sha256", pin, *args],
                          text=True, capture_output=True, check=False, timeout=40)


with tempfile.TemporaryDirectory(prefix="inta-cli storage fixture ") as directory:
    work = Path(directory).resolve()
    target = work / "synthetic things & diagnostics"
    target.mkdir()
    nested = target / "nested"
    nested.mkdir()
    old = nested / "old private name.bin"
    fresh = target / "new private name.bin"
    old.write_bytes(b"old!")
    fresh.write_bytes(b"recent bytes")
    past = time.time() - 14 * 86400
    os.utime(old, (past, past))
    def fixture_state():
        return {item.relative_to(target).as_posix():
                (item.stat().st_mtime_ns, item.read_bytes()) if item.is_file() else None
                for item in target.rglob("*")}
    initial = fixture_state()
    ok = run(target)
    assert ok.returncode == 0 and not ok.stderr, "Expected successful metadata-only snapshot"
    total = json.loads(ok.stdout)
    assert total["operation"] == "storage-preview" and total["status"] == "ready" and total["partial"] is False
    assert (total["entriesScanned"], total["filesScanned"], total["logicalBytes"],
            total["olderFiles"], total["olderLogicalBytes"]) == (3, 2, len(b"old!recent bytes"), 1, len(b"old!"))
    for secret in (str(target), old.name, fresh.name, "recent bytes"):
        assert secret not in ok.stdout and secret not in ok.stderr
    bounded = run(target, "--entry-limit", "1")
    limited = json.loads(bounded.stdout)
    assert bounded.returncode == 1 and not bounded.stderr
    assert limited["status"] == "incomplete" and limited["entryLimitReached"] is True and limited["entriesScanned"] == 1
    for refused in (run(target, pin="0" * 64), run(target / "not present"), run(target.anchor)):
        assert refused.returncode == 2 and not refused.stderr
        assert json.loads(refused.stdout) == {"operation": "storage-preview", "status": "refused"}
    assert fixture_state() == initial, "A preview changed fixture bytes, modification times, or inventory"
print(json.dumps({"operation": "storage-preview", "passed": True, "agentSha256": PIN,
                  "sourceCommit": os.environ.get("GITHUB_SHA"), "platform": sys.platform}))
