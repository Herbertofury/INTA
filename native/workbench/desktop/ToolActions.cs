using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Text.RegularExpressions;
using System.Threading;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Automation;
using System.Windows.Controls;
using System.Windows.Documents;
using Microsoft.Win32;

namespace Inta.Desktop
{
    internal sealed class ToolActionRoute
    {
        internal string Id, Operation;
        internal static bool Parse(string[] args, out ToolActionRoute route)
        {
            route = null;
            if (args.Length == 0) return true;
            if (args.Length != 1 && !(args.Length == 2 && args[0] == "--tool-action")) return false;
            var match = Regex.Match(args[args.Length - 1], @"\Ainta-tool://open/([a-z0-9][a-z0-9-]{0,79})/(install|launch)\z",
                RegexOptions.CultureInvariant);
            if (!match.Success) return false;
            route = new ToolActionRoute { Id = match.Groups[1].Value, Operation = match.Groups[2].Value };
            return true;
        }
    }

    // The offline hub and desktop consume the same generated editorial registry.
    // Match longer aliases first and keep deliberately case-sensitive names intact.
    internal sealed class ToolMentions
    {
        private readonly Dictionary<string, string> ids = new Dictionary<string, string>(StringComparer.OrdinalIgnoreCase);
        private readonly Dictionary<string, string> names = new Dictionary<string, string>(StringComparer.Ordinal);
        private readonly Regex matcher;
        internal ToolMentions()
        {
            using (var reader = new StreamReader(Assembly.GetExecutingAssembly().GetManifestResourceStream("ToolCatalog.json"))) {
                var catalog = Json.Object(Json.Serializer().DeserializeObject(reader.ReadToEnd()));
                foreach (var record in Json.Array(Json.Get(catalog, "tools")).Select(Json.Object))
                    names.Add(Json.Text(record, "id"), Json.Text(record, "name"));
                var aliases = new List<string>();
                foreach (var record in Json.Array(Json.Get(catalog, "links")).Select(Json.Object)) {
                    if (Json.Text(record, "kind") != "tool") continue;
                    string id = Json.Text(record, "id"), alias = Json.Text(record, "name");
                    if (!names.ContainsKey(id)) throw new InvalidDataException("Unknown tool in the mention registry.");
                    ids.Add(alias, id);
                    bool exact = Object.Equals(Json.Get(record, "caseSensitive"), true);
                    aliases.Add(exact ? Regex.Escape(alias) : "(?i:" + Regex.Escape(alias) + ")");
                }
                // Consume URLs, paths and inline code before looking for editorial references.
                string protectedText = @"(?:https?://|[A-Za-z]:[\\/]|\u005c\u005c)[^\s<>]+|\x60[^\x60]*\x60";
                matcher = new Regex(protectedText + @"|(?<![\w])(?<tool>" + String.Join("|", aliases) + @")(?![\w])",
                    RegexOptions.Compiled | RegexOptions.CultureInvariant);
            }
        }
        internal string Name(string id) { string name; return names.TryGetValue(id, out name) ? name : id; }
        internal void Visit(string text, Action<string> literal, Action<string, string> mention)
        {
            if (String.IsNullOrEmpty(text)) return;
            int start = 0;
            foreach (Match match in matcher.Matches(text)) {
                if (!match.Groups["tool"].Success) continue;
                if (match.Index > start) literal(text.Substring(start, match.Index - start));
                mention(match.Value, ids[match.Value]);
                start = match.Index + match.Length;
            }
            if (start < text.Length) literal(text.Substring(start));
        }
    }

    internal sealed partial class Workspace
    {
        private readonly ToolMentions toolMentions = new ToolMentions();

        internal TextBlock RowText(string value, bool title, bool link)
        {
            var block = new TextBlock { FontSize = title ? 14 : 12, FontWeight = title ? FontWeights.SemiBold : FontWeights.Normal,
                TextWrapping = TextWrapping.Wrap };
            block.SetResourceReference(TextBlock.ForegroundProperty, title ? "Text" : "Muted");
            if (link) AddMentions(block, value); else block.Text = value;
            return block;
        }

        private void RowActionClick(object sender, RoutedEventArgs e)
        {
            var button = e.OriginalSource as Button;
            var entry = button == null ? null : button.DataContext as Entry;
            if (entry == null || !entry.HasActions || changingSetting) return;
            string operation = button.Tag as string;
            if (operation != "install" && operation != "launch") return;
            e.Handled = true;
            ReviewTool(entry.Id, operation, entry.Kind == "package" ? "Managed toolbox" : "Tools");
        }

        private void ReviewTool(string id, string operation, string destination = "Tools")
        {
            if (changingSetting) return;
            var visible = listing == null ? null : listing.FirstOrDefault(e => e.HasActions && e.Id == id);
            if (visible == null) { OpenRecord(destination, id, operation); return; }
            initializing = true; entries.SelectedItem = visible; entries.ScrollIntoView(visible); initializing = false;
            ShowSelection(operation);
        }

        private Hyperlink Link(string label, string accessible, Action click)
        {
            var link = new Hyperlink(new Run(label)) { ToolTip = accessible };
            AutomationProperties.SetName(link, accessible);
            link.SetResourceReference(Hyperlink.ForegroundProperty, "Accent");
            link.Click += delegate(object sender, RoutedEventArgs e) { e.Handled = true; if (!changingSetting) click(); };
            return link;
        }

        private void AddMentions(TextBlock block, string text, Uri source = null)
        {
            toolMentions.Visit(text, literal => {
                if (source == null) block.Inlines.Add(literal);
                else block.Inlines.Add(Link(literal, "Open " + source.Host, delegate { OpenWeb(source); }));
            }, (spelling, id) => {
                string name = toolMentions.Name(id);
                var menu = new ContextMenu { PlacementTarget = block, Placement = System.Windows.Controls.Primitives.PlacementMode.RelativePoint };
                MenuAction(menu, "Review installation for " + name, delegate { ReviewTool(id, "install"); });
                MenuAction(menu, "Review launch for " + name, delegate { ReviewTool(id, "launch"); });
                MenuAction(menu, "Open " + name + " record", delegate { OpenRecord("Tools", id); });
                if (source != null) MenuAction(menu, "Open " + source.Host, delegate { OpenWeb(source); });
                Hyperlink link = null;
                link = Link(spelling, "Installation and launch options for " + name, delegate {
                    var anchor = link.ContentStart.GetCharacterRect(LogicalDirection.Forward);
                    if (!anchor.IsEmpty) { menu.HorizontalOffset = anchor.X; menu.VerticalOffset = anchor.Bottom; }
                    menu.IsOpen = true;
                });
                link.ContextMenu = menu;
                block.Inlines.Add(link);
            });
        }

        private static void MenuAction(ContextMenu menu, string label, Action run)
        {
            var item = new MenuItem { Header = label };
            AutomationProperties.SetName(item, label);
            item.Click += delegate { run(); };
            menu.Items.Add(item);
        }

        private void AddRecommendation(string id)
        {
            string name = toolMentions.Name(id);
            var block = Paragraph("", "Text", 14, false);
            block.Inlines.Add(Link(name, "Open " + name + " record", delegate { OpenRecord("Tools", id); }));
            block.Inlines.Add("   ");
            block.Inlines.Add(Link("Install…", "Review installation for " + name, delegate { ReviewTool(id, "install"); }));
            block.Inlines.Add("   ");
            block.Inlines.Add(Link("Launch…", "Review launch for " + name, delegate { ReviewTool(id, "launch"); }));
        }

        private string[] ToolCommand(string operation, string id, string selection = null)
        {
            var args = new List<string> { "tool", operation, id };
            if (selection != null) args.Add(selection);
            if (operation != "actions") args.Add("--approve");
            if (toolboxRoot != null) { args.Add("--root"); args.Add(toolboxRoot); }
            return args.ToArray();
        }

        private static string Purpose(string operation)
        {
            if (operation == "workflow") return "Verification test";
            if (operation == "configure") return "Console configuration";
            if (operation == "linux") return "Linux launch";
            return operation == "install" ? "Installation" : operation == "launch" ? "Launch" : "Executable binding";
        }

        private async Task ShowToolActions(Entry entry, string requested, CancellationToken token)
        {
            var panel = new StackPanel { Margin = new Thickness(0, 4, 0, 10) };
            detail.Children.Insert(Math.Min(1, detail.Children.Count), panel);
            var heading = RowText("Checking installation and launch options…", false, false);
            panel.Children.Add(heading);
            try {
                var options = Json.Object(await core.Read(ToolCommand("actions", entry.Id), token));
                if (token.IsCancellationRequested) return;
                if (Json.Text(options, "id") != entry.Id) throw new InvalidDataException("The core returned a different tool identifier.");
                string name = Json.Text(options, "name");
                var operations = new List<string> { "install", "launch" };
                var workflow = Json.Get(options,"workflow") as Dictionary<string,object>;
                if (workflow != null && Object.Equals(Json.Get(workflow,"available"),true)) operations.Add("workflow");
                var binding = Json.Get(options, "bind") as Dictionary<string, object>;
                if (binding != null && Object.Equals(Json.Get(binding, "available"), true)) operations.Add("bind");
                var configure = Json.Get(options, "configure") as Dictionary<string, object>;
                if (configure != null && Object.Equals(Json.Get(configure, "available"), true)) operations.Add("configure");
                var linux = Json.Get(options, "linux") as Dictionary<string, object>;
                if (linux != null && Object.Equals(Json.Get(linux, "available"), true) &&
                    Json.Text(Json.Object(Json.Get(options, "launch")), "kind") != "linux") operations.Add("linux");
                if (requested != null) { operations.Remove(requested); operations.Insert(0, requested); }
                var records = operations.Select(op => Json.Object(Json.Get(options, op))).ToList();
                for (int i = 0; i < records.Count; i++) {
                    var r = records[i];
                    if (String.IsNullOrWhiteSpace(Json.Text(r, "label")) || !(Json.Get(r, "available") is bool) ||
                        String.IsNullOrWhiteSpace(Json.Text(r, "kind")) || String.IsNullOrWhiteSpace(Json.Text(r, "detail")))
                        throw new InvalidDataException("The core returned incomplete tool action metadata.");
                }
                panel.Children.Clear();
                if (requested != null) {
                    var review = RowText("Review " + Purpose(requested).ToLowerInvariant() + " for " + name, true, false);
                    AutomationProperties.SetAutomationId(review, "RequestedToolAction");
                    panel.Children.Add(review);
                }
                for (int i = 0; i < records.Count; i++) AddToolOption(panel, entry.Id, name, operations[i], records[i]);
                if (requested != null) Status(Purpose(requested) + " options for " + name + ".");
            } catch (OperationCanceledException) { }
            catch (Exception error) {
                if (token.IsCancellationRequested) return;
                panel.Children.Clear();
                panel.Children.Add(RowText("Tool actions unavailable. " + error.GetBaseException().Message, false, false));
                if (requested != null) Status("Unable to review the requested tool action.");
            }
        }

        private void AddToolOption(StackPanel panel, string id, string name, string operation, Dictionary<string, object> option)
        {
            string purpose = Purpose(operation), label = Json.Text(option, "label"), description = Json.Text(option, "detail");
            string kind = Json.Text(option, "kind");
            bool ready = (bool)Json.Get(option, "available");
            var row = new WrapPanel { Margin = new Thickness(0, 7, 0, 0) };
            var button = Action(row, label, delegate { PerformToolAction(id, name, operation, kind, label, description); });
            button.Margin = new Thickness(0, 0, 8, 4);
            button.IsEnabled = ready;
            button.ToolTip = description;
            ToolTipService.SetShowOnDisabled(button, true);
            AutomationProperties.SetName(button, purpose + " — " + label);
            AutomationProperties.SetAutomationId(button, "ToolAction-" + operation);
            var badge = RowText(purpose + (ready ? "" : " unavailable"), false, false);
            badge.VerticalAlignment = VerticalAlignment.Center;
            row.Children.Add(badge);
            panel.Children.Add(row);
            var explanation = RowText(description, false, true);
            AutomationProperties.SetAutomationId(explanation, "ToolDetail-" + operation);
            panel.Children.Add(explanation);
        }

        private void PerformToolAction(string id, string name, string operation, string kind, string label, string description)
        {
            if (changingSetting) return;
            string file = null;
            if (operation == "configure") {
                file = ChooseConsoleUrl(name);
                if (file == null) return;
            } else if (operation == "bind" || (operation == "launch" && kind == "boot-media")) {
                bool executable = operation == "bind";
                var picker = new OpenFileDialog { Title = executable ? "Choose the installed executable for " + name : "Choose a local boot ISO for " + name,
                    Filter = executable ? "Windows application (*.exe)|*.exe" : "Boot ISO (*.iso)|*.iso", Multiselect = false,
                    CheckFileExists = true, RestoreDirectory = true };
                if (picker.ShowDialog(window) != true) return;
                file = picker.FileName;
            }
            string title = "Confirm " + Purpose(operation).ToLowerInvariant() + " for " + name;
            string content = label + "\n\n" + description + (file == null ? "" : "\n\n" + file);
            if (MessageBox.Show(window, content, title, MessageBoxButton.OKCancel, MessageBoxImage.Question, MessageBoxResult.Cancel)
                != MessageBoxResult.OK) { Status("Canceled " + Purpose(operation).ToLowerInvariant() + " for " + name + "."); return; }
            if (operation == "launch" && (kind == "rescue-media" || kind == "media-creator" || kind == "vm-workspace")) {
                RunOperation(label + " — " + name, ToolCommand(operation, id), true, result => {
                    var handoff = Json.Object(result);
                    if (Json.Text(handoff, "id") != id || Json.Text(handoff, "workspace") != kind || Json.Text(handoff, "state") != "workspace")
                        throw new InvalidDataException("The core did not authorize the requested workspace.");
                    if (kind == "vm-workspace" && !navigation.Items.Contains("Virtual machines")) {
                        initializing = true; BindNavigation(true); initializing = false;
                    }
                    navigation.SelectedItem = kind == "vm-workspace" ? "Virtual machines" : "Rescue media";
                    if (kind == "media-creator") OpenMedia();
                });
                return;
            }
            RunOperation(label + " — " + name, ToolCommand(operation, id, file), true);
        }

        private string ChooseConsoleUrl(string name)
        {
            var dialog = new Window { Owner = window, Title = "Configure " + name + " console", Width = 490,
                SizeToContent = SizeToContent.Height, ResizeMode = ResizeMode.NoResize,
                WindowStartupLocation = WindowStartupLocation.CenterOwner, ShowInTaskbar = false };
            foreach (string key in BrushKeys) dialog.Resources[key] = window.Resources[key];
            foreach (Type type in new[] { typeof(TextBox), typeof(TextBlock), typeof(Button) })
                dialog.Resources[type] = window.FindResource(type);
            dialog.SetResourceReference(Window.BackgroundProperty, "Canvas");
            var panel = new StackPanel { Margin = new Thickness(18) };
            panel.Children.Add(RowText("HTTPS console address", true, false));
            var field = new TextBox { MaxLength = 2048, Margin = new Thickness(0, 8, 0, 7) };
            AutomationProperties.SetName(field, "HTTPS console address");
            var hint = RowText("Enter the organization’s console URL without credentials, query parameters or a fragment.", false, false);
            panel.Children.Add(field); panel.Children.Add(hint);
            var buttons = new WrapPanel { HorizontalAlignment = HorizontalAlignment.Right, Margin = new Thickness(0, 12, 0, 0) };
            var cancel = new Button { Content = "Cancel", IsCancel = true, Margin = new Thickness(0, 0, 6, 0) };
            var review = new Button { Content = "Review address", IsEnabled = false, IsDefault = true };
            string selected = null;
            field.TextChanged += delegate {
                Uri uri;
                string address = field.Text.Trim();
                review.IsEnabled = address.StartsWith("https://", StringComparison.Ordinal) &&
                    Uri.TryCreate(address, UriKind.Absolute, out uri) && uri.Scheme == Uri.UriSchemeHttps &&
                    !String.IsNullOrEmpty(uri.Host) && uri.UserInfo.Length == 0 && uri.Query.Length == 0 && uri.Fragment.Length == 0 &&
                    !address.Any(c => Char.IsWhiteSpace(c) || Char.IsControl(c)) && address.IndexOfAny(new[] { '\\', '@', '"', '<', '>', '%' }) < 0;
            };
            review.Click += delegate { selected = field.Text.Trim(); dialog.DialogResult = true; };
            buttons.Children.Add(cancel); buttons.Children.Add(review); panel.Children.Add(buttons);
            dialog.Content = panel; dialog.Loaded += delegate { field.Focus(); };
            return dialog.ShowDialog() == true ? selected : null;
        }
    }
}
