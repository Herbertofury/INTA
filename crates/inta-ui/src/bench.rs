//! Shared command, maintenance, online-service and news workspaces.
//! Rendering and filtering are Rust-owned; host events use the stable data attributes.
use crate::{common::*, App, Route};
use serde_json::Value;
use std::collections::BTreeSet;
use std::fmt::Write;

pub fn render(app: &App, route: &Route) -> String {
    match route.name.as_str() {
        "online" => online(app, route),
        "news" => news(app, route),
        "command-center" => commands(app, route),
        "updates" => updates(app, route),
        _ => empty(
            "Workspace unavailable",
            "Choose a workspace from the navigation.",
            "#home",
            "Open technician home",
        ),
    }
}
fn field(v: &Value, key: &str) -> String {
    formatted(s(v, key))
}
fn select(id: &str, label: &str, values: &[(&str, &str)], selected: &str) -> String {
    let mut out = format!("<label class=\"sr-only\" for=\"{id}\">{}</label><select class=\"filter-select\" id=\"{id}\">",esc(label));
    for (value, text) in values {
        let _ = write!(
            out,
            "<option value=\"{}\"{}>{}</option>",
            esc(value),
            if *value == selected { " selected" } else { "" },
            esc(text)
        );
    }
    out.push_str("</select>");
    out
}
fn changed_href(route: &Route, changes: &[(&str, &str)]) -> String {
    let mut next = route.clone();
    for (key, value) in changes {
        if value.is_empty() {
            next.params.remove(*key);
        } else {
            next.params.insert((*key).into(), (*value).into());
        }
    }
    next.href()
}
fn section_heading(eyebrow: &str, title: &str, body: &str, class: &str) -> String {
    format!("<div class=\"{class}\"><div><p class=\"page-label\">{}</p><h2>{}</h2><p>{}</p></div></div>",esc(eyebrow),esc(title),formatted(body))
}
fn find<'a>(values: &'a Value, id: &str) -> Option<&'a Value> {
    a(values).iter().find(|x| s(x, "id") == id)
}
fn risk_label(risk: &str) -> &str {
    match risk {
        "read-only" => "Read only",
        "long-running" => "Long-running",
        "downloads-file" => "Downloads file",
        "installs-software" => "Installs software",
        "changes-system" => "Changes system",
        _ => "Review first",
    }
}
const RISKS: &[(&str, &str)] = &[
    ("", "All impact levels"),
    ("read-only", "Read only"),
    ("long-running", "Long-running"),
    ("downloads-file", "Downloads file"),
    ("installs-software", "Installs software"),
    ("changes-system", "Changes system"),
];

pub fn command_matches(item: &Value, route: &Route, is_update: bool) -> bool {
    let selected = route.get("category");
    let category = s(item, "category");
    let title = s(item, "title").to_lowercase();
    let group = selected.is_empty()
        || selected == "all"
        || selected == category
        || if is_update {
            match selected {
                "updates" => [
                    "Windows Update",
                    "Microsoft Store",
                    "WinGet",
                    "Chocolatey",
                    "Scoop",
                    "UniGetUI",
                    "Ninite Pro",
                    "Linux",
                    "macOS Software Update",
                    "Update inventory",
                ]
                .contains(&category),
                "recovery" => {
                    [
                        "Installer recovery",
                        "Application inventory",
                        "Windows Update",
                        "Microsoft Store",
                    ]
                    .contains(&category)
                        || [
                            "repair",
                            "reinstall",
                            "recover",
                            "cache",
                            "failed",
                            "stuck",
                            "logs",
                        ]
                        .iter()
                        .any(|v| title.contains(v))
                }
                "uninstall" => title.contains("uninstall") || title.contains("remove"),
                _ => false,
            }
        } else {
            false
        };
    group
        && (route.get("shell").is_empty() || route.get("shell") == s(item, "shell"))
        && (route.get("risk").is_empty() || route.get("risk") == s(item, "risk"))
        && matches(
            &[
                s(item, "title"),
                s(item, "summary"),
                s(item, "code"),
                s(item, "safety"),
                category,
                s(item, "shell"),
                s(item, "sourceTitle"),
            ]
            .join(" "),
            route.get("q"),
        )
}
fn command_card(item: &Value, is_update: bool) -> String {
    let risk = if s(item, "risk").is_empty() {
        "changes-system"
    } else {
        s(item, "risk")
    };
    let (card, attribute, prefix, head, kicker, summary, bar, copy, code, footer) = if is_update {
        (
            "update-command-card",
            "data-update-command-card",
            "update",
            "update-command-heading",
            "update-command-kicker",
            "update-command-summary",
            "update-code-toolbar",
            "data-copy-update-command",
            "update-code",
            "update-command-footer",
        )
    } else {
        (
            "command-card",
            "data-command-item",
            "command",
            "command-card-header",
            "command-card-kicker",
            "command-card-summary",
            "command-code-toolbar",
            "data-copy-center-command",
            "command-center-code",
            "command-card-footer",
        )
    };
    format!(
        r#"<article class="{card}" {attribute}="{id}" data-{prefix}-category-value="{category}" data-{prefix}-shell-value="{shell}" data-{prefix}-risk-value="{risk}"><header class="{head}"><div><p class="{kicker}">{category} · {shell} · {privilege}</p><h3>{title}</h3></div><span class="{prefix}-risk {prefix}-risk-{risk}">{risk_text}</span></header><p class="{summary}">{description}</p><div class="{bar}"><span>{shell}</span><button type="button" class="copy-command" {copy}="{id}">{copy_icon}Copy command</button></div><pre class="{code}"><code>{command}</code></pre><p class="command-safety">{info_icon}<span>{safety}</span></p><footer class="{footer}">{source}</footer></article>"#,
        id = esc(s(item, "id")),
        category = esc(s(item, "category")),
        shell = esc(s(item, "shell")),
        risk = esc(risk),
        privilege = esc(s(item, "privilege")),
        title = esc(s(item, "title")),
        risk_text = risk_label(risk),
        description = field(item, "summary"),
        copy_icon = icon("copy"),
        command = esc(s(item, "code")),
        info_icon = icon("info"),
        safety = field(item, "safety"),
        source = external(
            s(item, "sourceUrl"),
            &format!(
                "Official reference: {} {}",
                esc(s(item, "sourceTitle")),
                icon("external")
            ),
            ""
        )
    )
}
fn command_library(data: &Value, route: &Route, is_update: bool) -> String {
    let values = a(&data["commands"]);
    let filtered: Vec<_> = values
        .iter()
        .filter(|x| command_matches(x, route, is_update))
        .collect();
    let (
        section,
        head,
        filters,
        input,
        shell_id,
        risk_id,
        tabs,
        category_attr,
        count_id,
        result_class,
        result_id,
    ) = if is_update {
        (
            "updates-library",
            "updates-section-heading",
            "updates-filters",
            "updates-search",
            "updates-shell",
            "updates-risk",
            "updates-category-tabs",
            "data-update-category",
            "updates-count",
            "updates-command-grid",
            "updates-command-grid",
        )
    } else {
        (
            "command-center-library",
            "section-heading-row",
            "command-center-filters",
            "command-center-search",
            "command-center-shell",
            "command-center-risk",
            "command-category-tabs",
            "data-command-category",
            "command-center-count",
            "command-center-list",
            "command-center-list",
        )
    };
    let mut out=format!("<section class=\"{section}\">{}<div class=\"{filters}\"><label class=\"input-wrap\">{}<span class=\"sr-only\">Search {}commands</span><input id=\"{input}\" type=\"search\" placeholder=\"Search a task, command, error, or package manager…\" value=\"{}\"></label>",section_heading(if is_update{"Repair & update library"}else{"Command library"},if is_update{"Find the exact action"}else{"Find a command"},"Filter by task, shell and impact. Copying a command never executes it.",head),icon("search"),if is_update{"maintenance "}else{""},esc(route.get("q")));
    let mut shells: Vec<_> = values
        .iter()
        .map(|x| s(x, "shell"))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    shells.sort();
    let mut shell_opts = vec![("", "All shells")];
    shell_opts.extend(shells.iter().map(|v| (*v, *v)));
    out.push_str(&select(shell_id, "Shell", &shell_opts, route.get("shell")));
    out.push_str(&select(risk_id, "Impact", RISKS, route.get("risk")));
    let _ = write!(
        out,
        "</div><nav class=\"{tabs}\" aria-label=\"Command categories\">"
    );
    let mut categories: Vec<(&str, &str)> = vec![("all", "All commands")];
    if is_update {
        categories.extend([
            ("updates", "Software updates"),
            ("recovery", "Repairs & recovery"),
            ("uninstall", "Uninstallers"),
        ]);
        for c in values
            .iter()
            .map(|x| s(x, "category"))
            .collect::<BTreeSet<_>>()
        {
            categories.push((c, c));
        }
    } else {
        categories.extend(
            a(&data["categories"])
                .iter()
                .map(|c| (s(c, "id"), s(c, "label"))),
        );
    }
    for (id, label) in categories {
        let active =
            route.get("category") == id || (id == "all" && route.get("category").is_empty());
        let _=write!(out,"<button type=\"button\" class=\"{}\" {category_attr}=\"{}\" aria-pressed=\"{active}\">{}</button>",if active{"active"}else{""},esc(id),esc(label));
    }
    let _=write!(out,"</nav><p class=\"{}\" id=\"{count_id}\" aria-live=\"polite\">{} of {} {}</p><div class=\"{result_class}\" id=\"{result_id}\">",if is_update{"updates-result-count"}else{"command-center-result"},filtered.len(),values.len(),if is_update{"maintenance actions"}else{"commands"});
    for item in filtered {
        out.push_str(&command_card(item, is_update));
    }
    if !values.is_empty()
        && !values
            .iter()
            .any(|item| command_matches(item, route, is_update))
    {
        out.push_str(&empty(
            "No commands match",
            "Broaden the search or clear shell and impact filters.",
            &format!("#{}", route.name),
            "Show all commands",
        ));
    }
    out.push_str("</div></section>");
    out
}
fn workflows(data: &Value, is_update: bool) -> String {
    let (class, heading_class, grid, card, kicker, sequence, copy, route) = if is_update {
        (
            "updates-workflows",
            "updates-section-heading",
            "updates-workflow-grid",
            "updates-workflow-card",
            "update-command-kicker",
            "updates-workflow-sequence",
            "data-copy-update-workflow",
            "updates",
        )
    } else {
        (
            "command-center-workflows",
            "section-heading-row",
            "command-workflow-grid",
            "command-workflow-card",
            "command-card-kicker",
            "command-workflow-sequence",
            "data-copy-center-workflow",
            "command-center",
        )
    };
    let mut out = format!(
        "<section class=\"{class}\">{}<div class=\"{grid}\">",
        section_heading(
            "Guided work",
            if is_update {
                "Maintenance workflows"
            } else {
                "Ready-made workflows"
            },
            "Follow the ordered steps, keep the safety notes, and inspect the result.",
            heading_class
        )
    );
    for flow in a(&data["workflows"]) {
        let _ = write!(
            out,
            "<article class=\"{card}\"><p class=\"{kicker}\">{}</p><h3>{}</h3><p>{}</p><ol>",
            esc(if s(flow, "category").is_empty() {
                s(flow, "id")
            } else {
                s(flow, "category")
            }),
            esc(s(flow, "title")),
            field(flow, "description")
        );
        for id in a(&flow["commandIds"]) {
            if let Some(item) = find(&data["commands"], id.as_str().unwrap_or("")) {
                let _ = write!(
                    out,
                    "<li><a href=\"#{route}?category={}&amp;q={}\">{}</a></li>",
                    encode(s(item, "category")),
                    encode(s(item, "title")),
                    esc(s(item, "title"))
                );
            }
        }
        let _=write!(out,"</ol><p class=\"{sequence}\">{}</p><p class=\"command-safety\">{}<span>{}</span></p><button type=\"button\" class=\"secondary-button\" {copy}=\"{}\">{}Copy workflow commands</button></article>",field(flow,"sequence"),icon("warning"),field(flow,"warning"),esc(s(flow,"id")),icon("copy"));
    }
    out.push_str("</div></section>");
    out
}
fn commands(app: &App, route: &Route) -> String {
    let data = &app.data["commandCenter"];
    let mut out=heading("Technician workspace","Command Center","Find the exact command, understand its impact, and keep the official reference beside the work.");
    out.push_str(&command_library(data, route, false));
    out.push_str(&workflows(data, false));
    let _ = write!(
        out,
        "<section class=\"command-center-toolkit\">{}<div class=\"command-toolkit-grid\">",
        section_heading(
            "Technician toolkits",
            "Maintained tools for the bench",
            "Verify the publisher, release and scope before installing or running a toolkit.",
            "section-heading-row"
        )
    );
    for kit in a(&data["toolkits"]) {
        let _=write!(out,"<article class=\"command-toolkit-card\"><div class=\"command-toolkit-top\"><span>{}</span><span>{}</span></div><h3>{}</h3><p>{}</p><p class=\"command-safety\">{}<span>{}</span></p><div class=\"command-toolkit-actions\">{}",esc(&s(kit,"kind").replace('-'," ")),esc(s(kit,"platform")),external(s(kit,"url"),&esc(s(kit,"name")),""),field(kit,"summary"),icon("shield"),field(kit,"trust"),external(if s(kit,"repository").is_empty(){s(kit,"url")}else{s(kit,"repository")},&format!("Project / documentation {}",icon("external")),""));
        if let Some(item) = find(&data["commands"], s(kit, "installCommandId")) {
            let _=write!(out,"<button type=\"button\" class=\"copy-command\" data-copy-center-command=\"{}\">{}Copy setup / launch</button>",esc(s(item,"id")),icon("copy"));
        }
        out.push_str("</div></article>");
    }
    out.push_str("</div></section><section class=\"command-center-maintenance\"><p class=\"page-label\">Separate update channels</p><h2>Application and operating system updates</h2><div class=\"command-maintenance-grid\">");
    for (title,description) in [("WinGet","Preview with `winget upgrade`, then choose the identified applications to update."),("Chocolatey","Use `choco outdated` and `choco upgrade all --noop` to preview changes in its own package database."),("Scoop","Updates applications managed by its configured buckets."),("Windows Update / Store","Operating system servicing and Store applications have separate update paths. A clean WinGet report does not prove Windows is patched.")]{let _=write!(out,"<div><h3>{}</h3><p>{}</p></div>",formatted(title),formatted(description));}
    out.push_str("</div></section>");
    out
}

fn agent_panel() -> String {
    format!(
        r#"<section class="updates-agent-panel" aria-labelledby="agent-panel-title"><div class="updates-section-heading"><div><p class="page-label">This computer</p><h2 id="agent-panel-title">Local maintenance agent</h2><p>Connect to inspect this computer. Inventory and previews are read-only; changes require one confirmation.</p></div><div class="agent-status-chip"><span class="agent-status-dot"></span><span id="agent-connection-status">Not connected</span></div></div><div class="agent-connect-actions"><button type="button" class="button primary" data-agent-connect>{plug}Connect this computer</button><button type="button" class="button ghost" data-agent-disconnect hidden>{close}Disconnect</button><button type="button" class="button ghost" data-agent-refresh disabled>{refresh}Refresh inventory &amp; progress</button></div><p class="agent-capabilities" id="agent-capabilities">Start the agent on this same computer, then connect. {download} This connection is local to the device; it does not provide phone-to-desktop account relay.</p><div class="agent-operation-controls"><label>Package manager<select id="agent-manager" disabled><option value="">Connect to detect managers</option></select></label><label>Exact package ID<input id="agent-package-id" type="text" placeholder="Package ID or Windows KB number"></label><label>Action<select id="agent-action"><option value="update">Update one</option><option value="repair">Repair / reinstall</option><option value="uninstall">Uninstall</option><option value="rollback">Rollback (Snap / Windows KB)</option></select></label><div class="agent-action-buttons"><button type="button" class="button primary" data-agent-run>{arrow}Run action</button><button type="button" class="button ghost" data-agent-update-all>{refresh}Update all in manager</button></div></div><div class="agent-live-data"><div class="agent-data-heading"><h3>Inventory &amp; update previews</h3><span>On request only</span></div><div id="agent-inventory"><p class="prose">Connect to load this computer’s inventory and package-manager previews.</p></div></div><div class="agent-live-data"><div class="agent-data-heading"><h3>Recent operations</h3><span>Saved on this computer</span></div><div id="agent-operations"><p class="prose">No local operation history loaded.</p></div></div></section>"#,
        plug = icon("usb"),
        close = icon("close"),
        refresh = icon("refresh"),
        arrow = icon("arrow"),
        download = external(
            &format!("{REPO}/tree/main/agent"),
            "Agent setup and build instructions",
            ""
        )
    )
}
fn updates(app: &App, route: &Route) -> String {
    let data = &app.data["updatesCenter"];
    let mut out = heading(
        "Maintenance workspace",
        "Updates & Maintenance",
        "Audit software, preview updates, repair failed installations, and verify the outcome.",
    );
    out.push_str(&agent_panel());
    out.push_str(&command_library(data, route, true));
    let _=write!(out,"<section class=\"updates-channel-section\">{}<div class=\"updates-channel-grid\">",section_heading("Update channels","Know what updates what","Audit each manager separately. Keep application patches, operating system servicing, drivers and firmware distinct.","updates-section-heading"));
    for channel in a(&data["channels"]) {
        let _=write!(out,"<article class=\"updates-channel-card\"><div class=\"updates-channel-top\"><span class=\"updates-channel-icon\">{}</span><span class=\"updates-channel-manager\">{}</span></div><h3>{}</h3><p>{}</p><p class=\"updates-channel-note\">{}</p><div class=\"updates-channel-actions\">",icon(s(channel,"icon")),esc(s(channel,"manager")),formatted(s(channel,"name")),field(channel,"scope"),field(channel,"note"));
        for (key, label, picture) in [
            ("auditCommand", "Copy audit", "search"),
            ("updateCommand", "Copy update", "download"),
        ] {
            if let Some(item) = find(&data["commands"], s(channel, key)) {
                let _=write!(out,"<button type=\"button\" class=\"copy-command\" data-copy-update-command=\"{}\">{}{label}</button>",esc(s(item,"id")),icon(picture));
            }
        }
        out.push_str("</div></article>");
    }
    out.push_str("</div></section>");
    out.push_str(&workflows(data, true));
    let _=write!(out,"<section class=\"updates-platforms\">{}<div class=\"updates-platform-grid\">",section_heading("Fleet patch management","Professional endpoint platforms","Managed fleets need enrolled devices, scoped authorization, deployment approvals and evidence from the actual endpoints.","updates-section-heading"));
    for platform in a(&data["platforms"]) {
        let _=write!(out,"<article class=\"updates-platform-card\"><p class=\"update-command-kicker\">{} · {}</p><h3>{}</h3><p>{}</p><p class=\"command-safety\">{}<span>{}</span></p>{}</article>",esc(&s(platform,"kind").replace('-'," ")),esc(s(platform,"platform")),external(s(platform,"url"),&esc(s(platform,"name")),""),field(platform,"summary"),icon("shield"),field(platform,"trust"),external(s(platform,"docsUrl"),&format!("Official documentation {}",icon("external")),""));
    }
    out.push_str("</div></section><section class=\"updates-recovery-principles\"><p class=\"page-label\">Recovery safety</p><h2>When an update goes wrong</h2><div class=\"updates-principle-grid\">");
    for (index, principle) in a(&data["principles"]).iter().enumerate() {
        let _ = write!(
            out,
            "<div><span>{:02}</span><p>{}</p></div>",
            index + 1,
            formatted(principle.as_str().unwrap_or(""))
        );
    }
    out.push_str("</div></section><div class=\"updates-install-note\"><button class=\"button ghost\" data-install-pwa hidden>Install INTA app</button><p>On iPhone or iPad, use Share → Add to Home Screen. On Android, use the browser’s Install app or Add to Home Screen action.</p></div>");
    out
}

fn online_entry<'a>(app: &'a App, id: &str) -> Option<&'a Value> {
    a(&app.data["onlineTools"]["tools"])
        .iter()
        .find(|x| s(x, "toolId") == id)
}
fn online_group<'a>(app: &'a App, id: &'a str) -> &'a str {
    find(&app.data["onlineTools"]["groups"], id)
        .map(|v| s(v, "label"))
        .unwrap_or(id)
}
fn online_task<'a>(app: &'a App, route: &Route) -> Option<&'a Value> {
    find(&app.data["onlineTools"]["workflows"], route.get("task"))
        .or_else(|| a(&app.data["onlineTools"]["workflows"]).first())
}
pub fn online_start(app: &App) -> String {
    let data = &app.data["onlineTools"];
    if a(&data["tools"]).is_empty() {
        return String::new();
    }
    let mut out=format!("<section class=\"online-start\" aria-labelledby=\"online-start-heading\"><div class=\"section-title\"><div><h2 id=\"online-start-heading\">Online tools</h2><p>Check a file, investigate a link, or diagnose a website.</p></div><a class=\"button ghost\" href=\"#online\">Browse {} online tools {}</a></div><div class=\"online-shortcuts\">",a(&data["tools"]).len(),icon("arrow"));
    for id in a(&data["featured"]) {
        if let Some(tool) = app.tool(id.as_str().unwrap_or("")) {
            let role = online_entry(app, s(tool, "id"))
                .map(|x| s(x, "shortRole"))
                .unwrap_or_else(|| s(tool, "summary"));
            let _ = write!(
                out,
                "<div>{}<span>{}</span></div>",
                app.tool_name(tool),
                esc(role)
            );
        }
    }
    out.push_str("</div><nav class=\"online-task-shortcuts\" aria-label=\"Online tool tasks\">");
    for flow in a(&data["workflows"]) {
        let _ = write!(
            out,
            "<a href=\"#online?task={}\">{}</a>",
            encode(s(flow, "id")),
            esc(s(flow, "title"))
        );
    }
    out.push_str("</nav></section>");
    out
}
pub fn online_detail(app: &App, id: &str) -> String {
    let Some(entry) = online_entry(app, id) else {
        return String::new();
    };
    format!("<section class=\"document-section online-detail\"><h2>Use this online service</h2><p><strong>First action:</strong> {}</p><p><strong>Expected result:</strong> {}</p><p><strong>Account:</strong> {}</p><p><strong>Submission behavior:</strong> {}</p><p><a href=\"#online?group={}\">Compare tools for this task</a></p>{}</section>",field(entry,"firstAction"),field(entry,"expectedResult"),field(entry,"account"),field(entry,"sharing"),encode(s(entry,"group")),source_list(&entry["sources"]))
}
fn online_workflow(app: &App, workflow: &Value) -> String {
    let mut out=format!("<section class=\"online-workflow\" aria-labelledby=\"online-workflow-heading\" data-online-workflow=\"{}\"><div><h2 id=\"online-workflow-heading\">{}</h2><p>{}</p><ol>",esc(s(workflow,"id")),esc(s(workflow,"title")),field(workflow,"summary"));
    for step in a(&workflow["steps"]) {
        let _ = write!(out, "<li>{}</li>", formatted(step.as_str().unwrap_or("")));
    }
    let _=write!(out,"</ol><a href=\"#job/{}\">Complete job guide {}</a></div><div class=\"online-workflow-kit\" aria-label=\"Tools for this task\">",encode(s(workflow,"jobId")),icon("book"));
    for id in a(&workflow["toolIds"]) {
        if let Some(tool) = app.tool(id.as_str().unwrap_or("")) {
            let role = online_entry(app, s(tool, "id"))
                .map(|x| s(x, "shortRole"))
                .unwrap_or_else(|| s(tool, "bestFor"));
            let _=write!(out,"<div>{}<p>{}</p>{}<a class=\"online-notes-link\" href=\"#online?tool={}\" data-online-inspect=\"{}\">Access &amp; submission behavior {}</a></div>",app.tool_name(tool),formatted(role),app.access_actions(tool,"",true),encode(s(tool,"id")),esc(s(tool,"id")),icon("arrow"));
        }
    }
    out.push_str("</div></section>");
    out
}
fn online_row(app: &App, entry: &Value) -> String {
    let Some(tool) = app.tool(s(entry, "toolId")) else {
        return String::new();
    };
    let id = s(tool, "id");
    let mut out = format!(
        r##"<article class="online-tool-row" id="online-service-{id}" role="listitem" data-online-tool="{id}" aria-labelledby="online-heading-{id}"><div class="online-tool-identity"><span class="row-meta">{group}</span><h3 id="online-heading-{id}" tabindex="-1">{name}</h3><p>{summary}</p><div class="online-service-actions">{access}<a href="#tool/{encoded}">Full reference</a><a href="#online?tool={encoded}" aria-label="Link to {title} in the online directory">Service link {link}</a>{save}</div></div><div class="online-tool-guidance"><p><strong>Start here</strong> {first}</p><p><strong>Read the result</strong> {expected}</p><dl class="online-access-notes"><div><dt>Access</dt><dd>{pricing} {account}</dd></div><div><dt>Submission behavior</dt><dd>{sharing}</dd></div></dl><details class="online-evidence"><summary>Official evidence · reviewed {reviewed}</summary><ul>"##,
        id = esc(id),
        group = esc(online_group(app, s(entry, "group"))),
        name = app.tool_name(tool),
        summary = field(tool, "summary"),
        access = app.access_actions(tool, "", true),
        encoded = encode(id),
        title = esc(s(tool, "name")),
        link = icon("link"),
        save = app.save_button(tool),
        first = field(entry, "firstAction"),
        expected = field(entry, "expectedResult"),
        pricing = field(tool, "pricing"),
        account = field(entry, "account"),
        sharing = field(entry, "sharing"),
        reviewed = esc(s(entry, "reviewedAt"))
    );
    for source in a(&entry["sources"]) {
        let _ = write!(
            out,
            "<li>{}<span>{}</span></li>",
            external(s(source, "url"), &esc(s(source, "title")), ""),
            field(source, "evidence")
        );
    }
    out.push_str("</ul></details></div></article>");
    out
}
pub fn online_matches(app: &App, entry: &Value, route: &Route) -> bool {
    let id = s(entry, "toolId");
    let Some(tool) = app.tool(id) else {
        return false;
    };
    let task = online_task(app, route);
    (route.get("group").is_empty() || route.get("group") == s(entry, "group"))
        && (route.get("saved") != "1" || app.favorites.contains(id))
        && (route.get("scope") != "task"
            || task.is_some_and(|t| a(&t["toolIds"]).iter().any(|x| x.as_str() == Some(id))))
        && matches(
            &format!(
                "{} {} {}",
                tool,
                entry,
                online_group(app, s(entry, "group"))
            ),
            route.get("q"),
        )
}
fn online(app: &App, route: &Route) -> String {
    let data = &app.data["onlineTools"];
    let all: Vec<_> = a(&data["tools"])
        .iter()
        .filter(|e| app.tool(s(e, "toolId")).is_some())
        .collect();
    let found: Vec<_> = all
        .iter()
        .copied()
        .filter(|e| online_matches(app, e, route))
        .collect();
    let task = online_task(app, route);
    let task_id = task.map(|t| s(t, "id")).unwrap_or("");
    let mut out=heading("Web toolkit","Online tools",&format!("{} source-reviewed services with direct access, first actions, and submission guidance.",all.len()));
    out.push_str("<nav class=\"online-task-nav\" aria-label=\"Choose an online task\">");
    for flow in a(&data["workflows"]) {
        let selected = s(flow, "id") == task_id;
        let _ = write!(
            out,
            "<a class=\"{}\" data-online-task=\"{}\" href=\"{}\"{}>{}</a>",
            if selected { "selected" } else { "" },
            esc(s(flow, "id")),
            esc(&changed_href(
                route,
                &[("task", s(flow, "id")), ("tool", "")]
            )),
            if selected {
                " aria-current=\"true\""
            } else {
                ""
            },
            esc(s(flow, "title"))
        );
    }
    out.push_str("</nav>");
    if let Some(task) = task {
        out.push_str(&online_workflow(app, task));
    }
    let _=write!(out,"<section class=\"online-directory\" aria-labelledby=\"online-directory-heading\"><div class=\"section-title\"><div><h2 id=\"online-directory-heading\">All online services</h2><p>Search names, tasks, access requirements, and capabilities.</p></div>{}</div><div class=\"filters online-filters\"><label class=\"input-wrap\">{}<span class=\"sr-only\">Search online tools</span><input id=\"online-search\" type=\"search\" placeholder=\"Try hash, phishing, DMARC, TLS, CVE…\" value=\"{}\"></label>",external(&format!("{REPO}/wiki/Online-Tools"),"Read the Wiki",""),icon("search"),esc(route.get("q")));
    let mut groups = vec![("", "All groups")];
    groups.extend(
        a(&data["groups"])
            .iter()
            .map(|g| (s(g, "id"), s(g, "label"))),
    );
    out.push_str(&select(
        "online-group",
        "Online tool category",
        &groups,
        route.get("group"),
    ));
    let _=write!(out,"<button type=\"button\" data-reset-online>Clear filters</button></div><div class=\"online-scope-filters\" role=\"group\" aria-label=\"Narrow the online directory\"><label><input type=\"checkbox\" id=\"online-task-only\"{}> Tools for this task</label><label><input type=\"checkbox\" id=\"online-saved-only\"{}> Saved only</label><span>Combine these with search and category filters.</span></div><nav class=\"online-group-nav\" aria-label=\"Filter online tools by task\"><button type=\"button\" data-online-group=\"\" aria-pressed=\"{}\">All groups <span>{}</span></button>",if route.get("scope")=="task"{" checked"}else{""},if route.get("saved")=="1"{" checked"}else{""},route.get("group").is_empty(),all.len());
    for group in a(&data["groups"]) {
        let _=write!(out,"<button type=\"button\" data-online-group=\"{}\" aria-pressed=\"{}\">{} <span>{}</span></button>",esc(s(group,"id")),route.get("group")==s(group,"id"),esc(s(group,"label")),all.iter().filter(|e|s(e,"group")==s(group,"id")).count());
    }
    let _=write!(out,"</nav><div class=\"filter-result\"><span id=\"online-result-count\" aria-live=\"polite\">{} of {} online services{}</span><span>Sources reviewed {}</span></div><div id=\"online-results\" class=\"online-tool-list\" role=\"list\">",found.len(),all.len(),if route.get("scope")=="task"{format!(" · Task: {}",esc(task.map(|t|s(t,"title")).unwrap_or("")))}else{String::new()},esc(s(data,"reviewedAt")));
    for entry in &found {
        out.push_str(&online_row(app, entry));
    }
    if found.is_empty() {
        out.push_str("<div class=\"empty-state\"><h3>No matching online tools</h3><p>Try a broader task or clear filters to see every service.</p><button class=\"button\" data-reset-online>Show all online tools</button></div>");
    }
    out.push_str("</div></section>");
    out
}

const NEWS_GROUPS: &[(&str, &str, &str)] = &[
    (
        "advisory",
        "Official advisories",
        "Government alerts and exposure triage",
    ),
    (
        "compliance",
        "Standards & compliance",
        "Frameworks, assessment and program references",
    ),
    (
        "vendor",
        "Vendor security",
        "Product updates, mitigations and release guidance",
    ),
    (
        "reporting",
        "Independent reporting",
        "Original reporting and technical analysis; verify before acting",
    ),
    (
        "community",
        "Community & social",
        "Professional discussion and public accounts; not official alerts",
    ),
    (
        "status",
        "Service health",
        "Publisher status and service incident pages",
    ),
];
fn news_group(id: &str) -> &str {
    NEWS_GROUPS
        .iter()
        .find(|v| v.0 == id)
        .map(|v| v.1)
        .unwrap_or(id)
}
fn time_label(value: &str) -> String {
    format!(
        "<time datetime=\"{}\">{}{}{}</time>",
        esc(value),
        esc(value.get(..16).unwrap_or(value)).replace('T', " "),
        if value.contains('T') { " " } else { "" },
        if value.contains('T') { "UTC" } else { "" }
    )
}
fn provenance(row: Option<&Value>) -> &str {
    match row {
        Some(row)
            if s(row, "origin") == "editorial"
                || (row["origin"].is_null() && s(row, "id") == "windows-health") =>
        {
            "editorial"
        }
        Some(row) if s(row, "status") == "unavailable" => "retained",
        _ => "feed",
    }
}
fn source_status(row: Option<&Value>, origin: &str) -> String {
    if origin == "editorial" {
        return "Editorial selection".into();
    }
    format!(
        "{} · {}",
        if origin == "retained" {
            "Retained · feed unavailable"
        } else {
            "Publisher feed"
        },
        row.map(|x| s(x, "lastSuccessfulCheckAt"))
            .filter(|x| !x.is_empty())
            .map(|x| format!("Last successful check {}", time_label(x)))
            .unwrap_or_else(|| "Last successful check unknown".into())
    )
}
fn digest(app: &App, route: &Route) -> String {
    let data = &app.data["newsDigest"];
    if !data.is_object() {
        return String::new();
    }
    let catalog = &app.data["news"];
    let sources = &catalog["sources"];
    let mut items: Vec<_> = a(&data["items"])
        .iter()
        .filter(|i| find(sources, s(i, "sourceId")).is_some())
        .collect();
    items.sort_by(|a, b| s(b, "date").cmp(s(a, "date")));
    let states = &data["sources"];
    let feeds: Vec<_> = a(states)
        .iter()
        .filter(|r| provenance(Some(r)) != "editorial")
        .collect();
    let active = feeds.iter().filter(|r| s(r, "status") == "ok").count();
    let retained = items
        .iter()
        .filter(|i| provenance(find(states, s(i, "sourceId"))) == "retained")
        .count();
    let editorial = items
        .iter()
        .filter(|i| provenance(find(states, s(i, "sourceId"))) == "editorial")
        .count();
    let unavailable = feeds
        .iter()
        .filter(|r| s(r, "status") == "unavailable")
        .filter_map(|r| find(sources, s(r, "id")))
        .map(|r| s(r, "name"))
        .collect::<Vec<_>>();
    let found: Vec<_> = items
        .iter()
        .copied()
        .filter(|i| {
            let source = find(sources, s(i, "sourceId")).unwrap();
            (route.get("digest-category").is_empty()
                || route.get("digest-category") == s(source, "category"))
                && (route.get("media") != "1" || !a(&i["media"]).is_empty())
        })
        .collect();
    let mut out=format!("<section id=\"news-digest\" aria-labelledby=\"news-digest-title\"><h2 id=\"news-digest-title\">Publisher digest</h2><p class=\"news-muted\">Snapshot {} · {} of {} publisher feeds available · {} retained · {} editorial.{} Media loads only when you ask. {}</p><div class=\"news-controls\"><label>Digest category<select id=\"news-digest-category\"><option value=\"\">All categories</option>",time_label(s(data,"generatedAt")),active,feeds.len(),retained,editorial,if unavailable.is_empty(){String::new()}else{format!(" Unavailable at refresh: {}.",esc(&unavailable.join(", ")))},external("https://herbertofury.github.io/INTA/news-digest.json","Shared feed JSON",""));
    let categories: BTreeSet<_> = items
        .iter()
        .filter_map(|i| find(sources, s(i, "sourceId")))
        .map(|r| s(r, "category"))
        .collect();
    for cat in categories {
        let _ = write!(
            out,
            "<option value=\"{}\"{}>{}</option>",
            esc(cat),
            if route.get("digest-category") == cat {
                " selected"
            } else {
                ""
            },
            esc(news_group(cat))
        );
    }
    let _=write!(out,"</select></label><label class=\"news-saved\"><input id=\"news-media-only\" type=\"checkbox\"{}> With media only</label></div><p id=\"news-digest-count\" aria-live=\"polite\">{} of {} items</p><div class=\"news-grid\">",if route.get("media")=="1"{" checked"}else{""},found.len(),items.len());
    for item in found {
        let source = find(sources, s(item, "sourceId")).unwrap();
        let row = find(states, s(item, "sourceId"));
        let origin = provenance(row);
        let _=write!(out,"<article class=\"news-brief news-story\" data-freshness=\"{origin}\" data-digest-id=\"{}\" data-digest-category=\"{}\" data-has-media=\"{}\"><span>{} · {} · {}</span><h3>{}</h3><p class=\"news-source-freshness\">{}</p>",esc(s(item,"id")),esc(s(source,"category")),!a(&item["media"]).is_empty(),external(s(source,"url"),&esc(s(source,"name")),""),esc(news_group(s(source,"category"))),time_label(s(item,"date")),external(s(item,"url"),&esc(s(item,"title")),""),source_status(row,origin));
        for (index, media) in a(&item["media"]).iter().enumerate() {
            let label = match s(media, "kind") {
                "image" => "Show image",
                "audio" => "Load audio",
                _ => "Load video",
            };
            let _=write!(out,"<div class=\"news-media\"><button type=\"button\" class=\"button\" data-media-index=\"{index}\" aria-label=\"{label} for {}\">{label}</button></div>",esc(s(item,"title")));
        }
        out.push_str("</article>");
    }
    out.push_str("</div></section>");
    out
}
fn normalize_news(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { ' ' })
        .collect::<String>()
}

/// Returns media only after the browser receives an explicit user click.
/// Embeds are restricted to the publisher's documented YouTube hosts and a valid ID.
pub fn media_markup(item: &Value, index: usize) -> Option<String> {
    let media = a(&item["media"]).get(index)?;
    let url = s(media, "url");
    if !safe_url(url) {
        return None;
    }
    let title = esc(s(item, "title"));
    let content=match s(media,"kind") {
        "image"=>format!("<img src=\"{}\" alt=\"Publisher image for {title}\" referrerpolicy=\"no-referrer\" decoding=\"async\">",esc(url)),
        "audio"|"video"=>format!("<{} src=\"{}\" controls preload=\"none\"></{}>",s(media,"kind"),esc(url),s(media,"kind")),
        "embed"=>{
            let target=url.strip_prefix("https://").or_else(||url.strip_prefix("http://"))?;
            let (host,path)=target.split_once('/').unwrap_or((target,""));
            let host=host.to_ascii_lowercase();
            if !["youtu.be","youtube.com","www.youtube.com","www.youtube-nocookie.com"].contains(&host.as_str()){return None;}
            let id=if host=="youtu.be"{path.split(['?','#','/']).next().unwrap_or("").to_string()}
                else if let Some(path)=path.strip_prefix("embed/"){path.split(['?','#','/']).next().unwrap_or("").to_string()}
                else{path.split_once('?').and_then(|(_,q)|q.split('&').find_map(|entry|{let(k,v)=entry.split_once('=')?;if k=="v"{Some(decode(v))}else{None}})).unwrap_or_default()};
            if id.len()!=11||!id.bytes().all(|c|c.is_ascii_alphanumeric()||c==b'_'||c==b'-'){return None;}
            format!("<iframe title=\"{title}\" src=\"https://www.youtube-nocookie.com/embed/{id}\" sandbox=\"allow-scripts allow-same-origin allow-presentation\" referrerpolicy=\"strict-origin-when-cross-origin\" allowfullscreen></iframe>")
        },
        _=>return None,
    };
    Some(format!(
        "{}{}",
        content,
        external(url, "Open original media", "")
    ))
}

pub fn agent_candidates(manager: &str, output: &str) -> Vec<String> {
    let lines: Vec<_> = output
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();
    let mut values = Vec::new();
    for line in &lines {
        let first = line.split_whitespace().next().unwrap_or("");
        let candidate = match manager {
            "choco" if line.contains('|') => line.split('|').next().unwrap_or(""),
            "apt" => first.split_once('/').map(|p| p.0).unwrap_or(""),
            "dnf"
                if !["Last metadata", "Obsoleting", "Available", "Installed"]
                    .iter()
                    .any(|v| line.starts_with(v))
                    && first.contains('.') =>
            {
                first
            }
            "pacman" if line.contains(" -> ") => first,
            "flatpak" if first.contains('/') => first,
            "snap" if !["Name", "Refresh"].contains(&first) => first,
            "brew" => first.split('(').next().unwrap_or(""),
            "scoop" if !["Name", "Installed"].contains(&first) => first,
            _ => "",
        };
        if !candidate.is_empty() {
            values.push(candidate.to_string());
        }
    }
    if manager == "winget" {
        if let Some(header) = lines.iter().position(|line| {
            line.split_whitespace().any(|v| v == "Id")
                && line.split_whitespace().any(|v| v == "Version")
        }) {
            let columns = split_columns(lines[header]);
            if let Some(id_index) = columns.iter().position(|c| c.eq_ignore_ascii_case("id")) {
                for line in &lines[header + 1..] {
                    if let Some(value) = split_columns(line).get(id_index) {
                        values.push((*value).to_string());
                    }
                }
            }
        }
    }
    let mut seen = BTreeSet::new();
    values
        .into_iter()
        .filter(|v| {
            !v.starts_with('-')
                && v.len() <= 180
                && v.chars()
                    .all(|c| c.is_ascii_alphanumeric() || "._+-:/@".contains(c))
                && seen.insert(v.clone())
        })
        .collect()
}
fn split_columns(line: &str) -> Vec<&str> {
    let bytes = line.as_bytes();
    let mut start = 0;
    let mut i = 0;
    let mut out = Vec::new();
    while i < bytes.len() {
        if bytes[i].is_ascii_whitespace() {
            let begin = i;
            while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            if i - begin >= 2 {
                let cell = line[start..begin].trim();
                if !cell.is_empty() {
                    out.push(cell);
                }
                start = i;
            }
        } else {
            i += 1;
        }
    }
    let tail = line[start..].trim();
    if !tail.is_empty() {
        out.push(tail);
    }
    out
}
pub fn agent_inventory(data: &Value) -> String {
    let apps = a(&data["applications"]);
    let mut out=format!("<div class=\"agent-inventory-summary\"><strong>{}</strong> registered apps · <strong>{}</strong> package managers · checked {}</div><div class=\"agent-preview-grid\">",apps.len(),a(&data["managers"]).len(),esc(s(data,"collectedAt")));
    for item in a(&data["updatePreviews"]) {
        let manager = s(item, "manager");
        let candidates = agent_candidates(manager, s(item, "output"));
        let code = item["exitCode"].as_i64();
        let ready = code == Some(0)
            || (manager == "dnf" && code == Some(100))
            || (manager == "pacman" && code == Some(2));
        let _=write!(out,"<details class=\"agent-preview-card\"><summary><strong>{}</strong><span>{} · {} candidates</span></summary><div class=\"agent-candidate-list\">",esc(manager),if ready{"Preview ready".into()}else{format!("Check output · {}",txt(&item["exitCode"]))},candidates.len());
        for id in candidates {
            let _=write!(out,"<div><code>{}</code><button type=\"button\" data-agent-update-package=\"{}\" data-agent-package-manager=\"{}\">Update</button></div>",esc(&id),esc(&id),esc(manager));
        }
        let _ = write!(out, "</div><pre>{}</pre></details>", esc(s(item, "output")));
    }
    let _=write!(out,"</div><details class=\"agent-app-inventory\"><summary>Registered applications ({})</summary><div class=\"agent-app-list\">",apps.len());
    for app in apps {
        let _ = write!(
            out,
            "<div><strong>{}</strong><span>{}</span></div>",
            esc(s(app, "name")),
            esc(&[s(app, "version"), s(app, "publisher"), s(app, "scope")]
                .into_iter()
                .filter(|v| !v.is_empty())
                .collect::<Vec<_>>()
                .join(" · "))
        );
    }
    out.push_str("</div></details>");
    out
}
pub fn agent_operations(records: &Value) -> String {
    let mut out = String::new();
    for item in a(records) {
        let output = if !s(item, "error").is_empty() {
            s(item, "error").into()
        } else if !s(item, "output").is_empty() {
            s(item, "output").into()
        } else {
            format!("Exit code: {}", txt(&item["exitCode"]))
        };
        let _=write!(out,"<article class=\"agent-operation-row\"><div><strong>{} · {}</strong><span>{} · {} · {}</span></div><details><summary>Output / details</summary><pre>{}</pre></details></article>",esc(s(item,"action")),esc(s(item,"packageId")),esc(s(item,"manager")),esc(s(item,"state")),esc(s(item,"startedAt")),esc(&output));
    }
    if out.is_empty() {
        out.push_str("<p class=\"prose\">No maintenance operations have been run from this computer yet.</p>");
    }
    out
}
fn news(app: &App, route: &Route) -> String {
    let data = &app.data["news"];
    let sources = a(&data["sources"]);
    let mut out=heading("Technician intelligence","Technician News","Find official alerts first, then compare vendor updates, reporting, and community perspectives.");
    let _=write!(out,"<div class=\"news-banner\"><div><strong>Start with the source</strong><p>Check the affected version in a vendor bulletin before changing a customer system. Treat social and reporting links as leads.</p></div><span>Catalog reviewed {}</span></div>",time_label(s(data,"reviewedAt")));
    out.push_str(&digest(app, route));
    if !a(&data["briefing"]).is_empty() {
        out.push_str("<section aria-labelledby=\"news-brief-title\"><h2 id=\"news-brief-title\">Selected briefing</h2><p class=\"news-muted\">Dated editorial snapshot. Open the publisher for the latest changes.</p><div class=\"news-grid\">");
        for item in a(&data["briefing"]) {
            let source = find(&data["sources"], s(item, "sourceId"));
            let _ = write!(
                out,
                "<article class=\"news-brief\"><span>{} · {}</span><h3>{}</h3><p>{}</p></article>",
                source
                    .map(|r| external(s(r, "url"), &esc(s(r, "name")), ""))
                    .unwrap_or_default(),
                time_label(s(item, "date")),
                external(s(item, "url"), &esc(s(item, "title")), ""),
                field(item, "description")
            );
        }
        out.push_str("</div></section>");
    }
    let query = normalize_news(route.get("q"));
    let found: Vec<_> = sources
        .iter()
        .filter(|source| {
            (route.get("category").is_empty() || route.get("category") == s(source, "category"))
                && (route.get("saved") != "1" || app.news_sources.contains(s(source, "id")))
                && matches(
                    &normalize_news(&format!(
                        "{} {} {} {}",
                        s(source, "name"),
                        s(source, "description"),
                        s(source, "url"),
                        news_group(s(source, "category"))
                    )),
                    &query,
                )
        })
        .collect();
    let _=write!(out,"<section aria-labelledby=\"news-sources-title\"><h2 id=\"news-sources-title\">Source directory</h2><p class=\"news-muted\">The directory works offline. Open publisher pages for current updates; saved sources stay in this browser.</p><div class=\"news-controls\"><label>Search sources<input id=\"news-search\" type=\"search\" placeholder=\"CISA, Microsoft, NIST…\" value=\"{}\"></label><label>Category<select id=\"news-category\"><option value=\"\">All categories</option>",esc(route.get("q")));
    for (id, label, _) in NEWS_GROUPS {
        let _ = write!(
            out,
            "<option value=\"{id}\"{}>{}</option>",
            if route.get("category") == *id {
                " selected"
            } else {
                ""
            },
            esc(label)
        );
    }
    let _=write!(out,"</select></label><label class=\"news-saved\"><input id=\"news-saved\" type=\"checkbox\"{}> Saved only</label></div><p id=\"news-count\" aria-live=\"polite\">{} of {} sources</p><div id=\"news-directory\">",if route.get("saved")=="1"{" checked"}else{""},found.len(),sources.len());
    for (id, label, description) in NEWS_GROUPS {
        let rows: Vec<_> = found
            .iter()
            .copied()
            .filter(|s1| s(s1, "category") == *id)
            .collect();
        if rows.is_empty() {
            continue;
        }
        let _=write!(out,"<section class=\"news-source-group\" aria-labelledby=\"news-group-{id}\"><h3 id=\"news-group-{id}\">{}</h3><p class=\"news-muted\">{}</p><div class=\"news-grid\">",esc(label),esc(description));
        for source in rows {
            let saved = app.news_sources.contains(s(source, "id"));
            let _=write!(out,"<article class=\"news-card\" data-news-id=\"{}\"><div><h4>{}</h4><p>{}</p></div><div class=\"news-actions\"><button class=\"button\" type=\"button\" data-news-save=\"{}\" aria-label=\"{} {}\" aria-pressed=\"{saved}\">{}</button>{}</div></article>",esc(s(source,"id")),external(s(source,"url"),&esc(s(source,"name")),""),field(source,"description"),esc(s(source,"id")),if saved{"Unsave"}else{"Save"},esc(s(source,"name")),if saved{"Saved"}else{"Save"},if safe_url(s(source,"feed")){external(s(source,"feed"),"Feed","")}else{String::new()});
        }
        out.push_str("</div></section>");
    }
    if found.is_empty() {
        out.push_str("<p>No sources match. Try a broader search or turn off Saved only.</p>");
    }
    out.push_str("</div></section>");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    fn fixture() -> App {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data");
        let read = |name: &str| {
            serde_json::from_str::<Value>(&std::fs::read_to_string(root.join(name)).unwrap())
                .unwrap()
        };
        App::new(
            json!({"tools":read("tools.json"),"commandCenter":read("command-center.json"),"updatesCenter":read("updates-center.json"),"onlineTools":read("online-tools.json"),"news":read("news-sources.json"),"newsDigest":read("news-digest.json")}),
        )
    }
    #[test]
    fn every_command_and_workflow_remains_rendered() {
        let app = fixture();
        for (route, key, command_attr, workflow_attr) in [
            (
                "command-center",
                "commandCenter",
                "data-command-item",
                "data-copy-center-workflow",
            ),
            (
                "updates",
                "updatesCenter",
                "data-update-command-card",
                "data-copy-update-workflow",
            ),
        ] {
            let html = render(&app, &Route::parse(route));
            for item in a(&app.data[key]["commands"]) {
                assert!(
                    html.contains(&format!("{command_attr}=\"{}\"", s(item, "id"))),
                    "missing command {}",
                    s(item, "id")
                );
                assert!(html.contains(&esc(s(item, "code"))));
            }
            for flow in a(&app.data[key]["workflows"]) {
                assert!(
                    html.contains(&format!("{workflow_attr}=\"{}\"", s(flow, "id"))),
                    "missing workflow {}",
                    s(flow, "id")
                );
            }
        }
    }
    #[test]
    fn online_directory_preserves_every_service_and_its_guidance() {
        let mut app = fixture();
        if app.data["tools"].is_object() {
            app.data["tools"] = app.data["tools"]["tools"].clone();
        }
        let html = online(&app, &Route::parse("online"));
        for entry in a(&app.data["onlineTools"]["tools"]) {
            assert!(
                html.contains(&format!("data-online-tool=\"{}\"", s(entry, "toolId"))),
                "missing online service {}",
                s(entry, "toolId")
            );
            assert!(html.contains(&field(entry, "sharing")));
            assert!(html.contains(&field(entry, "expectedResult")));
        }
        let only_saved = online(&app, &Route::parse("online?saved=1"));
        assert!(!only_saved.contains("data-online-tool="));
        assert!(only_saved.contains("No matching online tools"));
    }
    #[test]
    fn news_preserves_sources_and_opt_in_media() {
        let app = fixture();
        let html = news(&app, &Route::parse("news"));
        for source in a(&app.data["news"]["sources"]) {
            assert!(html.contains(&format!("data-news-id=\"{}\"", s(source, "id"))));
        }
        for item in a(&app.data["newsDigest"]["items"]) {
            assert!(html.contains(&format!("data-digest-id=\"{}\"", s(item, "id"))));
        }
        assert!(!html.contains("<iframe"));
        assert!(!html.contains("<video"));
        assert!(!html.contains("<audio"));
    }
    #[test]
    fn media_embeds_restrict_host_and_identifier() {
        let valid = json!({"title":"Publisher video","media":[{"kind":"embed","url":"https://youtu.be/abcdefghijk"}]});
        assert!(media_markup(&valid, 0)
            .unwrap()
            .contains("https://www.youtube-nocookie.com/embed/abcdefghijk"));
        let wrong_host = json!({"media":[{"kind":"embed","url":"https://youtube.com.evil.example/watch?v=abcdefghijk"}]});
        assert!(media_markup(&wrong_host, 0).is_none());
        let wrong_id =
            json!({"media":[{"kind":"embed","url":"https://www.youtube.com/watch?v=bad"}]});
        assert!(media_markup(&wrong_id, 0).is_none());
    }
    #[test]
    fn candidate_ids_use_manager_output_and_reject_options() {
        assert_eq!(
            agent_candidates(
                "winget",
                "Name  Id  Version  Available\nTool  Publisher.Tool  1.0  2.0"
            ),
            vec!["Publisher.Tool"]
        );
        assert_eq!(
            agent_candidates("choco", "package|1.0|2.0\n--danger|x|y\npackage|1.0|2.0"),
            vec!["package"]
        );
        assert!(agent_candidates("unknown", "any-package 1.0 -> 2.0").is_empty());
    }
    #[test]
    fn update_group_filters_preserve_exact_manager_and_impact() {
        let item = json!({"title":"Repair Windows Update cache","category":"Windows Update","risk":"changes-system","shell":"PowerShell","code":"repair","summary":"failed update"});
        assert!(command_matches(
            &item,
            &Route::parse("#updates?category=recovery&q=failed%20cache"),
            true
        ));
        assert!(!command_matches(
            &item,
            &Route::parse("#updates?category=recovery&risk=read-only"),
            true
        ));
        assert!(!command_matches(
            &item,
            &Route::parse("#updates?category=Chocolatey"),
            true
        ));
    }
    #[test]
    fn provenance_never_calls_retained_items_current_feeds() {
        assert_eq!(
            provenance(Some(&json!({"id":"vendor","status":"unavailable"}))),
            "retained"
        );
        assert_eq!(
            provenance(Some(&json!({"id":"windows-health"}))),
            "editorial"
        );
        assert!(source_status(None, "feed").contains("unknown"));
    }
    #[test]
    fn copy_cards_escape_command_text_without_executing() {
        let html = command_card(
            &json!({"id":"example","title":"Example","code":"echo '<script>alert(1)</script>'","risk":"read-only","sourceUrl":"javascript:alert(1)"}),
            false,
        );
        assert!(html.contains("&lt;script&gt;"));
        assert!(html.contains("data-copy-center-command=\"example\""));
        assert!(!html.contains("href=\"javascript:"));
    }
}
