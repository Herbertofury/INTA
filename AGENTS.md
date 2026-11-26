# INTA contributor contract

Read [Knowledge Standards](docs/knowledge-standards.md) before changing the hub, guides or generated wiki. Use [Contributing](CONTRIBUTING.md) for the edit → proof → publication path. Preserve existing content, working features and the current project checkpoint.

For visual work, follow [Interface Design](docs/interface-design.md): a compact, neutral professional workspace informed by the linked [Athena](https://github.com/Herbertofury/Athena) references. The rejected green palette, oversized cards, decorative gradients and promotional hero must not return. Preserve every tool, guide, field and interaction while changing their presentation.

## Canonical inputs

- [Tool catalog](data/tools.json): stable IDs, official routes, platform and pricing boundaries, editorial review evidence.
- [Job guides](data/jobs.json): procedure, recommended tool IDs, verification, escalation and sources.
- [Method registry](data/methods.json): precise names/aliases and canonical method destinations.
- [Link aliases](data/link-aliases.json) and [linking helper](scripts/wiki_links.py): exact names and repeated-mention linking.
- [Wiki layout](data/wiki-layout.json): structured page presentation and the job decision diagrams.
- [Hub metadata](data/meta.json), [portal source](portal/), and authored [documentation](docs/).

The [hub builder](scripts/build-hub.py) owns generated `site/`, `wiki/` and `docs/runbooks/`. Edit their inputs and rebuild. Keep the complete catalog available; performance changes must preserve content, features, search coverage and accessibility.

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
