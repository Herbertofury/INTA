using System.Runtime.InteropServices;
using System.Runtime.Versioning;
using Microsoft.Win32.SafeHandles;

namespace INTA.LocalAgent;

// Separate opt-in aggregate. An allocated file with an unseen alias is not
// reclaimable space. Never expose IDs, filenames, or file contents.
internal sealed class AllocationSummary
{
    private sealed record Sample(long Bytes, uint Links, int Seen, bool Conflict);
    private readonly Dictionary<(ulong Volume, Guid Id), Sample> _files = [];
    private int _unavailable;

    internal void Record(ulong volume, Guid identity, long allocated, uint links)
    {
        if (identity == Guid.Empty || allocated < 0 || links == 0) { Unavailable(); return; }
        var key = (volume, identity);
        if (_files.TryGetValue(key, out var before))
            _files[key] = before with
            {
                Seen = before.Seen + 1,
                Conflict = before.Conflict || before.Bytes != allocated || before.Links != links
            };
        else _files.Add(key, new Sample(allocated, links, 1, false));
    }
    internal void Unavailable() => _unavailable++;

    internal AllocationReceipt Finish(bool traversalPartial)
    {
        long allocated = 0, invisibleAliasBytes = 0;
        int unidentified = _unavailable, conflicts = 0, identified = 0, repeatPaths = 0, unseenLinks = 0;
        foreach (var sample in _files.Values)
        {
            if (sample.Conflict || sample.Seen > sample.Links) { conflicts++; continue; }
            identified++;
            repeatPaths += sample.Seen - 1;
            allocated = checked(allocated + sample.Bytes);
            if (sample.Seen < sample.Links)
            {
                unseenLinks++;
                invisibleAliasBytes = checked(invisibleAliasBytes + sample.Bytes);
            }
        }
        return new AllocationReceipt(allocated, identified, repeatPaths, unidentified, conflicts,
            unseenLinks, invisibleAliasBytes, traversalPartial || unidentified > 0 || conflicts > 0, false);
    }
}

internal sealed record AllocationReceipt(long ObservedAllocationBytes, int IdentifiedFiles, int RepeatedPathnames,
    int UnavailablePathnames, int ConflictingIdentities, int FilesWithUnseenAliases, long AllocationWithUnseenAliases,
    bool Partial, bool ReclaimableSpaceInferred);

[SupportedOSPlatform("windows")]
internal static class WindowsAllocation
{
    [StructLayout(LayoutKind.Sequential)]
    private struct StandardInfo
    {
        public long AllocationSize, EndOfFile;
        public uint NumberOfLinks;
        public byte DeletePending, Directory;
    }

    [StructLayout(LayoutKind.Sequential)]
    private struct FileId
    {
        public ulong Volume;
        public Guid Identifier;
    }

    [StructLayout(LayoutKind.Sequential)]
    private struct FileTag
    {
        public uint Attributes, ReparseTag;
    }

    [DllImport("kernel32.dll", EntryPoint = "CreateFileW", CharSet = CharSet.Unicode, ExactSpelling = true, SetLastError = true)]
    private static extern SafeFileHandle Open(string name, uint access, uint share, IntPtr security, uint disposition, uint flags, IntPtr template);

    [DllImport("kernel32.dll", EntryPoint = "GetFileInformationByHandleEx", ExactSpelling = true, SetLastError = true)]
    [return: MarshalAs(UnmanagedType.Bool)]
    private static extern bool ReadTag(SafeFileHandle handle, int kind, out FileTag value, uint size);

    [DllImport("kernel32.dll", EntryPoint = "GetFileInformationByHandleEx", ExactSpelling = true, SetLastError = true)]
    [return: MarshalAs(UnmanagedType.Bool)]
    private static extern bool ReadStandard(SafeFileHandle handle, int kind, out StandardInfo value, uint size);

    [DllImport("kernel32.dll", EntryPoint = "GetFileInformationByHandleEx", ExactSpelling = true, SetLastError = true)]
    [return: MarshalAs(UnmanagedType.Bool)]
    private static extern bool ReadIdentity(SafeFileHandle handle, int kind, out FileId value, uint size);

    internal static void Measure(FileInfo file, long observedLogicalBytes, AllocationSummary summary)
    {
        // FILE_READ_ATTRIBUTES, all sharing, OPEN_EXISTING, OPEN_REPARSE_POINT.
        using var opened = Open(file.FullName, 0x80, 7, IntPtr.Zero, 3, 0x00200000, IntPtr.Zero);
        if (opened.IsInvalid
            || !ReadTag(opened, 9, out var tag, (uint)Marshal.SizeOf<FileTag>())
            || tag.ReparseTag != 0 || (tag.Attributes & (0x10 | 0x40 | 0x400 | 0x1000)) != 0
            || !ReadStandard(opened, 1, out var size, (uint)Marshal.SizeOf<StandardInfo>())
            || size.Directory != 0 || size.DeletePending != 0 || size.EndOfFile != observedLogicalBytes
            || !ReadIdentity(opened, 18, out var id, (uint)Marshal.SizeOf<FileId>()))
        {
            summary.Unavailable();
            return;
        }
        summary.Record(id.Volume, id.Identifier, size.AllocationSize, size.NumberOfLinks);
    }
}
