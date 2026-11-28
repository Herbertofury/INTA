#!/usr/bin/env python3
"""Verify the Recovery edition's real GUI in a read-only, isolated QEMU guest."""
from pathlib import Path
import runpy
import sys

TABS = ["Inspect", "Recover files", "Copy and repair"]
BUTTONS = {"List devices","Inspect health","Open TestDisk","Open PhotoRec",
           "Open files","Open Clonezilla","Open editor","Show usage"}


def guest():
    import tempfile
    from tkinter import ttk
    root = runpy.run_path("/usr/local/bin/inta-rescue")["build_app"]()
    root.lift()
    root.update()
    assert root.title() == "INTA Rescue Recovery"
    tabs = next(w for w in root.winfo_children() if isinstance(w,ttk.Notebook))
    assert [tabs.tab(item,"text") for item in tabs.tabs()] == TABS
    def descend(widget):
        for child in widget.winfo_children():
            yield child
            yield from descend(child)
    buttons = {w.cget("text") for w in descend(tabs) if isinstance(w,ttk.Button)}
    assert buttons == BUTTONS
    for tab in tabs.tabs():
        tabs.select(tab)
        root.update()
        assert tabs.select() == tab
    tabs.select(1)
    header=root.inta_optics
    assert header.native is not None
    with tempfile.TemporaryDirectory() as settings:
        header.settings=Path(settings)/"profile"
        for name in ("Full Prism","Classic"):
            header.profile.set(name)
            header.choose()
            root.update()
            assert bool(header.cells) == (name=="Full Prism")
            assert header.settings.read_text().strip() == name
            print("\nINTA_OPTICAL_VISIBLE_"+name.split()[0].upper(),flush=True)
            root.after(2200,root.quit)
            root.mainloop()
        header.profile.set("Professional")
        header.choose()
        root.update()
        header.canvas.event_generate("<ButtonPress-1>",x=20,y=15)
        root.update()
        assert header.pending is not None and header.impulses
        root.after(1750,root.quit)
        root.mainloop()
        assert header.pending is None and not header.impulses
    root.destroy()
    print("INTA_RECOVERY_GUI_PASS",flush=True)
    print("INTA_OPTICAL_GUEST_PASS",flush=True)


if __name__=="__main__":
    if sys.argv[1:]==["--guest"]:
        guest()
    else:
        import argparse
        parser=argparse.ArgumentParser(description=__doc__)
        parser.add_argument("iso",type=Path)
        parser.add_argument("sha256")
        parser.add_argument("new_evidence_directory",type=Path)
        args=parser.parse_args()
        shared = runpy.run_path(str(Path(__file__).with_name("smoke-rescue-optics.py")))
        shared["host"](args.iso,args.sha256,args.new_evidence_directory.absolute(),guest_script=Path(__file__),verified_tabs=TABS)
