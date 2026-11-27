<a href="https://herbertofury.github.io/INTA/" title="Open INTA official download or service page" aria-label="Open INTA official download or service page" data-inta-brand="app-inta"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA official page" width="18" height="18" valign="middle" /></a> [INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Service map](https://github.com/Herbertofury/INTA/wiki/Service-Map) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

# 🗂️ Keep <a href="https://herbertofury.github.io/INTA/" title="Open INTA official download or service page" aria-label="Open INTA official download or service page" data-inta-brand="app-inta"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA official page" width="18" height="18" valign="middle" /></a> [INTA](https://herbertofury.github.io/INTA/) organized as it grows

[Home](https://github.com/Herbertofury/INTA/wiki) › **Knowledge organization**

**Find the job, use the answer, improve the same trusted place.** The [Service map](https://github.com/Herbertofury/INTA/wiki/Service-Map) gives every guide one primary home. The [Tool directory](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) gives every application one canonical record. Specific faults belong inside their existing guide, with direct links from the map and search.

**[🗺️ Find a service](https://herbertofury.github.io/INTA/#services)** · **[🎯 Choose a quick answer](https://github.com/Herbertofury/INTA/wiki)** · **[🧭 Check coverage and reviews](https://github.com/Herbertofury/INTA/wiki/Knowledge-Health)** · **[🔎 Research next](https://github.com/Herbertofury/INTA/wiki/Research-Queue)**

## Where each thing belongs

| Add this | Its authoritative home | Users receive |
| --- | --- | --- |
| First answer to a common problem | [Quick-start selections](https://github.com/Herbertofury/INTA/blob/main/data/quick-start.json) | A visible scenario, first action, three steps and exact tool route |
| Complete service procedure | [Job guides](https://github.com/Herbertofury/INTA/blob/main/data/jobs.json) | Ordered work, verification and escalation with stable steps |
| Particular symptom or failure | [Specific fixes](https://github.com/Herbertofury/INTA/blob/main/data/field-notes.json) | A direct case link, concrete actions, expected result and fallback inside the correct job |
| Driver portals and service resources | [Job workspaces](https://github.com/Herbertofury/INTA/blob/main/data/job-workspaces.json) | The persistent working toolkit and official resources |
| Application or built-in capability | [Tool catalog](https://github.com/Herbertofury/INTA/blob/main/data/tools.json) and [acquisitions](https://github.com/Herbertofury/INTA/blob/main/data/tool-access.json) | One identity with official page, exact downloads, platforms and access limits |
| Reusable named technique | [Method registry](https://github.com/Herbertofury/INTA/blob/main/data/methods.json) | One precise destination at every mention |
| Future investigation | [Research queue](https://github.com/Herbertofury/INTA/blob/main/data/research-queue.json) | A question, useful output and evidence needed to finish |
| Organization or review rule | [Knowledge policy](https://github.com/Herbertofury/INTA/blob/main/data/knowledge-policy.json) | Shared rules enforced by checks and shown in the health report |

The existing [layout groups](https://github.com/Herbertofury/INTA/blob/main/data/wiki-layout.json) are the only service taxonomy. Related cases can link across several fields without copying the procedure. Editions and platform installers stay in one acquisition record unless they are genuinely distinct products.

The `choices` list in each [quick-start selection](https://github.com/Herbertofury/INTA/blob/main/data/quick-start.json) controls which tools appear immediately for that case. It references canonical tool and acquisition IDs and adds only the task role, reason, access boundary and source evidence. That same list feeds the front pages and initial job toolkit. A tool being present in the complete catalog is not enough to make it discoverable during the relevant job. See [current choices by scenario](https://github.com/Herbertofury/INTA/wiki/Current-Tool-Choices).

## Add useful depth in the right place

1. **Find the existing job.** Search the service map, case titles, tool IDs and official product pages.
2. **Extend that home.** Add the missing diagnostic case, step, resource or acquisition alternative. A new guide needs a distinct job, complete procedure and acceptance test.
3. **Prove the useful difference.** State the starting condition, exact first action, result interpretation and fallback. Keep official evidence and its limits.
4. **Publish the same change everywhere.** Rebuild and check the app, Wiki and offline copy; publish the source and actual Wiki, then verify the user path.

This uses the distinction in <a href="https://diataxis.fr/" title="Open diataxis.fr official download or service page" aria-label="Open diataxis.fr official download or service page" data-inta-brand="site-diataxis-fr"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/b8b16a3e1b0b02253b30.png" alt="diataxis.fr official page" width="18" height="18" valign="middle" /></a> [Diátaxis practical guides](https://diataxis.fr/how-to-guides/): task instructions help complete a job, while reference material supplies broader options. <a href="https://herbertofury.github.io/INTA/" title="Open INTA official download or service page" aria-label="Open INTA official download or service page" data-inta-brand="app-inta"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA official page" width="18" height="18" valign="middle" /></a> [INTA](https://herbertofury.github.io/INTA/) keeps its existing architecture and stable routes.

## Automatic checks prevent drift

The [organization checker](https://github.com/Herbertofury/INTA/blob/main/scripts/check-knowledge.py) runs with normal build checks. It rejects duplicate normalized tool names, unexplained duplicate official product pages, unknown references, missing service homes, empty specific fixes, unsafe source URLs and gaps in the research queue. A legitimate shared-product-page exception needs explicit tool IDs and a reason in the policy.

Every fix needs a symptom, first action, concrete steps, observable result, fallback, known tools, official evidence and a real review date. Every job needs specific fixes. Every job and tool category needs an actionable research entry. The [Wiki checker](https://github.com/Herbertofury/INTA/blob/main/scripts/check-wiki.py) also checks repeated named links, internal destinations, command legends and diagrams.

**Structure checks support editorial judgment.** Before publication, try the actual search-and-fix path. A valid entry still needs to reduce uncertainty for a technician.

## Keep research balanced and reviews honest

The [knowledge health page](https://github.com/Herbertofury/INTA/wiki/Knowledge-Health) is a dated coverage snapshot. The [scheduled repository check](https://github.com/Herbertofury/INTA/actions/workflows/knowledge-health.yml) recalculates age using its run date and stores the complete report. Scheduled execution follows <a href="https://github.com/" title="Open GitHub official download or service page" aria-label="Open GitHub official download or service page" data-inta-brand="app-github"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/99f4edb33272b70febb8.png" alt="GitHub official page" width="18" height="18" valign="middle" /></a> [GitHub's documented scheduling behavior](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows#schedule); configuration does not prove a future run finished.

The existing **Grow the <a href="https://herbertofury.github.io/INTA/" title="Open INTA official download or service page" aria-label="Open INTA official download or service page" data-inta-brand="app-inta"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA official page" width="18" height="18" valign="middle" /></a> [INTA](https://herbertofury.github.io/INTA/) Wiki** task continues on weekdays around **8:00 AM, America/Denver**. Choose harmful guidance, broken routes and overdue high-impact reviews first. Then rotate through the least recently reviewed fields, resolving the concrete questions in the research queue and recording adopt, retain, specialist, evaluate or unresolved decisions.

A loaded page, successful link check or rebuild never advances a review date. Read the relevant source, assess what changed and record the decision. New, free, popular and suitable for this client are separate claims. Keep preview status and business-use limits visible.

## Preserve useful work

Keep old IDs, guide URLs, stable anchors, commands, sources, saved items and checklist state. Explain replacements and retain useful migration routes when a product is superseded. Canonical source drives the generated site and Wiki; a manual generated-page edit will be overwritten by the next build.

Customer names, credentials, device identifiers and raw case evidence stay in the approved private service record. Public notes use sanitized examples. Close contributions with the source revision, actual verification, published destinations and exact remaining action.

**Continue:** [Contribute](https://github.com/Herbertofury/INTA/wiki/Contribute) · [Knowledge standards](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Continuous improvement](https://github.com/Herbertofury/INTA/wiki/Continuous-Improvement)

---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
