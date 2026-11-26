# 🛠 Contribute to the INTA technician home

**Start with [Knowledge Standards](docs/knowledge-standards.md), then use the [Contribution Guide](docs/contribute.md) and [Contribution Template](docs/contribution-template.md).**

Contributions should help a technician choose a suitable tool, understand the next action, or prove that the customer's original task works. A small verified correction is useful. A large addition needs the same clear sources, navigation and proof.

## 🧭 Edit the source of truth

| Change | Canonical input |
| --- | --- |
| Tool, official download, platform, pricing or review evidence | [Tool catalog](data/tools.json) |
| Procedure, tool selection, commands, success checks or escalation | [Job guides](data/jobs.json) |
| Named troubleshooting method and its canonical destination | [Method registry](data/methods.json) |
| Additional precise application names or repeated-mention linking | [Link aliases](data/link-aliases.json) and [linking helper](scripts/wiki_links.py) |
| Wiki page presentation or job decision diagrams | [Wiki layout](data/wiki-layout.json) |
| Wiki editorial rules, onboarding or service templates | Authored [documentation](docs/) |
| Navigation, interaction or visual behavior | [Portal source](portal/) and [hub builder](scripts/build-hub.py) |

Generated `wiki/`, `site/` and `docs/runbooks/` are build outputs. Change the appropriate input so the live hub, offline copy and wiki remain consistent.

## 🧪 Make the change reviewable

1. Describe the technician problem and the smallest complete improvement.
2. Use original publisher or project documentation for factual claims. Record what was inspected and what was exercised.
3. Apply the [Knowledge Standards](docs/knowledge-standards.md), including links on repeated tool and method mentions, meaningful section icons, a clear next action and honest evidence dates.
4. Rebuild and run the checks that prove the changed behavior. For visual changes, inspect the rendered page on desktop and a narrow screen.
5. Record the source revision, test result, remaining limits and publication evidence using the [Contribution Template](docs/contribution-template.md).

## 🧰 Build and validate

```sh
python3 scripts/build-hub.py
python3 -m unittest discover -s tests
python3 scripts/build-hub.py --check
python3 scripts/check-wiki.py
```

Command legend: [Python](https://www.python.org/) runs the [hub builder](scripts/build-hub.py), [test suite](tests/) and [wiki checker](scripts/check-wiki.py). Inspect the changed rendered path as described in the [Contribution Guide](docs/contribute.md).

## 🚀 Finish the whole contribution

Follow [the maintenance guide](docs/maintaining-the-hub.md) to publish the exact source checkpoint, matching site, actual live wiki and complete artifact copy. Read back the deployed result. Keep failed or unverified gates visible until they are resolved.

Use the existing [issue forms](https://github.com/Herbertofury/INTA/issues/new/choose) for a suggestion or correction. Include the job, official source, expected behavior and a reproducible observation. Generic examples belong here; customer-specific evidence belongs in the approved private service record.

**Continue:** [Choose an improvement](docs/continuous-improvement.md) · [Browse the decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Read the standards](docs/knowledge-standards.md)
