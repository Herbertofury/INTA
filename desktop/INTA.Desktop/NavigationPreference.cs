namespace INTA.Desktop;

internal static class NavigationPreference
{
    internal static MaintenanceView Read(string applicationDirectory) =>
        PortableMaintenanceRoute.Read(applicationDirectory) ?? MaintenanceCatalog.Default;

    internal static bool Save(string applicationDirectory, MaintenanceView view) =>
        PortableMaintenanceRoute.Save(applicationDirectory, view);
}
