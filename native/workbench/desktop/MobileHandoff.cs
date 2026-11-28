using System;
using System.Diagnostics;
using System.IO;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Automation;
using System.Windows.Controls;
using System.Windows.Media;
using Microsoft.Win32;

namespace Inta.Desktop
{
    internal sealed partial class Workspace
    {
        private async void ReviewPhoneRepairDraft()
        {
            var file = new OpenFileDialog { Title = "Choose an unchanged INTA Mobile draft",
                Filter = "INTA Mobile repair draft (*.txt)|*.txt", CheckFileExists = true, Multiselect = false };
            if (file.ShowDialog(window) != true) return;
            Status("Reading the selected draft. Nothing will run.");
            try {
                var rows = await Task.Run(delegate { return MobileDraftReader.ReadFile(file.FileName); });
                if (lifetime.IsCancellationRequested) return;
                var panel = new StackPanel { Margin = new Thickness(18) };
                var dialog = new Window { Owner = window, Title = "Review phone repair draft",
                    Background = (Brush)window.FindResource("Surface"), Foreground = (Brush)window.FindResource("Text"),
                    Width = 540, Height = 440, WindowStartupLocation = WindowStartupLocation.CenterOwner, Content = panel };
                panel.Children.Add(new TextBlock {
                    Text = "This is an unapproved draft, not a command or a paired-device job. Closing leaves the original unchanged.",
                    TextWrapping = TextWrapping.Wrap, Margin = new Thickness(0, 0, 0, 12)
                });
                var close = new Button { Content = "Close review", IsCancel = true, Margin = new Thickness(0, 0, 0, 12) };
                close.Click += delegate { dialog.DialogResult = false; };
                panel.Children.Add(close);
                var list = new ListBox { MaxHeight = 260, Background = (Brush)window.FindResource("Canvas"),
                    Foreground = (Brush)window.FindResource("Text") };
                AutomationProperties.SetName(list, "Ordered unapproved repair steps");
                for (int i = 0; i < rows.Count; i++)
                    list.Items.Add(new TextBlock { Text = (i + 1) + ". " + rows[i].Kind + " \u00b7 " + rows[i].Label,
                        TextWrapping = TextWrapping.Wrap, MaxWidth = 450, Padding = new Thickness(4) });
                panel.Children.Add(list);
                dialog.ContentRendered += delegate { close.Focus(); };
                dialog.ShowDialog();
                Status("Draft review closed. No job was queued and no file changed.");
            } catch (Exception) {
                if (!lifetime.IsCancellationRequested) Status("Draft not opened. Select an unchanged local INTA Mobile export.");
            }
        }

        private void ExportMobileSnapshot()
        {
            var panel = new StackPanel { Margin = new Thickness(18) };
            var dialog = new Window { Owner = window, Title = "Mobile diagnostic handoff",
                Width = 440, SizeToContent = SizeToContent.Height, WindowStartupLocation = WindowStartupLocation.CenterOwner,
                Content = panel, ResizeMode = ResizeMode.NoResize };
            panel.Children.Add(new TextBlock { Text = "Open Diagnostic handoff on your phone and enter its pairing code. The encrypted file contains the capture time, memory use, AC power and battery level.",
                TextWrapping = TextWrapping.Wrap, Margin = new Thickness(0,0,0,12) });
            var code = new PasswordBox { MaxLength = 80, Padding = new Thickness(6) };
            AutomationProperties.SetName(code, "One-time phone pairing code");
            panel.Children.Add(code);
            var choose = new Button { Content = "Save encrypted snapshot…", IsDefault = true, Margin = new Thickness(0,12,0,0),
                Padding = new Thickness(8), IsEnabled = false };
            code.PasswordChanged += delegate { choose.IsEnabled = code.SecurePassword.Length >= 32; };
            choose.Click += delegate { dialog.DialogResult = true; };
            panel.Children.Add(choose);
            dialog.ContentRendered += delegate { code.Focus(); };
            if (dialog.ShowDialog() != true) { code.Clear(); return; }
            var file = new SaveFileDialog { Title = "Save a new encrypted snapshot", Filter = "INTA diagnostic snapshot (*.intadiag)|*.intadiag",
                AddExtension = true, DefaultExt = ".intadiag", FileName = "INTA-diagnostics-" + DateTime.Now.ToString("yyyyMMdd-HHmmss"),
                OverwritePrompt = true };
            try { if (file.ShowDialog(window) == true) WriteMobileSnapshot(file.FileName, code.Password); }
            finally { code.Clear(); }
        }

        private async void WriteMobileSnapshot(string path, string secret)
        {
            activeExports++;
            try {
                var start = new ProcessStartInfo { FileName = Path.Combine(baseDirectory, "inta-diagnostic-handoff.exe"),
                    Arguments = "export " + CoreClient.Quote(path) + " --approve", WorkingDirectory = baseDirectory,
                    UseShellExecute = false, CreateNoWindow = true, RedirectStandardInput = true, RedirectStandardOutput = true };
                using (var job = OperationJob.Create())
                using (var child = Process.Start(start)) {
                    job.Attach(child);
                    var read = child.StandardOutput.ReadToEndAsync();
                    child.StandardInput.WriteLine(secret);
                    child.StandardInput.Close();
                    secret = null;
                    if (!await Task.Run(delegate { return child.WaitForExit(15000); })) throw new TimeoutException("Snapshot export timed out.");
                    var response = Json.Object(Json.Serializer().DeserializeObject(await read));
                    if (child.ExitCode != 0 || !Object.Equals(Json.Get(response, "ok"), true))
                        throw new IOException(Json.Text(Json.Object(Json.Get(response, "error")), "message"));
                }
                Status("Encrypted snapshot saved. Transfer the file to your phone and open it in Diagnostic handoff.");
            } catch (Exception e) { Status("Mobile handoff failed. " + e.GetBaseException().Message); }
            finally { activeExports--; }
        }
    }
}
