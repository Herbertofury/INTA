[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

# 🔌 Make a boot USB

**Choose the official ISO and identify a USB drive that may be erased.**

<a id="working-kit"></a>

## 🧰 Tools and links for this job

**[Open working view](https://herbertofury.github.io/INTA/#job/boot-usb/quick/boot-usb)** · Keep the tool panel beside the instructions.

**First move:** Choose the official ISO and identify a USB drive that may be erased.

| Use this for | Official tool + exact action |
| --- | --- |
| Make a boot USB | **[Rufus](https://rufus.ie/)** · [Official page](https://rufus.ie/) · **[Download Windows x64 portable · EXE](https://github.com/pbatard/rufus/releases/download/v4.16/rufus-4.16p.exe)** |
| Maintain a verified multiboot collection. | **[Ventoy](https://www.ventoy.net/)** · [Official page](https://www.ventoy.net/) · **[Download Windows package · ZIP](https://github.com/ventoy/Ventoy/releases/download/v1.1.18/ventoy-1.1.18-windows.zip)** |
| Obtain supported [Windows repair](https://github.com/Herbertofury/INTA/wiki/Job-windows-repair)/install media. | **[Windows 11 installation media](https://www.microsoft.com/software-download/windows11)** · [Official page](https://www.microsoft.com/software-download/windows11) · **[Download media creation tool · EXE](https://go.microsoft.com/fwlink/?linkid=2156295)** |
| Provide an independent imaging and recovery environment. | **[SystemRescue](https://www.system-rescue.org/)** · [Official page](https://www.system-rescue.org/) · **[Download x64 rescue image · ISO](https://fastly-cdn.system-rescue.org/releases/13.03/systemrescue-13.03-amd64.iso)** |

### Create media

| Open the exact resource | What to do there |
| --- | --- |
| **[Rufus operation and firmware choices](https://github.com/pbatard/rufus/wiki/FAQ)** | Confirm image, target USB and the intended boot mode. |
| **[Ventoy install versus update](https://www.ventoy.net/en/doc_start.html)** | Distinguish a destructive install from updating existing media. |

### Verify media

| Open the exact resource | What to do there |
| --- | --- |
| **[Verify a selected file hash](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.utility/get-filehash)** | Compare the exact downloaded image with its publisher’s checksum. |

### Boot compatibility

| Open the exact resource | What to do there |
| --- | --- |
| **[Ventoy Secure Boot instructions](https://www.ventoy.net/en/doc_secure.html)** | Review image/firmware support and recovery before changing keys or settings. |

## ⚡ First three actions

1. Download the correct image and check its published checksum or signature where supplied.
2. In [Rufus](https://rufus.ie/), confirm the USB model and capacity, select the ISO and choose the target’s firmware-compatible options.
3. Write the USB, check the log and test that it boots before taking it to the client.

**[Rufus](https://rufus.ie/)** · **[Download Windows x64 portable · EXE](https://github.com/pbatard/rufus/releases/download/v4.16/rufus-4.16p.exe)**

**[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-boot-usb)** · **[Selection evidence](https://github.com/Herbertofury/INTA/wiki/Free-Tool-Picks)**

<a id="next-action"></a>

## 🧭 Match the symptom

| You see this | Do this next | You should get |
| --- | --- | --- |
| **Only one rescue or installation image is needed** | Use the official ISO and [Rufus](https://rufus.ie/); confirm the selected USB can be erased and choose firmware-compatible options. | The intended image is written to the correct device. |
| **Several verified images must stay on one USB** | Use the documented [Ventoy](https://www.ventoy.net/) install/update path and copy the checked images to its data partition. | The expected images appear in the boot menu. |
| **The publisher supplies a checksum** | Run Get-FileHash for the exact downloaded file and compare it with the official value before writing. | The calculated hash matches the trusted release record. |
| **Writing finished but the client may have unusual hardware** | Boot-test the intended firmware mode and verify keyboard, storage, network and the required recovery tool. | The actual service path works on representative hardware rather than only completing an image write. |

### ✅ Finish the job

- [ ] Image source, hash and release are recorded.
- [ ] The USB boots and detects the hardware required for the planned job.

**Change route when:** The target USB or its erase permission is uncertain. · A firmware/security change lacks the necessary recovery material.


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
