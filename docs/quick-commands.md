# INTA quick commands: diagnose, repair, verify

**Purpose:** make every technician playbook immediately actionable with copy-ready commands.  
**Reviewed:** 2026-11-26.

## The command contract

Every job in `data/jobs.json` must include at least one command block in a relevant step. A command block must identify:

- **Shell and privilege:** CMD, PowerShell, elevated shell, or INTA Rescue/Linux shell.
- **Purpose:** the exact question the command answers or action it performs.
- **Safety:** whether it is read-only, changes state, needs a maintenance window, or risks data/service availability.
- **Official reference:** a primary vendor/project documentation URL.
- **Copy action:** copy to clipboard only. INTA never executes commands automatically.

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

For Windows package-manager installs, identify the exact package and source first:

```powershell
winget search --name "Exact application name"
winget show --id Publisher.Package --exact
```

After checking publisher, package identity, architecture, license and change approval, install only the exact ID:

```powershell
winget install --id Publisher.Package --exact --source winget --interactive
```

Replace the example ID with a verified package ID from the official publisher and inspect prompts. Do not run bulk upgrades or install unknown package IDs as a generic repair.

### Inspect before changing Windows services

```powershell
Get-Service -Name ServiceName | Format-List Name,Status,StartType
```

Only change a service after confirming its role, dependencies, management policy and approved maintenance window. Do not disable security, backup, update, identity or remote-management services as a performance shortcut.

### Discover update availability without installing

```powershell
winget upgrade
```

This lists available package-manager updates. Review each package and release notes before a targeted upgrade. It does not replace Windows Update, firmware management or vendor-specific update processes.

### Collect a minimal Windows baseline

```powershell
Get-ComputerInfo | Select-Object WindowsProductName,WindowsVersion,OsBuildNumber,OsArchitecture,CsSystemType
Get-Volume | Select-Object DriveLetter,FileSystem,HealthStatus,SizeRemaining,Size
```

Review and sanitize output before attaching it to a service ticket or sharing it with an external service.

### Linux rescue inventory

```bash
lsblk -o NAME,SIZE,MODEL,SERIAL,TYPE,MOUNTPOINTS
```

Confirm source and destination by physical identity, not device-letter guesswork. Never run filesystem repair or write an image to an unverified target.

## Repair-command standards

- Prefer Microsoft-supported repair sequences such as DISM component-store repair followed by SFC verification when the evidence points to Windows component/system-file corruption.
- Do not issue generic `netsh winsock reset`, DNS flush, registry edits, bootrec, BCDEdit writes, diskpart clean, format, chkdsk repair, driver removal or service resets as first-line commands. Explain the symptom, target, prerequisites, rollback and expected effect first.
- For a failing disk, commands must favor read-only inventory and image-first recovery. Do not run write repairs or repeated stress tests on the failing source.
- For ransomware, preserve evidence and follow incident leadership. Do not automate network isolation, shutdown, account resets or tenant-wide changes from a generic copy block.
- For domain trust, test first; use repair only after verifying network path, domain controller, time and authorized credentials.
- For printers, inspect queue, port and spooler before restarting the service or deleting jobs; account for shared queues.
- For boot repair, inspect recovery and boot configuration before making changes. Require verified target installation, firmware mode, encryption recovery material and a rollback path.
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
