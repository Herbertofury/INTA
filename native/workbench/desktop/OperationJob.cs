using System;
using System.ComponentModel;
using System.Diagnostics;
using System.Runtime.InteropServices;
using Microsoft.Win32.SafeHandles;

namespace Inta.Desktop
{
    // Keep Windows provider processes in the per-operation lifetime when navigation
    // cancels an inspection or the window closes during a read-only scenario.
    internal sealed class OperationJob : SafeHandleZeroOrMinusOneIsInvalid
    {
        [StructLayout(LayoutKind.Sequential)]
        private struct Limits
        {
            internal long ProcessTime, JobTime;
            internal uint Flags;
            internal UIntPtr MinWorkingSet, MaxWorkingSet;
            internal uint ActiveCount;
            internal UIntPtr Affinity;
            internal uint Priority, Scheduling;
            internal ulong ReadCount, WriteCount, OtherCount, ReadBytes, WriteBytes, OtherBytes;
            internal UIntPtr ProcessMemory, JobMemory, PeakProcessMemory, PeakJobMemory;
        }
        [DllImport("kernel32.dll", SetLastError = true, CharSet = CharSet.Unicode)]
        private static extern OperationJob CreateJobObject(IntPtr security, string name);
        [DllImport("kernel32.dll", SetLastError = true)]
        private static extern bool SetInformationJobObject(OperationJob job, int infoClass, ref Limits info, int size);
        [DllImport("kernel32.dll", SetLastError = true)]
        private static extern bool AssignProcessToJobObject(OperationJob job, IntPtr process);
        [DllImport("kernel32.dll", SetLastError = true)]
        private static extern bool TerminateJobObject(OperationJob job, uint exitCode);
        [DllImport("kernel32.dll")]
        private static extern bool CloseHandle(IntPtr handle);
        private OperationJob() : base(true) { }
        protected override bool ReleaseHandle() { return CloseHandle(handle); }
        internal static OperationJob Create()
        {
            var job = CreateJobObject(IntPtr.Zero, null);
            if (job.IsInvalid) throw new Win32Exception(Marshal.GetLastWin32Error());
            var limits = new Limits { Flags = 0x2000 }; // JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE
            if (SetInformationJobObject(job, 9, ref limits, Marshal.SizeOf(typeof(Limits)))) return job;
            int error = Marshal.GetLastWin32Error(); job.Dispose(); throw new Win32Exception(error);
        }
        internal void Attach(Process process)
        {
            if (AssignProcessToJobObject(this, process.Handle)) return;
            int error = Marshal.GetLastWin32Error();
            if (process.HasExited) return;
            process.Kill(); throw new Win32Exception(error, "The core could not be assigned to a cancellable Windows job.");
        }
        internal void Stop() { TerminateJobObject(this, 1223); }
    }
}
