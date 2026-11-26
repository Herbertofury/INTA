[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

<a id="-networking"></a>

# Networking

Separate link, addressing, name resolution, transport, and application evidence.

**9 tools** · [All categories](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Selection standards](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards)

**Selection question:** One device, one service, or the whole office?

<a id="-start-with-a-workflow"></a>

## Related playbooks

- [Fix internet, DNS, or local-network problems](https://github.com/Herbertofury/INTA/wiki/Job-network-dns)
- [Bring an office back after a power outage](https://github.com/Herbertofury/INTA/wiki/Job-outage-recovery)
- [Repair a workstation’s domain trust relationship](https://github.com/Herbertofury/INTA/wiki/Job-ad-trust)

<a id="compare-options"></a>

<a id="️-compare-the-options"></a>
<a id="-compare-the-options"></a>

## Tool comparison

| Tool | Choose it for | Platforms | Role |
| --- | --- | --- | --- |
| [Angry IP Scanner](#angry-ip-scanner) | Quickly finding devices on a known managed subnet. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) | Core |
| [iperf3](#iperf3) | Separating local network bandwidth limits from internet or application problems. | [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos), FreeBSD | Core |
| [OpenSSH](#openssh) | Repeatable authenticated remote shell and transfer operations. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) | Core |
| [PuTTY](#putty) | Connecting to network equipment or [Linux](https://www.kernel.org/) systems from [Windows](https://www.microsoft.com/windows). | [Windows](https://www.microsoft.com/windows) | Core |
| [Windows Packet Monitor](#pktmon) | Collecting [Windows](https://www.microsoft.com/windows) network evidence without installing a separate capture application. | [Windows](https://www.microsoft.com/windows) | Core |
| [WinSCP](#winscp) | Transferring support logs and configuration files to an approved host. | [Windows](https://www.microsoft.com/windows) | Core |
| [Wireshark](#wireshark) | Explaining DNS, TCP and application-protocol failures from a bounded capture. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) | Core |
| [Nmap](#nmap) | Identifying expected hosts and exposed services on a managed network. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) | Specialist |
| [tcpdump](#tcpdump) | A small, bounded network capture from [INTA Rescue](https://github.com/Herbertofury/INTA/blob/main/docs/rescue.md) or a [Linux](https://www.kernel.org/) host. | [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos), BSD | Specialist |

Core means an everyday candidate; Specialist means a focused advanced use; Evaluate means prove it in a suitable test environment before promotion.

<a id="angry-ip-scanner"></a>

## [Angry IP Scanner](https://angryip.org/)

Graphical IP-address and port discovery for local network inventory.

[Official download](https://angryip.org/download/) · [Documentation](https://angryip.org/documentation/) · [Publisher](https://angryip.org/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Quickly finding devices on a known managed subnet. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | installed |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** Use the intended subnet and reasonable scan settings; results can be incomplete when devices block probes.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://angryip.org/documentation/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="iperf3"></a>

## [iperf3](https://software.es.net/iperf/)

Measures throughput and loss between a controlled client and server.

[Official download](https://downloads.es.net/pub/iperf/) · [Documentation](https://software.es.net/iperf/invoking.html) · [Publisher](https://software.es.net/iperf/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Separating local network bandwidth limits from internet or application problems. |
| **Where it runs** | [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos), FreeBSD |
| **How you use it** | installed |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** Upstream officially supports [Ubuntu](https://ubuntu.com/) [Linux](https://www.kernel.org/), FreeBSD and [macOS](https://support.apple.com/macos), not native [Windows](https://www.microsoft.com/windows). Use the INTA [Linux](https://www.kernel.org/) environment for the upstream path.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://software.es.net/iperf/invoking.html)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="openssh"></a>

## [OpenSSH](https://www.openssh.com/)

Secure shell and file-transfer tools, including supported [Windows](https://www.microsoft.com/windows) client and server components.

[Official download](https://www.openssh.com/portable.html) · [Documentation](https://learn.microsoft.com/en-us/windows-server/administration/openssh/openssh-overview) · [Publisher](https://www.openssh.com/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Repeatable authenticated remote shell and transfer operations. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | installed |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** Client and server setup are different. Opening an inbound server is an explicit deployment decision.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://learn.microsoft.com/en-us/windows-server/administration/openssh/openssh-overview)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="putty"></a>

## [PuTTY](https://www.chiark.greenend.org.uk/~sgtatham/putty/)

SSH and serial terminal client with companion transfer utilities.

[Official download](https://www.chiark.greenend.org.uk/~sgtatham/putty/latest.html) · [Documentation](https://www.chiark.greenend.org.uk/~sgtatham/putty/docs.html) · [Publisher](https://www.chiark.greenend.org.uk/~sgtatham/putty/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Connecting to network equipment or [Linux](https://www.kernel.org/) systems from [Windows](https://www.microsoft.com/windows). |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | portable |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** Verify host keys through the expected inventory rather than accepting an unexpected change.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://www.chiark.greenend.org.uk/~sgtatham/putty/docs.html)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json

</details>

[Back to comparison](#compare-options)

<a id="pktmon"></a>

## [Windows Packet Monitor](https://learn.microsoft.com/en-us/windows-server/networking/technologies/pktmon/pktmon)

Built-in packet capture, filtering and network-stack diagnostics.

[Official download](https://learn.microsoft.com/en-us/windows-server/networking/technologies/pktmon/pktmon) · [Documentation](https://learn.microsoft.com/en-us/windows-server/networking/technologies/pktmon/pktmon) · [Publisher](https://learn.microsoft.com/en-us/windows-server/networking/technologies/pktmon/pktmon)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Collecting [Windows](https://www.microsoft.com/windows) network evidence without installing a separate capture application. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | built-in |
| **Access / pricing** | Included with supported [Windows](https://www.microsoft.com/windows) |
| **Selection role** | Core |

> **Before using it:** Stop the capture after reproduction and protect the trace file.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://learn.microsoft.com/en-us/windows-server/networking/technologies/pktmon/pktmon)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="winscp"></a>

## [WinSCP](https://winscp.net/)

SFTP, SCP and other file-transfer workflows with a [Windows](https://www.microsoft.com/windows) graphical interface.

[Official download](https://winscp.net/eng/download.php) · [Documentation](https://winscp.net/eng/docs/start) · [Publisher](https://winscp.net/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Transferring support logs and configuration files to an approved host. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows) |
| **How you use it** | portable |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** Verify host identity and keep stored credentials protected.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://winscp.net/eng/docs/start)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json

</details>

[Back to comparison](#compare-options)

<a id="wireshark"></a>

## [Wireshark](https://www.wireshark.org/)

Packet capture and protocol analysis with display filters and stream inspection.

[Official download](https://www.wireshark.org/download.html) · [Documentation](https://www.wireshark.org/docs/) · [Publisher](https://www.wireshark.org/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Explaining DNS, TCP and application-protocol failures from a bounded capture. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | installed |
| **Access / pricing** | Free / open source |
| **Selection role** | Core |

> **Before using it:** Captures can contain client traffic and credentials. Scope the capture and keep it in the private job record.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://www.wireshark.org/docs/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json

</details>

[Back to comparison](#compare-options)

<a id="nmap"></a>

## [Nmap](https://nmap.org/)

Network inventory and service discovery for an approved address range.

[Official download](https://nmap.org/download.html) · [Documentation](https://nmap.org/book/man.html) · [Publisher](https://nmap.org/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Identifying expected hosts and exposed services on a managed network. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | installed |
| **Access / pricing** | Source-available under [Nmap](https://nmap.org/) terms; Npcap terms differ |
| **Selection role** | Specialist |

> **Before using it:** Choose a scoped, non-disruptive discovery plan. Avoid intrusive script categories during ordinary troubleshooting.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://nmap.org/book/man.html)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="tcpdump"></a>

## [tcpdump](https://www.tcpdump.org/)

Command-line packet capture and filtering.

[Official download](https://github.com/the-tcpdump-group/tcpdump/releases) · [Documentation](https://www.tcpdump.org/manpages/tcpdump.1.html) · [Publisher](https://www.tcpdump.org/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A small, bounded network capture from [INTA Rescue](https://github.com/Herbertofury/INTA/blob/main/docs/rescue.md) or a [Linux](https://www.kernel.org/) host. |
| **Where it runs** | [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos), BSD |
| **How you use it** | installed |
| **Access / pricing** | Free / open source |
| **Selection role** | Specialist |

> **Before using it:** Store packet captures privately and stop collection after the reproduction.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://github.com/the-tcpdump-group/tcpdump)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation
  - https://github.com/Herbertofury/INTA/blob/1ecba1129aac275a02f40b9c4f5bea438c9e2688/live/package-lists/inta.list.chroot

</details>

[Back to comparison](#compare-options)


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
