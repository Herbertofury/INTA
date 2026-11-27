[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

<a id="-advanced-defensive-security"></a>

# Advanced defensive security

Specialist investigation and research candidates belong to an explicit scope and a proven evidence workflow.

**15 tools** · [All categories](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Selection standards](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards)

**Selection question:** Collect evidence, analyze a capture, or evaluate a candidate?

<a id="-start-with-a-workflow"></a>

## Related playbooks

- **[🚨 Contain ransomware](https://github.com/Herbertofury/INTA/wiki/Job-ransomware-response)** · **[Open working view](https://herbertofury.github.io/INTA/#job/ransomware-response)**
- **[🦠 Remove viruses](https://github.com/Herbertofury/INTA/wiki/Job-malware-removal)** · **[Open working view](https://herbertofury.github.io/INTA/#job/malware-removal)**

<a id="compare-options"></a>

<a id="️-compare-the-options"></a>
<a id="-compare-the-options"></a>

## Tool comparison

| Tool | Choose it for | Platforms / role | Official page + download |
| --- | --- | --- | --- |
| [Autopsy](#autopsy) | A structured specialist investigation of an acquired image. | [Windows](https://www.microsoft.com/windows) · Specialist | [Official page](https://www.autopsy.com/) · **[Download Windows x64 · MSI](https://github.com/sleuthkit/autopsy/releases/download/autopsy-4.23.1/autopsy-4.23.1-64bit.msi)** |
| [Chainsaw](#chainsaw) | Searching exported logs and generating investigation leads. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) · Specialist | [Official page](https://github.com/WithSecureOpenSource/chainsaw) · **[Download Windows x64 · ZIP](https://github.com/WithSecureOpenSource/chainsaw/releases/download/v2.18.0/chainsaw_x86_64-pc-windows-msvc.zip)** |
| [Fibratus](#fibratus) | A scoped lab or managed investigation requiring event correlation. | [Windows](https://www.microsoft.com/windows) · Specialist | [Official page](https://github.com/rabbitstack/fibratus) · **[Download Windows x64 · MSI](https://github.com/rabbitstack/fibratus/releases/download/v3.1.1/fibratus-3.1.1-amd64.msi)** |
| [Hayabusa](#hayabusa) | Analyzing exported EVTX logs during incident triage. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) · Specialist | [Official page](https://github.com/Yamato-Security/hayabusa) · **[Download Windows x64 · ZIP](https://github.com/Yamato-Security/hayabusa/releases/download/v4.4.0/hayabusa-4.4.0-win-x64.zip)** |
| [HollowsHunter](#hollows-hunter) | A broader on-demand scan for suspicious process-memory modifications. | [Windows](https://www.microsoft.com/windows) · Specialist | [Official page](https://github.com/hasherezade/hollows_hunter) · **[Download Windows x64 · EXE](https://github.com/hasherezade/hollows_hunter/releases/download/v0.4.1.1/hollows_hunter64.exe)** |
| [osquery](#osquery) | Repeatable endpoint inventory and documented state checks. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) · Specialist | [Official page](https://osquery.io/) · **[Download Windows x64 · ZIP](https://github.com/osquery/osquery/releases/download/5.24.1/osquery-5.24.1.windows_x86_64.zip)** |
| [PE-sieve](#pe-sieve) | A specialist on-demand memory-integrity investigation. | [Windows](https://www.microsoft.com/windows) · Specialist | [Official page](https://github.com/hasherezade/pe-sieve) · **[Download Windows x64 · EXE](https://github.com/hasherezade/pe-sieve/releases/download/v0.4.1.1/pe-sieve64.exe)** |
| [Sigma rules](#sigma) | A starting point for reviewed detections in an existing analysis platform. | Cross-platform · Specialist | [Official page](https://sigmahq.io/) · **[Download detection rules · ZIP](https://github.com/SigmaHQ/sigma/releases/download/r2026-07-01/sigma_all_rules.zip)** |
| [Velociraptor](#velociraptor) | Scoped forensic collection across a managed endpoint fleet. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) · Specialist | [Official page](https://docs.velociraptor.app/) · **[Download Windows x64 · MSI](https://github.com/Velocidex/velociraptor/releases/download/v0.77.4/velociraptor-v0.77.4-windows-amd64.msi)** |
| [Volatility 3](#volatility) | Specialist examination of memory evidence after collection. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) · Specialist | [Official page](https://volatilityfoundation.org/) · **[Download Windows executables · ZIP](https://github.com/volatilityfoundation/volatility3/releases/download/v2.28.2/volatility3-win-exes-2.28.2.zip)** |
| [Wazuh](#wazuh) | An ongoing client security-monitoring deployment with managed operations. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) · Specialist | [Official page](https://wazuh.com/) · **[Download Windows endpoint agent · MSI](https://packages.wazuh.com/4.x/windows/wazuh-agent-4.14.11-1.msi)** |
| [YARA-X](#yara-x) | Scanning selected evidence files with reviewed detection rules. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) · Specialist | [Official page](https://github.com/VirusTotal/yara-x) · **[Download Windows x64 CLI · ZIP](https://github.com/VirusTotal/yara-x/releases/download/v1.25.0/yara-x-v1.25.0-x86_64-pc-windows-msvc.zip)** |
| [PRX-SD](#prx-sd) | Comparative lab evaluation of a Rust security engine and its documented platform coverage. | [Linux](https://www.kernel.org/), [Windows](https://www.microsoft.com/windows), [macOS](https://support.apple.com/macos) · Evaluate | [Official page](https://github.com/openprx/prx-sd) · **[Download Windows x64 CLI · EXE](https://github.com/openprx/prx-sd/releases/download/v0.2.3/sd-windows-x86_64.exe)** |
| [Rustinel](#rustinel) | An isolated evaluation of collection, rules, health checks and replay. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) · Evaluate | [Official page](https://github.com/Karib0u/rustinel) · **[Download Windows x64 · ZIP](https://github.com/Karib0u/rustinel/releases/download/v1.11.0/rustinel-1.11.0-x86_64-pc-windows-msvc.zip)** |
| [Sentinella](#sentinella) | Studying and testing scheduling, cache correctness and cancellation behavior. | [Windows](https://www.microsoft.com/windows) · Evaluate | [Official page](https://github.com/LeewayOpenSoftware/Sentinella) · **[Download Windows x64 setup · EXE](https://github.com/LeewayOpenSoftware/Sentinella/releases/download/v0.1.13/Sentinella_0.1.13_x64-setup.exe)** |

Core means an everyday candidate; Specialist means a focused advanced use; Evaluate means prove it in a suitable test environment before promotion.

<a id="autopsy"></a>

## [Autopsy](https://www.autopsy.com/)

Graphical digital-forensics platform for examining disk and file evidence.

[Official page](https://www.autopsy.com/) · **[Download Windows x64 · MSI](https://github.com/sleuthkit/autopsy/releases/download/autopsy-4.23.1/autopsy-4.23.1-64bit.msi)** · [All download options](https://www.autopsy.com/download/) · [Documentation](https://www.sleuthkit.org/autopsy/docs/user-docs/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A structured specialist investigation of an acquired image. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | installed |
| **Access / pricing** | Free / open source |
| **Selection role** | Specialist |

> **Before using it:** Work from a verified image and record acquisition hashes and analysis settings.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://www.sleuthkit.org/autopsy/docs/user-docs/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="chainsaw"></a>

## [Chainsaw](https://github.com/WithSecureOpenSource/chainsaw)

Fast [Windows](https://www.microsoft.com/windows) event-log search and detection tooling.

[Official page](https://github.com/WithSecureOpenSource/chainsaw) · **[Download Windows x64 · ZIP](https://github.com/WithSecureOpenSource/chainsaw/releases/download/v2.18.0/chainsaw_x86_64-pc-windows-msvc.zip)** · [All download options](https://github.com/WithSecureOpenSource/chainsaw/releases/latest) · [Documentation](https://github.com/WithSecureOpenSource/chainsaw)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Searching exported logs and generating investigation leads. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | portable |
| **Access / pricing** | Free / open source |
| **Selection role** | Specialist |

> **Before using it:** Match the rule and mapping versions to the evidence and retain the original log files.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://github.com/WithSecureOpenSource/chainsaw)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="fibratus"></a>

## [Fibratus](https://github.com/rabbitstack/fibratus)

[Windows](https://www.microsoft.com/windows) event telemetry and rule-driven threat investigation with capture capabilities.

[Official page](https://github.com/rabbitstack/fibratus) · **[Download Windows x64 · MSI](https://github.com/rabbitstack/fibratus/releases/download/v3.1.1/fibratus-3.1.1-amd64.msi)** · [All download options](https://github.com/rabbitstack/fibratus/releases/latest) · [Documentation](https://www.fibratus.io/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A scoped lab or managed investigation requiring event correlation. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | installed |
| **Access / pricing** | Free / open source |
| **Selection role** | Specialist |

> **Before using it:** Event collection has a resource cost and observes only its collection window; configure and stop it deliberately.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://www.fibratus.io/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/engineering/docs/windows-endpoint-security.md

</details>

[Back to comparison](#compare-options)

<a id="hayabusa"></a>

## [Hayabusa](https://github.com/Yamato-Security/hayabusa)

[Windows](https://www.microsoft.com/windows) event-log threat hunting and timeline generation with Sigma-based detections.

[Official page](https://github.com/Yamato-Security/hayabusa) · **[Download Windows x64 · ZIP](https://github.com/Yamato-Security/hayabusa/releases/download/v4.4.0/hayabusa-4.4.0-win-x64.zip)** · [All download options](https://github.com/Yamato-Security/hayabusa/releases/latest) · [Documentation](https://github.com/Yamato-Security/hayabusa)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Analyzing exported EVTX logs during incident triage. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | portable |
| **Access / pricing** | Free / open source |
| **Selection role** | Specialist |

> **Before using it:** Rule alerts need corroboration. Preserve original logs and report missing channels or collection periods.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://github.com/Yamato-Security/hayabusa)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="hollows-hunter"></a>

## [HollowsHunter](https://github.com/hasherezade/hollows_hunter)

Uses [PE-sieve](https://github.com/hasherezade/pe-sieve) to inspect multiple [Windows](https://www.microsoft.com/windows) processes and collect findings.

[Official page](https://github.com/hasherezade/hollows_hunter) · **[Download Windows x64 · EXE](https://github.com/hasherezade/hollows_hunter/releases/download/v0.4.1.1/hollows_hunter64.exe)** · [All download options](https://github.com/hasherezade/hollows_hunter/releases/latest) · [Documentation](https://github.com/hasherezade/hollows_hunter)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A broader on-demand scan for suspicious process-memory modifications. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | portable |
| **Access / pricing** | Free / open source |
| **Selection role** | Specialist |

> **Before using it:** Do not automatically terminate every flagged process. Record inaccessible processes and explain benign hooks.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://github.com/hasherezade/hollows_hunter)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/engineering/docs/windows-endpoint-security.md

</details>

[Back to comparison](#compare-options)

<a id="osquery"></a>

## [osquery](https://osquery.io/)

Exposes supported system state as SQL-queryable tables.

[Official page](https://osquery.io/) · **[Download Windows x64 · ZIP](https://github.com/osquery/osquery/releases/download/5.24.1/osquery-5.24.1.windows_x86_64.zip)** · [All download options](https://osquery.io/downloads/) · [Documentation](https://osquery.readthedocs.io/en/stable/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Repeatable endpoint inventory and documented state checks. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | installed |
| **Access / pricing** | Free / open source |
| **Selection role** | Specialist |

> **Before using it:** Table coverage differs by OS. Scheduled queries and extensions must be explicitly configured.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://osquery.readthedocs.io/en/stable/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="pe-sieve"></a>

## [PE-sieve](https://github.com/hasherezade/pe-sieve)

Inspects one [Windows](https://www.microsoft.com/windows) process for suspicious memory modifications and injected code.

[Official page](https://github.com/hasherezade/pe-sieve) · **[Download Windows x64 · EXE](https://github.com/hasherezade/pe-sieve/releases/download/v0.4.1.1/pe-sieve64.exe)** · [All download options](https://github.com/hasherezade/pe-sieve/releases/latest) · [Documentation](https://github.com/hasherezade/pe-sieve)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A specialist on-demand memory-integrity investigation. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | portable |
| **Access / pricing** | Free / open source |
| **Selection role** | Specialist |

> **Before using it:** JIT code, overlays and legitimate hooks can look suspicious; findings need interpretation and coverage notes.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://github.com/hasherezade/pe-sieve)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/engineering/docs/windows-endpoint-security.md

</details>

[Back to comparison](#compare-options)

<a id="sigma"></a>

## [Sigma rules](https://sigmahq.io/)

Generic detection-rule format and community rules for security log analysis.

[Official page](https://sigmahq.io/) · **[Download detection rules · ZIP](https://github.com/SigmaHQ/sigma/releases/download/r2026-07-01/sigma_all_rules.zip)** · [All download options](https://github.com/SigmaHQ/sigma) · [Documentation](https://sigmahq.io/docs/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A starting point for reviewed detections in an existing analysis platform. |
| **Where it runs** | Cross-platform |
| **How you use it** | portable |
| **Access / pricing** | Open-source rule set; rule and tooling licenses vary |
| **Selection role** | Specialist |

> **Before using it:** Rules require compatible backends, correct field mappings and local false-positive tuning.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://sigmahq.io/docs/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="velociraptor"></a>

## [Velociraptor](https://docs.velociraptor.app/)

Endpoint collection and investigation platform using VQL artifacts.

[Official page](https://docs.velociraptor.app/) · **[Download Windows x64 · MSI](https://github.com/Velocidex/velociraptor/releases/download/v0.77.4/velociraptor-v0.77.4-windows-amd64.msi)** · [All download options](https://github.com/Velocidex/velociraptor/releases/latest) · [Documentation](https://docs.velociraptor.app/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Scoped forensic collection across a managed endpoint fleet. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | installed |
| **Access / pricing** | Free / open source |
| **Selection role** | Specialist |

> **Before using it:** Artifact scope and server access determine the data collected; use a private evidence repository.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://docs.velociraptor.app/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="volatility"></a>

## [Volatility 3](https://volatilityfoundation.org/)

Framework for analyzing supported memory captures.

[Official page](https://volatilityfoundation.org/) · **[Download Windows executables · ZIP](https://github.com/volatilityfoundation/volatility3/releases/download/v2.28.2/volatility3-win-exes-2.28.2.zip)** · [All download options](https://github.com/volatilityfoundation/volatility3/releases/latest) · [Documentation](https://volatility3.readthedocs.io/en/latest/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Specialist examination of memory evidence after collection. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | installed |
| **Access / pricing** | Open-source framework; license terms apply |
| **Selection role** | Specialist |

> **Before using it:** Requires a compatible memory image and symbols; it does not acquire a memory image by itself.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://volatility3.readthedocs.io/en/latest/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="wazuh"></a>

## [Wazuh](https://wazuh.com/)

Endpoint agents and a central platform for security monitoring and analysis.

[Official page](https://wazuh.com/) · **[Download Windows endpoint agent · MSI](https://packages.wazuh.com/4.x/windows/wazuh-agent-4.14.11-1.msi)** · [All download options](https://documentation.wazuh.com/current/installation-guide/index.html) · [Documentation](https://documentation.wazuh.com/current/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | An ongoing client security-monitoring deployment with managed operations. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | installed |
| **Access / pricing** | Free / open source; hosting and operations cost |
| **Selection role** | Specialist |

> **Before using it:** A real deployment needs sizing, alert tuning, retention and ownership; installing an agent alone is not an operating SOC.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://documentation.wazuh.com/current/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="yara-x"></a>

## [YARA-X](https://github.com/VirusTotal/yara-x)

Rust-based pattern-matching engine for compiling and applying YARA rules.

[Official page](https://github.com/VirusTotal/yara-x) · **[Download Windows x64 CLI · ZIP](https://github.com/VirusTotal/yara-x/releases/download/v1.25.0/yara-x-v1.25.0-x86_64-pc-windows-msvc.zip)** · [All download options](https://github.com/VirusTotal/yara-x/releases/latest) · [Documentation](https://virustotal.github.io/yara-x/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Scanning selected evidence files with reviewed detection rules. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | portable |
| **Access / pricing** | Free / open source |
| **Selection role** | Specialist |

> **Before using it:** A rule match is an investigation lead. Rule compatibility and quality must be checked; this is not a complete antivirus.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://virustotal.github.io/yara-x/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/engineering/docs/windows-endpoint-security.md

</details>

[Back to comparison](#compare-options)

<a id="prx-sd"></a>

## [PRX-SD](https://github.com/openprx/prx-sd)

Cross-platform file-scanning and detection project evaluated in [ATHENA](https://github.com/Herbertofury/Athena) research.

[Official page](https://github.com/openprx/prx-sd) · **[Download Windows x64 CLI · EXE](https://github.com/openprx/prx-sd/releases/download/v0.2.3/sd-windows-x86_64.exe)** · [All download options](https://github.com/openprx/prx-sd/releases) · [Documentation](https://github.com/openprx/prx-sd)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Comparative lab evaluation of a Rust security engine and its documented platform coverage. |
| **Where it runs** | [Linux](https://www.kernel.org/), [Windows](https://www.microsoft.com/windows), [macOS](https://support.apple.com/macos) |
| **How you use it** | installed |
| **Access / pricing** | Open-source project; inspect current license |
| **Selection role** | Evaluate |

> **Before using it:** The reviewed [Windows](https://www.microsoft.com/windows) path differs from [Linux](https://www.kernel.org/) interception. Do not transfer [Linux](https://www.kernel.org/) protection claims to [Windows](https://www.microsoft.com/windows).

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://github.com/openprx/prx-sd)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/engineering/docs/windows-endpoint-security.md

</details>

[Back to comparison](#compare-options)

<a id="rustinel"></a>

## [Rustinel](https://github.com/Karib0u/rustinel)

Rust endpoint telemetry and detection project studied by [ATHENA](https://github.com/Herbertofury/Athena) for reusable components.

[Official page](https://github.com/Karib0u/rustinel) · **[Download Windows x64 · ZIP](https://github.com/Karib0u/rustinel/releases/download/v1.11.0/rustinel-1.11.0-x86_64-pc-windows-msvc.zip)** · [All download options](https://github.com/Karib0u/rustinel/releases) · [Documentation](https://github.com/Karib0u/rustinel)

| Decision | Detail |
| --- | --- |
| **Choose it for** | An isolated evaluation of collection, rules, health checks and replay. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | installed |
| **Access / pricing** | Open-source project; inspect current license |
| **Selection role** | Evaluate |

> **Before using it:** [ATHENA](https://github.com/Herbertofury/Athena) records missing pre-execution blocking and anti-tamper. It is not approved here as a production antivirus replacement.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://github.com/Karib0u/rustinel)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/engineering/docs/windows-endpoint-security.md

</details>

[Back to comparison](#compare-options)

<a id="sentinella"></a>

## [Sentinella](https://github.com/LeewayOpenSoftware/Sentinella)

Security-engine research candidate with resource-management and scan-cache lessons.

[Official page](https://github.com/LeewayOpenSoftware/Sentinella) · **[Download Windows x64 setup · EXE](https://github.com/LeewayOpenSoftware/Sentinella/releases/download/v0.1.13/Sentinella_0.1.13_x64-setup.exe)** · [All download options](https://github.com/LeewayOpenSoftware/Sentinella/releases) · [Documentation](https://github.com/LeewayOpenSoftware/Sentinella)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Studying and testing scheduling, cache correctness and cancellation behavior. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | installed |
| **Access / pricing** | Open-source project; inspect current license |
| **Selection role** | Evaluate |

> **Before using it:** [ATHENA](https://github.com/Herbertofury/Athena) treats architectural claims as hypotheses requiring runtime proof; not a certified production replacement.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://github.com/LeewayOpenSoftware/Sentinella)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/engineering/docs/windows-endpoint-security.md

</details>

[Back to comparison](#compare-options)


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
