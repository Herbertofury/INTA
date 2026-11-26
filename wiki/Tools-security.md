[Open the technician hub](https://herbertofury.github.io/INTA/) · [Wiki home](https://github.com/Herbertofury/INTA/wiki) · [Source repository](https://github.com/Herbertofury/INTA)

# Advanced defensive security

15 purpose-selected tools. Use the official download link for the current publisher offering; check architecture and release notes before installing.

| Tool | Best use | Platform | Kit level |
| --- | --- | --- | --- |
| [YARA-X](#yara-x) | Scanning selected evidence files with reviewed detection rules. | Windows, Linux, macOS | Specialist |
| [PE-sieve](#pe-sieve) | A specialist on-demand memory-integrity investigation. | Windows | Specialist |
| [HollowsHunter](#hollows-hunter) | A broader on-demand scan for suspicious process-memory modifications. | Windows | Specialist |
| [Fibratus](#fibratus) | A scoped lab or managed investigation requiring event correlation. | Windows | Specialist |
| [Rustinel](#rustinel) | An isolated evaluation of collection, rules, health checks and replay. | Windows, Linux, macOS | Evaluate |
| [PRX-SD](#prx-sd) | Comparative lab evaluation of a Rust security engine and its documented platform coverage. | Linux, Windows, macOS | Evaluate |
| [Sentinella](#sentinella) | Studying and testing scheduling, cache correctness and cancellation behavior. | Windows | Evaluate |
| [Velociraptor](#velociraptor) | Scoped forensic collection across a managed endpoint fleet. | Windows, Linux, macOS | Specialist |
| [Hayabusa](#hayabusa) | Analyzing exported EVTX logs during incident triage. | Windows, Linux, macOS | Specialist |
| [Chainsaw](#chainsaw) | Searching exported logs and generating investigation leads. | Windows, Linux, macOS | Specialist |
| [Sigma rules](#sigma) | A starting point for reviewed detections in an existing analysis platform. | Cross-platform | Specialist |
| [osquery](#osquery) | Repeatable endpoint inventory and documented state checks. | Windows, Linux, macOS | Specialist |
| [Wazuh](#wazuh) | An ongoing client security-monitoring deployment with managed operations. | Windows, Linux, macOS | Specialist |
| [Volatility 3](#volatility) | Specialist examination of memory evidence after collection. | Windows, Linux, macOS | Specialist |
| [Autopsy](#autopsy) | A structured specialist investigation of an acquired image. | Windows | Specialist |

<a name="yara-x"></a>

## YARA-X

Rust-based pattern-matching engine for compiling and applying YARA rules.

**Choose it for:** Scanning selected evidence files with reviewed detection rules.

**Platforms:** Windows, Linux, macOS · **Delivery:** portable · **Kit level:** Specialist

**Access / pricing:** Free / open source

[Official download](https://github.com/VirusTotal/yara-x/releases/latest) · [Documentation](https://virustotal.github.io/yara-x/) · [Publisher](https://github.com/VirusTotal/yara-x)

> **Before you use it:** A rule match is an investigation lead. Rule compatibility and quality must be checked; this is not a complete antivirus.

**Source:** https://virustotal.github.io/yara-x/  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/engineering/docs/windows-endpoint-security.md


<a name="pe-sieve"></a>

## PE-sieve

Inspects one Windows process for suspicious memory modifications and injected code.

**Choose it for:** A specialist on-demand memory-integrity investigation.

**Platforms:** Windows · **Delivery:** portable · **Kit level:** Specialist

**Access / pricing:** Free / open source

[Official download](https://github.com/hasherezade/pe-sieve/releases/latest) · [Documentation](https://github.com/hasherezade/pe-sieve) · [Publisher](https://github.com/hasherezade/pe-sieve)

> **Before you use it:** JIT code, overlays and legitimate hooks can look suspicious; findings need interpretation and coverage notes.

**Source:** https://github.com/hasherezade/pe-sieve  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/engineering/docs/windows-endpoint-security.md


<a name="hollows-hunter"></a>

## HollowsHunter

Uses PE-sieve to inspect multiple Windows processes and collect findings.

**Choose it for:** A broader on-demand scan for suspicious process-memory modifications.

**Platforms:** Windows · **Delivery:** portable · **Kit level:** Specialist

**Access / pricing:** Free / open source

[Official download](https://github.com/hasherezade/hollows_hunter/releases/latest) · [Documentation](https://github.com/hasherezade/hollows_hunter) · [Publisher](https://github.com/hasherezade/hollows_hunter)

> **Before you use it:** Do not automatically terminate every flagged process. Record inaccessible processes and explain benign hooks.

**Source:** https://github.com/hasherezade/hollows_hunter  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/engineering/docs/windows-endpoint-security.md


<a name="fibratus"></a>

## Fibratus

Windows event telemetry and rule-driven threat investigation with capture capabilities.

**Choose it for:** A scoped lab or managed investigation requiring event correlation.

**Platforms:** Windows · **Delivery:** installed · **Kit level:** Specialist

**Access / pricing:** Free / open source

[Official download](https://github.com/rabbitstack/fibratus/releases/latest) · [Documentation](https://www.fibratus.io/) · [Publisher](https://github.com/rabbitstack/fibratus)

> **Before you use it:** Event collection has a resource cost and observes only its collection window; configure and stop it deliberately.

**Source:** https://www.fibratus.io/  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/engineering/docs/windows-endpoint-security.md


<a name="rustinel"></a>

## Rustinel

Rust endpoint telemetry and detection project studied by ATHENA for reusable components.

**Choose it for:** An isolated evaluation of collection, rules, health checks and replay.

**Platforms:** Windows, Linux, macOS · **Delivery:** installed · **Kit level:** Evaluate

**Access / pricing:** Open-source project; inspect current license

[Official download](https://github.com/Karib0u/rustinel/releases) · [Documentation](https://github.com/Karib0u/rustinel) · [Publisher](https://github.com/Karib0u/rustinel)

> **Before you use it:** ATHENA records missing pre-execution blocking and anti-tamper. It is not approved here as a production antivirus replacement.

**Source:** https://github.com/Karib0u/rustinel  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/engineering/docs/windows-endpoint-security.md


<a name="prx-sd"></a>

## PRX-SD

Cross-platform file-scanning and detection project evaluated in ATHENA research.

**Choose it for:** Comparative lab evaluation of a Rust security engine and its documented platform coverage.

**Platforms:** Linux, Windows, macOS · **Delivery:** installed · **Kit level:** Evaluate

**Access / pricing:** Open-source project; inspect current license

[Official download](https://github.com/openprx/prx-sd/releases) · [Documentation](https://github.com/openprx/prx-sd) · [Publisher](https://github.com/openprx/prx-sd)

> **Before you use it:** The reviewed Windows path differs from Linux interception. Do not transfer Linux protection claims to Windows.

**Source:** https://github.com/openprx/prx-sd  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/engineering/docs/windows-endpoint-security.md


<a name="sentinella"></a>

## Sentinella

Security-engine research candidate with resource-management and scan-cache lessons.

**Choose it for:** Studying and testing scheduling, cache correctness and cancellation behavior.

**Platforms:** Windows · **Delivery:** installed · **Kit level:** Evaluate

**Access / pricing:** Open-source project; inspect current license

[Official download](https://github.com/LeewayOpenSoftware/Sentinella/releases) · [Documentation](https://github.com/LeewayOpenSoftware/Sentinella) · [Publisher](https://github.com/LeewayOpenSoftware/Sentinella)

> **Before you use it:** ATHENA treats architectural claims as hypotheses requiring runtime proof; not a certified production replacement.

**Source:** https://github.com/LeewayOpenSoftware/Sentinella  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/engineering/docs/windows-endpoint-security.md


<a name="velociraptor"></a>

## Velociraptor

Endpoint collection and investigation platform using VQL artifacts.

**Choose it for:** Scoped forensic collection across a managed endpoint fleet.

**Platforms:** Windows, Linux, macOS · **Delivery:** installed · **Kit level:** Specialist

**Access / pricing:** Free / open source

[Official download](https://github.com/Velocidex/velociraptor/releases/latest) · [Documentation](https://docs.velociraptor.app/) · [Publisher](https://docs.velociraptor.app/)

> **Before you use it:** Artifact scope and server access determine the data collected; use a private evidence repository.

**Source:** https://docs.velociraptor.app/  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="hayabusa"></a>

## Hayabusa

Windows event-log threat hunting and timeline generation with Sigma-based detections.

**Choose it for:** Analyzing exported EVTX logs during incident triage.

**Platforms:** Windows, Linux, macOS · **Delivery:** portable · **Kit level:** Specialist

**Access / pricing:** Free / open source

[Official download](https://github.com/Yamato-Security/hayabusa/releases/latest) · [Documentation](https://github.com/Yamato-Security/hayabusa) · [Publisher](https://github.com/Yamato-Security/hayabusa)

> **Before you use it:** Rule alerts need corroboration. Preserve original logs and report missing channels or collection periods.

**Source:** https://github.com/Yamato-Security/hayabusa  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="chainsaw"></a>

## Chainsaw

Fast Windows event-log search and detection tooling.

**Choose it for:** Searching exported logs and generating investigation leads.

**Platforms:** Windows, Linux, macOS · **Delivery:** portable · **Kit level:** Specialist

**Access / pricing:** Free / open source

[Official download](https://github.com/WithSecureOpenSource/chainsaw/releases/latest) · [Documentation](https://github.com/WithSecureOpenSource/chainsaw) · [Publisher](https://github.com/WithSecureOpenSource/chainsaw)

> **Before you use it:** Match the rule and mapping versions to the evidence and retain the original log files.

**Source:** https://github.com/WithSecureOpenSource/chainsaw  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="sigma"></a>

## Sigma rules

Generic detection-rule format and community rules for security log analysis.

**Choose it for:** A starting point for reviewed detections in an existing analysis platform.

**Platforms:** Cross-platform · **Delivery:** portable · **Kit level:** Specialist

**Access / pricing:** Open-source rule set; rule and tooling licenses vary

[Official download](https://github.com/SigmaHQ/sigma) · [Documentation](https://sigmahq.io/docs/) · [Publisher](https://sigmahq.io/)

> **Before you use it:** Rules require compatible backends, correct field mappings and local false-positive tuning.

**Source:** https://sigmahq.io/docs/  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="osquery"></a>

## osquery

Exposes supported system state as SQL-queryable tables.

**Choose it for:** Repeatable endpoint inventory and documented state checks.

**Platforms:** Windows, Linux, macOS · **Delivery:** installed · **Kit level:** Specialist

**Access / pricing:** Free / open source

[Official download](https://osquery.io/downloads/) · [Documentation](https://osquery.readthedocs.io/en/stable/) · [Publisher](https://osquery.io/)

> **Before you use it:** Table coverage differs by OS. Scheduled queries and extensions must be explicitly configured.

**Source:** https://osquery.readthedocs.io/en/stable/  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="wazuh"></a>

## Wazuh

Endpoint agents and a central platform for security monitoring and analysis.

**Choose it for:** An ongoing client security-monitoring deployment with managed operations.

**Platforms:** Windows, Linux, macOS · **Delivery:** installed · **Kit level:** Specialist

**Access / pricing:** Free / open source; hosting and operations cost

[Official download](https://documentation.wazuh.com/current/installation-guide/index.html) · [Documentation](https://documentation.wazuh.com/current/) · [Publisher](https://wazuh.com/)

> **Before you use it:** A real deployment needs sizing, alert tuning, retention and ownership; installing an agent alone is not an operating SOC.

**Source:** https://documentation.wazuh.com/current/  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="volatility"></a>

## Volatility 3

Framework for analyzing supported memory captures.

**Choose it for:** Specialist examination of memory evidence after collection.

**Platforms:** Windows, Linux, macOS · **Delivery:** installed · **Kit level:** Specialist

**Access / pricing:** Open-source framework; license terms apply

[Official download](https://github.com/volatilityfoundation/volatility3/releases/latest) · [Documentation](https://volatility3.readthedocs.io/en/latest/) · [Publisher](https://volatilityfoundation.org/)

> **Before you use it:** Requires a compatible memory image and symbols; it does not acquire a memory image by itself.

**Source:** https://volatility3.readthedocs.io/en/latest/  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="autopsy"></a>

## Autopsy

Graphical digital-forensics platform for examining disk and file evidence.

**Choose it for:** A structured specialist investigation of an acquired image.

**Platforms:** Windows · **Delivery:** installed · **Kit level:** Specialist

**Access / pricing:** Free / open source

[Official download](https://www.autopsy.com/download/) · [Documentation](https://www.sleuthkit.org/autopsy/docs/user-docs/) · [Publisher](https://www.autopsy.com/)

> **Before you use it:** Work from a verified image and record acquisition hashes and analysis settings.

**Source:** https://www.sleuthkit.org/autopsy/docs/user-docs/  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation
