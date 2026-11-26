# Technician hub validation

## Content and build

- 111 tool records, 22 job guides and all cross-references validated.
- 132 ordered workflow steps and 72 verification checks in canonical guide data.
- 41 generated Wiki files, including navigation and footer.
- Deterministic generator verifies committed outputs match source.
- 11 targeted Python tests pass, including all pre-existing Media Creator tests.
- All local Markdown links checked after integration.
- Official source provenance and the editorial review date travel with the catalog.

## Browser and deployment

The actual 111 tools and 22 guides passed browser checks in Chromium 158. Search and all recommendation filters work, browser Back preserves filters, favorites and checklist progress persist, reset Undo works, Markdown exports preserve checked progress, and the offline export reopens with all 111 tools. At 390 px width the layout has no horizontal overflow and all tools remain available. The suite recorded zero JavaScript errors and zero external requests for the offline workspace. Exact source hashes are recorded in `reference/browser-checks.json`.

The published GitHub Pages site was opened and exercised independently: all 111 cards rendered, the evaluation filter returned 4 tools, RustDesk search returned one matching card and opened the correct detail page.

## Material challenge pass

Corrected guide/tool mismatches for DISM/SFC/WinRE, Windows recovery media and WinDbg. Scoped PowerShell commands explicitly. Clarified Sophos Linux as a distinct deployment. Preserved failing-drive imaging before repair, separation of routine malware cleanup from incident response, and classic/new Outlook boundaries. Fixed the core-tool filter option markup during UI review.

## Scope of this proof

These checks validate the hub, guide consistency and Media Creator regression tests. They do not certify third-party binaries, benchmark antivirus efficacy, or prove physical USB/Secure Boot compatibility. The separate Rescue pipeline remains responsible for image build and VM boot evidence.
