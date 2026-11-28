"""Round-trip a generated file through the actual INTA Linux tool console on Windows."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import queue
import shlex
import subprocess
import threading
import time

from winpty import PtyProcess


def exercise(cli, destination):
    destination.mkdir(parents=True, exist_ok=False)
    source = destination / "source.bin"
    source.write_bytes(bytes(range(256)) * 1024)
    expected = hashlib.sha256(source.read_bytes()).hexdigest()
    wsl = Path(os.environ["SystemRoot"]) / "System32/wsl.exe"
    path = subprocess.check_output(
        [
            wsl,
            "-d",
            "INTA-Rescue-Builder",
            "-u",
            "inta-lab",
            "--exec",
            "/usr/bin/wslpath",
            "-u",
            str(destination),
        ],
        text=True,
    ).strip()
    args = " ".join(
        shlex.quote(path + "/" + f)
        for f in ("source.bin", "recovered.bin", "rescue.map")
    )
    env = dict(os.environ, INTA_DATA_DIR=str(destination / "state"))
    preview = json.loads(
        subprocess.check_output([cli, "tool", "actions", "ddrescue"], env=env)
    )
    assert preview["launch"]["kind"] == "linux" and preview["launch"]["available"]
    proc = PtyProcess.spawn(
        [str(cli), "linux-session", "launch", "ddrescue", "--approve"],
        env=env,
        dimensions=(46, 140),
    )
    chunks = queue.Queue()

    def reader():
        try:
            while data := proc.read(8192):
                chunks.put(data)
        except EOFError:
            pass
        finally:
            chunks.put(None)

    threading.Thread(target=reader, daemon=True).start()
    transcript = ""
    responses = iter(["help", args, "exit"])
    prompts = 0
    deadline = time.monotonic() + 60
    try:
        while time.monotonic() < deadline:
            try:
                data = chunks.get(timeout=0.5)
            except queue.Empty:
                continue
            if data is None:
                break
            transcript += data
            if transcript.count("GNU ddrescue> ") > prompts:
                command = next(responses, None)
                if command is not None:
                    proc.write(command + "\r")
                prompts += 1
    finally:
        if proc.isalive():
            proc.terminate(force=True)
        (destination / "transcript.txt").write_text(transcript, encoding="utf8")
    assert "Usage: /usr/bin/ddrescue" in transcript and '"complete"' in transcript
    assert (
        expected
        == hashlib.sha256((destination / "recovered.bin").read_bytes()).hexdigest()
        == hashlib.sha256(source.read_bytes()).hexdigest()
    )
    assert "+" in (destination / "rescue.map").read_text()
    print(
        json.dumps(
            {
                "tool": "ddrescue",
                "console": True,
                "bytesRecovered": source.stat().st_size,
                "sha256": expected,
                "sourcePreserved": True,
            }
        )
    )


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("cli", type=Path)
    parser.add_argument("destination", type=Path)
    args = parser.parse_args()
    exercise(args.cli.resolve(), args.destination.resolve())
