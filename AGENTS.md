# INTA contributor contract

Read [Knowledge Standards](docs/knowledge-standards.md) before changing the hub, guides or generated wiki. Use [Contributing](CONTRIBUTING.md) for the edit → proof → publication path. Preserve existing content, working features and the current project checkpoint.

For visual work, follow [Interface Design](docs/interface-design.md): a compact, neutral professional workspace informed by the linked [Athena](https://github.com/Herbertofury/Athena) references. The rejected green palette, oversized cards, decorative gradients and promotional hero must not return. Preserve every tool, guide, field and interaction while changing their presentation.

Wiki navigation must make every service scenario and tool category directly visible. Use bold, short, clickable action labels with meaningful emoji. Put common scenarios and official tool shortcuts first; never hide these destinations in collapsed groups or require an extra expansion click.

Lead the application Home, Wiki Home, README and Start Here with shared, source-checked free tool picks for common urgent scenarios, direct official links and the matching guide. Preserve the complete catalog below this fast starting point. Record the evidence and current free-use boundary for each pick; do not invent ratings or present a limited test result as a universal ranking.

Every service scenario needs a visible plain-language answer route. Use labels such as **Remove viruses** and **Fix a printer**. Put exactly three concrete first actions above the full procedure. Shared guides must retain distinct case links so a recovery or network shortcut opens the intended answer. Home rows need one useful first-action sentence, not repeated selection essays.

Show separate **Official page** and exact acquisition links wherever tools are offered. A file action must identify its platform and file type and come from an observed publisher link. Do not call an HTML landing page a direct download, guess filenames, substitute source code for an installer, or send every built-in troubleshooter to the same problem. Label Store, built-in, package, web and account-specific routes honestly. Preserve alternate platforms and existing routes in details.

## Canonical inputs

- [Tool catalog](data/tools.json): stable IDs, official routes, platform and pricing boundaries, editorial review evidence.
- [Tool acquisition routes](data/tool-access.json): official product pages, exact observed files, platform alternatives, scenario action IDs and source evidence.
- [Quick-start selections](data/quick-start.json): scenario-specific free starting tools, practical fit, current sources and evidence for their selection.
- [Job guides](data/jobs.json): procedure, recommended tool IDs, verification, escalation and sources.
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

Command legend: [Python](https://www.python.org/) runs the [hub builder](scripts/build-hub.py), the [test suite](tests/) and the [wiki checker](scripts/check-wiki.py). Follow [the contribution checklist](docs/contribute.md) for actual browser review, complete artifact publication, live-wiki readback and the final source checkpoint. Use [the maintenance guide](docs/maintaining-the-hub.md) for the exact publishing commands.

Recurring contribution work follows [Continuous Improvement](docs/continuous-improvement.md). It improves the knowledge base with evidence; it does not silently install software on client computers.
