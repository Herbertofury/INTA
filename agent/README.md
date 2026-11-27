# INTA Local Maintenance Agent

The Local Maintenance Agent is the small, user-installed bridge between the INTA website/PWA and package managers on the same computer. It exposes typed operations only; it is not a remote shell.

## Install on Windows

Download the successful Windows build artifact from [INTA Local Agent build 45947359692](https://github.com/Herbertofury/INTA/actions/runs/45947359692) (or check [the workflow page](https://github.com/Herbertofury/INTA/actions/workflows/local-agent.yml) for newer builds). Extract the Actions artifact ZIP, then run the included installer script with the Windows package ZIP and its `.sha256` sidecar in the same folder:

    powershell -NoProfile -ExecutionPolicy Bypass -File .\\install-agent.ps1 -PackagePath .\\inta-agent-win-x64.zip

The installer verifies SHA-256, installs to the current user's LocalAppData, creates normal and administrator Start menu shortcuts, and starts the agent. The first build is not code-signed; Windows may show a SmartScreen warning. Only use artifacts from the official INTA workflow and verify the checksum.

## User experience

1. Install and start the agent once on the computer.
2. Open INTA → Updates & Maintenance → Connect this computer.
3. Click Connect once. The browser receives an in-memory pairing token from the loopback-only agent and stores it in session storage for this tab.
4. Inventory and previews are read-only and do not ask for confirmation.
5. Updating, repair and uninstall require one clear confirmation in the INTA UI. Update-all has one confirmation for the selected manager.
6. Refresh progress when needed. The agent does not poll in the background.
7. Operation history is saved locally and remains available after restarting the agent.

## Security and scope

- Binds only to IPv4/IPv6 loopback at port 47821.
- Requires the official INTA GitHub Pages Origin (or local development origins), validates loopback peer and Host, and answers browser private-network preflights.
- Uses one in-memory bearer token per agent session. The token is not written to disk.
- Exposes typed inventory, preview, update, repair, uninstall and history endpoints. There is no arbitrary command or shell endpoint.
- Uses ProcessStartInfo.ArgumentList; commands are not passed through a shell.
- Validates package IDs; only fixed manager/action combinations are accepted.
- Does not kill an installer if the observation window elapses; it continues waiting and keeps the package-manager lock.
- Runs with the user's current privilege level. If an operation needs elevation, start the agent from the administrator shortcut; there are no repeated agent authorization prompts.
- Stores operation history under the user's local INTA application data directory.

## Supported manager actions

| Manager | Preview | Update one | Update all | Repair/reinstall | Uninstall | Rollback |
|---|---:|---:|---:|---:|---:|---:|
| Windows Update | Installed KB history | Open Settings | Open Settings | DISM + SFC | Exact KB via WUSA | Exact KB via WUSA |
| WinGet | Yes | Yes | Yes | If package supports it | Yes | No generic rollback |
| Chocolatey | Yes | Yes | Yes | Reinstall | Yes | Requires an explicit previous package version |
| Scoop | Yes | Yes | Yes | Reinstall | Yes | Requires an explicit previous version |
| APT | Yes | Yes | Yes | Reinstall | Yes | Requires a selected package version |
| DNF | Yes | Yes | Yes | Reinstall | Yes | Requires a selected package version |
| Pacman | Yes | Yes | Yes | Reinstall | Yes | Requires a cached package archive |
| Homebrew | Yes | Yes | Yes | Reinstall | Yes | Requires a pinned/formula-specific prior version |
| Flatpak | Yes | Yes | Yes | Repository repair | Yes | Commit-specific rollback is not automated |
| Snap | Yes | Yes | Yes | Not generic | Yes | Native previous-revision revert |
| UniGetUI | Yes | Yes | Yes | Not yet | Not yet | Not yet |

The agent currently collects Windows installed applications from the uninstall registry. Linux/macOS currently provide manager detection and update previews; richer unified installed-app inventory is not included in the first release. Package IDs are copied from the selected manager's preview. Native rollback is currently exposed for Snap via `snap revert` and for Windows Update through an exact KB uninstall. Other package managers require a separately identified previous version or vendor-supported rollback procedure. Windows Update inventory is installed-hotfix history, not a complete list of offered OS updates; installation opens the supported Settings interface. Windows servicing repair runs DISM then SFC.

## Build locally

Requires .NET 10 SDK.

    dotnet publish agent/INTA.LocalAgent/INTA.LocalAgent.csproj -c Release -r win-x64 --self-contained true -p:PublishSingleFile=true -o dist/inta-agent

Other runtime identifiers: linux-x64, osx-arm64, osx-x64.

## Boundaries

Windows OS update installation and Microsoft Store updates remain in their supported Windows UI. The agent can inspect installed KB history, run DISM/SFC repair, and uninstall a specific uninstallable KB through WUSA; it does not silently manipulate Windows servicing internals. The website and PWA can only connect to the agent on the same device. Phone-to-desktop management through a remote account relay is not available yet because this repository currently has a static GitHub Pages frontend and no authenticated relay service.

## Storage Preview

Run `inta-agent --storage-preview "<local-folder>" [minimum-age-days] [entry-limit]` as an explicit one-shot diagnostic. Defaults: 7 days and 100,000 entries. The preview rejects drive roots, Windows UNC/device paths, and roots passing through a link. The scan skips link, device and offline entries marked by the filesystem and reports entry/depth caps or read errors as `partial`. It never starts the local server.

The JSON reports counts and **logical file bytes**. Hard links can be counted multiple times; compression, sparse files and allocation overhead mean this is not the space a cleanup would free. The preview is not a filesystem snapshot, and file age is not authorization to remove anything. It does not open file contents, modify the folder, include paths in the JSON or send results elsewhere.

### Read the current Windows power plan

Run `inta-agent --power-plans` locally to print the active plan GUID and installed
power-plan IDs and names as JSON. The command is read-only and on demand. It
does not start the local server, expose a browser API, change the active plan,
or promise that an installed plan can be activated under current policy.
Non-Windows hosts return `unsupported`; incomplete or changing reads are marked
rather than guessed.
## Windows logon registrations (preview)

Run `inta-agent --startup-inventory` for a one-shot local JSON inventory of the
documented current-user and machine `Run` and `RunOnce` keys. On 64-bit Windows,
inspect the independent machine views separately. Current-user names are counted
once because this part of HKCU is shared. The command does not start the local
web agent or install anything.

Each source reports **read**, **not present**, or **unavailable** and whether
the output names were truncated. A registration does not prove the app is
enabled, safe, or runnable. Startup folders, scheduled tasks, services, disabled
states, packaged startup actions, repair and edits are not included. Only names
are collected: targets, command lines, content and arguments are never read.
Treat names as local user data and do not share output unless intended.

Sources: [Microsoft Run and RunOnce](https://learn.microsoft.com/windows/win32/setupapi/run-and-runonce-registry-keys)
and [registry view scope](https://learn.microsoft.com/windows/win32/winprog64/shared-registry-keys).

Add `--include-folders` to include the two Windows-resolved Startup folders in
the same local output. The original command and schema 1 stay unchanged. The
optional schema 2 returns each folder's scope, status and immediate names, never
the absolute folder path, shortcut target, content, or command line. Matching
names in two scopes remain separate. An empty folder is an observed source,
not a missing source. Network drives, redirected network locations and
junctions are not traversed. A very large folder returns a bounded sample with
`truncated` set. Treat displayed names as private to the machine.

Schema 2 includes the Windows Startup Apps Settings link for explicit use by a
later Desktop interface or by the technician. The command does not open
Settings or change enabled state. See Microsoft's [special-folder resolution](https://learn.microsoft.com/en-us/dotnet/api/system.environment.getfolderpath?view=net-10.0)
and [Startup Apps Settings URL](https://learn.microsoft.com/en-us/windows/apps/develop/launch/launch-settings).
