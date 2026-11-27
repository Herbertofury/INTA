# 🛠 Make a useful contribution

[Home](https://github.com/Herbertofury/INTA/wiki) › **Contribute**

**Improve one complete technician outcome, then publish the evidence with it.** Use [Knowledge Standards](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) for the writing and linking rules, and [Templates](https://github.com/Herbertofury/INTA/wiki/Templates) to keep the change reviewable.

## 🧭 Choose the kind of improvement

| You found… | Make this contribution | Show this proof |
| --- | --- | --- |
| A moved or misleading official link | Correct its canonical catalog or method entry and all affected routes. | The destination is the intended publisher page and rendered mentions point there. |
| A confusing step | Rewrite the starting condition, action, expected observation and fallback. | A technician can select the next action without guessing the environment. |
| A better tool candidate | Add its job fit, platform, operating mode, pricing context, sources and limits. | The documented capability is accurate; any claimed runtime advantage has a real comparison. |
| A missing decision | Add a small decision table or <a href="https://mermaid.js.org/" title="Open Mermaid official download or service page" aria-label="Open Mermaid official download or service page" data-inta-brand="app-mermaid"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/c733be15cbccdcb88240.png" alt="Mermaid official page" width="18" height="18" valign="middle" /></a> [Mermaid](https://mermaid.js.org/intro/) diagram. | Every branch has a useful next guide and no branch implies an unverified repair. |
| A new release | Perform a [manual freshness review](https://github.com/Herbertofury/INTA/wiki/Job-preventive-maintenance). | The applicable release and compatibility decision are recorded separately from URL reachability. |
| A repeated field lesson | Generalize it into a guide or named method. | The public example is sanitized, the original evidence stays private, and the method has an acceptance test. |

## 🗂 Edit once, improve every surface

| Canonical file | Author here |
| --- | --- |
| [Tool catalog](https://github.com/Herbertofury/INTA/blob/main/data/tools.json) | Tool identity, official links, usage boundary, tier, tags and review evidence |
| [Quick-start selections](https://github.com/Herbertofury/INTA/blob/main/data/quick-start.json) | Free starting tools for urgent scenarios, task fit, source evidence and practical limits; shared by the front pages |
| [Job guides](https://github.com/Herbertofury/INTA/blob/main/data/jobs.json) | Procedures, selected tool IDs, verification, escalation and sources |
| [Job workspaces](https://github.com/Herbertofury/INTA/blob/main/data/job-workspaces.json) | Purpose-specific toolkits, official action resources, symptom decisions and completion evidence |
| [Specific fixes](https://github.com/Herbertofury/INTA/blob/main/data/field-notes.json) | Distinct fault cases inside existing jobs, concrete actions, proof, fallback and official evidence |
| [Knowledge policy](https://github.com/Herbertofury/INTA/blob/main/data/knowledge-policy.json) and [research queue](https://github.com/Herbertofury/INTA/blob/main/data/research-queue.json) | Shared organization rules, real review intervals and concrete next investigations for every field |
| [Method registry](https://github.com/Herbertofury/INTA/blob/main/data/methods.json) | Precise method names, aliases, canonical destinations and proof |
| [Link aliases](https://github.com/Herbertofury/INTA/blob/main/data/link-aliases.json) and [linking helper](https://github.com/Herbertofury/INTA/blob/main/scripts/wiki_links.py) | Additional precise application names and linking of every repeated mention |
| [Wiki layout](https://github.com/Herbertofury/INTA/blob/main/data/wiki-layout.json) | Page presentation and job decision diagrams |
| [Authored documentation](https://github.com/Herbertofury/INTA/tree/main/docs) | Onboarding, decisions, standards, contribution guidance and service templates |
| [Portal source](https://github.com/Herbertofury/INTA/tree/main/portal) | Actual interface and interaction changes |
| [Hub builder](https://github.com/Herbertofury/INTA/blob/main/scripts/build-hub.py) | Consistent generation of the site, wiki and rendered guides |

Keep existing stable IDs when the identity is unchanged. A product rename may need an alias and a moved official URL; it does not justify breaking saved references. Generated `site/`, `wiki/` and `docs/runbooks/` are outputs, so repair their source instead of leaving a one-surface patch.

## 🧪 Follow the evidence path

1. **Record the actual problem.** State the affected job and what a technician cannot currently do or understand. Identify the canonical source revision and reuse the known project state.
2. **Check the decisive source.** Read the original project, publisher or platform documentation. Separate observed capability, your inference and an untested candidate. Avoid broad rediscovery once the next edit is known.
3. **Make a coherent change.** Include the complete procedure or catalog entry, the appropriate links and the actual success test. Preserve existing useful content.
4. **Review every named route.** Repeated tool, application and method mentions must remain clickable. Check aliases, headings, tables, lists and the linked legends immediately following command blocks and diagrams.
5. **Build and target the risk.** Check generation and the meaningful behavior affected by the change. A link edit needs destination and generated-output checks; a UI interaction needs a real browser exercise; a new operational claim needs corresponding evidence.
6. **Challenge once.** Look for one material omitted platform boundary, stale source, incorrect next step, missing recovery path or rendering regression. Resolve significant findings, then finish the accepted scope.
7. **Publish and read back.** Preserve the source checkpoint and matching complete outputs, then confirm the deployed destinations show the intended content.

## 🧰 Local build commands

```sh
python3 scripts/build-hub.py
python3 -m unittest discover -s tests
python3 scripts/build-hub.py --check
python3 scripts/check-wiki.py
python3 scripts/check-knowledge.py --output reports/knowledge-health.json
```

Command legend: <a href="https://www.python.org/" title="Open Python official download or service page" aria-label="Open Python official download or service page" data-inta-brand="app-python"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/e3ea97f62e4259df2d67.png" alt="Python official page" width="18" height="18" valign="middle" /></a> [Python](https://www.python.org/) runs the [hub builder](https://github.com/Herbertofury/INTA/blob/main/scripts/build-hub.py), [test suite](https://github.com/Herbertofury/INTA/tree/main/tests) and [wiki checker](https://github.com/Herbertofury/INTA/blob/main/scripts/check-wiki.py). The checks verify generated outputs, known named links, routes and linked diagram or command legends.

For a content change, inspect the relevant rendered page and follow its actual links. For a visual or interaction change, check desktop and narrow layouts, keyboard navigation, both themes, complete search results, checklists, saved items and exports where affected. Check the offline build as well as the hosted surface when the change touches offline behavior. For working kits, verify every manufacturer/resource link is present, the exact case survives navigation, sticky controls remain usable during real scrolling, and narrow layouts do not obscure the instructions.

## 🚀 Definition of finished

- [ ] The user-visible improvement is complete in canonical source.
- [ ] Every factual claim has an appropriate source and its evidence level is clear.
- [ ] Named links, commands, diagram legends and navigation were reviewed.
- [ ] Generation and the relevant behavior checks passed.
- [ ] Significant challenge findings were resolved.
- [ ] The exact source checkpoint is on the canonical remote.
- [ ] The matching site and actual live wiki were published and read back.
- [ ] The complete deliverable was saved to the project's connected <a href="https://drive.google.com/" title="Open Google Drive official download or service page" aria-label="Open Google Drive official download or service page" data-inta-brand="app-google-drive"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4a3cfc17be3f4e8ad2c8.png" alt="Google Drive official page" width="18" height="18" valign="middle" /></a> [Google Drive](https://drive.google.com/) destination and its identity verified.
- [ ] The contribution record includes proof, limits, published locations and any remaining next action.

Use [Maintaining the Hub](https://github.com/Herbertofury/INTA/wiki/Maintaining-the-Hub) for publication commands and [Continuous Improvement](https://github.com/Herbertofury/INTA/wiki/Continuous-Improvement) for recurring work. If an external gate fails, retain the last verified checkpoint and continue on a materially different supported route. Do not label an unpublished or unverified required surface complete.

**➜ Continue:** [Copy the contribution template](https://github.com/Herbertofury/INTA/wiki/Templates) · [Choose a method](https://github.com/Herbertofury/INTA/wiki/Methods) · [Report a correction](https://github.com/Herbertofury/INTA/issues/new/choose)

## ⚡ Add a useful quick command

Every playbook needs at least one command placed in the step where it belongs. Add shell, purpose, safety, official reference URL and reference title beside the command in `data/jobs.json`. Prefer read-only discovery before state changes; include the condition that justifies a repair, its prerequisites, rollback and a verification step. The interactive hub surfaces the first relevant command at the top of the guide and keeps the full command at its procedure step. Follow [Quick Command Standards](https://github.com/Herbertofury/INTA/blob/main/docs/quick-commands.md).


## ⚡ Commands inside specific fixes

When a shell action helps a specific symptom, add a `commands` array to that exact record in `data/field-notes.json`. Keep each block focused and include shell, purpose, safety, code and official source. The interactive article and generated Wiki render the same copyable block. Use the [Quick Commands Wiki](https://github.com/Herbertofury/INTA/wiki/Quick-Commands) for standards and trusted tool installation patterns.
