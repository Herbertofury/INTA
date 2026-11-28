"""Independent hosted qualification of the Windows allocation opt-in."""
import ctypes
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

agent = Path(sys.argv[1]).resolve(strict=True)

def invoke(directory, allocation=True, limit=100000):
    args = [str(agent), "--storage-allocation-preview" if allocation else "--storage-preview",
            str(directory), "7", str(limit)]
    return subprocess.run(args, capture_output=True, text=True, timeout=30)

with tempfile.TemporaryDirectory(prefix="inta-allocation-fixture-") as work:
    root = Path(work).resolve()
    selected = root / "selected"
    selected.mkdir()
    if sys.platform != "win32":
        native = invoke(selected)
        assert native.returncode == 3 and native.stdout == ""
        assert "allocation" not in json.loads(invoke(selected, False).stdout)
        print("Non-Windows explicitly refuses allocation; normal preview remains available.")
        raise SystemExit(0)

    from ctypes import wintypes
    import msvcrt
    kernel = ctypes.WinDLL("kernel32", use_last_error=True)
    ioctl = kernel.DeviceIoControl
    ioctl.argtypes = [wintypes.HANDLE, wintypes.DWORD, wintypes.LPVOID, wintypes.DWORD,
                      wintypes.LPVOID, wintypes.DWORD, ctypes.POINTER(wintypes.DWORD), wintypes.LPVOID]
    ioctl.restype = wintypes.BOOL
    compressed = kernel.GetCompressedFileSizeW
    compressed.argtypes = [wintypes.LPCWSTR, ctypes.POINTER(wintypes.DWORD)]
    compressed.restype = wintypes.DWORD
    def observed_size(file):
        high = wintypes.DWORD()
        ctypes.set_last_error(0)
        low = compressed(str(file), ctypes.byref(high))
        assert low != 0xFFFFFFFF or ctypes.get_last_error() == 0, "Fixture size observation failed"
        return (high.value << 32) | low

    ordinary = selected / "regular private name.bin"
    sparse = selected / "sparse private name.bin"
    outside = root / "outside private alias.bin"
    ordinary.write_bytes(os.urandom(8192))
    os.link(ordinary, outside)
    with sparse.open("w+b") as fh:
        returned = wintypes.DWORD()
        assert ioctl(msvcrt.get_osfhandle(fh.fileno()), 0x000900C4, None, 0, None, 0,
                     ctypes.byref(returned), None), "Local fixture volume must support sparse files"
        fh.seek(4 * 1024 * 1024)
        fh.write(b"fixture" * 512)
        fh.truncate(8 * 1024 * 1024)

    def state(paths):
        return {p: (hashlib.sha256(p.read_bytes()).hexdigest(), p.stat().st_mtime_ns) for p in paths}

    expected = observed_size(ordinary) + observed_size(sparse)
    assert observed_size(sparse) < sparse.stat().st_size, "Test fixture must really be sparse"
    files = (ordinary, sparse, outside)
    before = state(files)

    default = invoke(selected, False)
    assert default.returncode == 0 and "allocation" not in json.loads(default.stdout)
    report = invoke(selected)
    assert report.returncode == 0 and all(p.name not in report.stdout + report.stderr for p in files)
    plain = json.loads(report.stdout)
    detail = plain["allocation"]
    assert plain["logicalBytes"] == ordinary.stat().st_size + sparse.stat().st_size
    assert detail["observedAllocationBytes"] == expected and detail["identifiedFiles"] == 2 and not detail["partial"]
    assert detail["repeatedPathnames"] == 0 and detail["filesWithUnseenAliases"] == 1
    assert detail["allocationWithUnseenAliases"] == observed_size(ordinary) and detail["reclaimableSpaceInferred"] is False
    assert state(files) == before

    second = selected / "second private alias.bin"
    os.link(ordinary, second)
    complete = json.loads(invoke(selected).stdout)
    full = complete["allocation"]
    assert complete["logicalBytes"] == plain["logicalBytes"] + ordinary.stat().st_size
    assert full["identifiedFiles"] == 2 and full["observedAllocationBytes"] == expected
    assert full["repeatedPathnames"] == 1 and full["filesWithUnseenAliases"] == 1
    assert json.loads(invoke(selected, limit=1).stdout)["allocation"]["partial"] is True
    assert state(files) == before and os.path.samefile(second, outside)
    print("Windows: actual sparse allocation, repeated aliases counted once, unseen link retained, default unchanged.")
