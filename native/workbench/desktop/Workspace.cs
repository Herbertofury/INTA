using System;
using System.Collections;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Automation;
using System.Windows.Controls;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Threading;
using Microsoft.Win32;

namespace Inta.Desktop
{
    internal sealed class Entry
    {
        public string Title { get; set; }
        public string Subtitle { get; set; }
        internal string Kind;
        internal string Id;
        internal Dictionary<string, object> Data;
        internal Workspace Owner;
        internal bool HasActions { get { return Kind == "tool" || Kind == "package"; } }
        public Visibility ToolControls { get { return HasActions ? Visibility.Visible : Visibility.Collapsed; } }
        public string InstallName { get { return "Review installation for " + Title; } }
        public string LaunchName { get { return "Review launch for " + Title; } }
        public TextBlock DisplayTitle { get { return Owner.RowText(Title, true, !HasActions); } }
        public TextBlock DisplaySubtitle { get { return Owner.RowText(Subtitle, false, true); } }
        public override string ToString() { return Title; }
    }

    internal sealed partial class Workspace
    {
        private static readonly string[] Sections = {
            "Directory", "Tools", "Playbooks", "Methods", "Diagnostics", "Scenarios",
            "Managed toolbox", "System Optimization", "Rescue media", "Virtual machines", "Local AI", "Activity"
        };
        private static readonly string[] BrushKeys = { "Canvas", "Rail", "Surface", "Hover", "Select", "Border", "Muted", "Text", "Accent" };
        private static readonly string[] Dark = { "#17191B","#111314","#222529","#30353A","#23384E","#35393E","#B0B7BE","#F1F3F5","#8FC4FF" };
        private static readonly string[] Light = { "#FFFFFF","#F2F3F5","#F7F8FA","#E7EBEF","#DCEBFA","#C7CED5","#4B5560","#18212B","#135F9F" };

        private readonly Window window;
        private readonly string baseDirectory;
        private readonly CoreClient core;
        private readonly ListBox navigation, entries;
        private readonly TextBox search;
        private readonly TextBlock heading, count, status;
        private readonly WrapPanel pageActions, detailActions;
        private readonly StackPanel detail;
        private readonly DispatcherTimer debounce;
        private CancellationTokenSource pageRequest, detailRequest;
        private string section = "Directory", toolboxRoot;
        private bool light, initializing, changingSetting, vmAvailable, repairLabAvailable, rustConsoleAvailable;
        private readonly CancellationTokenSource lifetime = new CancellationTokenSource();
        private int activeExports;
        private List<Entry> listing;
        private string pendingRecord, pendingToolAction;

        internal Workspace(Window window, string directory, ToolActionRoute route)
        {
            this.window = window;
            using (var art = typeof(Workspace).Assembly.GetManifestResourceStream("INTA.Application.png"))
            {
                if (art != null)
                {
                    var image = new System.Windows.Media.Imaging.BitmapImage();
                    image.BeginInit();
                    image.CacheOption = System.Windows.Media.Imaging.BitmapCacheOption.OnLoad;
                    image.StreamSource = art;
                    image.EndInit();
                    image.Freeze();
                    Find<Image>("ApplicationMark").Source = image;
                }
            }
            baseDirectory = directory;
            core = new CoreClient(directory);
            Find<Grid>("OpticalBackground").Children.Add(new PrismField { IsHitTestVisible = false });
            navigation = Find<ListBox>("Navigation"); entries = Find<ListBox>("Entries");
            search = Find<TextBox>("Search"); heading = Find<TextBlock>("Heading"); count = Find<TextBlock>("Count");
            status = Find<TextBlock>("Status"); pageActions = Find<WrapPanel>("PageActions");
            detailActions = Find<WrapPanel>("DetailActions"); detail = Find<StackPanel>("Detail");
            debounce = new DispatcherTimer { Interval = TimeSpan.FromMilliseconds(190) };
            debounce.Tick += delegate { debounce.Stop(); RefreshPage(); };
            navigation.ItemsSource = Sections.Where(s => s != "Virtual machines").ToArray();
            navigation.SelectionChanged += delegate {
                if (initializing || navigation.SelectedItem == null) return;
                section = (string)navigation.SelectedItem;
                pendingRecord = null; pendingToolAction = null;
                debounce.Stop();
                RefreshPage();
            };
            entries.SelectionChanged += delegate { if (!initializing) ShowSelection(); };
            entries.AddHandler(Button.ClickEvent, new RoutedEventHandler(RowActionClick));
            search.TextChanged += delegate {
                if (initializing) return;
                pendingRecord = null; pendingToolAction = null;
                if (!IsCatalog) {
                    initializing = true; section = "Directory"; navigation.SelectedItem = section; initializing = false;
                }
                Cancel(ref pageRequest); Cancel(ref detailRequest);
                count.Text = ""; Status("Searching\u2026");
                debounce.Stop(); debounce.Start();
            };
            Find<Button>("Clear").Click += delegate { search.Clear(); search.Focus(); };
            Find<Button>("Theme").Click += delegate { ApplyTheme(!light); SaveTheme(); };
            Find<Button>("Hub").Click += delegate { OpenHub(); };
            Find<Button>("Appearance").Click += delegate { OpenFeature("appearance"); };
            window.PreviewKeyDown += delegate(object sender, KeyEventArgs e) {
                if (e.Key == Key.K && Keyboard.Modifiers == ModifierKeys.Control && !changingSetting)
                {
                    if (!IsCatalog || section != "Directory") navigation.SelectedItem = "Directory";
                    search.Focus(); search.SelectAll(); e.Handled = true;
                }
                else if (e.Key == Key.Escape && search.IsKeyboardFocusWithin) { search.Clear(); e.Handled = true; }
            };
            window.Closing += delegate(object sender, System.ComponentModel.CancelEventArgs e) {
                if (changingSetting || activeExports > 0) { e.Cancel = true; Status("Finishing the current operation. Close the window once it completes."); }
                else { debounce.Stop(); Cancel(ref pageRequest); Cancel(ref detailRequest); }
            };
            window.Closed += delegate { ReleaseModelServer(); lifetime.Cancel(); lifetime.Dispose(); };
            window.Loaded += async delegate {
                ApplyTheme(ReadTheme()); WatchInstalledBuild();
                if (route == null) navigation.SelectedItem = "Directory";
                else OpenRecord("Tools", route.Id, route.Operation);
                await LoadCapabilities();
            };
        }
        private async Task LoadCapabilities()
        {
            var token = lifetime.Token;
            try {
                var version = Json.Object(await core.Read(new[] { "version" }, token));
                var advertised = Json.Get(version, "capabilities") as IList;
                vmAvailable = advertised != null && advertised.Cast<object>().Any(c => Convert.ToString(c) == "vm");
                repairLabAvailable = advertised != null && advertised.Cast<object>().Any(c => Convert.ToString(c) == "rust-repair-lab");
                rustConsoleAvailable = advertised != null && advertised.Cast<object>().Any(c => Convert.ToString(c) == "rust-rescue-console");
                if (token.IsCancellationRequested || (!vmAvailable && !repairLabAvailable && !rustConsoleAvailable)) return;
                initializing = true; navigation.ItemsSource = Sections; navigation.SelectedItem = section; initializing = false;
                if (section == "Rescue media") RefreshPage();
            } catch (OperationCanceledException) { }
            catch (Exception) { /* The catalog pane reports a missing or incompatible core. */ }
        }

        private T Find<T>(string name) where T : class { return (T)window.FindName(name); }
        private bool IsCatalog { get { return Array.IndexOf(Sections, section) <= 3; } }
        private void Status(string message) { status.Text = message; status.ToolTip = message; }
        private void ApplyTheme(bool useLight)
        {
            light = useLight;
            for (int i = 0; i < BrushKeys.Length; i++)
                window.Resources[BrushKeys[i]] = new SolidColorBrush((Color)ColorConverter.ConvertFromString((light ? Light : Dark)[i]));
            Find<Button>("Theme").Content = light ? "Use dark theme" : "Use light theme";
            var handle = new System.Windows.Interop.WindowInteropHelper(window).Handle;
            if (handle != IntPtr.Zero) { int darkCaption = light ? 0 : 1; DwmSetWindowAttribute(handle, 20, ref darkCaption, sizeof(int)); }
        }
        [System.Runtime.InteropServices.DllImport("dwmapi.dll")]
        private static extern int DwmSetWindowAttribute(IntPtr window, int attribute, ref int value, int size);
        private string ThemePath
        {
            get {
                string configured = Environment.GetEnvironmentVariable("INTA_DATA_DIR");
                if (!String.IsNullOrWhiteSpace(configured) && !Path.IsPathRooted(configured))
                    throw new IOException("INTA_DATA_DIR must be an absolute directory.");
                if (String.IsNullOrWhiteSpace(configured))
                    configured = File.Exists(Path.Combine(baseDirectory, "inta-portable.marker")) ?
                        Path.Combine(baseDirectory, "state") :
                        Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData), "INTA", "state");
                return Path.Combine(configured, "desktop-theme.txt");
            }
        }
        private bool ReadTheme()
        {
            try { return File.Exists(ThemePath) && File.ReadAllText(ThemePath).Trim() == "light"; }
            catch (IOException) { return false; }
            catch (UnauthorizedAccessException) { return false; }
        }
        private void SaveTheme()
        {
            try { Directory.CreateDirectory(Path.GetDirectoryName(ThemePath)); File.WriteAllText(ThemePath, light ? "light" : "dark"); }
            catch (Exception e) { Status("Theme changed for this session. " + e.Message); }
        }

        private static void Cancel(ref CancellationTokenSource request)
        {
            if (request != null) { request.Cancel(); request.Dispose(); request = null; }
        }
        private void ResetDetail()
        {
            detail.Children.Clear(); detailActions.Children.Clear();
            Find<ScrollViewer>("DetailScroll").ScrollToHome();
        }
        private void SetRows(IEnumerable<Entry> source)
        {
            listing = source.ToList();
            foreach (var entry in listing) entry.Owner = this;
            initializing = true;
            entries.ItemsSource = listing;
            string requested = pendingRecord, operation = pendingToolAction;
            int selected = requested == null ? (listing.Count == 0 ? -1 : 0) : listing.FindIndex(v => v.Id == requested);
            pendingRecord = null; pendingToolAction = null;
            entries.SelectedIndex = selected;
            if (selected >= 0) entries.ScrollIntoView(listing[selected]);
            initializing = false;
            count.Text = listing.Count + (listing.Count == 1 ? " entry" : " entries");
            if (selected >= 0) ShowSelection(operation);
            else if (requested != null) {
                Message("Record unavailable", "The requested identifier is absent from this catalog.");
                Status("Unknown record " + requested + ".");
            }
        }
        private Entry Current { get { return entries.SelectedItem as Entry; } }
        private void Split(bool split)
        {
            Find<ColumnDefinition>("ListColumn").Width = split ? new GridLength(2, GridUnitType.Star) : new GridLength(0);
            Find<ColumnDefinition>("GapColumn").Width = new GridLength(split ? 16 : 0);
            Find<FrameworkElement>("EntryBorder").Visibility = split ? Visibility.Visible : Visibility.Collapsed;
        }
        private Button Action(WrapPanel panel, string title, Action handler)
        {
            var button = new Button { Content = title, Margin = new Thickness(0, 0, 7, 7) };
            AutomationProperties.SetName(button, title);
            button.Click += delegate {
                try { handler(); }
                catch (Exception e) { Status(title.TrimEnd('\u2026') + " failed. " + e.GetBaseException().Message); }
            };
            panel.Children.Add(button);
            return button;
        }
        private TextBlock Paragraph(string text, string style, double size, bool bold)
        {
            var block = new TextBlock { FontSize = size, TextWrapping = TextWrapping.Wrap,
                FontWeight = bold ? FontWeights.SemiBold : FontWeights.Normal, Margin = new Thickness(0, 0, 0, bold ? 7 : 9) };
            block.SetResourceReference(TextBlock.ForegroundProperty, style);
            AddMentions(block, text);
            detail.Children.Add(block); return block;
        }
        private void Title(string text) { Paragraph(text, "Text", 20, true); }
        private void Note(string text) { Paragraph(text, "Muted", 14, false); }
        private void Message(string title, string text)
        {
            ResetDetail(); Title(title); Note(text);
        }
        private void OpenHub()
        {
            try {
                string file = Path.Combine(baseDirectory, "site", "index.html");
                if (!File.Exists(file)) throw new FileNotFoundException("The offline hub is missing from this installation.");
                Process.Start(new ProcessStartInfo { FileName = file, UseShellExecute = true });
                Status("Opened the bundled technician hub.");
            } catch (Exception e) { Status(e.Message); }
        }
        private void OpenWeb(Uri uri)
        {
            if (uri.Scheme != Uri.UriSchemeHttps && uri.Scheme != Uri.UriSchemeHttp) return;
            try { Process.Start(new ProcessStartInfo { FileName = uri.AbsoluteUri, UseShellExecute = true }); }
            catch (Exception e) { Status(e.Message); }
        }

        private async void RefreshPage()
        {
            Cancel(ref pageRequest); Cancel(ref detailRequest);
            pageRequest = new CancellationTokenSource();
            CancellationToken token = pageRequest.Token;
            initializing = true; entries.ItemsSource = null; initializing = false;
            listing = null; ResetDetail(); pageActions.Children.Clear();
            heading.Text = section == "Directory" ? "Technician directory" : section;
            count.Text = ""; Status("Loading\u2026");
            Split(IsCatalog || section == "Diagnostics" || section == "Scenarios" || section == "Managed toolbox" || section == "System Optimization");
            try
            {
                if (IsCatalog)
                {
                    string query = search.Text.Trim();
                    string required = section == "Tools" ? "tool" : section == "Playbooks" ? "job" : section == "Methods" ? "method" : null;
                    object response = await core.Read(query.Length == 0 ? new[] { "catalog" } : new[] { "catalog", query }, token);
                    if (token.IsCancellationRequested) return;
                    SetRows(Json.Array(response).Select(Json.Object).Where(d => required == null || Json.Text(d, "kind") == required)
                        .Select(d => new Entry { Title = Json.Text(d, "title"), Id = Json.Text(d, "id"), Kind = Json.Text(d, "kind"),
                            Subtitle = String.Join("  \u00b7  ", new[] { Label(Json.Text(d, "kind")), Label(Json.Text(d, "category")) }.Where(s => s.Length > 0)), Data = d }));
                    if (listing.Count == 0) {
                        Message("No matching records", query.Length == 0 ? "The catalog has no records in this section." : "Clear the search or try a shorter term.");
                        Status("Search complete.");
                    }
                }
                else await LoadOperationPage(token);
            }
            catch (OperationCanceledException) { }
            catch (Exception e) {
                if (token.IsCancellationRequested) return;
                Message("Unable to load " + section.ToLowerInvariant(), e.GetBaseException().Message);
                Action(detailActions, "Retry", RefreshPage);
                Status("The workspace could not load this view.");
            }
        }

        private void ShowSelection() { ShowSelection(null); }
        private async void ShowSelection(string operation)
        {
            Cancel(ref detailRequest);
            Entry item = Current;
            if (item == null) return;
            detailRequest = new CancellationTokenSource();
            var token = detailRequest.Token;
            ResetDetail(); Status("Loading " + item.Title + "\u2026");
            try
            {
                if (!IsCatalog) {
                    await ShowOperation(item, token);
                    if (item.Kind == "package" && !token.IsCancellationRequested) await ShowToolActions(item, operation, token);
                    return;
                }
                var data = Json.Object(await core.Read(new[] { "record", item.Kind, item.Id }, token));
                if (token.IsCancellationRequested) return;
                RenderRecord(item, data);
                Status(item.Title);
                if (item.Kind == "tool") await ShowToolActions(item, operation, token);
            }
            catch (OperationCanceledException) { }
            catch (Exception e) {
                if (token.IsCancellationRequested) return;
                Message("Unable to open this record", e.GetBaseException().Message);
                Action(detailActions, "Retry", ShowSelection);
                Status("Record unavailable.");
            }
        }

        private static string Label(string key)
        {
            switch (key) {
                case "job": return "Playbook"; case "tool": return "Tool"; case "method": return "Method";
                case "id": return "Identifier"; case "verify": return "Verification"; case "bestFor": return "Best use";
                case "mode": return "Delivery"; case "docsUrl": return "Documentation"; case "downloadUrl": return "Official download";
                case "sourceUrl": return "Source"; case "sourceUrls": return "Sources"; case "relativePath": return "Relative path";
                case "sha256": return "SHA-256"; case "cpus": return "Virtual CPUs";
                case "url": return "Reference"; case "stopIf": return "Stop conditions"; case "when": return "When to use";
                case "reviewedAt": return "Editorial review"; case "observedAt": return "Observed at";
            }
            string words = Regex.Replace(key ?? "", "([a-z0-9])([A-Z])", "$1 $2").Replace("_", " ").Replace("-", " ")
                .Replace("Gi B","GiB").Replace("Mi B","MiB").Replace("Ki B","KiB").Replace(" Utc"," UTC");
            return words.Length == 0 ? words : Char.ToUpperInvariant(words[0]) + words.Substring(1);
        }

        private void RenderRecord(Entry item, Dictionary<string, object> data)
        {
            ResetDetail(); Title(item.Title);
            string summary = Json.Text(data, "summary");
            if (summary.Length > 0) Note(summary);
            if (item.Kind == "tool") Action(detailActions, "Check in toolbox", delegate { LocatePackage(item); });
            if (item.Kind == "tool" || item.Kind == "job" || item.Kind == "method")
                Action(detailActions, "Explain locally\u2026", delegate { ReviewExplanation(item); });
            string[] priority = { "startHere", "steps", "verify", "stopIf", "bestFor", "when", "proof", "platforms", "mode", "pricing", "cautions", "evidenceStatus", "homepage", "downloadUrl", "docsUrl", "sourceUrl" };
            foreach (var pair in data.OrderBy(p => {
                int rank = Array.IndexOf(priority,p.Key); return rank < 0 ? priority.Length : rank;
            }))
            {
                if (new[] { "name", "title", "summary", "id" }.Contains(pair.Key)) continue;
                if (item.Kind == "job" && (pair.Key == "tools" || pair.Key == "related"))
                {
                    Paragraph(pair.Key == "tools" ? "Recommended tools" : "Related playbooks", "Muted", 13, true);
                    foreach (var id in Json.Array(pair.Value).Select(x => Convert.ToString(x))) {
                        string key = id, destination = pair.Key == "tools" ? "Tools" : "Playbooks";
                        if (destination == "Tools") { AddRecommendation(key); continue; }
                        var link = new Hyperlink(new Run(key)); link.SetResourceReference(Hyperlink.ForegroundProperty, "Accent");
                        link.Click += delegate { OpenRecord(destination, key); };
                        Paragraph("", "Text", 14, false).Inlines.Add(link);
                    }
                    continue;
                }
                bool sourceLinks = new[] { "homepage", "url", "docsUrl", "downloadUrl", "sourceUrl", "sourceUrls", "sources" }.Contains(pair.Key);
                Field(pair.Key, pair.Key == "mode" && Convert.ToString(pair.Value) == "installed" ? "Installed application" : pair.Value, sourceLinks);
            }
            Field("id", item.Id);
        }
        private void OpenRecord(string destination, string id, string operation = null)
        {
            initializing = true; search.Clear(); navigation.SelectedItem = destination; section = destination; initializing = false;
            pendingRecord = id; pendingToolAction = operation; debounce.Stop(); RefreshPage();
        }
        private async void LocatePackage(Entry item)
        {
            Cancel(ref detailRequest); detailRequest = new CancellationTokenSource();
            var token = detailRequest.Token;
            Status("Checking the managed toolbox\u2026");
            try {
                var result = Json.Object(await core.Read(Toolbox("list"), token));
                if (token.IsCancellationRequested) return;
                if (Json.Array(Json.Get(result, "packages")).Select(Json.Object).Any(p => Json.Text(p, "id") == item.Id)) {
                    OpenRecord("Managed toolbox", item.Id);
                } else Status(item.Title + " has no managed package in the selected toolbox. Official routes are listed in the record.");
            } catch (OperationCanceledException) { }
            catch (Exception e) { if (!token.IsCancellationRequested) Status("Toolbox unavailable. " + e.GetBaseException().Message); }
        }
        private void Field(string key, object value, bool sourceLinks = false)
        {
            if (value == null) return;
            var header = Paragraph(Label(key), "Muted", 13, true);
            header.Margin = new Thickness(0, 7, 0, 3);
            RenderValue(DisplayValue(key, value), 0, sourceLinks);
        }
        private static object DisplayValue(string key, object value)
        {
            if (!key.EndsWith("Bytes", StringComparison.Ordinal) || !(value is IConvertible) || value is string || value is bool) return value;
            double bytes = Convert.ToDouble(value);
            if (bytes < 1024) return value;
            string[] units = { "bytes", "KiB", "MiB", "GiB", "TiB", "PiB" };
            int i = 0; double size = bytes;
            while (size >= 1024 && i < units.Length - 1) { size /= 1024; i++; }
            return String.Format("{0:0.##} {1} ({2:N0} bytes)", size, units[i], bytes);
        }
        private void RenderValue(object value, int depth, bool sourceLinks = false)
        {
            var map = value as Dictionary<string, object>;
            var array = value as IList;
            if (map != null)
            {
                foreach (var pair in map) {
                    if (pair.Key == "anchor") continue;
                    if (pair.Key != "title" && pair.Key != "detail") {
                        var key = Paragraph(Label(pair.Key), "Muted", 12, true);
                        key.Margin = new Thickness(0, 1, 0, 2);
                    }
                    if (pair.Key == "title") Paragraph(Convert.ToString(pair.Value), "Text", 15, true);
                    else RenderValue(DisplayValue(pair.Key, pair.Value), depth + 1, sourceLinks);
                }
                return;
            }
            if (array != null)
            {
                if (array.Count == 0) Note("None reported.");
                else if (array.Cast<object>().All(v => v is string && ((string)v).Length < 45 && !((string)v).Contains("://")))
                    Paragraph(String.Join("  \u00b7  ", array.Cast<string>()), "Text", 14, false);
                else if (array.Count <= 40) foreach (object item in array) RenderValue(item, depth + 1, sourceLinks);
                else RenderPage(array, 0, depth + 1, sourceLinks);
                return;
            }
            string text = value == null ? "\u2014" : Convert.ToString(value, System.Globalization.CultureInfo.CurrentCulture);
            Uri uri;
            if (sourceLinks && Uri.TryCreate(text, UriKind.Absolute, out uri) && (uri.Scheme == Uri.UriSchemeHttps || uri.Scheme == Uri.UriSchemeHttp))
            {
                var block = Paragraph("", "Text", 14, false);
                var link = new Hyperlink(new Run(text)) { ToolTip = text };
                link.SetResourceReference(Hyperlink.ForegroundProperty, "Accent");
                link.Click += delegate { OpenWeb(uri); }; block.Inlines.Add(link);
            }
            else Paragraph(text, "Text", 14, false);
        }
        private void RenderPage(IList items, int start, int depth, bool sourceLinks)
        {
            int end = Math.Min(start + 40, items.Count);
            for (int i = start; i < end; i++) RenderValue(items[i], depth, sourceLinks);
            if (end >= items.Count) return;
            var more = new Button { Content = "Show more (" + (items.Count - end) + " remaining)", HorizontalAlignment = HorizontalAlignment.Left };
            detail.Children.Add(more);
            more.Click += delegate {
                int position = detail.Children.IndexOf(more);
                detail.Children.Remove(more);
                int previous = detail.Children.Count;
                RenderPage(items, end, depth, sourceLinks);
                var appended = detail.Children.Cast<UIElement>().Skip(previous).ToList();
                foreach (var element in appended) { detail.Children.Remove(element); detail.Children.Insert(position++, element); }
            };
        }
    }
}
