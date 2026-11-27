use crate::{bench, common::*, guides, App};
use serde_json::{json, Value};
use std::cmp::Ordering;
use std::collections::BTreeSet;

pub const SORTS:&[(&str,&str,&str)]=&[
 ("catalog","Catalog order","Technician tools first, followed by online services."),
 ("name","Name · A to Z","Alphabetical by tool name."),("name-desc","Name · Z to A","Reverse alphabetical by tool name."),
 ("recommendation","INTA recommendation","Everyday picks, then Specialist and Evaluate first. Editorial guidance is separate from user ratings."),
 ("reviewed","Recently reviewed","Most recently reviewed source records first."),
 ("stars","GitHub stars","Matched project repository stars measure interest, not quality."),
 ("rating","User rating","SourceForge user ratings, with review count breaking ties. Providers are not mixed."),
 ("reviews","User review count","Number of SourceForge user ratings; sample size is not sentiment."),
 ("popularity","Popularity · downloads","SourceForge weekly downloads are not unique users or endorsements."),
 ("users","Weekly active users","Publisher-reported users for the exact product and linked reporting date."),
 ("forks","GitHub forks","Matched project repository forks. Unmeasured tools remain visible.")];
pub const CATEGORIES: &[&str] = &[
    "malware",
    "security",
    "remote",
    "windows",
    "storage",
    "hardware",
    "network",
    "boot",
    "productivity",
    "ai",
    "online",
];
pub fn category_icon(category: &str) -> &str {
    match category {
        "ai" => "terminal",
        "boot" => "usb",
        "hardware" => "activity",
        "malware" => "shield",
        "network" => "network",
        "productivity" => "wrench",
        "remote" | "windows" => "monitor",
        "security" => "lock",
        "storage" => "drive",
        "online" => "globe",
        _ => "wrench",
    }
}
fn library_name(app: &App, tool: &Value) -> String {
    let brand = a(&app.data["brands"]["brands"])
        .iter()
        .find(|b| s(b, "toolId") == s(tool, "id"));
    let image = brand
        .and_then(|b| app.data["brands"]["assets"][s(b, "assetId")].as_str())
        .filter(|v| v.starts_with("data:image/"));
    let art = image
        .map(|src| {
            format!(
                "<img class=\"library-brand-icon\" src=\"{}\" alt=\"\" width=\"40\" height=\"40\">",
                esc(src)
            )
        })
        .unwrap_or_else(|| icon(category_icon(s(tool, "category"))));
    let url = s(&app.data["toolAccess"][s(tool, "id")], "officialUrl");
    let url = if url.is_empty() {
        s(tool, "homepage")
    } else {
        url
    };
    external(url,&format!("<span class=\"library-art\" aria-hidden=\"true\">{art}</span><span class=\"library-tool-title\">{}</span>",esc(s(tool,"name"))),"library-official-title").replacen("<a ","<a data-brand-decorated=\"true\" ",1)
}
fn choice_url(app: &App, choice: &Value) -> String {
    let record = &app.data["toolAccess"][s(choice, "toolId")];
    let action = s(choice, "actionId");
    if action.is_empty() {
        return s(&record["primary"], "url").to_string();
    }
    std::iter::once(&record["primary"])
        .chain(a(&record["alternates"]).iter())
        .find(|option| s(option, "id") == action)
        .map(|option| s(option, "url").to_string())
        .filter(|url| !url.is_empty())
        .unwrap_or_else(|| format!("action:{action}"))
}
/// Every authored choice remains; add the original scenario action only when its
/// exact acquisition destination is not represented by an authored choice.
pub fn normalized_choices(app: &App, item: &Value) -> Vec<Value> {
    let mut choices: Vec<_> = a(&item["choices"])
        .iter()
        .filter(|c| app.tool(s(c, "toolId")).is_some())
        .cloned()
        .collect();
    if app.tool(s(item, "toolId")).is_some() {
        let original = json!({"toolId":s(item,"toolId"),"actionId":s(item,"toolAction"),"role":"Starting tool","why":s(item,"why"),"freeUse":s(item,"freeUse"),"evidenceUrls":item["evidenceUrls"],"reviewedAt":s(&app.data["quickStart"],"reviewedAt")});
        let url = choice_url(app, &original);
        if !choices.iter().any(|choice| {
            s(choice, "toolId") == s(&original, "toolId") && choice_url(app, choice) == url
        }) {
            choices.push(original);
        }
    }
    choices
}
pub fn sort_key(key: &str) -> &str {
    if key == "tier" {
        "recommendation"
    } else if SORTS.iter().any(|x| x.0 == key) {
        key
    } else {
        "catalog"
    }
}
fn provider<'a>(tool: &'a Value, key: &str) -> &'a Value {
    &tool["reputation"][match key {
        "stars" | "forks" => "github",
        "users" => "usage",
        _ => "sourceforge",
    }]
}
pub fn metric(tool: &Value, key: &str) -> Option<f64> {
    if key == "recommendation" {
        return Some(match s(tool, "tier") {
            "core" => 3.,
            "specialist" => 2.,
            _ => 1.,
        });
    }
    if key == "reviewed" {
        return s(tool, "reviewedAt").replace('-', "").parse().ok();
    }
    let p = provider(tool, key);
    if s(p, "status") != "observed" {
        return None;
    }
    let v = match key {
        "stars" => &p["stars"],
        "forks" => &p["forks"],
        "rating" => &p["rating"]["value"],
        "reviews" => &p["rating"]["reviews"],
        "popularity" => &p["weeklyDownloads"],
        "users" => &p["weeklyActiveUsers"],
        _ => &Value::Null,
    };
    v.as_f64().filter(|v| v.is_finite())
}
pub fn filtered<'a>(app: &'a App) -> Vec<&'a Value> {
    let r = &app.route;
    let mut results: Vec<_> = app
        .tools()
        .iter()
        .filter(|tool| {
            let id = s(tool, "id");
            let hay = app.search_index.get(id).map(String::as_str).unwrap_or("");
            (r.name != "saved" || app.favorites.contains(id))
                && matches(hay, r.get("q"))
                && (r.get("category").is_empty() || s(tool, "category") == r.get("category"))
                && (r.get("platform").is_empty()
                    || a(&tool["platforms"])
                        .iter()
                        .any(|p| p.as_str() == Some(r.get("platform"))))
                && (r.get("tier").is_empty() || s(tool, "tier") == r.get("tier"))
                && (r.get("mode").is_empty()
                    || s(tool, "mode")
                        .split('/')
                        .any(|m| m.trim() == r.get("mode")))
        })
        .collect();
    let key = sort_key(r.get("sort"));
    let by_name = |l: &&Value, r: &&Value| {
        s(l, "name")
            .to_lowercase()
            .cmp(&s(r, "name").to_lowercase())
            .then(s(l, "id").cmp(s(r, "id")))
    };
    match key {
        "catalog" => results.sort_by_key(|t| s(t, "category") == "online"),
        "name" => results.sort_by(by_name),
        "name-desc" => results.sort_by(|l, r| by_name(r, l)),
        _ => results.sort_by(|l, r| match (metric(l, key), metric(r, key)) {
            (Some(a), Some(b)) => b
                .partial_cmp(&a)
                .unwrap_or(Ordering::Equal)
                .then_with(|| {
                    if key == "rating" {
                        metric(r, "reviews")
                            .unwrap_or(0.)
                            .partial_cmp(&metric(l, "reviews").unwrap_or(0.))
                            .unwrap_or(Ordering::Equal)
                    } else {
                        Ordering::Equal
                    }
                })
                .then_with(|| by_name(l, r)),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => by_name(l, r),
        }),
    };
    results
}
fn number(value: f64) -> String {
    let raw = if value.fract() == 0. {
        format!("{value:.0}")
    } else {
        format!("{value}")
    };
    let (int, frac) = raw.split_once('.').unwrap_or((&raw, ""));
    let digits = int
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            if i > 0 && i % 3 == 0 {
                format!(",{c}")
            } else {
                c.to_string()
            }
        })
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>();
    if frac.is_empty() {
        digits
    } else {
        format!("{digits}.{frac}")
    }
}
pub fn evidence_signal(tool: &Value, key: &str) -> String {
    if key == "catalog" || key == "name" || key == "name-desc" || key == "recommendation" {
        return String::new();
    }
    let value = metric(tool, key);
    let p = provider(tool, key);
    let status = if key == "reviewed" {
        "Review date unavailable"
    } else {
        match s(p, "status") {
            "unavailable" => "Source unavailable",
            "unreported" | "observed" => "Not reported by source",
            "not-applicable" => "Not applicable",
            _ => "No verified data yet",
        }
    };
    let Some(value) = value else {
        return format!("<div class=\"tool-sort-signal unknown\">{status}</div>");
    };
    if key == "reviewed" {
        return format!(
            "<div class=\"tool-sort-signal\">Source reviewed <time datetime=\"{}\">{}</time></div>",
            esc(s(tool, "reviewedAt")),
            esc(s(tool, "reviewedAt"))
        );
    }
    let suffix = match key {
        "stars" => "GitHub stars",
        "forks" => "GitHub forks",
        "reviews" => "SourceForge ratings",
        "popularity" => "SourceForge downloads this week",
        "users" => "reported weekly active users",
        _ => "SourceForge rating",
    };
    let label = if key == "rating" {
        format!(
            "{}/{} · {} SourceForge ratings",
            number(value),
            txt(&p["rating"]["scale"]),
            txt(&p["rating"]["reviews"])
        )
    } else {
        format!(
            "{}{} {suffix}",
            if key == "users" && p["approximate"] == true {
                "≈"
            } else {
                ""
            },
            number(value)
        )
    };
    let date = if key == "users" {
        s(p, "reportedAt")
    } else {
        s(p, "checkedAt")
    };
    format!(
        "<div class=\"tool-sort-signal\">{} <time datetime=\"{}\">{}</time></div>",
        external(s(p, "url"), &esc(&label), ""),
        esc(date),
        esc(date)
    )
}
pub fn article(app: &App, tool: &Value, sort: &str) -> String {
    let id = s(tool, "id");
    let rec = &app.data["toolAccess"][id];
    let alternatives = a(&rec["alternates"]);
    let more = if alternatives.is_empty() {
        String::new()
    } else {
        format!("<details class=\"library-alternates\"><summary>{} more access {}</summary><ul>{}</ul></details>",alternatives.len(),if alternatives.len()==1{"option"}else{"options"},alternatives.iter().map(|option|format!("<li>{}</li>",external(s(option,"url"),&format!("{} {}",esc(s(option,"label")),icon(if s(option,"kind")=="file"{"download"}else{"external"})),"library-alternate-link"))).collect::<String>())
    };
    format!("<article class=\"library-record\" role=\"listitem\" data-tool-id=\"{}\" data-tool-card=\"{}\"><div class=\"library-identity\"><h2>{}</h2><p>{}</p><span class=\"library-category-label\">{} <span aria-hidden=\"true\">·</span> {}</span></div><div class=\"library-context\"><div class=\"library-platforms\" aria-label=\"Platforms\">{}</div>{}{}</div><div class=\"library-links\">{}<div class=\"library-secondary\"><a href=\"#tool/{}\" data-library-detail=\"{}\">Details</a>{}</div>{}</div><div class=\"library-save\">{}</div></article>",esc(id),esc(id),library_name(app,tool),formatted(s(tool,"summary")),esc(category(s(tool,"category"))),esc(mode(s(tool,"mode"))),a(&tool["platforms"]).iter().map(|p|format!("<span>{}</span>",esc(&txt(p)))).collect::<String>(),tier_pill(s(tool,"tier")),evidence_signal(tool,sort),app.access_actions(tool,"",true),encode(id),esc(id),external(s(tool,"docsUrl"),&format!("Documentation {}",icon("external")),"library-doc-link"),more,app.save_button(tool))
}
pub fn results(app: &App) -> String {
    let found = filtered(app);
    if found.is_empty() {
        let body = if app.route.name == "saved" && app.favorites.is_empty() {
            empty(
                "Your toolkit is ready to fill",
                "Save a tool using its bookmark button. Your choices stay on this device.",
                "#tools",
                "Browse tools",
            )
        } else {
            "<div class=\"empty-state\"><h2>No tools match these filters</h2><p>Try a different name, platform or category.</p><button class=\"button primary\" data-reset-filters>Clear filters</button></div>".into()
        };
        return format!("<div class=\"library-empty\" role=\"listitem\">{body}</div>");
    }
    found
        .iter()
        .map(|t| article(app, t, sort_key(app.route.get("sort"))))
        .collect()
}
fn select(id: &str, label: &str, content: &str) -> String {
    format!("<label class=\"library-filter\" for=\"{id}\"><span>{label}</span><select id=\"{id}\" class=\"filter-select\"{}>{content}</select></label>",if id=="tool-sort"{" aria-describedby=\"tool-sort-status\""}else{""})
}
fn named_options(values: &[&str], any: &str, selected: &str, label: fn(&str) -> &str) -> String {
    format!(
        "<option value=\"\">{}</option>{}",
        esc(any),
        values
            .iter()
            .map(|v| format!(
                "<option value=\"{}\"{}>{}</option>",
                esc(v),
                if *v == selected { " selected" } else { "" },
                esc(label(v))
            ))
            .collect::<String>()
    )
}
pub fn library(app: &App, embedded: bool) -> String {
    let r = &app.route;
    let saved = r.name == "saved";
    let view = app.view();
    let sort = sort_key(r.get("sort"));
    let platforms = app
        .tools()
        .iter()
        .flat_map(|t| a(&t["platforms"]).iter().map(txt))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let modes = app
        .tools()
        .iter()
        .flat_map(|t| s(t, "mode").split('/').map(|s| s.trim().to_string()))
        .filter(|s| !s.is_empty())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let top = if embedded {
        String::new()
    } else {
        heading(
            "INTA / TOOL DIRECTORY",
            if saved { "Saved tools" } else { "Tool library" },
            if saved {
                "Your working toolkit, kept on this device."
            } else {
                "Find the right utility and go straight to its official source."
            },
        )
    };
    let sort_options = SORTS
        .iter()
        .map(|(key, label, _)| {
            format!(
                "<option value=\"{key}\"{}>{}</option>",
                if sort == *key { " selected" } else { "" },
                esc(label)
            )
        })
        .collect::<String>();
    let mut html=format!("<div class=\"library-workspace\" data-library-view=\"{view}\">{top}<div class=\"library-toolbar home-tool-finder\"><div class=\"library-searchbar\"><label class=\"input-wrap\">{}<span class=\"sr-only\">Search tools</span><input id=\"library-search\" type=\"search\" placeholder=\"Search tools, symptoms or tasks…\" value=\"{}\" autocomplete=\"off\"></label><div class=\"library-view-switch\" role=\"group\" aria-label=\"Tool layout\">{}</div></div><div class=\"library-options\"><div class=\"library-filter-primary\">{}<button class=\"library-more-filters\" type=\"button\" aria-controls=\"tool-filters\" aria-expanded=\"false\" data-library-filters>{}Filters <span data-library-filter-count hidden></span></button></div><div class=\"library-filters filters\" id=\"tool-filters\">{}{}{}{}</div></div><div class=\"library-categories\" role=\"group\" aria-label=\"Quick categories\">",icon("search"),esc(r.get("q")),[("list","List"),("gallery","Gallery")].iter().map(|(v,label)|format!("<button type=\"button\" data-library-view-toggle=\"{v}\" aria-pressed=\"{}\">{}<span>{label}</span></button>",view==*v,icon(if *v=="list"{"list"}else{"grid"}))).collect::<String>(),select("tool-sort","Sort by",&sort_options),icon("filter"),select("category-filter","Category",&named_options(CATEGORIES,"All categories",r.get("category"),category)),select("platform-filter","Platform",&options(&platforms,"All platforms",r.get("platform"))),select("tier-filter","Recommendation",&named_options(&["core","specialist","evaluate"],"All recommendations",r.get("tier"),tier)),select("mode-filter","Use",&options(&modes,"All types",r.get("mode"))));
    for cat in std::iter::once("").chain(CATEGORIES.iter().copied()) {
        let count = app
            .tools()
            .iter()
            .filter(|t| {
                (!saved || app.favorites.contains(s(t, "id")))
                    && (cat.is_empty() || s(t, "category") == cat)
            })
            .count();
        html.push_str(&format!("<button type=\"button\" data-library-category=\"{cat}\" aria-pressed=\"{}\">{}<span>{}</span><small>{count}</small></button>",r.get("category")==cat,icon(if cat.is_empty(){"layers"}else{category_icon(cat)}),esc(if cat.is_empty(){"All tools"}else{category(cat)})));
    }
    html.push_str(&format!("</div></div><div class=\"library-result-bar\"><span id=\"library-result-count\" aria-live=\"polite\">{}</span><a href=\"{}\" class=\"library-saved-link\">{}{} <span{}>{}</span></a><button type=\"button\" data-copy-library>{}<span>Share view</span></button><button type=\"button\" data-reset-filters>Clear filters</button></div><div id=\"active-tool-filters\" class=\"active-tool-filters\" aria-label=\"Active tool filters\">{}</div><p id=\"tool-sort-status\" class=\"tool-sort-status\" aria-live=\"polite\">{}</p><p id=\"library-storage-status\" class=\"tool-sort-status\"{}>Browser storage is unavailable. Saves and layout changes last for this visit. Export your work to keep it.</p><div class=\"library-column-head\" aria-hidden=\"true\"><span>Tool &amp; purpose</span><span>Platform &amp; evidence</span><span>Official access</span><span>Save</span></div><div class=\"library-results\" id=\"tool-results\" role=\"list\" aria-label=\"Matching tools\" data-saved-only=\"{saved}\">{}</div></div>",result_count(app),if saved{"#tools"}else{"#saved"},icon(if saved{"layers"}else{"bookmark"}),if saved{"All tools"}else{"Saved"},if saved{""}else{" data-library-saved-count"},if saved{app.tools().len()}else{app.favorites.len()},icon("link"),filter_chips(app),esc(&sort_summary(app)),if app.storage_available{" hidden"}else{""},results(app)));
    if filtered(app).is_empty() {
        html = html.replacen(
            "class=\"library-workspace\"",
            "class=\"library-workspace library-no-results\"",
            1,
        );
    }
    if sort == "catalog" {
        html = html.replacen(
            "id=\"tool-sort-status\"",
            "id=\"tool-sort-status\" hidden",
            1,
        );
    }
    html
}
pub fn result_count(app: &App) -> String {
    format!(
        "<strong>{}</strong> of {} tools",
        filtered(app).len(),
        if app.route.name == "saved" {
            app.favorites.len()
        } else {
            app.tools().len()
        }
    )
}
pub fn sort_description(sort: &str) -> &str {
    SORTS
        .iter()
        .find(|x| x.0 == sort)
        .map(|x| x.2)
        .unwrap_or("")
}
pub fn sort_summary(app: &App) -> String {
    let key = sort_key(app.route.get("sort"));
    let mut result = sort_description(key).to_string();
    if !["catalog", "name", "name-desc"].contains(&key) {
        let found = filtered(app);
        let measured = found
            .iter()
            .filter(|tool| metric(tool, key).is_some())
            .count();
        if measured != found.len() {
            result.push_str(&format!(
                " {measured} of {} matching tools have this data. Unknowns are last.",
                found.len()
            ));
        }
    }
    result
}
pub fn filter_chips(app: &App) -> String {
    [("q","Search","library-search"),("category","Category","category-filter"),("platform","Platform","platform-filter"),("tier","Recommendation","tier-filter"),("mode","Use","mode-filter")].iter().filter(|(key,_,_)|!app.route.get(key).is_empty()).map(|(key,label,field)|format!("<button type=\"button\" class=\"filter-chip\" data-clear-tool-filter=\"{field}\" aria-label=\"Clear {label}\">{label}: {} {}</button>",esc(match *key{"category"=>category(app.route.get(key)),"tier"=>tier(app.route.get(key)),_=>app.route.get(key)}),icon("close"))).collect()
}

pub fn home(app: &App) -> String {
    let mut html=format!("<div class=\"workspace-home\"><header class=\"page-heading home-heading\"><div><p class=\"page-label\">INTA / WORKSPACE</p><h1>Technician workspace</h1><p class=\"page-description\">Find a tool. Follow the procedure. Verify the result.</p></div><a class=\"button ghost\" href=\"#checklist\">{}Open checklist</a></header><nav class=\"home-actions\" aria-label=\"Common technician actions\"><a href=\"#tools\" class=\"button primary\">{}Browse all tools</a><a href=\"#command-center\" class=\"button ghost\">{}Open Command Center</a><a href=\"#updates\" class=\"button ghost\">{}Updates &amp; Maintenance</a><a href=\"#playbooks\" class=\"button ghost\">{}Job playbooks</a><a href=\"#services\" class=\"button ghost\">{}Service map</a></nav><section class=\"all-tools-section\" aria-labelledby=\"directory-heading\"><div class=\"section-title\"><div><h2 id=\"directory-heading\">Tool directory</h2><p>Every tool, with its platform, purpose and official acquisition route.</p></div><span>{} tools · {} playbooks</span></div>{}</section>",icon("checkCircle"),icon("grid"),icon("terminal"),icon("refresh"),icon("book"),icon("route"),app.tools().len(),app.jobs().len(),library(app,true));
    html.push_str(&bench::online_start(app));
    html.push_str("<section class=\"quick-start-section\" aria-labelledby=\"quick-start-heading\"><div class=\"section-title\"><div><h2 id=\"quick-start-heading\">Fix a problem</h2><p>Start with the first action and choose each tool by its role.</p></div><a href=\"#playbooks\">All playbooks →</a></div><div class=\"quick-start-list\" role=\"list\">");
    for item in a(&app.data["quickStart"]["items"]) {
        html.push_str(&format!("<article class=\"quick-start-row\" role=\"listitem\" data-quick-start=\"{}\" data-quick-job=\"{}\" data-quick-tool=\"{}\"><h3><a class=\"quick-scenario-link\" href=\"#job/{}/quick/{}\"><span class=\"scenario-emoji\" aria-hidden=\"true\">{}</span><span>{}</span></a></h3><p class=\"quick-first-step\">{}</p>{}</article>",esc(s(item,"id")),esc(s(item,"jobId")),esc(s(item,"toolId")),encode(s(item,"jobId")),encode(s(item,"id")),esc(s(item,"icon")),esc(s(item,"label")),formatted(s(item,"firstStep")),app.quick_choices(item)));
    }
    html.push_str("</div></section></div>");
    html
}

pub fn detail(app: &App, id: &str) -> String {
    let Some(tool) = app.tool(id) else {
        return empty(
            "Tool not found",
            "The catalog link may have changed.",
            "#tools",
            "Browse all tools",
        );
    };
    let access = &app.data["toolAccess"][id];
    let tool_title =
        app.tool_name(tool)
            .replacen("tool-name brand-link", "detail-brand-link brand-link", 1);
    let saved = app.favorites.contains(id);
    let save = format!("<button class=\"button ghost\" type=\"button\" data-save=\"{}\" aria-pressed=\"{saved}\" aria-label=\"{} {}\">{}<span data-save-label>{}</span></button>",esc(id),if saved{"Unsave"}else{"Save"},esc(s(tool,"name")),icon("bookmark"),if saved{"Saved to your toolkit"}else{"Save to your toolkit"});
    let access_note = if s(access, "note").is_empty() {
        String::new()
    } else {
        format!(
            "<p class=\"access-note\">{}</p>",
            formatted(s(access, "note"))
        )
    };
    let best_for = if s(tool, "bestFor").is_empty() {
        s(tool, "summary")
    } else {
        s(tool, "bestFor")
    };
    let use_mode = mode(s(tool, "mode"));
    let pricing = if s(tool, "pricing").is_empty() {
        "See the official source for current terms."
    } else {
        s(tool, "pricing")
    };
    let recommendation = match s(tool, "tier") {
        "evaluate" => "Validate it against your own workflow before relying on it.",
        "specialist" => "Best for a specific diagnostic or recovery need.",
        _ => "A practical starting point for routine service work.",
    };
    let mut html = format!(
        r##"<a class="back-link" href="#tools" data-back-results>{}Back to results</a><header class="detail-header"><div class="detail-tool-title"><div><div class="page-eyebrow">{}</div><h1>{tool_title}</h1></div></div><p class="summary">{}</p><div class="detail-meta">{}{}<span class="pill">{}</span></div><div class="detail-actions">{}{}{save}</div></header><div class="detail-layout"><div class="detail-main">{access_note}<div class="callout">{}<div><h2>When to reach for it</h2><p>{}</p></div></div><div class="tool-info-grid"><section class="info-block"><span class="small-label">Platforms</span><p>{}</p></section><section class="info-block"><span class="small-label">How you use it</span><p>{}</p></section><section class="info-block"><span class="small-label">Availability &amp; pricing</span><p>{}</p></section><section class="info-block"><span class="small-label">Recommendation</span><p>{} · {recommendation}</p></section></div>"##,
        icon("back"),
        esc(category(s(tool, "category"))),
        formatted(s(tool, "summary")),
        tier_pill(s(tool, "tier")),
        a(&tool["platforms"])
            .iter()
            .map(|v| format!("<span class=\"pill\">{}</span>", esc(&txt(v))))
            .collect::<String>(),
        esc(use_mode),
        app.access_actions(tool, "", false),
        external(
            s(tool, "docsUrl"),
            &format!("{}Documentation", icon("book")),
            "button ghost"
        ),
        icon("compass"),
        formatted(best_for),
        esc(&a(&tool["platforms"])
            .iter()
            .map(txt)
            .collect::<Vec<_>>()
            .join(" · ")),
        esc(if use_mode.is_empty() {
            "See official documentation"
        } else {
            use_mode
        }),
        formatted(pricing),
        esc(tier(s(tool, "tier")))
    );
    if !s(tool, "cautions").is_empty() {
        html.push_str(&format!(
            "<div class=\"callout warning\">{}<div><h2>Before you use it</h2><p>{}</p></div></div>",
            icon("info"),
            formatted(s(tool, "cautions"))
        ));
    }
    if !a(&tool["tags"]).is_empty() {
        html.push_str(&format!(
            "<div class=\"detail-tags\">{}</div>",
            a(&tool["tags"])
                .iter()
                .map(|x| format!("<span class=\"pill\">{}</span>", esc(&txt(x))))
                .collect::<String>()
        ));
    }
    html.push_str(&bench::online_detail(app, id));
    html.push_str("<section class=\"tool-evidence\" aria-labelledby=\"tool-evidence-heading\"><h2 class=\"block-heading\" id=\"tool-evidence-heading\">Tool reputation</h2><dl>");
    for (key, title) in [
        ("stars", "GitHub stars"),
        ("forks", "GitHub forks"),
        ("rating", "User rating"),
        ("popularity", "Weekly downloads"),
        ("users", "Weekly active users"),
    ] {
        html.push_str(&format!(
            "<div><dt>{title}</dt><dd>{}</dd></div>",
            evidence_signal(tool, key)
        ));
    }
    html.push_str(
        "</dl><p>Independent, dated observations. Unknown values remain unknown.</p></section>",
    );
    let picks: Vec<_> = a(&app.data["quickStart"]["items"])
        .iter()
        .filter(|item| {
            s(item, "toolId") == id
                || a(&item["choices"])
                    .iter()
                    .any(|choice| s(choice, "toolId") == id)
        })
        .collect();
    html.push_str(&guides::selection_notes(app, &picks, "Used in these jobs"));
    let mut seen_sources = BTreeSet::new();
    let references = Value::Array(
        [
            ("Official website", s(tool, "homepage")),
            ("Documentation and usage", s(tool, "docsUrl")),
            ("Source or upstream reference", s(tool, "sourceUrl")),
        ]
        .iter()
        .filter(|(_, url)| safe_url(url) && seen_sources.insert(*url))
        .map(|(title, url)| json!({"title":title,"url":url}))
        .collect(),
    );
    let reviewed = if s(tool, "reviewedAt").is_empty() {
        "Not recorded"
    } else {
        s(tool, "reviewedAt")
    };
    html.push_str(&format!("<section><h2 class=\"block-heading\">{}Go to the source</h2>{}<p class=\"sources-note\">Source review: {}. Official pages and acquisition links are checked separately. File versions and availability can change after review.</p></section>",icon("book"),source_list(&references),esc(reviewed)));
    if !s(tool, "evidenceStatus").is_empty() {
        html.push_str(&format!(
            "<p class=\"sources-note\">Evidence status: {}</p>",
            formatted(s(tool, "evidenceStatus"))
        ));
    }
    if !a(&tool["provenance"]).is_empty() {
        html.push_str("<section><h2 class=\"block-heading\">Why it is in the toolkit</h2><ul class=\"bullet-list\">");
        for x in a(&tool["provenance"]) {
            let v = txt(x);
            html.push_str(&format!(
                "<li>{}</li>",
                if safe_url(&v) {
                    external(&v, &esc(&v), "")
                } else {
                    formatted(&v)
                }
            ));
        }
        html.push_str("</ul></section>");
    }
    let related = app
        .jobs()
        .iter()
        .filter(|j| a(&j["tools"]).iter().any(|t| t.as_str() == Some(id)))
        .collect::<Vec<_>>();
    html.push_str(&format!("</div><aside class=\"detail-aside\"><section class=\"aside-card\"><h2>{}</h2><p>Compare diagnostic roles, supported systems, required access, and operating requirements before choosing a tool for the job.</p></section>",external(&format!("{REPO}/wiki/Tool-Selection"),"Tool selection","")));
    if !related.is_empty() {
        html.push_str("<section class=\"aside-card\"><h2>Use it in a playbook</h2><div class=\"aside-tools\">");
        for job in related {
            html.push_str(&format!(
                "<a href=\"#job/{}\"><span>{}</span>{}</a>",
                encode(s(job, "id")),
                esc(s(job, "title")),
                icon("arrow")
            ));
        }
        html.push_str("</div></section>");
    }
    html.push_str(&format!("<section class=\"aside-card\"><h2>Contribute a correction</h2><p>Bring the official source and explain the useful change.</p>{}</section></aside></div>",external(&format!("{REPO}/issues/new"),"Suggest an update","button text")));
    html
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> App {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data");
        let read = |name: &str| {
            serde_json::from_str::<Value>(&std::fs::read_to_string(root.join(name)).unwrap())
                .unwrap()
        };
        App::new(
            json!({"tools":read("tools.json"),"jobs":read("jobs.json"),"quickStart":read("quick-start.json"),"toolAccess":read("tool-access.json")["tools"],"onlineTools":read("online-tools.json")}),
        )
    }
    #[test]
    fn unknown_is_not_zero_and_filters_compose() {
        let app = App::new(
            json!({"tools":[{"id":"a","name":"Alpha","category":"windows","tier":"core","platforms":["Windows"],"mode":"portable","reputation":{"github":{"status":"observed","stars":0}}},{"id":"b","name":"Beta","category":"online","platforms":["Web browser"],"reputation":{"github":{"status":"unverified"}}}]}),
        );
        assert_eq!(metric(&app.tools()[0], "stars"), Some(0.));
        assert_eq!(metric(&app.tools()[1], "stars"), None);
        let mut app = app;
        app.route.params.insert("sort".into(), "stars".into());
        assert_eq!(s(filtered(&app)[0], "id"), "a");
        app.route
            .params
            .insert("q".into(), "portable Windows".into());
        assert_eq!(filtered(&app).len(), 1);
        app.route.params.insert("platform".into(), "Linux".into());
        assert!(filtered(&app).is_empty());
    }
    #[test]
    fn list_and_gallery_have_identical_result_ids() {
        let mut app =
            App::new(json!({"tools":[{"id":"a","name":"Alpha"},{"id":"b","name":"Beta"}]}));
        let ids = filtered(&app)
            .iter()
            .map(|x| s(x, "id").to_string())
            .collect::<Vec<_>>();
        app.route.params.insert("view".into(), "gallery".into());
        assert_eq!(
            ids,
            filtered(&app)
                .iter()
                .map(|x| s(x, "id").to_string())
                .collect::<Vec<_>>()
        );
    }
    #[test]
    fn complete_library_keeps_every_identity_and_acquisition() {
        let mut app = fixture();
        for view in ["list", "gallery"] {
            app.route.params.insert("view".into(), view.into());
            let html = library(&app, false);
            assert_eq!(html.matches("data-tool-card=").count(), app.tools().len());
            for tool in app.tools() {
                assert!(html.contains(&format!("data-tool-card=\"{}\"", s(tool, "id"))));
                let record = &app.data["toolAccess"][s(tool, "id")];
                assert!(
                    html.contains(&format!("href=\"{}\"", esc(s(record, "officialUrl")))),
                    "missing official {}",
                    s(tool, "id")
                );
                for action in
                    std::iter::once(&record["primary"]).chain(a(&record["alternates"]).iter())
                {
                    assert!(
                        html.contains(&format!("href=\"{}\"", esc(s(action, "url")))),
                        "missing acquisition {}",
                        s(tool, "id")
                    );
                }
            }
        }
    }
    #[test]
    fn scenario_primary_dedup_uses_exact_acquisition_url() {
        let app = App::new(
            json!({"tools":[{"id":"scanner","name":"Scanner"}],"toolAccess":{"scanner":{"primary":{"id":"main","url":"https://example.org/main.exe"},"alternates":[{"id":"case","url":"https://example.org/case.exe"}]}},"quickStart":{"reviewedAt":"2026-11-26"}}),
        );
        let item = json!({"toolId":"scanner","choices":[{"toolId":"scanner","actionId":"main","role":"Authored role"}]});
        assert_eq!(
            normalized_choices(&app, &item).len(),
            1,
            "a named primary is the same acquisition as the implicit primary"
        );
        let case = json!({"toolId":"scanner","toolAction":"case","choices":[{"toolId":"scanner","role":"Authored role"}]});
        let resolved = normalized_choices(&app, &case);
        assert_eq!(resolved.len(), 2);
        assert_eq!(s(&resolved[0], "role"), "Authored role");
        assert_eq!(s(&resolved[1], "actionId"), "case");
        assert_eq!(
            choice_url(&app, &resolved[1]),
            "https://example.org/case.exe"
        );
    }
    #[test]
    fn choices_are_complete_and_guide_only_cases_remain_empty() {
        let app = fixture();
        for item in a(&app.data["quickStart"]["items"]) {
            let resolved = normalized_choices(&app, item);
            for choice in a(&item["choices"]) {
                assert!(
                    resolved.contains(choice),
                    "authored choice removed in {}",
                    s(item, "id")
                );
            }
            if item["guideOnly"] == true {
                assert!(resolved.is_empty());
            } else if !s(item, "toolId").is_empty() {
                let original = json!({"toolId":s(item,"toolId"),"actionId":s(item,"toolAction")});
                assert!(resolved
                    .iter()
                    .any(|choice| s(choice, "toolId") == s(item, "toolId")
                        && choice_url(&app, choice) == choice_url(&app, &original)));
            }
        }
    }
    #[test]
    fn every_tool_detail_preserves_metadata_actions_and_selection_notes() {
        let app = fixture();
        for tool in app.tools() {
            let id = s(tool, "id");
            let html = detail(&app, id);
            for key in ["summary", "pricing", "bestFor", "cautions"] {
                assert!(
                    html.contains(&formatted(s(tool, key))),
                    "{} lost {}",
                    id,
                    key
                );
            }
            for section in [
                "Platforms",
                "How you use it",
                "Availability &amp; pricing",
                "Recommendation",
                "Tool reputation",
                "Go to the source",
                "Tool selection",
                "data-save-label",
            ] {
                assert!(html.contains(section), "{id} lost {section}");
            }
            for item in a(&app.data["quickStart"]["items"]).iter().filter(|item| {
                s(item, "toolId") == id
                    || a(&item["choices"])
                        .iter()
                        .any(|choice| s(choice, "toolId") == id)
            }) {
                assert!(
                    html.contains(&esc(s(item, "label"))),
                    "{id} lost selection context {}",
                    s(item, "id")
                );
            }
        }
    }
    #[test]
    fn measured_rating_ties_use_sample_size_and_leave_unknown_last() {
        let mut app = App::new(
            json!({"tools":[{"id":"unknown","name":"A unknown","reputation":{"sourceforge":{"status":"unavailable","rating":{"value":5,"reviews":1000}}}},{"id":"few","name":"B few","reputation":{"sourceforge":{"status":"observed","rating":{"value":4.5,"reviews":2}}}},{"id":"many","name":"C many","reputation":{"sourceforge":{"status":"observed","rating":{"value":4.5,"reviews":90}}}}]}),
        );
        app.route.params.insert("sort".into(), "rating".into());
        assert_eq!(
            filtered(&app)
                .iter()
                .map(|v| s(v, "id"))
                .collect::<Vec<_>>(),
            vec!["many", "few", "unknown"]
        );
        assert!(
            sort_summary(&app).contains("2 of 3 matching tools have this data. Unknowns are last.")
        );
        assert!(evidence_signal(&app.tools()[0], "rating").contains("Source unavailable"));
    }
    #[test]
    fn home_order_keeps_full_tools_and_all_authored_scenarios() {
        let app = fixture();
        let html = home(&app);
        assert!(
            html.find("id=\"directory-heading\"").unwrap()
                < html.find("id=\"online-start-heading\"").unwrap()
        );
        assert_eq!(html.matches("data-tool-card=").count(), app.tools().len());
        for item in a(&app.data["quickStart"]["items"]) {
            assert!(html.contains(&format!("data-quick-start=\"{}\"", s(item, "id"))));
        }
    }
}
