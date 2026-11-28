using INTA.Desktop;
using Inta.Desktop;

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
    Require(PortableMaintenanceRoute.Read(portable) == null && !Directory.Exists(Path.Combine(portable, "state")), "A passive Desktop open does not write state");
    Require(PortableMaintenanceRoute.Save(portable, MaintenanceView.LargeFileCleanup), "Write portable route");
    Require(PortableMaintenanceRoute.Read(portable) == MaintenanceView.LargeFileCleanup, "Restore last maintenance route");
    Require(PortableMaintenanceRoute.Save(portable, null) && PortableMaintenanceRoute.Read(portable) == null, "Technician Desk remains the fallback");
}
finally { Directory.Delete(portable, recursive: true); }
Console.WriteLine("PASS: shared nine routes and search; one scoped storage evidence report with separate owner views; exact portable navigation restoration.");
static void Require(bool value, string name) { if (!value) throw new InvalidOperationException(name); }
static void ExpectRejected(Dictionary<string, object> sample)
{
    try { MaintenanceEvidence.Read("chosen-folder", sample); }
    catch (InvalidDataException) { return; }
    throw new InvalidOperationException("Invalid core report was accepted");
}
