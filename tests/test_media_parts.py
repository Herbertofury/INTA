"""Headless download tests and an opt-in proof against real release files.

CI: python3 tests/test_media_parts.py --parts-dir RELEASE_DIRECTORY \
        --expected-sha256 ORIGINAL_ISO_SHA256

The proof uses the real downloader and local file responses. A parts manifest
forces the multipart route; without one, it exercises the ordinary ISO route.
"""
import argparse
import copy
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import re
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest import mock


spec = importlib.util.spec_from_file_location(
    "inta_media_parts", Path(__file__).resolve().parents[1] / "media" / "inta-media.py")
media = importlib.util.module_from_spec(spec)
spec.loader.exec_module(media)
TEST_TAG = "rescue-123.1"
TEST_API = media.RELEASE_API + TEST_TAG


def fixture_channel(expected):
    return {"schemaVersion": 1, "tag": TEST_TAG, "sha256": expected, "qualification": "qualified"}


def digest(data):
    return hashlib.sha256(data).hexdigest()


class TrackedResponse(io.BytesIO):
    def __init__(self, data):
        super().__init__(data)
        self.read_sizes = []

    def read(self, size=-1):
        self.read_sizes.append(size)
        return super().read(size)


class ReleaseFixture:
    def __init__(self, chunks=(b"first ISO part\x00", b"second ISO part\xff", b"last part")):
        self.image = b"".join(chunks)
        self.expected = digest(self.image)
        self.manifest = {
            "schemaVersion": 1,
            "filename": media.ISO,
            "size": len(self.image),
            "sha256": self.expected,
            "parts": [
                {"name": f"{media.ISO}.part{index:03d}", "size": len(data), "sha256": digest(data)}
                for index, data in enumerate(chunks, 1)
            ],
        }
        self.data = {}
        self.assets = {}
        self.extra_assets = []
        self.requests = []
        self.responses = []
        self.response_factories = {}
        for part, data in zip(self.manifest["parts"], chunks):
            self.add(part["name"], data)
        self.add(media.SUMS, f"{self.expected}  {media.ISO}\n".encode("ascii"))
        self.update_manifest()

    @staticmethod
    def url(name):
        return media.ASSET_ROOT + TEST_TAG + "/" + name

    def add(self, name, data):
        self.data[name] = data
        self.assets[name] = {"name": name, "size": len(data), "browser_download_url": self.url(name)}

    def update_manifest(self):
        self.add(media.PARTS, json.dumps(self.manifest).encode("utf-8"))

    def open(self, url, timeout):
        self.requests.append(url)
        if url == TEST_API:
            release = {"tag_name":TEST_TAG, "draft":False, "prerelease":False,
                       "assets": list(self.assets.values()) + self.extra_assets}
            response = TrackedResponse(json.dumps(release).encode("utf-8"))
        else:
            matches = [name for name in self.data if url == self.url(name)]
            if len(matches) != 1:
                raise AssertionError(f"Unexpected download URL: {url}")
            name = matches[0]
            data = self.data[name]
            if isinstance(data, Exception):
                raise data
            response = self.response_factories.get(name, TrackedResponse)(data)
        self.responses.append(response)
        return response


def verify_local_release_directory(directory, expected_sha256, temporary_directory=None):
    """Assemble actual release bytes through download_release(), then remove them.

    The release API and HTTP responses are mapped to files in the supplied
    directory. No network, Tk, or display is required. The expected digest must
    come from the original boot-tested ISO, independently of the parts manifest.
    """
    directory = Path(directory).resolve()
    if not re.fullmatch(r"[0-9a-fA-F]{64}", expected_sha256):
        raise ValueError("Supply the original ISO SHA-256 digest.")
    expected_sha256 = expected_sha256.lower()
    published_digest = media.expected_digest((directory / media.SUMS).read_text(encoding="ascii"))
    if published_digest != expected_sha256:
        raise ValueError("SHA256SUMS does not match the supplied original ISO digest.")
    if (directory / media.PARTS).is_file():
        names = [media.SUMS, media.PARTS] + sorted(
            path.name for path in directory.iterdir()
            if path.is_file() and re.fullmatch(re.escape(media.ISO) + r"\.part[0-9]{3,}", path.name)
        )
        mode = "parts"
    else:
        names = [media.SUMS, media.ISO]
        mode = "iso"
    urls = {media.ASSET_ROOT + TEST_TAG + "/" + name: directory / name for name in names}
    release = {"tag_name":TEST_TAG, "draft":False, "prerelease":False, "assets": [
        {"name": path.name, "size": path.stat().st_size, "browser_download_url": url}
        for url, path in urls.items()
    ]}
    requested = []

    def local_urlopen(url, timeout):
        requested.append(url)
        if url == TEST_API:
            return io.BytesIO(json.dumps(release).encode("utf-8"))
        if url not in urls:
            raise AssertionError(f"Downloader requested a non-release asset: {url}")
        return urls[url].open("rb")

    with tempfile.TemporaryDirectory(prefix="inta-media-release-proof-", dir=temporary_directory) as folder:
        target = Path(folder) / media.ISO
        with mock.patch.object(media.urllib.request, "urlopen", side_effect=local_urlopen):
            media.download_release(Path(folder), fixture_channel(expected_sha256))
        actual = hashlib.sha256()
        with target.open("rb") as image:
            while chunk := image.read(1024 * 1024):
                actual.update(chunk)
        if actual.hexdigest() != expected_sha256:
            raise AssertionError("The assembled ISO differs from the original ISO digest.")
        if (Path(folder) / media.SUMS).read_text(encoding="ascii") != f"{expected_sha256}  {media.ISO}\n":
            raise AssertionError("The downloaded ISO checksum sidecar is incorrect.")
        if sorted(path.name for path in Path(folder).iterdir()) != sorted([media.ISO, media.SUMS]):
            raise AssertionError("Temporary download or part files were left behind.")
        result = {"mode": mode, "filename": media.ISO, "size": target.stat().st_size,
                  "sha256": actual.hexdigest(), "requestedAssets": [urls[url].name for url in requested if url in urls],
                  "sourceDirectory": str(directory)}
    result["temporaryOutputRemoved"] = not Path(folder).exists()
    if not result["temporaryOutputRemoved"]:
        raise AssertionError("The proof output was not removed.")
    return result


class MediaPartsTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.folder = Path(self.temporary.name)
        self.fixture = ReleaseFixture()

    def download(self, fixture=None):
        fixture = fixture or self.fixture
        with mock.patch.object(media.urllib.request, "urlopen", side_effect=fixture.open):
            media.download_release(self.folder, fixture_channel(fixture.expected))

    def assert_failed_atomically(self, fixture=None, exception=ValueError):
        fixture = fixture or self.fixture
        old_image = b"previous verified ISO"
        old_sums = f"{digest(old_image)}  {media.ISO}\n"
        (self.folder / media.ISO).write_bytes(old_image)
        (self.folder / media.SUMS).write_text(old_sums, encoding="ascii")
        with self.assertRaises(exception):
            self.download(fixture)
        self.assertEqual((self.folder / media.ISO).read_bytes(), old_image)
        self.assertEqual((self.folder / media.SUMS).read_text(encoding="ascii"), old_sums)
        self.assertEqual(sorted(path.name for path in self.folder.iterdir()), sorted([media.ISO, media.SUMS]))
        self.assertTrue(all(response.closed for response in fixture.responses))

    def test_streamed_parts_produce_the_original_iso_and_checksum(self):
        self.fixture = ReleaseFixture((b"A" * (1024 * 1024 + 17), b"B" * 51, b"C" * 23))
        (self.folder / media.ISO).write_bytes(b"previous")
        self.download()
        self.assertEqual((self.folder / media.ISO).read_bytes(), self.fixture.image)
        self.assertEqual((self.folder / media.SUMS).read_text(encoding="ascii"), f"{self.fixture.expected}  {media.ISO}\n")
        self.assertEqual(self.fixture.requests, [TEST_API, self.fixture.url(media.SUMS), self.fixture.url(media.PARTS)] +
                         [self.fixture.url(part["name"]) for part in self.fixture.manifest["parts"]])
        self.assertEqual(sorted(path.name for path in self.folder.iterdir()), sorted([media.ISO, media.SUMS]))
        for response in self.fixture.responses[3:]:
            self.assertTrue(response.closed)
            self.assertTrue(all(0 < size <= 1024 * 1024 for size in response.read_sizes))

    def test_ordinary_iso_is_preferred_and_ignores_invalid_parts(self):
        self.fixture.add(media.ISO, self.fixture.image)
        self.fixture.add(media.PARTS, b"not valid JSON")
        self.download()
        self.assertEqual((self.folder / media.ISO).read_bytes(), self.fixture.image)
        self.assertEqual(self.fixture.requests, [TEST_API, self.fixture.url(media.SUMS), self.fixture.url(media.ISO)])

    def test_ordinary_iso_without_a_manifest_still_downloads(self):
        self.fixture.add(media.ISO, self.fixture.image)
        del self.fixture.assets[media.PARTS]
        self.download()
        self.assertEqual((self.folder / media.ISO).read_bytes(), self.fixture.image)

    def test_corrupt_short_and_oversized_parts_preserve_previous_files(self):
        for alteration in (lambda data: b"?" + data[1:], lambda data: data[:-1], lambda data: data + b"extra"):
            with self.subTest(alteration=alteration):
                fixture = ReleaseFixture()
                name = fixture.manifest["parts"][1]["name"]
                fixture.data[name] = alteration(fixture.data[name])
                self.assert_failed_atomically(fixture)

    def test_missing_part_fails_before_any_part_download(self):
        del self.fixture.assets[self.fixture.manifest["parts"][1]["name"]]
        self.assert_failed_atomically()
        self.assertEqual(len(self.fixture.requests), 3)

    def test_duplicate_release_assets_are_rejected_before_downloading(self):
        for name in (self.fixture.manifest["parts"][0]["name"], media.ISO):
            with self.subTest(name=name):
                fixture = ReleaseFixture()
                fixture.add(media.ISO, fixture.image)
                fixture.extra_assets.append(copy.deepcopy(fixture.assets[name]))
                self.assert_failed_atomically(fixture)
                self.assertEqual(fixture.requests, [TEST_API])

    def test_manifest_size_is_bounded_before_its_http_request(self):
        for size in (0, media.MAX_MANIFEST_SIZE + 1):
            with self.subTest(size=size):
                fixture = ReleaseFixture()
                fixture.assets[media.PARTS]["size"] = size
                self.assert_failed_atomically(fixture)
                self.assertEqual(fixture.requests, [TEST_API, fixture.url(media.SUMS)])

    def test_out_of_order_duplicate_and_gapped_parts_are_rejected(self):
        for change in (lambda parts: parts.reverse(),
                       lambda parts: parts.__setitem__(1, copy.deepcopy(parts[0])),
                       lambda parts: parts[1].__setitem__("name", media.ISO + ".part003")):
            with self.subTest(change=change):
                fixture = ReleaseFixture()
                change(fixture.manifest["parts"])
                fixture.update_manifest()
                self.assert_failed_atomically(fixture)

    def test_malformed_manifest_fields_sizes_hashes_and_paths_are_rejected(self):
        changes = [
            lambda value: value.clear(),
            lambda value: value.__setitem__("schemaVersion", True),
            lambda value: value.__setitem__("schemaVersion", 2),
            lambda value: value.__setitem__("filename", "../" + media.ISO),
            lambda value: value.__setitem__("size", True),
            lambda value: value.__setitem__("size", -1),
            lambda value: value.__setitem__("size", str(value["size"])),
            lambda value: value.__setitem__("size", value["size"] + 1),
            lambda value: value.__setitem__("sha256", "bad"),
            lambda value: value.__setitem__("parts", []),
            lambda value: value.__setitem__("parts", {}),
            lambda value: value.__setitem__("url", "https://example.invalid/iso"),
            lambda value: value["parts"][0].__setitem__("name", "../" + media.ISO + ".part001"),
            lambda value: value["parts"][0].__setitem__("name", "https://example.invalid/part001"),
            lambda value: value["parts"][0].__setitem__("url", "https://example.invalid/part001"),
            lambda value: value["parts"][0].__setitem__("size", True),
            lambda value: value["parts"][0].__setitem__("size", 0),
            lambda value: value["parts"][0].__setitem__("size", media.MAX_PART_SIZE + 1),
            lambda value: value["parts"][0].__setitem__("sha256", "z" * 64),
        ]
        for index, change in enumerate(changes):
            with self.subTest(case=index):
                fixture = ReleaseFixture()
                change(fixture.manifest)
                fixture.update_manifest()
                self.assert_failed_atomically(fixture)
        for value in ([], None, "manifest"):
            with self.subTest(value=value):
                fixture = ReleaseFixture()
                fixture.manifest = value
                fixture.update_manifest()
                self.assert_failed_atomically(fixture)

    def test_invalid_json_duplicate_fields_and_manifest_size_are_rejected(self):
        for data in (b"not JSON", b'{"schemaVersion":1,"schemaVersion":1}', b"null"):
            with self.subTest(data=data):
                fixture = ReleaseFixture()
                fixture.add(media.PARTS, data)
                self.assert_failed_atomically(fixture)
        self.fixture.assets[media.PARTS]["size"] += 1
        self.assert_failed_atomically()

    def test_manifest_digest_must_agree_with_sha256sums(self):
        self.fixture.manifest["sha256"] = "f" * 64
        self.fixture.update_manifest()
        self.assert_failed_atomically()
        self.assertEqual(len(self.fixture.requests), 3)

    def test_final_iso_digest_is_verified_even_when_all_parts_are_valid(self):
        wrong = "f" * 64
        self.fixture.expected = wrong
        self.fixture.manifest["sha256"] = wrong
        self.fixture.add(media.SUMS, f"{wrong}  {media.ISO}\n".encode("ascii"))
        self.fixture.update_manifest()
        self.assert_failed_atomically()
        self.assertEqual(len(self.fixture.requests), 3 + len(self.fixture.manifest["parts"]))

    def test_channel_checksum_mismatch_stops_before_downloading_image_parts(self):
        self.fixture.expected = "f" * 64
        self.assert_failed_atomically()
        self.assertEqual(self.fixture.requests, [TEST_API, self.fixture.url(media.SUMS)])

    def test_historical_channel_requires_plain_warning(self):
        channel = {**fixture_channel(self.fixture.expected), "qualification": "historical-recovery"}
        for invalid in (None, "", {}, 1):
            with self.subTest(warning=invalid), self.assertRaises(ValueError):
                media.validated_channel({**channel, "scannerWarning": invalid})

    def test_foreign_product_tag_is_refused(self):
        release = {"tag_name":"toolbox-123.1", "draft":False, "prerelease":False, "assets":[]}
        with mock.patch.object(media.urllib.request, "urlopen", return_value=io.BytesIO(json.dumps(release).encode())):
            with self.assertRaisesRegex(ValueError, "channel"):
                media.download_release(self.folder, fixture_channel(self.fixture.expected))
        self.assertFalse(list(self.folder.iterdir()))

    def test_part_asset_sizes_and_origins_must_match_the_release(self):
        for key, value in (("size", 1), ("size", True),
                           ("browser_download_url", "https://example.invalid/" + media.ISO + ".part001")):
            with self.subTest(key=key, value=value):
                fixture = ReleaseFixture()
                fixture.assets[fixture.manifest["parts"][0]["name"]][key] = value
                self.assert_failed_atomically(fixture)

    def test_missing_manifest_preserves_previous_files(self):
        del self.fixture.assets[media.PARTS]
        self.assert_failed_atomically()

    def test_part_connection_failure_closes_streams_and_removes_temporary_file(self):
        name = self.fixture.manifest["parts"][1]["name"]
        self.fixture.data[name] = OSError("connection interrupted")
        self.assert_failed_atomically(exception=OSError)

    def test_read_failure_closes_active_part_and_removes_temporary_file(self):
        class InterruptedResponse(TrackedResponse):
            def read(self, size=-1):
                if self.tell():
                    raise OSError("interrupted during body")
                return super().read(min(size, 3))
        name = self.fixture.manifest["parts"][0]["name"]
        self.fixture.response_factories[name] = InterruptedResponse
        self.assert_failed_atomically(exception=OSError)

    def test_failure_without_a_previous_iso_leaves_no_partial_files(self):
        name = self.fixture.manifest["parts"][0]["name"]
        self.fixture.data[name] = b"bad"
        with self.assertRaises(ValueError):
            self.download()
        self.assertEqual(list(self.folder.iterdir()), [])

    def test_space_is_checked_before_streaming_parts(self):
        with mock.patch.object(media.shutil, "disk_usage", return_value=SimpleNamespace(free=0)):
            self.assert_failed_atomically()
        self.assertEqual(len(self.fixture.requests), 3)

    def test_uppercase_manifest_digests_are_accepted(self):
        self.fixture.manifest["sha256"] = self.fixture.expected.upper()
        for part in self.fixture.manifest["parts"]:
            part["sha256"] = part["sha256"].upper()
        self.fixture.update_manifest()
        self.download()
        self.assertEqual((self.folder / media.ISO).read_bytes(), self.fixture.image)

    def test_real_file_proof_forces_parts_and_removes_its_output(self):
        for name, data in self.fixture.data.items():
            (self.folder / name).write_bytes(data)
        (self.folder / media.ISO).write_bytes(b"must not be used")
        result = verify_local_release_directory(self.folder, self.fixture.expected)
        self.assertEqual(result["mode"], "parts")
        self.assertEqual(result["sha256"], self.fixture.expected)
        self.assertEqual(result["size"], len(self.fixture.image))
        self.assertNotIn(media.ISO, result["requestedAssets"])
        self.assertTrue(result["temporaryOutputRemoved"])

    def test_real_file_proof_also_supports_ordinary_iso_releases(self):
        (self.folder / media.ISO).write_bytes(self.fixture.image)
        (self.folder / media.SUMS).write_bytes(self.fixture.data[media.SUMS])
        result = verify_local_release_directory(self.folder, self.fixture.expected)
        self.assertEqual(result["mode"], "iso")
        self.assertTrue(result["temporaryOutputRemoved"])

    def test_real_file_proof_requires_the_independent_original_digest(self):
        (self.folder / media.SUMS).write_bytes(self.fixture.data[media.SUMS])
        with self.assertRaises(ValueError):
            verify_local_release_directory(self.folder, "f" * 64)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--parts-dir", type=Path)
    parser.add_argument("--expected-sha256")
    parser.add_argument("--temporary-dir", type=Path)
    args, unittest_args = parser.parse_known_args()
    if args.parts_dir is not None:
        if not args.expected_sha256 or unittest_args:
            parser.error("--parts-dir requires --expected-sha256 and no unittest arguments")
        print(json.dumps(verify_local_release_directory(args.parts_dir, args.expected_sha256, args.temporary_dir), indent=2))
    else:
        if args.expected_sha256 or args.temporary_dir:
            parser.error("--expected-sha256 and --temporary-dir require --parts-dir")
        unittest.main(argv=[sys.argv[0], *unittest_args])
