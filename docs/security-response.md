# Endpoint Security & Malware Response

The first job during suspected compromise is to protect people, systems and evidence—not to run every scanner available.

## First response

1. Confirm report, device identity, incident time, exact alert and observed behavior.
2. Notify the designated security/incident-response owner and follow severity policy.
3. If compromise is active, isolate using approved EDR/network controls. Power-state decisions belong to the incident lead; do not improvise shutdowns.
4. Do not attach backup drives or technician laptops to suspected infected systems.
5. Do not delete files, clear logs, uninstall agents, run cleanup tools or reset the machine before evidence requirements are understood.
6. Use a known-clean admin workstation for account/session actions.

Isolation can disrupt critical operations and remote access. Coordinate with the incident lead.

## Choose the response

| Observation | Initial action |
|---|---|
| EDR alert, execution unconfirmed | Preserve alert, timeline, file hash; ask security owner to classify. |
| Ransom note or mass encryption | Escalate immediately, isolate per IR plan, protect backups; no cleanup/reboot without direction. |
| Credential theft / suspicious sign-in | Revoke sessions and rotate credentials from a clean device under identity-team direction. |
| Browser pop-ups / unwanted extension | Capture evidence, inspect extensions/policies, scan with approved tools. |
| Rootkit / persistence suspected | EDR/Defender Offline or approved forensic workflow; consider trusted rebuild. |
| False positive | Preserve detection and hash; submit through approved vendor process; avoid broad exclusions. |

## Defender checks

Elevated PowerShell, only where permitted:

```powershell
Get-MpComputerStatus |
  Select-Object AntivirusEnabled, RealTimeProtectionEnabled, BehaviorMonitorEnabled,
    AntivirusSignatureVersion, AntivirusSignatureLastUpdated
Get-MpThreatDetection |
  Select-Object InitialDetectionTime, ThreatID, ActionSuccess, Resources
Get-MpPreference |
  Select-Object DisableRealtimeMonitoring, DisableBehaviorMonitoring,
    ExclusionPath, ExclusionExtension, ExclusionProcess
Update-MpSignature
# Coordinate with incident response before scanning/remediating:
Start-MpScan -ScanType QuickScan
```

Cmdlets and fields vary by Windows platform and policy. Empty results do not prove a clean device. Never disable Defender/tamper protection or add broad exclusions to make another scanner work.

## Approved scanner options

- [Microsoft Safety Scanner (MSERT)](https://learn.microsoft.com/defender-endpoint/safety-scanner-download): on-demand scanner; obtain a fresh Microsoft copy.
- [Microsoft Defender Offline](https://learn.microsoft.com/defender-endpoint/microsoft-defender-offline): scans outside normal Windows; check BitLocker recovery readiness and incident approval.
- [ClamAV](https://docs.clamav.net/): useful additional scanner in INTA live image, not a replacement for managed EDR.
- [YARA](https://yara.readthedocs.io/) / [YARA-X](https://virustotal.github.io/yara-x/): rule-based file triage; validate rule compatibility.
- Third-party second-opinion scanners only under approved license and vendor-direct downloads. Avoid simultaneous real-time AV products.

## Evidence to preserve

As directed by the incident lead: alert ID/severity/time, device identity, isolation status, file path/hash/signer, EDR timeline, relevant EVTX logs, network indicators and approved memory/disk images. Record collector, timestamp, tool/version, source, destination and access history. Hash collected evidence. Do not store customer evidence in this public repository or ordinary chat.

```powershell
Get-FileHash -LiteralPath 'C:\Path\To\SuspiciousFile.exe' -Algorithm SHA256
Get-AuthenticodeSignature -LiteralPath 'C:\Path\To\SuspiciousFile.exe' |
  Format-List Status, StatusMessage, SignerCertificate
```

A valid signature does not prove a file is safe; an unsigned file is not automatically malicious.

## Recovery and return to service

1. Security owner decides whether in-place cleanup is appropriate or trusted rebuild is required.
2. Validate backups from before compromise; ensure backups are not still exposed to compromised identities/devices.
3. Rebuild from trusted, patched media/approved deployment when scope or persistence cannot be bounded.
4. Restore only validated data and approved software.
5. Re-enroll EDR/device management; verify policy, encryption and logging.
6. Revoke affected sessions/tokens and rotate credentials from clean systems as directed.
7. Validate user workflows and monitor before closing the incident.

## Never

- Upload client files, memory dumps, email archives or logs to public scanners without authorization.
- Disable endpoint security/firewall/tamper protection as a generic fix.
- Use cracked tools, unknown removal kits or repackaged installers.
- Promise a scan found everything or a clean result proves safety.
- Reconnect a compromised endpoint before incident owner approval.

References: [Microsoft Defender](https://learn.microsoft.com/defender/), [Defender for Endpoint](https://learn.microsoft.com/defender-endpoint/), [MSERT](https://learn.microsoft.com/defender-endpoint/safety-scanner-download), [Defender Offline](https://learn.microsoft.com/defender-endpoint/microsoft-defender-offline), [Sleuth Kit](https://www.sleuthkit.org/).
