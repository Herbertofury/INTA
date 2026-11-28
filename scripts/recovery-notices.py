#!/usr/bin/env python3
"""Export the original copyright notices directly from the qualified ISO."""
import argparse
import gzip
import hashlib
import io
import json
from pathlib import Path
import re
import subprocess
import tarfile
import tempfile


def digest(path):
    with path.open("rb") as f:
        return hashlib.file_digest(f,"sha256").hexdigest()


def export(iso, audit_dir, boot_dir, output):
    receipt=json.loads((audit_dir/"image-qualification.json").read_bytes())
    metadata=audit_dir/"debian-package-inventory.json"
    if receipt["edition"]!="recovery-only-v1" or receipt["sha256"]!=digest(iso) or receipt["packageInventorySha256"]!=digest(metadata):
        raise ValueError("The selected ISO and its qualified package inventory must match")
    installed={p["package"] for p in json.loads(metadata.read_bytes()) if p["status"].endswith(" installed")}
    boot=json.loads((boot_dir/"boot-components.json").read_bytes())
    if boot["isoSha256"]!=receipt["sha256"]:
        raise ValueError("Boot package notices are for another ISO")
    with tempfile.TemporaryDirectory() as wd:
        work=Path(wd)
        fs=work/"root.squashfs"
        subprocess.run(["xorriso","-osirrox","on","-indev","stdio:"+str(iso),"-extract","/live/filesystem.squashfs",str(fs)],check=True,timeout=180,
                       stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
        if digest(fs)!=receipt["squashfsSha256"]:
            raise ValueError("Unexpected filesystem")
        extracted=work/"files"
        subprocess.run(["unsquashfs","-processors","2","-no-progress","-no-xattrs","-d",str(extracted),str(fs),
                        "usr/share/doc","usr/share/common-licenses"],check=True,timeout=180,stdout=subprocess.DEVNULL)
        common=extracted/"usr/share/common-licenses"
        docs=extracted/"usr/share/doc"
        entries={}
        for name in sorted(installed):
            if not re.fullmatch(r"[a-z0-9][a-z0-9.+-]+",name):
                raise ValueError("Invalid package name")
            path=(docs/name/"copyright").resolve(strict=True)
            if not path.is_relative_to(docs) or not path.is_file():
                raise ValueError("Missing confined copyright file for "+name)
            entries["packages/"+name+"/copyright"]=path
        for item in sorted(common.iterdir()):
            resolved=item.resolve(strict=True)
            if not resolved.is_relative_to(common) or not resolved.is_file():
                raise ValueError("Invalid common license")
            entries["common-licenses/"+item.name]=resolved
        for entry in boot["packages"]:
            notice=boot_dir/"notices"/entry["package"]/"copyright"
            if digest(notice)!=entry["noticeSha256"]:
                raise ValueError("Boot-stage copyright digest differs")
            entries["boot-and-build/"+entry["package"]+"/copyright"]=notice
        with output.open("xb") as raw,gzip.GzipFile(fileobj=raw,filename="",mode="wb",mtime=0) as compressed,tarfile.open(fileobj=compressed,mode="w|") as archive:
            for name,path in entries.items():
                data=path.read_bytes()
                record=tarfile.TarInfo(name)
                record.size=len(data)
                record.mode=0o644
                archive.addfile(record,io.BytesIO(data))
        print(f"Archived {len(installed)} installed and {len(boot['packages'])} assembly/compiler package notices")


if __name__=="__main__":
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument("iso",type=Path)
    p.add_argument("qualification_directory",type=Path)
    p.add_argument("boot_components_directory",type=Path)
    p.add_argument("new_archive",type=Path)
    args=p.parse_args()
    export(args.iso.absolute(),args.qualification_directory,args.boot_components_directory,args.new_archive)
