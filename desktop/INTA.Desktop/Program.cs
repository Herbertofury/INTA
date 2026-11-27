using System.ComponentModel;
using System.Diagnostics;
using System.Text.Json;
using INTA.LocalAgent;

namespace INTA.Desktop;

internal static class Program
{
    [STAThread]
    private static void Main()
    {
        ApplicationConfiguration.Initialize();
        Application.Run(new StartupWindow());
    }
}

internal sealed record StartupRow(string Item, string Scope, string Source, string Observation);
internal sealed record StartupSnapshot(StartupRow[] Rows, int SourceCount);

internal static class StartupReader
{
    internal static StartupSnapshot Read()
    {
        using var data = new StringWriter();
        using var errors = new StringWriter();
        if (StartupInventory.Run(["--include-folders"], data, errors) != 0)
            throw new InvalidOperationException("The local startup inventory is unavailable.");

        using var report = JsonDocument.Parse(data.ToString());
        var root = report.RootElement;
        if (root.GetProperty("schema").GetInt32() != 2 || !root.GetProperty("readOnly").GetBoolean())
            throw new InvalidDataException("Unexpected local inventory format.");
        var rows = new List<StartupRow>();
        var sources = 0;
        foreach (var group in root.GetProperty("sources").EnumerateArray())
        {
            var view = Get(group, "view");
            Add(group, $"Registry / {Get(group, "key")} · {view}");
        }
        foreach (var group in root.GetProperty("startupFolders").EnumerateArray()) Add(group, "Startup folder");
        return new(rows.ToArray(), sources);

        void Add(JsonElement group, string source)
        {
            sources++;
            string scope = Get(group, "scope") switch
            {
                "current-user" => "Current user",
                "all-users" => "All users",
                _ => "Machine"
            };
            var entries = group.GetProperty("entryNames");
            foreach (var entry in entries.EnumerateArray())
                rows.Add(new(entry.GetString() is { Length: > 0 } name ? name : "(Unnamed)", scope, source,
                    "Present · enabled state unknown"));
            if (entries.GetArrayLength() == 0)
                rows.Add(new("No entries shown", scope, source, Get(group, "status") switch
                {
                    "read" => "Empty source",
                    "not-present" => "Source not present",
                    "location-not-scanned" => "Location not scanned",
                    _ => "Source unavailable"
                }));
            if (group.GetProperty("truncated").GetBoolean())
                rows.Add(new("More entries not shown", scope, source, "Sample limit reached"));
        }
    }

    private static string Get(JsonElement group, string name) => group.GetProperty(name).GetString() ?? "";
}

internal sealed class StartupWindow : Form
{
    private readonly bool _contrast = SystemInformation.HighContrast;
    private readonly TextBox _filter = new() { AccessibleName = "Find a startup registration", PlaceholderText = "Search names, sources, or scope", Width = 300, TabIndex = 0 };
    private readonly Button _refresh = new() { Text = "&Refresh", AccessibleName = "Refresh", AutoSize = true, TabIndex = 1 };
    private readonly Button _settings = new() { Text = "&Open Windows Startup Settings", AccessibleName = "Open Windows Startup Settings", AutoSize = true, TabIndex = 2 };
    private readonly Label _status = new() { AutoSize = true, Text = "Ready to scan on this PC.", AccessibleName = "Inventory status" };
    private readonly DataGridView _grid = new()
    {
        AccessibleName = "Read-only startup registrations",
        Dock = DockStyle.Fill,
        VirtualMode = true,
        AllowUserToAddRows = false,
        AllowUserToDeleteRows = false,
        RowHeadersVisible = false,
        ReadOnly = true,
        SelectionMode = DataGridViewSelectionMode.FullRowSelect,
        AutoSizeColumnsMode = DataGridViewAutoSizeColumnsMode.Fill,
        ClipboardCopyMode = DataGridViewClipboardCopyMode.EnableAlwaysIncludeHeaderText,
    };
    private StartupRow[] _all = [], _visible = [];
    private int _completedScans;
    private int _sortColumn = -1;
    private bool _sortDescending;
    private string _summary = "Ready to scan on this PC.";

    internal StartupWindow()
    {
        Text = "INTA Desktop · Startup inventory";
        StartPosition = FormStartPosition.CenterScreen;
        Width = 1030;
        Height = 680;
        MinimumSize = new Size(760, 500);
        Font = new Font("Segoe UI", 10F);
        BackColor = _contrast ? SystemColors.Window : Color.FromArgb(17, 19, 25);
        ForeColor = _contrast ? SystemColors.WindowText : Color.FromArgb(238, 242, 248);
        var layout = new TableLayoutPanel { Dock = DockStyle.Fill, Padding = new Padding(24), ColumnCount = 1, RowCount = 5 };
        layout.RowStyles.Add(new RowStyle(SizeType.AutoSize));
        layout.RowStyles.Add(new RowStyle(SizeType.AutoSize));
        layout.RowStyles.Add(new RowStyle(SizeType.AutoSize));
        layout.RowStyles.Add(new RowStyle(SizeType.Percent, 100));
        layout.RowStyles.Add(new RowStyle(SizeType.AutoSize));
        layout.Controls.Add(new Label { AutoSize = true, Text = "Startup inventory", Font = new Font(Font.FontFamily, 26F, FontStyle.Bold) }, 0, 0);
        layout.Controls.Add(new Label { AutoSize = true, Text = "LOCAL ONLY  /  READ-ONLY  /  REGISTRY + STARTUP FOLDERS", Margin = new Padding(3, 4, 3, 14) }, 0, 1);
        var actions = new FlowLayoutPanel { AutoSize = true, Dock = DockStyle.Fill, WrapContents = true, Padding = new Padding(0, 0, 0, 8) };
        actions.Controls.AddRange([_filter, _refresh, _settings]);
        layout.Controls.Add(actions, 0, 2);
        layout.Controls.Add(_grid, 0, 3);
        _status.Margin = new Padding(3, 12, 3, 0);
        layout.Controls.Add(_status, 0, 4);
        Controls.Add(layout);
        StyleControls();
        AddColumn("Registration", 39);
        AddColumn("Scope", 14);
        AddColumn("Source", 24);
        AddColumn("Observation", 23);
        _grid.CellValueNeeded += (_, e) =>
        {
            if (e.RowIndex < 0 || e.RowIndex >= _visible.Length) return;
            var row = _visible[e.RowIndex];
            e.Value = e.ColumnIndex switch { 0 => row.Item, 1 => row.Scope, 2 => row.Source, _ => row.Observation };
        };
        _grid.ColumnHeaderMouseClick += (_, e) =>
        {
            if (e.ColumnIndex < 0) return;
            _sortDescending = _sortColumn == e.ColumnIndex && !_sortDescending;
            _sortColumn = e.ColumnIndex;
            foreach (DataGridViewColumn column in _grid.Columns)
                column.HeaderCell.SortGlyphDirection = column.Index != _sortColumn ? SortOrder.None :
                    _sortDescending ? SortOrder.Descending : SortOrder.Ascending;
            ApplyFilter();
        };
        _filter.TextChanged += (_, _) => ApplyFilter();
        _refresh.Click += async (_, _) => await RefreshInventory();
        _settings.Click += (_, _) => OpenSettings();
        Shown += async (_, _) => await RefreshInventory();
    }

    private void AddColumn(string name, float share) => _grid.Columns.Add(new DataGridViewTextBoxColumn
    {
        Name = name, HeaderText = name, FillWeight = share, SortMode = DataGridViewColumnSortMode.Programmatic
    });

    private void StyleControls()
    {
        _grid.RowTemplate.Height = 30;
        if (_contrast) return;
        var surface = Color.FromArgb(29, 33, 44);
        var foreground = ForeColor;
        _grid.BackgroundColor = surface;
        _grid.EnableHeadersVisualStyles = false;
        _grid.GridColor = Color.FromArgb(54, 61, 75);
        _grid.DefaultCellStyle.BackColor = surface;
        _grid.DefaultCellStyle.ForeColor = foreground;
        _grid.DefaultCellStyle.SelectionBackColor = Color.FromArgb(62, 81, 119);
        _grid.DefaultCellStyle.SelectionForeColor = Color.White;
        _grid.ColumnHeadersDefaultCellStyle.BackColor = Color.FromArgb(43, 49, 66);
        _grid.ColumnHeadersDefaultCellStyle.ForeColor = foreground;
        _filter.BackColor = surface;
        _filter.ForeColor = foreground;
        foreach (var button in new[] { _refresh, _settings })
        {
            button.FlatStyle = FlatStyle.Flat;
            button.UseVisualStyleBackColor = false;
            button.BackColor = surface;
            button.ForeColor = foreground;
        }
    }

    private async Task RefreshInventory()
    {
        if (!_refresh.Enabled) return;
        _refresh.Enabled = false;
        _summary = "Reading local startup sources…";
        ApplyFilter();
        try
        {
            var next = await Task.Run(StartupReader.Read);
            if (IsDisposed || Disposing) return;
            _all = next.Rows;
            _completedScans++;
            _summary = $"Checked {next.SourceCount} sources at {DateTime.Now:t} · Scan {_completedScans} · nothing changed";
            ApplyFilter();
        }
        catch (Exception e) when (e is IOException or InvalidOperationException or JsonException)
        {
            if (!IsDisposed && !Disposing)
            {
                _summary = "Couldn’t refresh. Existing results, if any, have been kept.";
                ApplyFilter();
            }
        }
        finally
        {
            if (!IsDisposed && !Disposing) _refresh.Enabled = true;
        }
    }

    private void ApplyFilter()
    {
        var query = _filter.Text.Trim();
        bool Match(string value) => value.Contains(query, StringComparison.OrdinalIgnoreCase);
        var filtered = query.Length == 0 ? _all : _all.Where(x =>
            Match(x.Item) || Match(x.Scope) || Match(x.Source) || Match(x.Observation)).ToArray();
        string Key(StartupRow row) => _sortColumn switch
        {
            0 => row.Item, 1 => row.Scope, 2 => row.Source, _ => row.Observation
        };
        _visible = _sortColumn < 0 ? filtered : (_sortDescending
            ? filtered.OrderByDescending(Key, StringComparer.CurrentCultureIgnoreCase)
            : filtered.OrderBy(Key, StringComparer.CurrentCultureIgnoreCase)).ToArray();
        _grid.RowCount = _visible.Length;
        _grid.Invalidate();
        _status.Text = $"{_summary}  •  {_visible.Length} of {_all.Length} rows";
        _status.AccessibleName = _status.Text;
    }

    private void OpenSettings()
    {
        try
        {
            using var opened = Process.Start(new ProcessStartInfo(StartupFolders.SettingsUri) { UseShellExecute = true });
            _status.Text = "Requested Windows Startup Settings. Make any changes there, then refresh.";
        }
        catch (Exception e) when (e is Win32Exception or InvalidOperationException)
        {
            _status.Text = "Windows couldn’t open Startup Settings. Open Settings → Apps → Startup manually.";
        }
    }
}
