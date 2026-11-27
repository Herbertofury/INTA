using Microsoft.Win32;
using System.Runtime.InteropServices;
namespace INTA.LocalAgent;
public sealed class Inventory(PackageManagers managers)
{
    public async Task<InventoryResult> CollectAsync(CancellationToken ct = default)
    {
        var detected = managers.Detect();
        var apps = OperatingSystem.IsWindows() ? WindowsApps() : Array.Empty<InstalledApp>();
        var previews = await managers.PreviewAsync(detected, ct);
        return new(DateTimeOffset.UtcNow, RuntimeInformation.OSDescription, apps, detected, previews);
    }
    private static IReadOnlyList<InstalledApp> WindowsApps()
    {
        var rows = new List<InstalledApp>();
        var roots = new (RegistryHive Hive, RegistryView View, string Scope)[]
        {
            (RegistryHive.LocalMachine, RegistryView.Registry64, "Machine"),
            (RegistryHive.LocalMachine, RegistryView.Registry32, "Machine"),
            (RegistryHive.CurrentUser, RegistryView.Default, "User")
        };
        foreach (var root in roots)
        {
            try
            {
                using var baseKey = RegistryKey.OpenBaseKey(root.Hive, root.View);
                using var key = baseKey.OpenSubKey(@"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall");
                if (key is null) continue;
                foreach (var name in key.GetSubKeyNames())
                {
                    try
                    {
                        using var app = key.OpenSubKey(name);
                        var displayName = app?.GetValue("DisplayName") as string;
                        if (string.IsNullOrWhiteSpace(displayName)) continue;
                        rows.Add(new(displayName, app?.GetValue("DisplayVersion") as string, app?.GetValue("Publisher") as string, "registry", name, root.Scope));
                    }
                    catch { }
                }
            }
            catch { }
        }
        return rows.GroupBy(x => (x.Name, x.Version, x.Publisher, x.Scope)).Select(x => x.First()).OrderBy(x => x.Name, StringComparer.OrdinalIgnoreCase).ToArray();
    }
}
