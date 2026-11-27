use crate::{
    bench, catalog, common::*, guides, App, Route, CHECKS_KEY, SAVED_KEY, THEME_KEY, VIEW_KEY,
};
use serde_json::{json, Value};
use std::cell::RefCell;
use std::collections::BTreeSet;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{
    Element, Event, HtmlElement, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement,
    KeyboardEvent, Request, RequestCache, RequestCredentials, RequestInit, RequestMode, Response,
};

thread_local! {
 static APP:RefCell<Option<App>>=const{RefCell::new(None)};
 static UNDO:RefCell<Option<(String,Value)>>=const{RefCell::new(None)};
 static RETURN_TO:RefCell<String>=const{RefCell::new(String::new())};
 static PWA:RefCell<Option<Event>>=const{RefCell::new(None)};
 static AGENT:RefCell<Value>=const{RefCell::new(Value::Null)};
}
fn window() -> web_sys::Window {
    web_sys::window().expect("browser window")
}
fn document() -> web_sys::Document {
    window().document().expect("browser document")
}
fn q(selector: &str) -> Option<Element> {
    document().query_selector(selector).ok().flatten()
}
fn all(selector: &str) -> Vec<Element> {
    let Ok(nodes) = document().query_selector_all(selector) else {
        return Vec::new();
    };
    (0..nodes.length())
        .filter_map(|i| nodes.get(i)?.dyn_into::<Element>().ok())
        .collect()
}
fn attr(el: &Element, name: &str) -> String {
    el.get_attribute(name).unwrap_or_default()
}
fn closest(el: &Element, selector: &str) -> Option<Element> {
    el.closest(selector).ok().flatten()
}
fn value(selector: &str) -> String {
    q(selector)
        .map(|e| {
            if let Some(i) = e.dyn_ref::<HtmlInputElement>() {
                i.value()
            } else if let Some(s) = e.dyn_ref::<HtmlSelectElement>() {
                s.value()
            } else {
                String::new()
            }
        })
        .unwrap_or_default()
}
fn set_value(selector: &str, value: &str) {
    if let Some(e) = q(selector) {
        if let Some(i) = e.dyn_ref::<HtmlInputElement>() {
            i.set_value(value);
        } else if let Some(s) = e.dyn_ref::<HtmlSelectElement>() {
            s.set_value(value);
        }
    }
}
fn is_checked(selector: &str) -> bool {
    q(selector)
        .and_then(|e| e.dyn_into::<HtmlInputElement>().ok())
        .is_some_and(|e| e.checked())
}
fn html(selector: &str, content: &str) {
    if let Some(e) = q(selector) {
        e.set_inner_html(content);
    }
}
fn text(selector: &str, content: &str) {
    if let Some(e) = q(selector) {
        e.set_text_content(Some(content));
    }
}
fn hidden(selector: &str, hide: bool) {
    if let Some(e) = q(selector) {
        if hide {
            let _ = e.set_attribute("hidden", "");
        } else {
            let _ = e.remove_attribute("hidden");
        }
    }
}
fn focus(selector: &str) {
    if let Some(e) = q(selector).and_then(|e| e.dyn_into::<HtmlElement>().ok()) {
        let _ = e.focus();
    }
}
fn with_app<T>(f: impl FnOnce(&App) -> T) -> T {
    APP.with(|a| f(a.borrow().as_ref().expect("application initialized")))
}
fn mutate<T>(f: impl FnOnce(&mut App) -> T) -> T {
    APP.with(|a| f(a.borrow_mut().as_mut().expect("application initialized")))
}
fn load(key: &str, fallback: Value) -> Value {
    window()
        .local_storage()
        .ok()
        .flatten()
        .and_then(|s| s.get_item(key).ok().flatten())
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or(fallback)
}
fn store(key: &str, data: &Value) {
    let success = window()
        .local_storage()
        .ok()
        .flatten()
        .is_some_and(|s| s.set_item(key, &data.to_string()).is_ok());
    if !success {
        mutate(|a| a.storage_available = false);
        hidden("#library-storage-status", false);
    }
}
fn session_token() -> String {
    window()
        .session_storage()
        .ok()
        .flatten()
        .and_then(|s| s.get_item("inta-local-agent-token").ok().flatten())
        .unwrap_or_default()
}
fn clear_token() {
    if let Ok(Some(storage)) = window().session_storage() {
        let _ = storage.remove_item("inta-local-agent-token");
    }
}
fn as_set(value: Value) -> BTreeSet<String> {
    a(&value)
        .iter()
        .filter_map(|v| v.as_str().map(str::to_owned))
        .collect()
}
fn persist_route() {
    let hash = with_app(|a| a.route.href());
    if let Ok(history) = window().history() {
        let _ = history.replace_state_with_url(&JsValue::NULL, "", Some(&hash));
    }
}
fn navigate(hash: &str) {
    let _ = window().location().set_hash(hash);
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let raw = document()
        .get_element_by_id("inta-data")
        .and_then(|e| e.text_content())
        .ok_or_else(|| JsValue::from_str("Missing INTA data"))?;
    let raw = raw
        .trim()
        .strip_prefix("window.INTA_DATA = ")
        .unwrap_or(raw.trim())
        .trim_end_matches(';');
    let data: Value = serde_json::from_str(raw).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let mut app = App::new(data);
    app.favorites = as_set(load(SAVED_KEY, json!([])));
    app.favorites.retain(|id| {
        app.data["tools"]
            .as_array()
            .is_some_and(|ts| ts.iter().any(|t| s(t, "id") == id))
    });
    app.checks = load(CHECKS_KEY, json!({}));
    if !app.checks.is_object() {
        app.checks = json!({});
    }
    app.preferred_view = if load(VIEW_KEY, json!("list")) == "gallery" {
        "gallery"
    } else {
        "list"
    }
    .into();
    app.news_sources = as_set(load("inta:news-follows:v1", json!([])));
    guides::migrate_checks(&mut app);
    APP.with(|a| *a.borrow_mut() = Some(app));
    let theme = load(THEME_KEY, json!("dark"));
    set_theme(if theme == "light" { "light" } else { "dark" });
    for element in all("[data-icon]") {
        element.set_inner_html(&icon(&attr(&element, "data-icon")));
    }
    on(document().as_ref(), "click", click)?;
    on(document().as_ref(), "input", input)?;
    on(document().as_ref(), "change", change)?;
    on(document().as_ref(), "keydown", keydown)?;
    on(window().as_ref(), "hashchange", |_| render(true))?;
    on(window().as_ref(), "storage", |_| {
        let saved = as_set(load(SAVED_KEY, json!([])));
        let checks = load(CHECKS_KEY, json!({}));
        let sources = as_set(load("inta:news-follows:v1", json!([])));
        mutate(|a| {
            a.favorites = saved;
            a.checks = checks;
            a.news_sources = sources;
            guides::migrate_checks(a);
        });
        render(false);
    })?;
    on(window().as_ref(), "beforeinstallprompt", |e| {
        e.prevent_default();
        PWA.with(|p| *p.borrow_mut() = Some(e));
        hidden("[data-install-pwa]", false);
    })?;
    render(false);
    if window().location().protocol().unwrap_or_default() != "file:" {
        let navigator = window().navigator();
        let promise = navigator.service_worker().register("./sw.js");
        spawn_local(async move {
            let _ = JsFuture::from(promise).await;
        });
    }
    Ok(())
}
fn on(
    target: &web_sys::EventTarget,
    name: &str,
    callback: impl FnMut(Event) + 'static,
) -> Result<(), JsValue> {
    let f = Closure::<dyn FnMut(Event)>::new(callback);
    target.add_event_listener_with_callback(name, f.as_ref().unchecked_ref())?;
    f.forget();
    Ok(())
}

fn title(name: &str) -> &str {
    match name {
        "home" => "Technician workspace",
        "tools" => "Tool library",
        "saved" => "Saved tools",
        "online" => "Online tools",
        "playbooks" => "Job playbooks",
        "job" => "Job guide",
        "tool" => "Tool details",
        "checklist" => "Checklist",
        "services" => "Service map",
        "news" => "News",
        "command-center" => "Command Center",
        "updates" => "Updates & Maintenance",
        "knowledge" => "Knowledge desk",
        "resources" => "Resources",
        _ => "Workspace",
    }
}
fn render(navigation: bool) {
    let route = Route::parse(&window().location().hash().unwrap_or_default());
    mutate(|a| a.route = route.clone());
    let content = with_app(App::render);
    html("#app", &content);
    let page_title = with_app(|a| {
        if route.name == "job" {
            a.job(&route.id)
                .map(|v| s(v, "title"))
                .unwrap_or("Job guide")
                .to_string()
        } else if route.name == "tool" {
            a.tool(&route.id)
                .map(|v| s(v, "name"))
                .unwrap_or("Tool details")
                .to_string()
        } else {
            title(&route.name).into()
        }
    });
    document().set_title(&format!("{page_title} | INTA Computer Services"));
    html(
        "#breadcrumb",
        &format!("Workspace <span>/</span> {}", esc(&page_title)),
    );
    for nav in all("[data-nav]") {
        let name = attr(&nav, "data-nav");
        let active = name == route.name
            || (route.name == "tool" && name == "tools")
            || (route.name == "job" && name == "playbooks");
        let _ = nav.class_list().toggle_with_force("active", active);
        if active {
            let _ = nav.set_attribute("aria-current", "page");
        } else {
            let _ = nav.remove_attribute("aria-current");
        }
    }
    update_counts();
    close_search();
    close_navigation();
    if route.name == "updates" {
        AGENT.with(|v| render_agent_status(&v.borrow()));
    }
    if let Some(a) = q("[data-back-results]") {
        let back = RETURN_TO.with(|s| s.borrow().clone());
        if !back.is_empty() {
            let _ = a.set_attribute("href", &back);
        }
    }
    let target = if route.name == "services" && !route.get("area").is_empty() {
        format!("service-{}", route.get("area"))
    } else if route.name == "job" && !route.get("section").is_empty() {
        format!("job-{}", route.get("section"))
    } else if route.name == "job" && !route.quick.is_empty() {
        format!("quick-{}", route.quick)
    } else if route.name == "online" && !route.get("tool").is_empty() {
        format!("online-service-{}", route.get("tool"))
    } else if route.name.starts_with("discipline-") {
        route.name.clone()
    } else {
        String::new()
    };
    if let Some(el) = document().get_element_by_id(&target) {
        el.scroll_into_view();
        if let Ok(Some(heading)) = el.query_selector("h2[tabindex],h3[tabindex]") {
            if let Some(h) = heading.dyn_ref::<HtmlElement>() {
                let _ = h.focus();
            }
        }
    } else if navigation {
        window().scroll_to_with_x_and_y(0., 0.);
        focus("#main");
    }
}
fn rerender_preserving_focus() {
    let active = document().active_element();
    let id = active.as_ref().map(Element::id).unwrap_or_default();
    let selection = active
        .as_ref()
        .and_then(|e| e.dyn_ref::<HtmlInputElement>())
        .map(|e| {
            (
                e.selection_start().ok().flatten(),
                e.selection_end().ok().flatten(),
            )
        });
    let y = window().scroll_y().unwrap_or(0.);
    render(false);
    window().scroll_to_with_x_and_y(0., y);
    if !id.is_empty() {
        if let Some(el) = document()
            .get_element_by_id(&id)
            .and_then(|e| e.dyn_into::<HtmlElement>().ok())
        {
            let _ = el.focus();
            if let (Some((Some(start), Some(end))), Some(i)) =
                (selection, el.dyn_ref::<HtmlInputElement>())
            {
                let _ = i.set_selection_range(start, end);
            }
        }
    }
}
fn update_counts() {
    with_app(|a| {
        text("#tool-count", &a.tools().len().to_string());
        text("#job-count", &a.jobs().len().to_string());
        text("#saved-count", &a.favorites.len().to_string());
        for el in all("[data-library-saved-count]") {
            el.set_text_content(Some(&a.favorites.len().to_string()));
        }
        for el in all("[data-save]") {
            let id = attr(&el, "data-save");
            let saved = a.favorites.contains(&id);
            let _ = el.set_attribute("aria-pressed", if saved { "true" } else { "false" });
            if let Ok(Some(label)) = el.query_selector("[data-save-label]") {
                label.set_text_content(Some(if saved {
                    "Saved to your toolkit"
                } else {
                    "Save to your toolkit"
                }));
            }
            if let Some(tool) = a.tool(&id) {
                let _ = el.set_attribute(
                    "title",
                    if saved {
                        "Remove from saved tools"
                    } else {
                        "Save this tool"
                    },
                );
                let _ = el.set_attribute(
                    "aria-label",
                    &format!(
                        "{} {}",
                        if saved { "Unsave" } else { "Save" },
                        s(tool, "name")
                    ),
                );
            }
        }
    });
}
fn toast(message: &str, undo: bool) {
    html("#toast",&format!("<span>{}</span>{}<button data-dismiss-toast aria-label=\"Dismiss message\">{}</button>",esc(message),if undo{"<button data-undo>Undo</button>"}else{""},icon("close")));
    hidden("#toast", false);
}
fn set_theme(theme: &str) {
    if let Some(root) = document().document_element() {
        let _ = root.set_attribute("data-theme", theme);
    }
    html(
        "#theme-toggle",
        &icon(if theme == "dark" { "sun" } else { "moon" }),
    );
    if let Some(toggle) = q("#theme-toggle") {
        let _ = toggle.set_attribute(
            "aria-label",
            &format!(
                "Switch to {} theme",
                if theme == "dark" { "light" } else { "dark" }
            ),
        );
    }
    if let Some(meta) = q("meta[name='theme-color']") {
        let _ = meta.set_attribute(
            "content",
            if theme == "dark" {
                "#101114"
            } else {
                "#f7f7f8"
            },
        );
    }
}
fn close_navigation() {
    if let Some(sidebar) = q("#sidebar") {
        let _ = sidebar.class_list().remove_1("is-open");
    }
    if let Some(menu) = q("#mobile-menu") {
        let _ = menu.set_attribute("aria-expanded", "false");
    }
    hidden("#nav-backdrop", true);
}
fn close_search() {
    hidden("#search-results", true);
    if let Some(search) = q("#global-search") {
        let _ = search.set_attribute("aria-expanded", "false");
    }
}
fn filter_tools() {
    let fields = [
        ("q", "#library-search"),
        ("category", "#category-filter"),
        ("platform", "#platform-filter"),
        ("tier", "#tier-filter"),
        ("mode", "#mode-filter"),
        ("sort", "#tool-sort"),
    ];
    let fields = fields.map(|(key, id)| (key, value(id)));
    mutate(|a| {
        for (key, v) in fields {
            if v.is_empty() {
                a.route.params.remove(key);
            } else {
                a.route.params.insert(key.into(), v);
            }
        }
    });
    persist_route();
    with_app(|a| {
        html("#tool-results", &catalog::results(a));
        html("#library-result-count", &catalog::result_count(a));
        html("#active-tool-filters", &catalog::filter_chips(a));
        text("#tool-sort-status", &catalog::sort_summary(a));
        hidden(
            "#tool-sort-status",
            catalog::sort_key(a.route.get("sort")) == "catalog",
        );
        for el in all("[data-library-category]") {
            let _ = el.set_attribute(
                "aria-pressed",
                if attr(&el, "data-library-category") == a.route.get("category") {
                    "true"
                } else {
                    "false"
                },
            );
        }
        if let Some(el) = q(".library-workspace") {
            let _ = el
                .class_list()
                .toggle_with_force("library-no-results", catalog::filtered(a).is_empty());
        }
    });
}
fn reset_tools() {
    for id in [
        "library-search",
        "category-filter",
        "platform-filter",
        "tier-filter",
        "mode-filter",
    ] {
        set_value(&format!("#{id}"), "");
    }
    set_value("#tool-sort", "catalog");
    filter_tools();
    focus("#library-search");
}
fn update_filter(key: &str, v: String) {
    let previous = with_app(|app| {
        if app.route.name == "online" && key == "q" {
            Some(
                a(&app.data["onlineTools"]["tools"])
                    .iter()
                    .filter(|entry| bench::online_matches(app, entry, &app.route))
                    .map(|entry| s(entry, "toolId").to_string())
                    .collect::<Vec<_>>(),
            )
        } else {
            None
        }
    });
    mutate(|app| {
        if v.is_empty() {
            app.route.params.remove(key);
        } else {
            app.route.params.insert(key.into(), v);
        }
    });
    persist_route();
    let identical = previous.is_some_and(|before| {
        with_app(|app| {
            before
                == a(&app.data["onlineTools"]["tools"])
                    .iter()
                    .filter(|entry| bench::online_matches(app, entry, &app.route))
                    .map(|entry| s(entry, "toolId").to_string())
                    .collect::<Vec<_>>()
        })
    });
    if !identical {
        rerender_preserving_focus();
    }
}
fn search_all() {
    let query = value("#global-search");
    if query.trim().is_empty() {
        close_search();
        return;
    }
    let content = with_app(|app| {
        let mut html = String::new();
        let results = app
            .tools()
            .iter()
            .filter(|t| {
                matches(
                    app.search_index
                        .get(s(t, "id"))
                        .map(String::as_str)
                        .unwrap_or(""),
                    &query,
                )
            })
            .collect::<Vec<_>>();
        if !results.is_empty() {
            html.push_str(&format!(
                "<div class=\"search-label\">Tools · {}</div>",
                results.len()
            ));
            for tool in results {
                html.push_str(&format!("<div class=\"search-tool-result\" data-search-tool=\"{}\"><div class=\"search-tool-main\">{}<small>{} · {}</small></div><a class=\"search-tool-details\" href=\"#tool/{}\">Details {}</a></div>",esc(s(tool,"id")),app.tool_name(tool),esc(category(s(tool,"category"))),esc(&a(&tool["platforms"]).iter().map(txt).collect::<Vec<_>>().join(", ")),encode(s(tool,"id")),icon("arrow")));
            }
        }
        let scenarios = a(&app.data["quickStart"]["items"])
            .iter()
            .filter(|x| matches(&x.to_string(), &query))
            .collect::<Vec<_>>();
        if !scenarios.is_empty() {
            html.push_str("<div class=\"search-label\">Fix a problem</div>");
            for x in scenarios {
                html.push_str(&search_row(
                    &format!(
                        "#job/{}/quick/{}",
                        encode(s(x, "jobId")),
                        encode(s(x, "id"))
                    ),
                    s(x, "label"),
                    s(x, "firstStep"),
                    "route",
                ));
            }
        }
        let jobs = app
            .jobs()
            .iter()
            .filter(|x| {
                matches(
                    &format!("{} {}", x, app.data["jobWorkspaces"]["jobs"][s(x, "id")]),
                    &query,
                )
            })
            .collect::<Vec<_>>();
        if !jobs.is_empty() {
            html.push_str("<div class=\"search-label\">Playbooks</div>");
            for x in jobs {
                html.push_str(&search_row(
                    &format!("#job/{}", encode(s(x, "id"))),
                    s(x, "title"),
                    s(x, "summary"),
                    "book",
                ));
            }
        }
        let notes = a(&app.data["fieldNotes"]["notes"])
            .iter()
            .filter(|x| matches(&x.to_string(), &query))
            .collect::<Vec<_>>();
        if !notes.is_empty() {
            html.push_str("<div class=\"search-label\">Specific fixes</div>");
            for x in notes {
                html.push_str(&search_row(
                    &format!(
                        "#job/{}?section=fix-{}",
                        encode(s(x, "jobId")),
                        encode(s(x, "id"))
                    ),
                    s(x, "title"),
                    s(x, "symptom"),
                    "wrench",
                ));
            }
        }
        for (key, route, label) in [
            ("commandCenter", "command-center", "Commands"),
            ("updatesCenter", "updates", "Updates & repairs"),
        ] {
            let items = command_search(&app.data[key]["commands"], &query);
            if !items.is_empty() {
                html.push_str(&format!("<div class=\"search-label\">{label}</div>"));
                for x in items {
                    html.push_str(&search_row(
                        &format!("#{route}?q={}", encode(s(x, "title"))),
                        s(x, "title"),
                        s(x, "summary"),
                        "terminal",
                    ));
                }
            }
        }
        if html.is_empty() {
            "<div class=\"search-empty\">No matches. Try a tool name or a symptom such as network or malware.</div>".into()
        } else {
            html
        }
    });
    html("#search-results", &content);
    hidden("#search-results", false);
    if let Some(search) = q("#global-search") {
        let _ = search.set_attribute("aria-expanded", "true");
    }
}
fn search_row(href: &str, label: &str, summary: &str, picture: &str) -> String {
    format!("<a class=\"search-result\" href=\"{}\"><span class=\"result-icon\">{}</span><span><strong>{}</strong><small>{}</small></span>{}</a>",esc(href),icon(picture),esc(label),esc(summary),icon("arrow"))
}

fn click(event: Event) {
    let Some(target) = event.target().and_then(|e| e.dyn_into::<Element>().ok()) else {
        return;
    };
    if closest(&target, ".skip-link").is_some() {
        event.prevent_default();
        focus("#main");
        return;
    }
    if closest(&target, "#theme-toggle").is_some() {
        let dark = document()
            .document_element()
            .is_some_and(|e| attr(&e, "data-theme") == "dark");
        let theme = if dark { "light" } else { "dark" };
        set_theme(theme);
        store(THEME_KEY, &json!(theme));
        return;
    }
    if closest(&target, "#mobile-menu").is_some() {
        if let Some(sidebar) = q("#sidebar") {
            let open = !sidebar.class_list().contains("is-open");
            let _ = sidebar.class_list().toggle_with_force("is-open", open);
            if let Some(menu) = q("#mobile-menu") {
                let _ = menu.set_attribute("aria-expanded", if open { "true" } else { "false" });
            }
            hidden("#nav-backdrop", !open);
            if open {
                focus("#sidebar a");
            }
        }
        return;
    }
    if closest(&target, "#nav-backdrop").is_some() {
        close_navigation();
        focus("#mobile-menu");
        return;
    }
    if closest(&target, "[data-dismiss-toast]").is_some() {
        hidden("#toast", true);
        return;
    }
    if let Some(toggle) = closest(&target, "[data-library-view-toggle]") {
        let view = attr(&toggle, "data-library-view-toggle");
        if view != "list" && view != "gallery" {
            return;
        }
        mutate(|a| {
            a.preferred_view = view.clone();
            a.route.params.insert("view".into(), view.clone());
        });
        if let Some(ws) = q(".library-workspace") {
            let _ = ws.set_attribute("data-library-view", &view);
        }
        for el in all("[data-library-view-toggle]") {
            let _ = el.set_attribute(
                "aria-pressed",
                if attr(&el, "data-library-view-toggle") == view {
                    "true"
                } else {
                    "false"
                },
            );
        }
        store(VIEW_KEY, &json!(view));
        persist_route();
        return;
    }
    if let Some(toggle) = closest(&target, "[data-library-filters]") {
        let expanded = attr(&toggle, "aria-expanded") != "true";
        let _ = toggle.set_attribute("aria-expanded", if expanded { "true" } else { "false" });
        if let Some(filters) = q("#tool-filters") {
            let _ = filters
                .class_list()
                .toggle_with_force("is-expanded", expanded);
        }
        return;
    }
    if let Some(category) = closest(&target, "[data-library-category]") {
        set_value(
            "#category-filter",
            &attr(&category, "data-library-category"),
        );
        filter_tools();
        return;
    }
    if let Some(clear) = closest(&target, "[data-clear-tool-filter]") {
        let id = attr(&clear, "data-clear-tool-filter");
        set_value(&format!("#{id}"), "");
        filter_tools();
        focus("#library-search");
        return;
    }
    if let Some(task) = closest(&target, "[data-online-task]") {
        event.prevent_default();
        let mut route = with_app(|app| app.route.clone());
        route
            .params
            .insert("task".into(), attr(&task, "data-online-task"));
        navigate(&route.href());
        return;
    }
    if closest(&target, "[data-reset-filters]").is_some() {
        reset_tools();
        return;
    }
    if let Some(detail) = closest(&target, "[data-library-detail]") {
        let _ = detail;
        RETURN_TO.with(|r| *r.borrow_mut() = window().location().hash().unwrap_or_default());
    }
    if closest(&target, "[data-copy-library]").is_some() {
        let mut r = with_app(|a| a.route.clone());
        r.name = "tools".into();
        let base = window()
            .location()
            .href()
            .unwrap_or_default()
            .split('#')
            .next()
            .unwrap_or("")
            .to_string();
        copy(
            &format!("{base}{}", r.href()),
            "Library view copied. Saved tools stay private to this device.",
        );
        return;
    }
    if let Some(save) = closest(&target, "[data-save]") {
        let id = attr(&save, "data-save");
        let Some(name) = with_app(|a| a.tool(&id).map(|t| s(t, "name").to_string())) else {
            return;
        };
        let saved = mutate(|a| {
            if a.favorites.remove(&id) {
                false
            } else {
                a.favorites.insert(id);
                true
            }
        });
        let data = with_app(|a| json!(a.favorites));
        store(SAVED_KEY, &data);
        update_counts();
        let route = with_app(|a| a.route.name.clone());
        if route == "saved" {
            filter_tools();
            if q("#tool-results [data-save]").is_some() {
                focus("#tool-results [data-save]");
            } else {
                focus("#library-search");
            }
        } else if route == "online" && is_checked("#online-saved-only") {
            rerender_preserving_focus();
            if q("#online-results [data-save]").is_some() {
                focus("#online-results [data-save]");
            } else {
                focus("#online-saved-only");
            }
        }
        toast(
            &format!(
                "{name} {} your toolkit.",
                if saved { "saved to" } else { "removed from" }
            ),
            false,
        );
        return;
    }
    if closest(&target, "[data-download-offline]").is_some() {
        download_offline();
        return;
    }
    if closest(&target, "[data-print]").is_some() {
        let _ = window().print();
        return;
    }
    if closest(&target, "[data-copy-link]").is_some() {
        copy(
            &window().location().href().unwrap_or_default(),
            "Link copied.",
        );
        return;
    }
    if let Some(button) = closest(&target, "[data-export-job]") {
        let id = attr(&button, "data-export-job");
        let content = with_app(|a| guides::export_job(a, &id));
        download(
            &format!("INTA-{id}-checklist.md"),
            &content,
            "text/markdown;charset=utf-8",
        );
        toast("Checklist exported with your current progress.", false);
        return;
    }
    if let Some(button) = closest(&target, "[data-reset-checklist]") {
        let id = attr(&button, "data-reset-checklist");
        let before = mutate(|a| {
            let before = a.checks[&id].clone();
            a.checks[&id] = json!({});
            before
        });
        UNDO.with(|u| *u.borrow_mut() = Some((id, before)));
        let checks = with_app(|a| a.checks.clone());
        store(CHECKS_KEY, &checks);
        rerender_preserving_focus();
        toast("Fresh checklist started.", true);
        return;
    }
    if closest(&target, "[data-undo]").is_some() {
        if let Some((id, state)) = UNDO.with(|u| u.borrow_mut().take()) {
            mutate(|a| a.checks[&id] = state);
            store(CHECKS_KEY, &with_app(|a| a.checks.clone()));
            rerender_preserving_focus();
            toast("Previous checklist restored.", false);
        }
        return;
    }
    for (attribute, key) in [
        ("data-command-category", "category"),
        ("data-update-category", "category"),
        ("data-online-group", "group"),
    ] {
        if let Some(el) = closest(&target, &format!("[{attribute}]")) {
            update_filter(key, attr(&el, attribute));
            return;
        }
    }
    for attribute in ["data-reset-online", "data-reset-jobs"] {
        if closest(&target, &format!("[{attribute}]")).is_some() {
            mutate(|a| a.route.params.clear());
            persist_route();
            rerender_preserving_focus();
            return;
        }
    }
    if let Some(el) = closest(&target, "[data-news-save]") {
        let id = attr(&el, "data-news-save");
        mutate(|a| {
            if !a.news_sources.remove(&id) {
                a.news_sources.insert(id);
            }
        });
        store("inta:news-follows:v1", &with_app(|a| json!(a.news_sources)));
        rerender_preserving_focus();
        return;
    }
    if let Some(button) = closest(&target, "[data-media-index]") {
        load_media(&button);
        return;
    }
    if closest(&target, "[data-news-refresh]").is_some() {
        refresh_news();
        return;
    }
    if closest(&target, "[data-agent-connect]").is_some() {
        spawn_local(async {
            if let Err(e) = connect_agent().await {
                toast(
                    &format!(
                        "{}. Start INTA Local Agent on this computer, then connect again.",
                        error_text(e)
                    ),
                    false,
                );
            }
        });
        return;
    }
    if closest(&target, "[data-agent-disconnect]").is_some() {
        clear_token();
        AGENT.with(|a| *a.borrow_mut() = Value::Null);
        render_agent_status(&Value::Null);
        html("#agent-inventory", "<p>Disconnected.</p>");
        html("#agent-operations", "");
        return;
    }
    if closest(&target, "[data-agent-refresh]").is_some() {
        spawn_local(async {
            if let Err(e) = refresh_agent().await {
                toast(&error_text(e), false);
            }
        });
        return;
    }
    if let Some(el) = closest(&target, "[data-agent-update-package]") {
        set_value("#agent-manager", &attr(&el, "data-agent-package-manager"));
        set_value("#agent-package-id", &attr(&el, "data-agent-update-package"));
        run_agent("update");
        return;
    }
    if closest(&target, "[data-agent-run]").is_some() {
        run_agent(&value("#agent-action"));
        return;
    }
    if closest(&target, "[data-agent-update-all]").is_some() {
        run_agent("update-all");
        return;
    }
    if closest(&target, "[data-install-pwa]").is_some() {
        PWA.with(|p| {
            if let Some(e) = p.borrow_mut().take() {
                if let Ok(f) = js_sys::Reflect::get(e.as_ref(), &"prompt".into())
                    .and_then(|v| v.dyn_into::<js_sys::Function>())
                {
                    let _ = f.call0(e.as_ref());
                }
            } else {
                toast(
                    "Use your browser menu to install INTA on this device.",
                    false,
                );
            }
        });
        return;
    }
    if let Some(command) = copy_action(&target) {
        copy(
            &command,
            "Command copied. Review its shell, impact and safety notes before running it.",
        );
        return;
    }
    if closest(&target, ".global-search-wrap").is_none() {
        close_search();
    }
    if let Some(row) = closest(&target, "[data-job-route]") {
        if closest(&target, "a,button,input,select,textarea").is_none() {
            navigate(&attr(&row, "data-job-route"));
        }
    }
    if let Some(link) = closest(&target, "a[href^='#']") {
        if attr(&link, "href") == window().location().hash().unwrap_or_default() {
            event.prevent_default();
            render(true);
        }
    }
}

fn copy_action(target: &Element) -> Option<String> {
    for (attribute, data_key, workflow) in [
        ("data-copy-center-command", "commandCenter", false),
        ("data-copy-update-command", "updatesCenter", false),
        ("data-copy-center-workflow", "commandCenter", true),
        ("data-copy-update-workflow", "updatesCenter", true),
    ] {
        if let Some(el) = closest(target, &format!("[{attribute}]")) {
            let id = attr(&el, attribute);
            return with_app(|app| {
                let commands = a(&app.data[data_key]["commands"]);
                if workflow {
                    let flow = a(&app.data[data_key]["workflows"])
                        .iter()
                        .find(|v| s(v, "id") == id)?;
                    Some(a(&flow["commandIds"]).iter().filter_map(|id|commands.iter().find(|c|c["id"]==*id)).map(|c|format!("# {} [{} · {} · {}]\n{}\n# Safety: {}\n# Official reference: {}",s(c,"title"),s(c,"shell"),s(c,"privilege"),s(c,"risk"),s(c,"code"),s(c,"safety"),s(c,"sourceUrl"))).collect::<Vec<_>>().join("\n\n"))
                } else {
                    commands
                        .iter()
                        .find(|v| s(v, "id") == id)
                        .map(|v| s(v, "code").to_string())
                }
            });
        }
    }
    if let Some(el) = closest(target, "[data-copy-command]") {
        let id = attr(&el, "data-copy-command");
        let index = attr(&el, "data-command-index").parse::<usize>().ok()?;
        return with_app(|a| {
            a.job(&id)
                .and_then(|j| j["steps"].get(index))
                .map(|s| txt(&s["command"]))
        });
    }
    if let Some(el) = closest(target, "[data-copy-field-command]") {
        let id = attr(&el, "data-copy-field-command");
        let index = attr(&el, "data-command-index").parse::<usize>().ok()?;
        return with_app(|app| {
            a(&app.data["fieldNotes"]["notes"])
                .iter()
                .find(|n| s(n, "id") == id)
                .and_then(|n| n["commands"].get(index))
                .map(|c| s(c, "code").to_string())
        });
    }
    None
}
fn input(event: Event) {
    let Some(target) = event.target().and_then(|t| t.dyn_into::<Element>().ok()) else {
        return;
    };
    match target.id().as_str() {
        "global-search" => search_all(),
        "library-search" => filter_tools(),
        "jobs-search" | "command-center-search" | "updates-search" | "online-search" => {
            update_filter("q", value(&format!("#{}", target.id())))
        }
        "news-search" => update_filter("q", value("#news-search")),
        "source-search" => update_filter("q", value("#source-search")),
        _ => {}
    }
}
fn change(event: Event) {
    let Some(target) = event.target().and_then(|t| t.dyn_into::<Element>().ok()) else {
        return;
    };
    if target.has_attribute("data-check-job") {
        let id = attr(&target, "data-check-job");
        let kind = attr(&target, "data-check-type");
        let index = attr(&target, "data-check-index").parse().unwrap_or(0);
        let title = attr(&target, "data-check-title");
        let checked = target
            .dyn_ref::<HtmlInputElement>()
            .is_some_and(|i| i.checked());
        let key = with_app(|a| {
            a.job(&id)
                .map(|j| guides::checklist_key(j, &kind, index, &title))
        });
        if let Some(key) = key {
            mutate(|a| {
                if !a.checks[&id].is_object() {
                    a.checks[&id] = json!({});
                }
                a.checks[&id][key] = json!(checked);
            });
            store(CHECKS_KEY, &with_app(|a| a.checks.clone()));
            let (done, total) = with_app(|a| {
                a.job(&id)
                    .map(|j| guides::progress(a, j, a.route.name == "checklist"))
                    .unwrap_or((0, 0))
            });
            let percent = if total == 0 { 0 } else { done * 100 / total };
            text("#progress-percentage", &format!("{percent}%"));
            text("#progress-count", &format!("{done} of {total} checks"));
            for track in all(".progress-track") {
                let _ = track.set_attribute("aria-valuenow", &percent.to_string());
                if let Some(child) = track
                    .first_element_child()
                    .and_then(|e| e.dyn_into::<HtmlElement>().ok())
                {
                    let _ = child.style().set_property("width", &format!("{percent}%"));
                }
            }
            if done == total && total > 0 && checked {
                toast(
                    "Checklist complete. Record the verified result in your service ticket.",
                    false,
                );
            }
        }
        return;
    }
    match target.id().as_str() {
        "category-filter" | "platform-filter" | "tier-filter" | "mode-filter" | "tool-sort" => {
            filter_tools()
        }
        "jobs-category" => update_filter("category", value("#jobs-category")),
        "online-group" => update_filter("group", value("#online-group")),
        "online-task-only" => update_filter(
            "scope",
            if is_checked("#online-task-only") {
                "task"
            } else {
                ""
            }
            .into(),
        ),
        "online-saved-only" => update_filter(
            "saved",
            if is_checked("#online-saved-only") {
                "1"
            } else {
                ""
            }
            .into(),
        ),
        "command-center-shell" | "updates-shell" => {
            update_filter("shell", value(&format!("#{}", target.id())))
        }
        "command-center-risk" | "updates-risk" => {
            update_filter("risk", value(&format!("#{}", target.id())))
        }
        "news-category" => update_filter("category", value("#news-category")),
        "news-saved" => update_filter(
            "saved",
            if is_checked("#news-saved") { "1" } else { "" }.into(),
        ),
        "news-digest-category" => update_filter("digest-category", value("#news-digest-category")),
        "news-media-only" => update_filter(
            "media",
            if is_checked("#news-media-only") {
                "1"
            } else {
                ""
            }
            .into(),
        ),
        _ => {}
    }
}
fn keydown(event: Event) {
    let Ok(key) = event.dyn_into::<KeyboardEvent>() else {
        return;
    };
    let active = document().active_element();
    let typing = active.as_ref().is_some_and(|e| {
        matches!(e.tag_name().as_str(), "INPUT" | "TEXTAREA" | "SELECT")
            || e.has_attribute("contenteditable")
    });
    if (key.key() == "/" && !typing)
        || ((key.ctrl_key() || key.meta_key()) && key.key().to_lowercase() == "k")
    {
        key.prevent_default();
        focus("#global-search");
        if let Some(i) = q("#global-search").and_then(|e| e.dyn_into::<HtmlInputElement>().ok()) {
            i.select();
        }
    } else if key.key() == "Escape" {
        close_search();
        close_navigation();
    } else if !q("#search-results").is_none_or(|e| e.has_attribute("hidden"))
        && matches!(key.key().as_str(), "ArrowDown" | "ArrowUp" | "Enter")
    {
        let links = all("#search-results a");
        let position = active
            .as_ref()
            .and_then(|a| links.iter().position(|e| e == a));
        let search_active = active.as_ref().is_some_and(|e| e.id() == "global-search");
        if search_active && (key.key() == "ArrowDown" || key.key() == "Enter") {
            key.prevent_default();
            if let Some(el) = links.first().and_then(|e| e.dyn_ref::<HtmlElement>()) {
                let _ = el.focus();
            }
        } else if let Some(i) = position {
            if key.key() == "ArrowUp" || key.key() == "ArrowDown" {
                key.prevent_default();
                if key.key() == "ArrowUp" && i == 0 {
                    focus("#global-search");
                } else {
                    let next = if key.key() == "ArrowUp" {
                        i - 1
                    } else {
                        (i + 1).min(links.len() - 1)
                    };
                    if let Some(el) = links[next].dyn_ref::<HtmlElement>() {
                        let _ = el.focus();
                    }
                }
            }
        }
    }
}
fn copy(content: &str, message: &str) {
    let content = content.to_string();
    let message = message.to_string();
    spawn_local(async move {
        let result = JsFuture::from(window().navigator().clipboard().write_text(&content)).await;
        if result.is_ok() {
            toast(&message, false);
            return;
        }
        let Ok(el) = document().create_element("textarea") else {
            return;
        };
        let Ok(input) = el.dyn_into::<HtmlTextAreaElement>() else {
            return;
        };
        input.set_value(&content);
        let _ = input.style().set_property("position", "fixed");
        let _ = input.style().set_property("opacity", "0");
        if let Some(body) = document().body() {
            let _ = body.append_child(&input);
        }
        input.select();
        let copied = js_sys::Reflect::get(document().as_ref(), &"execCommand".into())
            .ok()
            .and_then(|f| f.dyn_into::<js_sys::Function>().ok())
            .and_then(|f| f.call1(document().as_ref(), &"copy".into()).ok())
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        input.remove();
        toast(
            if copied {
                &message
            } else {
                "Clipboard access is unavailable. Select the command text and copy it manually."
            },
            false,
        );
    });
}
fn download(filename: &str, content: &str, mime: &str) {
    let parts = js_sys::Array::new();
    parts.push(&content.into());
    let options = web_sys::BlobPropertyBag::new();
    options.set_type(mime);
    let Ok(blob) = web_sys::Blob::new_with_str_sequence_and_options(&parts, &options) else {
        return;
    };
    let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) else {
        return;
    };
    let Ok(el) = document().create_element("a") else {
        return;
    };
    let Ok(a) = el.dyn_into::<web_sys::HtmlAnchorElement>() else {
        return;
    };
    a.set_href(&url);
    a.set_download(filename);
    if let Some(body) = document().body() {
        let _ = body.append_child(&a);
    }
    a.click();
    a.remove();
    let release = Closure::once_into_js(move || {
        let _ = web_sys::Url::revoke_object_url(&url);
    });
    let _ = window()
        .set_timeout_with_callback_and_timeout_and_arguments_0(release.unchecked_ref(), 5000);
}
fn download_offline() {
    let Some(root) = document().document_element() else {
        return;
    };
    let Ok(clone) = root
        .clone_node_with_deep(true)
        .and_then(|n| n.dyn_into::<Element>().map_err(JsValue::from))
    else {
        return;
    };
    for selector in ["#app", "#search-results", "#toast"] {
        if let Ok(Some(el)) = clone.query_selector(selector) {
            el.set_inner_html("");
        }
    }
    if let Ok(Some(el)) = clone.query_selector("#global-search") {
        let _ = el.set_attribute("value", "");
    }
    download(
        "INTA-offline.html",
        &format!("<!doctype html>\n{}", clone.outer_html()),
        "text/html;charset=utf-8",
    );
    toast(
        "Complete offline workspace downloaded. External downloads still need a connection.",
        false,
    );
}

fn load_media(button: &Element) {
    let Some(story) = closest(button, "[data-digest-id]") else {
        return;
    };
    let id = attr(&story, "data-digest-id");
    let index = attr(button, "data-media-index")
        .parse::<usize>()
        .unwrap_or(0);
    let markup = with_app(|app| {
        a(&app.data["newsDigest"]["items"])
            .iter()
            .find(|x| s(x, "id") == id)
            .and_then(|item| bench::media_markup(item, index))
    });
    if let Some(markup) = markup {
        if let Some(parent) = button.parent_element() {
            parent.set_inner_html(&markup);
        }
    }
}
fn refresh_news() {
    spawn_local(async {
        match request("./news-digest.json", "GET", None, "").await {
            Ok(data) if data["items"].is_array() => {
                mutate(|a| a.data["newsDigest"] = data);
                render(false);
                toast("Publisher snapshot refreshed.", false);
            }
            Ok(_) => toast(
                "The publisher snapshot did not have a valid item list.",
                false,
            ),
            Err(e) => toast(
                &format!(
                    "Refresh unavailable: {}. The bundled snapshot is still available.",
                    error_text(e)
                ),
                false,
            ),
        }
    });
}
fn error_text(value: JsValue) -> String {
    value
        .as_string()
        .unwrap_or_else(|| "The request could not complete".into())
}
async fn request(
    url: &str,
    method: &str,
    body: Option<Value>,
    token: &str,
) -> Result<Value, JsValue> {
    let opts = RequestInit::new();
    opts.set_method(method);
    opts.set_mode(RequestMode::Cors);
    opts.set_cache(RequestCache::NoStore);
    opts.set_credentials(RequestCredentials::Omit);
    if let Some(body) = body {
        opts.set_body(&body.to_string().into());
    }
    let request = Request::new_with_str_and_init(url, &opts)?;
    if !token.is_empty() {
        request
            .headers()
            .set("Authorization", &format!("Bearer {token}"))?;
    }
    if method == "POST" {
        request.headers().set("Content-Type", "application/json")?;
    }
    let response = JsFuture::from(window().fetch_with_request(&request))
        .await?
        .dyn_into::<Response>()?;
    let raw = JsFuture::from(response.text()?)
        .await?
        .as_string()
        .unwrap_or_default();
    let data: Value = serde_json::from_str(&raw).unwrap_or(Value::Null);
    if !response.ok() {
        return Err(JsValue::from_str(&format!(
            "{} (HTTP {})",
            if s(&data, "error").is_empty() {
                "Request refused"
            } else {
                s(&data, "error")
            },
            response.status()
        )));
    }
    Ok(data)
}
async fn agent_request(path: &str, method: &str, body: Option<Value>) -> Result<Value, JsValue> {
    request(
        &format!("http://127.0.0.1:47821{path}"),
        method,
        body,
        &session_token(),
    )
    .await
}
async fn connect_agent() -> Result<(), JsValue> {
    if !session_token().is_empty() {
        if let Ok(status) = agent_request("/api/v1/status", "GET", None).await {
            AGENT.with(|v| *v.borrow_mut() = status.clone());
            render_agent_status(&status);
            return refresh_agent().await;
        }
        clear_token();
    }
    let pair = request("http://127.0.0.1:47821/api/v1/pair", "POST", None, "").await?;
    let token = s(&pair, "token");
    if token.is_empty() {
        return Err("Agent did not provide a session token".into());
    }
    let storage = window()
        .session_storage()?
        .ok_or_else(|| JsValue::from_str("Session storage is unavailable"))?;
    storage.set_item("inta-local-agent-token", token)?;
    refresh_agent().await?;
    toast("Connected to this computer.", false);
    Ok(())
}
async fn refresh_agent() -> Result<(), JsValue> {
    let status = agent_request("/api/v1/status", "GET", None).await?;
    AGENT.with(|v| *v.borrow_mut() = status.clone());
    render_agent_status(&status);
    let inventory = agent_request("/api/v1/inventory", "GET", None).await?;
    html("#agent-inventory", &bench::agent_inventory(&inventory));
    let operations = agent_request("/api/v1/operations", "GET", None).await?;
    html("#agent-operations", &bench::agent_operations(&operations));
    Ok(())
}
fn render_agent_status(status: &Value) {
    let connected = status.is_object();
    text(
        "#agent-connection-status",
        &if connected {
            format!(
                "{} · {} · {}",
                s(status, "hostname"),
                if status["elevated"] == true {
                    "Administrator"
                } else {
                    "Standard privileges"
                },
                s(status, "platform")
            )
        } else {
            "Not connected".into()
        },
    );
    hidden("[data-agent-connect]", connected);
    hidden("[data-agent-disconnect]", !connected);
    if let Some(b) = q("[data-agent-refresh]") {
        if connected {
            let _ = b.remove_attribute("disabled");
        } else {
            let _ = b.set_attribute("disabled", "");
        }
    }
    if connected {
        let previous = value("#agent-manager");
        html(
            "#agent-manager",
            &a(&status["managers"])
                .iter()
                .map(|v| format!("<option>{}</option>", esc(&txt(v))))
                .collect::<String>(),
        );
        if a(&status["managers"])
            .iter()
            .any(|v| v.as_str() == Some(&previous))
        {
            set_value("#agent-manager", &previous);
        }
        if let Some(select) = q("#agent-manager") {
            let _ = select.remove_attribute("disabled");
        }
        text(
            "#agent-capabilities",
            &format!(
                "Available: {}. {}",
                a(&status["managers"])
                    .iter()
                    .map(txt)
                    .collect::<Vec<_>>()
                    .join(", "),
                if status["elevated"] == true {
                    "Elevated session."
                } else {
                    "Standard session. Relaunch the agent as administrator for system-wide changes."
                }
            ),
        );
    }
}
fn run_agent(action: &str) {
    let manager = value("#agent-manager");
    if manager.is_empty() {
        toast(
            "Connect the agent and select an installed package manager.",
            false,
        );
        return;
    }
    if manager == "windows-update" && (action == "update" || action == "update-all") {
        let _ = window().location().set_href("ms-settings:windowsupdate");
        return;
    }
    let package = if action == "update-all" || (manager == "windows-update" && action == "repair") {
        "*".into()
    } else {
        value("#agent-package-id").trim().to_string()
    };
    if package.is_empty() {
        toast(
            "Enter the exact package ID or KB number from the preview.",
            false,
        );
        return;
    }
    let description =
        format!("{action} {package} using {manager}? This changes software on this computer.");
    if !window().confirm_with_message(&description).unwrap_or(false) {
        return;
    }
    let body = json!({"manager":manager,"action":action,"packageId":package,"confirmed":true});
    spawn_local(async move {
        match agent_request("/api/v1/operations", "POST", Some(body)).await {
            Ok(_) => {
                toast(
                    "Operation queued. Refresh progress to see its actual result.",
                    false,
                );
                if let Ok(ops) = agent_request("/api/v1/operations", "GET", None).await {
                    html("#agent-operations", &bench::agent_operations(&ops));
                }
            }
            Err(e) => toast(&error_text(e), false),
        }
    });
}
