// Read-only inventory of documented Windows logon registrations, not a startup manager.
using System.Runtime.Versioning;
using System.Security;
using System.Text.Json;
using Microsoft.Win32;

namespace INTA.LocalAgent;

internal static class StartupInventory
{
    private const string PathPrefix = @"Software\Microsoft\Windows\CurrentVersion\";
    private const int SourceLimit = 1024;
    private static readonly JsonSerializerOptions Json = new(JsonSerializerDefaults.Web);

    internal static int Run(ReadOnlySpan<string> args, TextWriter output, TextWriter error)
    {
        var includeFolders = args.Length == 1 && args[0] == "--include-folders";
        if (!args.IsEmpty && !includeFolders)
        {
            error.WriteLine("Usage: inta-agent --startup-inventory [--include-folders]");
            return 2;
        }
        if (!OperatingSystem.IsWindows())
        {
            error.WriteLine("Windows logon registration inventory is not supported on this platform.");
            return 3;
        }

        if (includeFolders)
        {
            output.WriteLine(JsonSerializer.Serialize(new
            {
                schema = 2,
                scope = "windows-logon-registrations-and-local-startup-folders",
                readOnly = true,
                enabledState = "not-evaluated",
                argumentsCollected = false,
                sources = ReadWindowsSources(),
                startupFolders = StartupFolders.ReadWindowsFolders(),
                startupSettingsUri = StartupFolders.SettingsUri
            }, Json));
            return 0;
        }

        output.WriteLine(JsonSerializer.Serialize(new
        {
            schema = 1,
            scope = "windows-logon-run-keys-only",
            readOnly = true,
            enabledState = "not-evaluated",
            argumentsCollected = false,
            sources = ReadWindowsSources()
        }, Json));
        return 0;
    }

    [SupportedOSPlatform("windows")]
    private static List<StartupSource> ReadWindowsSources()
    {
        var sources = new List<StartupSource>(6);
        // HKCU Software is shared; probing it twice would duplicate the same entries.
        // HKLM Software is redirected on 64-bit Windows; keep those views separate.
        foreach (var name in new[] { "Run", "RunOnce" })
        {
            sources.Add(ReadKey(RegistryHive.CurrentUser, RegistryView.Default, name, "current-user", "shared"));
            if (Environment.Is64BitOperatingSystem)
            {
                sources.Add(ReadKey(RegistryHive.LocalMachine, RegistryView.Registry64, name, "machine", "64-bit"));
                sources.Add(ReadKey(RegistryHive.LocalMachine, RegistryView.Registry32, name, "machine", "32-bit"));
            }
            else
            {
                sources.Add(ReadKey(RegistryHive.LocalMachine, RegistryView.Default, name, "machine", "default"));
            }
        }
        return sources;
    }

    [SupportedOSPlatform("windows")]
    private static StartupSource ReadKey(RegistryHive hive, RegistryView view, string name, string scope, string viewName)
    {
        try
        {
            using var baseKey = RegistryKey.OpenBaseKey(hive, view);
            using var key = baseKey.OpenSubKey(PathPrefix + name, writable: false);
            if (key is null) return new(scope, viewName, name, "not-present", [], false);

            // Value names only. Do not read targets/arguments, resolve links, or execute anything.
            var entries = key.GetValueNames();
            Array.Sort(entries, StringComparer.OrdinalIgnoreCase);
            return new(scope, viewName, name, "read", entries.Take(SourceLimit).ToArray(), entries.Length > SourceLimit);
        }
        catch (Exception e) when (e is SecurityException or UnauthorizedAccessException or IOException)
        {
            return new(scope, viewName, name, "unavailable", [], false);
        }
    }

    private sealed record StartupSource(string Scope, string View, string Key, string Status, string[] EntryNames, bool Truncated);
}
