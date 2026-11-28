"""Exercise the bundled Workbench downloader with a genuine El Torito fixture."""
import argparse
import copy
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import struct
import sys
import tempfile
import unittest
from unittest.mock import Mock, patch

parser = argparse.ArgumentParser(add_help=False)
parser.add_argument("--payload", type=Path)
options, remaining_args = parser.parse_known_args()
WORKBENCH = options.payload.resolve() if options.payload else Path(__file__).resolve().parents[1]
sys.path.insert(0, str(WORKBENCH))
spec = importlib.util.spec_from_file_location("workbench_media", WORKBENCH / "media/inta-media.py")
media = importlib.util.module_from_spec(spec)
spec.loader.exec_module(media)
sys.path.pop(0)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def boot_fixture():
    image = bytearray(24 * 2048)
    for sector, kind in ((16, 1), (17, 0), (18, 255)):
        offset = sector * 2048
        image[offset:offset+7] = bytes([kind]) + b"CD001\x01"
    image[16*2048+40:16*2048+44] = b"INTA"
    image[17*2048+7:17*2048+30] = b"EL TORITO SPECIFICATION"
    struct.pack_into("<I", image, 17*2048+71, 20)
    offset = 20*2048
    image[offset] = 1
    image[offset+30:offset+32] = b"\x55\xaa"
    struct.pack_into("<H", image, offset+28, -(1 + 0xaa55) & 0xffff)
    image[offset+32] = 0x88
    struct.pack_into("<HI", image, offset+38, 4, 21)
    return bytes(image)


class Fixture:
    def __init__(self):
        self.iso = boot_fixture()
        self.digest = sha(self.iso)
        self.tag = "rescue-12.1"
        self.channel = {"schemaVersion": 1, "tag": self.tag, "qualification": "historical-recovery",
                        "sha256": self.digest, "scannerWarning": "Example offline-only warning."}
        self.api = media.RELEASE_API + self.tag
        self.base = media.ASSET_ROOT + self.tag + "/"
        self.urls = {}
        self.assets = {}
        self.requests = []
        self.reads = []
        self.add(media.SUMS, f"{self.digest}  {media.ISO}\n".encode("ascii"))
        parts = []
        for index, chunk in enumerate((self.iso[:13000], self.iso[13000:41000], self.iso[41000:]), 1):
            name = f"{media.ISO}.part{index:03d}"
            self.add(name, chunk)
            parts.append({"name":name, "size":len(chunk), "sha256":sha(chunk)})
        self.parts = {"schemaVersion":1, "filename":media.ISO, "size":len(self.iso),
                      "sha256":self.digest, "parts":parts}
        self.set_parts()

    def add(self, name, data):
        self.urls[self.base + name] = data
        self.assets[name] = {"name":name, "size":len(data), "browser_download_url":self.base+name,
                             "digest":"sha256:"+sha(data)}

    def set_parts(self):
        self.add(media.PARTS, json.dumps(self.parts).encode())

    def open(self, url, timeout):
        self.requests.append(url)
        if url == self.api:
            data = json.dumps({"tag_name":self.tag, "draft":False, "prerelease":False,
                               "assets":list(self.assets.values())}).encode()
        else:
            data = self.urls[url]
        reads = self.reads
        class CheckedReader(io.BytesIO):
            def read(self, size=-1):
                if not 0 < size <= 1024*1024+1:
                    raise AssertionError(f"Unexpected unbounded or oversized read {size}")
                reads.append(size)
                return super().read(size)
        return CheckedReader(data)

    def run(self, dest, reviewed=True):
        with patch.object(media, "download_channel", return_value=self.channel), \
                patch.object(media.urllib.request, "build_opener", return_value=self):
            return media.download_release(dest, copy.deepcopy(self.channel) if reviewed else None)

    def target(self, dest):
        return dest / f"{self.tag}-{self.digest[:12]}"


class WorkbenchMediaTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix="inta-rescue-media-test-")
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.fixture = Fixture()

    def test_bundled_channel_and_warning(self):
        c = media.download_channel()
        self.assertRegex(c["tag"], r"^rescue-[0-9]+\.[0-9]+$")
        if c["qualification"] == "historical-recovery":
            self.assertTrue(c["scannerWarning"].strip())
        with self.assertRaisesRegex(ValueError, "scanner warning"):
            self.fixture.run(self.root, reviewed=False)
        self.assertEqual(self.fixture.requests, [])

    def test_dialog_shows_full_warning_before_selection_or_download(self):
        messages, chooser, start = Mock(), Mock(), Mock()
        parent = object()
        channel = self.fixture.channel
        with patch.object(media, "download_channel", return_value=channel):
            messages.askokcancel.return_value = False
            media.choose_download(parent, messages, chooser, start)
            title, body = messages.askokcancel.call_args.args
            self.assertIn("Historical", title)
            self.assertIn(channel["tag"], body)
            self.assertIn(channel["scannerWarning"], body)
            self.assertEqual(messages.askokcancel.call_args.kwargs["default"], "cancel")
            chooser.askdirectory.assert_not_called()
            start.assert_not_called()
            messages.askokcancel.return_value = True
            chooser.askdirectory.return_value = str(self.root)
            media.choose_download(parent, messages, chooser, start)
        self.assertEqual(start.call_args.args[1:], (media.download_release, self.root, channel))

    def test_verified_parts_and_networkless_repeat(self):
        f = self.fixture
        f.run(self.root)
        target = f.target(self.root)
        self.assertEqual((target / media.ISO).read_bytes(), f.iso)
        self.assertEqual(media.verify_iso(target / media.ISO, f.digest)["bootEntries"], ["bios"])
        self.assertEqual(media.image_digest(target / media.ISO), f.digest)
        self.assertEqual(f.requests, [f.api]+[f.base+n for n in
                        (media.SUMS, media.PARTS, *[p["name"] for p in f.parts["parts"]])])
        self.assertTrue(f.reads)
        requests=len(f.requests)
        f.run(self.root)
        self.assertEqual(requests, len(f.requests))

    def test_monolithic_release(self):
        f = self.fixture
        f.add(media.ISO, f.iso)
        f.run(self.root)
        self.assertEqual((f.target(self.root) / media.ISO).read_bytes(), f.iso)
        self.assertNotIn(f.base+media.PARTS, f.requests)

    def test_damage_does_not_publish_or_touch_earlier_release(self):
        previous = self.root / "earlier-release"
        previous.mkdir()
        old = previous / media.ISO
        old.write_bytes(b"preserved")
        mutations = ("corrupt-part","truncated","manifest-order","mismatched-tag","wrong-pin",
                     "oversized","duplicate","wrong-part-digest","oversized-checksums")
        for mutation in mutations:
            with self.subTest(mutation=mutation):
                f = Fixture()
                n = f.parts["parts"][1]["name"]
                if mutation == "corrupt-part":
                    f.urls[f.base+n] = b"x"*len(f.urls[f.base+n])
                elif mutation == "truncated":
                    f.urls[f.base+n] = f.urls[f.base+n][:-1]
                elif mutation == "manifest-order":
                    f.parts["parts"].reverse()
                    f.set_parts()
                elif mutation == "mismatched-tag":
                    f.assets[n]["browser_download_url"] = media.ASSET_ROOT+"rescue-11.1/"+n
                elif mutation == "wrong-pin":
                    f.channel["sha256"] = "0"*64
                elif mutation == "oversized":
                    f.assets[n]["size"] = media.MAX_PART+1
                elif mutation == "duplicate":
                    f.assets["extra"] = f.assets[n]
                elif mutation == "wrong-part-digest":
                    f.assets[n]["digest"] = "sha256:"+"0"*64
                else:
                    f.assets[media.SUMS]["size"] = 65537
                with self.assertRaises(ValueError):
                    f.run(self.root)
                self.assertEqual(list(self.root.iterdir()), [previous])
                self.assertEqual(old.read_bytes(), b"preserved")

    def test_damaged_existing_exact_target_is_never_overwritten(self):
        f = self.fixture
        target = f.target(self.root)
        target.mkdir()
        (target / media.ISO).write_bytes(b"previous")
        (target / media.SUMS).write_text(f"{f.digest}  {media.ISO}\n", encoding="ascii")
        with self.assertRaises(ValueError):
            f.run(self.root)
        self.assertEqual((target / media.ISO).read_bytes(), b"previous")
        self.assertEqual(f.requests, [])


if __name__ == "__main__":
    unittest.main(argv=[sys.argv[0], *remaining_args])
