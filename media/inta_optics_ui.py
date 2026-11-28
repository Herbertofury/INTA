"""Offline native optical field for the Rescue launcher and portable Media Creator."""
import ctypes
import math
import os
from pathlib import Path
import sys
import tempfile
import time
import tkinter as tk
from tkinter import ttk
from xml.etree import ElementTree

PROFILES = ("Full Prism", "Professional", "Deep Focus", "Silent Eco", "Classic")
SETTLE_MS = 1450
BASE = "#0b1019"


def native_sampler(library=None):
    if library is None:
        library = (Path(__file__).resolve().with_name("inta_optics.dll") if os.name == "nt"
                   else Path("/usr/lib/inta/libinta_optics.so"))
    try:
        sample = ctypes.CDLL(str(library)).inta_optics_cell
        sample.argtypes = [ctypes.c_uint32] * 5 + [ctypes.c_uint64] * 2
        sample.restype = ctypes.c_uint32
        return sample
    except (AttributeError, OSError):
        return None


def software_cell(x, y, cols, rows, profile, newest, previous):
    """Small native-Tk compatibility field for a standalone cross-platform zipapp."""
    if profile >= 4:
        return 0
    seed = (x * 0x9e3779b1 ^ y * 0x85ebca77) & 0xffffffff
    seed = ((seed ^ seed >> 16) * 0x7feb352d) & 0xffffffff
    seed ^= seed >> 15
    glow = 0
    if profile < 2:
        for value, weight in ((newest, 1), (previous, .5)):
            if not value:
                continue
            age = (value >> 32) & 65535
            px = (value & 65535) / 65535 * cols
            py = ((value >> 16) & 65535) / 65535 * rows
            distance = math.hypot((x + .5 - px) * 16, (y + .5 - py) * 20)
            delta = (distance - age / 4) / 30
            glow += max(0, 1 - abs(delta)) ** 2 * (1 - age / SETTLE_MS) * (value >> 48) * weight
    if (seed & 255) + glow / 2 < (82, 102, 146, 211)[profile]:
        return 0
    light = (seed >> 27) + int(glow / 2)
    haze = max(0, 1 - abs(x / cols - .72 + (y / rows - .5) / 3) * 2.7)
    gain = (1, .73, .58, .46)[profile]
    rgb = [min(196, int(base + (light + haze * tint) * gain))
           for base, tint in ((16, 11), (22, 32), (31, 48))]
    glyph = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"[((seed >> 9) + int(glow / 45)) % 36]
    return ord(glyph) << 24 | rgb[0] << 16 | rgb[1] << 8 | rgb[2]


def system_accessibility():
    contrast = "highcontrast" in os.environ.get("GTK_THEME", "").replace("-", "").lower()
    reduced = os.environ.get("GTK_ANIMATION_DURATION_FACTOR") == "0"
    if sys.platform.startswith("linux"):
        base = Path(os.environ.get("XDG_CONFIG_HOME", Path.home() / ".config"))
        try:
            channel = ElementTree.parse(base / "xfce4/xfconf/xfce-perchannel-xml/xsettings.xml")
            values = {p.get("name"): p.get("value", "") for p in channel.iter("property")}
            contrast |= "highcontrast" in values.get("ThemeName", "").replace("-", "").lower()
            reduced |= values.get("EnableAnimations", "true").lower() in ("false", "0")
        except (OSError, ElementTree.ParseError):
            pass
    if os.name == "nt":
        class HighContrast(ctypes.Structure):
            _fields_ = [("size", ctypes.c_uint), ("flags", ctypes.c_uint), ("scheme", ctypes.c_void_p)]
        high = HighContrast(ctypes.sizeof(HighContrast), 0, None)
        motion = ctypes.c_int(1)
        query = ctypes.windll.user32.SystemParametersInfoW
        if query(0x42, high.size, ctypes.byref(high), 0):
            contrast = bool(high.flags & 1)
        if query(0x1042, 0, ctypes.byref(motion), 0):
            reduced = not bool(motion.value)
    return contrast or os.environ.get("INTA_HIGH_CONTRAST") == "1", reduced or os.environ.get("INTA_REDUCED_MOTION") == "1"


class OpticalHeader(ttk.Frame):
    """Local, event-driven rendering. All controls and links are ordinary Tk widgets."""
    def __init__(self, parent, *, library=None, settings=None):
        super().__init__(parent)
        root = self.winfo_toplevel()
        config = os.environ.get("APPDATA" if os.name == "nt" else "XDG_CONFIG_HOME")
        self.settings = settings or (Path(config) if config else Path.home() / ".config") / "INTA" / "appearance-profile"
        try:
            with self.settings.open(encoding="ascii") as stream:
                selected = stream.read(32).strip()
        except (OSError, UnicodeError):
            selected = ""
        self.profile = tk.StringVar(self, selected if selected in PROFILES else PROFILES[0])
        self.high_contrast, self.reduced = system_accessibility()
        self.native = native_sampler(library)
        self.sample = self.native or software_cell
        self.canvas = tk.Canvas(self, width=140, height=62, highlightthickness=0, borderwidth=0,
                                background="#000000" if self.high_contrast else BASE, takefocus=False)
        self.canvas.pack(fill="both", expand=True)
        controls = ttk.Frame(self)
        controls.pack(anchor="e")
        ttk.Label(controls, text="Appearance").pack(side="left", padx=(0, 6))
        picker = ttk.Combobox(controls, textvariable=self.profile, values=PROFILES, state="readonly", width=13)
        picker.pack(side="left", pady=(3, 0))
        picker.bind("<<ComboboxSelected>>", self.choose)
        self.pitch = max(10, round(10 * float(self.tk.call("tk", "scaling"))))
        self.cells = []
        self.grid_size = None
        self.impulses = []
        self.pending = None
        self.canvas.bind("<Configure>", self.queue)
        self.canvas.bind("<Map>", self.queue)
        self.canvas.bind("<Unmap>", self.sleep)
        self.canvas.bind("<Destroy>", self.sleep)
        self._bindings = [(event, root.bind(event, self.interact, add="+"))
                          for event in ("<Motion>", "<ButtonPress-1>")]
        def unbind(event):
            if event.widget is self:
                for sequence, callback in self._bindings:
                    root.unbind(sequence, callback)
        self.bind("<Destroy>", unbind)

    def choose(self, _event=None):
        self.impulses.clear()
        try:
            self.settings.parent.mkdir(parents=True, exist_ok=True)
            temporary = tempfile.NamedTemporaryFile("w", encoding="ascii", dir=self.settings.parent, delete=False)
            path = Path(temporary.name)
            try:
                with temporary:
                    temporary.write(self.profile.get() + "\n")
                os.replace(path, self.settings)
            finally:
                path.unlink(missing_ok=True)
        except OSError:
            pass  # Read-only live homes still apply the selected appearance to this session.
        self.queue()

    def interact(self, event):
        if self.high_contrast or self.reduced or self.profile.get() not in PROFILES[:2] or not self.canvas.winfo_ismapped():
            return
        now = time.monotonic()
        press = event.type == tk.EventType.ButtonPress
        if not press and self.impulses and now - self.impulses[0][0] < .08:
            return
        target = self.canvas if event.widget is self.canvas else self.winfo_toplevel()
        x = max(0, min(65535, (event.x_root - target.winfo_rootx()) * 65535 // max(1, target.winfo_width())))
        y = max(0, min(65535, (event.y_root - target.winfo_rooty()) * 65535 // max(1, target.winfo_height())))
        self.impulses = [(now, x | y << 16 | (255 if press else 138) << 48), *self.impulses[:1]]
        self.queue()

    def sleep(self, _event=None):
        if self.pending is not None:
            self.after_cancel(self.pending)
            self.pending = None
        self.impulses.clear()

    def queue(self, _event=None):
        if self.pending is None and self.canvas.winfo_ismapped():
            self.pending = self.after_idle(self.draw)

    def draw(self):
        self.pending = None
        if not self.canvas.winfo_ismapped():
            return
        p = PROFILES.index(self.profile.get())
        now = time.monotonic()
        self.impulses = [(start, bits) for start, bits in self.impulses if (now-start)*1000 < SETTLE_MS]
        impulses = [bits | (int((now-start)*1000) << 32) for start,bits in self.impulses]
        newest, previous = (impulses + [0, 0])[:2]
        cols = math.ceil(self.canvas.winfo_width() / self.pitch)
        height = round(self.pitch * 1.25)
        rows = math.ceil(self.canvas.winfo_height() / height)
        dimensions = (cols, rows) if p < 4 and not self.high_contrast else (0, 0)
        if self.grid_size != dimensions:
            self.canvas.delete("all")
            self.cells = [[self.canvas.create_text((x + .5)*self.pitch, (y + .5)*height,
                           font=("TkFixedFont", max(6, round(self.pitch*.49)))), None]
                          for y in range(dimensions[1]) for x in range(dimensions[0])]
            self.grid_size = dimensions
        for i, cell in enumerate(self.cells):
            value = self.sample(i % cols, i // cols, cols, rows, p, newest, previous)
            if cell[1] != value:
                self.canvas.itemconfigure(cell[0], text=chr(value >> 24) if value else "",
                                          fill=f"#{value & 0xffffff:06x}")
                cell[1] = value
        if self.impulses and not self.reduced and p < 2 and not self.high_contrast:
            self.pending = self.after(32, self.draw)
