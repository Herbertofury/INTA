#!/usr/bin/env python3
"""Resolve the exact source archives of an image inventory against signed Debian indexes."""
import argparse
from datetime import datetime, timedelta, timezone
from email.utils import parsedate_to_datetime
import hashlib
import json
import lzma
from pathlib import Path
import re
import subprocess
import tempfile
import urllib.parse
import urllib.request

REPOS=(("https://deb.debian.org/debian","trixie"),("https://deb.debian.org/debian","trixie-updates"),
       ("https://security.debian.org/debian-security","trixie-security"))
KEYRING="/usr/share/keyrings/debian-archive-keyring.gpg"


def download(url,path,maximum):
    with urllib.request.urlopen(url,timeout=45) as incoming,path.open("xb") as out:
        if urllib.parse.urlparse(incoming.url).scheme!="https":
            raise ValueError("Debian metadata needs HTTPS")
        count=0
        while block:=incoming.read(262144):
            count+=len(block)
            if count>maximum:
                raise ValueError("Debian metadata exceeded its bounds")
            out.write(block)


def digest(path):
    with path.open("rb") as src:
        return hashlib.file_digest(src,"sha256").hexdigest()


def fields(lines):
    result={}
    key=None
    for line in lines:
        if line.startswith((" ","\t")):
            if key is None:
                raise ValueError("Malformed Debian metadata")
            result[key]+="\n"+line.strip()
        else:
            key,sep,value=line.partition(":")
            if not sep or key in result:
                raise ValueError("Invalid Debian metadata fields")
            result[key]=value.strip()
    return result


def match_repo(root,suite,temp,wanted,found):
    url=f"{root}/dists/{suite}/"
    inrelease=temp/(suite+".InRelease")
    release=temp/(suite+".Release")
    download(url+"InRelease",inrelease,4*1024**2)
    subprocess.run(["gpgv","--keyring",KEYRING,"--output",str(release),str(inrelease)],check=True,
                   stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL,timeout=20)
    meta=fields(release.read_text().splitlines())
    if meta["Codename"]!=suite:
        raise ValueError("Signed suite differs")
    now=datetime.now(timezone.utc)
    if parsedate_to_datetime(meta["Date"])>now+timedelta(hours=1) or ("Valid-Until" in meta and parsedate_to_datetime(meta["Valid-Until"])<now):
        raise ValueError("Debian metadata is outside its validity interval")
    entries=[r.split() for r in meta["SHA256"].splitlines() if r.strip()]
    expected,size,path=next(row for row in entries if row[2]=="main/source/Sources.xz")
    compressed=temp/(suite+".xz")
    download(url+path,compressed,min(int(size),64*1024**2))
    if compressed.stat().st_size!=int(size) or digest(compressed)!=expected:
        raise ValueError("Source package index failed Debian's signed digest")
    with lzma.open(compressed,"rt",encoding="utf-8") as stream:
        record=[]
        for line in stream:
            if line.strip():
                record.append(line.rstrip("\n"))
                continue
            data=fields(record)
            record=[]
            identity=(data["Package"],data["Version"])
            if identity not in wanted or identity in found:
                continue
            location=data["Directory"]
            if not re.fullmatch(r"pool/(?:updates/)?main/[a-z0-9]+/[a-z0-9+.-]+",location):
                raise ValueError("Unexpected Debian source directory")
            archived = {parts[2]:parts[0] for row in data.get("Checksums-Sha1","").splitlines() if row
                        for parts in [row.split()] if len(parts)==3 and re.fullmatch(r"[0-9a-f]{40}",parts[0])}
            files=[]
            for row in data["Checksums-Sha256"].splitlines():
                if not row:
                    continue
                sha,size,name=row.split()
                if not re.fullmatch(r"[0-9a-f]{64}",sha) or not re.fullmatch(r"[A-Za-z0-9+._~:%-]+",name):
                    raise ValueError("Invalid source archive metadata")
                item=dict(name=name,bytes=int(size),sha256=sha,url=root+"/"+location+"/"+urllib.parse.quote(name))
                if name in archived:
                    item["snapshotUrl"]="https://snapshot.debian.org/file/"+archived[name]
                files.append(item)
            if not any(f["name"].endswith(".dsc") for f in files):
                raise ValueError("Source control file missing")
            found[identity]=dict(source=identity[0],version=identity[1],suite=suite,files=files,
                                versionArchive="https://snapshot.debian.org/package/"+identity[0]+"/"+urllib.parse.quote(identity[1],safe="")+"/")
    return {"suite":suite,"inRelease":url+"InRelease","inReleaseSha256":digest(inrelease),"indexSha256":expected}


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument("package_inventory",type=Path)
    parser.add_argument("new_output_file",type=Path)
    parser.add_argument("--boot-components",type=Path,required=True)
    args=parser.parse_args()
    packages=json.loads(args.package_inventory.read_text())
    wanted={(r["source"],r["sourceVersion"]) for r in packages if r["status"].endswith(" installed")}
    boot=json.loads(args.boot_components.read_bytes())
    wanted.update((p["source"],p["sourceVersion"]) for p in boot["packages"])
    found={}
    with tempfile.TemporaryDirectory() as temp:
        metadata=[match_repo(root,suite,Path(temp),wanted,found) for root,suite in REPOS]
    missing=sorted(wanted-set(found))
    if missing:
        raise ValueError(f"Source archives for {len(missing)} versions unresolved: {missing[:5]}")
    output=dict(schemaVersion=1,verifiedAt=datetime.now(timezone.utc).isoformat(),inventorySha256=digest(args.package_inventory),
                indexes=metadata,bootAndBuildComponents=boot,sourcePackages=[found[key] for key in sorted(found)])
    with args.new_output_file.open("x") as target:
        json.dump(output,target,indent=2)
        target.write("\n")
    print(f"Resolved {len(found)} exact source-package versions against Debian-signed release metadata")


if __name__=="__main__":
    main()
