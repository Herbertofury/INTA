using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Text.RegularExpressions;
using System.Threading.Tasks;
using System.Windows;
using Microsoft.Win32;

namespace Inta.Desktop
{
    internal sealed partial class Workspace
    {
        private void RescueConsole(string image)
        {
            Cancel(ref detailRequest);
            Message("Rust rescue console", "Open an offline root shell with 2 vCPUs and 2 GiB of memory through Linux/KVM in the prepared Debian WSL2 environment. Windows runs under your current account. The optional base appears as /dev/vdc using a disposable private disk copy. Type exit in the guest to power it off.");
            Field("Base disk image", image ?? "No additional disk");
            Action(detailActions, "Choose raw disk image\u2026", delegate {
                var file = new OpenFileDialog { Title = "Choose a local raw disk image for the Rust guest", Filter = "Raw disk images (*.img)|*.img",
                    CheckFileExists = true, Multiselect = false, RestoreDirectory = true };
                if (file.ShowDialog(window) != true) return;
                var selected = new FileInfo(file.FileName);
                if (selected.FullName.StartsWith(@"\\") || !selected.Extension.Equals(".img", StringComparison.OrdinalIgnoreCase) ||
                        (selected.Attributes & (FileAttributes.Directory | FileAttributes.ReparsePoint)) != 0) {
                    Status("Choose a regular local .img file."); return;
                }
                RescueConsole(selected.FullName);
            });
            if (image != null) Action(detailActions, "Remove base image", delegate { RescueConsole(null); });
            Action(detailActions, "Open console window\u2026", delegate {
                string text = "Start an offline Rust rescue guest in a separate console?\n\n" +
                    (image == null ? "No additional disk is attached." : "Read-only source image\n" + image) +
                    "\n\nThe shell has root privileges in the guest. Writes to the optional base go to its disposable private disk copy.";
                if (MessageBox.Show(window, text, "Open Rust rescue guest", MessageBoxButton.OKCancel, MessageBoxImage.Question,
                        MessageBoxResult.Cancel) == MessageBoxResult.OK) OpenRustConsole(image);
            });
            Status("Review the guest configuration before opening the console.");
        }
        private async void OpenRustConsole(string image)
        {
            LockNavigation(true); detailActions.IsEnabled = false;
            Status("Verifying the rescue console launcher\u2026");
            try {
                var command = new List<string> { "vm", "console", "--approve" };
                if (image != null) { command.Add("--base"); command.Add(image); }
                if (toolboxRoot != null) { command.Add("--root"); command.Add(toolboxRoot); }
                await Task.Run(delegate { LaunchConsole(command); });
                Status("Opened Rust rescue console. Type exit in the guest to power it off.");
            } catch (Exception e) { Status("Unable to open the rescue console. " + e.GetBaseException().Message); }
            finally { detailActions.IsEnabled = true; LockNavigation(false); }
        }
        private void LaunchConsole(List<string> command)
        {
            string cli = Path.Combine(baseDirectory, "inta.exe");
            string manifest = Path.Combine(baseDirectory, "manifest.json");
            if (!File.Exists(manifest)) throw new InvalidDataException("Open the installed or verified portable Desktop to launch the interactive guest.");
            var doc = Json.Object(Json.Serializer().DeserializeObject(File.ReadAllText(manifest)));
            string digest = Json.Text(doc, "sourceDigest");
            if (!Regex.IsMatch(digest, @"\A[a-f0-9]{64}\z")) throw new InvalidDataException("The payload manifest has no valid source digest.");
            string home = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData), @"Programs\INTA Desktop");
            var payload = new DirectoryInfo(baseDirectory);
            if (payload.Parent != null && payload.Name.Equals("base", StringComparison.OrdinalIgnoreCase)
                    && File.Exists(Path.Combine(payload.Parent.FullName, "inta-portable.marker"))) home = payload.Parent.FullName;
            string verifier = Path.Combine(home, "INTA Verify.exe");
            if (!File.Exists(verifier)) throw new InvalidDataException("The installed package verifier is unavailable.");
            // Keep the same binary locked against replacement from verification through launch.
            using (File.Open(cli, FileMode.Open, FileAccess.Read, FileShare.Read)) {
                using (var check = Process.Start(new ProcessStartInfo { FileName = verifier, WorkingDirectory = home, UseShellExecute = false,
                        Arguments = CoreClient.Quote(payload.FullName.TrimEnd(Path.DirectorySeparatorChar)) + " " + CoreClient.Quote(digest), CreateNoWindow = true })) {
                    if (!check.WaitForExit(90000)) { check.Kill(); throw new TimeoutException("Application payload verification timed out."); }
                    if (check.ExitCode != 0) throw new InvalidDataException("The application's signed payload failed verification.");
                }
                using (Process.Start(new ProcessStartInfo { FileName = cli, WorkingDirectory = payload.FullName, UseShellExecute = true,
                        WindowStyle = ProcessWindowStyle.Normal, Arguments = String.Join(" ", command.Select(CoreClient.Quote)) })) { }
            }
        }
    }
}
