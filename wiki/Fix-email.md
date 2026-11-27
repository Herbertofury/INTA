[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Service map](https://github.com/Herbertofury/INTA/wiki/Service-Map) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

# ✉️ Fix email

**Check webmail and identify whether the affected app is classic [Outlook](https://support.microsoft.com/outlook) or new [Outlook](https://support.microsoft.com/outlook) before changing a profile.**

<a id="working-kit"></a>

## 🧰 Tools and links for this job

**[Open working view](https://herbertofury.github.io/INTA/#job/outlook-email/quick/fix-email)** · Keep the tool panel beside the instructions.

**First move:** Check webmail and identify whether the affected app is classic [Outlook](https://support.microsoft.com/outlook) or new [Outlook](https://support.microsoft.com/outlook) before changing a profile.

| Use this for | Official tool + exact action |
| --- | --- |
| Fix email | **[Windows Get Help](https://support.microsoft.com/en-us/support/get-help/about-get-help)** · [Official page](https://support.microsoft.com/en-us/support/get-help/about-get-help) · **[Choose a classic Outlook diagnostic](https://support.microsoft.com/en-us/support/get-help/classic-outlook-troubleshooters)** |
| Separating DNS, TCP, TLS and HTTP failures when a web service or application endpoint is unreachable. | **[curl](https://curl.se/)** · [Official page](https://curl.se/) · **[Use the Windows-included curl.exe](https://curl.se/windows/microsoft.html)** |

### Client repair

| Open the exact resource | What to do there |
| --- | --- |
| **[Classic Outlook diagnostics](https://support.microsoft.com/en-us/support/get-help/classic-outlook-troubleshooters)** | Choose startup, authentication or connectivity; not for new [Outlook](https://support.microsoft.com/outlook). |
| **[New and classic Outlook repair branches](https://support.microsoft.com/en-us/outlook/getstarted/outlook-for-windows-not-responding-hangs-freezes-or-stops-working)** | Use the tab matching the installed client. |

### Compatibility

| Open the exact resource | What to do there |
| --- | --- |
| **[New Outlook architecture and differences](https://learn.microsoft.com/en-us/microsoft-365-apps/outlook/administration/architecture-changes-new-outlook)** | Check the client’s supported behavior before applying a classic-only fix. |

## ⚡ First three actions

1. Test sign-in and send/receive in webmail; record the exact error and whether other users are affected.
2. For classic [Outlook](https://support.microsoft.com/outlook), run the matching [Windows Get Help](https://support.microsoft.com/en-us/support/get-help/about-get-help) diagnostic; for new [Outlook](https://support.microsoft.com/outlook), use its supported troubleshooting branch.
3. Retest send, receive and the affected shared mailbox or calendar; preserve local-only mail before any profile rebuild.

**[Windows Get Help](https://support.microsoft.com/en-us/support/get-help/about-get-help)** · **[Choose a classic Outlook diagnostic](https://support.microsoft.com/en-us/support/get-help/classic-outlook-troubleshooters)**

**[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-outlook-email)** · **[Specific fixes for this job](https://herbertofury.github.io/INTA/#job/outlook-email/quick/fix-email?section=fixes)** · **[Selection evidence](https://github.com/Herbertofury/INTA/wiki/Free-Tool-Picks)**

<a id="next-action"></a>

## 🧭 Match the symptom

| You see this | Do this next | You should get |
| --- | --- | --- |
| **Webmail also fails or several users are affected** | Capture the sign-in/send error and compare another affected account; involve the mail or identity service owner. | The fault is identified as service, account or policy scope rather than a local profile guess. |
| **Classic [Outlook](https://support.microsoft.com/outlook) will not start or repeatedly asks for credentials** | Open the matching classic [Outlook](https://support.microsoft.com/outlook) diagnostic in [Windows Get Help](https://support.microsoft.com/en-us/support/get-help/about-get-help) on the affected PC. | The test identifies a startup, authentication or connectivity finding. |
| **Only classic [Outlook](https://support.microsoft.com/outlook) fails and an add-in is suspected** | Press Win+R, run [outlook](https://support.microsoft.com/outlook).exe /safe and compare the same task; record enabled add-ins before changing one. | The safe-mode comparison establishes whether the add-in branch is relevant. |
| **New [Outlook](https://support.microsoft.com/outlook) is the affected client** | Preserve unsent work where possible; in [Task Manager](https://support.microsoft.com/en-us/windows/experience/system-configuration-tools-in-windows) select Microsoft [Outlook](https://support.microsoft.com/outlook) > End task, then reopen new [Outlook](https://support.microsoft.com/outlook) and repeat the failing task. If it still fails, follow Microsoft’s new-client reinstall branch after protecting local-only mail. | The supported new-client path restores the failing task or identifies its service dependency. |
| **A profile rebuild or data-file repair is proposed** | Identify PST/local-only mail and preserve it before changing the profile or file. | No unique mail is lost while the repaired client is tested. |

### ✅ Finish the job

- [ ] Send and receive succeed for the affected account.
- [ ] The affected calendar, shared mailbox or attachment workflow works.
- [ ] Local-only data is preserved and the actual client/version is recorded.

**Change route when:** Local-only mail lacks a protected copy. · The fault involves tenant policy, identity compromise or a wider service outage.


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
