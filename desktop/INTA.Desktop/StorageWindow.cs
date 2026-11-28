using System.Globalization;
using System.Text.Json;
using INTA.LocalAgent;

namespace INTA.Desktop;

internal sealed record FolderSnapshot(
    int SchemaVersion, string ResultType, DateTimeOffset ObservedAt, int MinimumAgeDays,
    int EntryLimit, int EntriesScanned, int FilesScanned, long LogicalBytes, int OlderFiles,
    long OlderLogicalBytes, int SpecialEntriesSkipped, int ReadErrors, int DepthLimitedDirectories,
    bool EntryLimitReached, bool Partial, AllocationReceipt? Allocation)
{
    internal static FolderSnapshot Read(string folder, int days, int limit, bool allocation, CancellationToken token)
    {
        if (!Path.IsPathFullyQualified(folder))
            throw new ArgumentException("Select an absolute local folder.");
        var root = Path.GetPathRoot(folder);
        if (root is null || root.Length != 3 || root[1] != ':' ||
            new DriveInfo(root).DriveType is not (DriveType.Fixed or DriveType.Removable or DriveType.Ram))
            throw new ArgumentException("Select a folder on a local drive. Network locations are not scanned.");
        using var output = new StringWriter();
        using var error = new StringWriter();
        string[] args = [folder, days.ToString(CultureInfo.InvariantCulture), limit.ToString(CultureInfo.InvariantCulture)];
        if (StoragePreview.Run(args, output, error, allocation, token) != 0)
            throw new ArgumentException("Windows could not read this folder. Pick an ordinary existing local folder, not a drive root or link.");
        var report = JsonSerializer.Deserialize<FolderSnapshot>(output.ToString(), JsonSerializerOptions.Web);
        if (report is null || report.SchemaVersion != 1 || report.ResultType != "storage-preview")
            throw new InvalidDataException("Unexpected storage inventory result.");
        return report;
    }
}

internal sealed class StorageWindow : Form
{
    private readonly bool _contrast = SystemInformation.HighContrast;
    private readonly TextBox _folder = new() { PlaceholderText = "Choose a local folder", AccessibleName = "Local folder path", Width = 240 };
    private readonly Button _choose = new() { Text = "&Choose folder", AccessibleName = "Choose local folder", AutoSize = true };
    private readonly Button _scan = new() { Text = "&Preview", AccessibleName = "Preview storage", AutoSize = true };
    private readonly CheckBox _allocation = new() { Text = "Include observed disk allocation", AutoSize = true,
        AccessibleDescription = "Measures allocation by file identity. This does not estimate recoverable space." };
    private readonly Button _cancel = new() { Text = "Cancel scan", AutoSize = true, Enabled = false };
    private CancellationTokenSource? _scanToken;
    private readonly NumericUpDown _days = new() { Minimum = 1, Maximum = 3650, Value = 7, Width = 84, AccessibleName = "Minimum age in days" };
    private readonly NumericUpDown _limit = new() { Minimum = 1, Maximum = 1000000, Value = 100000, Width = 112, AccessibleName = "Entry scan limit", ThousandsSeparator = true };
    private readonly Label _status = new() { AutoSize = true, Text = "Choose a folder. Nothing is scanned automatically." };
    private readonly DataGridView _summary = new()
    {
        AccessibleName = "Storage preview totals",
        Dock = DockStyle.Fill, ReadOnly = true, AllowUserToAddRows = false,
        AllowUserToDeleteRows = false, RowHeadersVisible = false, SelectionMode = DataGridViewSelectionMode.FullRowSelect,
        AutoSizeColumnsMode = DataGridViewAutoSizeColumnsMode.Fill,
    };

    internal StorageWindow()
    {
        Text = "INTA Desktop · Storage preview";
        StartPosition = FormStartPosition.CenterParent;
        Width = 930;
        Height = 640;
        MinimumSize = new Size(720, 520);
        Font = new Font("Segoe UI", 10F);
        BackColor = _contrast ? SystemColors.Window : Color.FromArgb(17, 19, 25);
        ForeColor = _contrast ? SystemColors.WindowText : Color.FromArgb(238, 242, 248);
        var layout = new TableLayoutPanel { Dock = DockStyle.Fill, Padding = new Padding(24), ColumnCount = 1, RowCount = 6 };
        for (var i = 0; i < 4; i++) layout.RowStyles.Add(new RowStyle(SizeType.AutoSize));
        layout.RowStyles.Add(new RowStyle(SizeType.Percent, 100));
        layout.RowStyles.Add(new RowStyle(SizeType.AutoSize));
        layout.Controls.Add(new Label { Text = "Storage preview", AutoSize = true, Font = new Font(Font.FontFamily, 26F, FontStyle.Bold) }, 0, 0);
        layout.Controls.Add(new Label { Text = "LOCAL METADATA ONLY  /  NO CLEANUP", AutoSize = true, Margin = new Padding(3, 4, 3, 12) }, 0, 1);
        var paths = new FlowLayoutPanel { AutoSize = true, Dock = DockStyle.Fill, WrapContents = true };
        paths.Controls.AddRange([_folder, _choose, _scan, _cancel]);
        layout.Controls.Add(paths, 0, 2);
        var options = new FlowLayoutPanel { AutoSize = true, Dock = DockStyle.Fill, WrapContents = true, Padding = new Padding(0, 4, 0, 8) };
        options.Controls.AddRange([new Label { Text = "Age threshold (days)", AutoSize = true }, _days,
            new Label { Text = "Maximum entries", AutoSize = true, Margin = new Padding(12, 3, 3, 3) }, _limit, _allocation]);
        layout.Controls.Add(options, 0, 3);
        _summary.Columns.Add("Measure", "Measure");
        _summary.Columns.Add("Value", "Value");
        layout.Controls.Add(_summary, 0, 4);
        _status.Margin = new Padding(3, 12, 3, 0);
        layout.Controls.Add(_status, 0, 5);
        Controls.Add(layout);
        layout.Resize += (_, _) =>
        {
            int width = Math.Max(1, layout.ClientSize.Width - layout.Padding.Horizontal - _status.Margin.Horizontal - 1);
            if (_status.MaximumSize.Width != width) _status.MaximumSize = new Size(width, 0);
        };
        StyleControls();
        _choose.Click += (_, _) =>
        {
            using var dialog = new FolderBrowserDialog { ShowNewFolderButton = false, Description = "Choose a folder for the read-only preview." };
            if (dialog.ShowDialog(this) == DialogResult.OK) _folder.Text = dialog.SelectedPath;
        };
        _scan.Click += async (_, _) => await Scan();
        _cancel.Click += (_, _) => { _cancel.Enabled = false; _scanToken?.Cancel(); SetStatus("Canceling…"); };
        FormClosing += (_, _) => _scanToken?.Cancel();
    }

    private void StyleControls()
    {
        _summary.RowTemplate.Height = 29;
        if (_contrast) return;
        var surface = Color.FromArgb(29, 33, 44);
        _summary.BackgroundColor = surface;
        _summary.EnableHeadersVisualStyles = false;
        _summary.GridColor = Color.FromArgb(54, 61, 75);
        _summary.DefaultCellStyle.BackColor = surface;
        _summary.DefaultCellStyle.ForeColor = ForeColor;
        _summary.DefaultCellStyle.SelectionBackColor = Color.FromArgb(62, 81, 119);
        _summary.DefaultCellStyle.SelectionForeColor = Color.White;
        _summary.ColumnHeadersDefaultCellStyle.BackColor = Color.FromArgb(43, 49, 66);
        _summary.ColumnHeadersDefaultCellStyle.ForeColor = ForeColor;
        foreach (var field in new Control[] { _folder, _days, _limit, _choose, _scan, _cancel, _allocation })
        {
            field.BackColor = surface;
            field.ForeColor = ForeColor;
            if (field is Button button)
            {
                button.FlatStyle = FlatStyle.Flat;
                button.UseVisualStyleBackColor = false;
            }
        }
    }

    private async Task Scan()
    {
        if (!_scan.Enabled) return;
        string folder = _folder.Text.Trim();
        if (folder.Length == 0) { SetStatus("Choose a local folder before previewing."); return; }
        int days = (int)_days.Value, limit = (int)_limit.Value;
        bool includeAllocation = _allocation.Checked;
        using var cancellation = new CancellationTokenSource();
        _scanToken = cancellation;
        SetControls(false);
        _cancel.Enabled = true;
        _summary.Rows.Clear();
        SetStatus("Reading folder metadata…");
        try
        {
            var result = await Task.Run(() => FolderSnapshot.Read(folder, days, limit, includeAllocation, cancellation.Token), cancellation.Token);
            cancellation.Token.ThrowIfCancellationRequested();
            if (IsDisposed || Disposing) return;
            _summary.Rows.Add("Entries examined", result.EntriesScanned.ToString("N0"));
            _summary.Rows.Add("Files examined", result.FilesScanned.ToString("N0"));
            _summary.Rows.Add("Logical size (not necessarily disk allocation)", $"{result.LogicalBytes:N0} bytes");
            _summary.Rows.Add("Files meeting the age threshold", result.OlderFiles.ToString("N0"));
            _summary.Rows.Add("Logical size meeting the age threshold", $"{result.OlderLogicalBytes:N0} bytes");
            _summary.Rows.Add("Special entries skipped", result.SpecialEntriesSkipped.ToString("N0"));
            _summary.Rows.Add("Unreadable entries or folders", result.ReadErrors.ToString("N0"));
            _summary.Rows.Add("Depth-limited folders", result.DepthLimitedDirectories.ToString("N0"));
            if (result.Allocation is { } allocation)
            {
                _summary.Rows.Add("Observed allocation (once per identified file)", $"{allocation.ObservedAllocationBytes:N0} bytes");
                _summary.Rows.Add("Additional names seen for the same files", $"{allocation.RepeatedPathnames:N0}");
                _summary.Rows.Add("Files with links outside this observation", $"{allocation.FilesWithUnseenAliases:N0}");
                _summary.Rows.Add("Allocation for files with unseen names", $"{allocation.AllocationWithUnseenAliases:N0} bytes");
                _summary.Rows.Add("Allocation not measured (names / conflicting identities)",
                    $"{allocation.UnavailablePathnames:N0} / {allocation.ConflictingIdentities:N0}");
                _summary.Rows.Add("Reclaimable space", "Not estimated");
            }
            bool partial = result.Partial || result.Allocation?.Partial == true;
            SetStatus($"{(partial ? "Partial preview" : "Preview complete")}: {result.FilesScanned} files, {result.LogicalBytes} logical bytes. Nothing changed.");
        }
        catch (OperationCanceledException)
        {
            if (!IsDisposed && !Disposing) SetStatus("Preview canceled. Nothing changed.");
        }
        catch (Exception e) when (e is ArgumentException or IOException or UnauthorizedAccessException or JsonException)
        {
            if (!IsDisposed && !Disposing) SetStatus(e is ArgumentException ? e.Message : "Could not preview the selected local folder.");
        }
        finally
        {
            _scanToken = null;
            if (!IsDisposed && !Disposing) { _cancel.Enabled = false; SetControls(true); }
        }
    }

    private void SetControls(bool enabled)
    {
        foreach (var control in new Control[] { _folder, _days, _limit, _choose, _scan, _allocation }) control.Enabled = enabled;
    }

    private void SetStatus(string value)
    {
        _status.Text = value;
        _status.AccessibleName = value;
    }
}
