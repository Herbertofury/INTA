# Remote Support & RMM Field Guide

Remote access is privileged access. Select the model based on attended/unattended needs, managed-device status and company policy.

## Tool choices

| Need | Candidate | Fit / trade-off |
|---|---|---|
| One-time attended help | [Windows Quick Assist](https://support.microsoft.com/windows/use-quick-assist-to-help-users) | User participation; not fleet management. |
| Commercial remote support | [TeamViewer](https://www.teamviewer.com/), [AnyDesk](https://anydesk.com/) | Review tenant, license, MFA, unattended access, privacy and audit controls. |
| Self-hosted remote access | [RustDesk](https://rustdesk.com/) | INTA owns relay hardening, TLS, ACLs, patching and logs. |
| Managed fleet | [Microsoft Intune](https://learn.microsoft.com/mem/intune/) | Enrollment, configuration, compliance and app deployment; licensing applies. |
| Self-hosted management | [MeshCentral](https://meshcentral.com/) | Secure hosting, updates and access governance required. |
| RMM automation | [Tactical RMM](https://github.com/amidaware/tacticalrmm) | High privilege; formal security and supply-chain review required. |
| Windows admin | PowerShell Remoting / [Windows Admin Center](https://learn.microsoft.com/windows-server/manage/windows-admin-center/overview) | Identity, transport, firewall scope and audit are critical. |

## Minimum security baseline

- Company-owned tenant/server; approved asset enrollment.
- Individual technician accounts, MFA and role-based least privilege.
- Explicit user consent for attended sessions; documented approval for unattended access.
- Session/admin audit logs with defined retention and review.
- Restrict network exposure; use VPN/private access or approved zero-trust gateway.
- Patch servers, relays, agents and clients; monitor vendor advisories.
- Store API keys/enrollment tokens in approved secrets management.
- Disable file transfer, clipboard, shell and privilege escalation where not needed.
- Define offboarding, agent revocation, lost-device response, emergency access and platform restore.

## Attended session workflow

1. Verify requester identity and device.
2. Explain what the technician can see/do and obtain consent.
3. Ask user to close unrelated confidential windows; never ask for passwords or MFA codes.
4. Start with read-only diagnostics.
5. Explain changes and obtain approval for disruptive actions.
6. End session, verify user outcome and record ticket/session reference.
7. Confirm temporary access is closed and transferred files are handled per policy.

## PowerShell Remoting patterns

Use domain Kerberos or organization-approved certificate-based HTTPS. Do not use Basic authentication over HTTP, disable certificate validation or expose WinRM to the public internet.

```powershell
# Check an approved host:
Test-WSMan -ComputerName 'approved-host.contoso.example'

# Interactive authorized session:
Enter-PSSession -ComputerName 'approved-host.contoso.example' -Authentication Kerberos

# Read-only inventory:
Invoke-Command -ComputerName 'approved-host.contoso.example' -Authentication Kerberos -ScriptBlock {
  Get-ComputerInfo | Select-Object CsName, WindowsProductName, OsBuildNumber
  Get-Volume | Select-Object DriveLetter, FileSystem, HealthStatus, SizeRemaining
}
```

Use JEA (Just Enough Administration) for delegated tasks that do not need full administrator rights. Follow Microsoft guidance for listener configuration, firewall scope, certificates, SPNs and auditing.

References: [PowerShell Remoting](https://learn.microsoft.com/powershell/scripting/learn/remoting/), [JEA](https://learn.microsoft.com/powershell/scripting/security/remoting/jea/overview), [Windows Admin Center](https://learn.microsoft.com/windows-server/manage/windows-admin-center/overview).

## RDP safety

Never port-forward TCP 3389 from the public internet. Prefer VPN/private access or managed gateway with MFA. Require NLA where supported, patch systems, restrict allowed groups and firewall scope, audit logons, and disable RDP when not needed.

## Self-hosted deployment review

For RustDesk/MeshCentral, review architecture, threat model and license; use supported TLS; harden and restrict management ports; separate test/production secrets; verify release artifacts; log enrollment/session/file transfer/admin changes; test agent revocation, platform restore and incident containment. Self-hosting does not automatically make remote access secure.
