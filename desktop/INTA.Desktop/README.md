# INTA Desktop Startup preview

This Windows-native WinForms view builds against the same source files as
the read-only Local Agent inventory. It does not launch the agent or use a
web server, account, service, background scan loop, or third-party runtime.
The inventory shows schema-2 registration names and provenance. It does not
infer enabled state. The Windows Settings button opens the system Startup
Apps page only on explicit interaction.

Build and test on a disposable Windows test host using the .NET 10 SDK:
`dotnet run -c Release --project desktop/INTA.Desktop/INTA.Desktop.csproj`.
Look for both Startup folder scopes and the registry views available on that
host. Use the search box and then clear it; it must not alter Windows state.
Verify keyboard access, high contrast, a 760×500 viewport, and 150% DPI.
Click the Settings button and verify Windows opens Apps → Startup. Close
without making changes. Do not export/screenshot live personal entries or
attach registry names to CI logs. Packaging and visible native acceptance
remain separate from cross-target compilation.

[Windows Forms .NET 10 desktop SDK](https://learn.microsoft.com/en-us/dotnet/core/project-sdk/msbuild-props-desktop)
and [Microsoft's Startup Apps Settings URL](https://learn.microsoft.com/en-us/windows/apps/develop/launch/launch-settings).

Open **Power plans** from the Startup window to inspect the installed Windows
power schemes and active plan. The preview reads Windows power APIs directly
on demand; it does not activate, edit, import, or remove a plan. Incomplete
and unstable snapshots do not label a plan active. Close the Power plans
window to return to Startup.

Open **Storage preview** to inspect a folder you explicitly choose. The view
reports file and byte counts, age-threshold totals, and partial or skipped
coverage. It never opens file contents or deletes anything. Drive roots,
network locations and link-backed roots are not scanned. Reported bytes are
logical size, not a guarantee of disk space you can reclaim. The Windows UI
test uses disposable files only.
Select **Include observed disk allocation** for an additional Windows
metadata pass during the scan. It counts allocation once per identified file,
reports repeats and files with links not seen in the selected scope, and
labels incomplete measurements. It does not claim a point-in-time global
snapshot or estimate reclaimable bytes. Use **Cancel scan** to stop between
entries. The default remains the logical-size-only preview.


Open **Installed programs** to browse desktop program registrations by name,
version, publisher and registry scope. Identical names in separate registrations
stay separate. Searches do not change the computer or invoke a package manager.
Hidden and unnamed records are omitted; unreadable sources and scan caps are
reported. This is not the Windows Store app list and does not uninstall programs.
See Microsoft's [Windows Installer registry properties](https://learn.microsoft.com/windows/win32/msi/uninstall-registry-key).

## Verify a downloaded preview

Open a successful **Verify INTA Desktop preview** workflow run and copy the
**Executable SHA-256** from its summary. Download `inta-desktop-startup-preview-win-x64`
from that run and extract it to an empty directory. With Python 3.11 or later,
run the following command from this checkout, substituting that directory and digest.

```text
python -B desktop/INTA.Desktop/verify-preview.py PATH_TO_EXTRACTED_PREVIEW --sha256 EXECUTABLE_SHA256
```

The verifier checks the complete two-file package, executable bytes and Windows x64
GUI header. After a successful check, open `INTA.Desktop.exe` from that directory.
The workflow exercises the downloaded executable on a fresh Windows runner and
publishes the digest after its native UI checks pass. This is an unsigned preview.

## Desktop installer candidate

The same workflow produces `inta-desktop-msix-unsigned-win-x64` from the verified
executable. It builds the installer twice and compares SHA-256, signs a disposable
copy on a hosted runner, installs it, checks registration and runs the four native
views before uninstalling. The downloadable candidate contains the unsigned MSIX
and its checksum. Windows requires a trusted publisher signature for deployment.

To rebuild on Windows, use Python 3.11 or later, the Windows SDK 10.0.26100.0
x64 packaging tools, the extracted preview, and its independently recorded
executable digest. Provide an unused output path and a four-part package version.

```text
python -B desktop/INTA.Desktop/msix/package.py build EXTRACTED_PREVIEW OUTPUT.msix --version 0.1.7.0 --exe-sha256 EXECUTABLE_SHA256
```

An authorized publisher can sign the verified candidate outside CI. The certificate
subject must match the manifest identity and the signing operation must include a
trusted timestamp. Before distribution, independently obtain the organization's
approved signing certificate fingerprint, then check the exact signed installer.

```text
python -B desktop/INTA.Desktop/msix/package.py verify SIGNED.msix --version 0.1.7.0 --exe-sha256 EXECUTABLE_SHA256 --publisher-sha256 CERTIFICATE_SHA256
```

This gate verifies the Windows trust chain, timestamp, certificate fingerprint,
package manifest, block hashes and executable. The project has not established
a trusted public signing identity. The generated test certificate stays on the
disposable runner. [Windows SDK packaging](https://learn.microsoft.com/en-us/windows/msix/package/create-app-package-with-makeappx-tool) · [Microsoft package signing](https://learn.microsoft.com/en-us/windows/msix/package/sign-app-package-using-signtool).
