# INTA Desktop for Windows x64

The installer opens the technician directory. The original search, tool actions, playbooks, Windows settings, recovery workflows and case history use the same local state as earlier Workstation installations. Open **Diagnostics** for system inventory and structured operations, **Managed toolbox → CLI Forge** for tested recipes, and **Local AI** to review an offline model’s inspection plan. **Appearance Studio** controls the optical field.

Select **Offline references** and type a symptom into Search to read matching passages from the installed guides, Wiki and catalog. Expand a guide result to see the surrounding steps, or open a catalog result to reach its tool actions. This search runs locally with or without a model.

## Install and verify

Download the MSI or portable ZIP and SHA256SUMS from the same [Desktop release](https://github.com/Herbertofury/INTA/releases/tag/desktop-0.2-preview.2). Verify the file digest before opening it. Run the MSI under the Windows account that will use Desktop. The portable archive needs to be extracted in full.

```powershell
Get-FileHash -Algorithm SHA256 .\INTA-Desktop-x64.msi
```

This preview uses a self-signed INTA Desktop Preview certificate. Windows may display an unknown-publisher or SmartScreen prompt. The package includes its public certificate to verify the manifest and every file in the application payload. The installer does not add a root certificate to Windows.

Installed case state stays in the current user’s local INTA state directory. The portable launcher uses a state directory beside itself. Desktop runs as the current user. Optional vendor tools, models, virtual-machine runtimes, Python for Media Creator, and user-supplied recovery media have their own prerequisites. Private Windows recovery images are built by their licensee and are excluded from these downloads.

## Command line

The CLI is beside the installed launcher in `%LOCALAPPDATA%\Programs\INTA Desktop`. These commands also work from an extracted portable directory.

```powershell
.\inta.exe catalog 'printer'
$hits = (.\inta.exe knowledge search 'Why is the printer queue stuck?' | ConvertFrom-Json).citations
$guide = $hits | Where-Object kind -in 'runbook','wiki' | Select-Object -First 1
.\inta.exe knowledge context $guide.source $guide.location
.\inta.exe capabilities
.\inta.exe tools inspect powercfg
.\inta.exe capabilities run system.memory
.\inta.exe forge verify .\base\examples\verify-workstation-basics.json
.\inta.exe scenario checkpoints
.\inta.exe assistant models
```

An eligible local Ollama model must already be serving with cloud access disabled for the following commands. The plan contains the exact scenario, stages, stop conditions and verification criteria. Review it before running its checks.

```powershell
$m = (.\inta.exe assistant models | ConvertFrom-Json).models | Select-Object -First 1
$p = .\inta.exe ai ask $m.name $m.digest 'Why is this PC slow?' | ConvertFrom-Json
$p.plan
.\inta.exe ai run-plan $p.plan.id $p.planSha256 --approve
.\inta.exe ai forge $m.name $m.digest 'Check memory and power' "$env:TEMP\new-workstation-baseline.json" --approve
```

Tool acquisition, opening an application, and controlling an operation have different coverage. Capability discovery lists the typed operations available in the binary. Forge tests local declarative recipes against those operations before import. Read-only inspection results can be exported from the native workspace.

## Build and development sync

On Windows x64 install Rust with the MSVC toolchain, the Microsoft .NET 10 SDK, Python 3.11 or newer, and a 64-bit Python 3.8–3.12 with msilib for MSI creation. The Windows .NET Framework compiler builds the preserved WPF workspace and manifest verifier. Place the .NET 10 SDK first on PATH.

```powershell
.\native\release\release.ps1 -OutputDirectory C:\Builds\inta-output -Thumbprint $signer.Thumbprint -MsiPythonVersion 3.12
```

The signer is an unexpired code-signing certificate from the current user’s Personal store. Keep its private key on the build host. Run `verify-workspace.ps1` against the packaged `base\inta-desktop.exe` to exercise the real directory and controls. The archived dependencies and license texts accompany the payload in THIRD-PARTY-NOTICES.zip.

Developers with the matching signing key can opt into `enable-development.ps1 -SourceDirectory <protected-checkout>` after installation. The worker rebuilds on local source changes, tests and signs the result, verifies the copied files, and atomically activates the passing revision for the next launch. Standard installations use their packaged release.
