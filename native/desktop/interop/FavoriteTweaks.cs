using System;
using System.Collections.Generic;
using System.IO;
using System.Security;

namespace Inta.Desktop.Tweaks
{
    internal sealed class FavoriteTweak
    {
        internal readonly string Id;
        internal readonly string Title;
        internal readonly string Summary;
        internal readonly bool DefaultPinned;

        internal FavoriteTweak(string id, string title, string summary, bool defaultPinned)
        {
            Id = id; Title = title; Summary = summary; DefaultPinned = defaultPinned;
        }
    }

    // Favorites are INTA bookmarks only, never Windows settings. Keep this local
    // model shared by the two native surfaces and compatible with Workbench's C# compiler.
    internal sealed class FavoriteTweaks
    {
        internal static readonly FavoriteTweak TaskbarEndTask = new FavoriteTweak(
            "taskbar-end-task", "Taskbar End task",
            "Adds End task to the right-click menu of a running app on the Windows 11 taskbar. Ending a task can discard unsaved work.", true);
        internal static readonly FavoriteTweak TakeOwnership = new FavoriteTweak(
            "take-ownership", "Take Ownership menu",
            "No verified single-item handler is available in this build. No INTA context menu is installed. INTA does not change ownership or permissions.", false);
        internal const string EndTaskUnavailable = "Not available in INTA on this Windows version. Check Microsoft’s requirements.";
        internal const string SettingsAction = "Open Windows settings";
        internal const string EndTaskHelp = "https://learn.microsoft.com/en-us/windows/advanced-settings/";
        internal const string BookmarkDescription = "Pins only this card in INTA. It never changes Windows.";
        private static readonly FavoriteTweak[] Cards = { TaskbarEndTask, TakeOwnership };
        private readonly Func<string> resolveStateDirectory;
        private readonly Dictionary<FavoriteTweak, bool> session = new Dictionary<FavoriteTweak, bool>();

        internal FavoriteTweaks(string applicationDirectory)
            : this(FixedStateDirectory(Path.Combine(Path.GetFullPath(applicationDirectory), "state"))) { }

        private FavoriteTweaks(Func<string> resolveStateDirectory)
        {
            this.resolveStateDirectory = resolveStateDirectory;
        }

        private static Func<string> FixedStateDirectory(string path) { return () => path; }

        internal static FavoriteTweaks ForStateDirectory(Func<string> resolveStateDirectory)
        {
            return new FavoriteTweaks(resolveStateDirectory);
        }

        internal IEnumerable<FavoriteTweak> Rows()
        {
            foreach (var card in Cards) if (Pinned(card)) yield return card;
            foreach (var card in Cards) if (!Pinned(card)) yield return card;
        }

        internal bool Pinned(FavoriteTweak card)
        {
            string name = BookmarkName(card);
            bool value;
            if (session.TryGetValue(card, out value)) return value;
            var stateDirectory = resolveStateDirectory();
            if (stateDirectory == null) return card.DefaultPinned;
            var path = Path.Combine(stateDirectory, name);
            // These are tiny fixed records. Do not follow a redirected state
            // folder, and do not parse arbitrary data supplied in an old marker.
            try
            {
                if (!ReparsePoint(stateDirectory) && !ReparsePoint(path))
                {
                    using (var input = new FileStream(path, FileMode.Open, FileAccess.Read, FileShare.Read | FileShare.Delete))
                    {
                        if (input.Length == 1)
                        {
                            int marker = input.ReadByte();
                            if (marker == '1' || marker == '0')
                            {
                                session[card] = marker == '1';
                                return session[card];
                            }
                        }
                    }
                }
            }
            catch (IOException) { }
            catch (UnauthorizedAccessException) { }
            catch (SecurityException) { }
            session[card] = card.DefaultPinned;
            return card.DefaultPinned;
        }

        internal bool SetPinned(FavoriteTweak card, bool value)
        {
            string name = BookmarkName(card);
            session[card] = value;
            var stateDirectory = resolveStateDirectory();
            if (stateDirectory == null) return false;
            string path = Path.Combine(stateDirectory, name);
            string temporary = String.Empty;
            try
            {
                if (ReparsePoint(stateDirectory) || ReparsePoint(path)) return false;
                Directory.CreateDirectory(stateDirectory);
                temporary = Path.Combine(stateDirectory, "favorite-tweak-" + Guid.NewGuid().ToString("N") + ".tmp");
                using (var output = new FileStream(temporary, FileMode.CreateNew, FileAccess.Write, FileShare.None))
                {
                    output.WriteByte(value ? (byte)'1' : (byte)'0');
                    output.Flush(true);
                }
                if (ReparsePoint(stateDirectory) || ReparsePoint(path)) return false;
                if (File.Exists(path)) File.Replace(temporary, path, null);
                else File.Move(temporary, path);
                return true;
            }
            catch (IOException) { return false; }
            catch (UnauthorizedAccessException) { return false; }
            catch (SecurityException) { return false; }
            finally
            {
                if (temporary.Length != 0 && !ReparsePointBestEffort(stateDirectory))
                {
                    try { File.Delete(temporary); }
                    catch (IOException) { }
                    catch (UnauthorizedAccessException) { }
                    catch (SecurityException) { }
                }
            }
        }

        private static string BookmarkName(FavoriteTweak card)
        {
            if (card != TaskbarEndTask && card != TakeOwnership)
                throw new ArgumentException("This tweak is not in the local favorites catalog.", "card");
            return "favorite-tweak-" + card.Id + ".v1";
        }

        private static bool ReparsePoint(string path)
        {
            try { return (File.GetAttributes(path) & FileAttributes.ReparsePoint) != 0; }
            catch (FileNotFoundException) { return false; }
            catch (DirectoryNotFoundException) { return false; }
        }

        private static bool ReparsePointBestEffort(string path)
        {
            try { return ReparsePoint(path); }
            catch (IOException) { return true; }
            catch (UnauthorizedAccessException) { return true; }
            catch (SecurityException) { return true; }
        }

        internal static string SettingsUri(Version version)
        {
            // The documented URI lets Windows route to its own page; the option
            // can still be absent or managed by policy. No registry write or
            // optimistic Enabled status is inferred from being on Windows 11.
            return version != null && version.Major == 10 && version.Minor == 0 && version.Build >= 22621
                ? "ms-settings:developers" : String.Empty;
        }

        internal static string EndTaskPath(Version version)
        {
            if (SettingsUri(version).Length == 0) return EndTaskUnavailable;
            return version.Build >= 26200
                ? "Settings > System > Advanced > Taskbar > End Task"
                : "Settings > System > For developers > End Task";
        }

        internal static string EndTaskStatus(Version version)
        {
            return SettingsUri(version).Length == 0 ? EndTaskUnavailable
                : "Windows controls this setting. Check its current value in " + EndTaskPath(version)
                + ". If it is missing or locked, check Windows updates or your organization’s settings.";
        }
    }
}
