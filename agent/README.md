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

| Manager | Preview | Update one | Update all | Repair/reinstall | Uninstall |
|---|---:|---:|---:|---:|---:|
| WinGet | Yes | Yes | Yes | Yes, if supported | Yes |
| Chocolatey | Yes | Yes | Yes | Reinstall | Yes |
| Scoop | Yes | Yes | Yes | Reinstall | Yes |
| APT | Yes | Yes | Yes | Reinstall | Yes |
| DNF | Yes | Yes | Yes | Reinstall | Yes |
| Pacman | Yes | Yes | Yes | Reinstall | Yes |
| Homebrew | Yes | Yes | Yes | Reinstall | Yes |
| Flatpak | Yes | Yes | Yes | Repository repair | Yes |
| Snap | Yes | Yes | Yes | Not generic | Yes |
| UniGetUI | Yes | Yes | Yes | Not yet | Not yet |

The agent currently collects Windows installed applications from the uninstall registry. Linux/macOS currently provide manager detection and update previews; richer unified installed-app inventory is not included in the first release. Package IDs are copied from the selected manager's preview.

## Build locally

Requires .NET 10 SDK.

    dotnet publish agent/INTA.LocalAgent/INTA.LocalAgent.csproj -c Release -r win-x64 --self-contained true -p:PublishSingleFile=true -o dist/inta-agent

Other runtime identifiers: linux-x64, osx-arm64, osx-x64.

## Boundaries

Windows OS servicing and Microsoft Store updates remain in their supported Windows UI and repair workflows; the agent does not silently manipulate servicing internals. The website and PWA can only connect to the agent on the same device. Phone-to-desktop management through a remote account relay is not available yet because this repository currently has a static GitHub Pages frontend and no authenticated relay service.
