using System;
using System.Collections.Generic;
using System.Globalization;
using System.IO;
using System.Text;

namespace Inta.Desktop
{
    // Parses only the reviewed Android export. It has no dispatch, rewrite or
    // storage API and deliberately never translates a label into arguments.
    internal static class MobileDraftReader
    {
        private const string Header = "INTA repair plan draft\nNot approved. Nothing in this document runs or grants access.\n\n";
        private const int MaxBytes = 16384;
        private static readonly UTF8Encoding StrictUtf8 = new UTF8Encoding(false, true);
        private static readonly string[] Kinds = {
            "Remove an app", "Apply a repair", "Use a reviewed PowerShell recipe",
            "Run a malware scan", "Apply an optimization"
        };

        internal sealed class Row
        {
            internal readonly string Kind, Label;
            internal Row(string kind, string label) { Kind = kind; Label = label; }
        }

        internal static IList<Row> ReadFile(string path)
        {
            if (String.IsNullOrEmpty(path) || !Path.IsPathRooted(path)
                || path.Length < 3 || path[1] != ':' || path[2] != '\\'
                || !Char.IsLetter(path[0]) || path.IndexOf(':', 2) >= 0) throw Invalid();
            var directory = Path.GetPathRoot(path);
            if (new DriveInfo(directory).DriveType != DriveType.Fixed) throw Invalid();
            for (string p = Path.GetFullPath(path); !String.Equals(p, directory, StringComparison.OrdinalIgnoreCase); p = Path.GetDirectoryName(p)) {
                if (String.IsNullOrEmpty(p) || (File.GetAttributes(p) & FileAttributes.ReparsePoint) != 0) throw Invalid();
            }
            using (var file = new FileStream(path, FileMode.Open, FileAccess.Read, FileShare.Read))
            {
                var bytes = new byte[MaxBytes + 1];
                int used = 0, size;
                while (used < bytes.Length && (size = file.Read(bytes, used, bytes.Length - used)) != 0) used += size;
                if (used > MaxBytes) throw Invalid();
                return Parse(StrictUtf8.GetString(bytes, 0, used));
            }
        }

        internal static IList<Row> Parse(string value)
        {
            if (value == null || value.Length > MaxBytes || !value.StartsWith(Header, StringComparison.Ordinal)
                || !value.EndsWith("\n", StringComparison.Ordinal)) throw Invalid();
            var rows = value.Substring(Header.Length).Split('\n');
            if (rows.Length < 2 || rows.Length > 25 || rows[rows.Length - 1].Length != 0) throw Invalid();
            var values = new List<Row>(rows.Length - 1);
            for (int i = 0; i < rows.Length - 1; i++)
            {
                int index = i + 1;
                string prefix = index.ToString(CultureInfo.InvariantCulture) + ". ";
                if (!rows[i].StartsWith(prefix, StringComparison.Ordinal)) throw Invalid();
                int separator = rows[i].IndexOf(" \u2014 ", prefix.Length, StringComparison.Ordinal);
                if (separator < 0) throw Invalid();
                string kind = rows[i].Substring(prefix.Length, separator - prefix.Length);
                if (Array.IndexOf(Kinds, kind) < 0) throw Invalid();
                string label = rows[i].Substring(separator + 3);
                if (label.Length < 1 || label.Length > 120 || label[0] <= 0x20 || label[label.Length - 1] <= 0x20) throw Invalid();
                for (int j = 0; j < label.Length; j++)
                {
                    var category = CharUnicodeInfo.GetUnicodeCategory(label, j);
                    if (category == UnicodeCategory.Control || category == UnicodeCategory.Format ||
                        category == UnicodeCategory.LineSeparator || category == UnicodeCategory.ParagraphSeparator ||
                        category == UnicodeCategory.Surrogate) throw Invalid();
                    if (Char.IsHighSurrogate(label[j])) j++;
                }
                values.Add(new Row(kind, label));
            }
            return values.AsReadOnly();
        }

        private static InvalidDataException Invalid() { return new InvalidDataException("Not an unchanged INTA Mobile repair draft."); }
    }
}
