using System;
using System.IO;
using System.Security;

namespace Inta.Desktop
{
    internal static class WorkbenchStateDirectory
    {
        internal static string Resolve(string payloadDirectory)
        {
            return Resolve(payloadDirectory, Environment.GetEnvironmentVariable("INTA_DATA_DIR"),
                Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData));
        }

        internal static string Resolve(string payloadDirectory, string configured, string localApplicationData)
        {
            try
            {
                string payload = FullDirectory(payloadDirectory);
                if (payload == null) return null;
                if (String.IsNullOrWhiteSpace(configured))
                {
                    var package = new DirectoryInfo(payload);
                    var parent = package.Parent;
                    // The official portable launcher supplies this same outer state.
                    // Infer it for a direct smoke-test launch of the packaged executable.
                    if (parent != null && package.Name.Equals("base", StringComparison.OrdinalIgnoreCase)
                        && File.Exists(Path.Combine(parent.FullName, "inta-portable.marker")))
                        configured = Path.Combine(parent.FullName, "state");
                    else if (!String.IsNullOrWhiteSpace(localApplicationData))
                        configured = Path.Combine(localApplicationData, "INTA", "state");
                }
                string state = FullDirectory(configured);
                if (state == null || Within(state, payload)) return null;
                // Check each ancestor, even when the immediate state folder does not yet exist.
                // This catches redirects and aliases of signed packages, including other revisions.
                for (var folder = new DirectoryInfo(state); folder != null; folder = folder.Parent)
                {
                    var attributes = Attributes(folder.FullName);
                    if (attributes.HasValue && ((attributes.Value & FileAttributes.ReparsePoint) != 0
                        || (attributes.Value & FileAttributes.Directory) == 0)) return null;
                    if (attributes.HasValue && Attributes(Path.Combine(folder.FullName, "manifest.json")).HasValue
                        && Attributes(Path.Combine(folder.FullName, "manifest.p7s")).HasValue) return null;
                }
                return state;
            }
            catch (ArgumentException) { return null; }
            catch (NotSupportedException) { return null; }
            catch (IOException) { return null; }
            catch (UnauthorizedAccessException) { return null; }
            catch (SecurityException) { return null; }
        }

        private static string FullDirectory(string value)
        {
            if (String.IsNullOrWhiteSpace(value) || !Path.IsPathRooted(value)) return null;
            if (Path.DirectorySeparatorChar == '\\')
            {
                value = value.Replace('/', '\\');
                if (value.StartsWith(@"\\?\", StringComparison.Ordinal) || value.StartsWith(@"\\.\", StringComparison.Ordinal))
                    return null;
                if (value.StartsWith(@"\\", StringComparison.Ordinal))
                {
                    var parts = value.Split('\\');
                    if (parts.Length < 4 || parts[2].Length == 0 || parts[3].Length == 0 || value.Contains(":"))
                        return null;
                }
                else if (value.Length < 3 || value[1] != ':' || value[2] != '\\' || value.IndexOf(':', 2) >= 0)
                    return null;
            }
            string absolute = Path.GetFullPath(value);
            string root = Path.GetPathRoot(absolute);
            return absolute.Length == root.Length ? root : absolute.TrimEnd(Path.DirectorySeparatorChar, Path.AltDirectorySeparatorChar);
        }

        private static bool Within(string candidate, string directory)
        {
            return String.Equals(candidate, directory, StringComparison.OrdinalIgnoreCase) ||
                candidate.StartsWith(directory.TrimEnd(Path.DirectorySeparatorChar, Path.AltDirectorySeparatorChar)
                    + Path.DirectorySeparatorChar, StringComparison.OrdinalIgnoreCase);
        }

        private static FileAttributes? Attributes(string path)
        {
            try { return File.GetAttributes(path); }
            catch (FileNotFoundException) { return null; }
            catch (DirectoryNotFoundException) { return null; }
        }
    }
}
