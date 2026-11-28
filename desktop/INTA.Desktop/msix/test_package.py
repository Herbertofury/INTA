import base64
import hashlib
import io
import unittest
from xml.etree import ElementTree as ET
from zipfile import ZipFile

import package


class InstallerMetadataTests(unittest.TestCase):
    def test_blockmap_covers_every_byte_and_member(self):
        payload = b"a" * 65536 + b"tail"
        ns = "{http://schemas.microsoft.com/appx/2010/blockmap}"
        root = ET.Element(ns + "BlockMap", HashMethod="http://www.w3.org/2001/04/xmlenc#sha256")
        item = ET.SubElement(root, ns + "File", Name="app.exe", Size=str(len(payload)))
        for chunk in [payload[:65536], payload[65536:]]:
            ET.SubElement(item, ns + "Block", Hash=base64.b64encode(hashlib.sha256(chunk).digest()).decode())
        def check(content):
            stream = io.BytesIO()
            with ZipFile(stream, "w") as bundle:
                bundle.writestr("AppxBlockMap.xml", ET.tostring(root))
                bundle.writestr("app.exe", content)
            with ZipFile(stream) as bundle:
                package.verify_blocks(bundle, {"app.exe"})
        check(payload)
        with self.assertRaisesRegex(ValueError, "checksum"):
            check(payload[:-1] + b"s")
        item.remove(item[-1])
        with self.assertRaisesRegex(ValueError, "omits"):
            check(payload)

    def test_content_type_override_cannot_reclassify_the_entry_point(self):
        root = ET.Element("Types", xmlns="http://schemas.openxmlformats.org/package/2006/content-types")
        for ext, kind in (("png", "image/png"), ("exe", "application/x-msdownload"), ("xml", "application/vnd.ms-appx.manifest+xml")):
            ET.SubElement(root, "Default", Extension=ext, ContentType=kind)
        ET.SubElement(root, "Override", PartName="/AppxBlockMap.xml", ContentType="application/vnd.ms-appx.blockmap+xml")
        def check():
            stream = io.BytesIO()
            with ZipFile(stream, "w") as bundle:
                bundle.writestr("[Content_Types].xml", ET.tostring(root))
            with ZipFile(stream) as bundle:
                package.verify_content_types(bundle, False)
        check()
        ET.SubElement(root, "Override", PartName="/INTA.Desktop.exe", ContentType="text/plain")
        with self.assertRaisesRegex(ValueError, "Incorrect installer content type"):
            check()
