using System.Text;

namespace INTA.Desktop;

internal static class NavigationPreference
{
    private const string PreferenceName = "maintenance-view.v1";
    private const int MaxBytes = 32;

    internal static MaintenanceView Read(string applicationDirectory)
    {
        string state = Path.Combine(applicationDirectory, "state");
        string path = Path.Combine(state, PreferenceName);
        try
        {
            if (!Redirected(state) && !Redirected(path))
            {
                using var file = new FileStream(path, FileMode.Open, FileAccess.Read, FileShare.Read | FileShare.Delete);
                if (file.Length is > 0 and <= MaxBytes)
                {
                    Span<byte> bytes = stackalloc byte[(int)file.Length];
                    file.ReadExactly(bytes);
                    return MaintenanceCatalog.Restore(Encoding.ASCII.GetString(bytes));
                }
            }
        }
        catch (Exception ex) when (Expected(ex)) { }
        return MaintenanceCatalog.Default;
    }

    internal static bool Save(string applicationDirectory, MaintenanceView view)
    {
        string state = Path.Combine(applicationDirectory, "state");
        string path = Path.Combine(state, PreferenceName);
        string? temporary = null;
        try
        {
            if (Redirected(state) || Redirected(path)) return false;
            Directory.CreateDirectory(state);
            temporary = Path.Combine(state, "maintenance-view-" + Guid.NewGuid().ToString("N") + ".tmp");
            using (var output = new FileStream(temporary, FileMode.CreateNew, FileAccess.Write, FileShare.None))
                output.Write(Encoding.ASCII.GetBytes(MaintenanceCatalog.Get(view).Key));
            if (Redirected(state) || Redirected(path)) return false;
            File.Move(temporary, path, overwrite: true);
            return true;
        }
        catch (Exception ex) when (Expected(ex)) { return false; }
        finally
        {
            if (temporary is not null)
                try { if (!Redirected(state)) File.Delete(temporary); }
                catch (Exception ex) when (Expected(ex)) { }
        }
    }

    private static bool Expected(Exception ex) => ex is IOException or UnauthorizedAccessException or System.Security.SecurityException;

    private static bool Redirected(string path)
    {
        try { return (File.GetAttributes(path) & FileAttributes.ReparsePoint) != 0; }
        catch (FileNotFoundException) { return false; }
        catch (DirectoryNotFoundException) { return false; }
    }
}
