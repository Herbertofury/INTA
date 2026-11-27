[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

# 🌐 Inspect network traffic

**Use packet capture only after basic link, address, gateway and DNS checks have narrowed the fault.**

<a id="working-kit"></a>

## 🧰 Tools and links for this job

**[Open working view](https://herbertofury.github.io/INTA/#job/network-dns/quick/inspect-network-traffic)** · Keep the tool panel beside the instructions.

**First move:** Use packet capture only after basic link, address, gateway and DNS checks have narrowed the fault.

| Use this for | Official tool + exact action |
| --- | --- |
| Inspect network traffic | **[Wireshark](https://www.wireshark.org/)** · [Official page](https://www.wireshark.org/) · **[Download Windows x64 · EXE](https://2.na.dl.wireshark.org/win64/Wireshark-4.6.10-x64.exe)** |
| Run supported local connection diagnostics. | **[Windows Get Help](https://support.microsoft.com/en-us/support/get-help/about-get-help)** · [Official page](https://support.microsoft.com/en-us/support/get-help/about-get-help) · **[Open network troubleshooting](https://support.microsoft.com/en-us/windows/experience/connectivity-networking/fix-wi-fi-connection-issues-in-windows)** |
| Read IP, DNS and connection results on the affected PC. | **[PowerShell](https://learn.microsoft.com/en-us/powershell/)** · [Official page](https://learn.microsoft.com/en-us/powershell/) · **[Use Windows PowerShell 5.1](https://learn.microsoft.com/en-us/powershell/scripting/windows-powershell/starting-windows-powershell)** |

### First checks

| Open the exact resource | What to do there |
| --- | --- |
| **[Windows network troubleshooting](https://support.microsoft.com/en-us/windows/experience/connectivity-networking/fix-wi-fi-connection-issues-in-windows)** | Work through local connection checks before a network reset. |

### Read-only diagnostics

| Open the exact resource | What to do there |
| --- | --- |
| **[Get-NetIPConfiguration](https://learn.microsoft.com/en-us/powershell/module/nettcpip/get-netipconfiguration)** | Read the active adapter, address, gateway and DNS servers. |
| **[Resolve-DnsName](https://learn.microsoft.com/en-us/powershell/module/dnsclient/resolve-dnsname)** | Test the actual failing service name. |
| **[Test-NetConnection](https://learn.microsoft.com/en-us/powershell/module/nettcpip/test-netconnection)** | Test the service’s documented TCP port. |

## ⚡ First three actions

1. Get authorization and choose the smallest interface, host and time window that can reproduce the issue.
2. Capture the failing request in [Wireshark](https://www.wireshark.org/) and inspect the relevant DNS or connection exchange.
3. Apply one evidenced fix, repeat the request and store the capture only in the private case record.

**[Wireshark](https://www.wireshark.org/)** · **[Download Windows x64 · EXE](https://2.na.dl.wireshark.org/win64/Wireshark-4.6.10-x64.exe)**

**[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-network-dns)** · **[Selection evidence](https://github.com/Herbertofury/INTA/wiki/Free-Tool-Picks)**

<a id="next-action"></a>

## 🧭 Match the symptom

| You see this | Do this next | You should get |
| --- | --- | --- |
| **Only this PC is disconnected** | Check cable/Wi-Fi state and run the Network and Internet diagnostic in [Windows Get Help](https://support.microsoft.com/en-us/support/get-help/about-get-help). | The active link is identified and the diagnostic has a specific result. |
| **The PC has no usable gateway or a self-assigned IPv4 address** | Run Get-NetIPConfiguration; compare the active adapter with the approved DHCP/network configuration. | The PC receives the intended address, gateway and DNS rather than an invented static setting. |
| **IP connectivity works but a hostname fails** | Run Resolve-DnsName for that service name and compare the configured DNS server with a working client. | The correct answer resolves or the faulty DNS dependency is identified. |
| **The name resolves but the application cannot connect** | Run Test-NetConnection for the target’s documented TCP port and compare a working path; a successful TCP test still needs the app test. | The evidence distinguishes blocked transport from an application or authentication failure. |
| **Many devices or only VPN users fail** | Use the shared-network or managed-VPN owner and preserve adapter, route and DNS settings before disruptive changes. | The shared dependency has a scoped repair plan and owner. |

### ✅ Finish the job

- [ ] The original site, share or application works by its normal hostname.
- [ ] Required VPN and business connectivity still work after the change.

**Change route when:** The fault is a shared outage or managed-policy issue outside local authority. · The next action would reset production networking without a recovery path.


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
