#!/usr/bin/env python3
"""Bind the two native firmware proofs to the same exact, read-only ISO."""
from concurrent.futures import ThreadPoolExecutor
import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess
from rescue_boot_log import passed


def sha(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream,"sha256").hexdigest()


def verify(image, expected, evidence):
    if not re.fullmatch(r"[0-9a-f]{64}",expected) or not image.is_file() or image.is_symlink() or sha(image)!=expected:
        raise ValueError("Expected the original checksum of a regular recovery ISO")
    evidence.mkdir(parents=True,exist_ok=False)
    def firmware(mode):
        subprocess.run(["bash",str(Path(__file__).with_name("smoke-iso")),image,mode,evidence],
                       check=True,stdin=subprocess.DEVNULL,timeout=420)
        path=evidence/(mode+"-serial.log")
        serial=path.read_bytes()
        lines=serial.splitlines()
        if (not passed(serial) or lines.count(b"INTA_RECOVERY_ONLY_RUNTIME_OK")!=1
                or lines.count(b"INTA_OPTICS_NATIVE_OK")!=1):
            raise ValueError("Required independent guest markers missing for "+mode)
        return {"firmware":mode,"serialSha256":sha(path),"passed":True,"recoveryRuntime":True,"nativeOptics":True}
    with ThreadPoolExecutor(max_workers=2) as pool:
        checked=list(pool.map(firmware,["bios","uefi"]))
    if sha(image)!=expected:
        raise ValueError("Booted media changed during verification")
    receipt={"schemaVersion":1,"sha256":expected,"readOnlyCdrom":True,"guestNetworkDevices":0,
             "writableGuestDisks":0,"firmware":checked}
    (evidence/"firmware.json").write_text(json.dumps(receipt,indent=2)+"\n")
    print(json.dumps(receipt))


if __name__=="__main__":
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument("iso",type=Path)
    p.add_argument("sha256")
    p.add_argument("new_evidence_directory",type=Path)
    args=p.parse_args()
    verify(args.iso.absolute(),args.sha256,args.new_evidence_directory.absolute())
