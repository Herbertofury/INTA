using System.Runtime.InteropServices;
using System.Text.Json;

namespace INTA.LocalAgent;

// Documented SystemPowerCapabilities query only. No setters, power transition,
// profile writes, process launches, polling, or remote endpoint.
internal static class SleepStates
{
    private const int SystemPowerCapabilities = 4;
    private sealed record States(bool StandbyS1, bool StandbyS2, bool StandbyS3,
        bool HibernateS4, bool HibernationFilePresent);
    private sealed record Reply(int Schema, string Status, bool ReadOnly, string Source, States? States);

    internal static int Run(ReadOnlySpan<string> args, TextWriter output, TextWriter errors)
    {
        if (!args.IsEmpty)
        {
            errors.WriteLine("Usage: inta-agent --sleep-states");
            return 2;
        }

        var status = "unsupported";
        States? states = null;
        if (OperatingSystem.IsWindows())
        {
            status = "unavailable";
            // The documented prefix of SYSTEM_POWER_CAPABILITIES consists of
            // single-byte Win32 BOOLEAN values. Give the OS a length-bounded
            // output buffer and ignore the padding and version-specific suffix.
            byte[] bytes = new byte[256];
            try
            {
                if (CallNtPowerInformation(SystemPowerCapabilities, 0, 0, bytes, (uint)bytes.Length) == 0)
                {
                    states = new States(bytes[3] != 0, bytes[4] != 0, bytes[5] != 0, bytes[6] != 0, bytes[8] != 0);
                    status = "ready";
                }
            }
            catch (Exception ex) when (ex is DllNotFoundException or EntryPointNotFoundException)
            {
                // Unavailable is not the absence of a feature.
            }
        }

        output.WriteLine(JsonSerializer.Serialize(
            new Reply(1, status, true, "system-power-capabilities-prefix", states), JsonSerializerOptions.Web));
        return 0;
    }

    [DllImport("powrprof.dll", ExactSpelling = true)]
    [DefaultDllImportSearchPaths(DllImportSearchPath.System32)]
    private static extern int CallNtPowerInformation(int level, nint input, uint inputLength,
        [Out] byte[] output, uint outputLength);
}
