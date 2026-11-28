using System;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Automation;
using System.Windows.Controls;
using System.Windows.Data;
using INTA.Desktop;

namespace Inta.Desktop
{
    internal sealed partial class Workspace
    {
        private static MaintenanceIdentity MaintenanceIdentityFor(string name)
        {
            var route = MaintenanceTaxonomy.FindView(name);
            return route.HasValue ? MaintenanceTaxonomy.Get(route.Value) : null;
        }

        private sealed class MaintenanceGroup : IValueConverter
        {
            public object Convert(object value, Type target, object parameter, CultureInfo culture)
            {
                var identity = MaintenanceIdentityFor(value as string);
                return identity == null ? "Technician Desk" : identity.Group;
            }
            public object ConvertBack(object value, Type target, object parameter, CultureInfo culture)
            { return Binding.DoNothing; }
        }

        private TextBox maintenanceSearch;
        private CheckBox maintenanceRecommended;
        private bool loadingMaintenanceFilter, restoringMaintenanceSelection, maintenanceSelectionChanged;
        private string optimizationMode;
        private List<Entry> profileRows;
        private readonly Dictionary<MaintenanceView, Tuple<string, bool>> maintenanceFilters =
            new Dictionary<MaintenanceView, Tuple<string, bool>>();
        private readonly Dictionary<MaintenanceView, string> maintenanceSelections = new Dictionary<MaintenanceView, string>();
        private readonly Dictionary<string, Entry> optimizationRows = new Dictionary<string, Entry>();
        private List<Entry> maintenanceRows;
        private MaintenanceEvidence storageEvidence;
        private CancellationTokenSource storageRead;

        private static readonly string[] MaintenanceDiagnosticIds = {
            "storage", "startup", "applications", "updates", "power", "performance", "drivers", "events", "disks"
        };

        private void BindNavigation(bool showVirtualMachines)
        {
            // The Desk's existing deep-link values are unchanged. A single
            // accessible ListBox groups the additional maintenance destinations.
            var names = MaintenanceTaxonomy.Rows.Select(identity => identity.Name)
                .Concat(Sections.Where(name => showVirtualMachines || name != "Virtual machines")).ToArray();
            var view = new ListCollectionView(names);
            view.GroupDescriptions.Add(new PropertyGroupDescription(null, new MaintenanceGroup()));
            navigation.ItemsSource = view;
        }

        private void ConfigureMaintenanceSearch()
        {
            maintenanceSearch = Find<TextBox>("MaintenanceSearch");
            maintenanceRecommended = Find<CheckBox>("MaintenanceRecommended");
            maintenanceSearch.TextChanged += delegate { MaintenanceFilterChanged(); };
            maintenanceRecommended.Checked += delegate { MaintenanceFilterChanged(); };
            maintenanceRecommended.Unchecked += delegate { MaintenanceFilterChanged(); };
            AutomationProperties.SetName(maintenanceRecommended, "INTA Recommended");
        }

        private void SetMaintenanceSearch(MaintenanceIdentity identity)
        {
            loadingMaintenanceFilter = true;
            maintenanceRows = null;
            optimizationMode = null;
            search.Visibility = identity == null ? Visibility.Visible : Visibility.Collapsed;
            maintenanceSearch.Visibility = identity == null ? Visibility.Collapsed : Visibility.Visible;
            maintenanceRecommended.Visibility = maintenanceSearch.Visibility;
            Tuple<string, bool> stored;
            if (identity == null || !maintenanceFilters.TryGetValue(identity.Id, out stored))
                stored = Tuple.Create("", false);
            maintenanceSearch.Text = stored.Item1;
            maintenanceRecommended.IsChecked = stored.Item2;
            loadingMaintenanceFilter = false;
        }

        private void ClearMaintenanceFilters()
        {
            loadingMaintenanceFilter = true;
            maintenanceSearch.Clear();
            maintenanceRecommended.IsChecked = false;
            loadingMaintenanceFilter = false;
            MaintenanceFilterChanged();
            maintenanceSearch.Focus();
        }

        private void MaintenanceFilterChanged()
        {
            if (loadingMaintenanceFilter) return;
            var identity = MaintenanceIdentityFor(section);
            if (identity == null) return;
            maintenanceFilters[identity.Id] = Tuple.Create(maintenanceSearch.Text, maintenanceRecommended.IsChecked == true);
            if (maintenanceRows != null) RenderMaintenanceRows(identity);
        }

        private void OpenMaintenance(MaintenanceView destination)
        {
            var name = MaintenanceTaxonomy.Get(destination).Name;
            navigation.SelectedItem = name;
            navigation.ScrollIntoView(name);
        }

        private static Entry MaintenanceAction(string id, string title, string scope, bool recommended)
        {
            return new Entry { Kind = "maintenance", Id = id, Title = title, Subtitle = scope, RecommendedMaintenanceTool = recommended };
        }

        private static Entry MaintainDiagnostic(string id, bool recommended)
        {
            var row = Diagnostics.Single(entry => entry.Id == id);
            return new Entry { Kind = row.Kind, Id = row.Id, Title = row.Title, Subtitle = row.Subtitle,
                RecommendedMaintenanceTool = recommended };
        }

        private async Task LoadMaintenancePage(MaintenanceIdentity identity, CancellationToken token)
        {
            switch (identity.Id)
            {
                case MaintenanceView.DeepCleanup:
                    maintenanceRows = new List<Entry> { MaintenanceAction("cache-review", "Review rebuildable cache files",
                        "Selected folder only · eligible rules, exact files, reviewed quarantine and restore", true) };
                    break;
                case MaintenanceView.LargeFileCleanup:
                    maintenanceRows = new List<Entry> { MaintenanceAction("largest-evidence", "Largest files from folder analysis",
                        "Up to 25 observed paths by logical size · read-only · no removal", true) };
                    break;
                case MaintenanceView.DuplicateCleanup:
                    maintenanceRows = new List<Entry> { MaintenanceAction("duplicate-evidence", "Equal-content candidates from folder analysis",
                        "Bounded content hashes · read-only · no keeper selection or removal", true) };
                    break;
                case MaintenanceView.DiskSpaceAnalysis:
                    maintenanceRows = new List<Entry> { MaintenanceAction("space", "Analyze a local folder",
                        "Scoped on-demand scan · logical usage; not a reclaimable-space estimate", true), MaintainDiagnostic("storage", true) };
                    break;
                case MaintenanceView.AppUninstaller:
                    maintenanceRows = new List<Entry> { MaintainDiagnostic("applications", true) };
                    break;
                case MaintenanceView.StartupItems:
                    maintenanceRows = new List<Entry> { MaintainDiagnostic("startup", true),
                        MaintenanceAction("startup-review", "Manage current-user sign-in entries",
                            "Individually reviewed disable or restore for supported Windows Run registrations", false) };
                    break;
                case MaintenanceView.SystemOptimization:
                    maintenanceRows = BuildOptimizationRows();
                    RenderMaintenanceRows(identity);
                    if (optimizationRows.Count != 0) return;
                    try {
                        var result = await core.Read(new[] { "tweak", "list" }, token);
                        if (token.IsCancellationRequested) return;
                        foreach (var item in Json.Array(result).Select(Json.Object))
                        {
                            string id = Json.Text(item, "id");
                            if (id.Length != 0)
                                optimizationRows[id] = new Entry { Kind = "setting", Id = id,
                                    Title = Json.Text(item, "setting"), Subtitle = Json.Text(item, "effect"), Data = item };
                        }
                        maintenanceRows = BuildOptimizationRows();
                    }
                    catch (OperationCanceledException) { return; }
                    catch (Exception) {
                        if (!token.IsCancellationRequested)
                            Status("Favorite Tweaks and the read-only diagnostic paths are available. The Explorer settings backend did not respond.");
                        return;
                    }
                    break;
                case MaintenanceView.SystemMaintenance:
                    maintenanceRows = new List<Entry> { MaintainDiagnostic("updates", true), MaintainDiagnostic("drivers", true),
                        MaintainDiagnostic("events", true), MaintainDiagnostic("disks", false) };
                    break;
                default:
                    maintenanceRows = new List<Entry>(); break;
            }
            RenderMaintenanceRows(identity);
        }

        private List<Entry> BuildOptimizationRows()
        {
            if (optimizationMode == "favorites") return FavoriteTweakRows().ToList();
            if (optimizationMode == "profiles") return profileRows;
            return FavoriteTweakRows().Concat(new[] {
                MaintainDiagnostic("power", true), MaintainDiagnostic("performance", true),
                MaintenanceAction("favorite-tweaks", "Favorite Tweaks", "Local INTA bookmarks and Windows taskbar preferences", false),
                MaintenanceAction("setting-profiles", "Setting profiles", "Explicitly preview a reviewed setting profile", false)
            }).Concat(optimizationRows.Values.OrderBy(row => row.Title, StringComparer.CurrentCultureIgnoreCase)).ToList();
        }

        private void RenderMaintenanceRows(MaintenanceIdentity identity)
        {
            // Do not rescan or reload the underlying route when changing filters.
            string[] terms = MaintenanceTaxonomy.Terms(maintenanceSearch.Text);
            bool recommended = maintenanceRecommended.IsChecked == true;
            string remembered;
            maintenanceSelections.TryGetValue(identity.Id, out remembered);
            var before = Current;
            var visible = maintenanceRows.Where(row => (!recommended || row.RecommendedMaintenanceTool)
                && MaintenanceTaxonomy.Matches(terms, row.Title, row.Subtitle)).ToList();
            var next = before == null ? null : visible.FirstOrDefault(row => row.Id == before.Id);
            if (next == null && remembered != null) next = visible.FirstOrDefault(row => row.Id == remembered);
            if (next == null && (remembered == null || terms.Length == 0 && !recommended))
                next = visible.FirstOrDefault();
            listing = visible;
            foreach (var row in listing) row.Owner = this;

            pageActions.Children.Clear();
            if (identity.Id == MaintenanceView.SystemOptimization && optimizationMode != null)
                Action(pageActions, "All optimization settings", delegate {
                    optimizationMode = null;
                    maintenanceSelections.Remove(identity.Id);
                    maintenanceRows = BuildOptimizationRows();
                    heading.Text = identity.Name;
                    RenderMaintenanceRows(identity);
                });

            initializing = true;
            entries.ItemsSource = listing;
            entries.SelectedItem = next;
            if (next != null) entries.ScrollIntoView(next);
            initializing = false;
            count.Text = listing.Count + (listing.Count == 1 ? " tool" : " tools");
            if (next == null)
            {
                Cancel(ref detailRequest);
                if (identity.Id == MaintenanceView.PrivacyCleanup && terms.Length == 0 && !recommended)
                    Message("Privacy Cleanup isn't available yet",
                        "No browser profiles or privacy data are read or cleared here. Future reviews should separate history, cookies, signed-in sessions and passwords.");
                else Message(listing.Count == 0 ? "No tools match" : "Select a matching tool",
                    listing.Count == 0 ? "Clear the filters to see this category's available paths." : "Your previous selection is hidden by the current filter.");
                Status("Local maintenance view. No settings changed.");
            }
            else if (before == null || before.Id != next.Id || !Object.ReferenceEquals(before, next))
                ShowSelection();
        }

        private void OpenOptimizationSubview(string mode, string title)
        {
            var identity = MaintenanceTaxonomy.Get(MaintenanceView.SystemOptimization);
            optimizationMode = mode;
            maintenanceRows = BuildOptimizationRows();
            maintenanceSelections.Remove(identity.Id);
            loadingMaintenanceFilter = true;
            maintenanceSearch.Clear();
            // This is an explicit user action opening an existing subsection; no
            // available setting or result is removed from the parent catalog.
            loadingMaintenanceFilter = false;
            maintenanceFilters[identity.Id] = Tuple.Create("", maintenanceRecommended.IsChecked == true);
            heading.Text = identity.Name + " · " + title;
            initializing = true;
            entries.SelectedItem = null;
            initializing = false;
            RenderMaintenanceRows(identity);
        }

        private void RememberMaintenanceSelection(Entry item)
        {
            var identity = MaintenanceIdentityFor(section);
            if (identity != null && item != null) maintenanceSelections[identity.Id] = item.Id;
        }

        private void ShowMaintenanceOperation(Entry item)
        {
            switch (item.Id)
            {
                case "cache-review":
                    Note("The existing cache review only considers named, rebuildable files under a selected folder. Protected, open and linked files are excluded.");
                    Action(detailActions, "Review cache files…", OpenCleanupReview); break;
                case "startup-review":
                    Note("The separate review shows the exact current-user Run registration and verifies its revision. Windows policy can affect what runs.");
                    Action(detailActions, "Manage sign-in entries…", OpenStartupReview); break;
                case "setting-profiles":
                    Note("Preview the supported settings before applying. No settings change without explicit approval.");
                    Action(detailActions, "Review setting profiles", ShowProfiles); break;
                case "favorite-tweaks":
                    Note("Favorite Tweaks stays within System Optimization. INTA bookmarks do not change Windows.");
                    Action(detailActions, "Open Favorite Tweaks", ShowFavoriteTweakList); break;
                case "space": ShowSpaceAnalysis(); break;
                case "largest-evidence": ShowStorageEvidence(MaintenanceEvidence.LargestKey, "Largest files", false); break;
                case "duplicate-evidence": ShowStorageEvidence(MaintenanceEvidence.DuplicatesKey, "Equal-content candidates", true); break;
            }
            Status("Local maintenance view.");
        }

        private void ShowSpaceAnalysis()
        {
            Note("Choose one local folder. INTA reads a bounded sample on demand. This does not clean up files or estimate reclaimable space.");
            if (storageRead == null)
                Action(detailActions, "Analyze a local folder…", delegate {
                    string path = ChooseFolder("Choose a local folder for bounded, read-only space analysis.");
                    if (path != null) ReadSpaceEvidence(path);
                });
            else
            {
                Note("An explicit folder analysis is in progress. Any previous complete result remains available.");
                Action(detailActions, "Cancel analysis", delegate { if (storageRead != null) storageRead.Cancel(); });
            }
            if (storageEvidence == null) return;
            ShowStorageScope(false);
            foreach (var key in MaintenanceEvidence.DiskKeys) Field(key, storageEvidence.Get(key));
            AddExport("storage", storageEvidence.Report);
            Action(detailActions, "See largest files", delegate { OpenMaintenance(MaintenanceView.LargeFileCleanup); });
            Action(detailActions, "See equal-content candidates", delegate { OpenMaintenance(MaintenanceView.DuplicateCleanup); });
        }

        private async void ReadSpaceEvidence(string path)
        {
            if (storageRead != null) return;
            var request = CancellationTokenSource.CreateLinkedTokenSource(lifetime.Token);
            storageRead = request;
            var token = request.Token;
            ShowSelection();
            string failure = null;
            try {
                var report = Json.Object(await core.Read(new[] { "storage", path }, token));
                if (token.IsCancellationRequested) return;
                storageEvidence = MaintenanceEvidence.Read(path, report);
            }
            catch (OperationCanceledException) { }
            catch (Exception error) { if (!token.IsCancellationRequested) failure = error.GetBaseException().Message; }
            finally {
                storageRead = null;
                request.Dispose();
                if (!lifetime.IsCancellationRequested && Current != null && Current.Kind == "maintenance"
                    && (Current.Id == "space" || Current.Id == "largest-evidence" || Current.Id == "duplicate-evidence"))
                {
                    ShowSelection();
                    if (failure != null) Status("Folder analysis failed. Existing evidence was retained. " + failure);
                }
            }
        }

        private void ShowStorageScope(bool duplicate)
        {
            Note("Last completed observation · " + storageEvidence.SelectedFolder);
            Field("observedAt", storageEvidence.Get("observedAt"));
            if (Object.Equals(storageEvidence.Get("scanTruncated"), true))
                Note("Partial: the overall folder scan reached its limit.");
            if (duplicate)
            {
                if (Object.Equals(storageEvidence.Get("duplicateScanTruncated"), true))
                    Note("Partial: duplicate coverage was limited by read errors, changing inputs or the hashing budget.");
                Field("duplicateBytesRead", storageEvidence.Get("duplicateBytesRead"));
                Field("duplicateHashBudgetBytes", storageEvidence.Get("duplicateHashBudgetBytes"));
            }
            Field("readErrors", storageEvidence.Get("readErrors"));
            Field("skippedReparsePoints", storageEvidence.Get("skippedReparsePoints"));
        }

        private void ShowStorageEvidence(string key, string caption, bool duplicate)
        {
            Note(duplicate ? "Hash matches are candidates, not a promise that separate copies occupy separate space. INTA will not select a keeper or delete files here."
                : "At most 25 observed entries. Logical file size is not the space removing the file would free. No removal is available here.");
            Action(detailActions, "Go to Disk Space Analysis", delegate { OpenMaintenance(MaintenanceView.DiskSpaceAnalysis); });
            if (storageEvidence == null) { Note("Analyze a folder once in Disk Space Analysis to see its evidence here."); return; }
            ShowStorageScope(duplicate);
            var list = Json.Array(storageEvidence.Get(key));
            if (!list.Any()) Note("No matching records were observed in this scan.");
            else Field(caption, list);
        }
    }
}
