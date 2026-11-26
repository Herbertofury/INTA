[🏠 Home](https://github.com/Herbertofury/INTA/wiki) · [🧭 Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [📚 Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [🧰 Tool directory](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [✨ Open interactive hub](https://herbertofury.github.io/INTA/)

# 🔬 [Hardware diagnostics](https://github.com/Herbertofury/INTA/wiki/Job-hardware-bsod)

Use readings and repeatable tests to distinguish the component, driver, cooling, and power paths.

**7 tools** · [All disciplines](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Selection standards](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards)

> [!TIP]
> **Choose by the job:** Load-related fault, idle crash, or unhealthy hardware?

## 🧭 Start with a workflow

- [Diagnose crashes, blue screens, and overheating](https://github.com/Herbertofury/INTA/wiki/Job-hardware-bsod)
- [Diagnose a slow, freezing, or hitching PC](https://github.com/Herbertofury/INTA/wiki/Job-slow-pc)

<a id="compare-options"></a>

## ⚖️ Compare the options

| Tool | Choose it for | Platforms | Role |
| --- | --- | --- | --- |
| [CPU-Z](#cpu-z) | Confirming the actual hardware and memory configuration. | [Windows](https://www.microsoft.com/windows) | 🟢 Core |
| [HWiNFO](#hwinfo) | Investigating temperatures, clocks, power and component identity. | [Windows](https://www.microsoft.com/windows) | 🟢 Core |
| [Libre Hardware Monitor](#libre-hardware-monitor) | A freely available option for temperature, fan and clock investigation. | [Windows](https://www.microsoft.com/windows) | 🟢 Core |
| [Memtest86+](#memtest86plus) | Investigating memory instability and repeatable hardware errors. | Bootable PC | 🟢 Core |
| [Official OEM driver and firmware portals](#oem-drivers) | Repairing a confirmed device issue or applying a required vendor update. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) | 🟢 Core |
| [PassMark MemTest86](#memtest86) | A documented bench memory test with the edition features the job requires. | UEFI PC | 🟢 Core |
| [OCCT](#occt) | Reproducing a suspected load-related CPU, GPU or memory problem. | [Windows](https://www.microsoft.com/windows) | 🔬 Specialist |

Core means an everyday candidate; Specialist means a focused advanced use; Evaluate means prove it in a suitable test environment before promotion.

---

<a id="cpu-z"></a>

## [CPU-Z](https://www.cpuid.com/softwares/cpu-z.html)

CPU, motherboard and memory identification with detailed operating parameters.

**[CPU-Z: official download →](https://www.cpuid.com/softwares/cpu-z.html)** · [Documentation](https://www.cpuid.com/softwares/cpu-z.html) · [Publisher](https://www.cpuid.com/softwares/cpu-z.html)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Confirming the actual hardware and memory configuration. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | portable |
| **Access / pricing** | Free utility; vendor terms apply |
| **Selection role** | 🟢 Core |

> **Before using it:** Inventory is not a stability test; compare readings with the system configuration.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://www.cpuid.com/softwares/cpu-z.html)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[↑ Compare the options](#compare-options)

---

<a id="hwinfo"></a>

## [HWiNFO](https://www.hwinfo.com/)

Detailed hardware inventory and supported sensor readings.

**[HWiNFO: official download →](https://www.hwinfo.com/download/)** · [Documentation](https://www.hwinfo.com/licenses/) · [Publisher](https://www.hwinfo.com/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Investigating temperatures, clocks, power and component identity. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | portable |
| **Access / pricing** | Pro license for commercial use of 64/ARM64 editions |
| **Selection role** | 🟢 Core |

> **Before using it:** Use the appropriate commercial license. Sensor availability varies by hardware; logging creates background work.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://www.hwinfo.com/licenses/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[↑ Compare the options](#compare-options)

---

<a id="libre-hardware-monitor"></a>

## [Libre Hardware Monitor](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor)

Open-source hardware sensor monitoring for supported components.

**[Libre Hardware Monitor: official download →](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor/releases/latest)** · [Documentation](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor) · [Publisher](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A freely available option for temperature, fan and clock investigation. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | portable |
| **Access / pricing** | Free / open source |
| **Selection role** | 🟢 Core |

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

[↑ Compare the options](#compare-options)

---

<a id="memtest86plus"></a>

## [Memtest86+](https://www.memtest.org/)

Open-source bootable memory testing independent of the installed operating system.

**[Memtest86+: official download →](https://www.memtest.org/#download)** · [Documentation](https://github.com/memtest86plus/memtest86plus) · [Publisher](https://www.memtest.org/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Investigating memory instability and repeatable hardware errors. |
| **Where it runs** | Bootable PC |
| **How you use it** | boot |
| **Access / pricing** | Free / open source |
| **Selection role** | 🟢 Core |

> **Before using it:** Record configuration, duration and error evidence. Passing tests do not rule out every intermittent hardware issue.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://github.com/memtest86plus/memtest86plus)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[↑ Compare the options](#compare-options)

---

<a id="oem-drivers"></a>

## [Official OEM driver and firmware portals](https://support.microsoft.com/en-us/windows/hardware/drivers/update-drivers-through-device-manager-in-windows)

Manufacturer-supported driver and firmware sources matched to a specific device.

**[Official OEM driver and firmware portals: official download →](https://support.microsoft.com/en-us/windows/hardware/drivers/update-drivers-through-device-manager-in-windows)** · [Documentation](https://support.microsoft.com/en-us/windows/hardware/drivers/update-drivers-through-device-manager-in-windows) · [Publisher](https://support.microsoft.com/en-us/windows/hardware/drivers/update-drivers-through-device-manager-in-windows)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Repairing a confirmed device issue or applying a required vendor update. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | web |
| **Access / pricing** | Vendor downloads; support terms vary |
| **Selection role** | 🟢 Core |

> **Before using it:** Use the exact model or service tag and vendor rollback procedure; do not substitute a generic driver-updater bundle.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://support.microsoft.com/en-us/windows/hardware/drivers/update-drivers-through-device-manager-in-windows)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[↑ Compare the options](#compare-options)

---

<a id="memtest86"></a>

## [PassMark MemTest86](https://www.memtest86.com/)

UEFI memory testing with free and professional editions.

**[PassMark MemTest86: official download →](https://www.memtest86.com/download.htm)** · [Documentation](https://www.memtest86.com/tech.html) · [Publisher](https://www.memtest86.com/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A documented bench memory test with the edition features the job requires. |
| **Where it runs** | UEFI PC |
| **How you use it** | boot |
| **Access / pricing** | Free and paid Pro / site editions |
| **Selection role** | 🟢 Core |

> **Before using it:** Choose a supported CPU/firmware platform and edition. It does not boot Apple Silicon Macs.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://www.memtest86.com/tech.html)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[↑ Compare the options](#compare-options)

---

<a id="occt"></a>

## [OCCT](https://www.ocbase.com/)

Component stress tests and stability diagnostics.

**[OCCT: official download →](https://www.ocbase.com/download)** · [Documentation](https://www.ocbase.com/) · [Publisher](https://www.ocbase.com/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Reproducing a suspected load-related CPU, GPU or memory problem. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | installed |
| **Access / pricing** | Personal and paid professional editions |
| **Selection role** | 🔬 Specialist |

> **Before using it:** Check cooling, power and data protection first. Use the professional edition for client work and stop on unsafe temperatures.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://www.ocbase.com/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[↑ Compare the options](#compare-options)


---

**Source review: 2026-11-26.** Guidance review and tool release versions are separate. [Report a correction](https://github.com/Herbertofury/INTA/issues/new/choose) · [How we maintain this guide](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards)
