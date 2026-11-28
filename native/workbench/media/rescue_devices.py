"""Read-only discovery of existing Ventoy USB data partitions."""
import json
import os
from contextlib import contextmanager
from pathlib import Path
import subprocess
import sys


def windows_volumes():
    script = """
$ErrorActionPreference = 'Stop'
[Console]::OutputEncoding = [System.Text.UTF8Encoding]::new($false)
$rows = @(foreach ($d in Get-Disk) {
    if ($d.BusType -ne 'USB' -or $d.IsBoot -or $d.IsSystem -or $d.IsOffline -or $d.IsReadOnly -or !$d.UniqueId) {continue}
    $parts = @(Get-Partition -DiskNumber $d.Number)
    $efi = $parts | Where-Object PartitionNumber -eq 2 | Get-Volume -ErrorAction SilentlyContinue
    if ($efi.FileSystemLabel -ne 'VTOYEFI' -or $efi.FileSystem -notlike 'FAT*') {continue}
    $p = $parts | Where-Object PartitionNumber -eq 1
    if (!$p.DriveLetter -or $p.IsReadOnly -or $p.IsBoot -or $p.IsSystem) {continue}
    $v = $p | Get-Volume
    if ($v.FileSystem -notin 'NTFS','exFAT','FAT32' -or !$v.UniqueId -or !$v.Path) {continue}
    [pscustomobject]@{root=("$($p.DriveLetter):" + [char]92); id=("$($d.UniqueId)|$($v.UniqueId)|$($p.Offset)");
        model=$d.FriendlyName; capacity=[long]$d.Size; free=[long]$v.SizeRemaining; filesystem=$v.FileSystem; bus='USB';
        partition=1; disk=[string]$d.Number; label=$v.FileSystemLabel; volumePath=$v.Path}
})
ConvertTo-Json -InputObject $rows -Compress -Depth 3
"""
    # Resolve the system executable outside the process search path.
    executable = Path(os.environ["SystemRoot"]) / "System32/WindowsPowerShell/v1.0/powershell.exe"
    output = subprocess.run([str(executable), "-NoLogo", "-NoProfile", "-NonInteractive",
                             "-Command", script], capture_output=True, check=True, timeout=40)
    return json.loads(output.stdout.decode("utf-8-sig"))


def linux_volumes():
    columns = "PATH,TYPE,SIZE,MODEL,SERIAL,WWN,TRAN,RO,FSTYPE,LABEL,UUID,MOUNTPOINTS,PARTN"
    output = subprocess.run(["lsblk", "--bytes", "--json", "--tree", "--output", columns],
                            check=True, capture_output=True, timeout=12)
    rows = []
    for disk in json.loads(output.stdout)["blockdevices"]:
        if disk["type"] != "disk" or disk["tran"] != "usb" or disk["ro"]:
            continue
        parts = {str(p.get("partn")): p for p in disk.get("children", [])}
        data, efi = parts.get("1", {}), parts.get("2", {})
        ident = disk["wwn"] or disk["serial"]
        if not ident or not data.get("uuid") or data.get("ro") or data.get("children"):
            continue
        if efi.get("label") != "VTOYEFI" or efi.get("fstype") != "vfat":
            continue
        if data.get("fstype") not in ("exfat", "ntfs", "vfat", "ext4", "xfs"):
            continue
        mounts = [m for m in data.get("mountpoints", []) if m]
        if len(mounts) != 1 or mounts[0] == "/" or not os.path.ismount(mounts[0]):
            continue
        # Refuse an entire disk if any descendant carries the running system.
        def active_system(node):
            return (any(m in ("/", "/boot", "/boot/efi", "/usr", "/home", "[SWAP]") for m in node.get("mountpoints", []))
                    or any(active_system(child) for child in node.get("children", [])))
        if active_system(disk):
            continue
        info = os.statvfs(mounts[0])
        if info.f_flag & os.ST_RDONLY:
            continue
        rows.append(dict(root=mounts[0], id=f"{ident}|{data['uuid']}", model=disk["model"],
                         capacity=disk["size"], free=info.f_bavail * info.f_frsize,
                         filesystem=data["fstype"], bus="USB", partition=1, disk=disk["path"], label=data["label"]))
    return rows


def candidates():
    if os.name == "nt":
        return windows_volumes()
    if sys.platform.startswith("linux"):
        return linux_volumes()
    raise ValueError("Ventoy volume discovery requires Windows or Linux.")


def selected_volume(path, expected_id=None):
    matches = [row for row in candidates() if Path(row["root"]) == path.absolute()]
    if len(matches) != 1 or (expected_id is not None and matches[0]["id"] != expected_id):
        raise ValueError("Choose the mounted first partition of the same USB disk with a VTOYEFI second partition.")
    return matches[0]


@contextmanager
def writable_root(volume, device):
    # The Windows volume GUID continues to identify the same filesystem if a
    # drive letter is reassigned. An open directory pins the mount on Linux.
    if os.name == "nt":
        yield Path(device["volumePath"])
    else:
        descriptor = os.open(volume, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
        try:
            yield Path(f"/proc/self/fd/{descriptor}")
        finally:
            os.close(descriptor)
