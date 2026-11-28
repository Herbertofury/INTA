using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Controls;
using Microsoft.Win32;

namespace Inta.Desktop
{
    internal sealed partial class Workspace
    {
        private string pendingIso;
        private string[] Vm(params string[] arguments)
        {
            var command = new List<string> { "vm" }; command.AddRange(arguments);
            if (toolboxRoot != null) { command.Add("--root"); command.Add(toolboxRoot); }
            return command.ToArray();
        }
        private void ChooseLabIso()
        {
            var dialog = new OpenFileDialog { Title = "Choose a local boot ISO", Multiselect = false, CheckFileExists = true,
                RestoreDirectory = true, Filter = "ISO images (*.iso)|*.iso" };
            if (dialog.ShowDialog(window) != true) return;
            if (section == "Virtual machines") PreviewGuest(dialog.FileName, false);
            else { pendingIso = dialog.FileName; navigation.SelectedItem = "Virtual machines"; }
        }
        private void AddRepairLab()
        {
            if (rustConsoleAvailable)
                Action(pageActions, "Open Rust rescue console\u2026", delegate { RescueConsole(null); });
            if (repairLabAvailable)
                Action(pageActions, "Run isolated repair test", delegate {
                    RunOperation("Isolated repair test", new[] { "vm", "repair-test" }, false);
                });
        }
        private async Task LoadLab(CancellationToken token)
        {
            if (!vmAvailable)
            {
                if (!repairLabAvailable && !rustConsoleAvailable) {
                    Message("Virtual machine engines unavailable", "Select a managed toolbox with a verified runtime, or prepare the Linux/KVM rescue lab.");
                    Status("Virtual machine runtime unavailable."); return;
                }
                AddRepairLab();
                Title("Rust rescue lab");
                RustLabDetails();
                Status("Rust rescue lab actions available."); return;
            }
            Dictionary<string, object> backend;
            try { backend = Json.Object(await core.Read(Vm("status"), token)); }
            catch { if (!token.IsCancellationRequested) AddRepairLab(); throw; }
            if (token.IsCancellationRequested) return;
            AddRepairLab();
            Action(pageActions, "Runtime status", RefreshPage);
            Action(pageActions, "Lab sessions", LabSessions);
            bool ready = Equals(Json.Get(backend, "ready"), true);
            if (ready) Action(pageActions, "Choose boot ISO\u2026", ChooseLabIso);
            RenderReport("Portable VM runtime", backend);
            if (ready) {
                try {
                    var library = Json.Object(await core.Read(Vm("images"), token));
                    if (token.IsCancellationRequested) return;
                    var definitions = Json.Get(library, "definitions") as Dictionary<string, object>;
                    foreach (var image in Json.Array(Json.Get(library, "images")).Select(Json.Object)) {
                        string path = Json.Text(image, "path"), digest = Json.Text(image, "sha256");
                        string name = Json.Text(image, "name") + " (" + Json.Text(image, "architecture") + ")";
                        if (Equals(Json.Get(image, "canBoot"), true))
                            Action(pageActions, "Preview " + name, delegate { PreviewGuest(path, false, null, digest); });
                        else Note(name + " requires a matching guest architecture. Local ISO: " + path);
                        if (definitions != null && Json.Text(definitions, "targetRescueSHA256") == digest)
                            Action(pageActions, "Preview offline antivirus lab", delegate { PreviewGuest(path, false, null, digest, true); });
                    }
                    foreach (var issue in Json.Array(Json.Get(library, "issues"))) Note(Convert.ToString(issue));
                } catch (OperationCanceledException) { return; }
                catch (Exception e) { Note("Rescue library unavailable. " + e.GetBaseException().Message); }
            }
            if (rustConsoleAvailable || repairLabAvailable) RustLabDetails();
            Status(ready ? "Virtual machine runtime ready." : "Virtual machine runtime unavailable. Review the runtime status.");
            if (pendingIso != null) { string iso = pendingIso; pendingIso = null; if (ready) PreviewGuest(iso, false); }
        }
        private void RustLabDetails()
        {
            Title("Rust rescue environment");
            Note("Cloud Hypervisor uses Linux/KVM in the prepared Debian WSL2 environment on Windows. Each offline guest has 2 vCPUs and 2 GiB of memory. Windows runs under your current account, and the guest has its own root privileges.");
        }
        private async void PreviewGuest(string iso, bool nat, string baseImage = null, string digest = null, bool antivirus = false)
        {
            Cancel(ref detailRequest); detailRequest = new CancellationTokenSource(); var token = detailRequest.Token;
            Message("Boot plan", "Checking the image and virtual machine configuration\u2026");
            Status("Preparing the boot plan\u2026");
            try {
                var args = new List<string> { "plan", iso };
                if (digest != null) { args.Add("--sha256"); args.Add(digest); }
                if (antivirus) args.Add("--clamav");
                if (nat) args.Add("--nat");
                if (baseImage != null) { args.Add("--base"); args.Add(baseImage); }
                var plan = Json.Object(await core.Read(Vm(args.ToArray()), token));
                if (token.IsCancellationRequested) return;
                RenderReport("Boot plan", plan);
                if (antivirus) Note("The second CD contains the verified ClamAV definitions. In INTA Rescue, open INTA_CLAMAV and read README.txt for the scan command and definition date.");
                bool ready = Equals(Json.Get(plan, "ready"), true);
                if (ready) Action(detailActions, nat ? "Start guest with NAT\u2026" : "Start offline guest\u2026", delegate {
                    string mode = nat ? "The guest will have outbound network access through NAT." : "The guest will start with networking disabled.";
                    string confirm = "Boot this ISO in an INTA virtual machine?\n\n" + iso + "\n\n" + mode +
                        (baseImage == null ? "" : "\n\nRead-only base disk snapshot\n" + baseImage) +
                        "\n\nINTA will create a session directory and virtual disk. The guest can continue running after you close this window.";
                    if (MessageBox.Show(window, confirm, "Start INTA virtual machine", MessageBoxButton.OKCancel, MessageBoxImage.Question,
                            MessageBoxResult.Cancel) != MessageBoxResult.OK) return;
                    var command = new List<string> { "start", iso, "--approve" };
                    if (digest != null) { command.Add("--sha256"); command.Add(digest); }
                    if (antivirus) command.Add("--clamav");
                    if (nat) command.Add("--nat");
                    if (baseImage != null) { command.Add("--base"); command.Add(baseImage); }
                    else { command.Add("--disk-gib"); command.Add(Json.Text(plan, "diskGiB")); }
                    RunOperation("Virtual machine startup", Vm(command.ToArray()), true);
                });
                Action(detailActions, nat ? "Preview offline plan" : "Preview NAT plan", delegate { PreviewGuest(iso, !nat, baseImage, digest, antivirus); });
                Action(detailActions, "Choose base disk image\u2026", delegate {
                    var dialog = new OpenFileDialog { Title = "Choose a local raw disk image for a read-only base", CheckFileExists = true,
                        RestoreDirectory = true, Multiselect = false, Filter = "Raw disk images (*.img)|*.img" };
                    if (dialog.ShowDialog(window) == true) PreviewGuest(iso, nat, dialog.FileName, digest, antivirus);
                });
                if (baseImage != null) Action(detailActions, "Use blank disk", delegate { PreviewGuest(iso, nat, null, digest, antivirus); });
                Status(ready ? "Boot plan ready for review." : "Boot requirements are incomplete. Review the plan.");
            } catch (OperationCanceledException) { }
            catch (Exception e) {
                if (token.IsCancellationRequested) return;
                Message("Boot plan unavailable", e.GetBaseException().Message); Status("Unable to prepare the selected ISO.");
            }
        }
        private async void LabSessions()
        {
            Cancel(ref detailRequest); detailRequest = new CancellationTokenSource(); var token = detailRequest.Token;
            Message("Lab sessions", "Reading INTA-owned sessions\u2026"); Status("Reading lab sessions\u2026");
            try {
                var result = Json.Object(await core.Read(Vm("sessions"), token));
                if (token.IsCancellationRequested) return;
                var sessions = Json.Array(Json.Get(result, "sessions")).Select(Json.Object).ToList();
                ResetDetail(); Title("Lab sessions");
                if (sessions.Count == 0) Note("There are no sessions in the selected lab workspace.");
                foreach (var item in sessions)
                {
                    string id = Json.Text(item, "id");
                    Paragraph(id, "Text", 16, true);
                    foreach (var pair in item.Where(p => p.Key != "id")) Field(pair.Key, pair.Value);
                    if (id.Length == 0 || Json.Text(item, "state") != "running") continue;
                    var buttons = new WrapPanel(); detail.Children.Add(buttons);
                    Action(buttons, "Power off lab " + id + "\u2026", delegate {
                        string confirm = "Power off this INTA guest immediately?\n\n" + id +
                                "\n\nUnsaved work in this guest can be lost. Its session directory and virtual disk will be retained.";
                        if (MessageBox.Show(window, confirm, "Power off INTA guest", MessageBoxButton.OKCancel, MessageBoxImage.Warning,
                                MessageBoxResult.Cancel) == MessageBoxResult.OK)
                            RunOperation("Virtual machine power off", Vm("stop", id, "--approve"), true);
                    });
                }
                Status(sessions.Count + (sessions.Count == 1 ? " INTA lab session." : " INTA lab sessions."));
            } catch (OperationCanceledException) { }
            catch (Exception e) {
                if (token.IsCancellationRequested) return;
                Message("Lab sessions unavailable", e.GetBaseException().Message); Status("Unable to read lab sessions.");
            }
        }
    }
}
