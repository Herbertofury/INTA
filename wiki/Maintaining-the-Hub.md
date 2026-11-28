<a href="https://herbertofury.github.io/INTA/" title="Open INTA official download or service page" aria-label="Open INTA official download or service page" data-inta-brand="app-inta"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA official page" width="18" height="18" valign="middle" /></a> [INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Service map](https://github.com/Herbertofury/INTA/wiki/Service-Map) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

# 🔧 Keep the hub current and consistent

[Home](https://github.com/Herbertofury/INTA/wiki) › [Contribute](https://github.com/Herbertofury/INTA/wiki/Contribute) › **Maintaining the hub**

**Change canonical content once, verify the affected behavior and publish matching surfaces.** Follow [Knowledge Standards](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) for editorial decisions and [Continuous Improvement](https://github.com/Herbertofury/INTA/wiki/Continuous-Improvement) for recurring contribution work.

## 🗂 The content map

| Canonical source | Owns |
| --- | --- |
| [Product releases](https://github.com/Herbertofury/INTA/blob/main/data/product-releases.json) | Current Desktop, Mobile and Rescue tags, named files, channels and published evidence. Change this record rather than editing generated links. |
| [Tool catalog](https://github.com/Herbertofury/INTA/blob/main/data/tools.json) | Names, official URLs, platforms, selection tier, source and review date |
| [Job guides](https://github.com/Herbertofury/INTA/blob/main/data/jobs.json) | Workflows, recommended tools, actions, verification, escalation and sources |
| [Job workspaces](https://github.com/Herbertofury/INTA/blob/main/data/job-workspaces.json) | Purpose-specific toolkits, official action resources, symptom decisions and completion evidence |
| [Method registry](https://github.com/Herbertofury/INTA/blob/main/data/methods.json) | Precise named methods, aliases, canonical destinations and proof |
| [Link aliases](https://github.com/Herbertofury/INTA/blob/main/data/link-aliases.json) | Additional explicit application and platform names used by the linker |
| [Linking helper](https://github.com/Herbertofury/INTA/blob/main/scripts/wiki_links.py) | Protected <a href="https://github.github.com/gfm/" title="Open Markdown official download or service page" aria-label="Open Markdown official download or service page" data-inta-brand="app-markdown"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/99f4edb33272b70febb8.png" alt="Markdown official page" width="18" height="18" valign="middle" /></a> [Markdown](https://github.github.com/gfm/) handling and links on every repeated known name |
| [Wiki layout](https://github.com/Herbertofury/INTA/blob/main/data/wiki-layout.json) | Structured wiki presentation and the decision diagrams for all 22 jobs |
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

The three product channels are independent. Verify each exact release tag, published asset and checksum before changing [the download record](https://github.com/Herbertofury/INTA/blob/main/data/product-releases.json); never substitute the repository-wide latest redirect. Check the generated landing pages and the [Hub Resources view](https://herbertofury.github.io/INTA/#resources) after publishing.

## 🔎 Run a freshness check on demand

```sh
python3 scripts/check-upstreams.py --output reports/upstreams.json
```

Command legend: <a href="https://www.python.org/" title="Open Python official download or service page" aria-label="Open Python official download or service page" data-inta-brand="app-python"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/e3ea97f62e4259df2d67.png" alt="Python official page" width="18" height="18" valign="middle" /></a> [Python](https://www.python.org/) runs the [upstream checker](https://github.com/Herbertofury/INTA/blob/main/scripts/check-upstreams.py). The resulting report supports [manual freshness review](https://github.com/Herbertofury/INTA/wiki/Job-preventive-maintenance).

The checker reports each tool. For projects with <a href="https://github.com/" title="Open GitHub official download or service page" aria-label="Open GitHub official download or service page" data-inta-brand="app-github"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/99f4edb33272b70febb8.png" alt="GitHub official page" width="18" height="18" valign="middle" /></a> [GitHub releases](https://docs.github.com/en/repositories/releasing-projects-on-github/about-releases), it queries stable-release metadata and preserves explicit results when no release exists or access fails. Other vendors receive official-link reachability and a manual-version-review status. A reachable URL never becomes an invented version, and the report does not install software or silently rewrite the catalog.

The **Check tool upstreams** [workflow](https://github.com/Herbertofury/INTA/actions/workflows/upstreams.yml) remains on demand. The enabled weekday knowledge-maintenance task is documented separately in [Continuous Improvement](https://github.com/Herbertofury/INTA/wiki/Continuous-Improvement). The existing [rescue build workflow](https://github.com/Herbertofury/INTA/actions/workflows/rescue.yml) and <a href="https://github.com/Herbertofury/INTA/blob/main/docs/rescue.md" title="Open Media Creator official download or service page" aria-label="Open Media Creator official download or service page" data-inta-brand="app-media-creator"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="Media Creator official page" width="18" height="18" valign="middle" /></a> [INTA Media Creator](https://github.com/Herbertofury/INTA/blob/main/media/inta-media.py) retain their own image and refresh lifecycles.

## 🧪 Build and inspect the affected path

```sh
python3 scripts/build-hub.py
python3 -m unittest discover -s tests
python3 scripts/build-hub.py --check
python3 scripts/check-wiki.py
python3 scripts/check-knowledge.py --output reports/knowledge-health.json
```

Command legend: <a href="https://www.python.org/" title="Open Python official download or service page" aria-label="Open Python official download or service page" data-inta-brand="app-python"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/e3ea97f62e4259df2d67.png" alt="Python official page" width="18" height="18" valign="middle" /></a> [Python](https://www.python.org/) runs the [hub builder](https://github.com/Herbertofury/INTA/blob/main/scripts/build-hub.py), [test suite](https://github.com/Herbertofury/INTA/tree/main/tests) and [wiki checker](https://github.com/Herbertofury/INTA/blob/main/scripts/check-wiki.py). Preserve any failed result as an open gate until resolved.

Open `site/index.html` directly or serve the `site` folder locally. Review the changed page in the actual rendered interface. Where affected, exercise full-catalog search, combined filters, job checkboxes, saved tools, export/print, both themes, keyboard focus and a narrow screen. Follow repeated named links and inspect diagrams and their linked legends. Keep every entry available; do not add quantity caps, viewport virtualization or content culling.

## 🚀 Publish the actual wiki

Use a <a href="https://git-scm.com/" title="Open Git official download or service page" aria-label="Open Git official download or service page" data-inta-brand="app-git"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/afb8a6bd2f69c8eb2aba.png" alt="Git official page" width="18" height="18" valign="middle" /></a> [Git](https://git-scm.com/) session authorized for the canonical wiki remote:

```sh
python3 scripts/publish-wiki.py --checkout ../INTA.wiki
```

Command legend: <a href="https://www.python.org/" title="Open Python official download or service page" aria-label="Open Python official download or service page" data-inta-brand="app-python"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/e3ea97f62e4259df2d67.png" alt="Python official page" width="18" height="18" valign="middle" /></a> [Python](https://www.python.org/) runs the [wiki publisher](https://github.com/Herbertofury/INTA/blob/main/scripts/publish-wiki.py); <a href="https://git-scm.com/" title="Open Git official download or service page" aria-label="Open Git official download or service page" data-inta-brand="app-git"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/afb8a6bd2f69c8eb2aba.png" alt="Git official page" width="18" height="18" valign="middle" /></a> [Git](https://git-scm.com/) supplies the authenticated checkout of the [live wiki](https://github.com/Herbertofury/INTA/wiki).

The publisher checks the exact remote, fetches current content, copies generated pages while preserving unrelated pages, and uses a normal fast-forward commit. Keep authentication with the credential mechanism, never inside source or documentation. Use an authorized maintainer checkout if an automated token cannot reach the wiki repository.

The [hub workflow](https://github.com/Herbertofury/INTA/actions/workflows/hub.yml) deploys the validated site to <a href="https://docs.github.com/en/pages" title="Open GitHub Pages official download or service page" aria-label="Open GitHub Pages official download or service page" data-inta-brand="app-github-pages"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/b6a41bd1fb24290efbeb.png" alt="GitHub Pages official page" width="18" height="18" valign="middle" /></a> [GitHub Pages](https://docs.github.com/en/pages/getting-started-with-github-pages/using-custom-workflows-with-github-pages). Publish the exact tested source checkpoint, matching generated site and live-wiki source. Save the complete deliverable to the project's connected <a href="https://drive.google.com/" title="Open Google Drive official download or service page" aria-label="Open Google Drive official download or service page" data-inta-brand="app-google-drive"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4a3cfc17be3f4e8ad2c8.png" alt="Google Drive official page" width="18" height="18" valign="middle" /></a> [Google Drive](https://drive.google.com/) destination, then verify the returned identity and relevant content.

## ✅ Read back before closure

| Surface | What to verify |
| --- | --- |
| Canonical repository | The published commit is the tested source revision. |
| Hosted hub | The changed page or interaction shows the intended content. |
| Actual live wiki | The pages, navigation, links and diagrams match generated source. |
| Complete artifact copy | The archive contains the full deliverable and belongs to the same revision. |
| Offline copy | The full embedded content works without requiring a network just to read it. |

Record results in [the contribution template](https://github.com/Herbertofury/INTA/wiki/Templates). Downloaded offline copies retain their build snapshot; replace the HTML when you want updated guidance.


## ⭐ Maintain tool reputation evidence

The canonical reputation design lives in [Tool Reputation](https://github.com/Herbertofury/INTA/blob/main/docs/tool-reputation.md), with source identities in [reputation-sources.json](../data/reputation-sources.json). Treat these as distinct evidence streams:

- <a href="https://github.com/" title="Open GitHub official download or service page" aria-label="Open GitHub official download or service page" data-inta-brand="app-github"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/99f4edb33272b70febb8.png" alt="GitHub official page" width="18" height="18" valign="middle" /></a> [GitHub](https://github.com/) stars, forks and repository activity (only after matching the exact project).
- Native professional-review ratings with their original scale and review count.
- Independent laboratory results with product version, platform, test date and methodology.
- Recommendations from named technical publications and verified practitioners.
- Community discussion signals with distinct-author and cross-post deduplication.
- Security advisories and <a href="https://herbertofury.github.io/INTA/" title="Open INTA official download or service page" aria-label="Open INTA official download or service page" data-inta-brand="app-inta"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA official page" width="18" height="18" valign="middle" /></a> [INTA](https://herbertofury.github.io/INTA/)'s own runtime/field validation.

A provider registry entry does not mean that provider is integrated, queried or endorsed. Use documented APIs, permitted public pages or feeds only. Respect provider terms, rate limits, privacy and access controls. Preserve the last successful snapshot on provider failure and mark it stale; never replace missing data with zero. Popularity, sentiment and expert evidence must never be merged into one unexplained score or bypass the existing promotion and rescue-image gates.


## 📚 Maintainer references

- <a href="https://github.com/" title="Open GitHub official download or service page" aria-label="Open GitHub official download or service page" data-inta-brand="app-github"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/99f4edb33272b70febb8.png" alt="GitHub official page" width="18" height="18" valign="middle" /></a> [GitHub: adding and editing wiki pages](https://docs.github.com/en/communities/documenting-your-project-with-wikis/adding-or-editing-wiki-pages)
- <a href="https://github.com/" title="Open GitHub official download or service page" aria-label="Open GitHub official download or service page" data-inta-brand="app-github"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/99f4edb33272b70febb8.png" alt="GitHub official page" width="18" height="18" valign="middle" /></a> [GitHub: custom Pages workflows](https://docs.github.com/en/pages/getting-started-with-github-pages/using-custom-workflows-with-github-pages)
- <a href="https://github.com/" title="Open GitHub official download or service page" aria-label="Open GitHub official download or service page" data-inta-brand="app-github"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/99f4edb33272b70febb8.png" alt="GitHub official page" width="18" height="18" valign="middle" /></a> [GitHub: diagram support](https://docs.github.com/en/get-started/writing-on-github/working-with-advanced-formatting/creating-diagrams)
- <a href="https://github.com/" title="Open GitHub official download or service page" aria-label="Open GitHub official download or service page" data-inta-brand="app-github"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/99f4edb33272b70febb8.png" alt="GitHub official page" width="18" height="18" valign="middle" /></a> [GitHub: writing and formatting](https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax)

**➜ Continue:** [Contribution Guide](https://github.com/Herbertofury/INTA/wiki/Contribute) · [Continuous Improvement](https://github.com/Herbertofury/INTA/wiki/Continuous-Improvement) · [Knowledge Standards](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards)

---

Editorial review: **2026-11-27** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
