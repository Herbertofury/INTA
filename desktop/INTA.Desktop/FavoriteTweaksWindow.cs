using System.ComponentModel;
using System.Diagnostics;
using Inta.Desktop.Tweaks;

namespace INTA.Desktop;

internal sealed class FavoriteTweaksWindow : Form
{
    private readonly FavoriteTweaks _favorites = new(AppContext.BaseDirectory);
    private readonly bool _highContrast = SystemInformation.HighContrast;
    private readonly Version _version = Environment.OSVersion.Version;
    private readonly FlowLayoutPanel _cards = new()
    {
        Dock = DockStyle.Fill, AutoScroll = true, FlowDirection = FlowDirection.TopDown, WrapContents = false,
        AccessibleName = "Favorite tweak cards"
    };
    private readonly Label _feedback = new() { Dock = DockStyle.Fill, AutoSize = true, AccessibleName = "Favorite Tweaks status" };
    private readonly Dictionary<FavoriteTweak, Panel> _panels = new();

    internal FavoriteTweaksWindow()
    {
        Text = "INTA Desktop · System Optimization";
        StartPosition = FormStartPosition.CenterParent;
        Size = new Size(800, 615);
        MinimumSize = new Size(560, 480);
        Font = new Font("Segoe UI", 10F);
        BackColor = _highContrast ? SystemColors.Window : Color.FromArgb(17, 19, 25);
        ForeColor = _highContrast ? SystemColors.WindowText : Color.FromArgb(238, 242, 248);

        var layout = new TableLayoutPanel { Dock = DockStyle.Fill, Padding = new Padding(22), ColumnCount = 1, RowCount = 4 };
        layout.ColumnStyles.Add(new ColumnStyle(SizeType.Percent, 100));
        layout.RowStyles.Add(new RowStyle(SizeType.AutoSize));
        layout.RowStyles.Add(new RowStyle(SizeType.AutoSize));
        layout.RowStyles.Add(new RowStyle(SizeType.Percent, 100));
        layout.RowStyles.Add(new RowStyle(SizeType.AutoSize));
        layout.Controls.Add(new Label { Text = "System Optimization", AutoSize = true, Font = new Font(Font.FontFamily, 24F, FontStyle.Bold) }, 0, 0);
        layout.Controls.Add(new Label { Text = "Favorite Tweaks  /  Bookmarks stay on this device", AutoSize = true, Font = new Font(Font.FontFamily, 12F, FontStyle.Bold), Margin = new Padding(3, 4, 3, 12) }, 0, 1);
        layout.Controls.Add(_cards, 0, 2);
        _feedback.Padding = new Padding(3, 6, 3, 0);
        _feedback.Text = FavoriteTweaks.BookmarkDescription;
        layout.Controls.Add(_feedback, 0, 3);
        Controls.Add(layout);

        foreach (var card in _favorites.Rows()) AddCard(card);
        _cards.SizeChanged += (_, _) => ResizeCards();
        ResizeCards();
    }

    private void ResizeCards()
    {
        var width = Math.Max(250, _cards.ClientSize.Width - SystemInformation.VerticalScrollBarWidth - 12);
        foreach (var card in _panels.Values)
        {
            card.MinimumSize = new Size(width, 0);
            card.MaximumSize = new Size(width, 0);
        }
    }

    private void AddCard(FavoriteTweak card)
    {
        var isEndTask = card == FavoriteTweaks.TaskbarEndTask;
        var panel = new Panel { AutoSize = true, AutoSizeMode = AutoSizeMode.GrowAndShrink, Padding = new Padding(13), Margin = new Padding(0, 0, 0, 8) };
        if (!_highContrast) panel.BackColor = Color.FromArgb(29, 33, 44);
        var body = new TableLayoutPanel { Dock = DockStyle.Top, AutoSize = true, AutoSizeMode = AutoSizeMode.GrowAndShrink, ColumnCount = 1, RowCount = 4 };
        body.ColumnStyles.Add(new ColumnStyle(SizeType.Percent, 100));
        for (var i = 0; i < 4; i++) body.RowStyles.Add(new RowStyle(SizeType.AutoSize));
        body.Controls.Add(new Label { Text = card.Title, Font = new Font(Font.FontFamily, 13F, FontStyle.Bold), AutoSize = true }, 0, 0);
        var summary = new Label { Text = card.Summary, AutoSize = true, Margin = new Padding(3, 4, 3, 6) };
        var setting = new Label
        {
            Text = isEndTask ? FavoriteTweaks.EndTaskStatus(_version)
                : "Unavailable · off. Any future context-menu action needs its own explicit opt-in.",
            AutoSize = true, Margin = new Padding(3, 0, 3, 5)
        };
        body.Controls.Add(summary, 0, 1);
        body.Controls.Add(setting, 0, 2);
        var actions = new FlowLayoutPanel { Dock = DockStyle.Fill, AutoSize = true, WrapContents = true };
        if (isEndTask)
        {
            var route = FavoriteTweaks.SettingsUri(_version);
            if (route.Length != 0)
                actions.Controls.Add(ActionButton(FavoriteTweaks.SettingsAction, () =>
                    OpenRoute(route, "Opened Windows Settings. INTA did not change the preference.")));
            actions.Controls.Add(ActionButton("Microsoft guidance", () => OpenRoute(FavoriteTweaks.EndTaskHelp, "Opened Microsoft guidance.")));
        }
        var pin = new CheckBox
        {
            AutoSize = true, Checked = _favorites.Pinned(card), Text = "Pin in Favorite Tweaks",
            AccessibleName = "Pin " + card.Title + " in Favorite Tweaks", Margin = new Padding(4, 9, 4, 7)
        };
        pin.CheckedChanged += (_, _) =>
        {
            var stored = _favorites.SetPinned(card, pin.Checked);
            foreach (var next in _favorites.Rows().Reverse()) _cards.Controls.SetChildIndex(_panels[next], 0);
            _feedback.Text = stored ? "Favorite changed. No Windows setting was changed."
                : "Favorite changed for this session only. Portable storage is unavailable.";
        };
        actions.Controls.Add(pin);
        body.Controls.Add(actions, 0, 3);
        panel.ClientSizeChanged += (_, _) =>
        {
            var available = new Size(Math.Max(100, panel.ClientSize.Width - panel.Padding.Horizontal - 8), 0);
            summary.MaximumSize = available;
            setting.MaximumSize = available;
        };
        panel.Controls.Add(body);
        _panels.Add(card, panel);
        _cards.Controls.Add(panel);
    }

    private Button ActionButton(string caption, Action open)
    {
        var button = new Button { Text = caption, AccessibleName = caption, AutoSize = true, MinimumSize = new Size(0, 34) };
        if (!_highContrast)
        {
            button.FlatStyle = FlatStyle.Flat;
            button.BackColor = Color.FromArgb(43, 49, 66);
            button.ForeColor = ForeColor;
        }
        button.Click += (_, _) => open();
        return button;
    }

    private void OpenRoute(string route, string feedback)
    {
        try
        {
            using var process = Process.Start(new ProcessStartInfo { FileName = route, UseShellExecute = true });
            _feedback.Text = feedback;
        }
        catch (Exception error) when (error is Win32Exception or InvalidOperationException)
        {
            _feedback.Text = route.StartsWith("ms-settings:", StringComparison.Ordinal)
                ? "Windows Settings did not open. Go to " + FavoriteTweaks.EndTaskPath(_version) + " manually."
                : "Microsoft guidance could not open in your browser.";
        }
    }
}
