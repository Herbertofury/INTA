#!/usr/bin/env python3
"""Qualify an exact, trusted INTA Recovery build without running image code."""
import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import re
import shlex
import subprocess
import tempfile

MIB = 1024**2
DENIED_PACKAGES = re.compile(r"^(?:clamav(?:[-0-9].*)?|libclamav.*|libclamunrar.*|libfreshclam.*|"
                             r"clamdscan|clamtk(?:-.*)?|freshclam|yara(?:-.*)?|libyara.*|python3-(?:clamd|yara)|"
                             r"nodejs(?:-.*)?|npm|claude-code|codex)$", re.I)
COMMANDS = {"clamscan","clamd","clamdscan","clamconf","clamdtop","clamav-config","clambc","clamonacc",
            "freshclam","sigtool","clamav-milter","yara","yarac","claude","codex","npm","node"}
REQUIRED = {"gddrescue","testdisk","clonezilla","gparted","gsmartcontrol","smartmontools","partclone",
            "xfce4","live-boot","linux-image-amd64","e2fsprogs","dosfstools"}
SOURCE_FILES = {"usr/local/bin/inta-rescue":"inta-rescue","usr/local/bin/inta-self-test":"inta-self-test",
                "etc/xdg/autostart/inta-rescue.desktop":"desktop.entry",
                "usr/share/applications/inta-rescue.desktop":"desktop.entry"}


def require(expression, message):
    if not expression:
        raise ValueError(message)


def sha(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def paths_without_payload(paths):
    for item in paths:
        path = PurePosixPath(item.lower().strip("/"))
        parts = path.parts
        if not parts:
            continue
        name = path.name
        # Match packages and libraries as well as unregistered manual installations.
        if (name.endswith((".deb", ".udeb", ".cvd", ".cld", ".cud")) or
            name in COMMANDS-{"node","npm"} or
            name.startswith(("libclamav","libclamunrar","libfreshclam","libyara.so")) or
            name in COMMANDS and any(p in {"bin","sbin",".bin","libexec"} for p in parts[:-1]) or
            any(p in {"clamav","clamd.d","clamav-daemon","clamav-freshclam","inta-ai","@anthropic-ai","@openai"} for p in parts) or
            re.search(r"^(?:clamav(?:[-.].*)?|libclamav[\w.+-]+|freshclam|clamd(?:@.*)?)\.(?:list|md5sums|service|socket|conf|timer)$", name)):
            raise ValueError(f"Excluded recovery payload: {item}")


def packages(status):
    inventory = []
    seen = set()
    for record in status.strip().split("\n\n"):
        fields = {}
        for line in record.splitlines():
            if line.startswith((" ", "\t")):
                continue
            key, separator, value = line.partition(":")
            require(separator and re.fullmatch(r"[A-Za-z][A-Za-z0-9-]*",key) and key not in fields, "Invalid Debian package control record")
            fields[key] = value.lstrip(" ")
        name, version, arch, state = (fields.get(k, "") for k in ("Package","Version","Architecture","Status"))
        require(re.fullmatch(r"[a-z0-9][a-z0-9.+-]+",name) and version and arch and state, "Incomplete package identity")
        spec = re.fullmatch(r"([a-z0-9][a-z0-9.+-]+)(?: \(([^()\s]+)\))?",fields.get("Source",name))
        require(spec,"Invalid Debian source package identity")
        source, source_version = spec[1], spec[2] or version
        require(not (DENIED_PACKAGES.fullmatch(name) or DENIED_PACKAGES.fullmatch(source)),
                f"Excluded package or source: {name} ({source})")
        require((name,arch) not in seen, "Duplicate Debian package identity")
        seen.add((name,arch))
        require(state in {"install ok installed","hold ok installed","deinstall ok config-files","purge ok config-files"},
                f"Incomplete Debian package state: {name}")
        inventory.append(dict(package=name,version=version,architecture=arch,source=source,sourceVersion=source_version,status=state))
    present = {r["package"] for r in inventory if r["status"].endswith(" installed")}
    require(REQUIRED <= present, "Missing required recovery packages: "+", ".join(sorted(REQUIRED-present)))
    return sorted(inventory, key=lambda r:(r["package"],r["architecture"]))


def qualified(iso, digest, revision, source, scratch):
    require(re.fullmatch(r"[0-9a-f]{64}",digest) and re.fullmatch(r"[0-9a-f]{40}",revision), "Invalid pinned digest or revision")
    require(not iso.is_symlink() and iso.is_file() and 0 < iso.stat().st_size <= 8*1024*MIB, "Select a regular bounded ISO")
    initial = iso.stat()
    require(sha(iso)==digest, "ISO SHA-256 does not match")
    # Pin the open descriptor throughout inspection, including all child-tool reads.
    with iso.open("rb") as held:
        current = os.fstat(held.fileno())
        require((current.st_ino,current.st_dev,current.st_mtime_ns,current.st_size)==
                (initial.st_ino,initial.st_dev,initial.st_mtime_ns,initial.st_size), "Image changed before inspection")
        fd = held.fileno()
        prefix = ["xorriso","-abort_on","FAILURE","-indev",f"stdio:/proc/self/fd/{fd}"]
        def run(argv, maximum=64*MIB):
            import resource
            def bound():
                resource.setrlimit(resource.RLIMIT_FSIZE,(maximum,maximum))
            with tempfile.TemporaryFile(dir=scratch) as out:
                subprocess.run(argv,stdout=out,stderr=subprocess.PIPE,stdin=subprocess.DEVNULL,
                               pass_fds=(fd,),preexec_fn=bound,timeout=240,check=True,env={**os.environ,"LC_ALL":"C"})
                require(out.tell() <= maximum,"Tool output exceeded the configured limit")
                out.seek(0)
                return out.read()
        def extract(name, filename, limit):
            out = scratch / filename
            run(prefix+["-osirrox","on","-extract",name,str(out)], limit)
            require(out.is_file() and not out.is_symlink() and 0 < out.stat().st_size <= limit, "Unexpected extracted file")
            return out
        pvd = run(prefix+["-pvd_info"]).decode()
        require(re.search(r"^Volume Id\s+: INTA_RECOVERY$",pvd,re.M), "Unexpected ISO volume ID")
        require(re.search(r"^App Id\s+: INTA RESCUE RECOVERY$",pvd,re.M), "Unexpected ISO application ID")
        boot = run(prefix+["-report_el_torito","plain"]).decode()
        require(re.findall(r"El Torito img path\s+:\s+\d+\s+(\S+)",boot)==
                ["/boot/grub/grub_eltorito","/boot/grub/efi.img"], "Unexpected boot images")
        require(re.search(r"1\s+BIOS\s+y\s+none",boot) and re.search(r"2\s+UEFI\s+y\s+none",boot),"Missing BIOS or UEFI boot record")
        iso_files = [shlex.split(line)[0] for line in run(prefix+["-find","/","-type","f","-exec","echo"]).decode().splitlines()]
        paths_without_payload(iso_files)
        rootfs = "/live/filesystem.squashfs"
        initrds = [p for p in iso_files if p.startswith("/live/initrd.img-")]
        require([p for p in iso_files if p.endswith(".squashfs")] == [rootfs] and len(initrds)==1, "Unexpected live payload count")
        kernel = initrds[0].replace("initrd.img-","vmlinuz-",1)
        require({p for p in iso_files if p.startswith("/live/")} ==
                {rootfs, initrds[0],kernel,"/live/filesystem.packages"}, "Unexpected live image payload")
        efi = extract("/boot/grub/efi.img","efi.img",32*MIB)
        efi_files = run(["mdir","-b","-s","-i",str(efi),"::/"]).decode().splitlines()
        paths_without_payload(efi_files)
        require(any(p.lower().endswith("/bootx64.efi") for p in efi_files),"Missing x64 EFI loader")
        initrd = extract(initrds[0],"initrd",256*MIB)
        init_files = run(["lsinitramfs",str(initrd)],32*MIB).decode().splitlines()
        require(len(init_files)>100 and any(p.strip("./")=="init" for p in init_files),"Incomplete initial RAM disk")
        paths_without_payload(init_files)
        squash = extract(rootfs,"filesystem.squashfs",initial.st_size)
        def cat(name, limit):
            return run(["unsquashfs","-processors","1","-cat",str(squash),name],limit)
        raw = run(["unsquashfs","-no-progress","-ls",str(squash)],128*MIB).decode().splitlines()
        listing = [p.removeprefix("squashfs-root/") for p in raw if p.startswith("squashfs-root/")]
        require(len(listing)>1000 and "var/lib/dpkg/status" in listing,"Incomplete root filesystem inventory")
        paths_without_payload(listing)
        require(cat("etc/inta-edition",128)==b"recovery-only-v1\n" and cat("etc/inta-source-revision",128)==(revision+"\n").encode(),"Unexpected product or build revision")
        for name, original in SOURCE_FILES.items():
            require(cat(name,MIB)==(source/"recovery"/original).read_bytes(), "Installed recovery source differs: "+name)
        registry=json.loads(cat("usr/share/inta/data/brand-icons.json",MIB))
        require({b["id"] for b in registry["brands"]}=={"app-inta","app-inta-rescue"} and len(registry["assets"])==1,
                "Unexpected embedded brand record")
        brand=next(iter(registry["assets"].values()))
        png="assets/brand-icons/70ad9f0981b05341142b.png"
        require(brand["path"]==png and brand["sourceUrl"]=="https://raw.githubusercontent.com/Herbertofury/INTA/main/docs/assets/inta-mark.svg",
                "Unexpected artwork source")
        records=[p for p in listing if p.startswith("usr/share/inta/assets/brand-icons/")]
        require(records==["usr/share/inta/"+png],"Unexpected additional wrapper artwork")
        mark=cat("usr/share/icons/hicolor/scalable/apps/inta-rescue.svg",MIB)
        require(mark==(source/"docs/assets/inta-mark.svg").read_bytes() and hashlib.sha256(mark).hexdigest()==brand["sourceSha256"]
                and cat("usr/share/inta/"+png,MIB)==(source/png).read_bytes()
                and hashlib.sha256((source/png).read_bytes()).hexdigest()==brand["sha256"],"Original INTA artwork differs")
        candidates=[p for p in listing if re.fullmatch(r"usr/share/icons/hicolor/(?:24x24|22x22|32x32|16x16)/apps/(?:gparted|gsmartcontrol|org\.gnome\.GParted|org\.gsmartcontrol\.GSmartControl)\.png",p)]
        owner_rows={}
        for name in ("gparted","gparted-common","gsmartcontrol"):
            info="var/lib/dpkg/info/"+name+".list"
            if info in listing:
                for p in cat(info,2*MIB).decode().splitlines():
                    if p.lstrip("/") in candidates:
                        owner_rows[p.lstrip("/")]=name
        require(set(owner_rows)==set(candidates),"A candidate program icon has no Debian package owner")
        artwork={"wrapperAssets":1,"wrapperLabels":["INTA","INTA Rescue"],"originalSVG":brand["sourceSha256"],"renderedPNG":brand["sha256"],
                 "debianProgramIcons":[{"path":p,"package":owner_rows[p],"sha256":hashlib.sha256(cat(p,MIB)).hexdigest()} for p in sorted(candidates)]}
        os_release = cat("usr/lib/os-release",65536).decode().splitlines()
        require("ID=debian" in os_release and "VERSION_CODENAME=trixie" in os_release, "Expected Debian trixie")
        inventory = packages(cat("var/lib/dpkg/status",32*MIB).decode())
        package_file = extract("/live/filesystem.packages","package-list",8*MIB)
        listed = [line.split()[0].split(":")[0] for line in package_file.read_text().splitlines() if line.strip()]
        require(all(not DENIED_PACKAGES.fullmatch(name) for name in listed) and REQUIRED <= set(listed), "Invalid ISO package list")
        held.seek(0)
        require(hashlib.file_digest(held,"sha256").hexdigest()==digest and os.path.samestat(iso.stat(),current),
                "Image changed during qualification")
        receipt = {"schemaVersion":1,"edition":"recovery-only-v1","sourceRevision":revision,"sha256":digest,
            "bytes":initial.st_size,"debian":"trixie","isoVolume":"INTA_RECOVERY","isoFiles":len(iso_files),
            "efiEntries":len(efi_files),"initramfsEntries":len(init_files),"rootfsEntries":len(listing),
            "packageRecords":len(inventory),"squashfsSha256":sha(squash),"initramfsSha256":sha(initrd),
            "artwork":artwork,
            "absent":["ClamAV binaries, libraries, packages and data","YARA binaries and packages","proprietary AI client payloads","cached Debian package archives"],
            "checkedAt":datetime.now(timezone.utc).isoformat()}
        return receipt,inventory


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("iso", type=Path)
    p.add_argument("sha256")
    p.add_argument("source_revision")
    p.add_argument("new_evidence_directory",type=Path)
    args=p.parse_args()
    with tempfile.TemporaryDirectory(prefix="inta-recovery-gate-") as working:
        result,inventory=qualified(args.iso.absolute(),args.sha256,args.source_revision,Path(__file__).resolve().parents[1],Path(working))
    args.new_evidence_directory.mkdir(parents=True,exist_ok=False)
    sbom=args.new_evidence_directory/"debian-package-inventory.json"
    sbom.write_text(json.dumps(inventory,indent=2)+"\n")
    result["packageInventorySha256"]=sha(sbom)
    (args.new_evidence_directory/"image-qualification.json").write_text(json.dumps(result,indent=2)+"\n")
    print(json.dumps(result,indent=2))


if __name__=="__main__":
    main()
