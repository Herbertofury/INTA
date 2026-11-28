import hashlib
import importlib.util
from pathlib import Path
import struct
import tempfile
import unittest


spec = importlib.util.spec_from_file_location("verify_preview", Path(__file__).with_name("verify-preview.py"))
preview = importlib.util.module_from_spec(spec)
spec.loader.exec_module(preview)


class PreviewPackageTests(unittest.TestCase):
    def setUp(self):
        temp = tempfile.TemporaryDirectory()
        self.addCleanup(temp.cleanup)
        self.directory = Path(temp.name)
        self.exe = self.directory / preview.EXE
        self.sums = self.directory / preview.SUMS
        self.data = bytearray(512)
        self.data[:2] = b"MZ"
        struct.pack_into("<I", self.data, 60, 128)
        struct.pack_into("<4sHHIIIHH", self.data, 128, b"PE\0\0", 0x8664, 1, 0, 0, 0, 240, 0x22)
        struct.pack_into("<H", self.data, 152, 0x20B)
        struct.pack_into("<H", self.data, 220, 2)
        self.pin = self.write_package(self.data)

    def write_package(self, data):
        self.exe.write_bytes(data)
        pin = hashlib.sha256(data).hexdigest()
        self.sums.write_bytes(f"{pin}  {preview.EXE}\n".encode("ascii"))
        return pin

    def test_matching_gui_pe_and_exact_inventory(self):
        self.assertEqual(preview.verify(self.directory, self.pin.upper()),
                         {"file": preview.EXE, "bytes": len(self.data), "sha256": self.pin, "target": "win-x64"})
        (self.directory / "companion.dll").write_bytes(b"unused")
        with self.assertRaisesRegex(ValueError, "Expected exactly"):
            preview.verify(self.directory, self.pin)

    def test_tampered_exe_and_replacement_receipt(self):
        self.exe.write_bytes(self.data + b"modified")
        with self.assertRaisesRegex(ValueError, "Downloaded executable"):
            preview.verify(self.directory, self.pin)
        self.write_package(self.data + b"modified")
        with self.assertRaisesRegex(ValueError, "receipt"):
            preview.verify(self.directory, self.pin)

    def test_bad_headers_still_rejected_with_matching_digest(self):
        for offset, value in [(0, b"ZZ"), (60, b"\xff" * 4), (128, b"XE"),
                              (132, b"\x64\xaa"), (148, b"\xff\xff"),
                              (150, b"\x22\x20"), (152, b"\x0b\x01"), (220, b"\x03\0")]:
            with self.subTest(offset=offset):
                image = self.data.copy()
                image[offset:offset + len(value)] = value
                pin = self.write_package(image)
                with self.assertRaises(ValueError):
                    preview.verify(self.directory, pin)

    def test_manifest_cannot_redirect_or_hide_a_second_path(self):
        for receipt in (f"{self.pin}  ../outside.exe\n", f"{self.pin}  {preview.EXE}\n\nextra"):
            with self.subTest(receipt=receipt):
                self.sums.write_bytes(receipt.encode("ascii"))
                with self.assertRaises(ValueError):
                    preview.verify(self.directory, self.pin)
