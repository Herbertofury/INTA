# Interface design

[Home](https://github.com/Herbertofury/INTA/wiki) · [Knowledge Standards](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · **Interface design**

The <a href="https://herbertofury.github.io/INTA/" title="Open INTA official download or service page" aria-label="Open INTA official download or service page" data-inta-brand="app-inta"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA official page" width="18" height="18" valign="middle" /></a> [INTA](https://herbertofury.github.io/INTA/) workspace is a working reference for technicians. Its first screen should make it easy to find a tool, start a job, or continue a checklist. The design should feel like professional service software: precise, readable, quiet, and specific to the work.

**Design review: November 27, 2026.** This direction incorporates the owner's correction to the previous green, oversized-card interface. It supersedes that presentation while preserving the complete catalog, playbooks, direct links, saved state, and offline features.

## Reference and decisions

The source review starts with <a href="https://github.com/Herbertofury/Athena" title="Open Athena official download or service page" aria-label="Open Athena official download or service page" data-inta-brand="app-athena"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/99f4edb33272b70febb8.png" alt="Athena official page" width="18" height="18" valign="middle" /></a> [Athena's interface library](https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/docs/ui-design-library/README.md) and <a href="https://github.com/Herbertofury/Athena" title="Open Athena official download or service page" aria-label="Open Athena official download or service page" data-inta-brand="app-athena"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/99f4edb33272b70febb8.png" alt="Athena official page" width="18" height="18" valign="middle" /></a> [five-reference comparison](https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/research/ui-reference-comparison-2026-11-21/README.md). The latter identifies <a href="https://github.com/heydamir/unslop" title="Open Unslop official download or service page" aria-label="Open Unslop official download or service page" data-inta-brand="app-unslop"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/99f4edb33272b70febb8.png" alt="Unslop official page" width="18" height="18" valign="middle" /></a> [Unslop](https://github.com/heydamir/unslop) by Damir Sokolovsky. Its review approach informs the choices below; the implementation uses original project code and native assets.

| Reference principle | Application in <a href="https://herbertofury.github.io/INTA/" title="Open INTA official download or service page" aria-label="Open INTA official download or service page" data-inta-brand="app-inta"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA official page" width="18" height="18" valign="middle" /></a> [INTA](https://herbertofury.github.io/INTA/) | Observable check |
| --- | --- | --- |
| Establish a product-specific direction before adding effects | Use the full technician directory, job procedures, evidence, and actual catalog counts as the interface | The first screen contains useful service content |
| Use hierarchy and alignment to organize information | Group tools into compact rows, align actions, and separate sections with thin rules | Names, descriptions, platforms, and actions scan predictably |
| Make color deliberate | Neutral charcoal surfaces, neutral text, and one restrained blue interaction accent | Green and teal are absent from the brand palette |
| Remove unnecessary visual effects | Use flat surfaces, small control radii, and brief state transitions | No decorative gradients, glows, moving backgrounds, or promotional floating panels |
| Validate the real interaction | Exercise navigation, filters, keyboard focus, saved state, exports, and narrow layouts | Recorded browser evidence comes from the final generated artifact |

The <a href="https://github.com/Herbertofury/Athena" title="Open Athena official download or service page" aria-label="Open Athena official download or service page" data-inta-brand="app-athena"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/99f4edb33272b70febb8.png" alt="Athena official page" width="18" height="18" valign="middle" /></a> [Compact and Sieve observations](https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/docs/ui-design-library/visual-studies.md) support studying contextual rows and clearly separated evidence states. Those observations are design references, not proof that another product's behavior or performance transfers to <a href="https://herbertofury.github.io/INTA/" title="Open INTA official download or service page" aria-label="Open INTA official download or service page" data-inta-brand="app-inta"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA official page" width="18" height="18" valign="middle" /></a> [INTA](https://herbertofury.github.io/INTA/). The <a href="https://github.com/Herbertofury/Athena" title="Open Athena official download or service page" aria-label="Open Athena official download or service page" data-inta-brand="app-athena"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/99f4edb33272b70febb8.png" alt="Athena official page" width="18" height="18" valign="middle" /></a> [Bencho and collection study](https://github.com/Herbertofury/Athena/blob/7d529423b52d3cc6104ce7469eec358f5dc9b9ac/docs/ui-design-library/bencho-and-saas.md) likewise informs consistent states and navigation; this project does not require its components.

## Layout

- **Application shell:** a quiet sidebar, a compact utility bar, and a fluid working area. Navigation names the actual destination.
- **Home:** lead with everyday problems such as **Remove viruses**, **Fix a printer**, **Fix internet** and **Fix email**. Each flat row has a bold emoji scenario link, one concrete first-action sentence, the official tool page and the exact acquisition action. Put the full searchable **Tool library** and its direct acquisition actions before **Online tools**, in Home and navigation. Keep every **Fix a problem** scenario and authored choice visible in the same workspace after tool discovery. This order follows the owner’s current instruction and supersedes the earlier quick-start → online → catalog order.
- **Tool library:** one searchable collection with compact list and gallery presentations. Keep query, category, platform, recommendation, use, sort, saved status and exact result IDs unchanged when switching presentation. Group the filters in one quiet control surface, expose truthful counts and official access actions, and keep source evidence distinguishable from editorial recommendations.
- **Playbooks:** a compact index. Individual procedures use a document column with a compact persistent toolkit, purpose-specific official links, diagnostic decisions and completion checks. On a wide screen the toolkit stays beside the reading position. Small screens show the complete toolkit first and a short sticky section strip; keep all links reachable and instructions unobscured.
- **Knowledge desk:** linked reference rows with short descriptions. Its purpose is finding guidance, so it does not need a gallery of equally sized cards.
- **Wiki:** use the host's document typography, bold task links, meaningful emoji, useful tables, and decision diagrams. Every service scenario and tool category must be visible immediately in the sidebar. Place common jobs and direct official tool links before administration links. The masthead establishes identity without dominating the page.

Avoid giant welcome sections, equal-height promotional tiles, decorative statistic panels, fake avatars, decorative status dots, slogan-filled footers, and oversized empty containers. A box belongs around a real control group or distinct working context. Ordinary text does not need a card.

## Working view

Use native sticky positioning for the contextual tool group. Do not run continuous scroll handlers to keep it in place. Handle toolkits taller than the viewport without trapping links, adding content caps or hiding manufacturers. Place manufacturer driver links together in a compact visible group. Link **Open working view** from each corresponding Wiki answer and full guide, preserving the exact selected case.

A fixed control must not cover the final line, focused item or completion check. Verify actual scrolling at desktop, narrow mobile and short landscape sizes, both themes, keyboard navigation and printed output. The <a href="https://github.com/" title="Open GitHub official download or service page" aria-label="Open GitHub official download or service page" data-inta-brand="app-github"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/99f4edb33272b70febb8.png" alt="GitHub official page" width="18" height="18" valign="middle" /></a> [GitHub renderer](https://github.com/github/markup#github-markup) removes custom styles; publish the same visible toolkit there without promising a sticky effect the host cannot provide.

## Type, color, and spacing

Use a coherent system font stack that works offline. Reading text is at least 1rem, and frequently used controls and labels are at least 0.875rem. Secondary metadata may be smaller when it remains legible. Do not shrink text to make a dense page look compact; improve the structure instead.

Use sentence case, a small number of text sizes, and moderate weights. Monospace is appropriate for commands, identifiers, and compact numerical metadata. Long instructions need comfortable line height and a readable line length. Interface density comes from removing redundant wrappers and empty space.

The default theme is neutral dark. The light theme is equally complete and keeps the same hierarchy. Blue identifies links, focus, selection, and the occasional primary action. Warning and failure colors communicate an actual state with a text label. They do not become decorative category colors. Use a consistent spacing scale and small, related control radii.

## Icons, emoji, and diagrams

Application controls use one small, consistent line-icon style with readable text labels or accessible names. Remove sparkle motifs, decorative icon tiles, and icon-only controls whose purpose is unclear.

The Wiki uses meaningful emoji on scenario, category and quick-tool navigation. Pair each icon with a bold, short linked label, such as **[🦠 Virus removal](https://github.com/Herbertofury/INTA/wiki/Job-malware-removal)** or **[💾 Recover data](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive)**. This is functional emphasis requested by the owner: users should recognize and open their destination immediately. Keep decorative emoji out of ordinary explanatory prose. Existing direct-link rules apply to every recognized tool, app, and named method, including repeated mentions.

**Navigation stays expanded.** Do not place scenarios, tool categories, official-download shortcuts or reference destinations inside collapsed groups, accordions, tabs or a “more” menu. A longer visible list is preferable to extra navigation clicks. Optional explanatory diagrams may use disclosures when all action destinations also remain visible outside them.

**Front pages start with useful tool discovery.** The application Home, Wiki Home, repository README and Start Here page share the same quick-start data. Favor tools available without a paid upgrade for the intended technician workflow. In the application, a scenario opens its own three-step answer before the complete procedure. In the Wiki, each shortcut opens a dedicated answer page with the same steps and tool links, followed by a clear complete-guide link. Cases that share a full guide still have distinct page URLs. Keep detailed selection evidence in visible guide and tool notes. Never invent ratings, apply one product's test score to a different edition or utility, or claim a universal winner when the evidence covers a narrower task.

**Two destinations, clearly named.** Keep the official product page beside the exact download action in Home, tool lists, details and recommended tool sections. File labels identify the platform and format. Built-in features offer their specific troubleshooter or instructions; Store, web, package and account-generated routes say what opens. Do not add a Details click to reach a download. Full tool details preserve alternate platforms and the original setup route.

**Prominent choices follow the job.** The shared quick-start choices define the visible shortlist for each case. Render every choice as a compact flat row with a task role, a bold linked tool name, its official page and exact acquisition. Show the short access boundary next to the action. Keep detailed comparison evidence in the linked selection review. The same choices appear on the application Home, repository front page, Wiki Home, quick answer and initial job toolkit. Preserve each original primary acquisition and case-specific launch action. Choose prominence from authored task evidence, not the first full-catalog entry or a fixed quantity limit.

Use <a href="https://mermaid.js.org/" title="Open Mermaid official download or service page" aria-label="Open Mermaid official download or service page" data-inta-brand="app-mermaid"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/c733be15cbccdcb88240.png" alt="Mermaid official page" width="18" height="18" valign="middle" /></a> [Mermaid](https://mermaid.js.org/) only where a branch, dependency, or sequence benefits from a diagram. Preserve plain labels and provide linked legends immediately afterward. A table is usually better for selecting a tool from a small set.

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

Check narrow layouts against the <a href="https://www.w3.org/" title="Open w3.org official download or service page" aria-label="Open w3.org official download or service page" data-inta-brand="site-www-w3-org"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/bf15584637e8e0ed7c87.png" alt="w3.org official page" width="18" height="18" valign="middle" /></a> [reflow guidance](https://www.w3.org/WAI/WCAG22/Understanding/reflow.html). Passing a selected browser check is evidence for that case, not an unsupported claim of complete accessibility conformance.

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

## Shared workspace system

The current visual source of truth is the revision 2 component section in `portal/styles.css`, together with the application shell in `portal/index.template.html`. Its neutral near-black default, restrained blue selection, consistent borders, 7px component radius, compact 38px desktop controls and 44px touch actions apply across the tool collection, procedures, Command Center, Updates & Maintenance, news, services, checklist and references. Light mode uses the same hierarchy. Edit the governed component rules instead of stacking page-specific cosmetic overrides.

Use a 16px reading size for full procedures, 14–15px for concise interface descriptions, 14px action labels, and 11–13px only for secondary metadata. Maintenance commands and safety text must remain legible; density does not justify the previous 9–11px body text. Phone layouts wrap categories and actions, turn records into readable vertical groups, and preserve every field. A horizontal scrollbar is appropriate inside a long command, never for the whole page.

The reusable CSS describes presentation. Application logic follows the mandatory [Rust implementation policy](https://github.com/Herbertofury/INTA/blob/main/docs/rust-policy.md); visual consistency does not establish native runtime or device-test completion. Desktop, Mobile, Rescue and Site still require their own executable acceptance evidence.
