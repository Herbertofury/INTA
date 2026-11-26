# Network Troubleshooting

## Diagnose by layer
1. Physical: cable, Wi-Fi radio, switch port, dock, airplane mode.
2. Interface: adapter state, driver, link speed, DHCP.
3. Local IP: address, subnet, gateway, duplicate-IP symptoms.
4. Routing: default route and path to destination.
5. DNS: resolver configuration and name resolution.
6. Transport: destination port, firewall, proxy and VPN.
7. Application: service, TLS/certificate and authentication.

## Windows quick sequence
```powershell
Get-NetAdapter | Format-Table Name, Status, LinkSpeed
Get-NetIPConfiguration
ipconfig /all
Test-Connection <gateway-ip> -Count 4
Resolve-DnsName example.com
Test-NetConnection example.com -Port 443 -InformationLevel Detailed
netsh winhttp show proxy
Get-NetConnectionProfile
```
Replace <gateway-ip> with the approved gateway address. Do not publish client network details in wiki examples.

## Interpret results
- No link: cable, access point, switch, adapter or driver.
- APIPA (169.254.x.x): DHCP path may be failing.
- Gateway reachable, external IP unreachable: routing/upstream/ISP.
- IP reachable, hostname fails: DNS.
- Name resolves, TCP port fails: service, firewall, proxy, route or port policy.
- One app fails: inspect app proxy, TLS, authentication and endpoint security before resetting the stack.

Use [Wireshark](https://www.wireshark.org/) or [tcpdump](https://www.tcpdump.org/) only with authorization and a defined capture window/filter. Captures may contain credentials, cookies, DNS names and private communications; protect and sanitize them.

References: [Nmap manual](https://nmap.org/book/man.html), [Microsoft NetTCPIP PowerShell](https://learn.microsoft.com/powershell/module/nettcpip/).
