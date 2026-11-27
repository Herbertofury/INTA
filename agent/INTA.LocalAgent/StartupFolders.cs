using System.Runtime.Versioning;
using System.Security;

namespace INTA.LocalAgent;

internal static class StartupFolders
{
    private const int EntryLimit = 1024;
    internal const string SettingsUri = "ms-settings:startupapps";

    [SupportedOSPlatform("windows")]
    internal static StartupFolder[] ReadWindowsFolders() =>
    [
        Read("current-user", () => Resolve(Environment.SpecialFolder.Startup), IsLocalDrive),
        Read("all-users", () => Resolve(Environment.SpecialFolder.CommonStartup), IsLocalDrive),
    ];

    private static string Resolve(Environment.SpecialFolder folder) =>
        Environment.GetFolderPath(folder, Environment.SpecialFolderOption.DoNotVerify);

    private static bool IsLocalDrive(string path)
    {
        var root = Path.GetPathRoot(path);
        if (root is null || root.Length != 3 || root[1] != ':') return false;
        return new DriveInfo(root).DriveType is DriveType.Fixed or DriveType.Removable or DriveType.Ram;
    }

    internal static StartupFolder Read(string scope, Func<string> resolve, Func<string, bool> allowLocation)
    {
        try
        {
            var directory = resolve();
            if (string.IsNullOrEmpty(directory))
                return new(scope, "unavailable", [], false);

            if (!Path.IsPathFullyQualified(directory) || !allowLocation(directory))
                return new(scope, "location-not-scanned", [], false);

            // Walk ancestors from root toward the directory. Don't descend
            // through a junction/link or follow a link capably pointing off-device.
            var ancestors = new Stack<DirectoryInfo>();
            for (var ancestor = new DirectoryInfo(directory); ancestor is not null; ancestor = ancestor.Parent)
                ancestors.Push(ancestor);
            while (ancestors.TryPop(out var ancestor))
            {
                var attributes = File.GetAttributes(ancestor.FullName);
                if ((attributes & FileAttributes.ReparsePoint) != 0)
                    return new(scope, "location-not-scanned", [], false);
                if ((attributes & FileAttributes.Directory) == 0)
                    return new(scope, "unavailable", [], false);
            }

            // Copy only the immediate base names. Do not open files or resolve
            // shortcut targets. A cap deliberately bounds directory traversal.
            var sample = Directory.EnumerateFileSystemEntries(directory, "*", SearchOption.TopDirectoryOnly)
                .Take(EntryLimit + 1)
                .Select(Path.GetFileName)
                .ToArray();
            var names = sample.Take(EntryLimit).Order(StringComparer.OrdinalIgnoreCase)
                .ThenBy(name => name, StringComparer.Ordinal).ToArray();
            return new(scope, "read", names, sample.Length > EntryLimit);
        }
        catch (Exception e) when (e is DirectoryNotFoundException or FileNotFoundException)
        {
            return new(scope, "not-present", [], false);
        }
        catch (Exception e) when (e is SecurityException or UnauthorizedAccessException or IOException or ArgumentException)
        {
            return new(scope, "unavailable", [], false);
        }
    }

    internal sealed record StartupFolder(string Scope, string Status, string?[] EntryNames, bool Truncated);
}
