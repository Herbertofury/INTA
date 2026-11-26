# Interface design

[Home](https://github.com/Herbertofury/INTA/wiki) · [Knowledge Standards](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · **Interface design**

The INTA workspace is a working reference for technicians. Its first screen should make it easy to find a tool, start a job, or continue a checklist. The design should feel like professional service software: precise, readable, quiet, and specific to the work.

**Design review: November 26, 2026.** This direction incorporates the owner's correction to the previous green, oversized-card interface. It supersedes that presentation while preserving the complete catalog, playbooks, direct links, saved state, and offline features.

## Reference and decisions

The source review starts with [Athena's interface library](https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/docs/ui-design-library/README.md) and [five-reference comparison](https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/ui-reference-comparison-2026-11-21/README.md). The latter identifies [Unslop](https://github.com/heydamir/unslop) by Damir Sokolovsky. Its review approach informs the choices below; the implementation uses original project code and native assets.

| Reference principle | Application in INTA | Observable check |
| --- | --- | --- |
| Establish a product-specific direction before adding effects | Use the full technician directory, job procedures, evidence, and actual catalog counts as the interface | The first screen contains useful service content |
| Use hierarchy and alignment to organize information | Group tools into compact rows, align actions, and separate sections with thin rules | Names, descriptions, platforms, and actions scan predictably |
| Make color deliberate | Neutral charcoal surfaces, neutral text, and one restrained blue interaction accent | Green and teal are absent from the brand palette |
| Remove unnecessary visual effects | Use flat surfaces, small control radii, and brief state transitions | No decorative gradients, glows, moving backgrounds, or floating panels |
| Validate the real interaction | Exercise navigation, filters, keyboard focus, saved state, exports, and narrow layouts | Recorded browser evidence comes from the final generated artifact |

The [Compact and Sieve observations](https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/docs/ui-design-library/visual-studies.md) support studying contextual rows and clearly separated evidence states. Those observations are design references, not proof that another product's behavior or performance transfers to INTA. The [Bencho and collection study](https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/docs/ui-design-library/bencho-and-saas.md) likewise informs consistent states and navigation; this project does not require its components.

## Layout

- **Application shell:** a quiet sidebar, a compact utility bar, and a fluid working area. Navigation names the actual destination.
- **Home:** lead with the problem, a suitable free starting tool, its official link, and the matching procedure. Use a compact, immediately visible selection table before the complete tool directory. Keep every catalog entry available below it.
- **Tool library:** a readable list with visible filters, result count, platform information, recommendation tier, details, official source, and save control.
- **Playbooks:** a compact index. Individual procedures use a document column with a restrained context area for the toolkit and related information.
- **Knowledge desk:** linked reference rows with short descriptions. Its purpose is finding guidance, so it does not need a gallery of equally sized cards.
- **Wiki:** use the host's document typography, bold task links, meaningful emoji, useful tables, and decision diagrams. Every service scenario and tool category must be visible immediately in the sidebar. Place common jobs and direct official tool links before administration links. The masthead establishes identity without dominating the page.

Avoid giant welcome sections, equal-height promotional tiles, decorative statistic panels, fake avatars, decorative status dots, slogan-filled footers, and oversized empty containers. A box belongs around a real control group or distinct working context. Ordinary text does not need a card.

## Type, color, and spacing

Use a coherent system font stack that works offline. Reading text is at least 1rem, and frequently used controls and labels are at least 0.875rem. Secondary metadata may be smaller when it remains legible. Do not shrink text to make a dense page look compact; improve the structure instead.

Use sentence case, a small number of text sizes, and moderate weights. Monospace is appropriate for commands, identifiers, and compact numerical metadata. Long instructions need comfortable line height and a readable line length. Interface density comes from removing redundant wrappers and empty space.

The default theme is neutral dark. The light theme is equally complete and keeps the same hierarchy. Blue identifies links, focus, selection, and the occasional primary action. Warning and failure colors communicate an actual state with a text label. They do not become decorative category colors. Use a consistent spacing scale and small, related control radii.

## Icons, emoji, and diagrams

Application controls use one small, consistent line-icon style with readable text labels or accessible names. Remove sparkle motifs, decorative icon tiles, and icon-only controls whose purpose is unclear.

The Wiki uses meaningful emoji on scenario, category and quick-tool navigation. Pair each icon with a bold, short linked label, such as **[🦠 Virus removal](https://github.com/Herbertofury/INTA/wiki/Job-malware-removal)** or **[💾 Recover data](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive)**. This is functional emphasis requested by the owner: users should recognize and open their destination immediately. Keep decorative emoji out of ordinary explanatory prose. Existing direct-link rules apply to every recognized tool, app, and named method, including repeated mentions.

**Navigation stays expanded.** Do not place scenarios, tool categories, official-download shortcuts or reference destinations inside collapsed groups, accordions, tabs or a “more” menu. A longer visible list is preferable to extra navigation clicks. Optional explanatory diagrams may use disclosures when all action destinations also remain visible outside them.

**Front pages start with useful tools.** The application Home, Wiki Home, repository README and Start Here page share the same curated quick-start data. Show the scenario, a strong free choice for that job, a direct official link, a concise reason and the guide. Favor tools available without a paid upgrade for the intended technician workflow. Explain a relevant limitation where it changes the next action. Put detailed selection evidence in the linked guide. Never invent ratings, apply one product's test score to a different edition or utility, or claim a universal winner when the evidence covers a narrower task.

Use [Mermaid](https://mermaid.js.org/) only where a branch, dependency, or sequence benefits from a diagram. Preserve plain labels and provide linked legends immediately afterward. A table is usually better for selecting a tool from a small set.

## Interaction and accessibility

| State or path | Required behavior |
| --- | --- |
| Keyboard navigation | Every action is reachable; visible focus remains clear against the surface |
| Wiki destinations | All scenarios, tool categories and quick tool links are bold and directly clickable without expanding a group |
| Search and filters | Show all matching entries and a truthful result count; preserve the query through expected navigation |
| Saved tools and checklists | Persist the existing keys and user state across reloads and redesigns |
| Empty result | Explain the current filter and provide a working way to clear it |
| Storage unavailable | Communicate the limitation without losing the current working session |
| Mobile and zoom | Reflow text and actions; preserve all entries and fields rather than hiding columns or cutting text |
| Long commands or true tables | Confine necessary horizontal scrolling to that content, not the entire page |
| Reduced motion | Preserve all state feedback without relying on animation |
| Offline use | Keep the full catalog, procedures, search, and local state useful without runtime network dependencies |
| Export | Produce the real current checklist or complete offline page |

Check narrow layouts against the [reflow guidance](https://www.w3.org/WAI/WCAG22/Understanding/reflow.html). Passing a selected browser check is evidence for that case, not an unsupported claim of complete accessibility conformance.

## Acceptance before publication

- [ ] Visually inspect the actual final home, library, procedure, knowledge, and mobile views.
- [ ] Confirm the new composition addresses the rejected green/card-heavy layout.
- [ ] Confirm all tools, jobs, methods, commands, and source routes remain available.
- [ ] Confirm every Wiki scenario and tool category is visible without opening a collapsed group, and official-download shortcuts go straight to the catalog's verified publisher route.
- [ ] Exercise search, filters, direct links, history, saving, checklists, reset/undo, theme, and exports.
- [ ] Check keyboard focus, readable control text, long labels, and narrow-screen overflow.
- [ ] Verify that the live site and actual Wiki reflect the tested source revision.
- [ ] Preserve the complete runnable deliverable and the review evidence.

Record results in [project validation](https://github.com/Herbertofury/INTA/blob/main/docs/validation.md). Future contributions should improve this direction through observed problems and evidence, rather than reintroducing a generic dashboard template.

**Continue:** [Contribution Guide](https://github.com/Herbertofury/INTA/wiki/Contribute) · [Knowledge Standards](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Continuous Improvement](https://github.com/Herbertofury/INTA/wiki/Continuous-Improvement)
