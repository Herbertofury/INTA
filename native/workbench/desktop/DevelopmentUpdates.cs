using System;
using System.Diagnostics;
using System.IO;
using System.Text.RegularExpressions;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Threading;

namespace Inta.Desktop
{
    internal sealed partial class Workspace
    {
        private FileSystemWatcher updateWatcher;
        private readonly DispatcherTimer updateDebounce = new DispatcherTimer { Interval = TimeSpan.FromMilliseconds(500) };

        private void WatchInstalledBuild()
        {
            string local = Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData);
            string pointer = Path.Combine(local, @"INTA\desktop-development\active.txt");
            string launcher = Path.Combine(local, @"Programs\INTA Desktop\INTA Desktop.exe");
            if (!String.Equals(Environment.GetEnvironmentVariable("INTA_INSTALL_LAUNCHER"), launcher, StringComparison.OrdinalIgnoreCase)
                    || !String.Equals(Environment.GetEnvironmentVariable("INTA_ACTIVE_POINTER"), pointer, StringComparison.OrdinalIgnoreCase)
                    || !File.Exists(launcher)) return;
            string booted = new DirectoryInfo(baseDirectory).Name;
            if (!Regex.IsMatch(booted, "^[a-fA-F0-9]{64}$"))
            {
                try {
                    string manifest = File.ReadAllText(Path.Combine(baseDirectory, "manifest.json"));
                    booted = Json.Text(Json.Object(Json.Serializer().DeserializeObject(manifest)), "sourceDigest");
                }
                catch (IOException) { return; }
                catch (UnauthorizedAccessException) { return; }
                catch (ArgumentException) { return; }
            }
            if (!Regex.IsMatch(booted, "^[a-fA-F0-9]{64}$")) return;
            var restart = Find<Button>("Restart");
            updateDebounce.Tick += async delegate {
                updateDebounce.Stop();
                try {
                    string next = await Task.Run(() => File.ReadAllText(pointer).Trim());
                    restart.Visibility = Regex.IsMatch(next, "^[a-fA-F0-9]{64}$") && !String.Equals(next, booted, StringComparison.OrdinalIgnoreCase) ?
                        Visibility.Visible : Visibility.Collapsed;
                } catch (IOException) { }
                catch (UnauthorizedAccessException) { }
            };
            restart.Click += delegate {
                if (changingSetting || activeExports > 0) { Status("Finish the current operation before opening an update."); return; }
                try {
                    using (Process.Start(new ProcessStartInfo { FileName = launcher, WorkingDirectory = Path.GetDirectoryName(launcher), UseShellExecute = false })) { }
                    window.Close();
                } catch (Exception e) { Status("Unable to open the updated version. " + e.Message); }
            };
            try {
                Directory.CreateDirectory(Path.GetDirectoryName(pointer));
                updateWatcher = new FileSystemWatcher(Path.GetDirectoryName(pointer), Path.GetFileName(pointer));
                FileSystemEventHandler changed = delegate {
                    if (window.Dispatcher.HasShutdownStarted) return;
                    window.Dispatcher.BeginInvoke(new Action(delegate { updateDebounce.Stop(); updateDebounce.Start(); }));
                };
                updateWatcher.NotifyFilter = NotifyFilters.FileName | NotifyFilters.LastWrite | NotifyFilters.Size;
                updateWatcher.Changed += changed; updateWatcher.Created += changed;
                updateWatcher.Renamed += delegate(object sender, RenamedEventArgs e) { changed(sender, e); };
                updateWatcher.EnableRaisingEvents = true;
                updateDebounce.Start();
                window.Closed += delegate { updateWatcher.Dispose(); updateDebounce.Stop(); };
            } catch (IOException) { if (updateWatcher != null) updateWatcher.Dispose(); }
            catch (UnauthorizedAccessException) { if (updateWatcher != null) updateWatcher.Dispose(); }
        }
    }
}
