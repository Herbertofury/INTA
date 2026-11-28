"""Exercise the real Cloud Hypervisor terminal with ConPTY. Requires pywinpty on Windows."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import queue
import re
import tempfile
import threading
import time

from winpty import PtyProcess


def validate(cli, toolbox, output):
    with tempfile.TemporaryDirectory(dir=output.parent) as temp:
        work = Path(temp)
        disk = work / "fixture.img"
        disk.write_bytes(bytes(1024 * 1024))
        original = hashlib.sha256(disk.read_bytes()).hexdigest()
        env = dict(os.environ, INTA_DATA_DIR=str(work / "state"))
        proc = PtyProcess.spawn(
            [
                str(cli),
                "vm",
                "console",
                "--approve",
                "--base",
                str(disk),
                "--root",
                str(toolbox),
            ],
            env=env,
            dimensions=(50, 150),
        )
        chunks = queue.Queue()

        def reader():
            try:
                while True:
                    chunk = proc.read(8192)
                    if not chunk:
                        break
                    chunks.put(chunk)
            except EOFError:
                pass
            finally:
                chunks.put(None)

        threading.Thread(target=reader, daemon=True).start()
        transcript = ""
        sent = False
        deadline = time.monotonic() + 180
        try:
            while time.monotonic() < deadline:
                try:
                    chunk = chunks.get(timeout=1)
                except queue.Empty:
                    continue
                if chunk is None:
                    break
                transcript += chunk
                if not sent and "inta-rescue-lab# " in transcript:
                    # Write only to the disposable guest copy of our synthetic fixture.
                    proc.write(
                        "printf '\\052' | dd of=/dev/vdc bs=1 count=1 conv=notrunc status=none; "
                        "printf '\\nINTA_ROOT_%s\\n' \"$(id -u)\"; "
                        "printf 'INTA_BYTE_%d\\n' \"$(od -An -tu1 -N1 /dev/vdc)\"; exit\r"
                    )
                    sent = True
        finally:
            if proc.isalive():
                proc.terminate(force=True)
            output.write_text(transcript, encoding="utf8")
        result = {
            "guestRoot": bool(re.search(r"INTA_ROOT_0\b", transcript)),
            "privateGuestDiskWritable": bool(re.search(r"INTA_BYTE_42\b", transcript)),
            "sourcePreserved": hashlib.sha256(disk.read_bytes()).hexdigest()
            == original,
            "cleanPoweroff": "powered-off" in transcript,
        }
        print(json.dumps(result))
        if not all(result.values()):
            raise SystemExit(1)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("cli", "toolbox", "output"):
        parser.add_argument("--" + name, type=Path, required=True)
    arguments = parser.parse_args()
    validate(
        arguments.cli.resolve(), arguments.toolbox.resolve(), arguments.output.resolve()
    )
