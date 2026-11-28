using System;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Automation;
using System.Windows.Controls;

namespace Inta.Desktop
{
    internal sealed partial class Workspace
    {
        private async Task SearchReferences(CancellationToken token)
        {
            string query = search.Text.Trim();
            if (query.Length == 0) {
                Note("Enter symptoms or a tool name in Search to find passages in the installed guides, catalog and Wiki.");
                Status("Offline references ready."); return;
            }
            var result = Json.Object(await core.Read(new[] { "knowledge", "search", query }, token));
            token.ThrowIfCancellationRequested();
            var hits = Json.Array(Json.Get(result, "citations")).Select(Json.Object).ToList();
            count.Text = hits.Count + (hits.Count == 1 ? " reference" : " references");
            if (hits.Count == 0) Note("No matching passages. Try a tool name or a shorter description of the symptom.");
            foreach (var hit in hits) RenderCitation(hit, token);
            Status("Offline reference search complete.");
        }

        private void RenderCitation(System.Collections.Generic.Dictionary<string, object> citation, CancellationToken token)
        {
            string location = Json.Text(citation, "location");
            Paragraph(Json.Text(citation, "title"), "Text", 15, true);
            Note(Json.Text(citation, "passage"));
            Note(Json.Text(citation, "section") + " · " + Json.Text(citation, "source") + " · " + location);
            string kind = Json.Text(citation, "kind");
            if (kind == "wiki" || kind == "runbook") {
                var content = new StackPanel { Margin = new Thickness(14, 7, 0, 9) };
                var expand = new Expander { Header = "Read surrounding passages", Content = content, Margin = new Thickness(0, 0, 0, 12) };
                expand.SetResourceReference(Expander.ForegroundProperty, "Text");
                AutomationProperties.SetName(expand, "Read surrounding passages for " + Json.Text(citation, "title"));
                bool loading = false, loaded = false;
                expand.Expanded += async delegate {
                    if (loading || loaded || token.IsCancellationRequested) return;
                    loading = true;
                    content.Children.Clear();
                    try {
                        var result = Json.Object(await core.Read(new[] { "knowledge", "context", Json.Text(citation, "source"), location }, token));
                        token.ThrowIfCancellationRequested();
                        if (Json.Text(result, "sourceTextSha256") != Json.Text(citation, "sourceTextSha256"))
                            throw new InvalidDataException("The guide changed. Run the search again.");
                        foreach (var row in Json.Array(Json.Get(result, "passages")).Select(Json.Object)) {
                            var block = RowText(Json.Text(row, "passage"), Json.Text(row, "location") == location, true);
                            block.Margin = new Thickness(0, 0, 0, 6);
                            content.Children.Add(block);
                        }
                        loaded = true;
                        Status("Surrounding passages loaded.");
                    } catch (OperationCanceledException) { }
                      catch (Exception e) {
                        if (!token.IsCancellationRequested) content.Children.Add(RowText(e.GetBaseException().Message, false, false));
                    } finally { loading = false; }
                };
                detail.Children.Add(expand);
                return;
            }
            string[] parts = location.Split('/');
            if (parts.Length < 3) return;
            string destination = parts[0] == "tool" ? "Tools" : parts[0] == "job" ? "Playbooks" : parts[0] == "method" ? "Methods" : null;
            if (destination == null) return;
            var actions = new WrapPanel();
            detail.Children.Add(actions);
            Action(actions, "Open " + Label(parts[0]).ToLowerInvariant() + " record", delegate { OpenRecord(destination, parts[1]); });
        }

        private void OpenFeature(string view)
        {
            bool inventory = view == "inventory";
            if (!inventory && view != "appearance" && view != "forge" && view != "diagnostics") return;
            string executable = Path.Combine(baseDirectory, inventory ? "INTA System Tools.exe" : "inta-views.exe");
            try {
                using (Process.Start(new ProcessStartInfo { FileName = executable, WorkingDirectory = baseDirectory,
                    Arguments = inventory ? "" : "--view " + view, UseShellExecute = false })) { }
            } catch (Exception e) { Status("Could not open " + view + ". " + e.GetBaseException().Message); }
        }

        private async Task LoadTriage(CancellationToken token)
        {
            Title("Describe the problem");
            Note("The model runs on this computer. Review its proposed checks before starting the inspection.");
            bool needsServer = false;
            try {
                var inventory = Json.Object(await core.Read(new[] { "assistant", "models" }, token));
                token.ThrowIfCancellationRequested();
                var models = Json.Array(Json.Get(inventory, "models")).Select(Json.Object).Select(d => new LocalModel {
                    Name = Json.Text(d, "name"), Display = Json.Text(d, "name"), Digest = Json.Text(d, "digest") }).ToList();
                if (modelServer != null && !modelServer.HasExited && OwnsModelListener(modelServer.Id))
                    models.RemoveAll(m => m.Digest != ownedModelDigest);
                if (models.Count == 0) throw new InvalidOperationException("No eligible offline model is loaded.");
                var choice = new ComboBox { ItemsSource = models, DisplayMemberPath = "Display",
                    SelectedIndex = 0, MaxWidth = 480, HorizontalAlignment = HorizontalAlignment.Left };
                AutomationProperties.SetName(choice, "Local diagnostic model");
                detail.Children.Add(choice);
                var question = new TextBox { TextWrapping = TextWrapping.Wrap, AcceptsReturn = true, MaxLength = 512,
                    MinHeight = 68, MaxHeight = 145, Margin = new Thickness(0, 10, 0, 8) };
                AutomationProperties.SetName(question, "Symptoms to diagnose");
                detail.Children.Add(question);
                var buttons = new WrapPanel();
                detail.Children.Add(buttons);
                var plan = Action(buttons, "Review diagnostic plan", delegate {
                    var selected = choice.SelectedItem as LocalModel;
                    if (selected != null) PreviewTriage(selected, question.Text.Trim());
                });
                var compose = Action(buttons, "Create tested recipe…", delegate {
                    var selected = choice.SelectedItem as LocalModel;
                    if (selected != null) ChooseRecipe(selected, question.Text.Trim());
                });
                plan.IsEnabled = compose.IsEnabled = false;
                question.TextChanged += delegate { plan.IsEnabled = compose.IsEnabled = !String.IsNullOrWhiteSpace(question.Text); };
                OfferStopModelServer(buttons, null);
                Status("Local model ready.");
            } catch (OperationCanceledException) { }
              catch (Exception e) {
                if (token.IsCancellationRequested) return;
                Note(e.GetBaseException().Message);
                Action(detailActions, "Refresh models", RefreshPage);
                needsServer = true;
            }
            if (needsServer && !token.IsCancellationRequested) {
                await OfferModelServer(null, token);
                if (!token.IsCancellationRequested) Status("Select a prepared model store or start your local model server.");
            }
        }

        private async void ChooseRecipe(LocalModel selected, string question)
        {
            var file = new Microsoft.Win32.SaveFileDialog { Title = "Create a new tested local recipe",
                Filter = "INTA Forge recipe (*.json)|*.json", AddExtension = true, DefaultExt = ".json",
                FileName = "INTA-baseline-" + DateTime.Now.ToString("yyyyMMdd-HHmmss"), OverwritePrompt = true };
            if (file.ShowDialog(window) != true) return;
            if (File.Exists(file.FileName)) { Status("Choose an unused filename for the recipe."); return; }
            Cancel(ref detailRequest);
            detailRequest = new CancellationTokenSource();
            var token = detailRequest.Token;
            Message("Testing a local recipe", "The model selects memory, power or startup checks. Forge runs them and validates their assertions before saving.");
            Action(detailActions, "Cancel", RefreshPage);
            try {
                var result = Json.Object(await core.Read(new[] { "ai", "forge", selected.Name, selected.Digest, question, file.FileName, "--approve" }, token));
                token.ThrowIfCancellationRequested();
                Message("Tested recipe saved", file.FileName);
                Field("Operations", Json.Get(result, "selectedOperations"));
                Field("Passing assertions", Json.Get(result, "testsPassed"));
                Field("Manifest SHA-256", Json.Get(result, "manifestSha256"));
                Action(detailActions, "Open CLI Forge", delegate { OpenFeature("forge"); });
                Action(detailActions, "New request", RefreshPage);
            } catch (OperationCanceledException) { }
              catch (Exception e) {
                if (token.IsCancellationRequested) return;
                Message("Recipe unavailable", e.GetBaseException().Message);
                Action(detailActions, "Try again", RefreshPage);
            }
        }

        private async void PreviewTriage(LocalModel selected, string question)
        {
            Cancel(ref detailRequest);
            detailRequest = new CancellationTokenSource();
            var token = detailRequest.Token;
            Message("Preparing an inspection", selected.Name + " is selecting local checks.");
            Action(detailActions, "Cancel", RefreshPage);
            try {
                var result = Json.Object(await core.Read(new[] { "ai", "ask", selected.Name, selected.Digest, question }, token));
                token.ThrowIfCancellationRequested();
                if (Json.Text(result, "state") == "unmatched") {
                    Message("No matching inspection", "Describe one symptom or choose a scenario from the sidebar.");
                } else {
                    var plan = Json.Object(Json.Get(result, "plan"));
                    string id = Json.Text(plan, "id"), hash = Json.Text(result, "planSha256");
                    Message(Json.Text(plan, "title"), Json.Text(plan, "effects"));
                    Field("Checks", Json.Get(plan, "stages"));
                    Field("Stop conditions", Json.Get(plan, "stopIf"));
                    Field("Verification", Json.Get(plan, "verify"));
                    Action(detailActions, "Run these checks", delegate {
                        RunOperation("Local diagnostic inspection", new[] { "ai", "run-plan", id, hash, "--approve" },
                            false, null);
                    });
                }
                var knowledge = Json.Get(result, "knowledge") as System.Collections.Generic.Dictionary<string, object>;
                var citations = knowledge == null ? null : Json.Get(knowledge, "citations") as System.Collections.IList;
                if (citations != null && citations.Count > 0) {
                    Paragraph("Offline references", "Text", 16, true);
                    foreach (var citation in citations.Cast<object>().Select(Json.Object)) RenderCitation(citation, token);
                }
                Action(detailActions, "New question", RefreshPage);
                Status("Plan review complete.");
            } catch (OperationCanceledException) { }
              catch (Exception e) {
                if (token.IsCancellationRequested) return;
                Message("Plan unavailable", e.GetBaseException().Message);
                Action(detailActions, "Try again", RefreshPage);
            }
        }
    }
}
