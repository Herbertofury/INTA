using System;
using System.Collections.Generic;
using System.Linq;

namespace INTA.Desktop
{
    internal enum MaintenanceView
    {
        DeepCleanup, LargeFileCleanup, DuplicateCleanup, DiskSpaceAnalysis, PrivacyCleanup,
        AppUninstaller, StartupItems, SystemOptimization, SystemMaintenance
    }

    internal sealed class MaintenanceIdentity
    {
        public MaintenanceView Id { get; private set; }
        public string Key { get; private set; }
        public string Group { get; private set; }
        public string Name { get; private set; }
        internal MaintenanceIdentity(MaintenanceView id, string key, string group, string name)
        {
            Id = id; Key = key; Group = group; Name = name;
        }
    }

    internal static class MaintenanceTaxonomy
    {
        // Shared by the released WPF Workbench and the separate WinForms preview.
        internal static readonly IList<MaintenanceIdentity> Rows = Array.AsReadOnly(new[] {
            new MaintenanceIdentity(MaintenanceView.DeepCleanup, "deep-cleanup", "Storage", "Deep Cleanup"),
            new MaintenanceIdentity(MaintenanceView.LargeFileCleanup, "large-file-cleanup", "Storage", "Large File Cleanup"),
            new MaintenanceIdentity(MaintenanceView.DuplicateCleanup, "duplicate-cleanup", "Storage", "Duplicate Cleanup"),
            new MaintenanceIdentity(MaintenanceView.DiskSpaceAnalysis, "disk-space-analysis", "Storage", "Disk Space Analysis"),
            new MaintenanceIdentity(MaintenanceView.PrivacyCleanup, "privacy-cleanup", "Privacy & Security", "Privacy Cleanup"),
            new MaintenanceIdentity(MaintenanceView.AppUninstaller, "app-uninstaller", "System Tools", "App Uninstaller"),
            new MaintenanceIdentity(MaintenanceView.StartupItems, "startup-items", "System Tools", "Startup Items"),
            new MaintenanceIdentity(MaintenanceView.SystemOptimization, "system-optimization", "System Tools", "System Optimization"),
            new MaintenanceIdentity(MaintenanceView.SystemMaintenance, "system-maintenance", "System Tools", "System Maintenance")
        });

        internal static MaintenanceIdentity Get(MaintenanceView view) { return Rows.First(row => row.Id == view); }
        // A value-type optional result expresses the same contract to the
        // Framework compiler and the nullable-enabled modern preview.
        internal static MaintenanceView? FindView(string name)
        {
            var row = Rows.FirstOrDefault(identity => identity.Name == name);
            return row == null ? (MaintenanceView?)null : row.Id;
        }
        internal static string[] Terms(string query)
        {
            return (query ?? String.Empty).Split(default(char[]), StringSplitOptions.RemoveEmptyEntries);
        }
        internal static bool Matches(string[] terms, params string[] fields)
        {
            return terms.All(term => fields.Any(text => text != null && text.IndexOf(term, StringComparison.OrdinalIgnoreCase) >= 0));
        }
    }
}
