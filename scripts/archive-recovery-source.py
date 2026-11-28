"""Archive only the immutable inputs consumed by the recovery product build."""
import argparse
from pathlib import Path
import re
import subprocess

BUILD_INPUTS=("recovery","live/includes.chroot/etc/lightdm","live/includes.chroot/etc/systemd/system/inta-boot-smoke.service",
    "live/includes.chroot/etc/xdg/autostart/inta-desktop-appearance.desktop","live/includes.chroot/etc/xdg/autostart/thunar-volman.desktop",
    "live/includes.chroot/etc/xdg/xfce4","live/includes.chroot/usr/local/bin/inta-boot-smoke","live/includes.chroot/usr/local/bin/inta-desktop-appearance",
    "native/optics","media/inta_optics_ui.py","media/inta_brand_ui.py","assets/brand-icons/70ad9f0981b05341142b.png",
    "data/brand-icons.json","docs/assets/inta-mark.svg","scripts/build-recovery-iso","scripts/build-rescue-wallpaper.py")


if __name__=="__main__":
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument("source_revision")
    p.add_argument("new_archive",type=Path)
    a=p.parse_args()
    if not re.fullmatch(r"[0-9a-f]{40}",a.source_revision):
        raise ValueError("Select an exact source commit")
    with a.new_archive.open("xb") as target:
        subprocess.run(["git","archive","--format=tar.gz","--prefix=inta-recovery/",a.source_revision,"--",*BUILD_INPUTS],
                       cwd=Path(__file__).resolve().parents[1],stdout=target,check=True)
