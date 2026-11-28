using System;
using System.Diagnostics;
using System.IO;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Automation;
using System.Windows.Controls;
using Microsoft.Win32;

namespace Inta.Desktop
{
    internal sealed partial class Workspace
    {
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
