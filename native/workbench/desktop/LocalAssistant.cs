using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading;
using System.Windows;
using System.Windows.Automation;
using System.Windows.Controls;

namespace Inta.Desktop
{
    internal sealed class LocalModel
    {
        internal string Name, Digest;
        public string Display { get; set; }
    }

    internal sealed partial class Workspace
    {
        private TextBox PlainText(string text, string label)
        {
            var box = new TextBox { Text = text, IsReadOnly = true, TextWrapping = TextWrapping.Wrap,
                AcceptsReturn = true, MaxHeight = 300, VerticalScrollBarVisibility = ScrollBarVisibility.Auto,
                Margin = new Thickness(0, 3, 0, 10), Padding = new Thickness(8) };
            AutomationProperties.SetName(box, label);
            detail.Children.Add(box);
            return box;
        }

        private async void ReviewExplanation(Entry entry)
        {
            if (entry == null) { RefreshPage(); return; }
            Cancel(ref detailRequest);
            detailRequest = new CancellationTokenSource(); var token = detailRequest.Token;
            Message("Local explanation", "Preparing the published record\u2026");
            bool offerServer = false;
            try {
                var preview = Json.Object(await core.Read(new[] { "assistant", "preview", entry.Kind, entry.Id }, token));
                if (token.IsCancellationRequested) return;
                if (Json.Text(preview, "kind") != entry.Kind || Json.Text(preview, "id") != entry.Id)
                    throw new InvalidOperationException("The core returned a different catalog record.");
                ResetDetail();
                Title("Local explanation");
                Note("Review the exact prompt for " + entry.Title + ". Only published catalog text is included.");
                PlainText(Json.Text(preview, "prompt"), "Exact local model prompt");
                Paragraph("Installed Ollama model", "Text", 14, true);
                Note("Local service at " + Json.Text(preview, "endpoint") + " with cloud forwarding disabled.");
                AddBack(); Status("Checking local Ollama models\u2026");
                var inventory = Json.Object(await core.Read(new[] { "assistant", "models" }, token));
                if (token.IsCancellationRequested) return;
                var models = Json.Array(Json.Get(inventory, "models")).Select(Json.Object).Select(d => {
                    string name = Json.Text(d, "name");
                    return new LocalModel { Name = name, Digest = Json.Text(d, "digest"),
                        Display = name + "  \u00b7  " + DisplayValue("modelBytes", Json.Get(d, "bytes")) };
                }).ToList();
                if (modelServer != null && !modelServer.HasExited && OwnsModelListener(modelServer.Id))
                    models.RemoveAll(model => model.Digest != ownedModelDigest);
                if (models.Count == 0) {
                    Note("Ollama returned no installed local models eligible for this action.");
                    Action(detailActions, "Refresh models", delegate { ReviewExplanation(entry); });
                    Status("No local models available."); return;
                }
                var selector = new ComboBox { ItemsSource = models, DisplayMemberPath = "Display", IsEditable = false,
                    Margin = new Thickness(0, 0, 0, 10), MinHeight = 28, SelectedIndex = models.Count == 1 ? 0 : -1 };
                AutomationProperties.SetName(selector, "Installed local model");
                AutomationProperties.SetAutomationId(selector, "LocalModelSelector");
                detail.Children.Add(selector);
                var buttons = new WrapPanel(); detail.Children.Add(buttons);
                var generate = Action(buttons, "Generate explanation\u2026", delegate {
                    var chosen = selector.SelectedItem as LocalModel;
                    if (chosen == null) return;
                    if (MessageBox.Show(window, "Send the reviewed catalog prompt to " + chosen.Name + " on this computer?",
                        "Confirm local explanation", MessageBoxButton.OKCancel, MessageBoxImage.Question,
                        MessageBoxResult.Cancel) == MessageBoxResult.OK)
                        GenerateExplanation(entry, chosen, Json.Text(preview, "promptSha256"));
                    else Status("Local generation canceled.");
                });
                generate.IsEnabled = selector.SelectedItem != null;
                selector.SelectionChanged += delegate { generate.IsEnabled = selector.SelectedItem != null; };
                Action(buttons, "Refresh models", delegate { ReviewExplanation(entry); });
                OfferStopModelServer(buttons,entry);
                Status("Review the local model and prompt.");
            } catch (OperationCanceledException) { }
            catch (Exception e) {
                if (token.IsCancellationRequested) return;
                Note(e.GetBaseException().Message);
                Action(detailActions, "Retry local explanation", delegate { ReviewExplanation(entry); });
                if (detailActions.Children.Count == 1) AddBack();
                Status("Local explanation unavailable.");
                offerServer = true;
            }
            if (offerServer) await OfferModelServer(entry,token);
        }

        private async void GenerateExplanation(Entry entry, LocalModel model, string promptHash)
        {
            Cancel(ref detailRequest); detailRequest = new CancellationTokenSource();
            var token = detailRequest.Token;
            Message("Local explanation", "Generating with " + model.Name + "\u2026");
            Action(detailActions, "Cancel generation", delegate { Cancel(ref detailRequest); ShowSelection(); });
            Status("Generating locally\u2026");
            try {
                var result = Json.Object(await core.Read(new[] { "assistant", "explain", entry.Kind, entry.Id,
                    model.Name, model.Digest, promptHash, "--approve" }, token));
                if (token.IsCancellationRequested) return;
                if (Json.Text(result, "id") != entry.Id || Json.Text(result, "kind") != entry.Kind ||
                    Json.Text(result, "modelDigest") != model.Digest || Json.Text(result, "promptSha256") != promptHash)
                    throw new InvalidOperationException("The response does not match the reviewed request.");
                ResetDetail(); Title("Local explanation");
                Note("Generated by " + model.Name + " for " + entry.Title + ". Review against the published record.");
                PlainText(Json.Text(result, "answer"), "Plain-text local model answer");
                AddExport("local-explanation", result); AddBack();
                OfferStopModelServer(detailActions,entry);
                Status("Local explanation complete.");
            } catch (OperationCanceledException) { }
            catch (Exception e) {
                if (token.IsCancellationRequested) return;
                Message("Local explanation unavailable", e.GetBaseException().Message);
                Action(detailActions, "Review prompt", delegate { ReviewExplanation(entry); }); AddBack();
                Status("Local generation failed.");
            }
        }
    }
}
