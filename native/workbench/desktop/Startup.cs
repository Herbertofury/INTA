using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Automation;
using System.Windows.Controls;
using System.Windows.Data;

namespace Inta.Desktop
{
    internal sealed partial class Workspace
    {
        private sealed class StartupEntry
        {
            public string Id { get; set; }
            public string Name { get; set; }
            public string State { get; set; }
            public string Type { get; set; }
            public string Revision { get; set; }
            public string Verb { get; set; }
        }

        private static bool ConfirmStartup(Window owner, Dictionary<string, object> preview, string verb)
        {
            var dialog = new Window { Owner = owner, Title = "Review sign-in change", SizeToContent = SizeToContent.Height,
                Width = 475, WindowStartupLocation = WindowStartupLocation.CenterOwner, ResizeMode = ResizeMode.NoResize };
            var panel = new StackPanel { Margin = new Thickness(18) };
            panel.Children.Add(new TextBlock { Text = Json.Text(preview, "name"), FontWeight = FontWeights.Bold, TextWrapping = TextWrapping.Wrap });
            panel.Children.Add(new TextBlock { Text = Json.Text(preview, "registryPath") + "\n" + Json.Text(preview, "type") + " · " + Json.Text(preview, "bytes") + " bytes", TextWrapping = TextWrapping.Wrap, Margin = new Thickness(0,6,0,10) });
            panel.Children.Add(new TextBlock { Text = Json.Text(preview, "effect"), TextWrapping = TextWrapping.Wrap });
            var choices = new StackPanel { Orientation = Orientation.Horizontal, HorizontalAlignment = HorizontalAlignment.Right, Margin = new Thickness(0,16,0,0) };
            var cancel = new Button { Content = "Cancel", IsCancel = true, IsDefault = true, Padding = new Thickness(8,3,8,3) };
            var approve = new Button { Content = verb + " this entry", Margin = new Thickness(8,0,0,0), Padding = new Thickness(8,3,8,3) };
            approve.Click += delegate { dialog.DialogResult = true; };
            choices.Children.Add(cancel); choices.Children.Add(approve); panel.Children.Add(choices); dialog.Content = panel;
            return dialog.ShowDialog() == true;
        }

        private void OpenStartupReview()
        {
            var layout = new DockPanel { Margin = new Thickness(14) };
            var dialog = new Window { Owner = window, Title = "Current-user sign-in entries", Width = 780, Height = 480,
                MinWidth = 480, MinHeight = 320, WindowStartupLocation = WindowStartupLocation.CenterOwner, Content = layout };
            var instructions = new TextBlock { Text = "Review one Windows Run registration at a time. Command lines stay private. A registration can be affected by Windows startup policies.", TextWrapping = TextWrapping.Wrap, Margin = new Thickness(0,0,0,10) };
            DockPanel.SetDock(instructions, Dock.Top); layout.Children.Add(instructions);
            var controls = new StackPanel { Orientation = Orientation.Horizontal };
            var refresh = new Button { Content = "Refresh", Padding = new Thickness(7,3,7,3) };
            var change = new Button { Content = "Review selected entry…", Padding = new Thickness(7,3,7,3), Margin = new Thickness(6,0,0,0), IsEnabled = false };
            controls.Children.Add(refresh); controls.Children.Add(change); DockPanel.SetDock(controls, Dock.Bottom); layout.Children.Add(controls);
            var status = new TextBlock { TextWrapping = TextWrapping.Wrap, Margin = new Thickness(0,6,0,8) };
            AutomationProperties.SetName(status, "Startup review status");
            DockPanel.SetDock(status, Dock.Bottom); layout.Children.Add(status);
            var grid = new DataGrid { AutoGenerateColumns = false, CanUserAddRows = false, IsReadOnly = true, SelectionMode = DataGridSelectionMode.Single };
            AutomationProperties.SetName(grid, "Sign-in registrations");
            foreach (string column in new[] { "Name", "State", "Type" })
                grid.Columns.Add(new DataGridTextColumn { Header = column, Binding = new Binding(column), Width = column == "Name" ? new DataGridLength(2,DataGridLengthUnitType.Star) : new DataGridLength(1,DataGridLengthUnitType.Star) });
            layout.Children.Add(grid);
            bool busy = false;
            System.Action buttons = delegate {
                var selected = grid.SelectedItem as StartupEntry;
                change.IsEnabled = !busy && selected != null;
                refresh.IsEnabled = !busy; grid.IsEnabled = !busy;
            };
            Func<string,Task> reload = async delegate(string selectedId) {
                var report = Json.Object(await core.Read(new[] { "startup", "list" }, CancellationToken.None));
                var entries = Json.Array(Json.Get(report, "entries")).Select(Json.Object).Select(r => new StartupEntry {
                    Name = Json.Text(r,"name"), State = Json.Text(r,"state"), Type = Json.Text(r,"type"), Id = Json.Text(r,"id"),
                    Revision = Json.Text(r,"revision"), Verb = Json.Text(r,"action") }).OrderBy(r => r.Name).ToList();
                grid.ItemsSource = entries;
                grid.SelectedItem = entries.FirstOrDefault(r => r.Id == selectedId);
                status.Text = entries.Count == 0 ? "This user has no Run registrations or saved INTA entries." : entries.Count + " entries reviewed. A conflict preserves both registrations for investigation.";
            };
            Func<Func<Task>,Task> work = async delegate(Func<Task> action) {
                if (busy) return;
                busy = true; buttons();
                try { await action(); } catch (Exception e) { status.Text = e.GetBaseException().Message; }
                finally { busy = false; buttons(); }
            };
            refresh.Click += async delegate { await work(delegate { return reload(null); }); };
            grid.SelectionChanged += delegate { buttons(); };
            change.Click += async delegate {
                var row = grid.SelectedItem as StartupEntry; if (row == null) return;
                await work(async delegate {
                    var preview = Json.Object(await core.Read(new[] { "startup", "preview", row.Id }, CancellationToken.None));
                    if (Json.Text(preview,"revision") != row.Revision || Json.Text(preview,"action") != row.Verb) {
                        await reload(row.Id); status.Text = "The selected entry changed. Review its current state."; return;
                    }
                    if (row.Verb != "disable" && row.Verb != "restore") {
                        status.Text = row.Name + " · " + Json.Text(preview, "effect"); return;
                    }
                    string verb = row.Verb == "disable" ? "Disable" : "Restore";
                    if (!ConfirmStartup(dialog,preview,verb)) return;
                    var result = Json.Object(await core.Read(new[] { "startup", row.Verb, row.Id, row.Revision, "--approve" }, CancellationToken.None));
                    await reload(row.Id);
                    status.Text = row.Name + " · " + Json.Text(result,"state") + ". The registry change was verified.";
                });
            };
            dialog.Closing += delegate(object sender, System.ComponentModel.CancelEventArgs e) { if (busy) e.Cancel = true; };
            dialog.Loaded += async delegate { await work(delegate { return reload(null); }); };
            dialog.ShowDialog();
        }
    }
}
