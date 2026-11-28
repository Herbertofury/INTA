"""Offline publisher identity for INTA's native Tk applications.

Uses the same verified registry and PNGs as the portal. All resource paths are
relative to a source/portable root or the complete zipapp; no icon service is
contacted at runtime. Launch actions remain separate from publisher links.
"""
from __future__ import annotations

import base64
import json
import math
import os
from pathlib import Path, PurePosixPath
import re
import sys
import tkinter as tk
from tkinter import font as tkfont, ttk
import webbrowser
import weakref
import zipfile

BG = "#0d121a"
SURFACE = "#18212d"
TEXT = "#e7e9ee"
MUTED = "#a8afb9"
ACCENT = "#8bb8ff"
LINE = "#344355"
SENSITIVE_NAMES = {"everything", "autopsy", "chainsaw", "windows", "terminal"}


class BrandResources:
    """Read identical verified bytes from source, installed media, or a zipapp."""

    def __init__(self, script_path=None, resource_root=None):
        self.root = None
        self.archive = None
        self.archive_prefix = "inta-resources/"
        roots = []
        if resource_root:
            roots.append(Path(resource_root))
        if os.environ.get("INTA_RESOURCE_ROOT"):
            roots.append(Path(os.environ["INTA_RESOURCE_ROOT"]))
        for value in (script_path, __file__):
            if value:
                location = Path(value).resolve()
                for parent in (location.parent, *location.parents):
                    roots.extend((parent / "inta-resources", parent))
        roots.append(Path("/usr/share/inta"))
        archive = Path(sys.argv[0]).resolve()
        if not resource_root and not os.environ.get("INTA_RESOURCE_ROOT") and archive.is_file() and zipfile.is_zipfile(archive):
            with zipfile.ZipFile(archive) as bundle:
                if self.archive_prefix + "data/brand-icons.json" in bundle.namelist():
                    self.archive = archive
        if self.archive is None:
            for candidate in dict.fromkeys(roots):
                if (candidate / "data/brand-icons.json").is_file():
                    self.root = candidate
                    break
        if self.root is None and self.archive is None:
            raise RuntimeError("The INTA icon resources are missing. Open the complete portable .pyz or the source folder.")
        self.registry = json.loads(self.read("data/brand-icons.json"))
        self.assets = self.registry["assets"]
        self.names = {}
        for record in self.registry["brands"]:
            for name in record.get("names", [record["name"]]):
                self.names.setdefault(name.casefold(), (name, record))
        names = sorted(self.names, key=lambda name: (-len(name), name))
        self.pattern = re.compile(r"(?<![\w])(?:" + "|".join(re.escape(name) for name in names) + r")(?![\w])", re.IGNORECASE)

    def read(self, relative):
        path = PurePosixPath(relative)
        if path.is_absolute() or ".." in path.parts:
            raise ValueError("Resource path must remain inside the INTA package.")
        if self.root is not None:
            return (self.root / str(path)).read_bytes()
        with zipfile.ZipFile(self.archive) as bundle:
            return bundle.read(self.archive_prefix + str(path))

    def resolve(self, name):
        value = self.names.get(name.casefold())
        return value[1] if value else None

    def mentions(self, value):
        for match in self.pattern.finditer(value):
            spelling, record = self.names[match.group().casefold()]
            if match.group().casefold() in SENSITIVE_NAMES and match.group() != spelling:
                continue
            yield match.start(), match.end(), record

    def image_bytes(self, record):
        return self.read(self.assets[record["assetId"]]["path"])


def native_font(root, specification):
    """Keep the platform's actual UI family when changing a named font's size."""
    if isinstance(specification, (tuple, list)) and specification[0] in tkfont.names(root):
        family = tkfont.nametofont(specification[0], root=root).actual("family")
        return (family, *specification[1:])
    return specification


def configure_theme(root):
    root.configure(background=BG)
    style = ttk.Style(root)
    style.theme_use("clam")
    style.configure(".", background=BG, foreground=TEXT, bordercolor=LINE, lightcolor=LINE,
                    darkcolor=LINE, troughcolor=BG, focuscolor=ACCENT)
    style.configure("TFrame", background=BG)
    style.configure("Surface.TFrame", background=SURFACE)
    style.configure("TLabel", background=BG, foreground=TEXT, font=native_font(root, ("TkDefaultFont", 11)))
    style.configure("TButton", background=SURFACE, foreground=TEXT, bordercolor=LINE, lightcolor=SURFACE,
                    darkcolor=SURFACE, relief="flat", padding=(12, 9), font=native_font(root, ("TkDefaultFont", 10)))
    style.map("TButton", background=[("disabled", SURFACE), ("pressed", "#30435a"), ("active", "#243345")],
              bordercolor=[("focus", ACCENT), ("active", "#6686a8")], foreground=[("disabled", "#929aa6")])
    style.configure("TCombobox", fieldbackground=SURFACE, foreground=TEXT, arrowcolor=TEXT, bordercolor=LINE)
    style.map("TCombobox", fieldbackground=[("readonly", SURFACE)], foreground=[("readonly", TEXT)],
              bordercolor=[("focus", ACCENT)])
    style.configure("TNotebook", background=BG, borderwidth=0)
    style.configure("TNotebook.Tab", background=SURFACE, foreground=MUTED, lightcolor=SURFACE,
                    darkcolor=SURFACE, bordercolor=LINE, padding=(15, 11), font=native_font(root, ("TkDefaultFont", 11)))
    style.map("TNotebook.Tab", background=[("selected", "#26384c"), ("active", "#223042")],
              foreground=[("selected", "#c2e3ff")], bordercolor=[("focus", ACCENT)])
    style.configure("TSeparator", background=LINE)
    style.configure("Vertical.TScrollbar", background="#263344", troughcolor=BG, arrowcolor=MUTED,
                    lightcolor=SURFACE, darkcolor=SURFACE, bordercolor=BG)
    return style


class ScrollableFrame(ttk.Frame):
    """A scrollable native viewport retaining every child and action."""

    def __init__(self, parent, *, padding=0):
        super().__init__(parent)
        self.canvas = tk.Canvas(self, background=BG, borderwidth=0, highlightthickness=0, width=1, height=1)
        self.scrollbar = ttk.Scrollbar(self, orient="vertical", command=self.canvas.yview)
        self.canvas.configure(yscrollcommand=self.scrollbar.set)
        self.scrollbar.pack(side="right", fill="y")
        self.canvas.pack(side="left", fill="both", expand=True)
        self.content = ttk.Frame(self.canvas, padding=padding)
        self.content_id = self.canvas.create_window(0, 0, anchor="nw", window=self.content)
        self.canvas.bind("<Configure>", lambda event: self.canvas.itemconfigure(self.content_id, width=event.width))
        self.content.bind("<Configure>", lambda _event: self.canvas.configure(scrollregion=self.canvas.bbox("all")))
        root = self.winfo_toplevel()
        if not getattr(root, "inta_scroll_bindings", False):
            root.inta_scroll_bindings = True

            def containing(widget):
                while widget is not None:
                    if isinstance(widget, ScrollableFrame):
                        return widget
                    widget = getattr(widget, "master", None)
                return None

            def wheel(event):
                scroller = containing(event.widget)
                if scroller:
                    direction = -1 if getattr(event, "num", None) == 4 or getattr(event, "delta", 0) > 0 else 1
                    scroller.canvas.yview_scroll(direction * 3, "units")
                    return "break"

            def focus(event):
                scroller = containing(event.widget)
                if scroller:
                    scroller.reveal(event.widget)

            root.bind("<MouseWheel>", wheel, add="+")
            root.bind("<Button-4>", wheel, add="+")
            root.bind("<Button-5>", wheel, add="+")
            root.bind("<FocusIn>", focus, add="+")

    def reveal(self, widget, offset=0, height=None):
        if not widget.winfo_ismapped():
            return
        total = max(1, self.content.winfo_height())
        y = widget.winfo_rooty() - self.content.winfo_rooty() + offset
        height = height or min(widget.winfo_height(), self.canvas.winfo_height())
        top = self.canvas.canvasy(0)
        bottom = top + self.canvas.winfo_height()
        if y < top:
            self.canvas.yview_moveto(max(0, y - 8) / total)
        elif y + height > bottom:
            self.canvas.yview_moveto(max(0, y + height - self.canvas.winfo_height() + 8) / total)


class BrandUI:
    def __init__(self, root, script_path=None, resource_root=None):
        self.root = root
        self.resources = BrandResources(script_path, resource_root)
        self.images = {}
        self.widgets = weakref.WeakSet()
        configure_theme(root)
        self.window_icon(root)

    def photo(self, record, size=22):
        key = (record["assetId"], size)
        if key not in self.images:
            image = tk.PhotoImage(master=self.root, data=base64.b64encode(self.resources.image_bytes(record)))
            divisor = max(1, math.ceil(max(image.width(), image.height()) / size))
            if divisor > 1:
                image = image.subsample(divisor, divisor)
            self.images[key] = image
        return self.images[key]

    def window_icon(self, window, name="INTA"):
        record = self.resources.resolve(name)
        if record and record.get("assetId"):
            window.iconphoto(True, self.photo(record, 64))

    def open(self, record):
        url = record["downloadPage"]
        if not url.startswith("https://"):
            raise ValueError("Publisher links must use HTTPS.")
        return webbrowser.open(url, new=2)

    def text(self, parent, value, *, font=None, foreground=MUTED, background=BG, icon_size=22, **geometry):
        widget = BrandText(parent, self, value, font=native_font(self.root, font or ("TkDefaultFont", 11)), foreground=foreground, background=background, icon_size=icon_size)
        self.widgets.add(widget)
        if geometry:
            widget.pack(**geometry)
        return widget

    def dialog(self, title, message, choices, default=None):
        window = tk.Toplevel(self.root)
        window.title(title)
        window.transient(self.root)
        window.configure(background=BG)
        window.minsize(470, 210)
        screen_height = max(210, window.winfo_screenheight() - 80)
        window.geometry(f"590x{min(360, screen_height)}")
        self.window_icon(window)
        panel = ttk.Frame(window, padding=20)
        panel.pack(fill="both", expand=True)
        controls = ttk.Frame(panel)
        controls.pack(side="bottom", fill="x", pady=(16, 0))
        self.text(panel, title, font=("TkDefaultFont", 15, "bold"), foreground=TEXT, icon_size=26, fill="x", pady=(0, 12))
        message_area = ScrollableFrame(panel)
        message_area.pack(fill="both", expand=True)
        self.text(message_area.content, str(message), fill="x")
        result = [default]

        def choose(value):
            result[0] = value
            window.destroy()

        for label, value in choices:
            ttk.Button(controls, text=label, command=lambda value=value: choose(value)).pack(side="left", padx=(8, 0))
        window.protocol("WM_DELETE_WINDOW", lambda: choose(default))
        window.bind("<Escape>", lambda _event: choose(default))
        window.grab_set()
        window.wait_window()
        return result[0]

    def showerror(self, title, message):
        return self.dialog(title, message, [("Close", None)])

    def showinfo(self, title, message):
        return self.dialog(title, message, [("OK", None)])

    def askokcancel(self, title, message):
        return self.dialog(title, message, [("Continue", True), ("Cancel", False)], False)

    def askyesnocancel(self, title, message):
        return self.dialog(title, message, [("Choose executable", True), ("Official download page", False), ("Cancel", None)])


class BrandText(tk.Text):
    """Wrapping text with real inline images and keyboard-operable hosted links."""

    def __init__(self, parent, owner, value, *, font, foreground, background, icon_size):
        super().__init__(parent, height=1, width=1, wrap="word", borderwidth=0, highlightthickness=0,
                         relief="flat", background=background, foreground=foreground, font=font,
                         padx=0, pady=2, cursor="arrow", takefocus=False, exportselection=False)
        self.owner = owner
        self.links = []
        self.active_link = -1
        self.pending_fit = None
        self.last_width = None
        self.font_metrics = tkfont.Font(self, font=font)
        self.tag_configure("brand", foreground=ACCENT, underline=True)
        self.tag_configure("brand-icon", background="#f1f4f8")
        self.tag_configure("keyboard-brand", background="#263955")
        position = 0
        for start, end, record in owner.resources.mentions(value):
            self.insert("end", value[position:start])
            first = self.index("end-1c")
            if record.get("assetId"):
                self.image_create("end", image=owner.photo(record, icon_size), align="center", padx=2)
            icon_end = self.index("end-1c")
            self.insert("end", (" " if record.get("assetId") else "") + value[start:end])
            last = self.index("end-1c")
            tag = "brand-" + str(len(self.links))
            self.tag_add("brand", first, last)
            self.tag_add("brand-icon", first, icon_end)
            self.tag_add(tag, first, last)
            self.tag_bind(tag, "<Button-1>", lambda _event, record=record: owner.open(record))
            self.tag_bind(tag, "<Enter>", lambda _event: self.configure(cursor="hand2"))
            self.tag_bind(tag, "<Leave>", lambda _event: self.configure(cursor="arrow"))
            self.links.append({"start": first, "end": last, "record": record, "tag": tag})
            position = end
        self.insert("end", value[position:])
        self.configure(state="disabled", takefocus=bool(self.links))
        self.bind("<Configure>", self._schedule_fit)
        self.bind("<FocusIn>", lambda _event: self._focus_link(0))
        self.bind("<FocusOut>", lambda _event: self.tag_remove("keyboard-brand", "1.0", "end"))
        self.bind("<Destroy>", self._cancel_fit)
        self.bind("<Tab>", lambda _event: self._advance(1))
        self.bind("<Shift-Tab>", lambda _event: self._advance(-1))
        self.bind("<ISO_Left_Tab>", lambda _event: self._advance(-1))
        self.bind("<Return>", self._activate)
        self.bind("<space>", self._activate)
        self.pending_fit = self.after_idle(self._fit_height)

    def _schedule_fit(self, event):
        if event.width != self.last_width:
            self.last_width = event.width
            if self.pending_fit is not None:
                self.after_cancel(self.pending_fit)
            self.pending_fit = self.after_idle(self._fit_height)

    def _cancel_fit(self, _event=None):
        if self.pending_fit is not None:
            self.after_cancel(self.pending_fit)
            self.pending_fit = None

    def _fit_height(self):
        self.pending_fit = None
        if not self.winfo_exists() or self.winfo_width() <= 16:
            return
        # Text caches wrapped-line metrics. Force recalculation after a viewport
        # width change so an early 1px layout cannot leave enormous blank rows.
        pixels = self.count("1.0", "end", "update", "ypixels")
        if pixels:
            height = max(1, math.ceil(pixels / max(1, self.font_metrics.metrics("linespace"))))
            if int(self.cget("height")) != height:
                self.configure(height=height)

    def _focus_link(self, index):
        if not self.links:
            return
        self.active_link = min(max(index, 0), len(self.links) - 1)
        link = self.links[self.active_link]
        self.tag_remove("keyboard-brand", "1.0", "end")
        self.tag_add("keyboard-brand", link["start"], link["end"])
        parent = self.master
        while parent is not None:
            if isinstance(parent, ScrollableFrame):
                line = self.dlineinfo(link["start"])
                parent.reveal(self, offset=line[1] if line else 0, height=line[3] if line else None)
                break
            parent = getattr(parent, "master", None)

    def _advance(self, direction):
        next_index = self.active_link + direction
        if 0 <= next_index < len(self.links):
            self._focus_link(next_index)
        else:
            target = self.tk_focusNext() if direction > 0 else self.tk_focusPrev()
            if target:
                target.focus_set()
        return "break"

    def _activate(self, _event=None):
        if self.links:
            self.owner.open(self.links[max(0, self.active_link)]["record"])
        return "break"
