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
