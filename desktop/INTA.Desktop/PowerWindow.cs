using System.Text.Json;
using INTA.LocalAgent;

namespace INTA.Desktop;

internal sealed record PowerRow(Guid Id, string? Name);

internal sealed record PowerSnapshot(string Status, Guid? ActiveId, PowerRow[] Plans, uint? ErrorCode)
{
    internal static PowerSnapshot Read()
    {
        using var output = new StringWriter();
        using var errors = new StringWriter();
        if (PowerPlans.Run([], output, errors) != 0)
            throw new InvalidOperationException("The local power inventory is unavailable.");
        return JsonSerializer.Deserialize<PowerSnapshot>(output.ToString(), JsonSerializerOptions.Web)
            ?? throw new InvalidDataException("No power inventory was returned.");
    }
}

internal sealed class PowerWindow : Form
{
    private readonly bool _contrast = SystemInformation.HighContrast;
    private readonly Button _refresh = new() { Text = "&Refresh", AccessibleName = "Refresh power plans", AutoSize = true };
    private readonly Label _status = new() { AutoSize = true, Text = "Ready to read Windows power plans." };
    private readonly DataGridView _grid = new()
    {
        Dock = DockStyle.Fill,
        ReadOnly = true,
        AllowUserToAddRows = false,
        AllowUserToDeleteRows = false,
        RowHeadersVisible = false,
        AccessibleName = "Installed Windows power plans",
        SelectionMode = DataGridViewSelectionMode.FullRowSelect,
        AutoSizeColumnsMode = DataGridViewAutoSizeColumnsMode.Fill,
    };
    private int _scans;

    internal PowerWindow()
    {
        Text = "INTA Desktop · Power plans";
        StartPosition = FormStartPosition.CenterParent;
        Width = 900;
        Height = 610;
        MinimumSize = new Size(700, 450);
        Font = new Font("Segoe UI", 10F);
        BackColor = _contrast ? SystemColors.Window : Color.FromArgb(17, 19, 25);
        ForeColor = _contrast ? SystemColors.WindowText : Color.FromArgb(238, 242, 248);

        var layout = new TableLayoutPanel { Dock = DockStyle.Fill, Padding = new Padding(24), ColumnCount = 1, RowCount = 5 };
        layout.RowStyles.Add(new RowStyle(SizeType.AutoSize));
        layout.RowStyles.Add(new RowStyle(SizeType.AutoSize));
        layout.RowStyles.Add(new RowStyle(SizeType.AutoSize));
        layout.RowStyles.Add(new RowStyle(SizeType.Percent, 100));
        layout.RowStyles.Add(new RowStyle(SizeType.AutoSize));
        layout.Controls.Add(new Label { Text = "Power plans", AutoSize = true, Font = new Font(Font.FontFamily, 26F, FontStyle.Bold) }, 0, 0);
        layout.Controls.Add(new Label { Text = "LOCAL ONLY  /  READ-ONLY  /  NO POWER SETTINGS CHANGED", AutoSize = true, Margin = new Padding(3, 4, 3, 12) }, 0, 1);
        layout.Controls.Add(_refresh, 0, 2);
        layout.Controls.Add(_grid, 0, 3);
        _status.Margin = new Padding(3, 12, 3, 0);
        layout.Controls.Add(_status, 0, 4);
        Controls.Add(layout);
        AddColumn("Power plan", 48);
        AddColumn("Plan identifier", 33);
        AddColumn("Observation", 19);
        StyleControls();
        _refresh.Click += async (_, _) => await ReadAsync();
        Shown += async (_, _) => await ReadAsync();
    }

    private void AddColumn(string name, float share) => _grid.Columns.Add(new DataGridViewTextBoxColumn
    {
        Name = name, HeaderText = name, FillWeight = share, SortMode = DataGridViewColumnSortMode.Automatic
    });

    private void StyleControls()
    {
        _grid.RowTemplate.Height = 30;
        if (_contrast) return;
        var surface = Color.FromArgb(29, 33, 44);
        _grid.BackgroundColor = surface;
        _grid.EnableHeadersVisualStyles = false;
        _grid.GridColor = Color.FromArgb(54, 61, 75);
        _grid.DefaultCellStyle.BackColor = surface;
        _grid.DefaultCellStyle.ForeColor = ForeColor;
        _grid.DefaultCellStyle.SelectionBackColor = Color.FromArgb(62, 81, 119);
        _grid.DefaultCellStyle.SelectionForeColor = Color.White;
        _grid.ColumnHeadersDefaultCellStyle.BackColor = Color.FromArgb(43, 49, 66);
        _grid.ColumnHeadersDefaultCellStyle.ForeColor = ForeColor;
        _refresh.FlatStyle = FlatStyle.Flat;
        _refresh.UseVisualStyleBackColor = false;
        _refresh.BackColor = surface;
        _refresh.ForeColor = ForeColor;
    }

    private async Task ReadAsync()
    {
        if (!_refresh.Enabled) return;
        _refresh.Enabled = false;
        SetStatus("Reading the Windows power inventory…");
        try
        {
            var result = await Task.Run(PowerSnapshot.Read);
            if (IsDisposed || Disposing) return;
            _scans++;
            _grid.Rows.Clear();
            foreach (var row in result.Plans.OrderByDescending(x => x.Id == result.ActiveId)
                         .ThenBy(x => x.Name, StringComparer.CurrentCultureIgnoreCase).ThenBy(x => x.Id))
            {
                string state = result.Status == "ready" && row.Id == result.ActiveId ? "Active"
                    : result.Status == "ready" ? "Installed" : "State not confirmed";
                _grid.Rows.Add(row.Name ?? "Unnamed plan", row.Id.ToString("D"), state);
            }
            string text = result.Status switch
            {
                "ready" => $"Active plan: {result.ActiveId:D} · {result.Plans.Length} plans · Scan {_scans}",
                "incomplete" => "Windows returned an incomplete list. Active state not confirmed.",
                "unstable" => "The active plan changed during the scan. Refresh before relying on this list.",
                _ => "Windows did not make power plan details available.",
            };
            SetStatus(text);
        }
        catch (Exception e) when (e is IOException or InvalidOperationException or JsonException)
        {
            if (!IsDisposed && !Disposing)
            {
                _grid.Rows.Clear();
                SetStatus("Unable to read Windows power plans.");
            }
        }
        finally
        {
            if (!IsDisposed && !Disposing) _refresh.Enabled = true;
        }
    }

    private void SetStatus(string value)
    {
        _status.Text = value;
        _status.AccessibleName = value;
    }
}
