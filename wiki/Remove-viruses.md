[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Service map](https://github.com/Herbertofury/INTA/wiki/Service-Map) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

# 🛡️ Remove viruses

**Disconnect a suspected compromised PC from the network before scanning; use the [incident guide](https://github.com/Herbertofury/INTA/wiki/Contain-ransomware) if encryption is spreading.**

<a id="working-kit"></a>

## 🧰 Tools and links for this job

**[Open working view](https://herbertofury.github.io/INTA/#job/malware-removal/quick/suspected-malware)** · Keep the tool panel beside the instructions.

**First move:** Disconnect a suspected compromised PC from the network before scanning; use the [incident guide](https://github.com/Herbertofury/INTA/wiki/Contain-ransomware) if encryption is spreading.

| Use this for | Official tool + exact action |
| --- | --- |
| Remove viruses | **[Microsoft Safety Scanner](https://learn.microsoft.com/en-us/defender-endpoint/safety-scanner-download)** · [Official page](https://learn.microsoft.com/en-us/defender-endpoint/safety-scanner-download) · **[Download Windows x64 scanner · EXE](https://go.microsoft.com/fwlink/?LinkId=212732)** |
| Focused adware and browser-hijacker cleanup. | **[Malwarebytes AdwCleaner](https://www.malwarebytes.com/adwcleaner)** · [Official page](https://www.malwarebytes.com/adwcleaner) · **[Download Windows cleaner · EXE](https://adwcleaner.malwarebytes.com/adwcleaner?channel=release)** |
| Inspect evidence-backed startup persistence. | **[Autoruns](https://learn.microsoft.com/en-us/sysinternals/downloads/autoruns)** · [Official page](https://learn.microsoft.com/en-us/sysinternals/downloads/autoruns) · **[Download Windows portable · ZIP](https://download.sysinternals.com/files/Autoruns.zip)** |
| Offline scan when the documented platform and recovery requirements are met. | **[Microsoft Defender Offline](https://learn.microsoft.com/en-us/defender-endpoint/microsoft-defender-offline)** · [Official page](https://learn.microsoft.com/en-us/defender-endpoint/microsoft-defender-offline) · **[Run the built-in offline scan](https://learn.microsoft.com/en-us/defender-endpoint/microsoft-defender-offline)** |
| Use existing endpoint telemetry for investigation; plan new deployment after active evidence-preservation needs are resolved. | **[Sysmon](https://learn.microsoft.com/en-us/sysinternals/downloads/sysmon)** · [Official page](https://learn.microsoft.com/en-us/sysinternals/downloads/sysmon) · **[Download Windows monitoring tools · ZIP](https://download.sysinternals.com/files/Sysmon.zip)** |

### Run and verify

| Open the exact resource | What to do there |
| --- | --- |
| **[Safety Scanner results and expiry](https://learn.microsoft.com/en-us/defender-endpoint/safety-scanner-download)** | Read the completion result and %SYSTEMROOT%\debug\msert.log. |
| **[AdwCleaner scan and review](https://help.malwarebytes.com/hc/en-us/articles/31589231147035-Scan-and-clean-a-device-with-AdwCleaner)** | Review detected items before cleanup and retain the report. |

### Escalation

| Open the exact resource | What to do there |
| --- | --- |
| **[Contain ransomware or a wider compromise](https://github.com/Herbertofury/INTA/wiki/Contain-ransomware)** | Use the incident route for active encryption, lateral spread or privileged-account compromise. |
| **[Defender Offline requirements](https://learn.microsoft.com/en-us/defender-endpoint/microsoft-defender-offline)** | Check platform, primary antivirus, [WinRE](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-recovery-environment--windows-re--technical-reference) and [BitLocker](https://learn.microsoft.com/en-us/windows/security/operating-system-security/data-protection/bitlocker/) preparation. |
| **[Trusted reinstall procedure](https://support.microsoft.com/en-us/windows/deployment/install-upgrade/reinstall-windows-with-the-installation-media)** | Preserve required evidence and data before a clean installation. |

## ⚡ First three actions

1. Record the symptoms and isolate affected connections; keep the existing real-time protection in place.
2. On a trusted PC, download a fresh [Microsoft Safety Scanner](https://learn.microsoft.com/en-us/defender-endpoint/safety-scanner-download); transfer it safely and run a Full scan.
3. Save the scan log, review remaining symptoms and persistence, then follow the guide’s cleanup or rebuild decision.

**[Microsoft Safety Scanner](https://learn.microsoft.com/en-us/defender-endpoint/safety-scanner-download)** · **[Download Windows x64 scanner · EXE](https://go.microsoft.com/fwlink/?LinkId=212732)**

**[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-malware-removal)** · **[Specific fixes for this job](https://herbertofury.github.io/INTA/#job/malware-removal/quick/suspected-malware?section=fixes)** · **[Selection evidence](https://github.com/Herbertofury/INTA/wiki/Free-Tool-Picks)**

<a id="next-action"></a>

## 🧭 Match the symptom

| You see this | Do this next | You should get |
| --- | --- | --- |
| **The installed protection reports a threat** | Update through an approved channel, run its full scan and quarantine identified detections; retain the report. | The run has a completion state and a recorded action for every detection. |
| **Adware, unwanted extensions or browser redirection dominate** | Run [AdwCleaner](https://www.malwarebytes.com/adwcleaner), review its proposed changes and restore the intended browser extensions, search and notification settings. | The original browser task works and unwanted behavior does not return after restart. |
| **A second on-demand check is needed** | Download a fresh [Microsoft Safety Scanner](https://learn.microsoft.com/en-us/defender-endpoint/safety-scanner-download) from a clean PC, run a Full scan and read the detailed log. | Completed coverage, detections and unresolved items are recorded; a cancelled run is not marked clean. |
| **The same detection returns after reboot** | Inspect the relevant [Autoruns](https://learn.microsoft.com/en-us/sysinternals/downloads/autoruns) entry and documented offline-scan prerequisites; use the rebuild decision if integrity remains uncertain. | The persistence cause is explained or a clean recovery plan replaces repeated scanner loops. |

### ✅ Finish the job

- [ ] Protection is healthy after reboot and there are no unresolved detections.
- [ ] The original task and browser behavior remain correct.
- [ ] Compromised accounts and sessions have a separate documented resolution.

**Change route when:** Ransomware, lateral spread or privileged access is suspected. · Detections recur or scan coverage and system integrity cannot be established.


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
