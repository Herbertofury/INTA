# <a href="https://herbertofury.github.io/INTA/" title="Open INTA official download or service page" aria-label="Open INTA official download or service page" data-inta-brand="app-inta"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA official page" width="18" height="18" valign="middle" /></a> [INTA](https://herbertofury.github.io/INTA/) contributor contract

Read [Knowledge Standards](docs/knowledge-standards.md) before changing the hub, guides or generated wiki. Use [Contributing](CONTRIBUTING.md) for the edit → proof → publication path. Preserve existing content, working features and the current project checkpoint.

For visual work, follow [Interface Design](docs/interface-design.md): a compact, neutral professional workspace informed by the linked <a href="https://github.com/Herbertofury/Athena" title="Open Athena official download or service page" aria-label="Open Athena official download or service page" data-inta-brand="app-athena"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/99f4edb33272b70febb8.png" alt="Athena official page" width="18" height="18" valign="middle" /></a> [Athena](https://github.com/Herbertofury/Athena) references. The rejected green palette, oversized cards, decorative gradients and promotional hero must not return. Preserve every tool, guide, field and interaction while changing their presentation.

Wiki navigation must make every service scenario and tool category directly visible. Use bold, short, clickable action labels with meaningful emoji. Put common scenarios and official tool shortcuts first; never hide these destinations in collapsed groups or require an extra expansion click.

Lead the application Home, Wiki Home, README and Start Here with shared, source-checked free tool picks for common urgent scenarios, direct official links and the matching guide. Preserve the complete catalog below this fast starting point. Record the evidence and current free-use boundary for each pick; do not invent ratings or present a limited test result as a universal ranking.

Each quick-start scenario has an explicit `choices` list in the shared selection data. Give each choice a short task role, useful reason, current access boundary, official evidence and review date. Show every authored choice with its official page and exact acquisition on the front pages and its job's first-view toolkit. Preserve the original scenario-specific action even when other choices are added. Guide-only scenarios explicitly retain an empty list when a download would not help. Never infer prominence from array position in the full catalog, a generic discovery tag or a fixed number of featured tools. Verify the actual first-view malware path includes <a href="https://www.malwarebytes.com/mwb-download" title="Open Malwarebytes official download or service page" aria-label="Open Malwarebytes official download or service page" data-inta-brand="malwarebytes"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4ed5ff877c2070a14163.png" alt="Malwarebytes official page" width="18" height="18" valign="middle" /></a> [Malwarebytes](https://www.malwarebytes.com/), <a href="https://www.malwarebytes.com/adwcleaner" title="Open Malwarebytes AdwCleaner official download or service page" aria-label="Open Malwarebytes AdwCleaner official download or service page" data-inta-brand="adwcleaner"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4ed5ff877c2070a14163.png" alt="Malwarebytes AdwCleaner official page" width="18" height="18" valign="middle" /></a> [AdwCleaner](https://www.malwarebytes.com/adwcleaner) and <a href="https://www.bitdefender.com/en-us/consumer/free-antivirus" title="Open Bitdefender Antivirus Free official download or service page" aria-label="Open Bitdefender Antivirus Free official download or service page" data-inta-brand="bitdefender-free"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/8bc7375d6cd5209e8866.png" alt="Bitdefender Antivirus Free official page" width="18" height="18" valign="middle" /></a> [Bitdefender Antivirus Free](https://www.bitdefender.com/en-us/consumer/free-antivirus), with their different purposes and edition boundaries visible.

Every service scenario needs a visible plain-language answer route. Use labels such as **Remove viruses** and **Fix a printer**. Put exactly three concrete first actions above the full procedure. The Wiki uses a stable dedicated answer page for each quick-start case, with a clear link to the complete guide. Do not rely on an in-page fragment to distinguish cases during cross-page navigation. Preserve existing full-guide URLs and anchors. Home rows need one useful first-action sentence, not repeated selection essays.

Show separate **Official page** and exact acquisition links wherever tools are offered. A file action must identify its platform and file type and come from an observed publisher link. Do not call an HTML landing page a direct download, guess filenames, substitute source code for an installer, or send every built-in troubleshooter to the same problem. Label Store, built-in, package, web and account-specific routes honestly. Preserve alternate platforms and existing routes in details.

Every job has a source-backed working kit before the long procedure: exact tool actions, useful vendor or service links, symptom → action → expected-result decisions, and concrete completion tests. The app keeps the compact kit within reach while reading; the <a href="https://docs.github.com/en/communities/documenting-your-project-with-wikis" title="Open GitHub Wiki official download or service page" aria-label="Open GitHub Wiki official download or service page" data-inta-brand="app-github-wiki"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/b6a41bd1fb24290efbeb.png" alt="GitHub Wiki official page" width="18" height="18" valign="middle" /></a> [GitHub Wiki](https://docs.github.com/en/communities/documenting-your-project-with-wikis) puts the same links first and links to the interactive working view. Test scrolling, keyboard access and small screens so persistent controls never cover instructions.

## Canonical inputs

- [Tool catalog](data/tools.json): stable IDs, official routes, platform and pricing boundaries, editorial review evidence.
- [Tool acquisition routes](data/tool-access.json): official product pages, exact observed files, platform alternatives, scenario action IDs and source evidence.
- [Quick-start selections](data/quick-start.json): scenario-specific free starting tools, practical fit, current sources and evidence for their selection.
- [Job guides](data/jobs.json): procedure, recommended tool IDs, verification, escalation and sources.
- [Job workspaces](data/job-workspaces.json): per-job working kits, vendor resources, diagnostic branches and observable completion criteria.
- [Method registry](data/methods.json): precise names/aliases and canonical method destinations.
- [Link aliases](data/link-aliases.json) and [linking helper](scripts/wiki_links.py): exact names and repeated-mention linking.
- [Wiki layout](data/wiki-layout.json): structured page presentation and the job decision diagrams.
- [Hub metadata](data/meta.json), [portal source](portal/), and authored [documentation](docs/).

The [hub builder](scripts/build-hub.py) owns generated `site/`, `wiki/` and `docs/runbooks/`. Edit their inputs and rebuild. Keep the complete catalog available; performance changes must preserve content, features, search coverage and accessibility.

The builder also owns the quick-start tables between `INTA:QUICK-START` markers in `README.md` and `docs/start-here.md`. Edit their shared selection data instead of changing those generated rows. Text outside the markers remains authored content.

## Required editorial behavior

Every rendered mention of a known tool, application or named method links to its exact official page or canonical guide, including repeated mentions. Preserve existing links, inline code, fenced code and diagram syntax. Add a linked legend immediately after a code example or diagram. Use precise aliases; never link an ambiguous ordinary word to a brand. See the complete [linking and layout rules](docs/knowledge-standards.md).

Show observed evidence and limits. A reachable URL does not prove a current version, a successful build does not prove runtime behavior, and research candidates do not become production recommendations without applicable proof. Keep customer details, credentials and case evidence out of the public project.

## Verify and publish

```sh
python3 scripts/build-hub.py
python3 -m unittest discover -s tests
python3 scripts/build-hub.py --check
python3 scripts/check-wiki.py
```

Command legend: <a href="https://www.python.org/" title="Open Python official download or service page" aria-label="Open Python official download or service page" data-inta-brand="app-python"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/e3ea97f62e4259df2d67.png" alt="Python official page" width="18" height="18" valign="middle" /></a> [Python](https://www.python.org/) runs the [hub builder](scripts/build-hub.py), the [test suite](tests/) and the [wiki checker](scripts/check-wiki.py). Follow [the contribution checklist](docs/contribute.md) for actual browser review, complete artifact publication, live-wiki readback and the final source checkpoint. Use [the maintenance guide](docs/maintaining-the-hub.md) for the exact publishing commands.

Recurring contribution work follows [Continuous Improvement](docs/continuous-improvement.md). It improves the knowledge base with evidence; it does not silently install software on client computers.

## Organized growth

Use the existing service groups in [the canonical layout](data/wiki-layout.json); every job has exactly one primary home. Follow [Knowledge Organization](docs/knowledge-organization.md) and the machine-readable [knowledge policy](data/knowledge-policy.json). Add a specific fault to [field notes](data/field-notes.json) inside its existing job before creating an overlapping page. Keep one tool record per product, with editions/platform downloads in its acquisition alternatives. Preserve existing IDs, anchors, procedures, commands and saved-state behavior.

Maintain an actionable [research queue](data/research-queue.json) for every service job and tool category. Prioritize harmful guidance, broken routes and overdue reviews, then rotate through the least recently reviewed areas. Research current primary sources and add substantial helpful coverage without filler, unsupported rankings or cosmetic date bumps. Every specific fix needs a first action, useful steps, observable result, fallback and official evidence. Run `python3 scripts/check-knowledge.py` alongside the existing checks; review the complete report before choosing the next contribution.
