//! The shared INTA application. All owned rendering, state and interaction logic
//! lives here; wasm-bindgen emits the browser ABI adapter at build time.
pub mod bench;
#[cfg(target_arch = "wasm32")]
mod browser;
pub mod catalog;
pub mod common;
pub mod guides;

use common::*;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

pub const SAVED_KEY: &str = "inta:favorite-tools:v1";
pub const CHECKS_KEY: &str = "inta:job-checklists:v1";
pub const VIEW_KEY: &str = "inta:tool-view:v1";
pub const THEME_KEY: &str = "inta:theme:v1";

#[derive(Clone, Default, Debug)]
pub struct Route {
    pub name: String,
    pub id: String,
    pub quick: String,
    pub params: BTreeMap<String, String>,
}

#[cfg(test)]
mod runtime_tests {
    use super::*;
    #[test]
    fn global_command_search_is_case_insensitive_and_uncapped() {
        let data: Value = serde_json::from_str(include_str!("../../../site/catalog.json")).unwrap();
        let catalog = &data["commandCenter"]["commands"];
        let updates = &data["updatesCenter"]["commands"];
        for query in [
            "PowerShell",
            "powershell",
            "PoWeRsHeLl",
            "NINITE",
            "choco noop",
            "technician baseline",
            "Installer recovery",
            "WiNgEt RePaIr",
        ] {
            let result = command_search(catalog, query)
                .into_iter()
                .chain(command_search(updates, query))
                .map(|v| s(v, "id"))
                .collect::<Vec<_>>();
            let expected = a(catalog)
                .iter()
                .chain(a(updates))
                .filter(|item| {
                    let fields = [
                        "title",
                        "summary",
                        "code",
                        "safety",
                        "shell",
                        "sourceTitle",
                        "category",
                    ]
                    .iter()
                    .map(|k| s(item, k))
                    .collect::<Vec<_>>()
                    .join(" ")
                    .to_lowercase();
                    query
                        .to_lowercase()
                        .split_whitespace()
                        .all(|term| fields.contains(term))
                })
                .map(|v| s(v, "id"))
                .collect::<Vec<_>>();
            assert_eq!(result, expected, "{query}");
        }
        assert!(command_search(catalog, "PowerShell").len() > 8);
        assert_eq!(
            command_search(catalog, "powershell").len(),
            command_search(catalog, "PoWeRsHeLl").len()
        );
    }
    #[test]
    fn route_round_trip_preserves_filters_and_scenario() {
        let route = Route::parse("#job/outlook-email/quick/fix%20mail?section=fix-a&q=DNS+TLS");
        assert_eq!(route.id, "outlook-email");
        assert_eq!(route.quick, "fix mail");
        assert_eq!(Route::parse(&route.href()).get("q"), "DNS TLS");
        assert_eq!(Route::parse(&route.href()).get("section"), "fix-a");
    }
}
impl Route {
    pub fn parse(hash: &str) -> Self {
        let raw = hash.trim_start_matches('#');
        let (path, query) = raw.split_once('?').unwrap_or((raw, ""));
        let mut parts = path.split('/');
        let mut name = parts.next().unwrap_or("home").to_string();
        if name.is_empty() || name == "main" {
            name = "home".into();
        }
        let id = decode(parts.next().unwrap_or(""));
        let quick = if parts.next() == Some("quick") {
            decode(parts.next().unwrap_or(""))
        } else {
            String::new()
        };
        let params = query
            .split('&')
            .filter_map(|part| {
                if part.is_empty() {
                    return None;
                }
                let (k, v) = part.split_once('=').unwrap_or((part, ""));
                Some((decode(k), decode(v)))
            })
            .collect();
        Self {
            name,
            id,
            quick,
            params,
        }
    }
    pub fn get(&self, key: &str) -> &str {
        self.params.get(key).map(String::as_str).unwrap_or("")
    }
    pub fn href(&self) -> String {
        let path = if self.id.is_empty() {
            self.name.clone()
        } else {
            format!(
                "{}/{}{}",
                self.name,
                encode(&self.id),
                if self.quick.is_empty() {
                    String::new()
                } else {
                    format!("/quick/{}", encode(&self.quick))
                }
            )
        };
        let params = self
            .params
            .iter()
            .filter(|(_, v)| !v.is_empty())
            .map(|(k, v)| format!("{}={}", encode(k), encode(v)))
            .collect::<Vec<_>>()
            .join("&");
        format!(
            "#{path}{}",
            if params.is_empty() {
                String::new()
            } else {
                format!("?{params}")
            }
        )
    }
}

#[derive(Clone)]
pub struct App {
    pub data: Value,
    pub favorites: BTreeSet<String>,
    pub checks: Value,
    pub route: Route,
    pub preferred_view: String,
    pub storage_available: bool,
    pub news_sources: BTreeSet<String>,
    pub news_read: BTreeSet<String>,
    pub news_saved: BTreeSet<String>,
    pub search_index: BTreeMap<String, String>,
}
impl App {
    pub fn new(data: Value) -> Self {
        init_links(&data["links"]);
        let mut app = Self {
            data,
            favorites: BTreeSet::new(),
            checks: json!({}),
            route: Route::parse(""),
            preferred_view: "list".into(),
            storage_available: true,
            news_sources: BTreeSet::new(),
            news_read: BTreeSet::new(),
            news_saved: BTreeSet::new(),
            search_index: BTreeMap::new(),
        };
        let mut search_index = BTreeMap::new();
        for tool in app.tools() {
            let mut terms = tool.to_string();
            for item in a(&app.data["quickStart"]["items"]) {
                if s(item, "toolId") == s(tool, "id")
                    || a(&item["choices"])
                        .iter()
                        .any(|choice| s(choice, "toolId") == s(tool, "id"))
                {
                    terms.push_str(&item.to_string());
                }
            }
            search_index.insert(s(tool, "id").into(), terms.to_lowercase());
        }
        app.search_index = search_index;
        app
    }
    pub fn tools(&self) -> &[Value] {
        a(&self.data["tools"])
    }
    pub fn jobs(&self) -> &[Value] {
        a(&self.data["jobs"])
    }
    pub fn tool(&self, id: &str) -> Option<&Value> {
        self.tools().iter().find(|v| s(v, "id") == id)
    }
    pub fn job(&self, id: &str) -> Option<&Value> {
        self.jobs().iter().find(|v| s(v, "id") == id)
    }
    pub fn view(&self) -> &str {
        match self.route.get("view") {
            "gallery" => "gallery",
            "list" => "list",
            _ => &self.preferred_view,
        }
    }
    pub fn tool_name(&self, tool: &Value) -> String {
        let brand = a(&self.data["brands"]["brands"])
            .iter()
            .find(|b| s(b, "toolId") == s(tool, "id"));
        let logo = brand
            .map(|b| {
                let asset = &self.data["brands"]["assets"][s(b, "assetId")];
                let src = if asset.is_string() {
                    asset.as_str().unwrap_or("")
                } else {
                    s(asset, "dataUrl")
                };
                if src.starts_with("data:image/") {
                    format!(
                        "<img class=\"brand-icon\" src=\"{}\" alt=\"\" width=\"28\" height=\"28\">",
                        esc(src)
                    )
                } else {
                    String::new()
                }
            })
            .unwrap_or_default();
        let url = brand
            .map(|b| s(b, "downloadPage"))
            .filter(|u| !u.is_empty())
            .unwrap_or_else(|| s(tool, "homepage"));
        external(
            url,
            &format!("{}<span>{}</span>", logo, esc(s(tool, "name"))),
            "tool-name brand-link",
        )
    }
    pub fn access_actions(&self, tool: &Value, action_id: &str, primary_only: bool) -> String {
        let record = &self.data["toolAccess"][s(tool, "id")];
        let official = if s(record, "officialUrl").is_empty() {
            s(tool, "homepage")
        } else {
            s(record, "officialUrl")
        };
        let mut options = Vec::new();
        if record["primary"].is_object() {
            options.push(&record["primary"]);
        }
        options.extend(a(&record["alternates"]));
        if !action_id.is_empty() {
            options.retain(|x| s(x, "id") == action_id);
        } else if primary_only {
            options.truncate(1);
        }
        let mut html = format!(
            "<div class=\"tool-access library-access\" data-tool-access=\"{}\">{}",
            esc(s(tool, "id")),
            external(
                official,
                &format!("Official page {}", icon("external")),
                "access-link official-page"
            )
        );
        for (i, option) in options.iter().enumerate() {
            if !safe_url(s(option, "url")) {
                continue;
            }
            html.push_str(&format!("<a class=\"access-link acquisition-link{}\" data-access-tool=\"{}\" data-acquisition-kind=\"{}\" data-access-id=\"{}\" data-access-role=\"{}\" href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{} {}</a>",if i==0{" primary-acquisition"}else{""},esc(s(tool,"id")),esc(s(option,"kind")),esc(s(option,"id")),if i==0{"primary"}else{"alternate"},esc(s(option,"url")),esc(s(option,"label")),icon(if s(option,"kind")=="file"{"download"}else{"external"})));
        }
        html.push_str("</div>");
        html
    }
    pub fn quick_choices(&self, item: &Value) -> String {
        let choices = catalog::normalized_choices(self, item);
        let mut html = if item["guideOnly"] == true {
            "<p class=\"no-extra-tool\">No extra download needed</p>".into()
        } else {
            String::new()
        };
        html.push_str("<div class=\"quick-choice-list\">");
        for choice in &choices {
            if let Some(tool) = self.tool(s(choice, "toolId")) {
                html.push_str(&format!("<article class=\"quick-choice\" data-quick-choice=\"{}\" data-choice-action=\"{}\"><div class=\"choice-description\"><h4>{}</h4><p class=\"choice-role\">{}</p><p class=\"choice-use\">{}</p></div>{}</article>",esc(s(tool,"id")),esc(s(choice,"actionId")),self.tool_name(tool),formatted(s(choice,"role")),formatted(s(choice,"freeUse")),self.access_actions(tool,s(choice,"actionId"),true)));
            }
        }
        html.push_str("</div>");
        html
    }
    pub fn save_button(&self, tool: &Value) -> String {
        let saved = self.favorites.contains(s(tool, "id"));
        format!("<button class=\"save-tool\" data-save=\"{}\" aria-pressed=\"{}\" aria-label=\"{} {}\" title=\"{} tool\">{}</button>",esc(s(tool,"id")),saved,if saved{"Unsave"}else{"Save"},esc(s(tool,"name")),if saved{"Unsave"}else{"Save"},icon("bookmark"))
    }
    pub fn render(&self) -> String {
        match self.route.name.as_str() {
            "home" => catalog::home(self),
            "tools" | "saved" => catalog::library(self, false),
            "tool" => catalog::detail(self, &self.route.id),
            "playbooks" | "job" | "checklist" | "services" | "knowledge" | "resources" => {
                guides::render(self, &self.route)
            }
            "online" | "news" | "command-center" | "updates" => bench::render(self, &self.route),
            x if x.starts_with("discipline-") => catalog::home(self),
            _ => empty(
                "Page not found",
                "Choose a destination in the workspace navigation.",
                "#home",
                "Open workspace",
            ),
        }
    }
}
