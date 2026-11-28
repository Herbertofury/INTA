using Microsoft.Win32;

namespace INTA.Desktop;

internal sealed record ProgramRow(string Name, string Version, string Publisher, string Scope);
internal sealed record ProgramSnapshot(ProgramRow[] Rows, int Skipped, int Errors, bool Capped);

internal static class DesktopPrograms
{
    private const string Registration = @"Software\Microsoft\Windows\CurrentVersion\Uninstall";
    private const int PerViewLimit = 4096;

    internal static ProgramSnapshot Read()
    {
        var rows = new List<ProgramRow>();
        int skipped = 0, errors = 0;
        bool capped = false;
        if (Environment.Is64BitOperatingSystem)
            Read(RegistryHive.LocalMachine, RegistryView.Registry64, "Machine · 64-bit");
        Read(RegistryHive.LocalMachine, Environment.Is64BitOperatingSystem ? RegistryView.Registry32 : RegistryView.Default,
             Environment.Is64BitOperatingSystem ? "Machine · 32-bit" : "Machine");
        Read(RegistryHive.CurrentUser, RegistryView.Default, "Current user");
        return new(rows.OrderBy(x => x.Name, StringComparer.CurrentCultureIgnoreCase).ThenBy(x => x.Scope).ToArray(), skipped, errors, capped);

        void Read(RegistryHive hive, RegistryView view, string scope)
        {
            try
            {
                using var root = RegistryKey.OpenBaseKey(hive, view);
                using var directory = root.OpenSubKey(Registration, writable: false);
                if (directory is null) return;
                var keys = directory.GetSubKeyNames();
                if (keys.Length > PerViewLimit) capped = true;
                foreach (string key in keys.Take(PerViewLimit))
                {
                    try
                    {
                        using var app = directory.OpenSubKey(key, writable: false);
                        if (app is null) { skipped++; continue; }
                        string name = Metadata(app, "DisplayName");
                        if (name.Length == 0 || app.GetValue("SystemComponent") is 1) { skipped++; continue; }
                        // Scope and display metadata only. Do not read uninstall commands, paths, or key identities.
                        rows.Add(new(name, Metadata(app, "DisplayVersion"), Metadata(app, "Publisher"), scope));
                    }
                    catch (Exception e) when (ReadFailure(e)) { errors++; }
                }
            }
            catch (Exception e) when (ReadFailure(e)) { errors++; }
        }
    }

    private static bool ReadFailure(Exception e) => e is IOException or UnauthorizedAccessException or System.Security.SecurityException;

    private static string Metadata(RegistryKey app, string name)
    {
        var raw = app.GetValue(name, null, RegistryValueOptions.DoNotExpandEnvironmentNames) as string;
        if (string.IsNullOrWhiteSpace(raw)) return "";
        var value = raw.Trim();
        if (value.Length <= 384) return value;
        var length = char.IsHighSurrogate(value[383]) ? 383 : 384;
        return value[..length] + "…";
    }
}

internal sealed class ProgramsWindow : Form
{
    private readonly TextBox _query = new() { Width = 255, AccessibleName = "Search desktop programs",
        PlaceholderText = "Search names or publishers" };
    private readonly Button _read = new() { Text = "&Refresh", AccessibleName = "Refresh desktop programs", AutoSize = true };
    private readonly Label _status = new() { Text = "Ready to read desktop program registrations.", AutoSize = true };
    private readonly DataGridView _list = new()
    {
        Dock = DockStyle.Fill, ReadOnly = true, AccessibleName = "Desktop program registrations", RowHeadersVisible = false,
        AllowUserToAddRows = false, AllowUserToDeleteRows = false, VirtualMode = true,
        AutoSizeColumnsMode = DataGridViewAutoSizeColumnsMode.Fill, SelectionMode = DataGridViewSelectionMode.FullRowSelect
    };
    private ProgramRow[] _all = [], _shown = [];
    private string _summary = "Ready";
    private int _revision, _sortColumn = -1;
    private bool _descending;

    internal ProgramsWindow()
    {
        Text = "INTA Desktop · Installed programs";
        StartPosition = FormStartPosition.CenterParent;
        Width = 980;
        Height = 650;
        MinimumSize = new Size(720, 480);
        Font = new Font("Segoe UI", 10F);
        bool contrast = SystemInformation.HighContrast;
        BackColor = contrast ? SystemColors.Window : Color.FromArgb(17, 19, 25);
        ForeColor = contrast ? SystemColors.WindowText : Color.FromArgb(238, 242, 248);
        var layout = new TableLayoutPanel { Dock = DockStyle.Fill, Padding = new Padding(24), ColumnCount = 1, RowCount = 5 };
        for (int i = 0; i < 3; i++) layout.RowStyles.Add(new RowStyle(SizeType.AutoSize));
        layout.RowStyles.Add(new RowStyle(SizeType.Percent, 100)); layout.RowStyles.Add(new RowStyle(SizeType.AutoSize));
        layout.Controls.Add(new Label { Text = "Installed programs", AutoSize = true, Font = new Font(Font.FontFamily, 26F, FontStyle.Bold) }, 0, 0);
        layout.Controls.Add(new Label { Text = "DESKTOP REGISTRATIONS ONLY  /  LOCAL  /  READ-ONLY", AutoSize = true, Margin = new Padding(3, 4, 3, 12) }, 0, 1);
        var controls = new FlowLayoutPanel { AutoSize = true, Dock = DockStyle.Fill, Padding = new Padding(0, 0, 0, 8) };
        controls.Controls.AddRange([_query, _read]); layout.Controls.Add(controls, 0, 2);
        foreach (var h in new[] { "Program", "Version", "Publisher", "Registration scope" })
            _list.Columns.Add(new DataGridViewTextBoxColumn { HeaderText = h, SortMode = DataGridViewColumnSortMode.Programmatic });
        _list.RowTemplate.Height = 29;
        if (!contrast)
        {
            var surface = Color.FromArgb(29, 33, 44);
            _list.BackgroundColor = surface; _list.EnableHeadersVisualStyles = false; _list.GridColor = Color.FromArgb(54, 61, 75);
            _list.DefaultCellStyle.BackColor = surface; _list.DefaultCellStyle.ForeColor = ForeColor;
            _list.DefaultCellStyle.SelectionBackColor = Color.FromArgb(62, 81, 119); _list.DefaultCellStyle.SelectionForeColor = Color.White;
            _list.ColumnHeadersDefaultCellStyle.BackColor = Color.FromArgb(43, 49, 66); _list.ColumnHeadersDefaultCellStyle.ForeColor = ForeColor;
            _query.BackColor = surface; _query.ForeColor = ForeColor;
            _read.FlatStyle = FlatStyle.Flat; _read.UseVisualStyleBackColor = false; _read.BackColor = surface; _read.ForeColor = ForeColor;
        }
        layout.Controls.Add(_list, 0, 3); layout.Controls.Add(_status, 0, 4); Controls.Add(layout);
        _list.CellValueNeeded += (_, e) =>
        {
            if (e.RowIndex < 0 || e.RowIndex >= _shown.Length) return;
            var row = _shown[e.RowIndex];
            e.Value = e.ColumnIndex switch { 0 => row.Name, 1 => row.Version, 2 => row.Publisher, _ => row.Scope };
        };
        _list.ColumnHeaderMouseClick += (_, e) =>
        {
            if (e.ColumnIndex < 0) return;
            _descending = _sortColumn == e.ColumnIndex && !_descending;
            _sortColumn = e.ColumnIndex;
            foreach (DataGridViewColumn column in _list.Columns)
                column.HeaderCell.SortGlyphDirection = column.Index != _sortColumn ? SortOrder.None :
                    _descending ? SortOrder.Descending : SortOrder.Ascending;
            Filter();
        };
        _query.TextChanged += (_, _) => Filter();
        _read.Click += async (_, _) => await Read();
        Shown += async (_, _) => await Read();
    }

    private async Task Read()
    {
        if (!_read.Enabled) return;
        _read.Enabled = false;
        _summary = "Reading desktop registrations…"; Filter();
        try
        {
            var data = await Task.Run(DesktopPrograms.Read);
            if (Disposing || IsDisposed) return;
            _all = data.Rows;
            _revision++;
            _summary = $"{(data.Capped || data.Errors > 0 ? "Partial inventory" : "Inventory complete")} · Scan {_revision} · {data.Skipped} hidden/unnamed or removed · {data.Errors} unreadable";
            Filter();
        }
        finally { if (!Disposing && !IsDisposed) _read.Enabled = true; }
    }

    private void Filter()
    {
        var text = _query.Text.Trim();
        bool Match(string value) => value.Contains(text, StringComparison.OrdinalIgnoreCase);
        var filtered = text.Length == 0 ? _all : _all.Where(x => Match(x.Name) || Match(x.Version) || Match(x.Publisher) || Match(x.Scope)).ToArray();
        string Key(ProgramRow x) => _sortColumn switch { 0 => x.Name, 1 => x.Version, 2 => x.Publisher, _ => x.Scope };
        _shown = _sortColumn < 0 ? filtered : (_descending ? filtered.OrderByDescending(Key, StringComparer.CurrentCultureIgnoreCase)
            : filtered.OrderBy(Key, StringComparer.CurrentCultureIgnoreCase)).ToArray();
        _list.RowCount = _shown.Length;
        _list.Invalidate();
        _status.Text = $"{_summary} · {_shown.Length} of {_all.Length} registrations";
        _status.AccessibleName = _status.Text;
    }
}
