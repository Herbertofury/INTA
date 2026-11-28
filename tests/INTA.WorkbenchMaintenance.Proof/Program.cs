using INTA.Desktop;
using Inta.Desktop;
using Inta.Desktop.Tweaks;

Require(MaintenanceTaxonomy.Rows.Select(v => v.Group).Distinct().SequenceEqual(["Storage", "Privacy & Security", "System Tools"]), "Three exact groups");
Require(MaintenanceTaxonomy.Rows.Count == 9 && MaintenanceTaxonomy.Rows.Select(v => v.Key).Distinct().Count() == 9, "Nine stable canonical keys");
Require(MaintenanceTaxonomy.FindView("System Optimization") == MaintenanceView.SystemOptimization
    && MaintenanceTaxonomy.Get(MaintenanceView.SystemOptimization).Key == "system-optimization", "Canonical Favorite Tweaks parent");
Require(!MaintenanceTaxonomy.FindView("Technician Desk").HasValue && !MaintenanceTaxonomy.FindView("INTA Recommended").HasValue, "The Desk and filter are not maintenance routes");
Require(MaintenanceTaxonomy.Matches(MaintenanceTaxonomy.Terms("    LARGE  fiLe "), "Large files", "read-only"), "Scoped multi-term match");
Require(!MaintenanceTaxonomy.Matches(MaintenanceTaxonomy.Terms("large privacy"), "Large files", "read-only"), "No broadening across routes");

var largest = new object[] { new Dictionary<string, object> { ["path"] = "fixture.bin", ["sizeBytes"] = 11 } };
var dupes = new object[] { new Dictionary<string, object> { ["sha256"] = new string('a', 64), ["sizeBytes"] = 5, ["files"] = new[] { "a", "b" } } };
var report = new Dictionary<string, object> {
    ["kind"] = "space-analysis", ["schemaVersion"] = 1, ["fileCount"] = 3,
    [MaintenanceEvidence.LargestKey] = largest, [MaintenanceEvidence.DuplicatesKey] = dupes
};
var evidence = MaintenanceEvidence.Read("chosen-folder", report);
Require(evidence.SelectedFolder == "chosen-folder" && ReferenceEquals(largest, evidence.Get(MaintenanceEvidence.LargestKey))
    && ReferenceEquals(dupes, evidence.Get(MaintenanceEvidence.DuplicatesKey)), "One exact observation drives both projections");
Require(!MaintenanceEvidence.DiskKeys.Contains(MaintenanceEvidence.LargestKey)
    && !MaintenanceEvidence.DiskKeys.Contains(MaintenanceEvidence.DuplicatesKey), "Disk totals do not repeat the dedicated evidence views");
ExpectRejected(new(report) { ["kind"] = "unexpected" });
ExpectRejected(new(report) { ["schemaVersion"] = 2 });
ExpectRejected(new(report) { ["schemaVersion"] = "1" });
ExpectRejected(new(report) { [MaintenanceEvidence.DuplicatesKey] = "not-an-array" });
Require(ReferenceEquals(report, evidence.Report), "Export retains all original scope evidence");

string portable = Path.Combine(Path.GetTempPath(), "inta-workbench-route-proof-" + Guid.NewGuid().ToString("N"));
Directory.CreateDirectory(portable);
try
{
    string legacy = Directory.CreateDirectory(Path.Combine(portable, "legacy")).FullName;
    Require(PortableMaintenanceRoute.Read(legacy) == null && !Directory.Exists(Path.Combine(legacy, "state")), "Legacy passive open does not write state");
    Require(PortableMaintenanceRoute.Save(legacy, MaintenanceView.LargeFileCleanup), "Legacy relative-to-app route saves");
    Require(PortableMaintenanceRoute.Read(legacy) == MaintenanceView.LargeFileCleanup, "Legacy route restores");
    Require(new FavoriteTweaks(legacy).SetPinned(FavoriteTweaks.TaskbarEndTask, false)
        && File.ReadAllText(Path.Combine(legacy, "state", "favorite-tweak-taskbar-end-task.v1")) == "0", "Legacy WinForms bookmark remains beside its app");

    string payload = Directory.CreateDirectory(Path.Combine(portable, "base")).FullName;
    string localAppData = Path.Combine(portable, "installed-profile");
    string external = Path.Combine(portable, "base-sibling", "prefs");
    string outerState = Path.Combine(portable, "state");
    File.WriteAllText(Path.Combine(payload, "manifest.json"), "{\"files\":[]}");
    File.WriteAllText(Path.Combine(payload, "manifest.p7s"), "fixture only");
    string[] payloadFiles = Directory.GetFiles(payload);

    Require(WorkbenchStateDirectory.Resolve(payload, null, localAppData) == Path.Combine(localAppData, "INTA", "state"),
        "Installed fallback is the existing theme directory, never payload/state");
    File.WriteAllText(Path.Combine(portable, "inta-portable.marker"), "fixture");
    Require(WorkbenchStateDirectory.Resolve(payload, null, localAppData) == outerState, "Direct packaged launch finds the outer portable state");
    Require(WorkbenchStateDirectory.Resolve(payload, outerState, localAppData) == outerState, "Official portable launcher state is accepted");
    Require(WorkbenchStateDirectory.Resolve(payload, external, localAppData) == external, "INTA_DATA_DIR wins and a similarly named sibling is allowed");

    Func<string> resolve = () => WorkbenchStateDirectory.Resolve(payload, outerState, localAppData);
    Require(PortableMaintenanceRoute.ReadStateDirectory(resolve()) == null
        && FavoriteTweaks.ForStateDirectory(resolve).Pinned(FavoriteTweaks.TaskbarEndTask)
        && !Directory.Exists(outerState), "Read-only defaults do not write state");
    Require(PortableMaintenanceRoute.SaveStateDirectory(resolve(), MaintenanceView.DiskSpaceAnalysis), "Save outside signed payload");
    Require(FavoriteTweaks.ForStateDirectory(resolve).SetPinned(FavoriteTweaks.TaskbarEndTask, false), "Save bookmark outside signed payload");
    Require(File.ReadAllText(Path.Combine(outerState, "maintenance-view.v1")) == "disk-space-analysis"
        && File.ReadAllText(Path.Combine(outerState, "favorite-tweak-taskbar-end-task.v1")) == "0", "Exact external marker contents");
    Require(PortableMaintenanceRoute.ReadStateDirectory(resolve()) == MaintenanceView.DiskSpaceAnalysis
        && !FavoriteTweaks.ForStateDirectory(resolve).Pinned(FavoriteTweaks.TaskbarEndTask), "Fresh-state restart restores route and bookmark");
    Require(PortableMaintenanceRoute.SaveStateDirectory(resolve(), null)
        && PortableMaintenanceRoute.ReadStateDirectory(resolve()) == null, "Technician Desk remains the fallback");

    string alternate = Directory.CreateDirectory(Path.Combine(portable, "other-signed-revision")).FullName;
    File.Copy(Path.Combine(payload, "manifest.json"), Path.Combine(alternate, "manifest.json"));
    File.Copy(Path.Combine(payload, "manifest.p7s"), Path.Combine(alternate, "manifest.p7s"));
    var blocked = new List<string> { "relative", payload, Path.Combine(payload, "state", "nested"),
        Path.Combine(portable, "sub", "..", "base", "state"), Path.Combine(alternate, "state") };
    if (OperatingSystem.IsWindows())
        blocked.AddRange([@"C:drive-relative", @"\drive-relative", @"\\?\C:\device-path", @"\\.\C:\device-path"]);
    else
    {
        // Windows symlink creation can require privileges; Linux validates both
        // a direct symlink and a missing descendant through a redirected ancestor.
        string link = Path.Combine(portable, "redirect");
        Directory.CreateSymbolicLink(link, payload);
        blocked.Add(link);
        blocked.Add(Path.Combine(link, "not-yet-created", "state"));
    }
    foreach (string invalid in blocked)
    {
        Func<string> reject = () => WorkbenchStateDirectory.Resolve(payload, invalid, localAppData);
        Require(reject() == null, "Reject unsafe path: " + invalid);
        Require(!PortableMaintenanceRoute.SaveStateDirectory(reject(), MaintenanceView.AppUninstaller)
            && PortableMaintenanceRoute.ReadStateDirectory(reject()) == null, "No persistent route for rejected path");
        var session = FavoriteTweaks.ForStateDirectory(reject);
        Require(!session.SetPinned(FavoriteTweaks.TaskbarEndTask, false) && !session.Pinned(FavoriteTweaks.TaskbarEndTask),
            "Blocked bookmark remains session-only");
        Require(FavoriteTweaks.ForStateDirectory(reject).Pinned(FavoriteTweaks.TaskbarEndTask), "No persistent bookmark for rejected path");
    }
    Require(Directory.GetFiles(payload, "*", SearchOption.AllDirectories).Order().SequenceEqual(payloadFiles.Order())
        && !Directory.Exists(Path.Combine(payload, "state")) && !Directory.Exists(Path.Combine(alternate, "state")),
        "Signed payload and alternate revision are not mutated");
}
finally { Directory.Delete(portable, recursive: true); }
Console.WriteLine("PASS: shared nine routes and search; one scoped storage evidence report with separate owner views; isolated portable and installed state, strict payload boundaries, route and bookmark restoration.");
static void Require(bool value, string name) { if (!value) throw new InvalidOperationException(name); }
static void ExpectRejected(Dictionary<string, object> sample)
{
    try { MaintenanceEvidence.Read("chosen-folder", sample); }
    catch (InvalidDataException) { return; }
    throw new InvalidOperationException("Invalid core report was accepted");
}
