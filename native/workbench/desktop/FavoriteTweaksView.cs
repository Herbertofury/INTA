using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Windows;
using Inta.Desktop.Tweaks;

namespace Inta.Desktop
{
    internal sealed partial class Workspace
    {
        private FavoriteTweaks localFavorites;
        private FavoriteTweaks LocalFavorites
        {
            get { return localFavorites ?? (localFavorites = new FavoriteTweaks(baseDirectory)); }
        }

        private IEnumerable<Entry> FavoriteTweakRows()
        {
            return LocalFavorites.Rows().Select(card => new Entry {
                Id = card.Id, Kind = "favorite-tweak", Title = card.Title,
                Subtitle = (LocalFavorites.Pinned(card) ? "Pinned · " : "") +
                    (card == FavoriteTweaks.TaskbarEndTask ? "Windows taskbar preference" : "Unavailable · not installed")
            });
        }

        private void ShowFavoriteTweakList()
        {
            // This listing still works before (or without) the local CLI.
            Cancel(ref pageRequest);
            SetRows(FavoriteTweakRows());
            pageActions.Children.Clear();
            Action(pageActions, "All optimization settings", RefreshPage);
            heading.Text = "System Optimization · Favorite Tweaks";
        }

        private void ShowFavoriteTweak(Entry item)
        {
            var card = LocalFavorites.Rows().Single(value => value.Id == item.Id);
            var version = Environment.OSVersion.Version;
            Note(card.Summary);
            Note(card == FavoriteTweaks.TaskbarEndTask ? FavoriteTweaks.EndTaskStatus(version)
                : "Unavailable · off. A future menu requires separate opt-in for a reviewed, single-target action.");
            Note(FavoriteTweaks.BookmarkDescription);
            var settings = FavoriteTweaks.SettingsUri(version);
            if (card == FavoriteTweaks.TaskbarEndTask)
            {
                if (settings.Length != 0)
                    Action(detailActions, FavoriteTweaks.SettingsAction, delegate {
                        Process.Start(new ProcessStartInfo { FileName = settings, UseShellExecute = true });
                        Status("Opened Windows Settings. INTA did not change the preference.");
                    });
                Action(detailActions, "Microsoft guidance", delegate { OpenWeb(new Uri(FavoriteTweaks.EndTaskHelp)); });
            }
            Action(detailActions, LocalFavorites.Pinned(card) ? "Unpin from favorites" : "Pin to favorites", delegate {
                var persisted = LocalFavorites.SetPinned(card, !LocalFavorites.Pinned(card));
                var others = listing.Where(entry => entry.Kind != "favorite-tweak").ToArray();
                pendingRecord = item.Id;
                SetRows(FavoriteTweakRows().Concat(others));
                Status(persisted ? "Favorite changed. No Windows setting was changed."
                    : "Favorite changed for this session only. Portable storage is unavailable.");
            });
            Status(card == FavoriteTweaks.TaskbarEndTask ? "Local Windows preference · no automatic change."
                : "No ownership menu installed by INTA.");
        }
    }
}
