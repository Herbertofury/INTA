using System;
using System.Collections;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using System.Windows;
using Microsoft.Win32;

namespace Inta.Desktop
{
    internal sealed partial class Workspace
    {
        private static readonly Entry[] Diagnostics = {
            Diagnostic("overview", "System overview", "Windows build, processor and memory"),
            Diagnostic("storage", "Volume capacity", "Filesystem and free space on local volumes"),
            Diagnostic("space", "Folder space analysis", "Largest files and duplicate-content candidates"),
            Diagnostic("security", "Endpoint protection", "Defender and Windows Firewall status"),
            Diagnostic("network", "Network interfaces", "Adapter state and link speed"),
            Diagnostic("startup", "Startup inventory", "Registered startup entries grouped by source"),
            Diagnostic("drivers", "Driver health", "Installed drivers and devices reporting problems"),
            Diagnostic("events", "Recent system errors", "Critical and error event counts from the past 24 hours"),
            Diagnostic("performance", "Resource usage", "Current processor, memory and process load"),
            Diagnostic("applications", "Installed applications", "Software inventory from Windows installation records"),
            Diagnostic("updates", "Windows servicing", "Servicing state and pending restart indicators"),
            Diagnostic("power", "Power and battery", "Active power scheme and reported battery state"),
            Diagnostic("printing", "Printing and spooler", "Printer state and the local Print Spooler service"),
            Diagnostic("directory", "Domain membership", "Local Windows domain membership and directory health"),
            Diagnostic("recovery", "Boot and recovery tools", "Boot configuration tools and local recovery services"),
            Diagnostic("disks", "Physical disk health", "Storage device health and physical disk state"),
            Diagnostic("models", "Local AI model inventory", "On-disk model counts and sizes by provider")
        };
        private static Entry Diagnostic(string id, string title, string description)
        {
            return new Entry { Kind = "diagnostic", Id = id, Title = title, Subtitle = description };
        }
        private string[] Toolbox(params string[] args)
        {
            var command = new List<string> { "toolbox" }; command.AddRange(args);
            if (toolboxRoot != null) { command.Add("--root"); command.Add(toolboxRoot); }
            return command.ToArray();
        }
        private async void SelectToolbox()
        {
            string path = ChooseFolder("Select the managed INTA Toolbox root folder.");
            if (path == null) return;
            Cancel(ref detailRequest); detailRequest = new CancellationTokenSource(); var token = detailRequest.Token;
            try {
                await core.Read(new[] { "toolbox", "use", path }, token);
                if (token.IsCancellationRequested) return;
                // Retain an explicit session override only when an environment hint would
                // otherwise outrank the user's persisted marker-based selection.
                toolboxRoot = String.IsNullOrWhiteSpace(Environment.GetEnvironmentVariable("INTA_TOOLBOX")) ? null : path;
                RefreshPage();
            } catch (OperationCanceledException) { }
            catch (Exception e) { if (!token.IsCancellationRequested) Status("Unable to select that toolbox. " + e.GetBaseException().Message); }
        }
        private sealed class DialogOwner : System.Windows.Forms.IWin32Window
        {
            public IntPtr Handle { get; private set; }
            internal DialogOwner(Window owner) { Handle = new System.Windows.Interop.WindowInteropHelper(owner).Handle; }
        }
        private string ChooseFolder(string purpose)
        {
            using (var dialog = new System.Windows.Forms.FolderBrowserDialog { Description = purpose, ShowNewFolderButton = false })
                return dialog.ShowDialog(new DialogOwner(window)) == System.Windows.Forms.DialogResult.OK ? dialog.SelectedPath : null;
        }
        private async Task LoadOperationPage(CancellationToken token)
        {
            if (section == "Offline references") { await SearchReferences(token); return; }
            if (section == "Local AI") { await LoadTriage(token); return; }
            if (section == "Diagnostics") {
                Action(pageActions, "System inventory", delegate { OpenFeature("inventory"); });
                Action(pageActions, "Capability diagnostics", delegate { OpenFeature("diagnostics"); });
                Action(pageActions, "Mobile handoff", ExportMobileSnapshot);
                Action(pageActions, "Review cache files\u2026", OpenCleanupReview);
                Action(pageActions, "Manage sign-in entries\u2026", OpenStartupReview);
                SetRows(Diagnostics); return;
            }
            if (section == "Virtual machines") { await LoadLab(token); return; }
            if (section == "Scenarios")
            {
                Action(pageActions, "Saved checkpoints", ShowCheckpoints);
                object result = await core.Read(new[] { "scenario", "list" }, token);
                if (token.IsCancellationRequested) return;
                SetRows(Json.Array(result).Select(Json.Object).Select(d => new Entry { Kind = "scenario", Id = Json.Text(d, "id"),
                    Title = Json.Text(d, "title"), Subtitle = String.Join("  \u00b7  ", Json.Array(Json.Get(d, "stages")).Select(v => Convert.ToString(v))), Data = d }));
                if (listing.Count == 0) { Message("No scenarios available", "This core returned an empty scenario library."); Status("Scenario library loaded."); }
                return;
            }
            if (section == "Managed toolbox")
            {
                Action(pageActions, "CLI Forge", delegate { OpenFeature("forge"); });
                Action(pageActions, "Choose toolbox folder", SelectToolbox);
                if (toolboxRoot != null) Action(pageActions, "Use environment toolbox", delegate { toolboxRoot = null; RefreshPage(); });
                Action(pageActions, "Refresh", RefreshPage);
                var result = Json.Object(await core.Read(Toolbox("list"), token));
                if (token.IsCancellationRequested) return;
                var packages = Json.Array(Json.Get(result, "packages")).Select(Json.Object).ToList();
                if (packages.Any(p => Json.Text(p, "id") == "7zip" && Json.Text(p, "state") == "present"))
                {
                    Action(pageActions, "Test .7z archive\u2026", delegate {
                        var file = new OpenFileDialog { Title = "Select a .7z archive for an integrity test", CheckFileExists = true,
                            RestoreDirectory = true, Filter = "7-Zip archives (*.7z)|*.7z" };
                        if (file.ShowDialog(window) == true)
                            RunOperation("Archive integrity", Toolbox("archive", file.FileName), false);
                    });
                    Action(pageActions, "Create verified .7z backup\u2026", CreateBackup);
                }
                SetRows(packages.Select(d => new Entry { Kind = "package", Id = Json.Text(d, "id"),
                    Title = Json.Text(d, "name"), Subtitle = Json.Text(d, "version") + "  \u00b7  " + Json.Text(d, "state"), Data = d }));
                if (listing.Count == 0) { Message("No managed packages", "The selected toolbox contains no packages in the managed index."); Status("Toolbox index loaded."); }
                return;
            }
            if (section == "Windows settings")
            {
                Action(pageActions, "Setting profiles", ShowProfiles);
                var settings = await core.Read(new[] { "tweak", "list" }, token);
                if (token.IsCancellationRequested) return;
                SetRows(Json.Array(settings).Select(Json.Object).Select(d => new Entry { Kind = "setting", Id = Json.Text(d, "id"),
                    Title = Json.Text(d, "setting"), Subtitle = Json.Text(d, "effect"), Data = d }));
                if (listing.Count == 0) { Message("No Windows settings available", "This core returned an empty list of supported settings."); Status("Settings loaded."); }
                return;
            }
            if (section == "Rescue media")
            {
                Title("INTA Media Creator");
                Note("Download a checksum-verified INTA Rescue image and prepare a Ventoy volume using the bundled media utility.");
                Action(pageActions, "Open Media Creator", OpenMedia);
                if (vmAvailable) Action(pageActions, "Preview ISO in virtual machine\u2026", ChooseLabIso);
                string guide = Path.Combine(baseDirectory, "docs", "rescue.md");
                if (File.Exists(guide)) {
                    string text = await Task.Run(() => File.ReadAllText(guide), token);
                    if (token.IsCancellationRequested) return;
                    RenderGuide(text);
                }
                Status("Rescue tools open on request."); return;
            }
            if (section == "Activity")
            {
                Action(pageActions, "Refresh", RefreshPage);
                var events = Json.Array(await core.Read(new[] { "history" }, token)).ToList();
                if (token.IsCancellationRequested) return;
                count.Text = events.Count + (events.Count == 1 ? " event" : " events");
                if (events.Count == 0) Message("No recorded activity", "Completed operations appear here when the core records them.");
                else { RenderReport("Local operation history", events); AddExport("activity", events); }
                Status("Local activity loaded.");
            }
        }
        private async Task ShowOperation(Entry item, CancellationToken token)
        {
            Title(item.Title);
            if (item.Kind == "profile") { await ShowProfile(item, token); return; }
            if (item.Kind == "setting") {
                var state = Json.Object(await core.Read(new[] { "tweak", "status", item.Id }, token));
                if (token.IsCancellationRequested) return;
                if (Json.Text(state, "id") != item.Id) throw new InvalidDataException("The core returned a different Windows setting.");
                string effect = Json.Text(state, "effect");
                Note(effect);
                object current = Json.Get(state, "registryValue"), target = Json.Get(state, "targetRegistryValue");
                Field("currentRegistryValue", current ?? "Absent (Explorer default)");
                Field("proposedRegistryValue", target);
                Field("savedChange", Json.Get(state, "phase") ?? "None");
                foreach (string key in new[] { "scope", "registryPath", "valueName", "activation", "risk", "compatibility",
                        "policy", "performanceEvidence", "requiresRestart", "requiresElevation" }) Field(key, Json.Get(state, key));
                bool defaultSetting = item.Id == "explorer-file-extensions";
                Action(detailActions, "Inspect setting", delegate {
                    RunOperation(defaultSetting ? "Current file extension setting" : "Current setting — " + item.Title,
                        new[] { "tweak", "status", item.Id }, false);
                });
                Action(detailActions, "Preview change", delegate {
                    RunOperation(defaultSetting ? "Proposed file extension change" : "Proposed change — " + item.Title,
                        new[] { "tweak", "preview", item.Id }, false);
                });
                var apply = Action(detailActions, defaultSetting ? "Show file extensions\u2026" : "Apply change\u2026", delegate {
                    ConfirmSetting(item, effect, false);
                });
                apply.IsEnabled = !Object.Equals(current, target) || Json.Text(state, "phase") == "prepared";
                var undo = Action(detailActions, "Undo change\u2026", delegate { ConfirmSetting(item, effect, true); });
                undo.IsEnabled = Object.Equals(Json.Get(state, "undoAvailable"), true);
                Status("Current-user Explorer setting."); return;
            }
            if (item.Kind == "diagnostic")
            {
                Note(item.Subtitle + ".");
                if (item.Id == "space")
                    Action(detailActions, "Choose folder\u2026", delegate {
                        string path = ChooseFolder("Select a local folder for read-only space analysis.");
                        if (path != null) RunOperation(item.Title, new[] { "storage", path }, false);
                    });
                else Action(detailActions, "Run inspection", delegate { RunOperation(item.Title, new[] { "inspect", item.Id }, false); });
                Status("Ready for read-only inspection.");
            }
            if (item.Kind == "scenario")
            {
                var response = await core.Read(new[] { "scenario", "preview", item.Id }, token);
                if (token.IsCancellationRequested) return;
                RenderReport(item.Title, response);
                var stages = new List<System.Windows.Controls.CheckBox>();
                foreach (object value in Json.Array(Json.Get(item.Data, "stages"))) {
                    string stage = Convert.ToString(value);
                    var selector = new System.Windows.Controls.CheckBox { Content = "Include " + stage, Tag = stage, IsChecked = true,
                        Margin = new Thickness(0, 6, 0, 3) };
                    selector.SetResourceReference(System.Windows.Controls.CheckBox.ForegroundProperty, "Text");
                    stages.Add(selector); detail.Children.Add(selector);
                }
                Action(detailActions, "Run scenario", delegate {
                    var arguments = new List<string> { "scenario", "run", item.Id };
                    foreach (var toggle in stages.Where(c => c.IsChecked != true)) { arguments.Add("--skip"); arguments.Add((string)toggle.Tag); }
                    RunOperation(item.Title, arguments.ToArray(), false);
                });
                Status("Scenario preview loaded.");
            }
            if (item.Kind == "package")
            {
                RenderRecord(item, item.Data);
                Action(detailActions, "Verify package", delegate { RunOperation("Package integrity", Toolbox("verify", item.Id), false); });
                Action(detailActions, "Run version probe", delegate { RunOperation("Package version probe", Toolbox("probe", item.Id), false); });
                Status("Managed package metadata loaded.");
            }
        }
        private void ConfirmSetting(Entry item, string effect, bool undo)
        {
            string text = undo ?
                "Restore the prior value saved by INTA for this setting?\n\n" + item.Title :
                "Apply this setting for the current Windows user?\n\n" + item.Title + "\n" + effect + "\n\nINTA will save the prior value and verify the change.";
            if (MessageBox.Show(window, text, undo ? "Restore Explorer setting" : "Change Explorer setting",
                    MessageBoxButton.OKCancel, MessageBoxImage.Question, MessageBoxResult.Cancel) != MessageBoxResult.OK) return;
            RunOperation((undo ? "Restore — " : "Apply — ") + item.Title, new[] { "tweak", undo ? "undo" : "apply", item.Id, "--approve" }, true);
        }
        private void CreateBackup()
        {
            var source = new OpenFileDialog { Title = "Choose one file to back up", Multiselect = false, CheckFileExists = true,
                Filter = "All files (*.*)|*.*" };
            if (source.ShowDialog(window) != true) return;
            var destination = new SaveFileDialog { Title = "Choose a new .7z backup file", CheckPathExists = true, AddExtension = true,
                DefaultExt = ".7z", Filter = "7-Zip archive (*.7z)|*.7z", OverwritePrompt = false, FileName = Path.GetFileName(source.FileName) + ".7z" };
            if (destination.ShowDialog(window) != true) return;
            if (File.Exists(destination.FileName) || Directory.Exists(destination.FileName) ||
                    !String.Equals(Path.GetExtension(destination.FileName), ".7z", StringComparison.OrdinalIgnoreCase))
            {
                Status("Choose a new .7z file name for this backup."); return;
            }
            string confirm = "Create and verify a single-file backup?\n\nSource file\n" + source.FileName +
                    "\n\nNew .7z archive\n" + destination.FileName;
            if (MessageBox.Show(window, confirm, "Confirm verified backup", MessageBoxButton.OKCancel, MessageBoxImage.Question,
                    MessageBoxResult.Cancel) != MessageBoxResult.OK) return;
            RunOperation("Verified single-file backup", Toolbox("backup", source.FileName, destination.FileName, "--approve"), true);
        }
        private void LockNavigation(bool locked)
        {
            changingSetting = locked;
            navigation.IsEnabled = !locked; pageActions.IsEnabled = !locked; entries.IsEnabled = !locked;
            search.IsEnabled = !locked; Find<System.Windows.Controls.Button>("Clear").IsEnabled = !locked;
        }
        private async void RunOperation(string title, string[] args, bool setting, Action<object> completed = null)
        {
            Cancel(ref detailRequest); detailRequest = new CancellationTokenSource();
            var token = detailRequest.Token;
            if (setting) LockNavigation(true);
            Message(title, "Operation in progress\u2026");
            if (!setting)
                Action(detailActions, "Cancel", delegate { Cancel(ref detailRequest); Message(title, "Operation canceled."); AddBack(); Status("Operation canceled."); });
            Status("Running " + title.ToLowerInvariant() + "\u2026");
            try {
                object result = await core.Read(args, token);
                if (token.IsCancellationRequested) return;
                RenderReport(title, result);
                AddExport(args[0], result); AddBack();
                var map = result as Dictionary<string, object>;
                Status(map != null && Json.Text(map, "status") == "stopped" ? title + " stopped. Review the stage observations." : title + " completed.");
                if (completed != null) completed(result);
            } catch (OperationCanceledException) { }
            catch (Exception e) {
                if (token.IsCancellationRequested) return;
                Message("Operation failed", e.GetBaseException().Message);
                Action(detailActions, "Return", delegate { if (Current != null) ShowSelection(); else RefreshPage(); });
                Status(title + " failed.");
            } finally { if (setting) LockNavigation(false); }
        }
        private void AddBack()
        {
            if (Current != null) Action(detailActions, "Back to " + Current.Title, ShowSelection);
        }
        private async void ShowCheckpoints()
        {
            Cancel(ref detailRequest);
            detailRequest = new CancellationTokenSource(); var token = detailRequest.Token;
            Message("Saved checkpoints", "Reading local scenario history\u2026");
            try {
                var items = Json.Array(await core.Read(new[] { "scenario", "checkpoints" }, token)).Select(Json.Object).ToList();
                if (token.IsCancellationRequested) return;
                ResetDetail(); Title("Saved checkpoints");
                if (items.Count == 0) Note("Run a scenario to create its first checkpoint.");
                foreach (var checkpoint in items)
                {
                    string id = Json.Text(checkpoint, "scenario"), run = Json.Text(checkpoint, "runId");
                    Paragraph(id + "  \u00b7  " + Json.Text(checkpoint, "created"), "Text", 15, true);
                    Field("stages", Json.Get(checkpoint, "stages"));
                    var scenario = listing == null ? null : listing.FirstOrDefault(s => s.Id == id);
                    var progress = Json.Get(checkpoint, "stages") as Dictionary<string, object>;
                    if (scenario != null && progress != null && Json.Array(Json.Get(scenario.Data, "stages")).All(
                        stage => new[] { "complete", "skipped" }.Contains(Json.Text(progress, Convert.ToString(stage))))) continue;
                    var row = new System.Windows.Controls.WrapPanel(); detail.Children.Add(row);
                    Action(row, "Resume " + id, delegate { RunOperation("Resume " + id, new[] { "scenario", "resume", id, run }, false); });
                }
                AddBack(); Status(items.Count + " saved checkpoints.");
            } catch (OperationCanceledException) { }
            catch (Exception e) { if (!token.IsCancellationRequested) { Message("Unable to read checkpoints", e.Message); AddBack(); } }
        }
    }
}
