# 🛠 Contribute to the <a href="https://herbertofury.github.io/INTA/" title="Open INTA official download or service page" aria-label="Open INTA official download or service page" data-inta-brand="app-inta"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA official page" width="18" height="18" valign="middle" /></a> [INTA](https://herbertofury.github.io/INTA/) ecosystem

**Start with [Knowledge Standards](docs/knowledge-standards.md) and [Interface Design](docs/interface-design.md), then use the [Contribution Guide](docs/contribute.md) and [Contribution Template](docs/contribution-template.md).**

Start by selecting the affected lane in [Ecosystem Development](docs/ecosystem-development.md): <a href="https://herbertofury.github.io/INTA/" title="Open INTA official download or service page" aria-label="Open INTA official download or service page" data-inta-brand="app-inta"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA official page" width="18" height="18" valign="middle" /></a> [INTA](https://herbertofury.github.io/INTA/) Desktop, <a href="https://herbertofury.github.io/INTA/" title="Open INTA official download or service page" aria-label="Open INTA official download or service page" data-inta-brand="app-inta"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA official page" width="18" height="18" valign="middle" /></a> [INTA](https://herbertofury.github.io/INTA/) Mobile, <a href="https://github.com/Herbertofury/INTA/releases" title="Open INTA Rescue official download or service page" aria-label="Open INTA Rescue official download or service page" data-inta-brand="app-inta-rescue"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA Rescue official page" width="18" height="18" valign="middle" /></a> [INTA Rescue](https://github.com/Herbertofury/INTA/blob/main/docs/rescue.md), or <a href="https://herbertofury.github.io/INTA/" title="Open INTA official download or service page" aria-label="Open INTA official download or service page" data-inta-brand="app-inta"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA official page" width="18" height="18" valign="middle" /></a> [INTA](https://herbertofury.github.io/INTA/) Site. For shared changes, name all affected surfaces and test their real integration before calling the milestone done.

Contributions should help a technician choose a suitable tool, understand the next action, or prove that the customer's original task works. A small verified correction is useful. A large addition needs the same clear sources, navigation and proof.

## 🧭 Edit the source of truth

| Change | Canonical input |
| --- | --- |
| Tool, official download, platform, pricing or review evidence | [Tool catalog](data/tools.json) |
| Exact file downloads, built-in launches, Store routes or acquisition evidence | [Tool acquisition routes](data/tool-access.json) |
| Plain-language problem links, first actions or a scenario-specific tool action | [Quick-start answers](data/quick-start.json) |
| Procedure, [tool selection](https://github.com/Herbertofury/INTA/wiki/Tool-Selection), commands, success checks or escalation | [Job guides](data/jobs.json) |
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

Walk the changed route as a person who needs an answer now: recognize the problem, open the exact tool or built-in action, and follow the first three steps. Keep official-page and acquisition actions visible together. Check the response destination and file identity before labelling a link a direct download. Test scenario-specific launch links independently when several jobs use the same application.

## 🧰 Build and validate

```sh
python3 scripts/build-hub.py
python3 -m unittest discover -s tests
python3 scripts/build-hub.py --check
python3 scripts/check-wiki.py
```

Command legend: <a href="https://www.python.org/" title="Open Python official download or service page" aria-label="Open Python official download or service page" data-inta-brand="app-python"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/e3ea97f62e4259df2d67.png" alt="Python official page" width="18" height="18" valign="middle" /></a> [Python](https://www.python.org/) runs the [hub builder](scripts/build-hub.py), [test suite](tests/) and [wiki checker](scripts/check-wiki.py). Inspect the changed rendered path as described in the [Contribution Guide](docs/contribute.md).

## 🚀 Finish the whole contribution

Publish the exact source checkpoint and the deliverables for each affected lane. Follow [the maintenance guide](docs/maintaining-the-hub.md) for Site, Wiki and archive. For Desktop, Mobile or Rescue, also record the matching package or image and its actual runtime proof. Read back every claimed published result; keep unverified gates visible.

Use the existing [issue forms](https://github.com/Herbertofury/INTA/issues/new/choose) for a suggestion or correction. Include the job, official source, expected behavior and a reproducible observation. Generic examples belong here; customer-specific evidence belongs in the approved private service record.

**Continue:** [Choose an improvement](docs/continuous-improvement.md) · [Browse the decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Read the standards](docs/knowledge-standards.md)
