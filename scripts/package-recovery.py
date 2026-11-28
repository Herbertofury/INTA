#!/usr/bin/env python3
"""Stage a release only after all local recovery-only evidence agrees on the ISO."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import runpy
import shutil
import tarfile
import zipfile


def digest(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream,"sha256").hexdigest()


def require(condition, detail):
    if not condition:
        raise ValueError(detail)


def read(path):
    return json.loads(path.read_bytes())


def additional_boot_receipts(secure, usb, iso_hash, size):
    receipts = {}
    for name, directory, serials in (
        ("enrolledSecureBoot", secure, ["secure-serial.log"]),
        ("virtualVentoyUSB", usb, ["bios-usb.log", "uefi-usb.log"]),
    ):
        if directory is None:
            continue
        receipt = read(directory / "evidence.json")
        require(receipt["isoSHA256"] == iso_hash and receipt["networkDevices"] == 0,
                f"{name} tested a different image or network configuration")
        if name == "enrolledSecureBoot":
            require(receipt["bytes"] == size and receipt["pflashSecure"] is True and
                    receipt["SMM"] is True and receipt["writableDataDisks"] == 0 and
                    "SecureBootEnable ON" in receipt["firmware"] and
                    "verified before and after boot" in receipt["firmware"] and
                    all(re.fullmatch(r"[0-9a-f]{64}", receipt[key])
                        for key in ("firmwareCodeSHA256", "initialMSVarsSHA256")),
                    "Missing active Secure Boot or enrolled firmware evidence")
        else:
            require(receipt["usbFirmware"] == ["bios", "uefi"] and receipt["passed"] is True
                    and receipt["deviceIdentityMismatchRejected"] is True and receipt["sentinelPreserved"] is True
                    and re.fullmatch(r"[0-9a-f]{64}", receipt["ventoyArchiveSHA256"])
                    and digest(directory / "inta-rescue-virtual-usb.qcow2") == receipt["virtualUSBSHA256"],
                    "Missing verified virtual USB evidence")
        serial_digests = {}
        for serial in serials:
            path = directory / serial
            lines = path.read_bytes().splitlines()
            require(b"INTA_SMOKE_PASS" in lines and b"INTA_RECOVERY_ONLY_RUNTIME_OK" in lines,
                    f"{name} guest did not confirm the recovery runtime")
            serial_digests[serial] = digest(path)
        receipts[name] = {**receipt, "serialSHA256": serial_digests}
    return receipts


def package(a):
    gate = read(a.qualification/"image-qualification.json")
    iso_hash = digest(a.iso)
    revision = gate["sourceRevision"]
    require(a.iso.name=="inta-rescue-recovery-amd64.iso" and gate["edition"]=="recovery-only-v1"
            and gate["sha256"]==iso_hash and gate["bytes"]==a.iso.stat().st_size, "Exact product identity required")
    pkg = a.qualification/"debian-package-inventory.json"
    require(digest(pkg)==gate["packageInventorySha256"],"Package inventory digest differs")
    sources=read(a.source_index)
    require(sources["inventorySha256"]==digest(pkg),"Source index is for a different package inventory")
    installed=[p for p in read(pkg) if p["status"].endswith(" installed")]
    boot=sources["bootAndBuildComponents"]
    require(boot["isoSha256"]==iso_hash and boot["imageSourceRevision"]==revision and boot["matchedExternalFileCount"]>0
        and {"grub2","grub-efi-amd64-signed","shim","shim-signed","syslinux","live-build","rustc"}<={p["source"] for p in boot["packages"]}
        and "grub-efi-amd64-signed" in boot["matchedExternalFiles"]["ISO/EFI/boot/grubx64.efi"]
        and "shim-signed" in boot["matchedExternalFiles"]["ISO/EFI/boot/bootx64.efi"], "Missing external boot payload evidence")
    require({(p["source"],p["sourceVersion"]) for p in installed+boot["packages"]}==
            {(s["source"],s["version"]) for s in sources["sourcePackages"]},"Incomplete source archive coverage")
    volumes=runpy.run_path(str(Path(__file__).with_name("recovery-source-volumes.py")))["verify_volumes"](a.source_index,a.source_volumes)
    require(re.fullmatch(r"[a-f0-9]{40}",a.media_source_revision) and digest(a.media_creator)==a.media_sha256,
            "The companion must match its approved digest and source revision")
    with zipfile.ZipFile(a.media_creator) as archive:
        artwork=[name for name in archive.namelist() if name.startswith("inta-resources/assets/brand-icons/")]
        require(artwork==["inta-resources/assets/brand-icons/70ad9f0981b05341142b.png"]
                and hashlib.sha256(archive.read(artwork[0])).hexdigest()==gate["artwork"]["renderedPNG"],
                "Unexpected companion artwork")
        channel=json.loads(archive.read("inta-resources/data/rescue-recovery-download-channel.json"))
        require(channel["edition"]=="recovery-only-v1" and channel["qualification"]=="recovery-only" and
                channel["filename"]==a.iso.name and channel["sha256"]==iso_hash and channel["bytes"]==gate["bytes"]
                and channel["sourceRevision"]==revision and channel["volumeId"]=="INTA_RECOVERY","Media Creator targets a different image")
    with tarfile.open(a.build_source,"r:gz") as archive:
        require(archive.pax_headers.get("comment")==revision,"Build input archive revision differs")
        wrapper_png=[m.name for m in archive if m.isfile() and "/assets/brand-icons/" in m.name]
        require(wrapper_png==["inta-recovery/assets/brand-icons/70ad9f0981b05341142b.png"],"Build source contains additional wrapper artwork")
    with tarfile.open(a.notices,"r:gz") as archive:
        included={m.name for m in archive.getmembers() if m.isfile() and m.size>0}
        require(all("packages/"+p["package"]+"/copyright" in included for p in installed),"Missing package licenses")
        require(all("boot-and-build/"+p["package"]+"/copyright" in included for p in boot["packages"]),"Missing boot component licenses")
    firmware = read(a.firmware/"firmware.json")
    require(firmware["sha256"]==iso_hash and firmware["guestNetworkDevices"]==0 and firmware["writableGuestDisks"]==0
            and [v["firmware"] for v in firmware["firmware"]]==["bios","uefi"],"Missing exact firmware evidence")
    for item in firmware["firmware"]:
        require(item["passed"] is True and item["nativeOptics"] is True and item["recoveryRuntime"] is True
                and digest(a.firmware/(item["firmware"]+"-serial.log"))==item["serialSha256"],"Incomplete firmware receipt")
    ui=read(a.ui/"evidence.json")
    require(ui["sha256"]==iso_hash and ui["nativeRenderer"] is True and ui["settled"] is True
            and ui["networkDevices"]==0 and ui["writableGuestDisks"]==0
            and ui["verifiedTabs"]==["Inspect","Recover files","Copy and repair"],"Missing recovery UI evidence")
    frames={p:digest(a.ui/(p+".ppm")) for p in ("FULL","CLASSIC")}
    require(len(set(frames.values()))==2,"Expected distinct guest-rendered views")
    repair=read(a.repair)
    require(repair["imageSHA256"]==iso_hash and repair["revision"]==revision and repair["sourceReadOnly"] is True
            and repair["copyFsckReadOnlyExit"]==0 and repair["restoredBytes"]>0 and repair["guestNetworkDevices"]==0
            and repair["writableGuestDisks"]==1 and all(re.fullmatch(r"[0-9a-f]{64}",repair[k])
            for k in ("sourceImageBeforeAfter","originalFileAndRestoredFileSHA256")),"Missing storage recovery proof")
    independent=read(a.independent_absence)
    require(independent["sha256"]==iso_hash and independent["revision"]==revision and independent["scannerPackages"]==[]
            and independent["scannerFilesOrAIClients"]==[] and independent["installedPackages"]==len(installed),"Independent absence proof differs")
    result={"schemaVersion":1,"edition":gate["edition"],"sourceRevision":revision,"isoSha256":iso_hash,
            "firmware":firmware,"graphicalDesktop":ui,"generatedFrameSha256":frames,"syntheticDamagedFatRecovery":repair,
            "independentAbsence":independent,"installedDebianPackages":len(installed),"exactSourcePackageVersions":len(sources["sourcePackages"]),
            "bootAndBuildPackageRecords":boot["packageCount"],"externalBootFileMatches":boot["matchedExternalFileCount"],
            "actualDebianSourceFiles":volumes["sourceFiles"],"actualDebianSourceBytes":volumes["sourceBytes"],
            "verifiedSourceVolumes":[v["name"] for v in volumes["volumes"]],
            "mediaCreator":{"sourceRevision":a.media_source_revision,"sha256":a.media_sha256,"tag":channel["tag"]},
            "runtimeScope":"Tests use isolated virtual hardware. Physical USB devices and physical firmware require separate hardware qualification."}
    result.update(additional_boot_receipts(a.secure_boot, a.virtual_usb, iso_hash, gate["bytes"]))
    inputs={a.iso.name:a.iso,"image-qualification.json":a.qualification/"image-qualification.json",
            "debian-package-inventory.json":pkg,"debian-source-index.json":a.source_index,"debian-notices.tar.gz":a.notices,
            "INTA-Recovery-build-source.tar.gz":a.build_source,"RECOVERY-GUIDE.md":Path(__file__).resolve().parents[1]/"recovery/README.md"}
    inputs["INTA-Media-Creator.pyz"]=a.media_creator
    inputs["debian-source-volumes.json"]=a.source_volumes/"debian-source-volumes.json"
    for volume in volumes["volumes"]:
        inputs[volume["name"]]=a.source_volumes/volume["name"]
    a.new_output_directory.mkdir(parents=True,exist_ok=False)
    hashes={}
    for name,src in inputs.items():
        dest=a.new_output_directory/name
        with src.open("rb") as incoming, dest.open("xb") as out:
            shutil.copyfileobj(incoming,out,1024**2)
        hashes[name]=digest(dest)
        require(hashes[name]==digest(src),"Staged release content differs")
    require(hashes[a.iso.name]==iso_hash,"ISO changed while staging")
    verification=a.new_output_directory/"verification.json"
    with verification.open("x",encoding="utf-8",newline="\n") as out:
        json.dump(result,out,indent=2)
        out.write("\n")
    hashes[verification.name]=digest(verification)
    with (a.new_output_directory/"SHA256SUMS").open("x",encoding="ascii",newline="\n") as out:
        for name,sha in sorted(hashes.items()):
            out.write(f"{sha}  {name}\n")
    print(f"Verified {len(hashes)} release files")


if __name__=="__main__":
    p=argparse.ArgumentParser(description=__doc__)
    for name in ["iso","qualification","firmware","ui","repair","independent-absence","source-index","source-volumes","notices","build-source","media-creator"]:
        p.add_argument("--"+name,type=Path,required=True)
    p.add_argument("--secure-boot", type=Path, help="Directory containing enrolled OVMF proof and guest serial log")
    p.add_argument("--virtual-usb", type=Path, help="Directory containing verified virtual USB image, proof and both guest serial logs")
    p.add_argument("--media-sha256",required=True)
    p.add_argument("--media-source-revision",required=True)
    p.add_argument("new_output_directory",type=Path)
    package(p.parse_args())
