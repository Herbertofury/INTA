using Inta.Desktop.Tweaks;

static void Require(bool condition, string message) { if (!condition) throw new InvalidOperationException(message); }

var path = Directory.CreateTempSubdirectory("inta-favorite-tweak-fixture-").FullName;
try
{
    var end = FavoriteTweaks.TaskbarEndTask;
    var ownership = FavoriteTweaks.TakeOwnership;
    var favorites = new FavoriteTweaks(path);
    Require(favorites.Rows().SequenceEqual([end, ownership]) && favorites.Pinned(end) && !favorites.Pinned(ownership),
        "Curated taskbar action is pinned by default; ownership is visible, not pinned, and unavailable.");
    Require(!Directory.Exists(Path.Combine(path, "state")), "Opening Favorite Tweaks does not write state.");
    Require(ownership.Summary.Contains("No INTA context menu is installed", StringComparison.Ordinal), "Never imply ownership is enabled.");

    Require(favorites.SetPinned(end, false) && favorites.SetPinned(ownership, true), "Bookmarks persist in the portable folder.");
    var reopened = new FavoriteTweaks(path);
    Require(reopened.Rows().SequenceEqual([ownership, end]) && reopened.Pinned(ownership) && !reopened.Pinned(end)
        && ownership.Summary.Contains("No INTA context menu is installed", StringComparison.Ordinal),
        "Moving a bookmarked unavailable action does not enable it.");
    Require(reopened.SetPinned(ownership, false), "Changing an existing bookmark succeeds.");
    Require(!new FavoriteTweaks(path).Pinned(ownership), "Replacement survives reload.");
    var markers = Directory.GetFiles(Path.Combine(path, "state")).Select(Path.GetFileName).OrderBy(x => x).ToArray();
    Require(markers.SequenceEqual(["favorite-tweak-take-ownership.v1", "favorite-tweak-taskbar-end-task.v1"]),
        "Only known portable marker files are produced.");
    File.WriteAllText(Path.Combine(path, "state", "favorite-tweak-taskbar-end-task.v1"), "not a valid marker");
    Require(new FavoriteTweaks(path).Pinned(end), "A corrupt marker falls back to the default.");

    var blockedRoot = Directory.CreateDirectory(Path.Combine(path, "blocked")).FullName;
    File.WriteAllText(Path.Combine(blockedRoot, "state"), "existing data");
    var blocked = new FavoriteTweaks(blockedRoot);
    Require(!blocked.SetPinned(ownership, true) && blocked.Pinned(ownership) && !new FavoriteTweaks(blockedRoot).Pinned(ownership),
        "A failed portable save remains honestly session-only.");
    Require(File.ReadAllText(Path.Combine(blockedRoot, "state")) == "existing data", "An unrelated file stays unchanged.");

    if (!OperatingSystem.IsWindows())
    {
        var reference = Directory.CreateDirectory(Path.Combine(path, "redirected")).FullName;
        var other = Directory.CreateDirectory(Path.Combine(path, "elsewhere")).FullName;
        Directory.CreateSymbolicLink(Path.Combine(reference, "state"), other);
        var throughLink = new FavoriteTweaks(reference);
        Require(!throughLink.SetPinned(ownership, true) && !Directory.EnumerateFileSystemEntries(other).Any(),
            "A redirected portable state directory cannot receive a write.");
    }

    Require(FavoriteTweaks.SettingsUri(new Version(10, 0, 19045)) == "" &&
        FavoriteTweaks.SettingsUri(new Version(10, 0, 22000)) == "",
        "Older releases never show a supported Windows 11 taskbar action.");
    Require(FavoriteTweaks.SettingsUri(new Version(10, 0, 26100)) == "ms-settings:developers"
        && FavoriteTweaks.EndTaskPath(new Version(10, 0, 26100)).Contains("For developers", StringComparison.Ordinal),
        "Older supported Windows 11 builds use the documented developer settings route.");
    Require(FavoriteTweaks.SettingsUri(new Version(10, 0, 26200)) == "ms-settings:developers"
        && FavoriteTweaks.EndTaskPath(new Version(10, 0, 26200)).Contains("Advanced > Taskbar > End Task", StringComparison.Ordinal),
        "25H2 fallback instructions point to Windows Advanced Settings.");
    Require(FavoriteTweaks.EndTaskStatus(new Version(10, 0, 28000)).Contains("managed", StringComparison.OrdinalIgnoreCase)
        || FavoriteTweaks.EndTaskStatus(new Version(10, 0, 28000)).Contains("organization", StringComparison.OrdinalIgnoreCase),
        "No assumed preference state if access is organization-managed.");
    Console.WriteLine("Favorite Tweaks proof passed: portable persistence, defaults, explicit bookmarking, redirected/unwritable storage, and version-aware Settings routing. No Windows setting was opened or changed.");
}
finally
{
    Directory.Delete(path, recursive: true);
}
