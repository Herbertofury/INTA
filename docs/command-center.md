# INTA Command Center

**Purpose:** one searchable home for PowerShell, CMD and Linux commands, guided technician workflows, package managers, application updates and trusted all-in-one utilities.

## Where commands live

The canonical catalog is `data/command-center.json`. Each command carries a stable ID, shell, privilege level, category, risk class, summary, exact code, safety boundary and official documentation. Workflows reference command IDs so they reuse the same vetted command rather than duplicating it. Toolkit cards link to the maintained upstream project and point to a setup or review command when one is appropriate.

The interactive app exposes the catalog at **Command Center**. The command library supports search, shell filtering, category tabs and impact filtering. Every command has a copy-to-clipboard control. Copying never executes a command.

## Package managers are separate update channels

### WinGet

Preview first:

```powershell
winget upgrade
```

Then apply updates interactively:

```powershell
winget upgrade --all --interactive
```

WinGet does not manage every installed app. Some packages have unknown versions, some are pinned, and some require vendor-specific update tools. Review package identity, source, publisher and pins before using broader options such as `--include-unknown`. [Microsoft upgrade documentation](https://learn.microsoft.com/en-us/windows/package-manager/winget/upgrade).

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

Install Chocolatey through its documented WinGet package route when available:

```powershell
winget show --id chocolatey.chocolatey --exact --source winget
winget install --id chocolatey.chocolatey --exact --source winget --interactive
```

Chocolatey's own setup documentation also describes bootstrap script methods and explicitly recommends inspecting remote scripts before execution. INTA does not use a remote download piped directly into an interpreter. [Chocolatey setup](https://docs.chocolatey.org/en-us/choco/setup/).

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

Scoop only covers apps installed through Scoop and configured buckets. Review bucket provenance and manifests. [Scoop project](https://github.com/ScoopInstaller/Scoop).

### Windows Update and Microsoft Store

OS servicing and Store app updates are separate from WinGet, Chocolatey and Scoop. Open the supported interfaces:

```powershell
Start-Process 'ms-settings:windowsupdate'
Start-Process 'ms-windows-store://downloadsandupdates'
```

Respect managed update policies, safeguard holds, active hours and reboot coordination.

### Linux package managers

Use the package manager that belongs to the distribution:
- Debian/Ubuntu: `sudo apt update`, inspect `apt list --upgradable`, then `sudo apt upgrade`.
- Fedora/RHEL: `sudo dnf check-update`, then `sudo dnf upgrade --refresh`.
- Arch: read current news and use full system upgrades only; `sudo pacman -Syu`. Do not perform partial upgrades.

## Trusted technician toolkits

### Microsoft Sysinternals Suite

Use the official suite for Process Explorer, Process Monitor, Autoruns, RAMMap, TCPView and related utilities. Prefer exact package identity and verify the publisher. [Official Sysinternals downloads](https://learn.microsoft.com/en-us/sysinternals/downloads/sysinternals-suite).

### Chris Titus Tech WinUtil

WinUtil combines app installs, tweaks, fixes, configuration and Windows update controls. It requires administrator privileges for system-wide changes. INTA provides a download-and-inspect workflow rather than piping remote script content directly into `iex`. Review the code, upstream release, preset contents and undo paths before running. Do not blindly apply debloat/privacy presets to managed customer endpoints. [WinUtil repository](https://github.com/ChrisTitusTech/winutil) · [Official user guide](https://github.com/ChrisTitusTech/winutil/tree/main/docs).

### Tron Script

Tron is a long-running, multi-stage cleanup, disinfection, repair and patch workflow. Its maintainer warns that the GitHub source repository is not the complete runnable package; follow the maintainer's distribution instructions and verify release checksums/signatures. Review its stages, reboot prerequisites, data impact, scan behavior, logs and skip switches. It can take hours and can remove software or data. Never treat it as a one-click generic repair. [Tron source and usage notes](https://github.com/bmrf/tron) · [Maintainer distribution community](https://www.reddit.com/r/TronScript/).

### Chocolatey, WinGet, Scoop and UniGetUI

These tools solve overlapping but distinct package-management problems. INTA does not merge their package databases or blindly update every app through every manager. UniGetUI provides a graphical interface to supported package managers; confirm which managers and sources are enabled. [UniGetUI project](https://github.com/Devolutions/UniGetUI).

## Safety model

- Commands are copied, never automatically executed by opening a guide or clicking Copy.
- Read-only diagnostics are clearly separated from installs, repairs and long-running scans.
- Bulk updates require a preview and explicit technician action.
- No silent bulk updates by default; no bypass of package pins, hashes, signatures or publisher checks.
- Do not run remote script download-and-execute pipelines such as `irm ... | iex` as a generic install pattern.
- Separate application updates, Windows Update, Microsoft Store updates, firmware and driver updates.
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
5. WinGet, Chocolatey, Scoop, Windows Update, Store updates and Linux package managers remain distinct channels.
6. Bulk updates have preview-first instructions and explicit safety boundaries.
7. The app, generated site, Wiki and offline artifact use the same canonical catalog.
8. Tests validate IDs, shells, categories, risk levels, source URLs, workflow references and toolkit references.
