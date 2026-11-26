# 🔧 Keep the hub current and consistent

[Home](https://github.com/Herbertofury/INTA/wiki) › [Contribute](https://github.com/Herbertofury/INTA/wiki/Contribute) › **Maintaining the hub**

**Change canonical content once, verify the affected behavior and publish matching surfaces.** Follow [Knowledge Standards](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) for editorial decisions and [Continuous Improvement](https://github.com/Herbertofury/INTA/wiki/Continuous-Improvement) for recurring contribution work.

## 🗂 The content map

| Canonical source | Owns |
| --- | --- |
| [Tool catalog](https://github.com/Herbertofury/INTA/blob/main/data/tools.json) | Names, official URLs, platforms, selection tier, source and review date |
| [Job guides](https://github.com/Herbertofury/INTA/blob/main/data/jobs.json) | Workflows, recommended tools, actions, verification, escalation and sources |
| [Method registry](https://github.com/Herbertofury/INTA/blob/main/data/methods.json) | Precise named methods, aliases, canonical destinations and proof |
| [Link aliases](https://github.com/Herbertofury/INTA/blob/main/data/link-aliases.json) | Additional explicit application and platform names used by the linker |
| [Linking helper](https://github.com/Herbertofury/INTA/blob/main/scripts/wiki_links.py) | Protected [Markdown](https://github.github.com/gfm/) handling and links on every repeated known name |
| [Wiki layout](https://github.com/Herbertofury/INTA/blob/main/data/wiki-layout.json) | Structured wiki presentation and the decision diagrams for all 23 jobs |
| [Hub metadata](https://github.com/Herbertofury/INTA/blob/main/data/meta.json) | Identity and editorial metadata |
| [Portal source](https://github.com/Herbertofury/INTA/tree/main/portal) | Accessible interactive interface |
| [Authored documentation](https://github.com/Herbertofury/INTA/tree/main/docs) | Onboarding, contribution rules, rescue guidance and templates |
| [Hub builder](https://github.com/Herbertofury/INTA/blob/main/scripts/build-hub.py) | Generated site, live-wiki source and rendered playbooks |
| [Wiki checker](https://github.com/Herbertofury/INTA/blob/main/scripts/check-wiki.py) | Wiki integrity, routes and editorial validation |
| [Upstream checker](https://github.com/Herbertofury/INTA/blob/main/scripts/check-upstreams.py) | On-demand official-link and release-status report |
| [Wiki publisher](https://github.com/Herbertofury/INTA/blob/main/scripts/publish-wiki.py) | Synchronization with the actual live wiki repository |

Generated `site/`, `wiki/` and `docs/runbooks/` are reproducible outputs. Preserve stable IDs and useful existing content. Add an explicit alias for a renamed product only when it cannot collide with ordinary language.

## 📅 Before adopting an update

1. Open the original release notes and download route. Confirm edition, operating system and CPU architecture.
2. Apply [publisher and checksum verification](https://github.com/Herbertofury/INTA/wiki/Job-boot-usb) to the exact acquired artifact. Record the resolved version in the private kit inventory.
3. Exercise the specific job on a disposable or agreed test system. Use disposable images for disk-writing tools and verify both endpoints.
4. Compare the result, compatibility, startup behavior and relevant resource cost with the current kit. Apply [stable-release promotion](https://github.com/Herbertofury/INTA/wiki/Job-preventive-maintenance) only when the applicable evidence supports it.
5. Update guidance and `reviewedAt` after real editorial review. Keep release date, retrieval date and runtime-test date distinct.
6. Rebuild, test and publish. Retain a usable previous installer, image or source when needed for recovery.

## 🔎 Run a freshness check on demand

```sh
python3 scripts/check-upstreams.py --output reports/upstreams.json
```

Command legend: [Python](https://www.python.org/) runs the [upstream checker](https://github.com/Herbertofury/INTA/blob/main/scripts/check-upstreams.py). The resulting report supports [manual freshness review](https://github.com/Herbertofury/INTA/wiki/Job-preventive-maintenance).

The checker reports each tool. For projects with [GitHub releases](https://docs.github.com/en/repositories/releasing-projects-on-github/about-releases), it queries stable-release metadata and preserves explicit results when no release exists or access fails. Other vendors receive official-link reachability and a manual-version-review status. A reachable URL never becomes an invented version, and the report does not install software or silently rewrite the catalog.

The **Check tool upstreams** [workflow](https://github.com/Herbertofury/INTA/actions/workflows/upstreams.yml) remains on demand. The enabled weekday knowledge-maintenance task is documented separately in [Continuous Improvement](https://github.com/Herbertofury/INTA/wiki/Continuous-Improvement). The existing [rescue build workflow](https://github.com/Herbertofury/INTA/actions/workflows/rescue.yml) and [INTA Media Creator](https://github.com/Herbertofury/INTA/blob/main/media/inta-media.py) retain their own image and refresh lifecycles.

## 🧪 Build and inspect the affected path

```sh
python3 scripts/build-hub.py
python3 -m unittest discover -s tests
python3 scripts/build-hub.py --check
python3 scripts/check-wiki.py
```

Command legend: [Python](https://www.python.org/) runs the [hub builder](https://github.com/Herbertofury/INTA/blob/main/scripts/build-hub.py), [test suite](https://github.com/Herbertofury/INTA/tree/main/tests) and [wiki checker](https://github.com/Herbertofury/INTA/blob/main/scripts/check-wiki.py). Preserve any failed result as an open gate until resolved.

Open `site/index.html` directly or serve the `site` folder locally. Review the changed page in the actual rendered interface. Where affected, exercise full-catalog search, combined filters, job checkboxes, saved tools, export/print, both themes, keyboard focus and a narrow screen. Follow repeated named links and inspect diagrams and their linked legends. Keep every entry available; do not add quantity caps, viewport virtualization or content culling.

## 🚀 Publish the actual wiki

Use a [Git](https://git-scm.com/) session authorized for the canonical wiki remote:

```sh
python3 scripts/publish-wiki.py --checkout ../INTA.wiki
```

Command legend: [Python](https://www.python.org/) runs the [wiki publisher](https://github.com/Herbertofury/INTA/blob/main/scripts/publish-wiki.py); [Git](https://git-scm.com/) supplies the authenticated checkout of the [live wiki](https://github.com/Herbertofury/INTA/wiki).

The publisher checks the exact remote, fetches current content, copies generated pages while preserving unrelated pages, and uses a normal fast-forward commit. Keep authentication with the credential mechanism, never inside source or documentation. Use an authorized maintainer checkout if an automated token cannot reach the wiki repository.

The [hub workflow](https://github.com/Herbertofury/INTA/actions/workflows/hub.yml) deploys the validated site to [GitHub Pages](https://docs.github.com/en/pages/getting-started-with-github-pages/using-custom-workflows-with-github-pages). Publish the exact tested source checkpoint, matching generated site and live-wiki source. Save the complete deliverable to the project's connected [Google Drive](https://drive.google.com/) destination, then verify the returned identity and relevant content.

## ✅ Read back before closure

| Surface | What to verify |
| --- | --- |
| Canonical repository | The published commit is the tested source revision. |
| Hosted hub | The changed page or interaction shows the intended content. |
| Actual live wiki | The pages, navigation, links and diagrams match generated source. |
| Complete artifact copy | The archive contains the full deliverable and belongs to the same revision. |
| Offline copy | The full embedded content works without requiring a network just to read it. |

Record results in [the contribution template](https://github.com/Herbertofury/INTA/wiki/Templates). Downloaded offline copies retain their build snapshot; replace the HTML when you want updated guidance.

## 📚 Maintainer references

- [GitHub: adding and editing wiki pages](https://docs.github.com/en/communities/documenting-your-project-with-wikis/adding-or-editing-wiki-pages)
- [GitHub: custom Pages workflows](https://docs.github.com/en/pages/getting-started-with-github-pages/using-custom-workflows-with-github-pages)
- [GitHub: diagram support](https://docs.github.com/en/get-started/writing-on-github/working-with-advanced-formatting/creating-diagrams)
- [GitHub: writing and formatting](https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax)

**➜ Continue:** [Contribution Guide](https://github.com/Herbertofury/INTA/wiki/Contribute) · [Continuous Improvement](https://github.com/Herbertofury/INTA/wiki/Continuous-Improvement) · [Knowledge Standards](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards)
