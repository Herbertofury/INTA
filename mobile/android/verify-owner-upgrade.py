"""Exercise an official signed upgrade on the CI-owned Android emulator."""
import re
import subprocess
import sys
import time
from xml.etree import ElementTree


def main(adb, serial, action):
    if serial != "emulator-5584" or action not in {"record", "check"}:
        raise ValueError("Expected the dedicated acceptance emulator and a valid mode")

    def call(*args):
        return subprocess.check_output([adb, "-s", serial, *args], timeout=25)

    call("shell", "am", "force-stop", "com.inta.mobile")
    call("shell", "am", "start", "-W", "-n", "com.inta.mobile/.JobActivity")
    target = "Confirm scope and back up"
    tapped = False
    for _ in range(8):
        call("shell", "uiautomator", "dump", "/sdcard/inta-upgrade-ui.xml")
        tree = ElementTree.fromstring(call("exec-out", "cat", "/sdcard/inta-upgrade-ui.xml"))
        node = next((n for n in tree.iter("node") if n.get("text") == target), None)
        if node is None:
            dims = re.findall(rb"(\d+)x(\d+)", call("shell", "wm", "size"))
            w, h = map(int, dims[-1])
            call("shell", "input", "swipe", str(w // 2), str(h * 3 // 4), str(w // 2), str(h // 3), "400")
            continue
        if node.get("checked") == "true":
            if action == "record" and not tapped:
                raise ValueError("Prior install was already populated")
            return
        if action == "record" and not tapped and node.get("enabled") == "true":
            left, top, right, bottom = map(int, re.findall(r"\d+", node.attrib["bounds"]))
            call("shell", "input", "tap", str((left + right) // 2), str((top + bottom) // 2))
            tapped = True
        time.sleep(0.25)
    raise RuntimeError("Legacy checklist item did not retain the expected state")


if __name__ == "__main__":
    main(*sys.argv[1:])
