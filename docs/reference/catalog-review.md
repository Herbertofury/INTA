# INTA technician catalog research

**Reviewed: November 26, 2026.** Prepared for the INTA Computer Services technician hub and wiki.

## Delivered catalog

The catalog contains **111 practical tools and entry points** across malware removal, remote support, Windows diagnostics and repair, storage and backup, hardware testing, networking, bootable media, daily technician utilities, AI assistance and advanced defensive analysis. Each entry has a stable ID, job-focused description, best-use guidance, platforms, usage mode, pricing context, official download route, documentation, review date, tier, search tags and provenance.

**Core**, **specialist** and **evaluate** are editorial workflow categories. They do not assert measured detection or performance superiority. The catalog contains no invented newest-version numbers and makes no claim that all applications were installed or benchmarked here. Download routes use official current download pages or release channels.

## Preserved project knowledge

- ATHENA's current repository is `Herbertofury/Athena`. The checked main reference is commit `7d529423b52d3cc6104ce7469eec358f5dc9b9ac`.
- Its endpoint-security guide now lives at `engineering/docs/windows-endpoint-security.md`; the earlier `docs/windows-endpoint-security.md` path is no longer the current path.
- The catalog preserves **32 ATHENA lineage links**, including Sysinternals-related workflow tools, System Informer, utilities, backup tools and the endpoint-security research candidates.
- **13 entries** also reference the existing INTA live-image package recipe at commit `1ecba1129aac275a02f40b9c4f5bea438c9e2688`. These are documented recipe inclusions, not a new ISO boot-test claim.
- Previously discussed tools are represented: Microsoft scanners, Malwarebytes and AdwCleaner, HitmanPro, ESET, Bitdefender Free and GravityZone, MeshCentral, RustDesk, Quick Assist, Chrome Remote Desktop, Rufus, Ventoy, Hiren's BootCD PE, SystemRescue, ChatGPT, Codex CLI and Claude Code.

## Source findings that changed the catalog

| Finding | Catalog decision | Primary evidence |
| --- | --- | --- |
| Microsoft Safety Scanner expires ten days after download. | Tell technicians to acquire a fresh copy for each job and distinguish it from resident protection. | [Microsoft](https://learn.microsoft.com/en-us/defender-endpoint/safety-scanner-download) |
| HWiNFO 64/ARM64 free use is non-commercial. | Label the commercial Pro requirement directly in the pricing field. | [HWiNFO licenses](https://www.hwinfo.com/licenses/) |
| FreeFileSync requires its Business Edition for commercial users. | Avoid labeling its technician use simply free. | [FreeFileSync FAQ](https://freefilesync.org/faq.php) |
| Emsisoft Emergency Kit has free private use and Pro business use. | Include the professional portable option with accurate pricing context. | [Emsisoft](https://www.emsisoft.com/en/help/1702/getting-started/) |
| Quick Assist's normal third-party support workflow is Windows; macOS is for Microsoft Support interactions. | List Windows as the general INTA platform. | [Microsoft Quick Assist](https://learn.microsoft.com/en-us/windows/client-management/client-tools/quick-assist) |
| Native Windows is not an officially supported upstream iperf3 platform. | Recommend the Linux rescue environment for the upstream path, with no invented official Windows binary. | [ESnet](https://software.es.net/iperf/) |
| The official Hiren's ISO lists older bundled applications. | Keep ISO freshness and tool freshness separate. | [Hiren's contents](https://www.hirensbootcd.org/download/) |
| Sophos' legacy virus-removal route redirects to HitmanPro. | Use the current HitmanPro entry; avoid a stale separate Scan & Clean download claim. | [Sophos free tools](https://www.sophos.com/en-us/free-tools) |
| Microsoft says WinSAT may change or be unavailable after Windows 8.1. | Retain the reference as **WinSAT (legacy)** in the evaluate tier; prefer WPR/WPA for current diagnosis. | [Microsoft WinSAT](https://learn.microsoft.com/en-us/windows/win32/winsat/windows-system-assessment-tool-portal) |
| Chainsaw moved to the WithSecureOpenSource organization. | Normalize homepage and release links to the current upstream. | [Chainsaw](https://github.com/WithSecureOpenSource/chainsaw) |

## ATHENA research is visible without overstating readiness

YARA-X, PE-sieve, HollowsHunter and Fibratus are documented as specialist investigation tools with their actual purpose and interpretation limits. PRX-SD, Rustinel and Sentinella stay in the evaluation tier. ATHENA's research does not establish that they are production-ready replacements for Defender or that Rust itself guarantees better detection or lower overhead. The source guide explicitly separates file scanning, snapshot memory inspection, event collection and prevention.

## Verification and limits

The JSON was parsed and checked for unique IDs, required fields, supported tier/mode values, valid HTTPS URLs and absence of tracking parameters. Official primary-source documentation or current project pages were used for the tool descriptions. Failed direct retrieval routes were checked through alternate official pages or official repositories, and retained in `source-checks.json` as retrieval limitations rather than product-absence claims.

Some documentation endpoints require authentication or block automated retrieval. The catalog uses an accessible official evidence route for those entries; links such as the Notepad++ user manual can still be valid for a human browser despite an automated 403. No executable signatures, installer behavior, runtime compatibility or live boot-media image were validated by this research subtask. Those are separate acceptance gates for actual deployment.

The source register maps every entry to its official download and evidence. The catalog can be refreshed on demand without automatically changing client systems, replacing binaries, or scheduling background agents.
