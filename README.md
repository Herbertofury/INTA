<p align="center"><img src="docs/assets/inta-banner.svg" alt="INTA Computer Services: Every fix starts here" width="100%"></p>

<p align="center"><strong>The shared technician home for INTA Computer Services.</strong><br>Choose the job. Find the tool. Follow the method. Verify the fix.</p>

<p align="center"><a href="https://herbertofury.github.io/INTA/"><strong>OPEN THE TECHNICIAN HUB →</strong></a> · <a href="https://github.com/Herbertofury/INTA/wiki">Read the Wiki</a> · <a href="https://github.com/Herbertofury/INTA/releases">Rescue releases</a></p>

---

## A home for the whole service desk

INTA brings **111 purpose-selected tools**, **22 complete job playbooks**, official downloads, and practical field knowledge into one searchable workspace. It covers everyday Windows service, malware removal, remote support, hardware diagnostics, data recovery, networking, boot media, backup, productivity and AI-assisted troubleshooting. Selected discoveries from **[Athena](https://github.com/Herbertofury/Athena)** retain their provenance and evaluation status.

The browser hub works online and as a single offline HTML file. The live GitHub Wiki provides the same job guidance in a readable, linked reference with a sidebar, tool comparisons and source links. INTA Rescue and its Media Creator remain in this repository as the bootable recovery side of the toolkit.

## What are you fixing?

| Your job | Open the guide | The result you are aiming for |
| --- | --- | --- |
| A new service request | [Intake & triage](https://github.com/Herbertofury/INTA/wiki/Job-intake-triage) | A clear problem, protected data and a testable outcome. |
| Suspected infection | [Malware removal](https://github.com/Herbertofury/INTA/wiki/Job-malware-removal) | Containment, purposeful scanning and a supported cleanup or rebuild decision. |
| Active ransomware | [Ransomware response](https://github.com/Herbertofury/INTA/wiki/Job-ransomware-response) | Containment, preserved evidence and a controlled recovery path. |
| A slow or unstable PC | [Slow PC](https://github.com/Herbertofury/INTA/wiki/Job-slow-pc) · [Hardware / BSOD](https://github.com/Herbertofury/INTA/wiki/Job-hardware-bsod) | An identified bottleneck or fault and a measured, repeatable fix. |
| Broken Windows or updates | [Windows repair](https://github.com/Herbertofury/INTA/wiki/Job-windows-repair) · [Updates](https://github.com/Herbertofury/INTA/wiki/Job-windows-update) | Correct servicing, healthy components and verified restart behavior. |
| A machine that will not boot | [Boot recovery](https://github.com/Herbertofury/INTA/wiki/Job-boot-recovery) | The right recovery environment and a data-conscious boot repair. |
| A failing disk or lost files | [Drive recovery](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive) | Data preservation before filesystem repair attempts. |
| An SSD upgrade or restore | [Migration](https://github.com/Herbertofury/INTA/wiki/Job-disk-migration) · [Backup / restore](https://github.com/Herbertofury/INTA/wiki/Job-backup-restore) | A verified copy, successful boot and a real restore test. |
| Connectivity or an outage | [Network / DNS](https://github.com/Herbertofury/INTA/wiki/Job-network-dns) · [Outage recovery](https://github.com/Herbertofury/INTA/wiki/Job-outage-recovery) | A traced failure boundary and service restored in dependency order. |
| Supporting someone remotely | [Remote support](https://github.com/Herbertofury/INTA/wiki/Job-remote-support) | The right attended or managed connection with a clear handoff. |
| Office and workplace issues | [Outlook](https://github.com/Herbertofury/INTA/wiki/Job-outlook-email) · [Printers](https://github.com/Herbertofury/INTA/wiki/Job-printer-repair) · [AD trust](https://github.com/Herbertofury/INTA/wiki/Job-ad-trust) | Diagnosis at the correct device, account or service layer. |
| Preparing the next job | [New PC](https://github.com/Herbertofury/INTA/wiki/Job-new-pc) · [Maintenance](https://github.com/Herbertofury/INTA/wiki/Job-preventive-maintenance) · [USB media](https://github.com/Herbertofury/INTA/wiki/Job-boot-usb) | A ready kit and a machine that is ready for real use. |

**[Browse all 22 playbooks →](https://github.com/Herbertofury/INTA/wiki/Playbooks)**

## Your workspace, with the useful details built in

- **Search the complete catalog.** Filter by discipline, platform and kit level. Every matching tool remains available.
- **Choose with context.** Each tool has a best-use case, delivery type, platform details, pricing notes, official download, documentation and source.
- **Follow a real workflow.** Each guide includes prerequisites, ordered actions, verification, stop conditions and official references.
- **Make it yours.** Save favorite tools, keep checklist progress, switch dark/light themes, print a guide or export a Markdown checklist.
- **Take the entire hub offline.** Download the source archive and open `site/index.html`, or export an offline workspace in Resources. No application server or package installation is needed to read it.
- **Keep improving it.** Maintainer scripts generate the hub and Wiki from the same data. Issue forms collect tool recommendations and procedure corrections.

Saved tools and checklist progress live in your browser. Keep the actual customer ticket and evidence in the team's approved private system. External downloads, documentation and online AI still need a connection.

## The toolkit

| Discipline | Examples |
| --- | --- |
| Malware removal & protection | Defender Offline, Safety Scanner, Malwarebytes, AdwCleaner, HitmanPro, ESET, Bitdefender, ClamAV. |
| Remote support | Quick Assist, RustDesk, MeshCentral, Chrome Remote Desktop and managed-service alternatives. |
| Windows diagnosis & repair | Sysinternals, Process Monitor, Autoruns, Windows Performance Toolkit and official Windows recovery tools. |
| Storage & recovery | SMART tools, CrystalDiskInfo, GNU ddrescue, TestDisk / PhotoRec, imaging and backup tools. |
| Hardware & networking | Memory and hardware diagnostics, Wireshark, local network diagnostics and monitoring. |
| Boot & service utilities | INTA Rescue, Ventoy, Rufus, Hiren's BootCD PE, SystemRescue and Rescuezilla. |
| AI & specialist research | ChatGPT, Codex, Claude Code, YARA-X, PE-sieve, HollowsHunter and carefully labeled Athena candidates. |

**Core** means an everyday selection. **Specialist** means a specific advanced use. **Evaluate** means a research candidate awaiting the team's own validation. The catalog does not claim every entry is installed in the ISO or that every vendor tool has been runtime-tested by this project.

## INTA Rescue & Media Creator

INTA Rescue is the separate x86-64 live recovery desktop for disk inspection, imaging, file recovery, hardware diagnosis and online AI assistance. The Media Creator verifies release checksums, downloads or refreshes the ISO, copies it to a mounted Ventoy volume, and opens the official Rufus/Ventoy utilities.

**[Prepare rescue media](docs/rescue-media.md)** · **[Rescue build and usage guide](docs/rescue.md)** · **[Installed-image tool overview](docs/tools.md)** · **[Build and boot checks](https://github.com/Herbertofury/INTA/actions/workflows/rescue.yml)**

Windows-native applications need Windows or suitable Windows recovery media. Use the ISO's package manifest to see exactly what a particular image contains. BIOS/UEFI VM checks and physical hardware/Secure Boot verification are distinct evidence.

## Stay current without mystery updates

Use official publisher links at the point of service. The catalog records its editorial review date separately from tool versions. The on-demand upstream checker reports releases and unresolved checks without silently installing or promoting software:

```sh
python3 scripts/check-upstreams.py --output reports/upstreams.json
```

The existing weekly Rescue image build remains separate from hub publication. Documentation-only changes do not cancel an in-progress ISO build. **[Read the maintenance workflow →](docs/maintaining-the-hub.md)**

## Contribute to the bench

```sh
python3 scripts/build-hub.py
python3 -m unittest discover -s tests
python3 scripts/build-hub.py --check
```

Edit `data/tools.json` or `data/jobs.json`, regenerate the outputs, review the changed guidance and publish. The hub deploys through GitHub Pages; `scripts/publish-wiki.py` synchronizes the generated pages to the real Wiki with an authenticated maintainer Git session.

**[Suggest a tool or correction](https://github.com/Herbertofury/INTA/issues/new/choose)** · **[New technician orientation](docs/start-here.md)** · **[Intake & handoff template](docs/technician-handoff.md)**

---

**INTA Computer Services** · Built for the people behind the fix.  
Editorial review: **November 26, 2026**. Complete source and provenance remain in this repository.
