[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Service map](https://github.com/Herbertofury/INTA/wiki/Service-Map) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

<a id="-hardware-diagnostics"></a>

# [Hardware diagnostics](https://github.com/Herbertofury/INTA/wiki/Job-hardware-bsod)

Use readings and repeatable tests to distinguish the component, driver, cooling, and power paths.

**9 tools** · [All categories](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Selection standards](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards)

**Selection question:** Load-related fault, idle crash, or unhealthy hardware?

<a id="-start-with-a-workflow"></a>

## Related playbooks

- **[🌡️ Diagnose crashes / BSOD](https://github.com/Herbertofury/INTA/wiki/Job-hardware-bsod)** · **[Open working view](https://herbertofury.github.io/INTA/#job/hardware-bsod)**
- **[🐢 Fix a slow PC](https://github.com/Herbertofury/INTA/wiki/Job-slow-pc)** · **[Open working view](https://herbertofury.github.io/INTA/#job/slow-pc)**

<a id="compare-options"></a>

<a id="️-compare-the-options"></a>
<a id="-compare-the-options"></a>

## Tool comparison

| Tool | Choose it for | Platforms / role | Official page + download |
| --- | --- | --- | --- |
| [CPU-Z](#cpu-z) | Confirming the actual hardware and memory configuration. | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://www.cpuid.com/softwares/cpu-z.html) · **[Download Windows portable ZIP](https://download.cpuid.com/cpu-z/cpu-z_3.02-en.zip)** |
| [HWiNFO](#hwinfo) | Investigating temperatures, clocks, power and component identity. | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://www.hwinfo.com/) · **[Download Windows x64 EXE](https://www.hwinfo.com/files/hwi64_860.exe)** |
| [Libre Hardware Monitor](#libre-hardware-monitor) | A freely available option for temperature, fan and clock investigation. | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor) · **[Download Windows portable · ZIP](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor/releases/download/v0.9.6/LibreHardwareMonitor.zip)** |
| [Memtest86+](#memtest86plus) | Investigating memory instability and repeatable hardware errors. | Bootable PC · Core | [Official page](https://www.memtest.org/) · **[Download x64 memory-test ISO · ZIP](https://www.memtest.org/download/v9.00/mt86plus_9.00_x86_64.iso.zip)** |
| [Official OEM driver and firmware portals](#oem-drivers) | Repairing a confirmed device issue or applying a required vendor update. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) · Core | [Official page](https://support.microsoft.com/en-us/windows/hardware/drivers/update-drivers-through-device-manager-in-windows) · **[Find the correct driver source](https://support.microsoft.com/en-us/windows/hardware/drivers/update-drivers-through-device-manager-in-windows)** |
| [PassMark MemTest86](#memtest86) | A documented bench memory test with the edition features the job requires. | UEFI PC · Core | [Official page](https://www.memtest86.com/) · **[Download UEFI USB image · ZIP](https://www.memtest86.com/downloads/memtest86-usb.zip)** |
| [powercfg](#powercfg) | Collecting battery and sleep evidence before changing hardware or power policy. | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/powercfg-command-line-options) · **[Open built-in battery and sleep diagnostics](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/powercfg-command-line-options)** |
| [USB Device Tree Viewer](#usb-device-tree-view) | Identifying the actual USB port, hub, device and negotiated connection when a printer, scanner or peripheral disconnects or runs slowly. | [Windows](https://www.microsoft.com/windows) · Core | [Official page](https://www.uwe-sieber.de/usbtreeview_e.html) · **[Download Windows x64 portable · ZIP](https://www.uwe-sieber.de/files/UsbTreeView_x64.zip)** |
| [OCCT](#occt) | Reproducing a suspected load-related CPU, GPU or memory problem. | [Windows](https://www.microsoft.com/windows) · Specialist | [Official page](https://www.ocbase.com/) · **[Licensed Pro download](https://www.ocbase.com/download-pro)** |

Core means an everyday candidate; Specialist means a focused advanced use; Evaluate means prove it in a suitable test environment before promotion.

<a id="cpu-z"></a>

## [CPU-Z](https://www.cpuid.com/softwares/cpu-z.html)

CPU, motherboard and memory identification with detailed operating parameters.

[Official page](https://www.cpuid.com/softwares/cpu-z.html) · **[Download Windows portable ZIP](https://download.cpuid.com/cpu-z/cpu-z_3.02-en.zip)** · [Download Windows installer EXE](https://download.cpuid.com/cpu-z/cpu-z_3.02-en.exe) · [Documentation](https://www.cpuid.com/softwares/cpu-z.html)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Confirming the actual hardware and memory configuration. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | portable |
| **Access / pricing** | Free utility; vendor terms apply |
| **Selection role** | Core |

> **Before using it:** Inventory is not a stability test; compare readings with the system configuration.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://www.cpuid.com/softwares/cpu-z.html)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="hwinfo"></a>

## [HWiNFO](https://www.hwinfo.com/)

Detailed hardware inventory and supported sensor readings.

[Official page](https://www.hwinfo.com/) · **[Download Windows x64 EXE](https://www.hwinfo.com/files/hwi64_860.exe)** · [Portable Windows ZIP](https://www.hwinfo.com/files/hwi_860.zip) · [Other versions and editions](https://www.hwinfo.com/download/) · [Documentation](https://www.hwinfo.com/licenses/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Investigating temperatures, clocks, power and component identity. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | portable |
| **Access / pricing** | Pro license for commercial use of 64/ARM64 editions |
| **Selection role** | Core |

> **Before using it:** Use the appropriate commercial license. Sensor availability varies by hardware; logging creates background work.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://www.hwinfo.com/licenses/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="libre-hardware-monitor"></a>

## [Libre Hardware Monitor](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor)

Open-source hardware sensor monitoring for supported components.

[Official page](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor) · **[Download Windows portable · ZIP](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor/releases/download/v0.9.6/LibreHardwareMonitor.zip)** · [All download options](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor/releases/latest) · [Documentation](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A freely available option for temperature, fan and clock investigation. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | portable |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** Validate sensor labels against the actual hardware and stop collection when the job is done.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json

</details>

[Back to comparison](#compare-options)

<a id="memtest86plus"></a>

## [Memtest86+](https://www.memtest.org/)

Open-source bootable memory testing independent of the installed operating system.

[Official page](https://www.memtest.org/) · **[Download x64 memory-test ISO · ZIP](https://www.memtest.org/download/v9.00/mt86plus_9.00_x86_64.iso.zip)** · [All download options](https://www.memtest.org/#download) · [Documentation](https://github.com/memtest86plus/memtest86plus)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Investigating memory instability and repeatable hardware errors. |
| **Where it runs** | Bootable PC |
| **How you use it** | boot |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** Record configuration, duration and error evidence. Passing tests do not rule out every intermittent hardware issue.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://github.com/memtest86plus/memtest86plus)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="oem-drivers"></a>

## [Official OEM driver and firmware portals](https://support.microsoft.com/en-us/windows/hardware/drivers/update-drivers-through-device-manager-in-windows)

Manufacturer-supported driver and firmware sources matched to a specific device.

[Official page](https://support.microsoft.com/en-us/windows/hardware/drivers/update-drivers-through-device-manager-in-windows) · **[Find the correct driver source](https://support.microsoft.com/en-us/windows/hardware/drivers/update-drivers-through-device-manager-in-windows)** · [Documentation](https://support.microsoft.com/en-us/windows/hardware/drivers/update-drivers-through-device-manager-in-windows)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Repairing a confirmed device issue or applying a required vendor update. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | web |
| **Access / pricing** | Vendor downloads; support terms vary |
| **Selection role** | Core |

> **Before using it:** Use the exact model or service tag and vendor rollback procedure; do not substitute a generic driver-updater bundle.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://support.microsoft.com/en-us/windows/hardware/drivers/update-drivers-through-device-manager-in-windows)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="memtest86"></a>

## [PassMark MemTest86](https://www.memtest86.com/)

UEFI memory testing with free and professional editions.

[Official page](https://www.memtest86.com/) · **[Download UEFI USB image · ZIP](https://www.memtest86.com/downloads/memtest86-usb.zip)** · [All download options](https://www.memtest86.com/download.htm) · [Documentation](https://www.memtest86.com/tech.html)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A documented bench memory test with the edition features the job requires. |
| **Where it runs** | UEFI PC |
| **How you use it** | boot |
| **Access / pricing** | Free and paid Pro / site editions |
| **Selection role** | Core |

> **Before using it:** Choose a supported CPU/firmware platform and edition. It does not boot Apple Silicon Macs.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://www.memtest86.com/tech.html)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="powercfg"></a>

## [powercfg](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/powercfg-command-line-options)

Built-in [Windows](https://www.microsoft.com/windows) power diagnostics for battery usage, supported sleep states, wake causes and application power requests.

[Official page](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/powercfg-command-line-options) · **[Open built-in battery and sleep diagnostics](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/powercfg-command-line-options)** · [Documentation](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/powercfg-command-line-options)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Collecting battery and sleep evidence before changing hardware or power policy. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | built-in |
| **Access / pricing** | Included with [Windows](https://www.microsoft.com/windows) |
| **Selection role** | Core |

> **Before using it:** Start with reporting and query options. Other switches change power plans, hibernation or wake policy; review those changes separately. SleepStudy depends on supported power-state hardware, and reports can contain private device or usage information.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/powercfg-command-line-options)
- **Editorial review:** 2026-11-26
- **Evidence status:** Primary documentation reviewed; published file routes were checked with bounded response samples where supplied. No INTA installation, hardware, recovery or efficacy test is claimed.
- **Selection provenance:**
  - Current primary documentation and publisher acquisition sources
  - https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/powercfg-command-line-options

</details>

[Back to comparison](#compare-options)

<a id="usb-device-tree-view"></a>

## [USB Device Tree Viewer](https://www.uwe-sieber.de/usbtreeview_e.html)

Displays USB controllers, hubs, device descriptors, negotiated speeds and connection details in a tree.

[Official page](https://www.uwe-sieber.de/usbtreeview_e.html) · **[Download Windows x64 portable · ZIP](https://www.uwe-sieber.de/files/UsbTreeView_x64.zip)** · [Download Windows ARM64 portable · ZIP](https://www.uwe-sieber.de/files/UsbTreeView_arm64.zip) · [Download Windows x86 portable · ZIP](https://www.uwe-sieber.de/files/UsbTreeView_Win32.zip) · [Documentation](https://www.uwe-sieber.de/usbtreeview_e.html)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Identifying the actual USB port, hub, device and negotiated connection when a printer, scanner or peripheral disconnects or runs slowly. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | portable |
| **Access / pricing** | Freeware; publisher explicitly permits use in commercial environments |
| **Selection role** | Core |

> **Before using it:** Current builds require [Windows](https://www.microsoft.com/windows) 8 or later. A descriptor or connection-speed report is evidence about enumeration, not proof of a good cable, adequate power or successful application I/O. The publisher restricts modification and downloader-wrapper distribution.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://www.uwe-sieber.de/usbtreeview_e.html)
- **Editorial review:** 2026-11-26
- **Evidence status:** Primary documentation reviewed; published file routes were checked with bounded response samples where supplied. No INTA installation, hardware, recovery or efficacy test is claimed.
- **Selection provenance:**
  - Current primary documentation and publisher acquisition sources
  - https://www.uwe-sieber.de/usbtreeview_e.html

</details>

[Back to comparison](#compare-options)

<a id="occt"></a>

## [OCCT](https://www.ocbase.com/)

Component stress tests and stability diagnostics.

[Official page](https://www.ocbase.com/) · **[Licensed Pro download](https://www.ocbase.com/download-pro)** · [Personal edition options](https://www.ocbase.com/download-personal) · [All download options](https://www.ocbase.com/download) · [Documentation](https://www.ocbase.com/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Reproducing a suspected load-related CPU, GPU or memory problem. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | installed |
| **Access / pricing** | Personal and paid professional editions |
| **Selection role** | Specialist |

> **Before using it:** Check cooling, power and data protection first. Use the professional edition for client work and stop on unsafe temperatures.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://www.ocbase.com/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
