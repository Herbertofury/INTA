using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Globalization;
using System.IO;
using System.Runtime.InteropServices;
using System.Web.Script.Serialization;
using System.Windows;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Threading;

namespace Inta.Desktop
{
    /// <summary>Noninteractive optical layer for the existing desktop layout.</summary>
    internal sealed class PrismField : FrameworkElement
    {
        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        private delegate uint Sample(uint col, uint row, uint cols, uint rows, uint profile, ulong newest, ulong previous);
        [DllImport("kernel32.dll", CharSet = CharSet.Unicode, SetLastError = true)]
        private static extern IntPtr LoadLibraryEx(string path, IntPtr reserved, uint flags);
        [DllImport("kernel32.dll", CharSet = CharSet.Ansi, ExactSpelling = true)]
        private static extern IntPtr GetProcAddress(IntPtr module, string name);
        private static readonly Lazy<Sample> Engine = new Lazy<Sample>(LoadEngine);
        private static readonly Typeface Font = new Typeface("Consolas");
        private readonly Dictionary<int, FormattedText> glyphs = new Dictionary<int, FormattedText>();
        private readonly Stopwatch clock = Stopwatch.StartNew();
        private readonly DispatcherTimer animate, reload;
        private FileSystemWatcher watcher;
        private Window host;
        private string settingsPath;
        private int profile, density = 60, intensity = 86, strength = 85;
        private bool reduced, highContrast, light;
        private double cachedSize, cachedDpi;
        private Wave latest, previous;
        private struct Wave { public Point Point; public long Started; public byte Strength; }

        public PrismField()
        {
            IsHitTestVisible = false;
            Focusable = false;
            animate = new DispatcherTimer(DispatcherPriority.Render, Dispatcher) { Interval = TimeSpan.FromMilliseconds(34) };
            animate.Tick += delegate { if (!CanMove || clock.ElapsedMilliseconds - latest.Started >= 1450) animate.Stop(); InvalidateVisual(); };
            reload = new DispatcherTimer(DispatcherPriority.Background, Dispatcher) { Interval = TimeSpan.FromMilliseconds(100) };
            reload.Tick += delegate { reload.Stop(); ReadSettings(); };
            Loaded += Attach;
            Unloaded += Detach;
        }

        private static Sample LoadEngine()
        {
            string path = Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "inta_optics.dll");
            try
            {
                if (!File.Exists(path) || (File.GetAttributes(path) & FileAttributes.ReparsePoint) != 0) return null;
                // Resolve native imports only from the payload and Windows system directory.
                IntPtr module = LoadLibraryEx(path, IntPtr.Zero, 0x100 | 0x800);
                if (module == IntPtr.Zero) return null;
                IntPtr function = GetProcAddress(module, "inta_optics_cell");
                return function == IntPtr.Zero ? null : (Sample)Marshal.GetDelegateForFunctionPointer(function, typeof(Sample));
            }
            catch (IOException) { return null; }
            catch (UnauthorizedAccessException) { return null; }
        }

        private void Attach(object sender, RoutedEventArgs args)
        {
            host = Window.GetWindow(this);
            if (host == null) return;
            host.PreviewMouseMove += Move;
            host.PreviewMouseDown += Press;
            host.Activated += Activated;
            host.Deactivated += Deactivated;
            string root = AppDomain.CurrentDomain.BaseDirectory;
            settingsPath = File.Exists(Path.Combine(root, "inta-portable.marker"))
                ? Path.Combine(root, "state", "appearance.json")
                : Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData), "INTA", "Desktop", "appearance.json");
            string stateRoot = Environment.GetEnvironmentVariable("INTA_DATA_DIR");
            if (!String.IsNullOrWhiteSpace(stateRoot) && Path.IsPathRooted(stateRoot) && Path.GetPathRoot(stateRoot).Length > 2)
                settingsPath = Path.Combine(stateRoot, "appearance.json");
            ReadSettings();
            try
            {
                string folder = Path.GetDirectoryName(settingsPath);
                Directory.CreateDirectory(folder);
                watcher = new FileSystemWatcher(folder, Path.GetFileName(settingsPath)) { NotifyFilter = NotifyFilters.FileName | NotifyFilters.LastWrite };
                watcher.Changed += Changed;
                watcher.Created += Changed;
                watcher.Deleted += Changed;
                watcher.Renamed += Changed;
                watcher.EnableRaisingEvents = true;
            }
            catch (IOException) { }
            catch (UnauthorizedAccessException) { }
        }

        private void Detach(object sender, RoutedEventArgs args)
        {
            if (host != null) { host.PreviewMouseMove -= Move; host.PreviewMouseDown -= Press; host.Activated -= Activated; host.Deactivated -= Deactivated; }
            if (watcher != null) { watcher.Dispose(); watcher = null; }
            host = null; reload.Stop(); animate.Stop();
        }

        private void Activated(object sender, EventArgs args) { ReadSettings(); }
        private void Deactivated(object sender, EventArgs args) { animate.Stop(); latest = previous = new Wave(); InvalidateVisual(); }
        private void Changed(object sender, FileSystemEventArgs args)
        {
            if (!Dispatcher.HasShutdownStarted) Dispatcher.BeginInvoke(new Action(delegate { if (IsLoaded) { reload.Stop(); reload.Start(); } }));
        }
        private static int Number(Dictionary<string, object> d, string key, int fallback, int min, int max)
        { object value; return d.TryGetValue(key, out value) && value is int ? Math.Max(min, Math.Min(max, (int)value)) : fallback; }
        private static bool Flag(Dictionary<string, object> d, string key)
        { object value; return d.TryGetValue(key, out value) && value is bool && (bool)value; }
        private void ReadSettings()
        {
            try
            {
                var values = new Dictionary<string, object>();
                if (File.Exists(settingsPath))
                {
                    var file = new FileInfo(settingsPath);
                    if (file.Length > 8192 || (file.Attributes & FileAttributes.ReparsePoint) != 0) return;
                    using (var source = new FileStream(settingsPath, FileMode.Open, FileAccess.Read, FileShare.ReadWrite | FileShare.Delete))
                    using (var reader = new StreamReader(source))
                        values = new JavaScriptSerializer { MaxJsonLength = 8192 }.Deserialize<Dictionary<string, object>>(reader.ReadToEnd());
                    if (values == null) return;
                }
                profile = Number(values, "preset", 0, 0, 4); density = Number(values, "density", 60, 15, 100);
                intensity = Number(values, "intensity", 86, 0, 100); strength = Number(values, "strength", 85, 0, 100);
                reduced = Flag(values, "reduced_motion"); highContrast = Flag(values, "high_contrast"); light = Flag(values, "light");
                glyphs.Clear(); if (!CanMove) animate.Stop(); InvalidateVisual();
            }
            catch (ArgumentException) { }
            catch (InvalidOperationException) { }
            catch (IOException) { }
            catch (UnauthorizedAccessException) { }
        }

        private bool CanMove { get { return IsVisible && host != null && host.IsActive && profile < 2 && strength > 0 && intensity > 0 && !reduced && !highContrast && !SystemParameters.HighContrast && SystemParameters.ClientAreaAnimation; } }
        private void Move(object sender, MouseEventArgs e)
        {
            Point point = e.GetPosition(this);
            if (clock.ElapsedMilliseconds - latest.Started >= 115 && (point - latest.Point).Length >= 12) Impulse(point, 120);
        }
        private void Press(object sender, MouseButtonEventArgs e) { Impulse(e.GetPosition(this), 255); }
        private void Impulse(Point point, int amplitude)
        {
            if (!CanMove || point.X < 0 || point.X >= ActualWidth || point.Y < 0 || point.Y >= ActualHeight) return;
            previous = latest; latest = new Wave { Point = point, Started = clock.ElapsedMilliseconds, Strength = (byte)(amplitude * strength / 100) };
            if (!animate.IsEnabled) animate.Start(); InvalidateVisual();
        }
        private ulong Word(Wave w)
        {
            long age = clock.ElapsedMilliseconds - w.Started;
            if (w.Strength == 0 || age < 0 || age >= 1450 || !CanMove) return 0;
            return (ulong)(ushort)(Math.Min(1, w.Point.X / ActualWidth) * 65535) |
                   ((ulong)(ushort)(Math.Min(1, w.Point.Y / ActualHeight) * 65535) << 16) |
                   ((ulong)age << 32) | ((ulong)w.Strength << 48);
        }
        protected override void OnRender(DrawingContext dc)
        {
            base.OnRender(dc);
            if (profile == 4 || highContrast || SystemParameters.HighContrast || intensity == 0 || ActualWidth <= 1 || ActualHeight <= 1) return;
            Sample sample = Engine.Value; if (sample == null) return;
            double dy = 32.0 - density * .15, dx = dy * .87, size = dy * .62;
            double dpi = VisualTreeHelper.GetDpi(this).PixelsPerDip;
            if (size != cachedSize || dpi != cachedDpi || glyphs.Count > 2048) { glyphs.Clear(); cachedSize = size; cachedDpi = dpi; }
            uint cols = (uint)Math.Max(1, Math.Min(180, ActualWidth / dx)), rows = (uint)Math.Max(1, Math.Min(120, ActualHeight / dy));
            ulong a = Word(latest), b = Word(previous);
            for (uint y = 0; y < rows; y++) for (uint x = 0; x < cols; x++)
            {
                uint cell = sample(x, y, cols, rows, (uint)profile, a, b); char c = (char)(cell >> 24); if (cell == 0 || c < 33) continue;
                int red = (int)((cell >> 16) & 255), green = (int)((cell >> 8) & 255), blue = (int)(cell & 255);
                // Quantize the purely decorative colors to keep text caching bounded on movement.
                int key = (c << 12) | ((red >> 4) << 8) | ((green >> 4) << 4) | (blue >> 4);
                FormattedText glyph; if (!glyphs.TryGetValue(key, out glyph))
                {
                    var brush = new SolidColorBrush(Color.FromArgb((byte)(intensity * (light ? 1 : 2)), (byte)red, (byte)green, (byte)blue)); brush.Freeze();
                    glyph = new FormattedText(c.ToString(), CultureInfo.InvariantCulture, FlowDirection.LeftToRight, Font, size, brush, dpi);
                    glyphs[key] = glyph;
                }
                dc.DrawText(glyph, new Point((x + .42) * dx, (y + .2) * dy));
            }
        }
    }
}
