#!/usr/bin/env python3
"""Identify signed Debian packages used when producing the ISO's external boot files."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import shutil
import subprocess
import tempfile
from concurrent.futures import ThreadPoolExecutor
from email.parser import Parser


def sha(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream,"sha256").hexdigest()


def audit(iso,gate,log,output):
    evidence=json.loads(gate.read_bytes())
    if sha(iso)!=evidence["sha256"]:
        raise ValueError("Boot evidence must come from the qualified ISO")
    text=log.read_text(errors="replace")
    stage=text.split(" lb binary_manifest ",1)
    if len(stage)!=2 or "P: Build completed successfully" not in stage[1]:
        raise ValueError("Complete build log required")
    versions={}
    for name,version in re.findall(r"^Unpacking ([a-z0-9+.:-]+) \(([^)]+)\)",stage[1],re.M):
        name=name.split(":")[0]
        if name in versions and versions[name]!=version:
            raise ValueError("Unexpected changing boot package version")
        versions[name]=version
    if not {"grub-common","grub-pc-bin","grub-efi-amd64-signed","shim-signed","shim-unsigned","isolinux"}<=set(versions):
        raise ValueError("Incomplete boot assembly log")
    live=subprocess.check_output(["dpkg-query","-W","-f=${Version}","live-build"],text=True)
    pvd=subprocess.check_output(["xorriso","-indev","stdio:"+str(iso),"-pvd_info"],stderr=subprocess.DEVNULL,text=True)
    if not re.search(r"^Preparer Id\s+: LIVE-BUILD "+re.escape(live.split(":")[-1])+r";",pvd,re.I|re.M):
        raise ValueError("The builder package differs from the ISO preparer")
    versions["live-build"]=live
    # The INTA optical shared object is built with the Debian Rust standard library
    # and system linker startup objects. Include their exact package sources and notices.
    optical_toolchain=("rustc","libstd-rust-dev","gcc","cpp-x86-64-linux-gnu","gcc-14","gcc-14-base","libgcc-14-dev","libc6-dev","libc6")
    manifest=subprocess.check_output(["dpkg-query","-W","-f=${Package}\t${Version}\n",*optical_toolchain],text=True)
    versions.update(dict(line.split("\t",1) for line in manifest.splitlines()))
    output.mkdir(parents=True,exist_ok=False)
    with tempfile.TemporaryDirectory(prefix="inta-boot-source-") as tmp:
        work=Path(tmp)
        # The signed apt index selects the original .deb; its package metadata, exact digest
        # and copyright are retained independently of packages installed in the live root.
        def get_package(pair):
            name,version=pair
            root=work/name
            root.mkdir()
            meta=Parser().parsestr(subprocess.check_output(["apt-cache","show",name+"="+version],text=True).split("\n\n")[0])
            if meta["Package"]!=name or meta["Version"]!=version or not re.fullmatch(r"[0-9a-f]{64}",meta["SHA256"] or ""):
                raise ValueError("Signed binary package identification failed: "+name)
            subprocess.run(["apt-get","download",name+"="+version],cwd=root,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL,check=True,timeout=120)
            deb,=root.glob("*.deb")
            if sha(deb)!=meta["SHA256"]:
                raise ValueError("Boot package hash differs from apt: "+name)
            pkg=root/"files"
            subprocess.run(["dpkg-deb","-x",deb,pkg],check=True)
            source=re.fullmatch(r"([a-z0-9][a-z0-9.+-]*)(?: \(([^)]+)\))?",meta.get("Source",name))
            if not source:
                raise ValueError("Invalid Debian source identity")
            return dict(package=name,version=version,architecture=meta["Architecture"],source=source[1],
                        sourceVersion=source[2] or version,binarySha256=meta["SHA256"]),pkg
        with ThreadPoolExecutor(max_workers=6) as pool:
            acquired=list(pool.map(get_package,sorted(versions.items())))
        rows=[p for p,_ in acquired]
        by_name={p["package"]:root for p,root in acquired}
        hashes={}
        for p,root in acquired:
            name=p["package"]
            doc=root/"usr/share/doc"/name
            if doc.is_symlink():
                dest=doc.readlink()
                if len(dest.parts)!=1 or dest.name not in by_name:
                    raise ValueError("Unresolved boot package notice: "+name)
                doc=by_name[dest.name]/"usr/share/doc"/dest.name
            notice=doc/"copyright"
            if not notice.is_file():
                raise ValueError("Missing boot package notice: "+name)
            target=output/"notices"/name/"copyright"
            target.parent.mkdir(parents=True)
            target.write_bytes(notice.read_bytes())
            p["noticeSha256"]=sha(target)
            for file in root.rglob("*"):
                if file.is_file() and not file.is_symlink() and not file.is_relative_to(root/"usr/share/doc"):
                    hashes.setdefault(sha(file),set()).add(name)
        boot=work/"disc"
        boot.mkdir()
        prefix=["xorriso","-abort_on","FAILURE","-osirrox","on","-indev","stdio:"+str(iso)]
        subprocess.run(prefix+["-extract","/EFI",str(boot/"EFI"),"-extract","/boot",str(boot/"boot")],
            stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL,check=True)
        efi=boot/"boot/grub/efi.img"
        fat=work/"fat"
        fat.mkdir()
        subprocess.run(["mcopy","-s","-i",efi,"::/EFI",str(fat)],check=True)
        matched={}
        for root in (boot,fat):
            for file in root.rglob("*"):
                if file.is_file() and file.suffix.lower() in {".mod",".efi",".pf2",".o"}:
                    h=sha(file)
                    providers=hashes.get(h)
                    if not providers:
                        raise ValueError("Unidentified boot module, font or EFI executable: "+file.name)
                    matched[("ISO" if root==boot else "EFI-FAT")+"/"+file.relative_to(root).as_posix()]=sorted(providers)
        if not all("ISO/EFI/boot/"+name in matched for name in ("bootx64.efi","grubx64.efi")) or not any(p.endswith(".mod") for p in matched):
            raise ValueError("Missing ELF GRUB module or signed EFI loader proof")
        result=dict(isoSha256=evidence["sha256"],imageSourceRevision=evidence["sourceRevision"],builder="live-build",opticalToolchain=list(optical_toolchain),
                    packageCount=len(rows),matchedExternalFileCount=len(matched),matchedExternalFiles=matched,packages=rows)
        (output/"boot-components.json").write_text(json.dumps(result,indent=2)+"\n")
        print(f"Verified {len(rows)} boot-stage Debian packages and {len(matched)} external module, font and EFI file matches")


if __name__=="__main__":
    args=argparse.ArgumentParser(description=__doc__)
    for name in ("iso","gate","build_log","new_output_directory"):
        args.add_argument(name,type=Path)
    a=args.parse_args()
    audit(a.iso.absolute(),a.gate,a.build_log,a.new_output_directory)
