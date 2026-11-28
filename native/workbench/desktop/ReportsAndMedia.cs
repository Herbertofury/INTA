using System;
using System.Collections;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Documents;
using Microsoft.Win32;

namespace Inta.Desktop
{
    internal sealed partial class Workspace
    {
        private void RenderReport(string title, object result)
        {
            ResetDetail(); Title(title);
            var map = result as Dictionary<string, object>;
            if (map != null) foreach (var pair in map) Field(pair.Key, pair.Value);
            else RenderValue(result, 0);
        }
        private void AddExport(string kind, object report)
        {
            string title = detail.Children.OfType<System.Windows.Controls.TextBlock>().Select(t => t.Text).FirstOrDefault() ?? Label(kind);
            Action(detailActions, "Export report\u2026", delegate { Export(kind, title, report, false); });
            Action(detailActions, "Save handoff\u2026", delegate { Export(kind, title, report, true); });
        }
        private async void Export(string kind, string title, object report, bool handoff)
        {
                var dialog = new SaveFileDialog { Title = handoff ? "Save technician handoff" : "Export local report", Filter = handoff ?
                    "Markdown handoff (*.md)|*.md" : "JSON report (*.json)|*.json|Text report (*.txt)|*.txt",
                    AddExtension = true, OverwritePrompt = true, FileName = "INTA-" + kind + "-" + DateTime.Now.ToString("yyyyMMdd-HHmmss") };
                if (dialog.ShowDialog(window) != true) return;
                activeExports++;
                try {
                    bool json = dialog.FilterIndex == 1;
                    string target = dialog.FileName;
                    await Task.Run(delegate {
                        string content = handoff ? Handoff(title, report) : json ? Json.Serializer().Serialize(report) : PlainReport(report, 0);
                        string temp = Path.Combine(Path.GetDirectoryName(target), ".inta-" + Guid.NewGuid().ToString("N") + ".tmp");
                        try {
                            File.WriteAllText(temp, content + Environment.NewLine, new UTF8Encoding(false));
                            if (File.Exists(target)) File.Replace(temp, target, null);
                            else File.Move(temp, target);
                        } finally { if (File.Exists(temp)) File.Delete(temp); }
                    });
                    Status("Report saved.");
                } catch (Exception e) { Status("Report export failed. " + e.GetBaseException().Message); }
                finally { activeExports--; }
        }
        private static string Handoff(string title, object report)
        {
            var document = new StringBuilder();
            document.AppendLine("# INTA technician handoff").AppendLine();
            document.AppendLine("## " + title).AppendLine();
            document.AppendLine("Exported " + DateTimeOffset.Now.ToString("yyyy-MM-dd HH:mm:ss zzz") + ".").AppendLine();
            var data = report as Dictionary<string, object>;
            if (data != null) {
                foreach (string key in new[] { "mode", "status", "observedAt" }) {
                    string text = Json.Text(data, key);
                    if (text.Length > 0) document.AppendLine("- " + Label(key) + " — " + text);
                }
                document.AppendLine();
            }
            document.AppendLine("## Recorded findings").AppendLine();
            foreach (string line in PlainReport(report, 0).Replace("\r", "").Split('\n')) document.Append("    ").AppendLine(line);
            document.AppendLine().AppendLine("## Acceptance check").AppendLine();
            document.AppendLine("Record the original symptom, the agreed success check, and its result in the approved private service record.");
            return document.ToString();
        }
        private static string PlainReport(object result, int level)
        {
            string pad = new String(' ', level * 2);
            var map = result as Dictionary<string, object>;
            if (map != null) return String.Join(Environment.NewLine, map.Select(p =>
                pad + Label(p.Key) + (p.Value is IDictionary || p.Value is IList ? Environment.NewLine + PlainReport(p.Value, level + 1) : "  " + Convert.ToString(p.Value))));
            var list = result as IList;
            if (list != null) return String.Join(Environment.NewLine + Environment.NewLine, list.Cast<object>().Select(p => PlainReport(p, level)));
            return pad + Convert.ToString(result);
        }
        private void RenderGuide(string markdown)
        {
            bool code = false;
            foreach (string raw in markdown.Replace("\r", "").Split('\n')) {
                string line = raw.Trim();
                if (line.StartsWith(new string((char)96, 3))) { code = !code; continue; }
                if (line.Length == 0 || line.StartsWith("# ")) continue;
                if (code) { detail.Children.Add(RowText(raw, false, false)); continue; }
                bool h = line.StartsWith("## ");
                if (h) line = line.Substring(3);
                var block = Paragraph("", h ? "Text" : "Muted", h ? 16 : 14, h);
                line = line.Replace("**", "");
                var matches = Regex.Matches(line, @"\[([^\]]+)\]\(([^)\s]+)\)");
                int start = 0;
                foreach (Match match in matches) {
                    AddMentions(block, line.Substring(start, match.Index - start));
                    string label = match.Groups[1].Value;
                    Uri uri;
                    if (Uri.TryCreate(match.Groups[2].Value, UriKind.Absolute, out uri) && (uri.Scheme == "https" || uri.Scheme == "http")) {
                        AddMentions(block, label, uri);
                    } else AddMentions(block, label);
                    start = match.Index + match.Length;
                }
                AddMentions(block, line.Substring(start));
            }
        }
        private async void OpenMedia()
        {
            Status("Checking the registered Python interpreter\u2026");
            try {
                string script = Path.Combine(baseDirectory, "media", "inta-media.py");
                if (!File.Exists(script)) throw new FileNotFoundException("The bundled Media Creator is missing. Repair this installation.");
                string interpreter = await Task.Run((Func<string>)PythonRuntime.FindTrustedWindowed);
                // Isolated Python omits the script directory. Add only the packaged media
                // directory after the standard library so its sibling modules can be imported.
                const string bootstrap = "import pathlib,runpy,sys; p=pathlib.Path(sys.argv[1]); sys.path.append(str(p.parent)); sys.argv=[str(p)]; runpy.run_path(str(p),run_name='__main__')";
                using (var media = Process.Start(new ProcessStartInfo { FileName = interpreter,
                        Arguments = "-I -S -B -c " + CoreClient.Quote(bootstrap) + " " + CoreClient.Quote(script),
                        WorkingDirectory = Path.GetDirectoryName(script), UseShellExecute = false })) {
                    var timer = Stopwatch.StartNew();
                    while (true) {
                        media.Refresh();
                        if (media.HasExited) throw new InvalidOperationException("Media Creator exited before opening. Verify the bundled media files and Python Tk installation.");
                        if (media.MainWindowHandle != IntPtr.Zero) break;
                        if (timer.Elapsed > TimeSpan.FromSeconds(12)) { Status("Media Creator started. Its window has not appeared yet."); return; }
                        await Task.Delay(140);
                    }
                }
                Status("Opened INTA Media Creator.");
            } catch (Exception e) { Status(e.GetBaseException().Message); }
        }
    }
}
