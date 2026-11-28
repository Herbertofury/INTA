import hashlib
import json
from pathlib import Path
import runpy
import subprocess
import sys
import tempfile
import unittest

ROOT=Path(__file__).resolve().parents[1]
CHECK=runpy.run_path(str(ROOT/"scripts/check-recovery-image.py"))
VOLUMES=runpy.run_path(str(ROOT/"scripts/recovery-source-volumes.py"))


class RecoveryOnly(unittest.TestCase):
    def test_edition_uses_reviewed_files_and_distinct_launcher(self):
        product=runpy.run_path(str(ROOT/"recovery/inta-rescue"))
        controls=[row for actions in product["SECTIONS"].values() for row in actions]
        self.assertEqual(product["TITLE"],"INTA Rescue Recovery")
        self.assertEqual(list(product["SECTIONS"]),["Inspect","Recover files","Copy and repair"])
        self.assertEqual({row[2] for row in controls},{"List devices","Inspect health","Open TestDisk","Open PhotoRec",
            "Open files","Open Clonezilla","Open editor","Show usage"})
        tools=set((ROOT/"recovery/packages.list").read_text().split())
        self.assertTrue({"gddrescue","clonezilla","gparted","testdisk","smartmontools"}<=tools)
        self.assertFalse(any(CHECK["DENIED_PACKAGES"].fullmatch(package) for package in tools))
        self.assertIn("clamav",(ROOT/"live/package-lists/inta.list.chroot").read_text().split())
        self.assertIn("check-rescue-image-scanner.py",(ROOT/".github/workflows/rescue.yml").read_text())

    def test_package_gate_rejects_scanner_packages_sources_residues_and_partial_records(self):
        def record(name,source=None,state="install ok installed"):
            return f"Package: {name}\nStatus: {state}\nArchitecture: amd64\nVersion: 1.0\nConffiles:\n /example digest\n"+("Source: "+source+"\n" if source else "")
        clean="\n".join(record(p) for p in CHECK["REQUIRED"])
        self.assertEqual(len(CHECK["packages"](clean)),len(CHECK["REQUIRED"]))
        for text in (record("clamav"),record("libclamav13"),record("innocent","clamav (9.0)"),
                    record("libfreshclam3",state="deinstall ok config-files"),record("yara"),record("nodejs"),
                    record("libyara10"),record("innocent",state="install reinstreq half-installed"),
                    record("testdisk"),"Package: repeated\nPackage: repeated\n"):
            with self.subTest(text=text),self.assertRaises(ValueError):
                CHECK["packages"](clean+"\n"+text)

    def test_binary_inventory_rejects_external_packages_and_manual_copies(self):
        for path in ("/usr/bin/clamscan","/opt/manual/clamscan","/usr/lib/aarch64-linux-gnu/libclamav.so.99",
            "/run/arbitrary.deb","/var/cache/sample.cvd","/usr/bin/yara","/opt/inta-ai/node_modules/.bin/claude",
            "/usr/lib/systemd/system/clamav-freshclam.service","/usr/local/lib/node_modules/@anthropic-ai/tool",
            "EFI/BOOT/clamd","usr/sbin/clamav-milter","var/lib/dpkg/info/libclamav12:amd64.list"):
            with self.subTest(path=path),self.assertRaises(ValueError):
                CHECK["paths_without_payload"]([path])
        CHECK["paths_without_payload"](["usr/lib/python3.13/tkinter/ttk.py","usr/bin/lsblk","etc/xdg/autostart/inta-rescue.desktop"])

    def test_source_installs_only_verified_product_icons(self):
        with tempfile.TemporaryDirectory() as temp:
            target=Path(temp)
            subprocess.run([sys.executable,str(ROOT/"recovery/select-brand-assets.py"),str(ROOT),str(target)],check=True)
            registry=json.loads((target/"data/brand-icons.json").read_text())
            self.assertEqual({r["id"] for r in registry["brands"]},{"app-inta","app-inta-rescue"})
            self.assertEqual(len(registry["assets"]),1)
            for asset in registry["assets"].values():
                self.assertEqual(hashlib.sha256((target/asset["path"]).read_bytes()).hexdigest(),asset["sha256"])

    def test_independent_source_archive_rejects_incomplete_or_altered_blobs(self):
        with tempfile.TemporaryDirectory() as temp:
            root=Path(temp)
            cache=root/"cache"
            blob=b"synthetic original"
            sha=hashlib.sha256(blob).hexdigest()
            dsc=f"Format: 3.0 (native)\nChecksums-Sha256:\n {sha} {len(blob)} fixture_1.tar.xz\n".encode()
            files=[]
            for name,data in [("fixture_1.tar.xz",blob),("fixture_1.dsc",dsc)]:
                h=hashlib.sha256(data).hexdigest()
                path=VOLUMES["cached"](cache,h)
                path.parent.mkdir(parents=True,exist_ok=True)
                path.write_bytes(data)
                item={"name":name,"sha256":h,"bytes":len(data),"url":"https://deb.debian.org/debian/pool/main/f/fixture/"+name}
                VOLUMES["acquire"](item,cache)
                files.append(item)
            document={"sourcePackages":[{"source":"fixture","version":"1","files":files}]}
            index=root/"index.json"
            index.write_text(json.dumps(document))
            groups=VOLUMES["plans"](document)
            VOLUMES["verify_dsc"](groups["fixture/1"],cache)
            out=root/"volumes"
            VOLUMES["write_volumes"](groups,cache,out,index)
            result=VOLUMES["verify_volumes"](index,out)
            self.assertEqual(result["sourceFiles"],2)
            archive=out/result["volumes"][0]["name"]
            content=archive.read_bytes()
            self.assertIn(blob,content)
            archive.write_bytes(content.replace(blob,b"X"*len(blob),1))
            with self.assertRaises(ValueError):
                VOLUMES["verify_volumes"](index,out)
            VOLUMES["cached"](cache,sha).write_bytes(b"changed")
            with self.assertRaises(ValueError):
                VOLUMES["acquire"](files[0],cache)
            with self.assertRaises(ValueError):
                VOLUMES["plans"]({"sourcePackages":[{**document["sourcePackages"][0],"files":files[:1]}]})


if __name__=="__main__":
    unittest.main()
