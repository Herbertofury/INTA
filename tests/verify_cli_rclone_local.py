"""Verify pinned publisher rclone on a non-sensitive, disposable local fixture."""
import hashlib
import importlib.util
import json
import os
import re
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]
binary = Path(sys.argv[1]).resolve(strict=True)
with binary.open("rb") as stream:
    pin = hashlib.file_digest(stream, "sha256").hexdigest()
s = importlib.util.spec_from_file_location("inta_rclone_cli", ROOT / "scripts" / "cli-anything-inta.py")
harness = importlib.util.module_from_spec(s)
s.loader.exec_module(harness)

with tempfile.TemporaryDirectory(prefix="inta-rclone local proof ") as tmp:
    work = Path(tmp).resolve()
    source = work / "private-name fixture.bin"
    fixture = bytes([0, 1, 10, 13, 128, 255]) * 37
    cli = [sys.executable, "-B", str(ROOT / "scripts/cli-anything-inta.py"), "rclone-sha256",
           "--rclone-binary", str(binary), "--rclone-sha256", pin, "--file", str(source)]
    for sample in (fixture, b""):
        source.write_bytes(sample)
        before = (source.read_bytes(), source.stat().st_mtime_ns)
        digest = hashlib.sha256(sample).hexdigest()
        # Distinguish input selection, CLI startup and receipt encoding while
        # keeping the fixture bytes, local paths and raw command output private.
        try:
            assert harness.regular_snapshot(source) == sample, "snapshot bytes differ"
        except OSError as error:
            raise AssertionError(f"snapshot failed: errno={error.errno}, winerror={getattr(error, 'winerror', None)}") from None
        native = subprocess.run([str(binary), "--config=", "hashsum", "sha256", "-"],
                                input=sample, capture_output=True, shell=False, timeout=30, cwd=binary.parent,
                                env={k: os.environ[k] for k in ("SystemRoot", "WINDIR") if k in os.environ})
        assert native.returncode == 0, (
            f"native exit={native.returncode}, stderr bytes={len(native.stderr)}, "
            f"panic={b'panic:' in native.stderr}, config_error={b'config' in native.stderr.lower()}"
        )
        assert digest.encode() in native.stdout, f"rclone did not emit digest, stdout bytes={len(native.stdout)}"
        if not re.fullmatch(rb"([a-f0-9]{64})[ \t]+-?\r?\n", native.stdout):
            tail = native.stdout.split(digest.encode(), 1)[1]
            safe_codes = [b if b in b" \t-\r\n" else -1 for b in tail[:12]]
            raise AssertionError(f"unrecognized native formatting, bytes={len(native.stdout)}, safe suffix={safe_codes}")
        for expectation, code, status in ((digest, 0, "ready"), ("0"*64, 1, "mismatch")):
            run = subprocess.run(cli + ["--expect-sha256", expectation], cwd=work,
                                 capture_output=True, text=True, timeout=40)
            assert run.returncode == code and not run.stderr, (
                f"fixture size={len(sample)}, expected exit={code}, got={run.returncode}, "
                f"stderr bytes={len(run.stderr)}, parsed status={json.loads(run.stdout).get('status') if run.stdout else 'none'}"
            )
            assert source.name not in run.stdout
            assert json.loads(run.stdout) == dict(operation="rclone-sha256", status=status, scope="rclone-local-stdin",
                sha256=digest, bytes=len(sample), readOnly=True, matchesExpected=(code==0))
        assert (source.read_bytes(), source.stat().st_mtime_ns) == before and list(work.iterdir()) == [source]
print(json.dumps(dict(passed=True, operation="rclone-sha256", platform=sys.platform, binarySha256=pin)))
