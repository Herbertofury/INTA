using System;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;
using System.Windows;

namespace Inta.Desktop
{
    internal sealed partial class Workspace
    {
        private async void ShowProfiles()
        {
            var token = pageRequest.Token;
            try {
                var profiles = await core.Read(new[] { "tweak", "profiles" }, token);
                if (token.IsCancellationRequested) return;
                pageActions.Children.Clear();
                Action(pageActions, "Individual settings", RefreshPage);
                SetRows(Json.Array(profiles).Select(Json.Object).Select(p => new Entry {
                    Kind = "profile", Id = Json.Text(p, "id"), Title = Json.Text(p, "name"),
                    Subtitle = Json.Array(Json.Get(p, "settings")).Count() + " Explorer preferences"
                }));
            } catch (OperationCanceledException) { }
            catch (Exception e) { if (!token.IsCancellationRequested) Message("Unable to load profiles", e.GetBaseException().Message); }
        }
        private async Task ShowProfile(Entry item, CancellationToken token)
        {
            var result = Json.Object(await core.Read(new[]{"tweak","profile","preview",item.Id},token));
            if (token.IsCancellationRequested) return;
            var rows = Json.Array(Json.Get(result,"settings")).Select(Json.Object).ToList();
            foreach (var row in rows) {
                Note(Json.Text(row,"setting") + "  ·  " + Json.Text(row,"disposition"));
                Note(Json.Text(row,"registryPath") + "\\" + Json.Text(row,"valueName") + "   " +
                    (Json.Get(row,"registryValue") == null ? "default" : Json.Text(row,"registryValue")) + " → " + Json.Text(row,"targetRegistryValue"));
            }
            string phase = Json.Text(result,"phase");
            var apply = Action(detailActions, "Apply profile…", delegate { ConfirmProfile(item,false); });
            apply.IsEnabled = phase != "restoring" && (phase == "prepared" || rows.Any(r => Json.Text(r,"disposition") == "change"));
            var undo = Action(detailActions, "Undo profile…", delegate { ConfirmProfile(item,true); });
            undo.IsEnabled = Object.Equals(Json.Get(result,"undoAvailable"), true);
            Status("Profile preview loaded.");
        }
        private void ConfirmProfile(Entry item, bool undo)
        {
            string prompt = undo ? "Restore the previous values saved by this profile?" :
                "Apply the listed Explorer changes for this Windows user? INTA saves previous values before it writes. An interrupted change can be restored with Undo profile.";
            if (MessageBox.Show(window,prompt,item.Title,MessageBoxButton.OKCancel,MessageBoxImage.Question,MessageBoxResult.Cancel) != MessageBoxResult.OK) return;
            RunOperation(item.Title,new[]{"tweak","profile",undo ? "undo" : "apply",item.Id,"--approve"},true);
        }
    }
}
