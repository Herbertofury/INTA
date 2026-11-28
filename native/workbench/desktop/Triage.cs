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
                plan.IsEnabled = false;
                question.TextChanged += delegate { plan.IsEnabled = !String.IsNullOrWhiteSpace(question.Text); };
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
