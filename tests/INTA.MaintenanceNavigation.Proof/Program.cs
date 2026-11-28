using INTA.Desktop;

var expected = new[]
{
    ("Storage", "Deep Cleanup", "deep-cleanup"), ("Storage", "Large File Cleanup", "large-file-cleanup"),
    ("Storage", "Duplicate Cleanup", "duplicate-cleanup"), ("Storage", "Disk Space Analysis", "disk-space-analysis"),
    ("Privacy & Security", "Privacy Cleanup", "privacy-cleanup"), ("System Tools", "App Uninstaller", "app-uninstaller"),
    ("System Tools", "Startup Items", "startup-items"), ("System Tools", "System Optimization", "system-optimization"),
    ("System Tools", "System Maintenance", "system-maintenance")
};
var views = MaintenanceCatalog.Views;
Require(expected.SequenceEqual(views.Select(v => (v.Group, v.Name, v.Key))), "Nine exact independently ordered routes");
Require(views.Select(v => v.Id).Distinct().Count() == 9, "Unique category IDs");
Require(MaintenanceCatalog.Operations.Select(o => o.Id).Distinct().Count() == MaintenanceCatalog.Operations.Count, "One home per real operation");
Require(MaintenanceCatalog.Operations.Count == 5, "Only verified native paths appear");
Require(MaintenanceCatalog.Filter(MaintenanceView.SystemOptimization, "  Windows  TASKBAR ", false)
    .Select(o => o.Id).SequenceEqual([MaintenanceTool.FavoriteTweaks]), "Multi-term search matches metadata in the current view");
Require(MaintenanceCatalog.Filter(MaintenanceView.SystemOptimization, "taskbar", true).Length == 0, "Filters combine");
Require(MaintenanceCatalog.Filter(MaintenanceView.SystemOptimization, "", true)
    .Select(o => o.Id).SequenceEqual([MaintenanceTool.PowerPlans]), "Only recommendation in Optimization is its native read-only starter tool");
Require(MaintenanceCatalog.Filter(MaintenanceView.StartupItems, "power", false).Length == 0, "Filtering never changes view");
foreach (var v in views)
{
    Require(MaintenanceCatalog.Restore(v.Key) == v.Id, "Restore exact canonical key");
    var all = MaintenanceCatalog.Filter(v.Id, "", false);
    var filtered = MaintenanceCatalog.Filter(v.Id, "", true);
    Require(filtered.All(x => x.Recommended && x.Home == v.Id && all.Contains(x)), "Recommended is a same-view subset");
    Require(all.SequenceEqual(MaintenanceCatalog.Filter(v.Id, "", false)), "Clearing restores exactly the original results");
    if (all.Length == 0) Require(v.Review.Length > 0 && v.Scope.StartsWith("Not yet available"), "No fake native capability");
}
var viaKeyboard = MaintenanceView.DeepCleanup;
foreach (var view in views)
{
    Require(viaKeyboard == view.Id, "Keyboard order reaches every category across group boundaries");
    viaKeyboard = MaintenanceCatalog.Move(viaKeyboard, NavigationTravel.Next);
}
Require(viaKeyboard == MaintenanceView.DeepCleanup, "Keyboard wraps once, with no hidden navigation destinations");
Require(MaintenanceCatalog.Move(viaKeyboard, NavigationTravel.Previous) == MaintenanceView.SystemMaintenance, "Previous wraps backward");
Require(MaintenanceCatalog.Move(MaintenanceView.PrivacyCleanup, NavigationTravel.First) == MaintenanceView.DeepCleanup, "Home goes to first");
Require(MaintenanceCatalog.Move(MaintenanceView.PrivacyCleanup, NavigationTravel.Last) == MaintenanceView.SystemMaintenance, "End goes to last");
var deep = MaintenanceCatalog.Get(MaintenanceView.DeepCleanup);
Require(MaintenanceCatalog.MatchesReference(deep, "disk usage", false), "Real cross-link can match search");
Require(!MaintenanceCatalog.MatchesReference(deep, "uninstall", false), "Nonmatching cross-link hides");
Require(!MaintenanceCatalog.MatchesReference(deep, "", true), "Cross-links are not recommendations");

var root = Path.Combine(Path.GetTempPath(), "inta-navigation-proof-" + Guid.NewGuid().ToString("N"));
Directory.CreateDirectory(root);
try
{
    var state = Path.Combine(root, "state");
    var preference = Path.Combine(state, "maintenance-view.v1");
    Require(NavigationPreference.Read(root) == MaintenanceCatalog.Default, "Fresh app starts on Startup Items");
    Require(!Directory.Exists(state), "Read does not write anything");
    foreach (var v in views)
    {
        Require(NavigationPreference.Save(root, v.Id), "Saved to portable directory");
        Require(NavigationPreference.Read(root) == v.Id, "Restored after restart");
    }
    File.WriteAllText(preference, new string('a', 128));
    Require(NavigationPreference.Read(root) == MaintenanceCatalog.Default, "Oversized records never parsed");
    File.WriteAllText(preference, "../../../elsewhere");
    Require(NavigationPreference.Read(root) == MaintenanceCatalog.Default, "Unknown and path-like values never interpreted");
    Require(Directory.GetFiles(state).Length == 1, "No temporary file remains");
    if (!OperatingSystem.IsWindows())
    {
        File.Delete(preference);
        var external = Path.Combine(root, "external");
        File.WriteAllText(external, "app-uninstaller");
        File.CreateSymbolicLink(preference, external);
        Require(NavigationPreference.Read(root) == MaintenanceCatalog.Default, "Redirected preference is not read");
        Require(!NavigationPreference.Save(root, MaintenanceView.PrivacyCleanup) && File.ReadAllText(external) == "app-uninstaller", "Redirected preference is not overwritten");
    }
}
finally { Directory.Delete(root, recursive: true); }
Console.WriteLine("PASS: nine exact routes; five unique native operations; scoped search and recommendations; honest unsupported paths; portable restoration and unsafe-state fallback.");

static void Require(bool value, string message) { if (!value) throw new InvalidOperationException(message); }
