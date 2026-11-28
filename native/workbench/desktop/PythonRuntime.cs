using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Runtime.InteropServices;
using System.Security.Cryptography;
using System.Security.Cryptography.X509Certificates;
using Microsoft.Win32;

namespace Inta.Desktop
{
    internal static class PythonRuntime
    {
        [StructLayout(LayoutKind.Sequential, CharSet = CharSet.Unicode)]
        private struct FileInfo { public uint Size; public string Path; public IntPtr Handle, KnownSubject; }
        [StructLayout(LayoutKind.Sequential)]
        private struct TrustData
        {
            public uint Size;
            public IntPtr PolicyData, SipData;
            public uint UIChoice, Revocation, UnionChoice;
            public IntPtr File;
            public uint StateAction;
            public IntPtr StateData, UrlReference;
            public uint Flags, UIContext;
            public IntPtr SignatureSettings;
        }
        [DllImport("wintrust.dll", ExactSpelling = true)]
        private static extern int WinVerifyTrust(IntPtr owner, ref Guid policy, ref TrustData data);

        internal static bool TrustedPublisher(string path, string publisher)
        {
            var file = new FileInfo { Size = (uint)Marshal.SizeOf(typeof(FileInfo)), Path = path };
            var data = new TrustData { Size = (uint)Marshal.SizeOf(typeof(TrustData)), UIChoice = 2, UnionChoice = 1,
                // Cache-only certificate resolution keeps the optional media launcher usable offline.
                Flags = 0x1000, File = Marshal.AllocHGlobal(Marshal.SizeOf(typeof(FileInfo))) };
            Guid policy = new Guid("00AAC56B-CD44-11d0-8CC2-00C04FC295EE");
            try {
                Marshal.StructureToPtr(file, data.File, false);
                if (WinVerifyTrust(new IntPtr(-1), ref policy, ref data) != 0) return false;
                using (var signer = new X509Certificate2(X509Certificate.CreateFromSignedFile(path)))
                    return String.Equals(signer.GetNameInfo(X509NameType.SimpleName, false), publisher, StringComparison.Ordinal);
            } catch (CryptographicException) { return false; }
            finally { Marshal.DestroyStructure(data.File, typeof(FileInfo)); Marshal.FreeHGlobal(data.File); }
        }
        internal static string FindTrustedWindowed()
        {
            var candidates = new List<KeyValuePair<Version, string>>();
            foreach (RegistryHive hive in new[] { RegistryHive.CurrentUser, RegistryHive.LocalMachine })
                using (var root = RegistryKey.OpenBaseKey(hive, RegistryView.Registry64))
                using (var versions = root.OpenSubKey(@"Software\Python\PythonCore"))
                {
                    if (versions == null) continue;
                    foreach (string key in versions.GetSubKeyNames())
                    {
                        Version version;
                        if (!Version.TryParse(key, out version) || version.Major != 3 || version.Minor < 11) continue;
                        using (var install = versions.OpenSubKey(key + @"\InstallPath"))
                        {
                            string path = install == null ? null : install.GetValue("WindowedExecutablePath") as string;
                            if (!String.IsNullOrWhiteSpace(path) && !path.StartsWith(@"\\") && Path.IsPathRooted(path) &&
                                Path.GetFileName(path).Equals("pythonw.exe", StringComparison.OrdinalIgnoreCase) && File.Exists(path))
                                candidates.Add(new KeyValuePair<Version, string>(version, path));
                        }
                    }
                }
            foreach (var candidate in candidates.OrderByDescending(v => v.Key))
                if (TrustedPublisher(candidate.Value, "Python Software Foundation")) return candidate.Value;
            throw new InvalidOperationException("Install a signed, 64-bit Python.org Python 3.11 or later with Tk to open Media Creator.");
        }
    }
}
