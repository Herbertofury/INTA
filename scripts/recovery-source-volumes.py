#!/usr/bin/env python3
"""Acquire and verify every Debian source, then build complete, independent source tar volumes."""
import argparse
from concurrent.futures import ThreadPoolExecutor, as_completed
import hashlib
import json
import os
from pathlib import Path
import re
import tarfile
import tempfile
import time
import urllib.parse
import urllib.request

VOLUME_LIMIT=1_150_000_000
MAX_FILE=1_100_000_000
CHUNK=1024*1024


def digest(path):
    with path.open("rb") as f:
        return hashlib.file_digest(f,"sha256").hexdigest()


def plans(index):
    groups={}
    for row in index["sourcePackages"]:
        pkg,version=row["source"],row["version"]
        if not re.fullmatch(r"[a-z0-9][a-z0-9.+-]+",pkg) or not re.fullmatch(r"[A-Za-z0-9.+:~%-]+",version):
            raise ValueError("Unexpected Debian source identity")
        key=pkg+"/"+urllib.parse.quote(version,safe="+.~")
        if key in groups:
            raise ValueError("Repeated Debian source identity")
        items={}
        for item in row["files"]:
            name=item["name"]
            url=urllib.parse.urlparse(item["url"])
            if (name in items or not re.fullmatch(r"[A-Za-z0-9+._~:%-]+",name) or name.startswith(".")
                    or not re.fullmatch(r"[0-9a-f]{64}",item["sha256"]) or not 0<item["bytes"]<=MAX_FILE
                    or url.scheme!="https" or url.netloc not in {"deb.debian.org","security.debian.org"}
                    or urllib.parse.unquote(url.path.rsplit("/",1)[-1])!=name):
                raise ValueError("Invalid source index item")
            items[name]=item
        if sum(name.endswith(".dsc") for name in items)!=1 or len(items)<2:
            raise ValueError("Expected the Debian source control file and its original archives")
        groups[key]=items
    if not groups:
        raise ValueError("Empty source index")
    return groups


def cached(cache,sha):
    return cache/sha[:2]/sha


def acquire(item,cache):
    expected=item["sha256"]
    path=cached(cache,expected)
    path.parent.mkdir(parents=True,exist_ok=True)
    if path.exists():
        if path.is_file() and not path.is_symlink() and path.stat().st_size==item["bytes"] and digest(path)==expected:
            return
        raise ValueError("A cached source file failed verification: "+item["name"])
    urls=[item["url"]]
    if urls[0].startswith("https://security.debian.org/debian-security/"):
        urls.append(urls[0].replace("security.debian.org","deb.debian.org",1))
    if re.fullmatch(r"https://snapshot\.debian\.org/file/[0-9a-f]{40}",item.get("snapshotUrl","")):
        urls.append(item["snapshotUrl"])
    last=None
    for attempt in range(4):
        try:
            source_url=urls[attempt%len(urls)]
            request=urllib.request.Request(source_url,headers={"Accept-Encoding":"identity","User-Agent":"INTA-Recovery-source-archive/1"})
            with urllib.request.urlopen(request,timeout=75) as response,tempfile.NamedTemporaryFile(dir=path.parent) as tmp:
                if not response.geturl().startswith("https://"):
                    raise ValueError("Unencrypted Debian source transport")
                hasher=hashlib.sha256()
                size=0
                while part:=response.read(CHUNK):
                    size+=len(part)
                    if size>item["bytes"]:
                        raise ValueError("Oversized Debian source")
                    hasher.update(part)
                    tmp.write(part)
                if size!=item["bytes"] or hasher.hexdigest()!=expected:
                    raise ValueError("Unexpected source length or digest")
                tmp.flush()
                os.fsync(tmp.fileno())
                try:
                    os.link(tmp.name,path)
                except FileExistsError:
                    if digest(path)!=expected:
                        raise ValueError("Conflicting source cache entry")
            return
        except (OSError,ValueError) as error:
            last=error
            time.sleep(attempt+1)
    raise RuntimeError("Failed verified acquisition for "+item["name"]) from last


def verify_dsc(group,cache):
    names={name for name in group if not name.endswith(".dsc")}
    control=next(f for k,f in group.items() if k.endswith(".dsc"))
    content=cached(cache,control["sha256"]).read_text(encoding="utf-8")
    section=re.search(r"(?m)^Checksums-Sha256:\s*\n((?: [^\n]+\n)+)",content)
    if section:
        records={parts[2]:(parts[0],int(parts[1])) for line in section[1].splitlines() for parts in [line.split()]}
        if {name:(group[name]["sha256"],group[name]["bytes"]) for name in names}!=records:
            raise ValueError("Source control SHA-256 section disagrees with the signed source index: "+control["name"])
    else:
        files=re.search(r"(?m)^Files:\s*\n((?: [^\n]+\n)+)",content)
        if not files or {line.split()[2] for line in files[1].splitlines()}!=names:
            raise ValueError("Source control file list differs from the signed source index")


def check_members(path,expected):
    pending=dict(expected)
    with tarfile.open(path,mode="r|") as archive:
        for member in archive:
            item=pending.pop(member.name,None)
            if not item or not member.isfile() or member.size!=item["bytes"]:
                raise ValueError("Unexpected source volume member")
            with archive.extractfile(member) as stream:
                if hashlib.file_digest(stream,"sha256").hexdigest()!=item["sha256"]:
                    raise ValueError("Archived source digest mismatch: "+member.name)
    if pending:
        raise ValueError("Source volume is incomplete")


def verify_volumes(index,out):
    groups=plans(json.loads(index.read_bytes()))
    receipt=json.loads((out/"debian-source-volumes.json").read_bytes())
    volume_count=len(receipt["volumes"])
    if not 1<=volume_count<=100 or receipt["sourceIndexSha256"]!=digest(index):
        raise ValueError("Volume manifest is for another source index")
    seen=set()
    for number,volume in enumerate(receipt["volumes"],1):
        name=f"Debian-recovery-sources-{number:02d}-of-{volume_count:02d}.tar"
        path=out/name
        if (volume["name"]!=name or path.is_symlink() or path.stat().st_size!=volume["bytes"] or
                not 0<volume["bytes"]<VOLUME_LIMIT or digest(path)!=volume["sha256"]):
            raise ValueError("Missing or modified source volume: "+name)
        expected={}
        for key in volume["sourceGroups"]:
            if key not in groups or key in seen:
                raise ValueError("Unexpected or duplicated source group")
            seen.add(key)
            expected.update({"sources/"+key+"/"+n:f for n,f in groups[key].items()})
        check_members(path,expected)
    if (seen!=set(groups) or receipt["sourceVersions"]!=len(groups) or
            receipt["sourceFiles"]!=sum(map(len,groups.values())) or
            receipt["sourceBytes"]!=sum(f["bytes"] for g in groups.values() for f in g.values())):
        raise ValueError("Incomplete source volume set")
    return receipt


def write_volumes(groups,cache,out,index):
    out.mkdir(parents=True,exist_ok=False)
    batches=[]
    # Group complete source packages together, including their .dsc and each upstream tar/diff.
    # Reserve a conservative PAX-header allowance for long Debian paths.
    for key,items in sorted(groups.items()):
        cost=sum((item["bytes"]+511)//512*512+2048 for item in items.values())
        if cost>VOLUME_LIMIT-10240:
            raise ValueError("One source package exceeds the independent-volume size limit")
        if not batches or batches[-1][0]+cost>VOLUME_LIMIT-10240:
            batches.append([0,[]])
        batches[-1][0]+=cost
        batches[-1][1].append(key)
    volumes=[]
    for number,(_,keys) in enumerate(batches,1):
        name=f"Debian-recovery-sources-{number:02d}-of-{len(batches):02d}.tar"
        path=out/name
        expected={}
        with tarfile.open(path,"x",format=tarfile.PAX_FORMAT) as archive:
            for key in keys:
                for filename,item in sorted(groups[key].items()):
                    target="sources/"+key+"/"+filename
                    expected[target]=item
                    info=tarfile.TarInfo(target)
                    info.size=item["bytes"]
                    info.mode=0o644
                    with cached(cache,item["sha256"]).open("rb") as stream:
                        archive.addfile(info,stream)
        if path.stat().st_size>=VOLUME_LIMIT:
            raise ValueError("Volume exceeded its upload limit")
        check_members(path,expected)
        volumes.append(dict(name=name,bytes=path.stat().st_size,sha256=digest(path),sourceGroups=keys))
    receipt={"schemaVersion":1,"sourceIndexSha256":digest(index),"sourceVersions":len(groups),
             "sourceFiles":sum(map(len,groups.values())),"sourceBytes":sum(i["bytes"] for g in groups.values() for i in g.values()),
             "memberPath":"sources/<package>/<URL-encoded-version>/<original-filename>","independentArchives":True,"volumes":volumes}
    (out/"debian-source-volumes.json").write_text(json.dumps(receipt,indent=2)+"\n",encoding="utf-8")
    print(json.dumps({k:v for k,v in receipt.items() if k!="volumes"}),len(volumes),"verified volumes",flush=True)


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source_index",type=Path)
    parser.add_argument("cache",type=Path)
    parser.add_argument("new_output_directory",type=Path)
    args=parser.parse_args()
    groups=plans(json.loads(args.source_index.read_bytes()))
    files={item["sha256"]:item for g in groups.values() for item in g.values()}
    with ThreadPoolExecutor(max_workers=10) as pool:
        tasks=[pool.submit(acquire,item,args.cache) for item in sorted(files.values(),key=lambda f:-f["bytes"])]
        for i,future in enumerate(as_completed(tasks),1):
            future.result()
            if i%100==0:
                print(f"Verified {i} of {len(tasks)} original source files",flush=True)
    for group in groups.values():
        verify_dsc(group,args.cache)
    write_volumes(groups,args.cache,args.new_output_directory,args.source_index)


if __name__=="__main__":
    import sys
    if len(sys.argv)==4 and sys.argv[1]=="--verify":
        print("Verified original source files:",verify_volumes(Path(sys.argv[2]),Path(sys.argv[3]))["sourceFiles"])
    else:
        main()
