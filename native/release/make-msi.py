"""Build a per-user Windows Installer database using the Windows SDK MSI engine.
Build-only interpreter: 64-bit Python 3.8–3.12 with msilib. App has no msilib dependency.
"""

import sys
import uuid
import json
import ctypes
from ctypes import wintypes
from pathlib import Path
import msilib
from msilib import schema, sequence

payload = Path(sys.argv[1]).resolve()
output = Path(sys.argv[2]).resolve()
if output.exists():
    raise FileExistsError(output)


def guid(key):
    return (
        "{"
        + str(
            uuid.uuid5(
                uuid.NAMESPACE_URL,
                "https://github.com/Herbertofury/INTA/windows/" + key,
            )
        ).upper()
        + "}"
    )


def file_version(path):
    """Use the actual PE version so Windows Installer can cost a major upgrade."""
    if path.suffix.lower() not in (".exe", ".dll"):
        return None, None
    query = ctypes.WinDLL("msi").MsiGetFileVersionW
    query.argtypes = [
        wintypes.LPCWSTR,
        wintypes.LPWSTR,
        ctypes.POINTER(wintypes.DWORD),
        wintypes.LPWSTR,
        ctypes.POINTER(wintypes.DWORD),
    ]
    version, language = (
        ctypes.create_unicode_buffer(256),
        ctypes.create_unicode_buffer(256),
    )
    vlen, llen = wintypes.DWORD(len(version)), wintypes.DWORD(len(language))
    result = query(str(path), version, ctypes.byref(vlen), language, ctypes.byref(llen))
    if result in (
        1006,
        1812,
        1813,
    ):  # An unversioned Rust PE can lack version resources.
        return None, None
    if result:
        raise ctypes.WinError(result)
    return version.value or None, language.value or None


manifest = json.loads((payload / "base/manifest.json").read_bytes())
if not 0 < manifest["generation"] < 65536:
    raise ValueError("Release generation is out of MSI range")
version = "0.2." + str(manifest["generation"])
db = msilib.init_database(
    str(output),
    schema,
    "INTA Desktop",
    guid("product/" + manifest["sourceDigest"]),
    version,
    "INTA Computer Services",
)
si = db.GetSummaryInformation(5)
si.SetProperty(msilib.PID_WORDCOUNT, 10)
si.Persist()
order = {"FindRelatedProducts": 50, "RemoveExistingProducts": 1501}
for table in sequence.tables:
    rows = [
        (r[0], r[1], order.get(r[0], r[2]))
        for r in getattr(sequence, table)
        if r[0] not in ("ExitDialog", "FatalError", "UserExit")
    ]
    msilib.add_data(db, table, rows)
msilib.add_data(
    db,
    "Property",
    [
        ("UpgradeCode", guid("upgrade")),
        ("ARPNOMODIFY", "1"),
        (
            "ARPCOMMENTS",
            "Technician tools, repairs and local diagnostics",
        ),
        ("ARPURLINFOABOUT", "https://github.com/Herbertofury/INTA"),
    ],
)
msilib.add_data(db, "Property", [("SecureCustomProperties", "INTA_OLDER;INTA_NEWER")])
msilib.add_data(
    db,
    "Upgrade",
    [
        (guid("upgrade"), None, version, None, 512, None, "INTA_OLDER"),
        (guid("upgrade"), version, None, None, 2, None, "INTA_NEWER"),
    ],
)
msilib.add_data(
    db,
    "LaunchCondition",
    [
        ("VersionNT64 OR Installed", "A 64-bit Windows installation is required."),
        (
            "NOT INTA_NEWER OR Installed",
            "A newer INTA Desktop release is installed.",
        ),
    ],
)
cab = msilib.CAB("inta.cab")


def directory(parent, physical, key, name):
    return msilib.Directory(db, cab, parent, str(physical), key, name, componentflags=0)


target = directory(None, payload, "TARGETDIR", "SourceDir")
local = directory(target, ".", "LocalAppDataFolder", ".")
programs = directory(local, ".", "UserPrograms", "Programs")
app = directory(programs, ".", "INSTALLDIR", "INTADESK|INTA Desktop")
start = directory(target, ".", "ProgramMenuFolder", ".")
startup = directory(target, ".", "StartupFolder", ".")
feature = msilib.Feature(
    db, "Workstation", "INTA Desktop", "Technician desktop and CLI", 1
)
dirs = {payload: app}
for i, p in enumerate(sorted(payload.rglob("*"))):
    parent = dirs[p.parent]
    if p.is_dir():
        dirs[p] = directory(
            parent, p.name, "D" + str(i), parent.make_short(p.name) + "|" + p.name
        )
        continue
    rel = p.relative_to(payload).as_posix()
    key = "C" + str(i)
    parent.start_component(key, feature, keyfile=p.name, uuid=guid("desktop-file/" + rel))
    pe_version, language = file_version(p)
    fileid = parent.add_file(p.name, version=pe_version, language=language)
    if rel == "INTA Desktop.exe":
        protocol = r"Software\Classes\inta-tool"
        msilib.add_data(
            db,
            "Registry",
            [
                ("ToolProtocol", 1, protocol, None, "URL:INTA tool action", key),
                ("ToolProtocolMarker", 1, protocol, "URL Protocol", "", key),
                (
                    "ToolProtocolIcon",
                    1,
                    protocol + r"\DefaultIcon",
                    None,
                    f'"[#{fileid}]",0',
                    key,
                ),
                (
                    "ToolProtocolOpen",
                    1,
                    protocol + r"\shell\open\command",
                    None,
                    f'"[#{fileid}]" --tool-action "%1"',
                    key,
                ),
            ],
        )
        msilib.add_data(
            db,
            "Shortcut",
            [
                (
                    "StartWorkstation",
                    "ProgramMenuFolder",
                    "INTADESK|INTA Desktop",
                    key,
                    "[#" + fileid + "]",
                    None,
                    "Open INTA Desktop",
                    None,
                    None,
                    None,
                    None,
                    "INSTALLDIR",
                )
            ],
        )
# Only engine-owned files are removed by Windows Installer. Private state and user toolbox are external.
msilib.add_data(
    db,
    "RemoveFile",
    [
        (
            "DevStartupCleanup",
            key,
            "INTADESC.LNK|INTA Desktop Development Sync.lnk",
            "StartupFolder",
            2,
        )
    ],
)
cab.commit(db)
db.Commit()
print(output)
