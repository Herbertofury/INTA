[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Service map](https://github.com/Herbertofury/INTA/wiki/Service-Map) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

# 🌐 Fix internet

**Check whether one device, one connection or the whole office has lost access.**

<a id="tool-choices"></a>

## 🧰 Choose the tool for this job

Choose by the task shown in the first column. Each option includes its acquisition route and access boundary.

| Need | Official tool + exact action | Why this choice / access |
| --- | --- | --- |
| **Connection diagnostic** | **[Windows Get Help](https://support.microsoft.com/en-us/support/get-help/about-get-help)** · **[Open network troubleshooting](https://support.microsoft.com/en-us/windows/experience/connectivity-networking/fix-wi-fi-connection-issues-in-windows)** | Run the supported network diagnostic after checking link, address, gateway and DNS. **Access:** Included with supported [Windows](https://www.microsoft.com/windows) |
| **DNS and port checks** | **[PowerShell](https://learn.microsoft.com/en-us/powershell/)** · **[Use Windows PowerShell 5.1](https://learn.microsoft.com/en-us/powershell/scripting/windows-powershell/starting-windows-powershell)** | Use existing commands to separate name resolution, routing and a required TCP service. **Access:** Included with supported [Windows](https://www.microsoft.com/windows) |
| **HTTP and TLS check** | **[curl](https://curl.se/)** · **[Use the Windows-included curl.exe](https://curl.se/windows/microsoft.html)** | Test the documented endpoint when ordinary connectivity works but an application request fails. **Access:** Included with supported [Windows](https://www.microsoft.com/windows) |

**[Current selection evidence](https://github.com/Herbertofury/INTA/wiki/Current-Tool-Choices)**

<a id="working-kit"></a>

## 🧰 Tools and links for this job

**[Open working view](https://herbertofury.github.io/INTA/#job/network-dns/quick/fix-internet)** · Keep the tool panel beside the instructions.

**First move:** Check whether one device, one connection or the whole office has lost access.

| Use this for | Official tool + exact action |
| --- | --- |
| Fix internet | **[Windows Get Help](https://support.microsoft.com/en-us/support/get-help/about-get-help)** · [Official page](https://support.microsoft.com/en-us/support/get-help/about-get-help) · **[Open network troubleshooting](https://support.microsoft.com/en-us/windows/experience/connectivity-networking/fix-wi-fi-connection-issues-in-windows)** |
| Read IP, DNS and connection results on the affected PC. | **[PowerShell](https://learn.microsoft.com/en-us/powershell/)** · [Official page](https://learn.microsoft.com/en-us/powershell/) · **[Use Windows PowerShell 5.1](https://learn.microsoft.com/en-us/powershell/scripting/windows-powershell/starting-windows-powershell)** |
| Capture only a scoped unresolved protocol failure. | **[Wireshark](https://www.wireshark.org/)** · [Official page](https://www.wireshark.org/) · **[Download Windows x64 · EXE](https://2.na.dl.wireshark.org/win64/Wireshark-4.6.10-x64.exe)** |
| Separating DNS, TCP, TLS and HTTP failures when a web service or application endpoint is unreachable. | **[curl](https://curl.se/)** · [Official page](https://curl.se/) · **[Use the Windows-included curl.exe](https://curl.se/windows/microsoft.html)** |
| Comparing a reproducible path problem from a [Linux](https://www.kernel.org/) or [macOS](https://support.apple.com/macos) bench, or [Windows](https://www.microsoft.com/windows) through WSL. | **[mtr](https://www.bitwizard.nl/mtr/)** · [Official page](https://www.bitwizard.nl/mtr/) · **[Install a Linux/WSL distribution package](https://github.com/traviscross/mtr#using-mtr-on-windows)** |

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

1. Check the cable or Wi-Fi connection and compare another device on the same network.
2. In [Windows Get Help](https://support.microsoft.com/en-us/support/get-help/about-get-help), search “connect to network and internet” and run [network diagnostics](https://github.com/Herbertofury/INTA/wiki/Job-network-dns).
3. Follow the relevant address, gateway or DNS branch, then retest the original site and business service.

**[Windows Get Help](https://support.microsoft.com/en-us/support/get-help/about-get-help)** · **[Open network troubleshooting](https://support.microsoft.com/en-us/windows/experience/connectivity-networking/fix-wi-fi-connection-issues-in-windows)**

**[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-network-dns)** · **[Specific fixes for this job](https://herbertofury.github.io/INTA/#job/network-dns/quick/fix-internet?section=fixes)** · **[Selection evidence](https://github.com/Herbertofury/INTA/wiki/Free-Tool-Picks)**

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
