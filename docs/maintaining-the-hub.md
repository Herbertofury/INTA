# Keeping the INTA hub current

The hub, Markdown playbooks and GitHub Wiki share canonical content. A correction should improve every surface instead of leaving several slightly different versions behind.

## The content map

| Source | Purpose |
| --- | --- |
| `data/tools.json` | Complete tool catalog, official URLs, platforms, selection tier, source and review date. |
| `data/jobs.json` | Complete workflows, recommended tools, steps, verification, escalation and sources. |
| `data/meta.json` | Hub identity and review metadata. |
| `portal/` | Accessible offline-ready interface. |
| `docs/` | Maintainer and rescue documentation, research provenance and service templates. |
| `scripts/build-hub.py` | Deterministic builder for `site/`, `wiki/` and Markdown playbooks. |
| `scripts/check-upstreams.py` | On-demand upstream release and official-link status report. |
| `scripts/publish-wiki.py` | Publishes generated pages into the actual GitHub Wiki using an authenticated Git checkout. |

## Before adopting a tool update

1. Open the original publisher's release notes and official download. Confirm the operating system and CPU architecture.
2. Check the publisher signature and/or published cryptographic checksum where available. Record the resolved version in the team's private kit inventory.
3. Test the specific job path on a disposable or agreed test system. For disk tools, use disposable images and unambiguous source/destination labels.
4. Compare the outcome, compatibility, startup cost and resource use with the current kit. A newer release is a candidate until it passes the job.
5. Update the catalog guidance and `reviewedAt` only after editorial review. Do not change that date just because a URL responded.
6. Rebuild, review the diff, run the focused tests, then publish. Keep the previous working installer or image in the team's approved artifact store when practical.

## Run an update check on demand

```sh
python3 scripts/check-upstreams.py --output reports/upstreams.json
```

The checker reports each tool. For GitHub release projects it queries the latest stable release metadata, with an explicit result when no release exists or access fails. Other vendors receive an official-link reachability check and a **manual version review** status. A responding URL never becomes an invented version number. The script neither installs software nor silently rewrites the catalog.

The **Check tool upstreams** Actions workflow provides the same report on request. It has no recurring schedule. INTA Rescue retains its existing weekly rebuild and on-demand Media Creator refresh workflow.

## Build and verify the hub

```sh
python3 scripts/build-hub.py
python3 -m unittest discover -s tests
python3 scripts/build-hub.py --check
```

Open `site/index.html` directly or serve the `site` folder locally. Verify search, filters, the entire catalog, job checkboxes, saved tools, exports, light/dark themes and a narrow screen. All tools remain reachable; do not add quantity caps or viewport virtualization.

## Publish the actual Wiki

Use a Git session authorized for `Herbertofury/INTA.wiki.git`:

```sh
python3 scripts/publish-wiki.py --checkout ../INTA.wiki
```

The script checks the exact wiki remote, fetches the current wiki, copies only generated pages, preserves unrelated pages, and creates a normal fast-forward commit. It never force-pushes. Authentication stays with Git; do not put tokens into documentation or repository files. The initial wiki has already been created. If an Actions token cannot access a wiki repository, publish through an authenticated maintainer checkout instead of weakening repository permissions.

The hub workflow deploys the validated `site/` artifact to GitHub Pages. Canonical content lives in `main`; downloaded offline copies represent their build's content snapshot and need to be replaced when you want newer guidance.

## Suggest an improvement

Use the repository's **Tool recommendation** or **Playbook correction** issue form. Supply the original source, job fit, tested environment, known limitations and a reproduction or verification method. Keep customer-specific information out of a public issue.

## References

- [GitHub: adding and editing wiki pages](https://docs.github.com/en/communities/documenting-your-project-with-wikis/adding-or-editing-wiki-pages)
- [GitHub: custom workflows for Pages](https://docs.github.com/en/pages/getting-started-with-github-pages/using-custom-workflows-with-github-pages)
- [GitHub: configuring the publishing source](https://docs.github.com/en/pages/getting-started-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site)
