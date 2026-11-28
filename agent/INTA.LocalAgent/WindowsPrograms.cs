using System.Runtime.Versioning;
using System.Text.Json;
using Microsoft.Win32;

namespace INTA.LocalAgent;

internal sealed record ProgramRow(string Name, string Version, string Publisher, string Scope);
internal sealed record ProgramSnapshot(ProgramRow[] Rows, int Skipped, int Errors, bool Capped);

internal static class WindowsPrograms
{
    private const string Registration = @"Software\Microsoft\Windows\CurrentVersion\Uninstall";
    private const int PerViewLimit = 4096;

    internal static int Run(ReadOnlySpan<string> args, TextWriter output, TextWriter error)
    {
        if (!args.IsEmpty)
        {
            error.WriteLine("Usage: inta-agent --installed-programs");
            return 2;
        }
        if (!OperatingSystem.IsWindows())
        {
            error.WriteLine("Desktop program registrations are supported only on Windows.");
            return 3;
        }
        var view = Read();
        output.WriteLine(JsonSerializer.Serialize(new
        {
            schema = 1,
            scope = "windows-desktop-registrations",
            readOnly = true,
            commandsCollected = false,
            rows = view.Rows,
            skipped = view.Skipped,
            errors = view.Errors,
            capped = view.Capped
        }, new JsonSerializerOptions(JsonSerializerDefaults.Web)));
        return 0;
    }

    [SupportedOSPlatform("windows")]
    internal static ProgramSnapshot Read()
    {
        var rows = new List<ProgramRow>();
        int skipped = 0, errors = 0;
        bool capped = false;
        if (Environment.Is64BitOperatingSystem)
            Read(RegistryHive.LocalMachine, RegistryView.Registry64, "Machine · 64-bit");
        Read(RegistryHive.LocalMachine, Environment.Is64BitOperatingSystem ? RegistryView.Registry32 : RegistryView.Default,
             Environment.Is64BitOperatingSystem ? "Machine · 32-bit" : "Machine");
        Read(RegistryHive.CurrentUser, RegistryView.Default, "Current user");
        return new(rows.OrderBy(x => x.Name, StringComparer.CurrentCultureIgnoreCase).ThenBy(x => x.Scope).ToArray(), skipped, errors, capped);

        void Read(RegistryHive hive, RegistryView view, string scope)
        {
            try
            {
                using var root = RegistryKey.OpenBaseKey(hive, view);
                using var directory = root.OpenSubKey(Registration, writable: false);
                if (directory is null) return;
                var keys = directory.GetSubKeyNames();
                if (keys.Length > PerViewLimit) capped = true;
                foreach (string key in keys.Take(PerViewLimit))
                {
                    try
                    {
                        using var app = directory.OpenSubKey(key, writable: false);
                        if (app is null) { skipped++; continue; }
                        string name = Metadata(app, "DisplayName");
                        if (name.Length == 0 || app.GetValue("SystemComponent") is 1) { skipped++; continue; }
                        // Scope and display metadata only. Do not read uninstall commands, paths, or key identities.
                        rows.Add(new(name, Metadata(app, "DisplayVersion"), Metadata(app, "Publisher"), scope));
                    }
                    catch (Exception e) when (ReadFailure(e)) { errors++; }
                }
            }
            catch (Exception e) when (ReadFailure(e)) { errors++; }
        }
    }

    private static bool ReadFailure(Exception e) => e is IOException or UnauthorizedAccessException or System.Security.SecurityException;

    [SupportedOSPlatform("windows")]
    private static string Metadata(RegistryKey app, string name)
    {
        var raw = app.GetValue(name, null, RegistryValueOptions.DoNotExpandEnvironmentNames) as string;
        if (string.IsNullOrWhiteSpace(raw)) return "";
        var value = raw.Trim();
        if (value.Length <= 384) return value;
        var length = char.IsHighSurrogate(value[383]) ? 383 : 384;
        return value[..length] + "…";
    }
}

