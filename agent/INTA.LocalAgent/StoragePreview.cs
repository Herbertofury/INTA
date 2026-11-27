using System.Globalization;
using System.Text.Json;

namespace INTA.LocalAgent;

internal static class StoragePreview
{
    private const int MaxDepth = 128;
    private static readonly JsonSerializerOptions OutputOptions = new(JsonSerializerDefaults.Web);

    internal static int Run(ReadOnlySpan<string> args, TextWriter output, TextWriter error)
    {
        if (args.Length is < 1 or > 3 || string.IsNullOrWhiteSpace(args[0]) || args[0].StartsWith("--", StringComparison.Ordinal))
            return Usage(error);
        if (!ReadRange(args, 1, 7, 1, 3650, out var days) ||
            !ReadRange(args, 2, 100000, 1, 1000000, out var limit))
            return Usage(error);

        try
        {
            var root = Path.GetFullPath(args[0]);
            // A preview should always be scoped. Refuse drive roots and Windows network/device paths.
            if (Directory.GetParent(root) is null ||
                (OperatingSystem.IsWindows() && root.StartsWith(@"\\", StringComparison.Ordinal)))
                throw new ArgumentException("Select a folder on a local drive, not a drive root or network path.");
            for (DirectoryInfo? directory = new(root); directory is not null; directory = directory.Parent)
            {
                var attributes = File.GetAttributes(directory.FullName);
                if ((attributes & FileAttributes.ReparsePoint) != 0)
                    throw new ArgumentException("The selected folder or one of its parents is a link or mount point.");
                if ((attributes & FileAttributes.Directory) == 0)
                    throw new ArgumentException("Select an existing directory.");
            }
            var snapshotTime = DateTimeOffset.UtcNow;
            var totals = new Totals(snapshotTime, days, limit);
            Visit(new DirectoryInfo(root), snapshotTime.UtcDateTime.AddDays(-days), 0, totals);
            output.WriteLine(JsonSerializer.Serialize(totals, OutputOptions));
            return 0;
        }
        catch (ArgumentException ex)
        {
            error.WriteLine(ex.Message);
            return 2;
        }
        catch (Exception ex) when (ReadFailure(ex))
        {
            error.WriteLine("Cannot read the selected folder.");
            return 2;
        }
    }

    private static int Usage(TextWriter error)
    {
        error.WriteLine("Usage: inta-agent --storage-preview <local-folder> [minimum-age-days:1-3650] [entry-limit:1-1000000]");
        return 2;
    }

    private static bool ReadRange(ReadOnlySpan<string> args, int index, int fallback, int min, int max, out int value)
    {
        value = fallback;
        return args.Length <= index ||
            (int.TryParse(args[index], NumberStyles.None, CultureInfo.InvariantCulture, out value) && value >= min && value <= max);
    }

    private static bool ReadFailure(Exception ex) =>
        ex is IOException or UnauthorizedAccessException or System.Security.SecurityException;

    private static void Visit(DirectoryInfo folder, DateTime cutoff, int depth, Totals totals)
    {
        try
        {
            foreach (var entry in folder.EnumerateFileSystemInfos("*", new EnumerationOptions
            {
                RecurseSubdirectories = false, ReturnSpecialDirectories = false,
                IgnoreInaccessible = false, AttributesToSkip = 0
            }))
            {
                if (totals.EntriesScanned == totals.EntryLimit)
                {
                    totals.EntryLimitReached = true;
                    return;
                }
                totals.EntriesScanned++;
                try
                {
                    var attributes = entry.Attributes;
                    if ((attributes & (FileAttributes.ReparsePoint | FileAttributes.Device | FileAttributes.Offline)) != 0)
                    {
                        totals.SpecialEntriesSkipped++;
                        continue;
                    }
                    if (entry is DirectoryInfo child)
                    {
                        if (depth == MaxDepth) totals.DepthLimitedDirectories++;
                        else Visit(child, cutoff, depth + 1, totals);
                        if (totals.EntryLimitReached) return;
                    }
                    else if (entry is FileInfo file)
                    {
                        // Metadata only; never open file contents or report file names.
                        var length = file.Length;
                        var modifiedAt = file.LastWriteTimeUtc;
                        totals.FilesScanned++;
                        totals.LogicalBytes += length;
                        if (modifiedAt <= cutoff)
                        {
                            totals.OlderFiles++;
                            totals.OlderLogicalBytes += length;
                        }
                    }
                }
                catch (Exception ex) when (ReadFailure(ex)) { totals.ReadErrors++; }
            }
        }
        catch (Exception ex) when (ReadFailure(ex)) { totals.ReadErrors++; }
    }

    private sealed class Totals(DateTimeOffset observedAt, int minimumAgeDays, int entryLimit)
    {
        public int SchemaVersion => 1;
        public string ResultType => "storage-preview";
        public DateTimeOffset ObservedAt { get; } = observedAt;
        public int MinimumAgeDays { get; } = minimumAgeDays;
        public int EntryLimit { get; } = entryLimit;
        public int EntriesScanned { get; set; }
        public int FilesScanned { get; set; }
        public long LogicalBytes { get; set; }
        public int OlderFiles { get; set; }
        public long OlderLogicalBytes { get; set; }
        public int SpecialEntriesSkipped { get; set; }
        public int ReadErrors { get; set; }
        public int DepthLimitedDirectories { get; set; }
        public bool EntryLimitReached { get; set; }
        public bool Partial => EntryLimitReached || ReadErrors > 0 || DepthLimitedDirectories > 0;
    }
}
