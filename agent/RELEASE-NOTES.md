# INTA Local Agent release notes

The first Local Agent release provides a loopback-only, typed package-maintenance API for INTA.

- Local pairing from the official INTA GitHub Pages origin.
- Windows installed-application inventory from uninstall registry entries.
- Package-manager detection and update previews.
- One-package update, repair/reinstall and uninstall operations.
- One-confirmation update-all per selected package manager.
- Local operation history and exit-code/output reporting.
- Self-contained Windows, Linux and macOS binaries.
- No arbitrary shell endpoint and no background inventory polling.

Limitations:
- Windows Update and Microsoft Store remain in their supported Windows UI / repair workflows; they are not silently serviced by the agent.
- Windows currently has the richest installed-app inventory. Linux/macOS currently expose installed package-manager previews but not a complete unified inventory.
- The first release is not code-signed. Verify the published SHA-256 checksum and obtain the installer only from the official INTA repository release.
- Remote phone-to-desktop management and cloud account synchronization are not part of this local-only agent release.
