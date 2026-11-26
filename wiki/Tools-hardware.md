[Open the technician hub](https://herbertofury.github.io/INTA/) · [Wiki home](https://github.com/Herbertofury/INTA/wiki) · [Source repository](https://github.com/Herbertofury/INTA)

# Hardware diagnostics

7 purpose-selected tools. Use the official download link for the current publisher offering; check architecture and release notes before installing.

| Tool | Best use | Platform | Kit level |
| --- | --- | --- | --- |
| [HWiNFO](#hwinfo) | Investigating temperatures, clocks, power and component identity. | Windows | Core |
| [Libre Hardware Monitor](#libre-hardware-monitor) | A freely available option for temperature, fan and clock investigation. | Windows | Core |
| [Memtest86+](#memtest86plus) | Investigating memory instability and repeatable hardware errors. | Bootable PC | Core |
| [PassMark MemTest86](#memtest86) | A documented bench memory test with the edition features the job requires. | UEFI PC | Core |
| [CPU-Z](#cpu-z) | Confirming the actual hardware and memory configuration. | Windows | Core |
| [OCCT](#occt) | Reproducing a suspected load-related CPU, GPU or memory problem. | Windows | Specialist |
| [Official OEM driver and firmware portals](#oem-drivers) | Repairing a confirmed device issue or applying a required vendor update. | Windows, Linux, macOS | Core |

<a name="hwinfo"></a>

## HWiNFO

Detailed hardware inventory and supported sensor readings.

**Choose it for:** Investigating temperatures, clocks, power and component identity.

**Platforms:** Windows · **Delivery:** portable · **Kit level:** Core

**Access / pricing:** Pro license for commercial use of 64/ARM64 editions

[Official download](https://www.hwinfo.com/download/) · [Documentation](https://www.hwinfo.com/licenses/) · [Publisher](https://www.hwinfo.com/)

> **Before you use it:** Use the appropriate commercial license. Sensor availability varies by hardware; logging creates background work.

**Source:** https://www.hwinfo.com/licenses/  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="libre-hardware-monitor"></a>

## Libre Hardware Monitor

Open-source hardware sensor monitoring for supported components.

**Choose it for:** A freely available option for temperature, fan and clock investigation.

**Platforms:** Windows · **Delivery:** portable · **Kit level:** Core

**Access / pricing:** Free / open source

[Official download](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor/releases/latest) · [Documentation](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor) · [Publisher](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor)

> **Before you use it:** Validate sensor labels against the actual hardware and stop collection when the job is done.

**Source:** https://github.com/LibreHardwareMonitor/LibreHardwareMonitor  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json


<a name="memtest86plus"></a>

## Memtest86+

Open-source bootable memory testing independent of the installed operating system.

**Choose it for:** Investigating memory instability and repeatable hardware errors.

**Platforms:** Bootable PC · **Delivery:** boot · **Kit level:** Core

**Access / pricing:** Free / open source

[Official download](https://www.memtest.org/#download) · [Documentation](https://github.com/memtest86plus/memtest86plus) · [Publisher](https://www.memtest.org/)

> **Before you use it:** Record configuration, duration and error evidence. Passing tests do not rule out every intermittent hardware issue.

**Source:** https://github.com/memtest86plus/memtest86plus  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="memtest86"></a>

## PassMark MemTest86

UEFI memory testing with free and professional editions.

**Choose it for:** A documented bench memory test with the edition features the job requires.

**Platforms:** UEFI PC · **Delivery:** boot · **Kit level:** Core

**Access / pricing:** Free and paid Pro / site editions

[Official download](https://www.memtest86.com/download.htm) · [Documentation](https://www.memtest86.com/tech.html) · [Publisher](https://www.memtest86.com/)

> **Before you use it:** Choose a supported CPU/firmware platform and edition. It does not boot Apple Silicon Macs.

**Source:** https://www.memtest86.com/tech.html  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="cpu-z"></a>

## CPU-Z

CPU, motherboard and memory identification with detailed operating parameters.

**Choose it for:** Confirming the actual hardware and memory configuration.

**Platforms:** Windows · **Delivery:** portable · **Kit level:** Core

**Access / pricing:** Free utility; vendor terms apply

[Official download](https://www.cpuid.com/softwares/cpu-z.html) · [Documentation](https://www.cpuid.com/softwares/cpu-z.html) · [Publisher](https://www.cpuid.com/softwares/cpu-z.html)

> **Before you use it:** Inventory is not a stability test; compare readings with the system configuration.

**Source:** https://www.cpuid.com/softwares/cpu-z.html  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="occt"></a>

## OCCT

Component stress tests and stability diagnostics.

**Choose it for:** Reproducing a suspected load-related CPU, GPU or memory problem.

**Platforms:** Windows · **Delivery:** installed · **Kit level:** Specialist

**Access / pricing:** Personal and paid professional editions

[Official download](https://www.ocbase.com/download) · [Documentation](https://www.ocbase.com/) · [Publisher](https://www.ocbase.com/)

> **Before you use it:** Check cooling, power and data protection first. Use the professional edition for client work and stop on unsafe temperatures.

**Source:** https://www.ocbase.com/  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="oem-drivers"></a>

## Official OEM driver and firmware portals

Manufacturer-supported driver and firmware sources matched to a specific device.

**Choose it for:** Repairing a confirmed device issue or applying a required vendor update.

**Platforms:** Windows, Linux, macOS · **Delivery:** web · **Kit level:** Core

**Access / pricing:** Vendor downloads; support terms vary

[Official download](https://support.microsoft.com/en-us/windows/hardware/drivers/update-drivers-through-device-manager-in-windows) · [Documentation](https://support.microsoft.com/en-us/windows/hardware/drivers/update-drivers-through-device-manager-in-windows) · [Publisher](https://support.microsoft.com/en-us/windows/hardware/drivers/update-drivers-through-device-manager-in-windows)

> **Before you use it:** Use the exact model or service tag and vendor rollback procedure; do not substitute a generic driver-updater bundle.

**Source:** https://support.microsoft.com/en-us/windows/hardware/drivers/update-drivers-through-device-manager-in-windows  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation
