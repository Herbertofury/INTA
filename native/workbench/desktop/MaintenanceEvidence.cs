using System;
using System.Collections;
using System.Collections.Generic;
using System.Globalization;

namespace Inta.Desktop
{
    internal sealed class MaintenanceEvidence
    {
        internal readonly string SelectedFolder;
        internal readonly Dictionary<string, object> Report;
        internal static readonly string[] DiskKeys = { "fileCount", "logicalBytes", "sizeByCategory", "topDirectories", "directoryGroupCount" };
        internal const string LargestKey = "largest";
        internal const string DuplicatesKey = "duplicateGroups";

        private MaintenanceEvidence(string folder, Dictionary<string, object> report)
        { SelectedFolder = folder; Report = report; }

        internal static MaintenanceEvidence Read(string selectedFolder, Dictionary<string, object> report)
        {
            object kind, schema, largest, duplicates;
            if (String.IsNullOrEmpty(selectedFolder) || report == null ||
                !report.TryGetValue("kind", out kind) || !String.Equals(kind as string, "space-analysis", StringComparison.Ordinal) ||
                !report.TryGetValue("schemaVersion", out schema) || !(schema is int || schema is long) ||
                Convert.ToInt64(schema, CultureInfo.InvariantCulture) != 1L ||
                !report.TryGetValue(LargestKey, out largest) || !(largest is IList) ||
                !report.TryGetValue(DuplicatesKey, out duplicates) || !(duplicates is IList))
                throw new System.IO.InvalidDataException("The local core did not return a supported space analysis report.");
            return new MaintenanceEvidence(selectedFolder, report);
        }

        internal object Get(string key)
        {
            object value;
            return Report.TryGetValue(key, out value) ? value : null;
        }
    }
}
