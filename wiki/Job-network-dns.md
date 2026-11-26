[Home](https://github.com/Herbertofury/INTA/wiki) · [All playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Interactive checklist](https://herbertofury.github.io/INTA/#job/network-dns)

# Fix internet, DNS, or local-network problems

> Locate the failing layer before resetting adapters or changing the whole network.

| Job profile | Details |
| --- | --- |
| Category | Networking |
| Working time | 15–60 min |
| Experience | Intermediate |

## Start here

Determine whether the problem affects one app, one device, one network segment, or every device. Capture the working and failing cases.

## Your toolkit

| Tool | Why it belongs here |
| --- | --- |
| [Wireshark](https://www.wireshark.org/download.html) | Explaining DNS, TCP and application-protocol failures from a bounded capture. |
| [iperf3](https://downloads.es.net/pub/iperf/) | Separating local network bandwidth limits from internet or application problems. |
| [Microsoft Sysinternals Suite](https://learn.microsoft.com/en-us/sysinternals/downloads/sysinternals-suite) | The core portable Windows troubleshooting kit. |

## Follow the workflow

### 01. Check the physical and radio link

- [ ] Confirm power, cable, switch port, adapter link speed, Wi-Fi association, and signal. Try a known-good cable or port when the evidence points there. Record any recent router, VPN, adapter-driver, or firewall changes.

### 02. Read the assigned configuration

- [ ] On Windows, run the command below in PowerShell. Check address, subnet, default route, DNS servers, and DHCP source. Compare with a working device on the same intended network. On a domain network, preserve the organization’s DNS design; replacing internal DNS with a public resolver can break domain services.

```text
Get-NetIPConfiguration
```

### 03. Test reachability and name resolution separately

- [ ] Test the observed gateway, then an approved external destination, then the failing hostname. Use the actual gateway from the previous step. An unsuccessful ping alone does not prove an outage because ICMP may be filtered; compare the application’s required TCP connection. The PowerShell command uses a public baseline hostname; repeat it with the actual failing hostname.

```text
Resolve-DnsName learn.microsoft.com
```

### 04. Narrow the application path

- [ ] Compare another browser or application and test with the approved VPN/proxy state. Check time, certificates, captive portal, and service-status information. Use a short, targeted Wireshark capture only when it can resolve a specific question, and keep the trace private.

### 05. Repair only the identified layer

- [ ] Correct the proven DNS, DHCP, routing, adapter, or application configuration. Back up current settings before changes. Reserve network reset for cases that justify rebuilding adapters and settings; it may disrupt VPNs, virtual switches, and a remote-support session.

### 06. Retest the business workflow

- [ ] Check the original site or app, local shares, printers, domain sign-in, and any VPN. For throughput questions, compare like-for-like endpoints and use upstream iperf3 on a supported Linux or macOS system rather than assuming a Windows build is official.

## Verify before handoff

- [ ] The failing application works using the intended network path.
- [ ] DNS, shares, printing, VPN, and remote support still work.
- [ ] The change and its rollback are documented.

## Stop and escalate when

- A remote-only network change would cut off the only recovery path.
- The problem is a widespread outage or an active security incident.

## Related playbooks

- [Outage Recovery](https://github.com/Herbertofury/INTA/wiki/Job-outage-recovery)
- [Ad Trust](https://github.com/Herbertofury/INTA/wiki/Job-ad-trust)
- [Remote Support](https://github.com/Herbertofury/INTA/wiki/Job-remote-support)
- [Outlook Email](https://github.com/Herbertofury/INTA/wiki/Job-outlook-email)

## Official references

- [Microsoft: Wi-Fi connection troubleshooting](https://support.microsoft.com/en-us/windows/experience/connectivity-networking/fix-wi-fi-connection-issues-in-windows)
- [Microsoft: Windows troubleshooters](https://support.microsoft.com/en-us/support/get-help/windows-troubleshooters)
- [ESnet: iperf3 supported platforms and documentation](https://software.es.net/iperf/)

---

INTA Computer Services · Editorial review: 2026-11-26. Timing is an estimate; stop conditions take priority.
