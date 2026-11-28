import os
from pathlib import Path
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "media"))
try:
    import tkinter as tk
    from inta_optics_ui import OpticalHeader, PROFILES, SETTLE_MS, software_cell
except ImportError:
    tk = None


@unittest.skipUnless(tk, "Python Tk is required")
class SoftwareFieldTests(unittest.TestCase):
    def test_touch_rest_and_static_profiles(self):
        event = 32768 | (32768 << 16) | (250 << 32) | (255 << 48)
        for profile in range(5):
            idle = [software_cell(x, y, 32, 8, profile, 0, 0) for y in range(8) for x in range(32)]
            moving = [software_cell(x,y,32,8,profile,event,0) for y in range(8) for x in range(32)]
            self.assertEqual(idle == moving, profile >= 2)
            self.assertTrue(all(not value or chr(value >> 24).isalnum() for value in idle))
            self.assertEqual(any(idle), profile < 4)

    @unittest.skipUnless(os.environ.get("DISPLAY"), "run under Xvfb or X11 to exercise real Tk")
    def test_profiles_persist_and_events_stop(self):
        with tempfile.TemporaryDirectory() as folder:
            root = tk.Tk()
            self.addCleanup(root.destroy)
            root.geometry("550x160")
            config = Path(folder) / "profile"
            panel = OpticalHeader(root, library=Path(folder) / "missing.so", settings=config)
            panel.pack(fill="both", expand=True)
            root.update()
            self.assertIsNone(panel.native)
            self.assertGreater(len(panel.cells), 12)
            for value in PROFILES:
                panel.profile.set(value)
                panel.choose()
                root.update()
                self.assertEqual(config.read_text().strip(), value)
                self.assertEqual(bool(panel.cells), value != "Classic")
            panel.profile.set(PROFILES[0])
            panel.choose()
            root.update()
            panel.canvas.event_generate("<ButtonPress-1>", x=40, y=30)
            root.update()
            self.assertTrue(panel.impulses)
            self.assertIsNotNone(panel.pending)
            panel.impulses = [(when - SETTLE_MS/1000, bits) for when,bits in panel.impulses]
            root.after(50, root.quit)
            root.mainloop()
            self.assertIsNone(panel.pending)
            panel.high_contrast = True
            panel.queue()
            root.update()
            self.assertFalse(panel.cells)
