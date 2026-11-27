# INTA Local Maintenance Agent

The Rust Local Maintenance Agent connects the [INTA workspace](https://herbertofury.github.io/INTA/#updates) to package managers on the **same computer**. Its source is [crates/inta-agent](../crates/inta-agent). It preserves the versioned API and existing operation history while replacing the previous C#/.NET implementation.

## Run or install

Download the matching archive and its `.sha256` sidecar from the [Local Agent builds](https://github.com/Herbertofury/INTA/actions/workflows/local-agent.yml). Verify the archive against the published SHA-256 before extracting it. Run `inta-agent.exe` on Windows or `inta-agent` on Linux/macOS. The executable is portable and requires no .NET or Python runtime.

For a current-user install, run the verified executable with `--install`. On Windows, this copies it into `%LOCALAPPDATA%\Programs\INTA\Agent` and creates normal and administrator Start menu shortcuts using native Windows APIs. On Linux/macOS it copies the executable to `~/.local/bin`; launch it explicitly. Installation does not launch the agent or register a background service. The release is unsigned unless its release notes explicitly state otherwise.

Open **INTA → Updates & Maintenance → Connect this computer**. Connect once, then request inventory or progress when needed. Read-only inventory and previews need no extra confirmation. Each update, repair, uninstall or rollback needs one clear confirmation. Closing the browser does not cancel an installer. The agent keeps observing the original process and retains the manager lock until it exits.

### Portable workspace

Use `inta-agent --data-dir /path/to/INTA/agent` to keep history in an explicit portable directory. Without this option, Windows continues using `%LOCALAPPDATA%\INTA\agent\operations.jsonl`; Linux/macOS use the local XDG data directory. Existing camelCase JSONL history is readable. Queued/running records found after restart become **interrupted**, retaining their output and requiring inspection before retrying.

Use `inta-agent --serve-site /path/to/INTA/site --open` to open a bundled Site build in the local browser. This host serves only that directory on loopback port 5173 and works with the same maintenance bridge. It is a local browser workspace, not evidence of a native Mobile application or authenticated remote device management.

## Security and behavior

- Binds exclusively to IPv4/IPv6 loopback on port 47821. The process rejects non-loopback peers and unexpected Host headers.
- Only the official INTA origins and exact local development origins may pair. Other origins receive no CORS grant. Browser private-network preflights are supported.
- Keeps a cryptographically random bearer token in memory for the current agent session. Restart invalidates previous tokens; credentials never enter history or release files.
- Accepts only typed manager/action/package requests. Exact executable/argument vectors are selected in Rust. There is no arbitrary shell endpoint.
- Validates the complete operation before queueing it. Batch validation is atomic: an unsupported Windows Update request or malformed package ID prevents the whole batch from launching.
- Serializes operations per manager, saves accepted requests before launch, drains both process output streams, and never kills an installer because a browser times out.
- Reports **queued**, **running**, **failed**, **completed-review**, **verified**, **awaiting-reboot**, and **interrupted** separately. Exit code zero alone does not establish a successful repair or rollback.
- Performs no background inventory, update checks or web polling. Refreshes are explicit user actions.

## Supported operations

| Manager | Preview | Update one/all | Repair | Uninstall | Rollback |
| --- | --- | --- | --- | --- | --- |
| Windows Update | Installed hotfix history | Use Windows Settings | DISM, then SFC if DISM succeeds | Exact KB via WUSA | Exact KB via WUSA |
| WinGet | Yes | Yes / yes | Manager-supported repair | Yes | Not generic |
| Chocolatey | Yes | Yes / yes | Reinstall | Yes | Needs a selected prior version |
| Scoop | Yes | Yes / yes | Reinstall | Yes | Needs a selected prior version |
| APT | Yes | Yes / yes | Reinstall | Yes | Needs a selected prior version |
| DNF | Yes | Yes / yes | Reinstall | Yes | Needs a selected prior version |
| Pacman | Yes | Yes / yes | Reinstall | Yes | Needs a selected cached archive |
| Homebrew | Yes | Yes / yes | Reinstall | Yes | Formula/version specific |
| Flatpak | Yes | Yes / yes | Repository repair | Yes | Commit specific; not automated |
| Snap | Yes | Yes / yes | No generic repair | Yes | Native previous-revision revert |
| UniGetUI | Yes | No / yes | Not supported | Not supported | Not supported |

Windows app inventory reads the installed-app registry. Other platforms currently expose manager detection and read-only update previews rather than a claimed unified installed-app inventory. Windows hotfix history is not a complete list of offered OS updates. Store updates remain in the supported Windows interface. APT updates can be marked verified only when a successful complete query proves the exact package installed and a successful update preview no longer lists it; other post-checks remain visible for review where definitive evidence is unavailable.

## Build and proof

```sh
cargo test -p inta-agent
cargo build --release -p inta-agent --bin inta-agent
cargo run --release -p inta-agent --bin inta-agent-package -- publish dist/inta-agent.zip
```

Command legend: [Cargo](https://doc.rust-lang.org/cargo/) compiles and tests the Rust [agent](../crates/inta-agent) and its Rust archive/checksum packager. The platform workflow builds Windows x64, Linux x64, macOS ARM64 and macOS x64 artifacts. A platform build does not establish real package-manager repair success on that platform.

The integration tests exercise live loopback HTTP pairing/CORS/authentication, invalid origins and Host values, typed-action rejection, unsafe-batch atomicity, real binary process capture, per-manager serialization, history restart/migration, and conservative verification. Test execution uses an explicit fixture backend and never updates this machine's packages.

## Product boundary

This agent and its local browser host complement the four product lanes. They do not supply account login, a remote relay, native phone screens, synced billing or device enrollment. [Ecosystem Development](../docs/ecosystem-development.md) tracks those acceptance gates separately.
