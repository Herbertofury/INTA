#!/usr/bin/env python3
"""Build the reviewed Media Creator with an exact recovery-channel record."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import runpy
import shutil
import subprocess
import tempfile


def build(repo,iso,gate,tag,output):
    record=json.loads(gate.read_bytes())
    if not re.fullmatch(r"rescue-recovery-[0-9]{8}\.[1-9][0-9]*",tag) or record["edition"]!="recovery-only-v1":
        raise ValueError("Unexpected release tag or edition")
    with iso.open("rb") as source:
        if hashlib.file_digest(source,"sha256").hexdigest()!=record["sha256"] or iso.stat().st_size!=record["bytes"]:
            raise ValueError("Media Creator requires the qualified image")
    revision=subprocess.check_output(["git","-C",str(repo),"rev-parse","HEAD"],text=True).strip()
    if subprocess.check_output(["git","-C",str(repo),"status","--porcelain","--untracked-files=no"]).strip():
        raise ValueError("Choose an unmodified, reviewed Media Creator checkout")
    with tempfile.TemporaryDirectory() as working:
        prepared=Path(working)
        for name in ("media","native/workbench/media"):
            shutil.copytree(repo/name,prepared/name,ignore=shutil.ignore_patterns("__pycache__","*.pyc"))
        for name in ("scripts/release_artwork.py","docs/assets/inta-mark.svg","assets/brand-icons/70ad9f0981b05341142b.png"):
            dest=prepared/name
            dest.parent.mkdir(parents=True,exist_ok=True)
            shutil.copyfile(repo/name,dest)
        (prepared/"data").mkdir()
        for name in ("brand-icons.json","rescue-download-channel.json","rescue-recovery-download-channel.json"):
            shutil.copyfile(repo/"data"/name,prepared/"data"/name)
        channel=prepared/"data/rescue-recovery-download-channel.json"
        reviewed=json.loads(channel.read_bytes())
        updated={**reviewed,"schemaVersion":1,"tag":tag,"qualification":"recovery-only","edition":"recovery-only-v1",
                 "filename":iso.name,"volumeId":"INTA_RECOVERY","bytes":record["bytes"],"sha256":record["sha256"],"sourceRevision":record["sourceRevision"]}
        if updated!=reviewed:
            channel.write_text(json.dumps(updated,indent=2)+"\n",encoding="utf-8",newline="\n")
        built=prepared/"INTA-Media-Creator.pyz"
        result=runpy.run_path(str(repo/"scripts/package-media.py"))["package"](built,prepared)
        with built.open("rb") as inp,output.open("xb") as out:
            shutil.copyfileobj(inp,out)
    print(json.dumps({"mediaSourceRevision":revision,"sha256":result["sha256"],"tag":tag}))


if __name__=="__main__":
    p=argparse.ArgumentParser(description=__doc__)
    for name in ["companion_checkout","iso","qualification","tag","new_file"]:
        p.add_argument(name,type=str if name=="tag" else Path)
    a=p.parse_args()
    build(a.companion_checkout,a.iso,a.qualification,a.tag,a.new_file)
