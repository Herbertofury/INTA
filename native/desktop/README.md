# INTA Desktop feature views

The existing Windows workspace opens three native views. Appearance Studio opens from the main window, Capability diagnostics from Diagnostics, and CLI Forge from Managed toolbox.

From the repository root, build the Windows x64 executable with `cargo build --release --locked --manifest-path native/desktop/Cargo.toml`. The package places the result beside the primary workspace as `inta-views.exe`. The executable accepts exactly one of `--view appearance`, `--view diagnostics` or `--view forge`.

Diagnostics and Forge invoke the sibling `inta-capabilities.exe` with argument arrays. Forge can verify a selected recipe, import it to the local store, and run an imported recipe after confirmation. The capability registry defines the supported operations and validates their inputs.

Appearance Studio and the primary workspace share an appearance file. An absolute `INTA_DATA_DIR` selects the state directory. A portable payload uses its adjacent `state` directory by default. A direct installed launch defaults to `%LOCALAPPDATA%\INTA\Desktop`. The workspace uses the sibling `inta_optics.dll` for its decorative field. Windows high contrast suppresses the field and reduced animation suppresses motion.

## Verify the integrated Windows payload

Run the following from the repository root in an interactive Windows desktop session. Supply an assembled payload and a fresh temporary state directory. The test opens the real workspace, uses Windows UI Automation to invoke its controls, and closes the test windows. The optional evidence directory receives screenshots of the application windows.

```powershell
.\native\desktop\verify-integrated.ps1 -Payload $payload -StateRoot $newStateDirectory -ExampleManifest .\native\capabilities\examples\verify-workstation-basics.json -EvidenceDirectory $evidence
```
