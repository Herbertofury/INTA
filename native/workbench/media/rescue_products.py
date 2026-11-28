"""Pinned Rescue downloads and product-specific readback shared by both Media Creators."""
from contextlib import closing
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import tempfile
import urllib.parse
import urllib.request

if __package__:
    from .rescue_iso import verify_iso
else:
    from rescue_iso import verify_iso

FULL = "full"
RECOVERY = "recovery-only"
ISO = "inta-rescue-amd64.iso"
RECOVERY_ISO = "inta-rescue-recovery-amd64.iso"
CHANNELS = {FULL: "rescue-download-channel.json", RECOVERY: "rescue-recovery-download-channel.json"}
SUMS = "SHA256SUMS"
ASSET_ROOT = "https://github.com/Herbertofury/INTA/releases/download/"
RELEASE_API = "https://api.github.com/repos/Herbertofury/INTA/releases/tags/"
MAX_MANIFEST = 1024 * 1024
MAX_PART = 1024 ** 3
MAX_IMAGE = 32 * 1024 ** 3


def validate(channel, edition=FULL):
    if (edition not in CHANNELS or not isinstance(channel, dict) or type(channel.get("schemaVersion")) is not int
            or channel["schemaVersion"] != 1 or not isinstance(channel.get("sha256"), str)
            or not re.fullmatch(r"[0-9a-f]{64}", channel["sha256"]) or not isinstance(channel.get("tag"), str)):
        raise ValueError("Invalid pinned Rescue product channel.")
    if edition == RECOVERY:
        if (channel.get("qualification") != RECOVERY or channel.get("edition") != "recovery-only-v1"
                or not re.fullmatch(r"rescue-recovery-[0-9]{8}\.[0-9]{1,8}", channel["tag"])
                or channel.get("filename") != RECOVERY_ISO or channel.get("volumeId") != "INTA_RECOVERY"
                or type(channel.get("bytes")) is not int or not 0 < channel["bytes"] <= MAX_IMAGE
                or not isinstance(channel.get("sourceRevision"), str)
                or not re.fullmatch(r"[0-9a-f]{40}", channel["sourceRevision"])):
            raise ValueError("Invalid recovery-only product identity.")
    else:
        if (channel.get("qualification") not in ("qualified", "historical-recovery")
                or not re.fullmatch(r"rescue-[0-9]{1,24}\.[0-9]{1,8}", channel["tag"])):
            raise ValueError("Invalid full Rescue product identity.")
        warning = channel.get("scannerWarning")
        if channel["qualification"] == "historical-recovery" and (
                not isinstance(warning, str) or not 1 <= len(warning.strip()) <= 1500):
            raise ValueError("Historical recovery requires a scanner warning.")
    return channel


def decode(content, edition=FULL):
    if len(content) > 8192:
        raise ValueError("Rescue product metadata exceeds 8 KiB.")
    return validate(json.loads(content, object_pairs_hook=unique_fields), edition)


def readback(path, channel, edition):
    info = verify_iso(path, channel["sha256"])
    if edition == RECOVERY and (info["volumeLabel"] != "INTA_RECOVERY" or info["bytes"] != channel["bytes"]
                                or set(info["bootEntries"]) != {"bios", "uefi"}):
        raise ValueError("The recovery-only image does not match its size, volume or boot profiles.")
    return info


def expected_digest(data, filename=ISO):
    pattern = rf"([0-9a-f]{{64}}) [ *]{re.escape(filename)}"
    matches = [match.group(1) for line in data.splitlines()
               if (match := re.fullmatch(pattern, line))]
    if len(matches) != 1:
        raise ValueError(f"Expected exactly one checksum for {filename}.")
    return matches[0]


def stream_verified(source, target, expected, expected_size=None, before_replace=None):
    digest = hashlib.sha256()
    total = 0
    with tempfile.NamedTemporaryFile(dir=target.parent, prefix=".inta-", delete=False) as output:
        partial = Path(output.name)
        try:
            while chunk := source.read(1024 * 1024):
                total += len(chunk)
                if expected_size is not None and total > expected_size:
                    raise ValueError("The download exceeds the release asset size.")
                digest.update(chunk)
                output.write(chunk)
            if digest.hexdigest() != expected or (expected_size is not None and total != expected_size):
                raise ValueError("Verification failed. Any previous ISO is intact.")
            output.flush()
            os.fsync(output.fileno())
            output.close()
            if before_replace:
                before_replace(partial)
            partial.replace(target)
        finally:
            if partial.exists():
                output.close()
                partial.unlink()


class ReleaseRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        url = urllib.parse.urlsplit(newurl)
        hosts = {"github.com", "api.github.com", "release-assets.githubusercontent.com", "objects.githubusercontent.com"}
        if url.scheme != "https" or url.hostname not in hosts or url.username or url.password or url.port not in (443, None):
            raise ValueError("Unexpected release download redirect.")
        return super().redirect_request(req, fp, code, msg, headers, newurl)


def unique_fields(items):
    result = {}
    for key, value in items:
        if key in result:
            raise ValueError(f"Duplicate JSON field {key}.")
        result[key] = value
    return result


def release_asset(assets, name, tag, limit):
    asset = assets.get(name)
    if not isinstance(asset, dict):
        raise ValueError(f"The release is missing {name}.")
    size = asset.get("size")
    if (asset.get("browser_download_url") != f"{ASSET_ROOT}{tag}/{name}"
            or type(size) is not int or not 0 < size <= limit):
        raise ValueError(f"Unexpected URL or size for {name}.")
    return asset["browser_download_url"], size


def metadata(opener, assets, name, tag, limit):
    url, size = release_asset(assets, name, tag, limit)
    with opener.open(url, timeout=30) as reply:
        content = reply.read(size + 1)
    if len(content) != size:
        raise ValueError(f"Incomplete or oversized {name}.")
    check_asset_digest(assets[name], hashlib.sha256(content).hexdigest())
    return content


def check_asset_digest(asset, expected):
    provided = asset.get("digest")
    if provided is not None and provided != "sha256:" + expected:
        raise ValueError(f"The release asset digest disagrees for {asset['name']}.")


def verified_parts(manifest, assets, tag, expected, filename):
    if (not isinstance(manifest, dict) or set(manifest) != {"schemaVersion", "filename", "size", "sha256", "parts"}
            or type(manifest["schemaVersion"]) is not int or manifest["schemaVersion"] != 1
            or manifest["filename"] != filename or type(manifest["size"]) is not int
            or not 0 < manifest["size"] <= MAX_IMAGE or manifest["sha256"] != expected
            or not isinstance(manifest["parts"], list) or not 1 <= len(manifest["parts"]) <= 999):
        raise ValueError("Invalid ISO parts manifest or pinned checksum mismatch.")
    parts = []
    for index, part in enumerate(manifest["parts"], 1):
        name = f"{filename}.part{index:03d}"
        if (not isinstance(part, dict) or set(part) != {"name", "size", "sha256"}
                or part["name"] != name or type(part["size"]) is not int
                or not isinstance(part["sha256"], str) or not re.fullmatch(r"[0-9a-f]{64}", part["sha256"])):
            raise ValueError(f"Invalid manifest entry for {name}.")
        url, size = release_asset(assets, name, tag, MAX_PART)
        if size != part["size"]:
            raise ValueError(f"The manifest and release sizes differ for {name}.")
        check_asset_digest(assets[name], part["sha256"])
        parts.append((name, url, size, part["sha256"]))
    if sum(p[2] for p in parts) != manifest["size"]:
        raise ValueError("The ordered parts do not add up to the declared ISO size.")
    return manifest["size"], parts


class VerifiedPartsReader:
    """Provide one bounded stream while independently checking each part."""

    def __init__(self, opener, parts):
        self.opener = opener
        self.parts = iter(parts)
        self.reply = None

    def close(self):
        if self.reply is not None:
            self.reply.close()
            self.reply = None

    def read(self, size):
        while True:
            if self.reply is None:
                part = next(self.parts, None)
                if part is None:
                    return b""
                self.name, url, self.remaining, self.expected = part
                self.digest = hashlib.sha256()
                self.reply = self.opener.open(url, timeout=45)
            chunk = self.reply.read(min(size, self.remaining + 1))
            if chunk:
                self.remaining -= len(chunk)
                if self.remaining < 0:
                    raise ValueError(f"Oversized part {self.name}.")
                self.digest.update(chunk)
                return chunk
            self.close()
            if self.remaining or self.digest.hexdigest() != self.expected:
                raise ValueError(f"ISO part {self.name} failed verification.")


def fetch_product(folder, channel, edition):
    validate(channel, edition)
    filename = RECOVERY_ISO if edition == RECOVERY else ISO
    tag, expected = channel["tag"], channel["sha256"]
    product = folder / f"{tag}-{expected[:12]}"
    if product.exists():
        with (product / SUMS).open("rb") as previous:
            saved = previous.read(65537)
        if len(saved) > 65536 or expected_digest(saved.decode("ascii"), filename) != expected:
            raise ValueError("The existing release directory has a different checksum manifest.")
        readback(product / filename, channel, edition)
        return product / filename
    opener = urllib.request.build_opener(ReleaseRedirect)
    with opener.open(RELEASE_API + tag, timeout=30) as reply:
        body = reply.read(MAX_MANIFEST + 1)
    if len(body) > MAX_MANIFEST:
        raise ValueError("Release metadata exceeds 1 MiB.")
    release = json.loads(body, object_pairs_hook=unique_fields)
    if (not isinstance(release, dict) or release.get("tag_name") != tag
            or release.get("draft") is not False or release.get("prerelease") is not False
            or not isinstance(release.get("assets"), list)):
        raise ValueError("The published release differs from the pinned Rescue channel.")
    assets = {}
    for asset in release["assets"]:
        if not isinstance(asset, dict) or not isinstance(asset.get("name"), str) or asset["name"] in assets:
            raise ValueError("Invalid or duplicate release asset.")
        assets[asset["name"]] = asset
    checksums = metadata(opener, assets, SUMS, tag, 65536)
    if expected_digest(checksums.decode("ascii"), filename) != expected:
        raise ValueError("The release checksum differs from the pinned Rescue channel.")
    parts = None
    if filename in assets:
        image_url, image_size = release_asset(assets, filename, tag, MAX_IMAGE)
        check_asset_digest(assets[filename], expected)
    else:
        manifest = json.loads(metadata(opener, assets, filename + ".parts.json", tag, MAX_MANIFEST), object_pairs_hook=unique_fields)
        image_size, parts = verified_parts(manifest, assets, tag, expected, filename)
    if edition == RECOVERY and image_size != channel["bytes"]:
        raise ValueError("Recovery-only asset size differs from the product pin.")
    if shutil.disk_usage(folder).free < image_size:
        raise ValueError("The destination has insufficient free space for a safe replacement.")
    with tempfile.TemporaryDirectory(dir=folder, prefix=".inta-release-") as temp:
        staging = Path(temp) / "image"
        staging.mkdir()
        with (closing(VerifiedPartsReader(opener, parts)) if parts else opener.open(image_url, timeout=45)) as reply:
            stream_verified(reply, staging / filename, expected, image_size)
        readback(staging / filename, channel, edition)
        (staging / SUMS).write_bytes(checksums)
        staging.rename(product)
    return product / filename
