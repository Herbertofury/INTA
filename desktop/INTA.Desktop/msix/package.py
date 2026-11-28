"""Build or independently verify the Windows x64 Desktop preview installer."""

import argparse
import base64
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import struct
import subprocess
import sys
import tempfile
import xml.etree.ElementTree as ET
import zipfile


HERE = Path(__file__).resolve().parent
EXE = "INTA.Desktop.exe"
LOGOS = ("logo44.png", "logo50.png", "logo150.png")
MANIFEST = "AppxManifest.xml"
SIG = "AppxSignature.p7x"
LIMIT = 300 * 1024 * 1024


def digest(stream):
    return hashlib.file_digest(stream, "sha256").hexdigest()


def pin(value):
    if not re.fullmatch("[a-fA-F0-9]{64}", value):
        raise ValueError("Expected a 64-character SHA-256 digest")
    return value.lower()


def manifest(version):
    if (not re.fullmatch(r"(?:0|[1-9][0-9]{0,4})(?:\.(?:0|[1-9][0-9]{0,4})){3}", version)
            or any(int(part) > 65535 for part in version.split(".")) or version == "0.0.0.0"):
        raise ValueError("MSIX requires a nonzero four-part version with fields from 0 to 65535")
    return (HERE / MANIFEST).read_bytes().replace(b'Version="0.1.0.0"', f'Version="{version}"'.encode("ascii"))


def sdk(tool):
    folder = Path(os.environ["ProgramFiles(x86)"]) / "Windows Kits/10/bin/10.0.26100.0/x64"
    path = folder / tool
    if not path.is_file():
        raise ValueError(f"Install the Microsoft Windows SDK 10.0.26100.0 x64 tools ({tool})")
    return path


def xml_contents(element):
    return (element.tag, element.attrib, (element.text or "").strip(), [xml_contents(child) for child in element])


def normalize_zip_times(package):
    # MakeAppx uses its wall clock for both ZIP headers. Leave the SDK's packed
    # bytes, block boundaries, ZIP64 offsets, and all header lengths untouched.
    with zipfile.ZipFile(package) as archive:
        files = archive.infolist()
        central = archive.start_dir
    fixed = struct.pack("<HH", 0, (20 << 9) | (1 << 5) | 1)  # 2000-01-01
    with package.open("r+b") as stream:
        for entry in files:
            stream.seek(entry.header_offset)
            if stream.read(4) != b"PK\x03\x04":
                raise ValueError("Invalid SDK local file header")
            stream.seek(entry.header_offset + 10)
            stream.write(fixed)
            stream.seek(central)
            record = stream.read(46)
            if len(record) != 46 or record[:4] != b"PK\x01\x02":
                raise ValueError("Invalid SDK central directory header")
            stream.seek(central + 12)
            stream.write(fixed)
            central += 46 + sum(struct.unpack_from("<HHH", record, 28))


def verify_blocks(archive, payload):
    ns = "{http://schemas.microsoft.com/appx/2010/blockmap}"
    mapping = ET.fromstring(archive.read("AppxBlockMap.xml"))
    if mapping.tag != ns + "BlockMap" or mapping.get("HashMethod") != "http://www.w3.org/2001/04/xmlenc#sha256":
        raise ValueError("Expected an SDK SHA-256 block map")
    seen = set()
    for item in mapping.findall(ns + "File"):
        name = item.attrib["Name"].replace("\\", "/")
        if name not in payload or name in seen or int(item.attrib["Size"]) != archive.getinfo(name).file_size:
            raise ValueError("Invalid block map file inventory")
        seen.add(name)
        with archive.open(name) as stream:
            for block in item.findall(ns + "Block"):
                chunk = stream.read(65536)
                if not chunk or base64.b64encode(hashlib.sha256(chunk).digest()).decode("ascii") != block.get("Hash"):
                    raise ValueError("Installer block-map checksum failed")
            if stream.read(1):
                raise ValueError("Installer block map omits payload bytes")
    if seen != payload:
        raise ValueError("Installer block map omits required files")


def verify_content_types(archive, signed):
    namespace = "{http://schemas.openxmlformats.org/package/2006/content-types}"
    types = ET.fromstring(archive.read("[Content_Types].xml"))
    if types.tag != namespace + "Types":
        raise ValueError("Invalid installer content types")
    defaults, overrides = {}, {}
    for item in types:
        if item.tag == namespace + "Default":
            mapping, key = defaults, item.attrib["Extension"].lower()
        elif item.tag == namespace + "Override":
            mapping, key = overrides, item.attrib["PartName"]
        else:
            raise ValueError("Unexpected installer content type element")
        if key in mapping:
            raise ValueError("Duplicate installer content type")
        mapping[key] = item.attrib["ContentType"]
    required = {EXE: "application/x-msdownload", MANIFEST: "application/vnd.ms-appx.manifest+xml",
                "AppxBlockMap.xml": "application/vnd.ms-appx.blockmap+xml",
                **{"Assets/" + name: "image/png" for name in LOGOS}}
    if signed:
        required[SIG] = "application/vnd.ms-appx.signature"
    for name, media_type in required.items():
        if overrides.get("/" + name, defaults.get(name.rsplit(".", 1)[-1].lower())) != media_type:
            raise ValueError("Incorrect installer content type for " + name)


def publisher(package, fingerprint):
    # SignTool checks MSIX integrity, the Windows trust chain and a timestamp.
    subprocess.run([sdk("signtool.exe"), "verify", "/pa", "/all", "/tw", package], check=True)
    powershell = Path(os.environ["SystemRoot"]) / "System32/WindowsPowerShell/v1.0/powershell.exe"
    script = """$ErrorActionPreference='Stop'
$s=Get-AuthenticodeSignature -LiteralPath $env:INTA_CANDIDATE
if ($s.Status -ne 'Valid' -or $null -eq $s.SignerCertificate -or $null -eq $s.TimeStamperCertificate) { throw 'Publisher signature or timestamp is unavailable' }
$hash=[Security.Cryptography.SHA256]::Create()
try { [BitConverter]::ToString($hash.ComputeHash($s.SignerCertificate.RawData)).Replace('-', '').ToLowerInvariant() } finally { $hash.Dispose() }"""
    actual = subprocess.check_output([powershell, "-NoProfile", "-NonInteractive", "-Command", script],
                                     env={**os.environ, "INTA_CANDIDATE": str(package.resolve()),
                                          "PSModulePath": str(powershell.parent / "Modules")}, text=True).strip()
    if actual != fingerprint:
        raise ValueError("The signing certificate does not match the independently approved publisher fingerprint")


def verify(package, executable_sha256, version, publisher_sha256=None):
    expected = pin(executable_sha256)
    expected_manifest = ET.fromstring(manifest(version))
    if package.stat().st_size > LIMIT:
        raise ValueError("Package exceeds the Desktop preview size bound")
    expected_files = {EXE, MANIFEST, "AppxBlockMap.xml", "[Content_Types].xml", *(f"Assets/{it}" for it in LOGOS)}
    with zipfile.ZipFile(package) as archive:
        names = archive.namelist()
        signed = SIG in names
        if signed:
            expected_files.add(SIG)
        if set(names) != expected_files or len(names) != len(expected_files):
            raise ValueError("Unexpected, missing or duplicate installer content")
        sizes = {entry.filename: entry.file_size for entry in archive.infolist()}
        if sum(sizes.values()) > LIMIT or any(size > 1024 * 1024 for name, size in sizes.items() if name != EXE):
            raise ValueError("Unexpected uncompressed installer content size")
        if xml_contents(ET.fromstring(archive.read(MANIFEST))) != xml_contents(expected_manifest):
            raise ValueError("Installer identity, version, assets or declared application differs from source")
        for item in LOGOS:
            if archive.read("Assets/" + item) != (HERE / item).read_bytes():
                raise ValueError("Installer brand assets differ from source")
        with archive.open(EXE) as stream:
            if digest(stream) != expected:
                raise ValueError("Installer contains a different executable")
        verify_blocks(archive, {EXE, MANIFEST, *(f"Assets/{it}" for it in LOGOS)})
        verify_content_types(archive, signed)
    if publisher_sha256 is not None:
        required = pin(publisher_sha256)
        if not signed:
            raise ValueError("A publisher signature is required for distribution")
        publisher(package, required)
    elif signed:
        raise ValueError("Supply --publisher-sha256 to verify the signed installer")
    with package.open("rb") as stream:
        return {"file": package.name, "version": version, "bytes": package.stat().st_size, "sha256": digest(stream),
                "executableSha256": expected, "publisherVerified": signed}


def build(preview, output, expected, version):
    if output.suffix.lower() != ".msix" or output.exists():
        raise ValueError("Choose a new .msix output file")
    xml = manifest(version)
    subprocess.run([sys.executable, "-B", HERE.parent / "verify-preview.py", preview, "--sha256", pin(expected)], check=True)
    with tempfile.TemporaryDirectory(dir=output.parent, prefix="inta-msix-") as tmp:
        work = Path(tmp)
        stage = work / "payload"
        assets = stage / "Assets"
        assets.mkdir(parents=True)
        (stage / MANIFEST).write_bytes(xml)
        for item in LOGOS:
            shutil.copyfile(HERE / item, assets / item)
        shutil.copyfile(preview / EXE, stage / EXE)
        candidate = work / output.name
        subprocess.run([sdk("makeappx.exe"), "pack", "/np", "/h", "SHA256", "/d", stage, "/p", candidate], check=True)
        normalize_zip_times(candidate)
        receipt = verify(candidate, expected, version)
        candidate.rename(output)  # Windows refuses to overwrite an existing destination.
        return receipt


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="verb", required=True)
    pack = commands.add_parser("build")
    pack.add_argument("preview", type=Path)
    pack.add_argument("output", type=Path)
    check = commands.add_parser("verify")
    check.add_argument("package", type=Path)
    check.add_argument("--publisher-sha256", help="Require a trusted, timestamped signature from this approved certificate")
    for command in (pack, check):
        command.add_argument("--exe-sha256", required=True)
        command.add_argument("--version", required=True)
    args = parser.parse_args()
    try:
        result = (build(args.preview, args.output, args.exe_sha256, args.version) if args.verb == "build" else
                  verify(args.package, args.exe_sha256, args.version, args.publisher_sha256))
        print(json.dumps(result))
    except (OSError, ValueError, KeyError, ET.ParseError, zipfile.BadZipFile, subprocess.CalledProcessError) as error:
        sys.exit(f"MSIX {args.verb} failed: {error}")
