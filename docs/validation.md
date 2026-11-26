# 🧪 Technician knowledge hub validation

**Review date: November 26, 2026.** This record covers the Service Atlas upgrade and the preserved complete homepage directory from source commit `5cb05efa3fd5948520f919c32f7b38265c60c16c`.

## Complete content and executable editorial rules

| Gate | Observed result |
| --- | --- |
| Original tool catalog and job data | 111 tools and 22 playbooks preserved; neither canonical catalog lost entries or procedure fields. |
| Workflow completeness | 132 ordered steps and 72 acceptance checks remain available. |
| Method coverage | 40 named methods cover all 22 jobs, with selection guidance, proof and primary sources. |
| Wiki publication set | 45 content pages and 2 shared navigation files. |
| Link registry | 315 exact tool, application, method and alias entries, with collision and URL validation. |
| Wiki link audit | Every registered prose occurrence, internal page/anchor and diagram legend passes the [complete audit](../scripts/check-wiki.py). |
| Diagram coverage | 27 diagrams, including a distinct branching decision map for every playbook. |
| Regression suite | 20 tests pass, including the original [Media Creator](rescue.md) tests and byte-preservation checks for every canonical command. |
| Generation | All 74 outputs match their canonical source; repeated builds are deterministic. |

The link tests include repeated occurrences, longest matches, word boundaries, ambiguous ordinary words, existing links, images, reference links, raw URLs, code spans, fenced commands, diagram source and malicious URL rejection. A deliberately unlinked name and nonexistent page both fail the gate.

## Actual browser and diagram proof

The complete built app was exercised in [Chromium](https://www.chromium.org/) 158.0.8127.0. The homepage renders all 111 tools and all 10 discipline destinations; every jump route was exercised. The tool library retains all 111 entries, core/specialist/evaluate filtering returns 74/33/4, and the six Knowledge Desk destinations are present. Search, combined filters, browser Back, favorites, checklist persistence, reset Undo, exports, offline reopening and mobile navigation passed. No JavaScript errors, runtime network requests or 390-pixel horizontal overflow were observed.

The linked-prose checks exercised headings, repeated names, exact-case ambiguity, escaped content, existing anchors and code preservation. The app creates no nested anchors and keeps commands byte-identical. A root review also loads a fresh copied HTML artifact and checks the actual tool-title link, Knowledge Desk, full directory, commands and mobile layout.

All 27 final [Mermaid](https://mermaid.js.org/) diagrams were parsed and rendered in a real browser using version 12.3.0 installed only in the verification workspace. The shipped hub adds no diagram runtime or CDN dependency. The diagram report records source hashes for every graph; representative rendered diagrams and the native vector banner were visually inspected.

## Evidence records

- [Complete Wiki audit](reference/knowledge-quality.json)
- [Browser regression results](reference/knowledge-browser-checks.json)
- [Adversarial linking results](reference/knowledge-link-checks.json)
- [Final fresh-copy smoke test](reference/knowledge-final-smoke.json)
- [Rendered diagram source hashes](reference/knowledge-diagram-checks.json)
- [New source-route checks and corrections](reference/knowledge-source-checks.json)

The [publishing workflow](../.github/workflows/hub.yml) reruns the regression suite, generated-output check and complete Wiki audit. Actual Wiki publication requires matching the live `.wiki.git` files to this generated publication set, as described in [Maintaining the Hub](maintaining-the-hub.md).

## Material challenge findings resolved

- Preserved the concurrent complete homepage directory instead of overwriting it during the design upgrade.
- Fixed discipline fragments so they open Home and scroll to the actual section, including direct and mobile links.
- Fixed longest-alias matching so a partial longer alias cannot swallow a valid shorter name.
- Added explicit section/step anchors so emoji headings retain reliable navigation.
- Added linked legends immediately after all job diagrams and command examples.
- Kept ordinary words such as “everything” and “windows” from becoming accidental product links.
- Resolved the moved virtualization product route through its official current source.

These checks establish the hub's behavior and the consistency of the knowledge system. Third-party software execution, antivirus efficacy and physical recovery-media compatibility require their own recorded evidence. The separate Rescue workflow continues to own image build and boot proof.
