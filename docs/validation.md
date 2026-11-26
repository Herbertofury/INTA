# Technician workspace validation

**Review date: November 26, 2026.** This record covers the professional interface redesign, always-visible Wiki navigation and shared free-tool starting points. The original redesign preservation baseline is `28275815ff602fbb03a1f510b677b4dfc72659f7`; the navigation and quick-start update is compared with `a651b47fce7740ce6fbb11b3f779969eeb64c5b8`. The catalog includes [Microsoft Sysinternals Suite](https://learn.microsoft.com/en-us/sysinternals/downloads/sysinternals-suite), [WizTree](https://diskanalyzer.com/download) and [WizFile](https://antibody-software.com/wizfile/download).

## Visible scenario and tool navigation

The latest navigation update follows the owner’s screenshot review. Every Wiki page now exposes all **22 service scenarios**, **10 tool categories**, and **12 official tool shortcuts** as bold, emoji-labeled links. No sidebar destination requires expanding a group. Common virus-removal, recovery, slow-PC and remote-help scenarios appear first; reference and contribution routes remain visible below them.

Home uses a complete scenario table with short action labels, and every one of the **113 entries** in the complete tool directory includes an immediate official-source route. Existing scenario pages, reference destinations, and sidebar anchors are preserved. The tool catalog, job procedures, method records and executable commands are unchanged from `a651b47fce7740ce6fbb11b3f779969eeb64c5b8`; all 27 diagram sources are unchanged. The application and standalone HTML include the new quick-start section. Three added regression gates require visible navigation, official routes for every catalog entry and consistent direct links across the front pages. See the [navigation review](reference/quick-navigation-checks.json) and [current Wiki audit](reference/quick-navigation-wiki-checks.json).

## Shared free-tool starting points

Application Home, Wiki Home, the repository README and Start Here lead with the same **12 scenario-specific free starting tools**, each with a bold direct official link and a matching guide. The source is [Quick-start selections](../data/quick-start.json); the generated Markdown blocks and application payload match it. README and authored Start Here retain all content outside their managed quick-start blocks. All 113 tools remain available below the application’s quick-start section.

The [selection evidence](free-tool-picks.md) distinguishes practical fit, business-use terms and current independent test results. Resident antivirus test results are attributed to the exact tested product and period; they are not presented as rankings of unrelated cleanup or diagnostic utilities. [WizTree](https://diskanalyzer.com/download) and [WizFile](https://antibody-software.com/wizfile/download) remain direct sidebar shortcuts while the free business-use defaults use [WinDirStat](https://github.com/windirstat/windirstat/releases/latest) and [Everything desktop](https://www.voidtools.com/downloads/).

The current browser review covers all 12 direct tool routes and guide links, dark and light themes, 390px and 320px layouts, offline export, checklist and favorite persistence, library filtering and keyboard search. No document overflow, hidden quick-start rows, nested links, JavaScript errors or runtime network requests were observed. Local Markdown previews cover Home, README and Start Here at desktop and 390px widths. A final visual review reduced their quick-choice tables to two directly linked columns, keeping long explanations on the selection page and full guides; the first tool link remains visible on the initial screen. These Markdown previews approximate the host’s styling; actual publication is checked separately.

## Content and generated output

| Gate | Observed result |
| --- | --- |
| Tool catalog | 113 complete records; all 111 previous tools retained, including [Microsoft Sysinternals Suite](https://learn.microsoft.com/en-us/sysinternals/downloads/sysinternals-suite). |
| Procedures | All 22 playbooks remain. The catalog now contains 140 ordered steps and 83 acceptance checks. |
| Original content | All 132 original steps retain their titles, original detail text, relative order and semantic destinations; all 72 original checks and nine executable commands remain. |
| Methods | All 40 original methods remain; five additions bring the registry and authored roster to 45. |
| Wiki | 49 content pages and two shared navigation files; 27 decision diagrams. |
| Link audit | 334 exact registered names and 3,790 inline links pass the complete audit, including repeated mentions, page destinations, fragments and linked legends. |
| Regression suite | 25 tests pass, including the original [Media Creator](rescue.md) tests, command preservation and published step destinations. |
| Generation | All 80 outputs match their canonical input; syntax and whitespace checks pass. |

The complete [content-preservation report](reference/professional-content-preservation.json) compares every original tool, job and method ID, original command and published step destination. The [contribution report](reference/professional-catalog-checks.json) additionally checks unchanged original step text, new tool references and the complete method roster.

## Professional redesign evidence

The earlier professional redesign artifact was exercised in [Chromium](https://www.chromium.org/) 158.0.8127.0. Home and Library each render **all 113 tools**, and all ten discipline routes work. The 22 playbooks, nine Knowledge Desk destinations and 52 official reference entries remain reachable. Recommendation filters return 76 everyday, 33 specialist and four evaluation entries. A broad search returns 104 matching records without a result cap.

The visual design follows [Interface Design](interface-design.md) and the linked [Athena](https://github.com/Herbertofury/Athena) references. Screenshots were reviewed at desktop and narrow widths in both themes. The default surface is neutral charcoal; headings use 30px text, reading text uses 16px, and working controls are at least 14px. Records use flat rows and thin dividers. No gradients are present in the inspected Home DOM. At 320, 390 and 720 CSS pixels, the checked layouts retain their content without document-wide horizontal overflow.

Search, combined filters, browser Back, saved tools, keyboard navigation, focus feedback, theme persistence, checklist progress, reset Undo, Markdown export and offline HTML export pass. The final source creates no external runtime resource requests, and the exercised flows record no JavaScript errors. All nine command blocks across all 22 guides match the canonical command bytes.

These observations establish the tested behavior and layout. They do not assert complete accessibility conformance or a controlled performance improvement. The [render report](reference/professional-render-checks.json) records DOM counts honestly: complete rows and additional linked content increase some counts. No record is culled, virtualized or hidden for the comparison.

## Saved progress and stable links

Inserting a step previously changed its position-based storage key. The portal now identifies steps by stable anchors and conservatively migrates an old saved item only when its unchanged title has one unambiguous match. The existing version-one storage key is retained. Browser fixtures prove that the old checked scanner-preparation and final-retest steps remain checked at their new positions, while inserted steps begin unchecked. Changed titles, duplicate titles and conflicting old candidates are not guessed. Unchecking, reloading, resetting, Undo and a storage-unavailable session were exercised.

Existing published step fragments remain attached to their original actions. New steps receive distinct descriptive fragments. Regression tests verify specific moved actions and reject duplicate destinations. The [migration report](reference/professional-migration-checks.json) and [content-preservation report](reference/professional-content-preservation.json) record these results.

## Wiki and source review

All 27 [Mermaid](https://mermaid.js.org/) diagrams were parsed and rendered using version 12.3.0 in the verification workspace. The shipped hub adds no diagram runtime or CDN dependency. Complete decision topologies and existing checkpoints remain; native disclosures keep long diagrams available without pushing the procedure down the page. The source hashes are recorded in the [diagram report](reference/professional-diagram-checks.json).

The new [Tool Selection](tool-selection.md) and [Service Evidence](service-evidence.md) pages explain diagnostic fit, remote-session capabilities, scan coverage, rebuild acceptance and image/mapfile reconciliation. Their primary-source review is recorded in the [source report](reference/professional-source-checks.json). Official source availability, documentation review and physical execution are distinct evidence levels; third-party software binaries were not executed or certified in this change. The current [CISA guide](https://www.cisa.gov/sites/default/files/2025-03/StopRansomware-Guide%20508.pdf) was checked through primary indexed text; full-PDF retrieval was unavailable in the review environment.

## Evidence and publication

- [Current navigation, preservation and quick-start review](reference/quick-navigation-checks.json)
- [Complete Wiki audit](reference/quick-navigation-wiki-checks.json)
- [Current quick-start browser checks](reference/quick-start-ui-checks.json)
- [Current front-page Markdown checks](reference/quick-start-markdown-checks.json)
- [Free-tool selection source review](reference/quick-start-source-checks.json)
- [Browser behavior checks](reference/professional-browser-checks.json)
- [Adversarial linked-prose checks](reference/professional-link-checks.json)
- [Rendering, typography, commands and candidate hashes](reference/professional-render-checks.json)
- [Root review of the final requested tool entries](reference/professional-root-review.json)

The [publishing workflow](../.github/workflows/hub.yml) reruns the test suite, generated-output check and complete Wiki audit. Publication requires the exact remote source checkpoint, matching hosted HTML, a byte-for-byte comparison of the actual live Wiki against generated files, and a verified complete source archive plus offline HTML in the existing project destination. Follow [Maintaining the Hub](maintaining-the-hub.md) for that process; local test success alone does not establish deployment success.

The material review resolved old checklist progress migration, stable published step links, mobile header height and anchor offset, linked command references, complete method coverage, and the last generic tool-detail slogan. The separate [Rescue workflow](rescue.md) continues to own image-build and physical boot evidence.
