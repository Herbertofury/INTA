namespace INTA.Desktop;

internal enum MaintenanceView
{
    DeepCleanup, LargeFileCleanup, DuplicateCleanup, DiskSpaceAnalysis, PrivacyCleanup,
    AppUninstaller, StartupItems, SystemOptimization, SystemMaintenance
}

internal enum NavigationTravel { First, Last, Previous, Next }

internal enum MaintenanceTool { StoragePreview, InstalledPrograms, StartupInventory, PowerPlans, FavoriteTweaks }

internal sealed record MaintenanceDestination(MaintenanceView Id, string Key, string Group, string Name,
    string Scope, string[] Review, MaintenanceView? Related = null);

internal sealed record MaintenanceOperation(MaintenanceTool Id, MaintenanceView Home, string Name,
    string Scope, bool Recommended);

internal static class MaintenanceCatalog
{
    internal const MaintenanceView Default = MaintenanceView.StartupItems;
    internal static IReadOnlyList<MaintenanceDestination> Views { get; } = Array.AsReadOnly<MaintenanceDestination>(
    [
        new(MaintenanceView.DeepCleanup, "deep-cleanup", "Storage", "Deep Cleanup",
            "Not yet available · no cleanup target enumeration, approval, or removal in this preview.",
            ["Folder age alone does not make a file safe to delete.",
             "A future cleanup must show exact targets, exclusions and what can be recovered before approval."],
            MaintenanceView.DiskSpaceAnalysis),
        new(MaintenanceView.LargeFileCleanup, "large-file-cleanup", "Storage", "Large File Cleanup",
            "Not yet available · the current preview reports totals, not a list of individual large files.",
            ["Review each file’s path, size and purpose before selecting it.",
             "Logical file size is not necessarily the disk space that removing it would free."],
            MaintenanceView.DiskSpaceAnalysis),
        new(MaintenanceView.DuplicateCleanup, "duplicate-cleanup", "Storage", "Duplicate Cleanup",
            "Not yet available · no content comparison, keeper selection, or deletion in this preview.",
            ["Names and sizes are not enough to confirm duplicates; compare file contents.",
             "Multiple names for one hard-linked file are not independent copies. Keep at least one verified copy."]),
        new(MaintenanceView.DiskSpaceAnalysis, "disk-space-analysis", "Storage", "Disk Space Analysis",
            "Read-only · scoped folder totals, optional observed allocation, no whole-drive scan or reclaimable-space estimate.",
            []),
        new(MaintenanceView.PrivacyCleanup, "privacy-cleanup", "Privacy & Security", "Privacy Cleanup",
            "Not yet available · INTA does not read or clear browser profiles or privacy data here.",
            ["Keep cookies, signed-in sessions, saved passwords and browsing history separate.",
             "Show the exact selected browser, profile, data type and time range before any approved removal."]),
        new(MaintenanceView.AppUninstaller, "app-uninstaller", "System Tools", "App Uninstaller",
            "Read-only · desktop registry inventory. No Store-app inventory, uninstall or leftover removal.",
            []),
        new(MaintenanceView.StartupItems, "startup-items", "System Tools", "Startup Items",
            "Read-only · registry and startup folders. Windows Settings handles changes; enabled status is not inferred.",
            []),
        new(MaintenanceView.SystemOptimization, "system-optimization", "System Tools", "System Optimization",
            "Local · inspect power plans or open Favorite Tweaks. This preview does not apply optimization presets.",
            []),
        new(MaintenanceView.SystemMaintenance, "system-maintenance", "System Tools", "System Maintenance",
            "Not yet available · no update, integrity-verification or repair run is triggered here.",
            ["Review servicing or reboot status first, and keep a recoverable checkpoint for changes.",
             "After an approved repair, repeat the original symptom check and record the outcome."])
    ]);

    // Recommendations apply to these local first actions only, not to external
    // catalog products. Other supported paths remain visible with the filter off.
    internal static IReadOnlyList<MaintenanceOperation> Operations { get; } = Array.AsReadOnly<MaintenanceOperation>(
    [
        new(MaintenanceTool.StoragePreview, MaintenanceView.DiskSpaceAnalysis, "Folder usage preview", "local disk size allocation metadata", true),
        new(MaintenanceTool.InstalledPrograms, MaintenanceView.AppUninstaller, "Desktop program registrations", "installed apps read-only registry", true),
        new(MaintenanceTool.StartupInventory, MaintenanceView.StartupItems, "Startup inventory", "local registry folders Windows Settings", true),
        new(MaintenanceTool.PowerPlans, MaintenanceView.SystemOptimization, "Power plans", "active Windows power scheme read-only", true),
        new(MaintenanceTool.FavoriteTweaks, MaintenanceView.SystemOptimization, "Favorite Tweaks", "Windows preferences taskbar end task bookmarks", false),
    ]);

    internal static MaintenanceDestination Get(MaintenanceView id) => Views.First(view => view.Id == id);
    internal static MaintenanceView Restore(string? key) =>
        Views.FirstOrDefault(view => string.Equals(view.Key, key, StringComparison.Ordinal))?.Id ?? Default;

    internal static MaintenanceView Move(MaintenanceView selection, NavigationTravel travel)
    {
        int index = Enumerable.Range(0, Views.Count).First(i => Views[i].Id == selection);
        return Views[travel switch
        {
            NavigationTravel.First => 0,
            NavigationTravel.Last => Views.Count - 1,
            NavigationTravel.Previous => (index + Views.Count - 1) % Views.Count,
            _ => (index + 1) % Views.Count
        }].Id;
    }

    internal static MaintenanceOperation[] Filter(MaintenanceView view, string query, bool recommendedOnly)
    {
        string[] terms = query.Split((char[]?)null, StringSplitOptions.TrimEntries | StringSplitOptions.RemoveEmptyEntries);
        return Operations.Where(operation => operation.Home == view && (!recommendedOnly || operation.Recommended)
            && terms.All(term => Match(operation.Name, term) || Match(operation.Scope, term))).ToArray();
    }

    internal static bool MatchesReference(MaintenanceDestination destination, string query, bool recommendedOnly)
        => !recommendedOnly && destination.Related is { } related &&
            query.Split((char[]?)null, StringSplitOptions.TrimEntries | StringSplitOptions.RemoveEmptyEntries)
                 .All(term => Match(Get(related).Name, term) || Match("reference disk usage folders", term));

    private static bool Match(string text, string term) => text.Contains(term, StringComparison.OrdinalIgnoreCase);
}
