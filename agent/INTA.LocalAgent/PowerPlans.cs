using System.Runtime.InteropServices;
using System.Text;
using System.Text.Json;

namespace INTA.LocalAgent;

internal static class PowerPlans
{
    private const uint Scheme = 16, Success = 0, MoreData = 234, NoMoreItems = 259;
    private const int Limit = 256;

    // One-shot local inventory: no implementation for activation, policy edits,
    // background monitoring, or ingress from the Local Agent web API.
    internal static int Run(ReadOnlySpan<string> args, TextWriter output, TextWriter errors)
    {
        if (!args.IsEmpty)
        {
            errors.WriteLine("Usage: inta-agent --power-plans");
            return 2;
        }

        if (!OperatingSystem.IsWindows())
        {
            output.WriteLine(JsonSerializer.Serialize(new Snapshot("unsupported", null, [], null), JsonSerializerOptions.Web));
            return 0;
        }

        Snapshot result;
        try { result = ReadWindows(); }
        catch (Exception ex) when (ex is DllNotFoundException or EntryPointNotFoundException)
        {
            result = new Snapshot("unavailable", null, [], null);
        }
        output.WriteLine(JsonSerializer.Serialize(result, JsonSerializerOptions.Web));
        return 0;
    }

    private static Snapshot ReadWindows()
    {
        if (Active() is not { } before)
            return new Snapshot("unavailable", null, [], null);

        var plans = new Dictionary<Guid, Plan>();
        var stopped = false;
        uint? error = null;
        var buffer = new byte[16]; // Native GUID bytes; never parse human-readable output.
        for (uint index = 0; index <= Limit; index++)
        {
            uint size = (uint)buffer.Length;
            var code = PowerEnumerate(0, 0, 0, Scheme, index, buffer, ref size);
            if (code == NoMoreItems) { stopped = true; break; }
            if (code != Success || size != buffer.Length)
            {
                error = code == Success ? null : code;
                break;
            }
            if (index == Limit) break;
            var id = new Guid(buffer);
            if (id != Guid.Empty && !plans.ContainsKey(id))
                plans.Add(id, new Plan(id, FriendlyName(id)));
        }

        var after = Active();
        var rows = plans.Values.OrderBy(p => p.Id).ToArray();
        if (after != before)
            return new Snapshot("unstable", null, rows, error);
        if (!stopped || !plans.ContainsKey(before))
            return new Snapshot("incomplete", before, rows, error);
        return new Snapshot("ready", before, rows, null);
    }

    private static Guid? Active()
    {
        var code = PowerGetActiveScheme(0, out var native);
        if (native == 0) return null;
        try
        {
            if (code != Success) return null;
            var id = Marshal.PtrToStructure<Guid>(native);
            return id == Guid.Empty ? null : id;
        }
        finally { _ = LocalFree(native); }
    }

    private static string? FriendlyName(Guid id)
    {
        uint size = 0;
        var code = PowerReadFriendlyName(0, in id, 0, 0, null, ref size);
        if ((code != Success && code != MoreData) || size is < 2 or > 4096 || (size & 1) != 0) return null;
        var bytes = new byte[size];
        code = PowerReadFriendlyName(0, in id, 0, 0, bytes, ref size);
        if (code != Success || size < 2 || size > bytes.Length || (size & 1) != 0) return null;
        var name = Encoding.Unicode.GetString(bytes, 0, (int)size);
        var end = name.IndexOf('\0');
        if (end >= 0) name = name[..end];
        return string.IsNullOrWhiteSpace(name) ? null : name;
    }

    private sealed record Plan(Guid Id, string? Name);
    private sealed record Snapshot(string Status, Guid? ActiveId, Plan[] Plans, uint? ErrorCode);

    [DllImport("powrprof.dll", ExactSpelling = true)]
    [DefaultDllImportSearchPaths(DllImportSearchPath.System32)]
    private static extern uint PowerEnumerate(nint key, nint scheme, nint subgroup, uint access, uint index, [Out] byte[] buffer, ref uint size);

    [DllImport("powrprof.dll", ExactSpelling = true)]
    [DefaultDllImportSearchPaths(DllImportSearchPath.System32)]
    private static extern uint PowerGetActiveScheme(nint key, out nint scheme);

    [DllImport("powrprof.dll", ExactSpelling = true)]
    [DefaultDllImportSearchPaths(DllImportSearchPath.System32)]
    private static extern uint PowerReadFriendlyName(nint key, in Guid scheme, nint subgroup, nint setting, [Out] byte[]? buffer, ref uint size);

    [DllImport("kernel32.dll", ExactSpelling = true)]
    [DefaultDllImportSearchPaths(DllImportSearchPath.System32)]
    private static extern nint LocalFree(nint handle);
}
