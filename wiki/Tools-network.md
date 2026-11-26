[Open the technician hub](https://herbertofury.github.io/INTA/) · [Wiki home](https://github.com/Herbertofury/INTA/wiki) · [Source repository](https://github.com/Herbertofury/INTA)

# Networking

9 purpose-selected tools. Use the official download link for the current publisher offering; check architecture and release notes before installing.

| Tool | Best use | Platform | Kit level |
| --- | --- | --- | --- |
| [Wireshark](#wireshark) | Explaining DNS, TCP and application-protocol failures from a bounded capture. | Windows, Linux, macOS | Core |
| [iperf3](#iperf3) | Separating local network bandwidth limits from internet or application problems. | Linux, macOS, FreeBSD | Core |
| [Nmap](#nmap) | Identifying expected hosts and exposed services on a managed network. | Windows, Linux, macOS | Specialist |
| [Angry IP Scanner](#angry-ip-scanner) | Quickly finding devices on a known managed subnet. | Windows, Linux, macOS | Core |
| [PuTTY](#putty) | Connecting to network equipment or Linux systems from Windows. | Windows | Core |
| [WinSCP](#winscp) | Transferring support logs and configuration files to an approved host. | Windows | Core |
| [OpenSSH](#openssh) | Repeatable authenticated remote shell and transfer operations. | Windows, Linux, macOS | Core |
| [Windows Packet Monitor](#pktmon) | Collecting Windows network evidence without installing a separate capture application. | Windows | Core |
| [tcpdump](#tcpdump) | A small, bounded network capture from INTA Rescue or a Linux host. | Linux, macOS, BSD | Specialist |

<a name="wireshark"></a>

## Wireshark

Packet capture and protocol analysis with display filters and stream inspection.

**Choose it for:** Explaining DNS, TCP and application-protocol failures from a bounded capture.

**Platforms:** Windows, Linux, macOS · **Delivery:** installed · **Kit level:** Core

**Access / pricing:** Free / open source

[Official download](https://www.wireshark.org/download.html) · [Documentation](https://www.wireshark.org/docs/) · [Publisher](https://www.wireshark.org/)

> **Before you use it:** Captures can contain client traffic and credentials. Scope the capture and keep it in the private job record.

**Source:** https://www.wireshark.org/docs/  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json


<a name="iperf3"></a>

## iperf3

Measures throughput and loss between a controlled client and server.

**Choose it for:** Separating local network bandwidth limits from internet or application problems.

**Platforms:** Linux, macOS, FreeBSD · **Delivery:** installed · **Kit level:** Core

**Access / pricing:** Free / open source

[Official download](https://downloads.es.net/pub/iperf/) · [Documentation](https://software.es.net/iperf/invoking.html) · [Publisher](https://software.es.net/iperf/)

> **Before you use it:** Upstream officially supports Ubuntu Linux, FreeBSD and macOS, not native Windows. Use the INTA Linux environment for the upstream path.

**Source:** https://software.es.net/iperf/invoking.html  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="nmap"></a>

## Nmap

Network inventory and service discovery for an approved address range.

**Choose it for:** Identifying expected hosts and exposed services on a managed network.

**Platforms:** Windows, Linux, macOS · **Delivery:** installed · **Kit level:** Specialist

**Access / pricing:** Source-available under Nmap terms; Npcap terms differ

[Official download](https://nmap.org/download.html) · [Documentation](https://nmap.org/book/man.html) · [Publisher](https://nmap.org/)

> **Before you use it:** Choose a scoped, non-disruptive discovery plan. Avoid intrusive script categories during ordinary troubleshooting.

**Source:** https://nmap.org/book/man.html  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="angry-ip-scanner"></a>

## Angry IP Scanner

Graphical IP-address and port discovery for local network inventory.

**Choose it for:** Quickly finding devices on a known managed subnet.

**Platforms:** Windows, Linux, macOS · **Delivery:** installed · **Kit level:** Core

**Access / pricing:** Free / open source

[Official download](https://angryip.org/download/) · [Documentation](https://angryip.org/documentation/) · [Publisher](https://angryip.org/)

> **Before you use it:** Use the intended subnet and reasonable scan settings; results can be incomplete when devices block probes.

**Source:** https://angryip.org/documentation/  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="putty"></a>

## PuTTY

SSH and serial terminal client with companion transfer utilities.

**Choose it for:** Connecting to network equipment or Linux systems from Windows.

**Platforms:** Windows · **Delivery:** portable · **Kit level:** Core

**Access / pricing:** Free / open source

[Official download](https://www.chiark.greenend.org.uk/~sgtatham/putty/latest.html) · [Documentation](https://www.chiark.greenend.org.uk/~sgtatham/putty/docs.html) · [Publisher](https://www.chiark.greenend.org.uk/~sgtatham/putty/)

> **Before you use it:** Verify host keys through the expected inventory rather than accepting an unexpected change.

**Source:** https://www.chiark.greenend.org.uk/~sgtatham/putty/docs.html  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json


<a name="winscp"></a>

## WinSCP

SFTP, SCP and other file-transfer workflows with a Windows graphical interface.

**Choose it for:** Transferring support logs and configuration files to an approved host.

**Platforms:** Windows · **Delivery:** portable · **Kit level:** Core

**Access / pricing:** Free / open source

[Official download](https://winscp.net/eng/download.php) · [Documentation](https://winscp.net/eng/docs/start) · [Publisher](https://winscp.net/)

> **Before you use it:** Verify host identity and keep stored credentials protected.

**Source:** https://winscp.net/eng/docs/start  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/windows-utilities-2026-11-14/catalog.json


<a name="openssh"></a>

## OpenSSH

Secure shell and file-transfer tools, including supported Windows client and server components.

**Choose it for:** Repeatable authenticated remote shell and transfer operations.

**Platforms:** Windows, Linux, macOS · **Delivery:** installed · **Kit level:** Core

**Access / pricing:** Free / open source

[Official download](https://www.openssh.com/portable.html) · [Documentation](https://learn.microsoft.com/en-us/windows-server/administration/openssh/openssh-overview) · [Publisher](https://www.openssh.com/)

> **Before you use it:** Client and server setup are different. Opening an inbound server is an explicit deployment decision.

**Source:** https://learn.microsoft.com/en-us/windows-server/administration/openssh/openssh-overview  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="pktmon"></a>

## Windows Packet Monitor

Built-in packet capture, filtering and network-stack diagnostics.

**Choose it for:** Collecting Windows network evidence without installing a separate capture application.

**Platforms:** Windows · **Delivery:** built-in · **Kit level:** Core

**Access / pricing:** Included with supported Windows

[Official download](https://learn.microsoft.com/en-us/windows-server/networking/technologies/pktmon/pktmon) · [Documentation](https://learn.microsoft.com/en-us/windows-server/networking/technologies/pktmon/pktmon) · [Publisher](https://learn.microsoft.com/en-us/windows-server/networking/technologies/pktmon/pktmon)

> **Before you use it:** Stop the capture after reproduction and protect the trace file.

**Source:** https://learn.microsoft.com/en-us/windows-server/networking/technologies/pktmon/pktmon  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation


<a name="tcpdump"></a>

## tcpdump

Command-line packet capture and filtering.

**Choose it for:** A small, bounded network capture from INTA Rescue or a Linux host.

**Platforms:** Linux, macOS, BSD · **Delivery:** installed · **Kit level:** Specialist

**Access / pricing:** Free / open source

[Official download](https://github.com/the-tcpdump-group/tcpdump/releases) · [Documentation](https://www.tcpdump.org/manpages/tcpdump.1.html) · [Publisher](https://www.tcpdump.org/)

> **Before you use it:** Store packet captures privately and stop collection after the reproduction.

**Source:** https://github.com/the-tcpdump-group/tcpdump  
**Editorial review:** 2026-11-26  
**Selection provenance:** Official project or vendor documentation, https://github.com/Herbertofury/INTA/blob/1ecba1129aac275a02f40b9c4f5bea438c9e2688/live/package-lists/inta.list.chroot
