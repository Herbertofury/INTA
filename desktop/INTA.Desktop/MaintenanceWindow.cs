using System.Windows.Forms.Automation;

namespace INTA.Desktop;

internal sealed class MaintenanceWindow : Form
{
    private readonly bool _contrast = SystemInformation.HighContrast;
    private readonly TableLayoutPanel _layout = new() { Dock = DockStyle.Fill, ColumnCount = 2, RowCount = 1, Margin = Padding.Empty };
    private readonly Panel _rail = new() { Dock = DockStyle.Fill, AutoScroll = true, AccessibleName = "Maintenance categories" };
    private readonly Label _title = new() { AutoSize = true, AccessibleRole = AccessibleRole.StaticText };
    private readonly Label _scope = new() { AutoSize = true, Margin = new Padding(3, 3, 3, 8) };
    private readonly TextBox _search = new() { AccessibleName = "Search tools in this category", PlaceholderText = "Search this category" };
    private readonly CheckBox _recommended = new() { AutoSize = true, Text = "INTA Recommended",
        AccessibleDescription = "Show only recommended local tools in this category. Does not change inventory or installation." };
    private readonly Button _clear = new() { AutoSize = true, Text = "Clear filters" };
    private readonly FlowLayoutPanel _actions = new() { Dock = DockStyle.Fill, AutoSize = true, WrapContents = true };
    private readonly Label _toolCount = new() { AutoSize = true, LiveSetting = AutomationLiveSetting.Polite, Margin = new Padding(3, 3, 3, 5) };
    private readonly Panel _body = new() { Dock = DockStyle.Fill };
    private readonly Dictionary<MaintenanceView, RadioButton> _destinations = [];
    private readonly Dictionary<MaintenanceTool, Form> _workspaces = [];
    private readonly Dictionary<MaintenanceView, MaintenanceTool> _lastTools = [];
    private readonly Dictionary<MaintenanceView, (string Query, bool Recommended)> _filters = [];
    private MaintenanceView? _active;
    private bool _binding, _changed;

    internal MaintenanceWindow()
    {
        Text = "INTA Desktop · Maintenance";
        StartPosition = FormStartPosition.CenterScreen;
        AutoScaleMode = AutoScaleMode.Dpi;
        ClientSize = new Size(1050, 675);
        MinimumSize = new Size(655, 500);
        Font = new Font("Segoe UI", 10F);
        BackColor = _contrast ? SystemColors.Window : Color.FromArgb(17, 19, 25);
        ForeColor = _contrast ? SystemColors.WindowText : Color.FromArgb(238, 242, 248);
        _rail.BackColor = _contrast ? SystemColors.Control : Color.FromArgb(24, 28, 38);
        _rail.TabIndex = 0;
        _rail.Padding = new Padding(7, 9, 7, 8);
        _layout.ColumnStyles.Add(new ColumnStyle(SizeType.Absolute, 195));
        _layout.ColumnStyles.Add(new ColumnStyle(SizeType.Percent, 100));
        _layout.Controls.Add(_rail, 0, 0);

        var links = new TableLayoutPanel { Dock = DockStyle.Top, AutoSize = true, ColumnCount = 1, Margin = Padding.Empty };
        links.ColumnStyles.Add(new ColumnStyle(SizeType.Percent, 100));
        var sectionFont = new Font(Font.FontFamily, 9F, FontStyle.Bold);
        string? previous = null;
        foreach (var destination in MaintenanceCatalog.Views)
        {
            if (destination.Group != previous)
            {
                links.Controls.Add(new Label { Text = destination.Group, UseMnemonic = false, AutoSize = true, Font = sectionFont,
                    Margin = new Padding(7, previous is null ? 2 : 10, 2, 3) });
                previous = destination.Group;
            }
            var target = new RadioButton
            {
                Text = destination.Name, Name = destination.Key, AccessibleName = destination.Name,
                AccessibleDescription = destination.Group, Appearance = Appearance.Button, AutoSize = true,
                TextAlign = ContentAlignment.MiddleLeft, Dock = DockStyle.Fill, TabStop = false,
                MinimumSize = new Size(0, 29), Padding = new Padding(5, 1, 2, 1),
                Margin = new Padding(1, 1, 1, 1)
            };
            if (!_contrast)
            {
                target.FlatStyle = FlatStyle.Flat;
                target.UseVisualStyleBackColor = false;
                target.FlatAppearance.BorderColor = _rail.BackColor;
                target.FlatAppearance.CheckedBackColor = Color.FromArgb(54, 74, 112);
                target.FlatAppearance.MouseOverBackColor = Color.FromArgb(40, 51, 72);
            }
            target.CheckedChanged += (_, _) =>
            {
                target.TabStop = target.Checked;
                if (target.Checked && !_binding) Open(destination.Id, user: true);
            };
            _destinations.Add(destination.Id, target);
            links.Controls.Add(target);
        }
        _rail.Controls.Add(links);

        var workspace = new TableLayoutPanel { Dock = DockStyle.Fill, ColumnCount = 1, RowCount = 6,
            Margin = Padding.Empty, Padding = new Padding(15, 10, 14, 8), TabIndex = 1, AccessibleName = "Current maintenance view" };
        for (var row = 0; row < 5; row++) workspace.RowStyles.Add(new RowStyle(SizeType.AutoSize));
        workspace.RowStyles.Add(new RowStyle(SizeType.Percent, 100));
        _title.Font = new Font(Font.FontFamily, 20F, FontStyle.Bold);
        workspace.Controls.Add(_title, 0, 0);
        workspace.Controls.Add(_scope, 0, 1);
        var filters = new FlowLayoutPanel { AutoSize = true, Dock = DockStyle.Fill, WrapContents = true,
            Margin = Padding.Empty, Padding = new Padding(0, 0, 0, 2) };
        _search.Width = 176;
        _recommended.Margin = new Padding(8, 6, 3, 2);
        if (!_contrast) { _search.BackColor = Color.FromArgb(29, 33, 44); _search.ForeColor = ForeColor; }
        StyleAction(_clear);
        filters.Controls.AddRange([_search, _recommended, _clear]);
        workspace.Controls.Add(filters, 0, 2);
        workspace.Controls.Add(_actions, 0, 3);
        workspace.Controls.Add(_toolCount, 0, 4);
        workspace.Controls.Add(_body, 0, 5);
        workspace.Resize += (_, _) =>
        {
            int width = Math.Max(1, workspace.ClientSize.Width - workspace.Padding.Horizontal - _scope.Margin.Horizontal - 1);
            if (_scope.MaximumSize.Width != width)
                _scope.MaximumSize = _toolCount.MaximumSize = new Size(width, 0);
        };
        _layout.Controls.Add(workspace, 1, 0);
        Controls.Add(_layout);
        _search.TextChanged += (_, _) => FilterChanged();
        _recommended.CheckedChanged += (_, _) => FilterChanged();
        _clear.Click += (_, _) =>
        {
            _binding = true;
            _search.Clear();
            _recommended.Checked = false;
            _binding = false;
            FilterChanged();
            _search.Focus();
        };
        Resize += (_, _) => SizeRail();
        DpiChanged += (_, _) => SizeRail();
        SizeRail();
        Shown += async (_, _) =>
        {
            // The preference read and the embedded readers never block the UI thread.
            var restored = await Task.Run(() => NavigationPreference.Read(AppContext.BaseDirectory));
            if (!IsDisposed && !Disposing && _active is null) Open(restored, user: false);
        };
    }

    private void SizeRail()
    {
        int dpi = DeviceDpi;
        _layout.ColumnStyles[0].Width = (ClientSize.Width < 850 * dpi / 96 ? 164 : 195) * dpi / 96;
    }

    protected override bool ProcessCmdKey(ref Message msg, Keys keyData)
    {
        if (_active is { } current && _rail.ContainsFocus && keyData is Keys.Up or Keys.Down or Keys.Left or Keys.Right or Keys.Home or Keys.End)
        {
            var next = MaintenanceCatalog.Move(current, keyData switch
            {
                Keys.Home => NavigationTravel.First,
                Keys.End => NavigationTravel.Last,
                Keys.Up or Keys.Left => NavigationTravel.Previous,
                _ => NavigationTravel.Next
            });
            Open(next, user: true);
            _destinations[next].Focus();
            return true;
        }
        return base.ProcessCmdKey(ref msg, keyData);
    }

    protected override void OnFormClosing(FormClosingEventArgs e)
    {
        base.OnFormClosing(e);
        // One tiny local write, only after a manual destination change and at close.
        if (!e.Cancel && _changed && _active is { } active)
            NavigationPreference.Save(AppContext.BaseDirectory, active);
    }

    protected override void OnFormClosed(FormClosedEventArgs e)
    {
        // Embedded Storage cancels in its FormClosing handler. A plain parent
        // dispose would otherwise skip that handler on Windows.
        foreach (var child in _workspaces.Values) if (!child.IsDisposed) child.Close();
        base.OnFormClosed(e);
    }

    private void Open(MaintenanceView id, bool user)
    {
        if (_active == id) return;
        _active = id;
        _changed |= user;
        var view = MaintenanceCatalog.Get(id);
        _binding = true;
        _destinations[id].Checked = true;
        _title.Name = "heading-" + view.Key;
        _title.Text = view.Name;
        _scope.Text = view.Scope;
        var filter = _filters.GetValueOrDefault(id, (Query: "", Recommended: false));
        _search.Text = filter.Query;
        _recommended.Checked = filter.Recommended;
        _binding = false;
        _rail.ScrollControlIntoView(_destinations[id]);
        ShowTools(view);
    }

    private void FilterChanged()
    {
        if (_binding || _active is not { } id) return;
        _filters[id] = (_search.Text, _recommended.Checked);
        ShowTools(MaintenanceCatalog.Get(id));
    }

    private void ShowTools(MaintenanceDestination view)
    {
        var choices = MaintenanceCatalog.Filter(view.Id, _search.Text, _recommended.Checked);
        bool reference = MaintenanceCatalog.MatchesReference(view, _search.Text, _recommended.Checked);
        _clear.Enabled = _search.TextLength != 0 || _recommended.Checked;
        _actions.SuspendLayout();
        _body.SuspendLayout();
        try
        {
            while (_actions.Controls.Count > 0) _actions.Controls[0].Dispose();
            bool hasSelected = _lastTools.TryGetValue(view.Id, out var selected);
            var shown = hasSelected ? choices.FirstOrDefault(item => item.Id == selected) : choices.FirstOrDefault();
            if (shown is not null && !hasSelected) _lastTools[view.Id] = shown.Id;
            foreach (var tool in choices)
            {
                // A single operation is already open, so do not repeat its action.
                if (choices.Length == 1 && shown?.Id == tool.Id) continue;
                var option = new RadioButton { Text = tool.Name, AccessibleName = tool.Name, Checked = shown?.Id == tool.Id,
                    AccessibleDescription = tool.Scope, Appearance = Appearance.Button, AutoSize = true, MinimumSize = new Size(0, 27) };
                if (!_contrast)
                {
                    option.FlatStyle = FlatStyle.Flat;
                    option.UseVisualStyleBackColor = false;
                    option.FlatAppearance.CheckedBackColor = Color.FromArgb(54, 74, 112);
                    option.FlatAppearance.BorderColor = Color.FromArgb(64, 73, 95);
                }
                option.CheckedChanged += (_, _) =>
                {
                    if (!option.Checked) return;
                    _lastTools[view.Id] = tool.Id;
                    ShowWorkspace(tool.Id);
                    ShowCount(choices.Length, reference, hasHidden: false);
                };
                _actions.Controls.Add(option);
            }
            if (reference && view.Related is { } related)
            {
                var link = new LinkLabel { Text = "See " + MaintenanceCatalog.Get(related).Name + " (read-only reference)",
                    AutoSize = true, Margin = new Padding(3, 6, 3, 5) };
                if (!_contrast) { link.LinkColor = Color.FromArgb(161, 190, 249); link.ActiveLinkColor = Color.White; }
                link.LinkClicked += (_, _) => { Open(related, user: true); _destinations[related].Focus(); };
                _actions.Controls.Add(link);
            }
            _actions.Visible = _actions.Controls.Count > 0;
            ShowCount(choices.Length, reference, shown is null && choices.Length > 0);
            if (shown is not null) ShowWorkspace(shown.Id);
            else ShowReview(view);
        }
        finally { _actions.ResumeLayout(); _body.ResumeLayout(); }
    }

    private void ShowCount(int count, bool reference, bool hasHidden)
    {
        _toolCount.Text = count switch
        {
            0 when reference => "1 reference · no native tool",
            0 when _clear.Enabled => "No tools match. Clear filters to see this category's available paths.",
            0 => "No native tool yet",
            1 when hasHidden => "1 matching tool · select to open",
            1 => "1 matching tool",
            _ => count + " matching tools"
        };
    }

    private void HideWorkspace(MaintenanceTool? except)
    {
        foreach (var item in _workspaces) if (item.Key != except && item.Value.Visible) item.Value.Hide();
        foreach (var label in _body.Controls.OfType<FlowLayoutPanel>().ToArray()) label.Dispose();
    }

    private void ShowWorkspace(MaintenanceTool id)
    {
        HideWorkspace(id);
        if (!_workspaces.TryGetValue(id, out var pane))
        {
            pane = id switch
            {
                MaintenanceTool.StoragePreview => new StorageWindow(),
                MaintenanceTool.InstalledPrograms => new ProgramsWindow(),
                MaintenanceTool.StartupInventory => new StartupWindow(),
                MaintenanceTool.PowerPlans => new PowerWindow(),
                MaintenanceTool.FavoriteTweaks => new FavoriteTweaksWindow(),
                _ => throw new ArgumentOutOfRangeException(nameof(id))
            };
            pane.TopLevel = false;
            pane.AutoScaleMode = AutoScaleMode.Dpi;
            pane.FormBorderStyle = FormBorderStyle.None;
            pane.MinimumSize = Size.Empty;
            pane.Dock = DockStyle.Fill;
            // The shell provides the category heading; keep the embedded tool
            // headings compact rather than stacking two page-size titles.
            if (pane.Controls.OfType<TableLayoutPanel>().FirstOrDefault() is { } layout)
            {
                layout.Padding = new Padding(0, 2, 0, 0);
                if (layout.GetControlFromPosition(0, 0) is Label heading)
                {
                    if (id == MaintenanceTool.FavoriteTweaks) heading.Hide();
                    else heading.Font = new Font(Font.FontFamily, 14F, FontStyle.Bold);
                }
            }
            _body.Controls.Add(pane);
            _workspaces.Add(id, pane);
        }
        if (!pane.Visible) pane.Show();
    }

    private void ShowReview(MaintenanceDestination view)
    {
        HideWorkspace(null);
        if (view.Review.Length == 0) return;
        var details = new FlowLayoutPanel { Dock = DockStyle.Fill, AutoScroll = true,
            FlowDirection = FlowDirection.TopDown, WrapContents = false, TabStop = false, AccessibleName = "Review notes" };
        details.Controls.Add(new Label { Text = "What to review", AutoSize = true,
            Font = new Font(Font.FontFamily, 12F, FontStyle.Bold), Margin = new Padding(3, 6, 3, 5) });
        foreach (var text in view.Review)
            details.Controls.Add(new Label { Text = text, AutoSize = true, Margin = new Padding(3, 2, 3, 6) });
        void Fit() { foreach (Control item in details.Controls) item.MaximumSize = new Size(Math.Max(1, details.ClientSize.Width - 26), 0); }
        details.Resize += (_, _) => Fit();
        _body.Controls.Add(details);
        Fit();
    }

    private void StyleAction(Button button)
    {
        if (_contrast) return;
        button.FlatStyle = FlatStyle.Flat;
        button.UseVisualStyleBackColor = false;
        button.BackColor = Color.FromArgb(29, 33, 44);
        button.FlatAppearance.BorderColor = Color.FromArgb(64, 73, 95);
    }
}
