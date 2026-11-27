[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Service map](https://github.com/Herbertofury/INTA/wiki/Service-Map) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

# 🚨 Contain ransomware

**Isolate affected connections and alert the incident owner immediately if files are being encrypted or compromise is spreading.**

<a id="working-kit"></a>

## 🧰 Tools and links for this job

**[Open working view](https://herbertofury.github.io/INTA/#job/ransomware-response/quick/contain-ransomware)** · Keep the tool panel beside the instructions.

**First move:** Isolate affected connections and alert the incident owner immediately if files are being encrypted or compromise is spreading.

| Use this for | Official tool + exact action |
| --- | --- |
| Use existing endpoint telemetry for investigation; plan new deployment after active evidence-preservation needs are resolved. | **[Sysmon](https://learn.microsoft.com/en-us/sysinternals/downloads/sysmon)** · [Official page](https://learn.microsoft.com/en-us/sysinternals/downloads/sysmon) · **[Download Windows monitoring tools · ZIP](https://download.sysinternals.com/files/Sysmon.zip)** |

### [Incident response](https://github.com/Herbertofury/INTA/wiki/Job-ransomware-response)

| Open the exact resource | What to do there |
| --- | --- |
| **[CISA ransomware response checklist](https://www.cisa.gov/sites/default/files/2025-03/StopRansomware-Guide%20508.pdf)** | Isolation, evidence preservation and recovery priorities. |
| **[Microsoft incident-response approach](https://learn.microsoft.com/en-us/security/ransomware/incident-response-playbook-dart-ransomware-approach)** | Scope compromised systems, identities and recovery dependencies. |

### Recovery

| Open the exact resource | What to do there |
| --- | --- |
| **[Recovery acceptance evidence](https://github.com/Herbertofury/INTA/wiki/Service-Evidence)** | Record clean media, restored data and return-to-service proof. |

## ⚡ First three actions

1. Disconnect affected systems from networks and protect reachable backups without destroying useful evidence.
2. Preserve the timeline, ransom note and available logs privately; scope affected accounts and systems with the incident owner.
3. Follow the incident plan for credential containment, clean rebuild and verified restoration before reconnecting.

**No extra download needed.**

**[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-ransomware-response)** · **[Specific fixes for this job](https://herbertofury.github.io/INTA/#job/ransomware-response/quick/contain-ransomware?section=fixes)** · **[Selection evidence](https://github.com/Herbertofury/INTA/wiki/Free-Tool-Picks)**

<a id="next-action"></a>

## 🧭 Match the symptom

| You see this | Do this next | You should get |
| --- | --- | --- |
| **Files are being encrypted or several endpoints are affected** | Isolate network links or the affected switch segment with the incident lead; keep isolated devices powered when feasible for evidence. | Spread is contained and the action time and affected scope are recorded. |
| **Isolation is impossible and damage continues** | Have the incident lead decide whether shutdown is necessary, recording the loss of volatile evidence. | Continuing harm is stopped with an explicit evidence tradeoff. |
| **Backups appear reachable from compromised systems** | Separate their access path and inspect recovery copies from a known-clean administration device. | At least one usable recovery copy is protected from the incident. |
| **A team is ready to restore services** | Agree dependency order, rebuild affected systems from trusted media and test restored business tasks before reconnection. | The incident lead has evidence for release, including account/session containment. |

### ✅ Finish the job

- [ ] Affected systems and identities have a documented scope and owner.
- [ ] Clean restoration passes the actual business workflow.
- [ ] The incident lead authorizes reconnection and records remaining unknowns.

**Change route when:** An action would overwrite evidence or the only recovery copy. · Identity compromise or infrastructure recovery exceeds the responder’s authority or capability.


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
