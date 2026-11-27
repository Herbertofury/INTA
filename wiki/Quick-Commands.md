<a href="https://herbertofury.github.io/INTA/" title="Open INTA official download or service page" aria-label="Open INTA official download or service page" data-inta-brand="app-inta"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA official page" width="18" height="18" valign="middle" /></a> [INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Service map](https://github.com/Herbertofury/INTA/wiki/Service-Map) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

# <a href="https://herbertofury.github.io/INTA/" title="Open INTA official download or service page" aria-label="Open INTA official download or service page" data-inta-brand="app-inta"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA official page" width="18" height="18" valign="middle" /></a> [INTA](https://herbertofury.github.io/INTA/) quick commands: diagnose, repair, verify

**Purpose:** make every technician playbook immediately actionable with copy-ready commands.<br>
**Reviewed:** 2026-11-26.

## The command contract

Every job in `data/jobs.json` must include at least one command block in a relevant step. A command block must identify:

- **Shell and privilege:** CMD, <a href="https://github.com/PowerShell/PowerShell/releases/latest" title="Open PowerShell official download or service page" aria-label="Open PowerShell official download or service page" data-inta-brand="powershell"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="PowerShell official page" width="18" height="18" valign="middle" /></a> [PowerShell](https://learn.microsoft.com/en-us/powershell/), elevated shell, or <a href="https://github.com/Herbertofury/INTA/releases" title="Open INTA Rescue official download or service page" aria-label="Open INTA Rescue official download or service page" data-inta-brand="app-inta-rescue"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA Rescue official page" width="18" height="18" valign="middle" /></a> [INTA Rescue](https://github.com/Herbertofury/INTA/blob/main/docs/rescue.md)/<a href="https://www.kernel.org/" title="Open Linux official download or service page" aria-label="Open Linux official download or service page" data-inta-brand="app-linux"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/eda717b4f81bf8bcb609.png" alt="Linux official page" width="18" height="18" valign="middle" /></a> [Linux](https://www.kernel.org/) shell.
- **Purpose:** the exact question the command answers or action it performs.
- **Safety:** whether it is read-only, changes state, needs a maintenance window, or risks data/service availability.
- **Official reference:** a primary vendor/project documentation URL.
- **Copy action:** copy to clipboard only. <a href="https://herbertofury.github.io/INTA/" title="Open INTA official download or service page" aria-label="Open INTA official download or service page" data-inta-brand="app-inta"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA official page" width="18" height="18" valign="middle" /></a> [INTA](https://herbertofury.github.io/INTA/) never executes commands automatically.

The playbook must explain when to run it and what a useful result looks like. A command that merely prints output is not a fix; pair diagnostics with a conditional repair path and a verification test whenever an appropriate, supported repair exists.

## Technician workflow

1. Confirm the affected system, scope, authorization and exact symptom.
2. Protect customer data and preserve evidence. If storage is failing, ransomware is active, or a wider incident is underway, follow the stop/escalation route first.
3. Select the exact shell and elevation shown in the command block.
4. Read the purpose and safety note. Confirm the command matches the target OS/build and the technician's approved change window.
5. Copy and inspect the command before running it. Never paste commands from an untrusted source without understanding them.
6. Record the relevant output in the approved private ticket. Remove customer identifiers, hostnames, IPs, serials, usernames, tokens and secrets before external sharing.
7. Apply only the repair that matches the observed fault. Prefer the narrowest reversible change.
8. Run the stated verification command or repeat the original user workflow. Record success, failure and remaining uncertainty.

## Safe command patterns

### Discover before installing

For <a href="https://www.microsoft.com/windows" title="Open Windows official download or service page" aria-label="Open Windows official download or service page" data-inta-brand="app-windows"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="Windows official page" width="18" height="18" valign="middle" /></a> [Windows](https://www.microsoft.com/windows) package-manager installs, identify the exact package and source first:

```powershell
winget search --name "Exact application name"
winget show --id Publisher.Package --exact
```

After checking publisher, package identity, architecture, license and change approval, install only the exact ID:

```powershell
winget install --id Publisher.Package --exact --source winget --interactive
```

Replace the example ID with a verified package ID from the official publisher and inspect prompts. Do not run bulk upgrades or install unknown package IDs as a generic repair.

### Inspect before changing <a href="https://www.microsoft.com/windows" title="Open Windows official download or service page" aria-label="Open Windows official download or service page" data-inta-brand="app-windows"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="Windows official page" width="18" height="18" valign="middle" /></a> [Windows](https://www.microsoft.com/windows) services

```powershell
Get-Service -Name ServiceName | Format-List Name,Status,StartType
```

Only change a service after confirming its role, dependencies, management policy and approved maintenance window. Do not disable security, backup, update, identity or remote-management services as a performance shortcut.

### Discover update availability without installing

```powershell
winget upgrade
```

This lists available package-manager updates. Review each package and release notes before a targeted upgrade. It does not replace <a href="https://learn.microsoft.com/en-us/windows/deployment/update/" title="Open Windows Update official download or service page" aria-label="Open Windows Update official download or service page" data-inta-brand="app-windows-update"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="Windows Update official page" width="18" height="18" valign="middle" /></a> [Windows Update](https://learn.microsoft.com/en-us/windows/deployment/update/), firmware management or vendor-specific update processes.

### Collect a minimal <a href="https://www.microsoft.com/windows" title="Open Windows official download or service page" aria-label="Open Windows official download or service page" data-inta-brand="app-windows"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="Windows official page" width="18" height="18" valign="middle" /></a> [Windows](https://www.microsoft.com/windows) baseline

```powershell
Get-ComputerInfo | Select-Object WindowsProductName,WindowsVersion,OsBuildNumber,OsArchitecture,CsSystemType
Get-Volume | Select-Object DriveLetter,FileSystem,HealthStatus,SizeRemaining,Size
```

Review and sanitize output before attaching it to a service ticket or sharing it with an external service.

### <a href="https://www.kernel.org/" title="Open Linux official download or service page" aria-label="Open Linux official download or service page" data-inta-brand="app-linux"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/eda717b4f81bf8bcb609.png" alt="Linux official page" width="18" height="18" valign="middle" /></a> [Linux](https://www.kernel.org/) rescue inventory

```bash
lsblk -o NAME,SIZE,MODEL,SERIAL,TYPE,MOUNTPOINTS
```

Confirm source and destination by physical identity, not device-letter guesswork. Never run filesystem repair or write an image to an unverified target.

## Repair-command standards

- Prefer <a href="https://www.microsoft.com/" title="Open microsoft.com official download or service page" aria-label="Open microsoft.com official download or service page" data-inta-brand="site-www-microsoft-com"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="microsoft.com official page" width="18" height="18" valign="middle" /></a> [Microsoft](https://www.microsoft.com/)-supported repair sequences such as <a href="https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image" title="Open DISM official download or service page" aria-label="Open DISM official download or service page" data-inta-brand="dism"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="DISM official page" width="18" height="18" valign="middle" /></a> [DISM](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/repair-a-windows-image) [component-store repair](https://github.com/Herbertofury/INTA/wiki/Job-windows-repair) followed by <a href="https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc" title="Open System File Checker official download or service page" aria-label="Open System File Checker official download or service page" data-inta-brand="sfc"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="System File Checker official page" width="18" height="18" valign="middle" /></a> [SFC](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sfc) verification when the evidence points to <a href="https://www.microsoft.com/windows" title="Open Windows official download or service page" aria-label="Open Windows official download or service page" data-inta-brand="app-windows"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="Windows official page" width="18" height="18" valign="middle" /></a> [Windows](https://www.microsoft.com/windows) component/system-file corruption.
- Do not issue generic `netsh winsock reset`, DNS flush, registry edits, bootrec, BCDEdit writes, <a href="https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/diskpart" title="Open DiskPart official download or service page" aria-label="Open DiskPart official download or service page" data-inta-brand="app-diskpart"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="DiskPart official page" width="18" height="18" valign="middle" /></a> [diskpart](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/diskpart) clean, format, <a href="https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/chkdsk" title="Open chkdsk official download or service page" aria-label="Open chkdsk official download or service page" data-inta-brand="app-chkdsk"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/4538691198fe0ea7526e.png" alt="chkdsk official page" width="18" height="18" valign="middle" /></a> [chkdsk](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/chkdsk) repair, driver removal or service resets as first-line commands. Explain the symptom, target, prerequisites, rollback and expected effect first.
- For a failing disk, commands must favor read-only inventory and [image-first recovery](https://github.com/Herbertofury/INTA/wiki/Job-failing-drive). Do not run write repairs or repeated stress tests on the failing source.
- For ransomware, preserve evidence and follow incident leadership. Do not automate network isolation, shutdown, account resets or tenant-wide changes from a generic copy block.
- For domain trust, test first; use repair only after verifying network path, domain controller, time and authorized credentials.
- For printers, inspect queue, port and spooler before restarting the service or deleting jobs; account for shared queues.
- For [boot repair](https://github.com/Herbertofury/INTA/wiki/Job-boot-recovery), inspect recovery and boot configuration before making changes. Require verified target installation, firmware mode, encryption recovery material and a rollback path.
- For remote access, inspect state only. Enabling services, firewall rules, unattended access or accounts requires explicit authorization and approved policy.
- For backup and recovery, inventory is not restore proof. Test an isolated restore and validate the actual application data.
- For endpoint security, a completed scan, coverage, signatures, detections and post-reboot state must be recorded. A clean headline is not proof of no compromise.

## UI and offline requirements

The interactive hub and generated Wiki/runbooks must render the same command context, code, safety notes and official source. Each command has an accessible copy button. Keep code horizontally scrollable on small screens, preserve line breaks, and provide manual selection if clipboard access is unavailable. Printing and exported checklists must retain the command and its caution.

## Acceptance tests

- All 22 current job playbooks contain at least one copy-ready command.
- Every command has shell, purpose, safety note and public HTTPS primary reference.
- Copying preserves exact text, line breaks and braces.
- A command is never executed by opening a guide or clicking Copy.
- Elevated commands are clearly identified.
- Destructive commands are not presented as generic quick fixes.
- Both themes, keyboard navigation, narrow layouts, offline HTML and printed output remain usable.
- Build validation fails if a future playbook has no command or a command lacks required metadata.
- Every repair has a conditional trigger and an observable verification step; diagnostics are not mislabeled as fixes.

**Canonical source:** `data/jobs.json`. The builder generates all rendered playbook surfaces from this source.

---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
