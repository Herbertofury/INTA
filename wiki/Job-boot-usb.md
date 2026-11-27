[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

<a id="-build-and-refresh-a-technician-rescue-usb"></a>

# [Build and refresh a technician rescue USB](https://github.com/Herbertofury/INTA/wiki/Job-boot-usb)

Keep verified recovery images, [Windows repair](https://github.com/Herbertofury/INTA/wiki/Job-windows-repair) media, and the INTA knowledge hub ready for offline use.

**Recovery & boot** · Intermediate · 20–60 min plus downloads · 6 steps

[Tools and useful links](#working-kit) · [Quick answer](#quick-answer) · [Next action](#next-action) · [Decision route](#decision-route) · [Tools](#toolkit) · [Procedure](#walkthrough) · [Verification](#acceptance) · [Sources](#sources) · [Interactive checklist](https://herbertofury.github.io/INTA/#job/boot-usb)

<a id="working-kit"></a>

## 🧰 Tools and links for this job

**[Open working view](https://herbertofury.github.io/INTA/#job/boot-usb)** · Keep the tool panel beside the instructions.

**First move:** Identify the USB by model, capacity and contents; copy off anything that must be kept before a write operation.

| Use this for | Official tool + exact action |
| --- | --- |
| Write one selected official ISO to the intended USB. | **[Rufus](https://rufus.ie/)** · [Official page](https://rufus.ie/) · **[Download Windows x64 portable · EXE](https://github.com/pbatard/rufus/releases/download/v4.16/rufus-4.16p.exe)** |
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

<a id="quick-answer"></a>

## ⚡ Quick answer

<a id="quick-boot-usb"></a>

### 🔌 Make a boot USB

**Choose the official ISO and identify a USB drive that may be erased.**

1. Download the correct image and check its published checksum or signature where supplied.
2. In [Rufus](https://rufus.ie/), confirm the USB model and capacity, select the ISO and choose the target’s firmware-compatible options.
3. Write the USB, check the log and test that it boots before taking it to the client.

**[Rufus](https://rufus.ie/)** · [Official page](https://rufus.ie/) · **[Download Windows x64 portable · EXE](https://github.com/pbatard/rufus/releases/download/v4.16/rufus-4.16p.exe)**

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

> [!NOTE]
> **Start here.** Choose a known USB drive and copy off anything that must be kept. Confirm its model, capacity, and contents before a media-writing operation.

<a id="decision-route"></a>

<a id="-choose-the-right-branch"></a>

## 🧭 Choose a route

```mermaid
flowchart TD
    A["Identify USB and preserve its contents"]:::action --> B{"One image or a multiboot kit?"}:::decision
    B -->|"One image"| C["Choose the dedicated writing path"]:::action
    B -->|"Several images"| D["Choose the multiboot path"]:::action
    C --> E["Get official image and verify checksum"]:::action
    D --> E
    E --> F{"Correct target and operation confirmed?"}:::decision
    F -->|"No"| G["Stop before writing"]:::stop
    F -->|"Yes"| H["Write media or update the intended image"]:::action
    H --> I{"Actual hardware boot succeeds?"}:::decision
    I -->|"No"| J["Check firmware and image compatibility"]:::action
    I -->|"Yes"| K["Record manifest and tested hardware"]:::done
    classDef decision fill:#eaf0f8,stroke:#607fa7,color:#172b46
    classDef action fill:#f1f3f6,stroke:#8792a2,color:#263445
    classDef stop fill:#faf0f0,stroke:#a46b70,color:#5e3137
    classDef done fill:#e9edf3,stroke:#6c7d94,color:#253449
```

**Route links:** [Continue this workflow](#walkthrough) · [Acceptance checks](#acceptance) · [Recover a Windows PC that will not boot](https://github.com/Herbertofury/INTA/wiki/Job-boot-recovery) · [Rescue data from a failing or unreadable drive](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive) · [Prove a backup can actually restore](https://github.com/Herbertofury/INTA/wiki/Job-backup-restore)

| Checkpoint | What to carry forward |
| --- | --- |
| **Before changing anything** | Identify the USB and save anything that must be retained. |
| **Keep during the job** | Official image, checksum, selected operation, and boot-test hardware. |
| **Before handing back** | Required tools and offline guide work on the actual boot path. |

<a id="toolkit"></a>

<a id="-build-the-right-toolkit"></a>

## Recommended tools

Select a tool for the next diagnostic or repair step. Check its platform, access and licensing requirements before use.

| Tool / official page + download | Use it for | Platform & access | Documentation |
| --- | --- | --- | --- |
| **[Rufus](https://rufus.ie/)** · [Official page](https://rufus.ie/) · **[Download Windows x64 portable · EXE](https://github.com/pbatard/rufus/releases/download/v4.16/rufus-4.16p.exe)** | Write one selected official ISO to the intended USB. | [Windows](https://www.microsoft.com/windows); Free / open source | [Open guide](https://github.com/pbatard/rufus/wiki/FAQ) |
| **[Ventoy](https://www.ventoy.net/)** · [Official page](https://www.ventoy.net/) · **[Download Windows package · ZIP](https://github.com/ventoy/Ventoy/releases/download/v1.1.18/ventoy-1.1.18-windows.zip)** | Maintain a verified multiboot collection. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/); Free / open source | [Open guide](https://www.ventoy.net/en/doc_start.html) |
| **[Windows 11 installation media](https://www.microsoft.com/software-download/windows11)** · [Official page](https://www.microsoft.com/software-download/windows11) · **[Download media creation tool · EXE](https://go.microsoft.com/fwlink/?linkid=2156295)** | Obtain supported [Windows repair](https://github.com/Herbertofury/INTA/wiki/Job-windows-repair)/install media. | [Windows](https://www.microsoft.com/windows), Bootable PC; Download available; [Windows](https://www.microsoft.com/windows) license required | [Open guide](https://www.microsoft.com/software-download/windows11) |
| **[SystemRescue](https://www.system-rescue.org/)** · [Official page](https://www.system-rescue.org/) · **[Download x64 rescue image · ISO](https://fastly-cdn.system-rescue.org/releases/13.03/systemrescue-13.03-amd64.iso)** | Provide an independent imaging and recovery environment. | Bootable PC; Free / open source | [Open guide](https://www.system-rescue.org/manual/) |
| **[Hiren's BootCD PE](https://www.hirensbootcd.org/)** · [Official page](https://www.hirensbootcd.org/) · **[Download x64 rescue image · ISO](https://www.hirensbootcd.org/files/HBCD_PE_x64.iso)** | A familiar [Windows](https://www.microsoft.com/windows)-oriented offline bench toolkit. | Bootable PC; Free distribution; bundled tools have individual terms | [Open guide](https://www.hirensbootcd.org/howtos/) |
| **[Rescuezilla](https://rescuezilla.com/)** · [Official page](https://rescuezilla.com/) · **[Download x64 rescue image · ISO](https://github.com/rescuezilla/rescuezilla/releases/download/2.6.2/rescuezilla-2.6.2-64bit.resolute.iso)** | A visual bare-metal backup or restore workflow at the bench. | Bootable PC; Free / open source | [Open guide](https://rescuezilla.com/help) |

<a id="walkthrough"></a>

<a id="-work-the-case"></a>

## Procedure

Work in order, record the result of each step, and change route when the evidence calls for it.

<a id="step-01"></a>

### 01 · Choose the media strategy

- [ ] Use [Ventoy](https://www.ventoy.net/) for a multiboot collection that can be refreshed by copying ISO files, or [Rufus](https://rufus.ie/) for a dedicated image. Keep official [Windows](https://www.microsoft.com/windows) media for [WinRE](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference) and [Windows](https://www.microsoft.com/windows)-native repair alongside [Linux](https://www.kernel.org/)-based rescue tools. An ISO’s platform determines which tools run inside it.

<a id="step-02"></a>

### 02 · Download from the official source

- [ ] Use the catalog’s official download links and keep release notes and vendor checksums with each image. Use the INTA [Media Creator](https://github.com/Herbertofury/INTA/blob/main/docs/rescue.md) for its verified INTA ISO download/update flow. A current boot environment does not guarantee every bundled third-party tool is current.

<a id="step-03"></a>

### 03 · Verify the bytes before writing

- [ ] Compare the local SHA-256 hash with the release’s published checksum using the exact file path. In [PowerShell](https://learn.microsoft.com/en-us/powershell/), select the downloaded ISO and run Get-FileHash for that file. The hash proves a match to the published value; also verify you obtained that value from the genuine release.

<a id="step-04"></a>

### 04 · Write or update the chosen device

- [ ] Installing [Ventoy](https://www.ventoy.net/) or writing with [Rufus](https://rufus.ie/) can erase the selected USB. Review the device identity before confirming. Use [Ventoy](https://www.ventoy.net/)’s Update operation for an existing [Ventoy](https://www.ventoy.net/) installation when intended, and copy refreshed verified ISOs to its data partition. Keep a rollback image until the new one boots.

<a id="step-05"></a>

### 05 · Test the actual boot path

- [ ] Boot the USB on a representative spare machine and confirm keyboard, display, storage, network, and recovery tools. Test the intended firmware mode. Read [Ventoy](https://www.ventoy.net/)’s current Secure Boot documentation before enrolling keys or changing firmware settings; firmware and image support must be proven on the target hardware.

<a id="step-06"></a>

### 06 · Keep an offline manifest and refresh on demand

- [ ] Record image name, source URL, release, hash, download date, and successful boot test. Include the INTA offline hub and keep client evidence on a separate encrypted destination. Refresh before a service visit and re-verify changed images instead of assuming the old USB is current.

<a id="acceptance"></a>

<a id="-prove-the-result"></a>

## Verification

- [ ] Each image matches its official checksum where one is provided.
- [ ] The intended USB boots and can see required hardware.
- [ ] The offline hub and recovery destination are accessible.
- [ ] The manifest distinguishes downloaded, verified, and hardware-boot-tested images.

> [!WARNING]
> **Pause this route and escalate when:**
> - The target USB is ambiguous or contains the only copy of important data.
> - Secure Boot or encryption changes are proposed without a recovery plan.

<a id="-continue-with-the-evidence"></a>

## Related playbooks

| Next playbook | What it resolves |
| --- | --- |
| [Recover a Windows PC that will not boot](https://github.com/Herbertofury/INTA/wiki/Job-boot-recovery) | Separate firmware, storage, encryption, and [Windows](https://www.microsoft.com/windows) startup faults before changing the boot configuration. |
| [Rescue data from a failing or unreadable drive](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive) | Preserve the source, create a recoverable image, and perform recovery on a copy. |
| [Prove a backup can actually restore](https://github.com/Herbertofury/INTA/wiki/Job-backup-restore) | Validate recovery credentials, data integrity, application usability, and recovery time. |

<a id="sources"></a>

<a id="-source-trail"></a>

## Official sources

- [Ventoy: install and update](https://www.ventoy.net/en/doc_start.html)
- [Ventoy: verify image checksums](https://www.ventoy.net/en/doc_checksum.html)
- [Ventoy: Secure Boot behavior](https://www.ventoy.net/en/doc_secure.html)
- [INTA Rescue source and build workflow](https://github.com/Herbertofury/INTA)

Reviewed **2026-11-26**. Timing is a planning range. A successful command is evidence about that operation; acceptance belongs to the user’s original task.


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
