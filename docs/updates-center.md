# INTA Updates & Maintenance Center

**Purpose:** one technician workspace for application patching, Windows servicing, package-manager maintenance, repair, uninstall and recovery after interrupted or corrupt updates.

## Important status boundary

The public website and installed responsive web app contain a searchable, reviewed command catalog. They do not have access to a customer's local package database or installer process. They must never display "up to date", "failed" or a live progress bar without a connected local INTA agent or an explicitly authorized package-manager integration.

The canonical source is `data/updates-center.json`. The UI is `#updates` and is designed for desktop and narrow mobile screens. All commands are copy-only in this build.

## Keep update channels separate

| Channel | Owns | Preview/audit |
|---|---|---|
| Windows Update | Windows cumulative, servicing and feature updates | Windows Update history, event logs, Get Help troubleshooter |
| WinGet | Packages recognized by WinGet | `winget upgrade` |
| Chocolatey | Chocolatey-managed packages | `choco outdated`, `choco upgrade all --noop` |
| Scoop | Scoop-managed apps | `scoop status` |
| UniGetUI | Supported configured package managers | `uniget.exe package updates --manager <manager>` |
| Ninite Pro Classic | Ninite-supported apps | `NinitePro.exe /audit /silent audit.txt` |
| Microsoft Store | Store-delivered apps | Store Downloads & updates UI |
| Linux package managers | Distribution-managed packages | APT, DNF or Arch package query |

A single app may appear in multiple inventory sources. The future agent must establish package identity, manager, source, version and install scope before proposing an update. It must not double-update an application or assume one manager covers all software.

## Recommended technician session

1. Identify OS, architecture, device owner, management policy and maintenance window.
2. Capture a package inventory and pending-reboot indicators.
3. Query every manager actually installed and enabled. Record manager health and last successful scan.
4. Normalize packages by publisher, product identity, version, source, scope and installer family. Preserve manager ownership.
5. Detect duplicate candidates, pinned/ignored packages, unknown versions, unsupported packages and vendor-specific updates.
6. Preview update plans and show the exact packages, current/target versions, source, estimated restart requirements and exclusions.
7. Ask the technician to approve the selected plan. Never infer consent from opening the page.
8. Apply in controlled batches, not one huge blind update. Keep per-package logs, exit codes and timestamps.
9. Verify installed version, app launch/smoke test, package-manager state and pending reboot.
10. Produce a handoff report with succeeded, failed, skipped, held, unknown and reboot-required packages.

## Recovery decision tree

### Update still running

- Check whether Windows Installer, Windows servicing or the package manager is actively working.
- Do not kill `msiexec.exe`, TrustedInstaller, TiWorker or package-manager processes as a generic fix.
- Do not start a second installer or initiate an uninstall while a transaction is active.
- Check whether a restart is pending and whether the installer is waiting for user input.

### Update failed or stopped halfway

- Preserve the exact error, exit code, package ID, version, source and log before retrying.
- Determine whether the old version remains installed, the new version is partially installed, or an uninstall occurred.
- Use the owning package manager's repair or reinstall action when supported.
- Back up app-specific configuration/data before reinstall or uninstall.
- Verify the installed version and the original application workflow after recovery.

### Windows Update corruption

1. Capture Windows Update Client events and error codes.
2. Check free space, services, network and pending restart.
3. Run the supported Windows Update troubleshooter / Get Help flow.
4. If evidence indicates servicing corruption, run DISM component-store repair, then SFC.
5. Only if the update cache remains implicated, use the reversible cache-reset command. It stops services and renames cache folders to timestamped backups; it does not delete them.
6. Retry Windows Update and verify history and event logs.

Microsoft's current support flow documents the troubleshooter, cache issues, common HRESULTs, DISM and SFC. [Troubleshoot problems updating Windows](https://support.microsoft.com/en-us/windows/deployment/updates-lifecycle/troubleshoot-problems-updating-windows).

### WinGet application repair

WinGet has a native `repair` command, but only when the package/installer supports repair. Identify the exact package first, then use `winget repair --id Publisher.Package --exact --interactive`. Never use `--ignore-security-hash` to bypass integrity validation. [WinGet repair](https://learn.microsoft.com/en-us/windows/package-manager/winget/repair).

### UniGetUI package repair

UniGetUI's documented CLI supports package repair, reinstall, update and uninstall. Its package repair is an uninstall-then-reinstall flow, not an in-place repair. Preserve app data and confirm exact manager/package identity first. Its current CLI also includes Manager Health actions for repairing WinGet and resetting sources. [UniGetUI CLI](https://github.com/Devolutions/UniGetUI/blob/main/docs/CLI.md).

### Chocolatey and Scoop

Chocolatey provides package-specific install/upgrade/uninstall commands; package scripts are executable code and may make machine-wide changes. Preview bulk changes with `choco outdated` and `choco upgrade all --noop`. Scoop's `scoop status`, `scoop update`, `scoop reinstall` and `scoop uninstall` apply only to Scoop-managed applications. Do not mix package managers or bypass pins by default. [Chocolatey upgrade](https://docs.chocolatey.org/en-us/choco/commands/upgrade/) · [Scoop](https://github.com/ScoopInstaller/Scoop).

### Ninite Pro

Ninite Pro Classic documents audit, update-only and repair switches. The CLI is Pro Classic-specific; the newer web product has different documentation. Use only with the correct commercial license and current account instructions. [Audit](https://ninite.com/help/features/audit.html) · [Update-only](https://ninite.com/help/features/update.html) · [Switches](https://ninite.com/help/features/switches.html).

## Uninstall and reinstall safety

- Confirm publisher, exact package ID, source, install scope and user-data retention.
- Prefer the package manager that owns the install.
- If manager ownership is unknown, inspect the Windows uninstall registry entries and vendor documentation; do not execute arbitrary UninstallString values without review.
- Preserve user profiles, browser data, license state, application databases, plugins and custom settings.
- Do not delete Program Files directories, MSI caches, registry uninstall keys or shared dependencies to force removal.
- Use a vendor's official cleanup utility only when its scope and supported versions are documented.
- After reinstall, verify app launch, version, sign-in, plugins and user data.

## Future local agent architecture

The website, desktop wrapper and phone-sized view should share the same canonical catalog and normalized package/operation model. Live machine access must be supplied by a separately installed, signed local agent that:

- inventories package managers only on request or an explicitly configured schedule;
- runs with least privilege and requests elevation only for a specific approved action;
- never exposes an arbitrary shell endpoint to the public website;
- accepts typed, allowlisted operations (inventory, preview, update one, update approved set, repair, uninstall);
- validates package IDs, source, version, scope and policy on the local machine;
- shows a dry-run plan before state changes;
- records stdout/stderr, exit codes, installer logs and timestamps;
- tracks operation states: queued, running, awaiting-reboot, succeeded, failed, cancelled, rollback-needed, verification-failed;
- prevents concurrent operations against the same package/installer lock;
- detects stale/partial operations after restart and resumes only when safe;
- supports cancellation only when the underlying manager safely supports it;
- creates a rollback or recovery plan when the installer exposes one, without promising universal rollback;
- verifies post-install versions and application health;
- stores customer data locally or in the approved account workspace with access controls and retention.

A public website cannot reliably enumerate or control arbitrary local apps without this agent. Do not fake live state with static catalog data.

## Acceptance tests

- One dedicated Updates & Maintenance tab is visible in the primary app navigation and technician home.
- Search and category, shell and impact filters work together.
- Channel cards distinguish OS, Store, WinGet, Chocolatey, Scoop, UniGetUI, Ninite Pro and Linux.
- Commands copy with exact multiline text and include shell, privilege, risk, safety and official reference.
- Workflows copy ordered commands and safety notes.
- Update audit always precedes bulk update in recommended workflows.
- A failed or stuck install leads to logs and package-specific repair before uninstall.
- No generic forced process termination, registry cleaning or blind cache deletion.
- Package identity and manager ownership prevent double updates.
- No "all up to date" or live progress claims without a connected local agent.
- Narrow screens remain usable; cards stack and code scrolls horizontally.
- Generated site, Wiki and offline guide use the canonical catalog.

## More package managers and managed-fleet platforms

The Updates tab also includes Flatpak, Snap and Homebrew. Their inventories and transactions are independent:
- Flatpak: `flatpak remote-ls --updates`, `flatpak update`, `flatpak repair`, `flatpak uninstall <app-id>`. [Flatpak docs](https://docs.flatpak.org/en/latest/using-flatpak.html).
- Snap: `snap refresh --list`, `snap refresh --time`, `sudo snap refresh`. Snap normally refreshes automatically; holds and schedules matter. [Snap update management](https://snapcraft.io/docs/how-to-guides/manage-snaps/manage-updates/).
- Homebrew: `brew outdated --verbose`, then review before `brew upgrade`. Homebrew is not a complete Mac App Store or vendor updater. [Homebrew manual](https://docs.brew.sh/Manpage.html).

For managed customer fleets, the page now links to:
- **Action1** — cloud patching, missing updates, approvals, update rings, deployment results and endpoint inventory. [Managed endpoint updates](https://www.action1.com/documentation/managed-endpoints/) · [Roll out updates](https://www.action1.com/documentation/roll-out-and-deploy-updates/).
- **Patch My PC Cloud** — Intune-integrated app deployment and update rings; **Publisher** — ConfigMgr/Intune/WSUS third-party update publishing. These are distinct products with different capabilities. [Cloud vs Publisher](https://patchmypc.com/faq/should-i-use-patch-my-pc-cloud-or-publisher/).
- **PDQ Deploy & Inventory** — software inventory, package library, Windows and third-party update deployment and result reporting. [PDQ overview](https://www.pdq.com/pdq-overview/).

These are commercial fleet-management platforms, not generic local shell commands. Live data requires an authorized account, agent and managed endpoint enrollment.

## Mobile installation

The site is generated as an installable Progressive Web App (PWA): mobile browsers can add INTA to the home screen, launch it in a standalone window and cache the app shell after the first successful visit. The same responsive Updates tab and canonical catalog are used by desktop and phone layouts. The service worker performs no periodic background polling or automatic update execution; it only maintains the cached app shell and uses network-first navigation with an offline fallback.

The PWA does not grant the website access to installed applications. Live update inventory and repair still require the local INTA agent or an approved management integration.

The Windows Update cache recovery workflow also includes a matching rollback command. It requires the exact timestamp and both backup folders created by the INTA reset command; it preserves any newly generated cache folders under rollback names and restores the original folders. This is specific to the INTA reversible cache-reset procedure, not a general-purpose Windows Update fix.

### Windows drivers and firmware

The Updates tab has a separate Windows driver inventory and optional-driver entry point. Compare the current signed driver and hardware model against the OEM's supported release. An optional Windows Update driver is not automatically newer or better. BIOS/UEFI and device firmware must use the exact manufacturer's supported process; never apply generic firmware-flash commands.

### macOS Software Update

Apple's `softwareupdate --list` and `sudo softwareupdate --install --all` manage macOS system updates. They do not cover every Homebrew formula, Mac App Store app or vendor updater. Review MDM policy, backup and restart requirements. [Apple macOS updates](https://support.apple.com/guide/mac-help/get-macos-updates-mchlpx1065/mac).
