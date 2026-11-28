"""Validate the ordered smoke markers in a bounded INTA guest serial log."""
import sys
from pathlib import Path

MAX_BYTES = 16 * 1024 * 1024


def passed(serial):
    markers = [line for line in serial.splitlines() if line.startswith(b"INTA_SMOKE_")]
    return markers == [b"INTA_SMOKE_START", b"INTA_SMOKE_PASS"]


if __name__ == "__main__":
    with Path(sys.argv[1]).open("rb") as stream:
        serial = stream.read(MAX_BYTES + 1)
    if len(serial) > MAX_BYTES or not passed(serial):
        sys.exit("Expected one completed INTA smoke test without a failure or another attempt")
