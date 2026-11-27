<a href="https://herbertofury.github.io/INTA/" title="Open INTA official download or service page" aria-label="Open INTA official download or service page" data-inta-brand="app-inta"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA official page" width="18" height="18" valign="middle" /></a> [INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Service map](https://github.com/Herbertofury/INTA/wiki/Service-Map) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

# <a href="https://herbertofury.github.io/INTA/" title="Open INTA official download or service page" aria-label="Open INTA official download or service page" data-inta-brand="app-inta"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA official page" width="18" height="18" valign="middle" /></a> [INTA](https://herbertofury.github.io/INTA/) Command Center

**Purpose:** one searchable home for <a href="https://github.com/PowerShell/PowerShell/releases/latest" title="Open PowerShell official download or service page" aria-label="Open PowerShell official download or service page" data-inta-brand="powershell"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="PowerShell official page" width="18" height="18" valign="middle" /></a> [PowerShell](https://learn.microsoft.com/en-us/powershell/), CMD and <a href="https://www.kernel.org/" title="Open Linux official download or service page" aria-label="Open Linux official download or service page" data-inta-brand="app-linux"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/c274b9b1ea45948db04e.png" alt="Linux official page" width="18" height="18" valign="middle" /></a> [Linux](https://www.kernel.org/) commands, guided technician workflows, package managers, application updates and trusted all-in-one utilities.

## Where commands live

The canonical catalog is `data/command-center.json`. Each command carries a stable ID, shell, privilege level, category, risk class, summary, exact code, safety boundary and official documentation. Workflows reference command IDs so they reuse the same vetted command rather than duplicating it. Toolkit cards link to the maintained upstream project and point to a setup or review command when one is appropriate.

The interactive app exposes the catalog at **Command Center**. The command library supports search, shell filtering, category tabs and impact filtering. Every command has a copy-to-clipboard control. Copying never executes a command.

## Package managers are separate update channels

### <a href="https://github.com/microsoft/winget-cli/releases/latest" title="Open WinGet official download or service page" aria-label="Open WinGet official download or service page" data-inta-brand="winget"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="WinGet official page" width="18" height="18" valign="middle" /></a> [WinGet](https://learn.microsoft.com/en-us/windows/package-manager/winget/)

Preview first:

```powershell
winget upgrade
```

Then apply updates interactively:

```powershell
winget upgrade --all --interactive
```

<a href="https://github.com/microsoft/winget-cli/releases/latest" title="Open WinGet official download or service page" aria-label="Open WinGet official download or service page" data-inta-brand="winget"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="WinGet official page" width="18" height="18" valign="middle" /></a> [WinGet](https://learn.microsoft.com/en-us/windows/package-manager/winget/) does not manage every installed app. Some packages have unknown versions, some are pinned, and some require vendor-specific update tools. Review package identity, source, publisher and pins before using broader options such as `--include-unknown`. <a href="https://www.microsoft.com/" title="Open microsoft.com official download or service page" aria-label="Open microsoft.com official download or service page" data-inta-brand="site-www-microsoft-com"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="microsoft.com official page" width="18" height="18" valign="middle" /></a> [Microsoft upgrade documentation](https://learn.microsoft.com/en-us/windows/package-manager/winget/upgrade).

### Chocolatey

Inspect candidates:

```powershell
choco outdated
choco upgrade all --noop
```

After reviewing package IDs, sources, scripts, pins and the change window:

```powershell
choco upgrade all --yes
```

Chocolatey package scripts can run with elevated privileges and can change the machine. Do not add `--force` or bypass pins as a default. The `--noop` option is a preview, not a substitute for inspecting package metadata. [Chocolatey command reference](https://docs.chocolatey.org/en-us/choco/commands/).

Install Chocolatey through its documented <a href="https://github.com/microsoft/winget-cli/releases/latest" title="Open WinGet official download or service page" aria-label="Open WinGet official download or service page" data-inta-brand="winget"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="WinGet official page" width="18" height="18" valign="middle" /></a> [WinGet](https://learn.microsoft.com/en-us/windows/package-manager/winget/) package route when available:

```powershell
winget show --id chocolatey.chocolatey --exact --source winget
winget install --id chocolatey.chocolatey --exact --source winget --interactive
```

Chocolatey's own setup documentation also describes bootstrap script methods and explicitly recommends inspecting remote scripts before execution. <a href="https://herbertofury.github.io/INTA/" title="Open INTA official download or service page" aria-label="Open INTA official download or service page" data-inta-brand="app-inta"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA official page" width="18" height="18" valign="middle" /></a> [INTA](https://herbertofury.github.io/INTA/) does not use a remote download piped directly into an interpreter. [Chocolatey setup](https://docs.chocolatey.org/en-us/choco/setup/).

### Scoop

Preview the managed app state:

```powershell
scoop status
```

Refresh Scoop and its managed apps only after reviewing the candidate list:

```powershell
scoop update
scoop update *
```

Scoop only covers apps installed through Scoop and configured buckets. Review bucket provenance and manifests. <a href="https://github.com/" title="Open GitHub official download or service page" aria-label="Open GitHub official download or service page" data-inta-brand="app-github"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/99f4edb33272b70febb8.png" alt="GitHub official page" width="18" height="18" valign="middle" /></a> [Scoop project](https://github.com/ScoopInstaller/Scoop).

### <a href="https://learn.microsoft.com/en-us/windows/deployment/update/" title="Open Windows Update official download or service page" aria-label="Open Windows Update official download or service page" data-inta-brand="app-windows-update"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="Windows Update official page" width="18" height="18" valign="middle" /></a> [Windows Update](https://learn.microsoft.com/en-us/windows/deployment/update/) and <a href="https://www.microsoft.com/" title="Open microsoft.com official download or service page" aria-label="Open microsoft.com official download or service page" data-inta-brand="site-www-microsoft-com"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="microsoft.com official page" width="18" height="18" valign="middle" /></a> [Microsoft](https://www.microsoft.com/) Store

OS servicing and Store app updates are separate from <a href="https://github.com/microsoft/winget-cli/releases/latest" title="Open WinGet official download or service page" aria-label="Open WinGet official download or service page" data-inta-brand="winget"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="WinGet official page" width="18" height="18" valign="middle" /></a> [WinGet](https://learn.microsoft.com/en-us/windows/package-manager/winget/), Chocolatey and Scoop. Open the supported interfaces:

```powershell
Start-Process 'ms-settings:windowsupdate'
Start-Process 'ms-windows-store://downloadsandupdates'
```

Respect managed update policies, safeguard holds, active hours and reboot coordination.

### <a href="https://www.kernel.org/" title="Open Linux official download or service page" aria-label="Open Linux official download or service page" data-inta-brand="app-linux"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/c274b9b1ea45948db04e.png" alt="Linux official page" width="18" height="18" valign="middle" /></a> [Linux](https://www.kernel.org/) package managers

Use the package manager that belongs to the distribution:
- <a href="https://www.debian.org/" title="Open Debian official download or service page" aria-label="Open Debian official download or service page" data-inta-brand="app-debian"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/98c05baf7fde67ee3abe.png" alt="Debian official page" width="18" height="18" valign="middle" /></a> [Debian](https://www.debian.org/)/<a href="https://ubuntu.com/" title="Open Ubuntu official download or service page" aria-label="Open Ubuntu official download or service page" data-inta-brand="app-ubuntu"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/f6faba46c59b19156b87.png" alt="Ubuntu official page" width="18" height="18" valign="middle" /></a> [Ubuntu](https://ubuntu.com/): `sudo apt update`, inspect `apt list --upgradable`, then `sudo apt upgrade`.
- Fedora/RHEL: `sudo dnf check-update`, then `sudo dnf upgrade --refresh`.
- Arch: read current news and use full system upgrades only; `sudo pacman -Syu`. Do not perform partial upgrades.

## Trusted technician toolkits

### <a href="https://learn.microsoft.com/en-us/sysinternals/downloads/sysinternals-suite" title="Open Microsoft Sysinternals Suite official download or service page" aria-label="Open Microsoft Sysinternals Suite official download or service page" data-inta-brand="sysinternals"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="Microsoft Sysinternals Suite official page" width="18" height="18" valign="middle" /></a> [Microsoft Sysinternals Suite](https://learn.microsoft.com/en-us/sysinternals/)

Use the official suite for <a href="https://learn.microsoft.com/en-us/sysinternals/downloads/process-explorer" title="Open Process Explorer official download or service page" aria-label="Open Process Explorer official download or service page" data-inta-brand="process-explorer"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="Process Explorer official page" width="18" height="18" valign="middle" /></a> [Process Explorer](https://learn.microsoft.com/en-us/sysinternals/downloads/process-explorer), <a href="https://learn.microsoft.com/en-us/sysinternals/downloads/procmon" title="Open Process Monitor official download or service page" aria-label="Open Process Monitor official download or service page" data-inta-brand="procmon"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="Process Monitor official page" width="18" height="18" valign="middle" /></a> [Process Monitor](https://learn.microsoft.com/en-us/sysinternals/downloads/procmon), <a href="https://learn.microsoft.com/en-us/sysinternals/downloads/autoruns" title="Open Autoruns official download or service page" aria-label="Open Autoruns official download or service page" data-inta-brand="autoruns"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="Autoruns official page" width="18" height="18" valign="middle" /></a> [Autoruns](https://learn.microsoft.com/en-us/sysinternals/downloads/autoruns), <a href="https://learn.microsoft.com/en-us/sysinternals/downloads/rammap" title="Open RAMMap official download or service page" aria-label="Open RAMMap official download or service page" data-inta-brand="rammap"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="RAMMap official page" width="18" height="18" valign="middle" /></a> [RAMMap](https://learn.microsoft.com/en-us/sysinternals/downloads/rammap), <a href="https://learn.microsoft.com/en-us/sysinternals/downloads/tcpview" title="Open TCPView official download or service page" aria-label="Open TCPView official download or service page" data-inta-brand="tcpview"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="TCPView official page" width="18" height="18" valign="middle" /></a> [TCPView](https://learn.microsoft.com/en-us/sysinternals/downloads/tcpview) and related utilities. Prefer exact package identity and verify the publisher. <a href="https://learn.microsoft.com/en-us/sysinternals/downloads/sysinternals-suite" title="Open Microsoft Sysinternals Suite official download or service page" aria-label="Open Microsoft Sysinternals Suite official download or service page" data-inta-brand="sysinternals"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="Microsoft Sysinternals Suite official page" width="18" height="18" valign="middle" /></a> [Official Sysinternals downloads](https://learn.microsoft.com/en-us/sysinternals/downloads/sysinternals-suite).

### Chris Titus Tech WinUtil

WinUtil combines app installs, tweaks, fixes, configuration and <a href="https://learn.microsoft.com/en-us/windows/deployment/update/" title="Open Windows Update official download or service page" aria-label="Open Windows Update official download or service page" data-inta-brand="app-windows-update"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="Windows Update official page" width="18" height="18" valign="middle" /></a> [Windows update](https://learn.microsoft.com/en-us/windows/deployment/update/) controls. It requires administrator privileges for system-wide changes. <a href="https://herbertofury.github.io/INTA/" title="Open INTA official download or service page" aria-label="Open INTA official download or service page" data-inta-brand="app-inta"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA official page" width="18" height="18" valign="middle" /></a> [INTA](https://herbertofury.github.io/INTA/) provides a download-and-inspect workflow rather than piping remote script content directly into `iex`. Review the code, upstream release, preset contents and undo paths before running. Do not blindly apply debloat/privacy presets to managed customer endpoints. <a href="https://github.com/" title="Open GitHub official download or service page" aria-label="Open GitHub official download or service page" data-inta-brand="app-github"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/99f4edb33272b70febb8.png" alt="GitHub official page" width="18" height="18" valign="middle" /></a> [WinUtil repository](https://github.com/ChrisTitusTech/winutil) · <a href="https://github.com/" title="Open GitHub official download or service page" aria-label="Open GitHub official download or service page" data-inta-brand="app-github"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/99f4edb33272b70febb8.png" alt="GitHub official page" width="18" height="18" valign="middle" /></a> [Official user guide](https://github.com/ChrisTitusTech/winutil/tree/main/docs).

### Tron Script

Tron is a long-running, multi-stage cleanup, disinfection, repair and patch workflow. Its maintainer warns that the <a href="https://github.com/" title="Open GitHub official download or service page" aria-label="Open GitHub official download or service page" data-inta-brand="app-github"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/99f4edb33272b70febb8.png" alt="GitHub official page" width="18" height="18" valign="middle" /></a> [GitHub](https://github.com/) source repository is not the complete runnable package; follow the maintainer's distribution instructions and verify release checksums/signatures. Review its stages, reboot prerequisites, data impact, scan behavior, logs and skip switches. It can take hours and can remove software or data. Never treat it as a one-click generic repair. <a href="https://github.com/" title="Open GitHub official download or service page" aria-label="Open GitHub official download or service page" data-inta-brand="app-github"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/99f4edb33272b70febb8.png" alt="GitHub official page" width="18" height="18" valign="middle" /></a> [Tron source and usage notes](https://github.com/bmrf/tron) · <a href="https://www.reddit.com/" title="Open reddit.com official download or service page" aria-label="Open reddit.com official download or service page" data-inta-brand="site-www-reddit-com"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/f8f762f531fa6a64964b.png" alt="reddit.com official page" width="18" height="18" valign="middle" /></a> [Maintainer distribution community](https://www.reddit.com/r/TronScript/).

### Chocolatey, <a href="https://github.com/microsoft/winget-cli/releases/latest" title="Open WinGet official download or service page" aria-label="Open WinGet official download or service page" data-inta-brand="winget"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="WinGet official page" width="18" height="18" valign="middle" /></a> [WinGet](https://learn.microsoft.com/en-us/windows/package-manager/winget/), Scoop and <a href="https://github.com/Devolutions/UniGetUI/releases/latest" title="Open UniGetUI official download or service page" aria-label="Open UniGetUI official download or service page" data-inta-brand="unigetui"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/99f4edb33272b70febb8.png" alt="UniGetUI official page" width="18" height="18" valign="middle" /></a> [UniGetUI](https://github.com/Devolutions/UniGetUI)

These tools solve overlapping but distinct package-management problems. <a href="https://herbertofury.github.io/INTA/" title="Open INTA official download or service page" aria-label="Open INTA official download or service page" data-inta-brand="app-inta"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA official page" width="18" height="18" valign="middle" /></a> [INTA](https://herbertofury.github.io/INTA/) does not merge their package databases or blindly update every app through every manager. <a href="https://github.com/Devolutions/UniGetUI/releases/latest" title="Open UniGetUI official download or service page" aria-label="Open UniGetUI official download or service page" data-inta-brand="unigetui"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/99f4edb33272b70febb8.png" alt="UniGetUI official page" width="18" height="18" valign="middle" /></a> [UniGetUI](https://github.com/Devolutions/UniGetUI) provides a graphical interface to supported package managers; confirm which managers and sources are enabled. <a href="https://github.com/Devolutions/UniGetUI/releases/latest" title="Open UniGetUI official download or service page" aria-label="Open UniGetUI official download or service page" data-inta-brand="unigetui"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/99f4edb33272b70febb8.png" alt="UniGetUI official page" width="18" height="18" valign="middle" /></a> [UniGetUI project](https://github.com/Devolutions/UniGetUI).

## Safety model

- Commands are copied, never automatically executed by opening a guide or clicking Copy.
- Read-only diagnostics are clearly separated from installs, repairs and long-running scans.
- Bulk updates require a preview and explicit technician action.
- No silent bulk updates by default; no bypass of package pins, hashes, signatures or publisher checks.
- Do not run remote script download-and-execute pipelines such as `irm ... | iex` as a generic install pattern.
- Separate application updates, <a href="https://learn.microsoft.com/en-us/windows/deployment/update/" title="Open Windows Update official download or service page" aria-label="Open Windows Update official download or service page" data-inta-brand="app-windows-update"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="Windows Update official page" width="18" height="18" valign="middle" /></a> [Windows Update](https://learn.microsoft.com/en-us/windows/deployment/update/), <a href="https://www.microsoft.com/" title="Open microsoft.com official download or service page" aria-label="Open microsoft.com official download or service page" data-inta-brand="site-www-microsoft-com"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="microsoft.com official page" width="18" height="18" valign="middle" /></a> [Microsoft](https://www.microsoft.com/) Store updates, firmware and driver updates.
- Never run broad cleanup/debloat scripts on customer devices without explicit authorization, a backup/recovery plan and review of every selected stage.
- For suspected ransomware, preserve evidence and follow the incident playbook; do not run generic cleanup tools first.
- For failing storage, image first and avoid write repairs against the source.
- Record the command, source, date, exit result and verification in the approved private ticket. Sanitize logs before external sharing.

## Adding a command

Add an entry to `data/command-center.json` with:
- unique stable ID;
- exact supported shell and privilege requirement;
- category and impact/risk class;
- concise purpose and expected outcome;
- exact command text;
- clear safety/rollback guidance;
- official HTTPS reference.

Add it to a workflow only by referencing its ID. The build validates command metadata, source URLs, workflow references, toolkit links and prohibited remote execution patterns.

## Acceptance criteria

1. One dedicated Command Center tab is available from the app's primary navigation.
2. Search, category, shell and impact filters work together.
3. Commands can be copied individually and as workflow bundles.
4. Toolkit cards link to the maintained upstream project and provide setup/review commands where appropriate.
5. <a href="https://github.com/microsoft/winget-cli/releases/latest" title="Open WinGet official download or service page" aria-label="Open WinGet official download or service page" data-inta-brand="winget"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="WinGet official page" width="18" height="18" valign="middle" /></a> [WinGet](https://learn.microsoft.com/en-us/windows/package-manager/winget/), Chocolatey, Scoop, <a href="https://learn.microsoft.com/en-us/windows/deployment/update/" title="Open Windows Update official download or service page" aria-label="Open Windows Update official download or service page" data-inta-brand="app-windows-update"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="Windows Update official page" width="18" height="18" valign="middle" /></a> [Windows Update](https://learn.microsoft.com/en-us/windows/deployment/update/), Store updates and <a href="https://www.kernel.org/" title="Open Linux official download or service page" aria-label="Open Linux official download or service page" data-inta-brand="app-linux"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/c274b9b1ea45948db04e.png" alt="Linux official page" width="18" height="18" valign="middle" /></a> [Linux](https://www.kernel.org/) package managers remain distinct channels.
6. Bulk updates have preview-first instructions and explicit safety boundaries.
7. The app, generated site, Wiki and offline artifact use the same canonical catalog.
8. Tests validate IDs, shells, categories, risk levels, source URLs, workflow references and toolkit references.

### Ninite Pro

Ninite Pro offers commercial application deployment and update workflows. Its documented Pro Classic command-line features include app selection, audit, update-only, exclusions and reports. The newer Pro web interface may differ; verify the current account documentation before automation. [Ninite Pro command-line reference](https://ninite.com/help/features/selection.html) · [Ninite help](https://ninite.com/help/).

### Patch My PC Home Updater

Home Updater is a consumer product and is restricted to non-commercial use. <a href="https://herbertofury.github.io/INTA/" title="Open INTA official download or service page" aria-label="Open INTA official download or service page" data-inta-brand="app-inta"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA official page" width="18" height="18" valign="middle" /></a> [INTA](https://herbertofury.github.io/INTA/) lists it only as a reference and must not recommend it for technician/customer deployments. Use Patch My PC's commercial offerings for business endpoints. [Home Updater licensing and FAQ](https://patchmypc.com/faq/category/home-updater/).

---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
