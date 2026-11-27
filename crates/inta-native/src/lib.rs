//! First-party native application logic for Desktop and Android Mobile.
//! The rendering and interaction source is Rust; no WebView or JavaScript app is embedded.
use eframe::egui::{self, Color32, RichText, Vec2};
use inta_ui::{
    common::{a, category, matches, mode, s, safe_url, tier, txt},
    guides, App, Route,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

pub const BUNDLED_CATALOG: &str = include_str!("../../../site/catalog.json");
const PRODUCT_ID: &str = "org.inta.technician-workspace";
#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(default)]
pub struct ToolFilters {
    pub query: String,
    pub category: String,
    pub platform: String,
    pub tier: String,
    pub mode: String,
    pub sort: String,
    pub online_group: String,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct NativeState {
    pub schema_version: u32,
    pub favorites: BTreeSet<String>,
    pub checks: Value,
    pub dark: bool,
    pub gallery: bool,
    pub filters: ToolFilters,
    pub section_queries: BTreeMap<String, String>,
    pub news_read: BTreeSet<String>,
    pub news_saved: BTreeSet<String>,
    pub news_sources: BTreeSet<String>,
}
impl Default for NativeState {
    fn default() -> Self {
        Self {
            schema_version: 1,
            favorites: BTreeSet::new(),
            checks: json!({}),
            dark: true,
            gallery: false,
            filters: ToolFilters::default(),
            section_queries: BTreeMap::new(),
            news_read: BTreeSet::new(),
            news_saved: BTreeSet::new(),
            news_sources: BTreeSet::new(),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Page {
    Tools,
    Online,
    Saved,
    Playbooks,
    Checklist,
    Commands,
    Updates,
    News,
    Services,
    Knowledge,
    Resources,
    Tool,
    Job,
}
impl Page {
    fn title(self) -> &'static str {
        match self {
            Self::Tools => "Tool library",
            Self::Online => "Online tools",
            Self::Saved => "Saved tools",
            Self::Playbooks => "Job playbooks",
            Self::Checklist => "Technician checklist",
            Self::Commands => "Command Center",
            Self::Updates => "Updates & maintenance",
            Self::News => "Technician news",
            Self::Services => "Service map",
            Self::Knowledge => "Knowledge desk",
            Self::Resources => "Resources & wiki",
            Self::Tool => "Tool details",
            Self::Job => "Job guide",
        }
    }
}
const NAV: &[(Page, &str)] = &[
    (Page::Tools, "Tool library"),
    (Page::Online, "Online tools"),
    (Page::Saved, "Saved tools"),
    (Page::Checklist, "Checklist"),
    (Page::Playbooks, "Job playbooks"),
    (Page::Services, "Service map"),
    (Page::Commands, "Command Center"),
    (Page::Updates, "Updates & maintenance"),
    (Page::News, "News"),
    (Page::Knowledge, "Knowledge desk"),
    (Page::Resources, "Resources & wiki"),
];
pub struct NativeApp {
    app: App,
    state: NativeState,
    dir: PathBuf,
    page: Page,
    detail_id: String,
    notice: String,
    dirty: bool,
    save_blocked: bool,
    reset_pending: Option<String>,
    smoke_path: Option<PathBuf>,
    smoke_requested: bool,
    news_saved_only: bool,
    news_unread_only: bool,
    command_shell: String,
    command_risk: String,
    command_category: String,
    online_workflow: String,
}

pub fn configure_style(ctx: &egui::Context, dark: bool) {
    let mut style = (*ctx.style()).clone();
    let mut visuals = if dark {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };
    if dark {
        visuals.panel_fill = Color32::from_rgb(16, 17, 19);
        visuals.window_fill = Color32::from_rgb(23, 25, 28);
        visuals.extreme_bg_color = Color32::from_rgb(11, 12, 14);
        visuals.faint_bg_color = Color32::from_rgb(30, 32, 36);
        visuals.override_text_color = Some(Color32::from_rgb(240, 241, 244));
        visuals.widgets.noninteractive.bg_stroke.color = Color32::from_rgb(52, 56, 63);
        visuals.widgets.inactive.bg_fill = Color32::from_rgb(30, 32, 36);
        visuals.widgets.hovered.bg_fill = Color32::from_rgb(37, 40, 45);
        visuals.hyperlink_color = Color32::from_rgb(162, 196, 255);
        visuals.selection.bg_fill = Color32::from_rgb(32, 44, 64);
    } else {
        visuals.hyperlink_color = Color32::from_rgb(32, 78, 137);
    }
    visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(5);
    visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(5);
    visuals.widgets.active.corner_radius = egui::CornerRadius::same(5);
    style.visuals = visuals;
    style.spacing.item_spacing = Vec2::new(10.0, 8.0);
    style.spacing.button_padding = Vec2::new(12.0, 8.0);
    style.spacing.interact_size = Vec2::new(38.0, 38.0);
    style
        .text_styles
        .insert(egui::TextStyle::Body, egui::FontId::proportional(16.0));
    style
        .text_styles
        .insert(egui::TextStyle::Button, egui::FontId::proportional(14.0));
    style
        .text_styles
        .insert(egui::TextStyle::Small, egui::FontId::proportional(13.0));
    style
        .text_styles
        .insert(egui::TextStyle::Heading, egui::FontId::proportional(25.0));
    style
        .text_styles
        .insert(egui::TextStyle::Monospace, egui::FontId::monospace(13.0));
    ctx.set_style(style);
}
fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
fn writable(path: &Path) -> bool {
    fs::create_dir_all(path)
        .and_then(|_| {
            let test = path.join(".inta-write-check");
            fs::write(&test, b"")?;
            fs::remove_file(test)
        })
        .is_ok()
}
pub fn data_directory() -> (PathBuf, String) {
    if let Some(dir) = std::env::var_os("INTA_DATA_DIR") {
        return (PathBuf::from(dir), String::new());
    }
    let portable = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.join("INTA-Data")));
    if let Some(path) = portable {
        if writable(&path) {
            return (path, String::new());
        }
    }
    let base = std::env::var_os("LOCALAPPDATA")
        .or_else(|| std::env::var_os("XDG_DATA_HOME"))
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|p| PathBuf::from(p).join(".local/share")))
        .unwrap_or_else(std::env::temp_dir);
    (base.join("INTA"),"The executable folder is read-only. Local state uses your user data folder; its location is shown in Resources.".into())
}
fn load_state(dir: &Path) -> (NativeState, String, bool) {
    let path = dir.join("native-state.json");
    match fs::read(&path) {
        Ok(bytes) => match serde_json::from_slice::<NativeState>(&bytes) {
            Ok(state) if state.schema_version == 1 => (state, String::new(), false),
            _ => {
                let backup = dir.join(format!("native-state.unreadable-{}.json", timestamp()));
                match fs::rename(&path,&backup){
                Ok(())=>(NativeState::default(),format!("Previous state could not be read and was preserved at {}.",backup.display()),false),
                Err(e)=>(NativeState::default(),format!("Previous state cannot be read or preserved: {e}. Choose another INTA_DATA_DIR before saving."),true)
            }
            }
        },
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            (NativeState::default(), String::new(), false)
        }
        Err(e) => (
            NativeState::default(),
            format!(
                "State could not be read: {e}. Saving is blocked to preserve the existing file."
            ),
            true,
        ),
    }
}

pub fn save_state(dir: &Path, state: &NativeState) -> Result<(), String> {
    fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let target = dir.join("native-state.json");
    let tmp = dir.join("native-state.next.json");
    let bytes = serde_json::to_vec_pretty(state).map_err(|e| e.to_string())?;
    fs::write(&tmp, bytes).map_err(|e| e.to_string())?;
    fs::rename(tmp, target).map_err(|e| e.to_string())?;
    let identity = dir.join("inta-workspace.json");
    if !identity.exists() {
        fs::write(
            identity,
            serde_json::to_vec_pretty(&json!({"identity":PRODUCT_ID,"schemaVersion":1})).unwrap(),
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
fn catalog(dir: &Path) -> (Value, String) {
    let external = dir.join("catalog.json");
    if external.exists() {
        match fs::read(&external).ok().and_then(|b|serde_json::from_slice::<Value>(&b).ok()){
        Some(v)if v["tools"].is_array()&&v["jobs"].is_array()&&v["toolAccess"].is_object()=>(v,String::new()),_=> (serde_json::from_str(BUNDLED_CATALOG).expect("bundled catalog"),"Local catalog could not be read. The complete bundled edition is open; your local file was preserved.".into())
    }
    } else {
        (
            serde_json::from_str(BUNDLED_CATALOG).expect("bundled catalog"),
            String::new(),
        )
    }
}
impl NativeApp {
    pub fn new(cc: &eframe::CreationContext<'_>, dir: PathBuf, initial_notice: String) -> Self {
        let (mut state, state_notice, save_blocked) = load_state(&dir);
        let (data, catalog_notice) = catalog(&dir);
        let mut app = App::new(data);
        app.favorites = state.favorites.clone();
        app.checks = state.checks.clone();
        guides::migrate_checks(&mut app);
        state.checks = app.checks.clone();
        configure_style(&cc.egui_ctx, state.dark);
        Self {
            app,
            state,
            dir,
            page: match std::env::var("INTA_SMOKE_PAGE").as_deref() {
                Ok("online") => Page::Online,
                Ok("job") => Page::Job,
                Ok("commands") => Page::Commands,
                Ok("updates") => Page::Updates,
                Ok("checklist") => Page::Checklist,
                _ => Page::Tools,
            },
            detail_id: std::env::var("INTA_SMOKE_JOB").unwrap_or_else(|_| "printer-repair".into()),
            notice: [initial_notice, state_notice, catalog_notice]
                .into_iter()
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join(" "),
            dirty: false,
            save_blocked,
            reset_pending: None,
            smoke_path: std::env::var_os("INTA_SMOKE_SCREENSHOT").map(PathBuf::from),
            smoke_requested: false,
            news_saved_only: false,
            news_unread_only: false,
            command_shell: String::new(),
            command_risk: String::new(),
            command_category: String::new(),
            online_workflow: String::new(),
        }
    }
    fn favorite(&mut self, id: &str) {
        if !self.state.favorites.remove(id) {
            self.state.favorites.insert(id.into());
        }
        self.app.favorites = self.state.favorites.clone();
        self.dirty = true;
    }
    fn open_job(&mut self, id: &str) {
        self.page = Page::Job;
        self.detail_id = id.into();
    }
    fn open_tool(&mut self, id: &str) {
        self.page = Page::Tool;
        self.detail_id = id.into();
    }
    fn section_search(&mut self, ui: &mut egui::Ui) -> String {
        let key = self.page.title().to_string();
        let text = self.state.section_queries.entry(key).or_default();
        let response = ui.add(
            egui::TextEdit::singleline(text)
                .hint_text("Search this section…")
                .desired_width(ui.available_width().min(500.0)),
        );
        if response.changed() {
            self.dirty = true;
        }
        text.clone()
    }
    fn save(&mut self) {
        if self.save_blocked {
            self.notice = "Saving is blocked because the existing state could not be read or preserved. Export the checklist or choose another INTA_DATA_DIR.".into();
            self.dirty = false;
            return;
        }
        self.state.checks = self.app.checks.clone();
        match save_state(&self.dir, &self.state) {
            Ok(()) => self.dirty = false,
            Err(e) => {
                self.notice = format!(
                    "Could not save local progress: {e}. Export your checklist to preserve it."
                );
                self.dirty = false;
            }
        }
    }
    fn title(ui: &mut egui::Ui, title: &str, description: &str) {
        ui.heading(title);
        if !description.is_empty() {
            ui.label(RichText::new(description).color(ui.visuals().weak_text_color()));
        }
        ui.add_space(8.0);
    }
    fn access(&self, ui: &mut egui::Ui, tool: &Value, action: &str, primary_only: bool) {
        let record = &self.app.data["toolAccess"][s(tool, "id")];
        let official = if s(record, "officialUrl").is_empty() {
            s(tool, "homepage")
        } else {
            s(record, "officialUrl")
        };
        ui.horizontal_wrapped(|ui| {
            if safe_url(official) {
                ui.hyperlink_to("Official page", official);
            }
            let mut options = vec![&record["primary"]];
            options.extend(a(&record["alternates"]));
            if !action.is_empty() {
                options.retain(|x| s(x, "id") == action);
            } else if primary_only {
                options.truncate(1);
            }
            for item in options {
                if safe_url(s(item, "url")) {
                    ui.hyperlink_to(s(item, "label"), s(item, "url"));
                }
            }
        });
    }
    fn select(
        ui: &mut egui::Ui,
        id: &str,
        label: &str,
        value: &mut String,
        values: &BTreeSet<String>,
    ) -> bool {
        let old = value.clone();
        egui::ComboBox::from_id_salt(id)
            .selected_text(if value.is_empty() {
                label
            } else {
                value.as_str()
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(value, String::new(), label);
                for item in values {
                    ui.selectable_value(value, item.clone(), item);
                }
            });
        *value != old
    }
    fn tools(&mut self, ui: &mut egui::Ui, saved_only: bool, online: bool) {
        Self::title(
            ui,
            if saved_only {
                "Saved tools"
            } else if online {
                "Online tools"
            } else {
                "Tool library"
            },
            if online {
                "Source-reviewed services. Check submission behavior before sending files or identifiers."
            } else {
                "Search the complete toolkit, compare its purpose, then open an exact official action."
            },
        );
        if online {
            let workflows = a(&self.app.data["onlineTools"]["workflows"]).to_vec();
            ui.horizontal_wrapped(|ui| {
                for flow in &workflows {
                    if ui
                        .selectable_label(self.online_workflow == s(flow, "id"), s(flow, "title"))
                        .clicked()
                    {
                        self.online_workflow = s(flow, "id").into();
                    }
                }
            });
            if let Some(flow) = workflows
                .iter()
                .find(|f| s(f, "id") == self.online_workflow)
            {
                ui.label(RichText::new(s(flow, "title")).strong());
                ui.label(s(flow, "summary"));
                numbered(ui, &flow["steps"]);
                for id in a(&flow["toolIds"]) {
                    if let Some(tool) = self.app.tool(id.as_str().unwrap_or("")).cloned() {
                        ui.label(RichText::new(s(&tool, "name")).strong());
                        self.access(ui, &tool, "", true);
                    }
                }
                if !s(flow, "jobId").is_empty() && ui.link("Open the matching job guide").clicked()
                {
                    self.open_job(s(flow, "jobId"));
                }
                ui.separator();
            }
        }
        let before = self.state.filters.clone();
        let mut view_changed = false;
        ui.horizontal_wrapped(|ui| {
            let response = ui.add(
                egui::TextEdit::singleline(&mut self.state.filters.query)
                    .hint_text("Search name, problem or purpose…")
                    .desired_width(ui.available_width().min(460.0)),
            );
            if ui.input(|i| i.modifiers.command && i.key_pressed(egui::Key::K)) {
                response.request_focus();
            }
            view_changed |= ui
                .selectable_value(&mut self.state.gallery, false, "List")
                .changed();
            view_changed |= ui
                .selectable_value(&mut self.state.gallery, true, "Gallery")
                .changed();
        });
        let tools = self.app.tools();
        let categories = tools.iter().map(|t| s(t, "category").into()).collect();
        let platforms = tools
            .iter()
            .flat_map(|t| a(&t["platforms"]))
            .map(txt)
            .collect();
        let modes = tools.iter().map(|t| s(t, "mode").into()).collect();
        ui.horizontal_wrapped(|ui| {
            Self::select(
                ui,
                "tool-category",
                "All categories",
                &mut self.state.filters.category,
                &categories,
            );
            Self::select(
                ui,
                "tool-platform",
                "All platforms",
                &mut self.state.filters.platform,
                &platforms,
            );
            Self::select(
                ui,
                "tool-tier",
                "All recommendations",
                &mut self.state.filters.tier,
                &["core", "specialist", "evaluate"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
            );
            Self::select(
                ui,
                "tool-mode",
                "All access modes",
                &mut self.state.filters.mode,
                &modes,
            );
            Self::select(
                ui,
                "tool-sort",
                "Catalog order",
                &mut self.state.filters.sort,
                &[
                    "Name A–Z",
                    "Name Z–A",
                    "Recommendation",
                    "Reviewed",
                    "GitHub stars",
                    "Provider rating",
                    "Provider reviews",
                    "Weekly downloads",
                    "Users",
                    "Forks",
                ]
                .into_iter()
                .map(str::to_string)
                .collect(),
            );
            if ui.button("Clear filters").clicked() {
                self.state.filters = ToolFilters::default();
            }
        });
        if online {
            let groups = a(&self.app.data["onlineTools"]["groups"]);
            let selected = groups
                .iter()
                .find(|g| s(g, "id") == self.state.filters.online_group)
                .map(|g| s(g, "label"))
                .unwrap_or("All online service groups");
            egui::ComboBox::from_id_salt("online-group")
                .selected_text(selected)
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.state.filters.online_group,
                        String::new(),
                        "All online service groups",
                    );
                    for group in groups {
                        ui.selectable_value(
                            &mut self.state.filters.online_group,
                            s(group, "id").into(),
                            s(group, "label"),
                        );
                    }
                });
        }
        if before != self.state.filters || view_changed {
            self.dirty = true;
        }
        let ids = filtered_tool_ids(&self.app, &self.state.filters, saved_only, online);
        ui.label(
            RichText::new(format!(
                "{} results · {} tools in this edition",
                ids.len(),
                self.app.tools().len()
            ))
            .small(),
        );
        ui.separator();
        if ids.is_empty() {
            ui.label("No tools match these filters. Clear the filters or try another term.");
            return;
        }
        let cards: Vec<_> = ids
            .iter()
            .filter_map(|id| self.app.tool(id).cloned())
            .collect();
        if self.state.gallery && ui.available_width() > 600.0 {
            let columns = (ui.available_width() / 340.0).floor().clamp(2.0, 4.0) as usize;
            for row in cards.chunks(columns) {
                ui.columns(columns, |columns| {
                    for (i, tool) in row.iter().enumerate() {
                        egui::Frame::group(columns[i].style())
                            .inner_margin(14)
                            .show(&mut columns[i], |ui| {
                                self.tool_row(ui, tool, online);
                            });
                    }
                });
                ui.add_space(4.0);
            }
        } else {
            for tool in &cards {
                self.tool_row(ui, tool, online);
                ui.separator();
            }
        }
    }
    fn tool_row(&mut self, ui: &mut egui::Ui, tool: &Value, online: bool) {
        ui.horizontal_wrapped(|ui| {
            ui.hyperlink_to(
                RichText::new(s(tool, "name")).strong().size(17.0),
                s(tool, "homepage"),
            );
            if ui.link("Details").clicked() {
                self.open_tool(s(tool, "id"));
            }
            let saved = self.state.favorites.contains(s(tool, "id"));
            if ui
                .selectable_label(saved, if saved { "Saved" } else { "Save" })
                .clicked()
            {
                self.favorite(s(tool, "id"));
            }
        });
        ui.label(s(tool, "summary"));
        ui.label(
            RichText::new(format!(
                "{} · {} · {} · {}",
                category(s(tool, "category")),
                a(&tool["platforms"])
                    .iter()
                    .map(txt)
                    .collect::<Vec<_>>()
                    .join(", "),
                tier(s(tool, "tier")),
                mode(s(tool, "mode"))
            ))
            .small()
            .color(ui.visuals().weak_text_color()),
        );
        self.access(ui, tool, "", true);
        if online {
            if let Some(entry) = a(&self.app.data["onlineTools"]["tools"])
                .iter()
                .find(|e| s(e, "toolId") == s(tool, "id"))
            {
                for (label, key) in [
                    ("Start here", "firstAction"),
                    ("Read the result", "expectedResult"),
                    ("Access", "account"),
                    ("Submission behavior", "sharing"),
                ] {
                    if !s(entry, key).is_empty() {
                        ui.label(RichText::new(label).strong());
                        ui.label(s(entry, key));
                    }
                }
            }
        }
    }
    fn tool_detail(&mut self, ui: &mut egui::Ui) {
        let Some(tool) = self.app.tool(&self.detail_id).cloned() else {
            ui.label("Tool not found in this edition.");
            return;
        };
        if ui.button("Back to tool library").clicked() {
            self.page = Page::Tools;
        }
        Self::title(ui, s(&tool, "name"), s(&tool, "summary"));
        self.access(ui, &tool, "", false);
        let saved = self.state.favorites.contains(s(&tool, "id"));
        if ui
            .selectable_label(
                saved,
                if saved {
                    "Remove from saved tools"
                } else {
                    "Save to toolkit"
                },
            )
            .clicked()
        {
            self.favorite(s(&tool, "id"));
        }
        for (label, key) in [
            ("When to reach for it", "bestFor"),
            ("Platforms", "platforms"),
            ("How it runs", "mode"),
            ("Availability & pricing", "pricing"),
            ("Before using it", "cautions"),
            ("Tags", "tags"),
            ("Source review", "reviewedAt"),
        ] {
            field(ui, label, &tool[key]);
        }
        if let Some(online) = a(&self.app.data["onlineTools"]["tools"])
            .iter()
            .find(|e| s(e, "toolId") == s(&tool, "id"))
        {
            ui.separator();
            for (label, key) in [
                ("Start here", "firstAction"),
                ("Expected result", "expectedResult"),
                ("Account and access", "account"),
                ("Submission behavior", "sharing"),
            ] {
                field(ui, label, &online[key]);
            }
            sources(ui, &online["sources"]);
        }
        if tool["reputation"].is_object() {
            ui.separator();
            ui.label(RichText::new("Observed reputation evidence").strong());
            for (provider, record) in tool["reputation"].as_object().unwrap() {
                ui.label(format!("{provider}: {}", s(record, "status")));
                for key in [
                    "stars",
                    "forks",
                    "rating",
                    "reviewCount",
                    "users",
                    "observedAt",
                    "source",
                ] {
                    if !record[key].is_null() {
                        field(ui, key, &record[key]);
                    }
                }
                if safe_url(s(record, "url")) {
                    ui.hyperlink_to("Metric source", s(record, "url"));
                }
            }
        }
        ui.separator();
        for (label, key) in [
            ("Documentation & usage", "docsUrl"),
            ("Source or upstream reference", "sourceUrl"),
        ] {
            if safe_url(s(&tool, key)) {
                ui.hyperlink_to(label, s(&tool, key));
            }
        }
        field(ui, "Why it is in the toolkit", &tool["provenance"]);
        let related: Vec<_> = self
            .app
            .jobs()
            .iter()
            .filter(|j| {
                a(&j["tools"])
                    .iter()
                    .any(|id| id.as_str() == Some(s(&tool, "id")))
            })
            .cloned()
            .collect();
        for job in related {
            if ui
                .link(format!("Open guide: {}", s(&job, "title")))
                .clicked()
            {
                self.open_job(s(&job, "id"));
            }
        }
    }
    fn playbooks(&mut self, ui: &mut egui::Ui) {
        Self::title(
            ui,
            "Job playbooks",
            "Complete procedures, working tools, specific fixes and observable completion checks.",
        );
        let query = self.section_search(ui);
        let jobs: Vec<_> = self
            .app
            .jobs()
            .iter()
            .filter(|j| {
                matches(
                    &format!(
                        "{} {} {}",
                        j,
                        self.app.data["jobWorkspaces"]["jobs"][s(j, "id")],
                        a(&self.app.data["fieldNotes"]["notes"])
                            .iter()
                            .filter(|n| s(n, "jobId") == s(j, "id"))
                            .map(Value::to_string)
                            .collect::<String>()
                    ),
                    &query,
                )
            })
            .cloned()
            .collect();
        ui.label(format!(
            "{} of {} guides",
            jobs.len(),
            self.app.jobs().len()
        ));
        for job in jobs {
            ui.separator();
            if ui
                .link(RichText::new(s(&job, "title")).strong().size(17.0))
                .clicked()
            {
                self.open_job(s(&job, "id"));
            }
            ui.label(s(&job, "summary"));
            ui.label(
                RichText::new(format!(
                    "{} · {} · {}",
                    s(&job, "category"),
                    s(&job, "estimatedTime"),
                    s(&job, "difficulty")
                ))
                .small(),
            );
        }
    }
    fn job(&mut self, ui: &mut egui::Ui, focus: bool) {
        let id = if focus {
            "client-handoff"
        } else {
            &self.detail_id
        };
        let Some(job) = self.app.job(id).cloned() else {
            ui.label("Guide not found.");
            return;
        };
        let id = s(&job, "id").to_string();
        let ws = self.app.data["jobWorkspaces"]["jobs"][&id].clone();
        if !focus && ui.button("All playbooks").clicked() {
            self.page = Page::Playbooks;
        }
        Self::title(
            ui,
            if focus {
                "Technician checklist"
            } else {
                s(&job, "title")
            },
            if focus {
                "One pass from protecting the data to handing the computer back."
            } else {
                s(&job, "summary")
            },
        );
        let (done, total) = guides::progress(&self.app, &job, focus);
        ui.add(
            egui::ProgressBar::new(if total == 0 {
                0.0
            } else {
                done as f32 / total as f32
            })
            .text(format!("{done} of {total} checks")),
        );
        ui.horizontal_wrapped(|ui| {
            if ui.button("Export checklist").clicked() {
                self.app.route = Route::parse(if focus { "#checklist" } else { "#job" });
                let text = guides::export_job(&self.app, &id);
                let path = self
                    .dir
                    .join("exports")
                    .join(format!("INTA-{id}-{}.md", timestamp()));
                self.notice = match path
                    .parent()
                    .map(fs::create_dir_all)
                    .transpose()
                    .and_then(|_| fs::write(&path, text))
                {
                    Ok(()) => format!("Exported {}", path.display()),
                    Err(e) => format!("Could not export: {e}"),
                };
            }
            if ui.button("Copy checklist").clicked() {
                self.app.route = Route::parse(if focus { "#checklist" } else { "#job" });
                ui.ctx().copy_text(guides::export_job(&self.app, &id));
                self.notice = "Checklist copied.".into();
            }
            if ui.button("Start a fresh checklist").clicked() {
                self.reset_pending = Some(id.clone());
            }
            ui.hyperlink_to(
                "Wiki guide",
                format!("https://github.com/Herbertofury/INTA/wiki/Job-{id}"),
            );
        });
        if !focus {
            let quicks: Vec<_> = a(&self.app.data["quickStart"]["items"])
                .iter()
                .filter(|q| s(q, "jobId") == id)
                .cloned()
                .collect();
            for quick in quicks {
                ui.separator();
                ui.label(
                    RichText::new(format!("Do this first · {}", s(&quick, "label")))
                        .strong()
                        .size(18.0),
                );
                ui.label(s(&quick, "firstStep"));
                numbered(ui, &quick["quickSteps"]);
                for choice in a(&quick["choices"]) {
                    if let Some(tool) = self.app.tool(s(choice, "toolId")) {
                        ui.label(
                            RichText::new(format!("{} · {}", s(tool, "name"), s(choice, "role")))
                                .strong(),
                        );
                        ui.label(s(choice, "freeUse"));
                        self.access(ui, tool, s(choice, "actionId"), true);
                    }
                }
                if !s(&quick, "toolId").is_empty() {
                    if let Some(tool) = self.app.tool(s(&quick, "toolId")) {
                        self.access(ui, tool, s(&quick, "toolAction"), true);
                    }
                }
            }
            ui.separator();
            ui.label(RichText::new("Tools & resources").strong().size(18.0));
            for entry in a(&ws["tools"]) {
                if let Some(tool) = self.app.tool(s(entry, "toolId")) {
                    ui.label(RichText::new(s(tool, "name")).strong());
                    ui.label(s(entry, "why"));
                    self.access(ui, tool, s(entry, "actionId"), true);
                }
            }
            for r in a(&ws["resources"]) {
                ui.hyperlink_to(s(r, "label"), s(r, "url"));
                ui.label(s(r, "description"));
                ui.label(
                    RichText::new(format!(
                        "{} · {} · checked {}",
                        s(r, "group"),
                        s(r, "kind"),
                        s(r, "checkedAt")
                    ))
                    .small(),
                );
                if s(r, "sourceUrl") != s(r, "url") && safe_url(s(r, "sourceUrl")) {
                    ui.hyperlink_to("Source", s(r, "sourceUrl"));
                }
            }
            ui.separator();
            field(ui, "Plan the job", &ws["goal"]);
            field(ui, "Start here", &ws["startHere"]);
            for d in a(&ws["decisions"]) {
                ui.label(RichText::new(s(d, "symptom")).strong());
                field(ui, "Next action", &d["action"]);
                field(ui, "Expected result", &d["expected"]);
            }
            field(ui, "Pause & escalate when", &job["stopIf"]);
            field(ui, "Additional escalation conditions", &ws["escalateWhen"]);
        } else {
            ui.label("Protect data before routine maintenance. If a drive may be failing, recover first; isolate suspected compromise before routine patching or optimization.");
        }
        ui.separator();
        ui.label(RichText::new("Work through the job").strong().size(20.0));
        let steps: Vec<_> = a(&job["steps"])
            .iter()
            .enumerate()
            .filter(|(_, step)| !focus || s(step, "anchor").starts_with("step-closeout-"))
            .map(|(i, s)| (i, s.clone()))
            .collect();
        for (number, (index, step)) in steps.iter().enumerate() {
            ui.add_space(8.0);
            let key = guides::checklist_key(&job, "step", *index, s(step, "title"));
            let mut done = self.app.checks[&id][&key] == true;
            if ui
                .checkbox(
                    &mut done,
                    RichText::new(format!("{}. {}", number + 1, s(step, "title"))).strong(),
                )
                .changed()
            {
                self.app.checks[&id][&key] = json!(done);
                self.dirty = true;
            }
            ui.label(s(step, "detail"));
            if !txt(&step["command"]).is_empty() {
                native_command(
                    ui,
                    &json!({"title":s(step,"commandPurpose"),"code":step["command"],"shell":s(step,"commandShell"),"safety":s(step,"commandSafety"),"sourceUrl":s(step,"commandUrl"),"sourceTitle":s(step,"commandName")}),
                );
            }
            for shortcut in a(&step["shortcuts"]) {
                ui.horizontal_wrapped(|ui| {
                    if !s(shortcut, "job").is_empty() {
                        if ui.link(s(shortcut, "label")).clicked() {
                            self.open_job(s(shortcut, "job"));
                        }
                    } else if let Some(tool) = self.app.tool(s(shortcut, "tool")).cloned() {
                        if ui.link(s(shortcut, "label")).clicked() {
                            self.open_tool(s(&tool, "id"));
                        }
                        self.access(ui, &tool, "", true);
                    } else if safe_url(s(shortcut, "url")) {
                        ui.hyperlink_to(s(shortcut, "label"), s(shortcut, "url"));
                    }
                });
            }
        }
        if !focus {
            ui.separator();
            ui.label(RichText::new("Verify the result").strong().size(20.0));
            for (index, item) in a(&job["verify"]).iter().enumerate() {
                let title = txt(item);
                let key = guides::checklist_key(&job, "verify", index, &title);
                let mut done = self.app.checks[&id][&key] == true;
                if ui.checkbox(&mut done, &title).changed() {
                    self.app.checks[&id][&key] = json!(done);
                    self.dirty = true;
                }
            }
            field(ui, "Ready to close when", &ws["doneWhen"]);
            let fixes: Vec<_> = a(&self.app.data["fieldNotes"]["notes"])
                .iter()
                .filter(|n| s(n, "jobId") == id)
                .cloned()
                .collect();
            if !fixes.is_empty() {
                ui.separator();
                ui.label(RichText::new("Specific fixes").strong().size(20.0));
            }
            for note in fixes {
                ui.add_space(10.0);
                ui.label(RichText::new(s(&note, "title")).strong().size(18.0));
                for (label, key) in [("Symptom", "symptom"), ("First action", "firstAction")] {
                    field(ui, label, &note[key]);
                }
                numbered(ui, &note["steps"]);
                field(ui, "Expected result", &note["expected"]);
                field(ui, "If unresolved", &note["ifUnresolved"]);
                for command in a(&note["commands"]) {
                    native_command(ui, command);
                }
                for tool_id in a(&note["toolIds"]) {
                    if let Some(tool) = self.app.tool(tool_id.as_str().unwrap_or("")) {
                        ui.label(RichText::new(s(tool, "name")).strong());
                        self.access(ui, tool, "", true);
                    }
                }
                sources(ui, &note["sourceUrls"]);
            }
            let picks: Vec<_> = a(&self.app.data["quickStart"]["items"])
                .iter()
                .filter(|q| s(q, "jobId") == id)
                .cloned()
                .collect();
            for item in picks {
                ui.separator();
                ui.label(
                    RichText::new(format!("Tool choice notes · {}", s(&item, "label"))).strong(),
                );
                for (label, key) in [
                    ("Why this choice", "why"),
                    ("Use and cost", "freeUse"),
                    ("Before you act", "boundary"),
                    ("Selection basis", "qualityBasis"),
                ] {
                    field(ui, label, &item[key]);
                }
                sources(ui, &item["evidenceUrls"]);
                for choice in a(&item["choices"]) {
                    field(ui, "Task role", &choice["role"]);
                    field(ui, "Best fit", &choice["why"]);
                    field(ui, "Use and cost", &choice["freeUse"]);
                    sources(ui, &choice["evidenceUrls"]);
                }
            }
            ui.separator();
            sources(ui, &job["sources"]);
            for related in a(&job["related"]) {
                if let Some(next) = self.app.job(related.as_str().unwrap_or("")).cloned() {
                    if ui
                        .link(format!("Continue: {}", s(&next, "title")))
                        .clicked()
                    {
                        self.open_job(s(&next, "id"));
                    }
                }
            }
        }
        ui.label("Keep customer-specific evidence, passwords and recovery keys in your private service record.");
    }
    fn command_center(&mut self, ui: &mut egui::Ui, updates: bool) {
        let data = self.app.data[if updates {
            "updatesCenter"
        } else {
            "commandCenter"
        }]
        .clone();
        Self::title(ui,if updates{"Updates & maintenance"}else{"Command Center"},"Search, review purpose and impact, then copy a precise command. Copying does not execute it.");
        let query = self.section_search(ui);
        let commands = a(&data["commands"]);
        ui.horizontal_wrapped(|ui| {
            Self::select(
                ui,
                "cmd-shell",
                "All shells",
                &mut self.command_shell,
                &commands.iter().map(|c| s(c, "shell").into()).collect(),
            );
            Self::select(
                ui,
                "cmd-risk",
                "All impact levels",
                &mut self.command_risk,
                &commands.iter().map(|c| s(c, "risk").into()).collect(),
            );
            Self::select(
                ui,
                "cmd-category",
                "All categories",
                &mut self.command_category,
                &commands.iter().map(|c| s(c, "category").into()).collect(),
            );
        });
        if updates {
            ui.label("This native edition provides the full update and recovery reference. It does not claim live machine inventory or remote execution without an enrolled, authenticated device connection.");
            for channel in a(&data["channels"]) {
                ui.separator();
                ui.label(RichText::new(s(channel, "name")).strong());
                field(ui, "Scope", &channel["scope"]);
                field(ui, "Manager", &channel["manager"]);
                field(ui, "Notes", &channel["note"]);
            }
        }
        ui.separator();
        ui.label(RichText::new("Command library").strong().size(20.0));
        let found: Vec<_> = commands
            .iter()
            .filter(|c| {
                matches(&c.to_string(), &query)
                    && (self.command_shell.is_empty() || s(c, "shell") == self.command_shell)
                    && (self.command_risk.is_empty() || s(c, "risk") == self.command_risk)
                    && (self.command_category.is_empty()
                        || s(c, "category") == self.command_category)
            })
            .collect();
        ui.label(format!("{} of {} commands", found.len(), commands.len()));
        for command in found {
            native_command(ui, command);
            ui.separator();
        }
        ui.label(RichText::new("Guided workflows").strong().size(20.0));
        for flow in a(&data["workflows"]) {
            if !matches(&flow.to_string(), &query) {
                continue;
            }
            ui.label(RichText::new(s(flow, "title")).strong());
            ui.label(s(flow, "description"));
            field(ui, "Sequence", &flow["sequence"]);
            field(ui, "Before running", &flow["warning"]);
            let items: Vec<_> = a(&flow["commandIds"])
                .iter()
                .filter_map(|id| {
                    commands
                        .iter()
                        .find(|c| s(c, "id") == id.as_str().unwrap_or(""))
                })
                .collect();
            for (index, item) in items.iter().enumerate() {
                ui.label(format!("{}. {}", index + 1, s(item, "title")));
            }
            if ui
                .button(format!("Copy workflow · {}", s(flow, "title")))
                .clicked()
            {
                ui.ctx().copy_text(
                    items
                        .iter()
                        .map(|c| format!("# {}\n{}", s(c, "title"), s(c, "code")))
                        .collect::<Vec<_>>()
                        .join("\n\n"),
                );
            }
            ui.separator();
        }
        for kit in a(&data["toolkits"]) {
            ui.label(RichText::new(s(kit, "name")).strong());
            field(ui, "Use", &kit["summary"]);
            field(ui, "Platform", &kit["platform"]);
            field(ui, "Trust and review", &kit["trust"]);
            if safe_url(s(kit, "url")) {
                ui.hyperlink_to("Official project", s(kit, "url"));
            }
            if let Some(install) = commands
                .iter()
                .find(|c| s(c, "id") == s(kit, "installCommandId"))
            {
                native_command(ui, install);
            }
        }
        field(ui, "Operating principles", &data["principles"]);
        field(ui, "Policies", &data["policies"]);
    }
    fn news(&mut self, ui: &mut egui::Ui) {
        Self::title(
            ui,
            "Technician news",
            "Bundled source snapshot. Publisher links open on demand; no background polling.",
        );
        let query = self.section_search(ui);
        ui.label(format!(
            "Snapshot: {}",
            s(&self.app.data["newsDigest"], "generatedAt")
        ));
        ui.horizontal_wrapped(|ui| {
            ui.checkbox(&mut self.news_saved_only, "Saved only");
            ui.checkbox(&mut self.news_unread_only, "Unread only");
        });
        let news = a(&self.app.data["newsDigest"]["items"]).to_vec();
        let sources_list = a(&self.app.data["news"]["sources"]).to_vec();
        for item in news
            .iter()
            .filter(|n| {
                matches(&n.to_string(), &query)
                    && (!self.news_saved_only || self.state.news_saved.contains(s(n, "id")))
                    && (!self.news_unread_only || !self.state.news_read.contains(s(n, "id")))
                    && (self.state.news_sources.is_empty()
                        || self.state.news_sources.contains(s(n, "sourceId")))
            })
            .cloned()
            .collect::<Vec<_>>()
        {
            ui.separator();
            ui.hyperlink_to(RichText::new(s(&item, "title")).strong(), s(&item, "url"));
            let source = sources_list
                .iter()
                .find(|s0| s(s0, "id") == s(&item, "sourceId"));
            ui.label(
                RichText::new(format!(
                    "{} · {}",
                    s(&item, "date"),
                    source
                        .map(|s0| s(s0, "name"))
                        .unwrap_or(s(&item, "sourceId"))
                ))
                .small(),
            );
            field(ui, "Summary", &item["summary"]);
            ui.horizontal_wrapped(|ui| {
                let id = s(&item, "id");
                let mut read = self.state.news_read.contains(id);
                if ui.checkbox(&mut read, "Read").changed() {
                    if read {
                        self.state.news_read.insert(id.into());
                    } else {
                        self.state.news_read.remove(id);
                    }
                    self.dirty = true;
                }
                let mut saved = self.state.news_saved.contains(id);
                if ui.checkbox(&mut saved, "Saved").changed() {
                    if saved {
                        self.state.news_saved.insert(id.into());
                    } else {
                        self.state.news_saved.remove(id);
                    }
                    self.dirty = true;
                }
            });
            for media in a(&item["media"]) {
                let url = s(media, "url");
                if safe_url(url) {
                    ui.hyperlink_to(format!("Open {} at publisher", s(media, "type")), url);
                }
            }
        }
        ui.separator();
        ui.label(RichText::new("Source directory").strong().size(20.0));
        ui.label(
            "Select sources to filter the snapshot. With none selected, all sources are shown.",
        );
        for source in sources_list {
            ui.horizontal_wrapped(|ui| {
                let id = s(&source, "id");
                let mut selected = self.state.news_sources.contains(id);
                if ui.checkbox(&mut selected, s(&source, "name")).changed() {
                    if selected {
                        self.state.news_sources.insert(id.into());
                    } else {
                        self.state.news_sources.remove(id);
                    }
                    self.dirty = true;
                }
                ui.hyperlink_to("Open source", s(&source, "url"));
            });
            ui.label(s(&source, "description"));
        }
    }
    fn services(&mut self, ui: &mut egui::Ui) {
        Self::title(
            ui,
            "Service map",
            "Every service area and complete job guide.",
        );
        let groups = a(&self.app.data["layout"]["groups"]).to_vec();
        for group in groups {
            if !group["jobs"].is_array() {
                continue;
            }
            ui.separator();
            ui.label(RichText::new(s(&group, "title")).strong().size(20.0));
            ui.label(s(&group, "summary"));
            for id in a(&group["jobs"]) {
                if let Some(job) = self.app.job(id.as_str().unwrap_or("")).cloned() {
                    if ui.link(s(&job, "title")).clicked() {
                        self.open_job(s(&job, "id"));
                    }
                    ui.label(s(&job, "startHere"));
                    for note in a(&self.app.data["fieldNotes"]["notes"])
                        .iter()
                        .filter(|n| s(n, "jobId") == s(&job, "id"))
                        .cloned()
                        .collect::<Vec<_>>()
                    {
                        if ui
                            .link(format!("Specific fix: {}", s(&note, "title")))
                            .clicked()
                        {
                            self.open_job(s(&job, "id"));
                        }
                    }
                }
            }
        }
    }
    fn knowledge(&mut self, ui: &mut egui::Ui) {
        Self::title(
            ui,
            "Knowledge desk",
            "Methods, evidence, standards and contribution paths.",
        );
        for (title, slug, body) in [
            (
                "Decision desk",
                "Decision-Desk",
                "Choose a procedure from the observed symptom.",
            ),
            (
                "Methods",
                "Methods",
                "Repeatable diagnostic and repair techniques.",
            ),
            (
                "Tool selection",
                "Tool-Selection",
                "Compare fit, boundaries and observed evidence.",
            ),
            (
                "Current tool choices",
                "Current-Tool-Choices",
                "Selection reasons and alternatives.",
            ),
            (
                "Knowledge standards",
                "Knowledge-Standards",
                "Structure, source quality and honest review dates.",
            ),
            (
                "Knowledge organization",
                "Knowledge-Organization",
                "One clear home for every service topic.",
            ),
            (
                "Knowledge health",
                "Knowledge-Health",
                "Coverage, validation and outstanding gaps.",
            ),
            (
                "Research queue",
                "Research-Queue",
                "Proposed improvements and evidence to collect.",
            ),
            (
                "Service templates",
                "Templates",
                "Reusable private-service record templates.",
            ),
            (
                "Service evidence",
                "Service-Evidence",
                "Connect findings, changes and verification.",
            ),
            (
                "Interface design",
                "Interface-Design",
                "Shared visual and interaction rules.",
            ),
            (
                "Contribute",
                "Contribute",
                "Publish a reproducible correction.",
            ),
            (
                "Continuous improvement",
                "Continuous-Improvement",
                "Keep the collection useful as products change.",
            ),
        ] {
            ui.separator();
            ui.hyperlink_to(
                RichText::new(title).strong(),
                format!("https://github.com/Herbertofury/INTA/wiki/{slug}"),
            );
            ui.label(body);
        }
    }
    fn resources(&mut self, ui: &mut egui::Ui) {
        Self::title(
            ui,
            "Resources & local data",
            "Source history, exact platform boundaries and your portable workspace.",
        );
        for (label, url) in [
            ("INTA Site", "https://herbertofury.github.io/INTA/"),
            ("GitHub wiki", "https://github.com/Herbertofury/INTA/wiki"),
            ("Source repository", "https://github.com/Herbertofury/INTA"),
            (
                "Releases & Rescue media",
                "https://github.com/Herbertofury/INTA/releases",
            ),
            (
                "Build results",
                "https://github.com/Herbertofury/INTA/actions",
            ),
            ("Athena research", "https://github.com/Herbertofury/Athena"),
        ] {
            ui.hyperlink_to(label, url);
        }
        ui.separator();
        ui.label(RichText::new("Local workspace").strong());
        ui.label(format!("Data directory: {}", self.dir.display()));
        ui.label("Saved tools and checklist state are written after your changes. Move this folder with the executable for portable desktop use. Android uses its app-private data directory.");
        ui.label("This build contains the public tools, guides, commands and news snapshot. Shared accounts, enrolled-device remote actions, fleet/job synchronization and billing are separate acceptance milestones; no connection or cloud sync is implied by local persistence.");
        ui.separator();
        let query = self.section_search(ui);
        let mut urls = BTreeMap::new();
        for job in self.app.jobs() {
            for source in a(&job["sources"]) {
                if safe_url(s(source, "url")) {
                    urls.insert(s(source, "url"), s(source, "title"));
                }
            }
        }
        for tool in a(&self.app.data["onlineTools"]["tools"]) {
            for source in a(&tool["sources"]) {
                if safe_url(s(source, "url")) {
                    urls.insert(s(source, "url"), s(source, "title"));
                }
            }
        }
        ui.label(RichText::new("Official reference desk").strong().size(20.0));
        for (url, title) in urls {
            if matches(&format!("{title} {url}"), &query) {
                ui.hyperlink_to(if title.is_empty() { url } else { title }, url);
            }
        }
    }
}
impl eframe::App for NativeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let narrow = ctx.content_rect().width() < 760.0;
        ctx.style_mut(|style| {
            style.spacing.interact_size = Vec2::new(
                if narrow { 44.0 } else { 38.0 },
                if narrow { 44.0 } else { 38.0 },
            )
        });
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            if self.reset_pending.take().is_none() {
                self.page = Page::Tools;
            }
        }
        egui::TopBottomPanel::top("inta-header").show(ctx, |ui| {
            #[cfg(target_os = "android")]
            ui.add_space(28.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("INTA").strong().size(21.0));
                ui.label(if cfg!(target_os = "android") {
                    "Mobile"
                } else {
                    "Desktop"
                });
                ui.separator();
                ui.label(self.page.title());
            });
            if narrow {
                ui.horizontal_wrapped(|ui| {
                    for (page, label) in NAV {
                        if ui.selectable_label(self.page == *page, *label).clicked() {
                            self.page = *page;
                        }
                    }
                });
            }
        });
        if !narrow {
            egui::SidePanel::left("inta-navigation")
                .exact_width(208.0)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.add_space(12.0);
                    ui.label(
                        RichText::new("WORKSPACE")
                            .small()
                            .color(ui.visuals().weak_text_color()),
                    );
                    for (page, label) in NAV {
                        if ui
                            .add_sized(
                                [ui.available_width(), 38.0],
                                egui::Button::selectable(self.page == *page, *label),
                            )
                            .clicked()
                        {
                            self.page = *page;
                        }
                    }
                    ui.separator();
                    if ui
                        .checkbox(&mut self.state.dark, "Dark appearance")
                        .changed()
                    {
                        configure_style(ctx, self.state.dark);
                        self.dirty = true;
                    }
                    ui.label(
                        RichText::new(format!(
                            "{} tools · {} guides",
                            self.app.tools().len(),
                            self.app.jobs().len()
                        ))
                        .small(),
                    );
                    ui.label(RichText::new("Offline reference available").small());
                });
        }
        egui::TopBottomPanel::bottom("inta-status").show(ctx, |ui| {
            if narrow
                && ui
                    .checkbox(&mut self.state.dark, "Dark appearance")
                    .changed()
            {
                configure_style(ctx, self.state.dark);
                self.dirty = true;
            }
            if self.notice.is_empty() {
                ui.label(RichText::new("Local progress · Official links open on demand").small());
            } else {
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new(&self.notice).small());
                    if ui.small_button("Dismiss").clicked() {
                        self.notice.clear();
                    }
                });
            }
            #[cfg(target_os = "android")]
            ui.add_space(20.0);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .id_salt(self.page.title())
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    match self.page {
                        Page::Tools => self.tools(ui, false, false),
                        Page::Saved => self.tools(ui, true, false),
                        Page::Online => self.tools(ui, false, true),
                        Page::Tool => self.tool_detail(ui),
                        Page::Playbooks => self.playbooks(ui),
                        Page::Job => self.job(ui, false),
                        Page::Checklist => self.job(ui, true),
                        Page::Commands => self.command_center(ui, false),
                        Page::Updates => self.command_center(ui, true),
                        Page::News => self.news(ui),
                        Page::Services => self.services(ui),
                        Page::Knowledge => self.knowledge(ui),
                        Page::Resources => self.resources(ui),
                    }
                });
        });
        if let Some(id) = self.reset_pending.clone() {
            egui::Window::new("Start a fresh checklist?").collapsible(false).resizable(false).anchor(egui::Align2::CENTER_CENTER,Vec2::ZERO).show(ctx,|ui|{ui.label("This clears the local checks for this guide. Export the completed checklist first if you need a record.");ui.horizontal(|ui|{if ui.button("Keep current checklist").clicked(){self.reset_pending=None;}if ui.button("Clear checks").clicked(){self.app.checks[&id]=json!({});self.reset_pending=None;self.dirty=true;}});});
        }
        if self.dirty {
            self.save();
        }
        if let Some(path) = self.smoke_path.clone() {
            if !self.smoke_requested {
                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                self.smoke_requested = true;
                ctx.request_repaint();
            }
            let capture = ctx.input(|input| {
                input.events.iter().find_map(|event| {
                    if let egui::Event::Screenshot { image, .. } = event {
                        Some(image.clone())
                    } else {
                        None
                    }
                })
            });
            if let Some(image) = capture {
                let bytes: Vec<u8> = image.pixels.iter().flat_map(|p| p.to_array()).collect();
                if let Some(parent) = path.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                match image::save_buffer_with_format(
                    &path,
                    &bytes,
                    image.width() as u32,
                    image.height() as u32,
                    image::ColorType::Rgba8,
                    image::ImageFormat::Png,
                ) {
                    Ok(()) => eprintln!("Native screenshot saved to {}", path.display()),
                    Err(e) => {
                        eprintln!("Native screenshot failed: {e}");
                        std::process::exit(1);
                    }
                };
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }
    }
}
fn field(ui: &mut egui::Ui, label: &str, v: &Value) {
    if v.is_null() || v.as_str() == Some("") {
        return;
    }
    ui.label(RichText::new(label).strong());
    if let Some(array) = v.as_array() {
        for item in array {
            let text = txt(item);
            if safe_url(&text) {
                ui.hyperlink_to(&text, &text);
            } else {
                ui.label(format!("• {text}"));
            }
        }
    } else {
        ui.label(txt(v));
    }
}
fn numbered(ui: &mut egui::Ui, v: &Value) {
    for (i, item) in a(v).iter().enumerate() {
        ui.label(format!("{}. {}", i + 1, txt(item)));
    }
}
fn sources(ui: &mut egui::Ui, v: &Value) {
    for item in a(v) {
        let url = item.as_str().unwrap_or_else(|| s(item, "url"));
        let title = if s(item, "title").is_empty() {
            url
        } else {
            s(item, "title")
        };
        if safe_url(url) {
            ui.hyperlink_to(title, url);
        }
        if !s(item, "evidence").is_empty() {
            ui.label(RichText::new(s(item, "evidence")).small());
        }
    }
}
fn native_command(ui: &mut egui::Ui, command: &Value) {
    ui.push_id(s(command, "id").to_string() + s(command, "code"), |ui| {
        egui::Frame::group(ui.style())
            .inner_margin(12)
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new(if s(command, "title").is_empty() {
                            s(command, "label")
                        } else {
                            s(command, "title")
                        })
                        .strong(),
                    );
                    if ui.button("Copy command").clicked() {
                        ui.ctx().copy_text(txt(&command["code"]));
                    }
                });
                ui.label(
                    RichText::new(format!(
                        "{} · {} · {}",
                        s(command, "shell"),
                        s(command, "privilege"),
                        s(command, "risk")
                    ))
                    .small(),
                );
                if !s(command, "summary").is_empty() {
                    ui.label(s(command, "summary"));
                }
                egui::ScrollArea::horizontal()
                    .id_salt("code")
                    .show(ui, |ui| {
                        ui.label(RichText::new(txt(&command["code"])).monospace());
                    });
                field(ui, "Before execution", &command["safety"]);
                if safe_url(s(command, "sourceUrl")) {
                    ui.hyperlink_to(
                        if s(command, "sourceTitle").is_empty() {
                            "Official reference"
                        } else {
                            s(command, "sourceTitle")
                        },
                        s(command, "sourceUrl"),
                    );
                }
            });
    });
}
fn numeric(tool: &Value, sort: &str) -> Option<f64> {
    inta_ui::catalog::metric(
        tool,
        match sort {
            "GitHub stars" => "stars",
            "Forks" => "forks",
            "Provider rating" => "rating",
            "Provider reviews" => "reviews",
            "Weekly downloads" => "popularity",
            "Users" => "users",
            _ => "unknown",
        },
    )
}

pub fn filtered_tool_ids(
    app: &App,
    filters: &ToolFilters,
    saved_only: bool,
    online: bool,
) -> Vec<String> {
    let mut tools: Vec<_> = app
        .tools()
        .iter()
        .filter(|tool| {
            let id = s(tool, "id");
            let entry = a(&app.data["onlineTools"]["tools"])
                .iter()
                .find(|entry| s(entry, "toolId") == id);
            let text = if online {
                format!(
                    "{} {}",
                    app.search_index.get(id).map(String::as_str).unwrap_or(""),
                    entry.map(Value::to_string).unwrap_or_default()
                )
            } else {
                app.search_index.get(id).cloned().unwrap_or_default()
            };
            matches(&text, &filters.query)
                && (!saved_only || app.favorites.contains(id))
                && (!online || entry.is_some())
                && (!online
                    || filters.online_group.is_empty()
                    || entry.is_some_and(|e| s(e, "group") == filters.online_group))
                && (filters.category.is_empty() || s(tool, "category") == filters.category)
                && (filters.platform.is_empty()
                    || a(&tool["platforms"])
                        .iter()
                        .any(|p| p.as_str() == Some(&filters.platform)))
                && (filters.tier.is_empty() || s(tool, "tier") == filters.tier)
                && (filters.mode.is_empty() || s(tool, "mode") == filters.mode)
        })
        .collect();
    if !filters.sort.is_empty() {
        tools.sort_by(|left, right| {
            let fallback = || {
                s(left, "name")
                    .to_lowercase()
                    .cmp(&s(right, "name").to_lowercase())
            };
            match filters.sort.as_str() {
                "Name A–Z" => fallback(),
                "Name Z–A" => fallback().reverse(),
                "Recommendation" => {
                    let rank = |v: &Value| match s(v, "tier") {
                        "core" => 3,
                        "specialist" => 2,
                        _ => 1,
                    };
                    rank(right).cmp(&rank(left)).then_with(fallback)
                }
                "Reviewed" => s(right, "reviewedAt")
                    .cmp(s(left, "reviewedAt"))
                    .then_with(fallback),
                sort => match (numeric(left, sort), numeric(right, sort)) {
                    (Some(x), Some(y)) => y.total_cmp(&x).then_with(fallback),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    _ => fallback(),
                },
            }
        });
    }
    tools.iter().map(|tool| s(tool, "id").into()).collect()
}
pub fn run_desktop() -> eframe::Result {
    let (dir, notice) = data_directory();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("INTA Desktop")
            .with_inner_size([
                std::env::var("INTA_SMOKE_WIDTH")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(1280.0),
                840.0,
            ])
            .with_min_inner_size([360.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        "INTA Desktop",
        options,
        Box::new(move |cc| Ok(Box::new(NativeApp::new(cc, dir, notice)))),
    )
}
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(android: winit::platform::android::activity::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_tag("INTA")
            .with_max_level(log::LevelFilter::Info),
    );
    let dir = android
        .internal_data_path()
        .unwrap_or_else(|| PathBuf::from("."));
    let options = eframe::NativeOptions {
        android_app: Some(android),
        ..Default::default()
    };
    if let Err(error) = eframe::run_native(
        "INTA Mobile",
        options,
        Box::new(move |cc| Ok(Box::new(NativeApp::new(cc, dir, String::new())))),
    ) {
        log::error!("Native UI failed: {error}");
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn complete_catalog_and_unknown_metrics_are_preserved() {
        let app = App::new(serde_json::from_str(BUNDLED_CATALOG).unwrap());
        let all = filtered_tool_ids(&app, &ToolFilters::default(), false, false);
        assert_eq!(all.len(), app.tools().len());
        let sorted = filtered_tool_ids(
            &app,
            &ToolFilters {
                sort: "GitHub stars".into(),
                ..Default::default()
            },
            false,
            false,
        );
        assert_eq!(
            all.iter().collect::<BTreeSet<_>>(),
            sorted.iter().collect::<BTreeSet<_>>()
        );
        let mut unknown = false;
        for id in sorted {
            let measured = numeric(app.tool(&id).unwrap(), "GitHub stars").is_some();
            if !measured {
                unknown = true;
            }
            assert!(!unknown || !measured);
        }
    }
    #[test]
    fn native_filter_combination_and_saved_set_are_real() {
        let mut app = App::new(serde_json::from_str(BUNDLED_CATALOG).unwrap());
        app.favorites.insert("rustdesk".into());
        let found = filtered_tool_ids(
            &app,
            &ToolFilters {
                query: "rustdesk".into(),
                ..Default::default()
            },
            true,
            false,
        );
        assert_eq!(found, vec!["rustdesk"]);
        assert!(filtered_tool_ids(
            &app,
            &ToolFilters {
                query: "definitely-no-such-tool-839274".into(),
                ..Default::default()
            },
            false,
            false
        )
        .is_empty());
    }
    #[test]
    fn portable_state_roundtrip_preserves_checks_and_view() {
        let dir = std::env::temp_dir().join(format!(
            "inta-state-test-{}-{}",
            std::process::id(),
            timestamp()
        ));
        let mut state = NativeState::default();
        state.gallery = true;
        state.favorites.insert("rustdesk".into());
        state.checks = json!({"job":{"step:anchor:stable":true}});
        save_state(&dir, &state).unwrap();
        let (restored, notice, blocked) = load_state(&dir);
        assert!(!blocked);
        assert!(notice.is_empty());
        assert!(restored.gallery);
        assert!(restored.favorites.contains("rustdesk"));
        assert_eq!(restored.checks, state.checks);
        assert!(dir.join("inta-workspace.json").is_file());
        fs::remove_dir_all(dir).unwrap();
    }
}
