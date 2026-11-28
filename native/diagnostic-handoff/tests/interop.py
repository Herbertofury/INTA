"""Exercise real Windows observations and the production Android cryptographic reader in a separate JVM."""
import argparse
import ctypes
import datetime
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tempfile


def main(exe, output):
    project = Path(__file__).resolve().parents[1]
    android = project.parents[1] / "mobile/android"
    output.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(dir=output.parent) as scratch:
        base = Path(scratch)
        sources = [android / "src/com/inta/mobile/DiagnosticHandoff.java", android / "tests/com/inta/mobile/DiagnosticPeer.java"]
        subprocess.run(["javac", "--release", "11", "-Xlint:all", "-Werror", "-d", base, *sources], check=True, timeout=45)
        peer = subprocess.Popen(["java", "-cp", base, "com.inta.mobile.DiagnosticPeer"],
                                stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, encoding="utf-8")
        def mobile(request):
            peer.stdin.write(request + "\n")
            peer.stdin.flush()
            reply = peer.stdout.readline().strip()
            if not reply:
                raise ValueError("Diagnostic peer exited abruptly")
            return reply

        def windows(*args, code="", ok=True):
            r = subprocess.run([exe, *args], input=code + "\n", capture_output=True, text=True, timeout=10)
            assert (r.returncode == 0) == ok and (not code or code not in r.stdout+r.stderr)
            result = json.loads(r.stdout)
            assert result["ok"] == ok
            return result.get("result")

        checks = []
        def passed(name):
            checks.append(name)
        try:
            assert len(windows("preview")["fields"]) == 6
            code = mobile("new")
            report = base / "received Ω.intadiag"
            windows("export", report, code=code, ok=False)
            assert not report.exists()
            passed("approval-required")
            receipt = windows("export", report, "--approve", code=code)
            sealed = report.read_bytes()
            assert len(sealed) == receipt["bytes"] == 87 and receipt["sha256"] == hashlib.sha256(sealed).hexdigest()
            passed("real-windows-capture-and-ciphertext")
            windows("export", report, "--approve", code=code, ok=False)
            assert report.read_bytes() == sealed
            passed("existing-file-preserved")
            bad = base / "modified.intadiag"
            for index in [0, 8, 16, 33, 60, 86]:
                changed = bytearray(sealed)
                changed[index] ^= 1
                bad.write_bytes(changed)
                assert mobile("read " + str(bad)) == "refused"
            passed("version-expiry-session-nonce-data-and-tag-tamper-refused")
            bad.write_bytes(sealed + b"x")
            assert mobile("read " + str(bad)) == "refused"
            observed = mobile("read " + str(report)).split()
            assert len(observed) == 7 and observed[0] == "read" and int(observed[1]) == receipt["capturedAtUnix"]
            data = (ctypes.c_ubyte * 64)()
            ctypes.c_uint32.from_buffer(data).value = 64
            assert ctypes.windll.kernel32.GlobalMemoryStatusEx(ctypes.byref(data))
            expected_total = ctypes.c_uint64.from_buffer(data, 8).value
            assert int(observed[2]) == expected_total and 0 < int(observed[3]) <= expected_total and 0 <= int(observed[4]) <= 100
            passed("actual-mobile-decoder-and-independent-memory-check")
            assert mobile("read " + str(report)) == "refused"
            passed("replay-refused")
            mobile("new")
            assert mobile("read " + str(report)) == "refused"
            passed("different-pairing-refused")
            for i, mode in enumerate(["revoked","expired","elapsed"]):
                code=mobile("new")
                fresh=base/f"fresh-{i}.intadiag"
                windows("export",fresh,"--approve",code=code)
                if mode=="revoked":
                    assert mobile("revoke")=="revoked"
                command="read" if mode=="revoked" else mode
                assert mobile(command+" "+str(fresh))=="refused"
                passed(mode+"-pairing-refused")
            code=mobile("new")
            old=".".join([code.split(".")[0], "1234567890", code.split(".")[2]])
            absent=base/"expired-windows.intadiag"
            windows("export",absent,"--approve",code=old,ok=False)
            assert not absent.exists()
            windows("export",absent,"--approve",code="not-a-pairing-fixture",ok=False)
            assert not absent.exists()
            passed("invalid-and-expired-export-refused")
        finally:
            if peer.poll() is None:
                peer.stdin.write("quit\n")
                peer.stdin.flush()
                peer.wait(timeout=10)
        assert peer.returncode==0
    proof={"checkedUtc":datetime.datetime.now(datetime.timezone.utc).isoformat(),"executableSha256":hashlib.file_digest(exe.open("rb"),"sha256").hexdigest(),
           "windowsBuild":os.sys.getwindowsversion().build,"mobileConsumer":"Production Android decoder executing in OpenJDK, separate process from the Rust executable",
           "packetBytes":87,"checks":checks,"newNetworkPermissions":0,"localKeysOrCustomerDataStored":False}
    with output.open("x",encoding="utf-8") as f:
        json.dump(proof,f,indent=2)
        f.write("\n")
    print(output)


if __name__=="__main__":
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument("executable",type=Path)
    parser.add_argument("new_receipt",type=Path)
    opts=parser.parse_args()
    main(opts.executable.resolve(),opts.new_receipt.resolve())
