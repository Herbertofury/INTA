using System;
using System.IO;
using System.Linq;
using System.Text;

namespace INTA.Desktop
{
    internal static class PortableMaintenanceRoute
    {
        private const string FileName = "maintenance-view.v1";
        private const int MaxBytes = 32;

        internal static MaintenanceView? Read(string applicationDirectory)
        {
            string state = Path.Combine(applicationDirectory, "state");
            string path = Path.Combine(state, FileName);
            try
            {
                if (Redirected(state) || Redirected(path)) return null;
                using (var file = new FileStream(path, FileMode.Open, FileAccess.Read, FileShare.Read | FileShare.Delete))
                {
                    if (file.Length < 1 || file.Length > MaxBytes) return null;
                    var bytes = new byte[(int)file.Length];
                    int total = 0, read;
                    while (total < bytes.Length && (read = file.Read(bytes, total, bytes.Length - total)) != 0) total += read;
                    if (total != bytes.Length) return null;
                    var key = Encoding.ASCII.GetString(bytes);
                    var row = MaintenanceTaxonomy.Rows.FirstOrDefault(v => v.Key == key);
                    return row == null ? (MaintenanceView?)null : row.Id;
                }
            }
            catch (IOException) { return null; }
            catch (UnauthorizedAccessException) { return null; }
            catch (System.Security.SecurityException) { return null; }
        }

        internal static bool Save(string applicationDirectory, MaintenanceView? view)
        {
            string state = Path.Combine(applicationDirectory, "state");
            string path = Path.Combine(state, FileName);
            string temporary = String.Empty;
            try
            {
                if (Redirected(state) || Redirected(path)) return false;
                Directory.CreateDirectory(state);
                temporary = Path.Combine(state, "maintenance-view-" + Guid.NewGuid().ToString("N") + ".tmp");
                using (var output = new FileStream(temporary, FileMode.CreateNew, FileAccess.Write, FileShare.None))
                {
                    var marker = Encoding.ASCII.GetBytes(view.HasValue ? MaintenanceTaxonomy.Get(view.Value).Key : "directory");
                    output.Write(marker, 0, marker.Length);
                }
                if (Redirected(state) || Redirected(path)) return false;
                if (File.Exists(path)) File.Replace(temporary, path, null);
                else File.Move(temporary, path);
                return true;
            }
            catch (IOException) { return false; }
            catch (UnauthorizedAccessException) { return false; }
            catch (System.Security.SecurityException) { return false; }
            finally
            {
                if (temporary.Length != 0)
                    try { if (!Redirected(state)) File.Delete(temporary); }
                    catch (IOException) { }
                    catch (UnauthorizedAccessException) { }
                    catch (System.Security.SecurityException) { }
            }
        }

        private static bool Redirected(string path)
        {
            try { return (File.GetAttributes(path) & FileAttributes.ReparsePoint) != 0; }
            catch (FileNotFoundException) { return false; }
            catch (DirectoryNotFoundException) { return false; }
        }
    }
}
