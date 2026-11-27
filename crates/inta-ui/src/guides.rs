//! Complete guide, checklist, service map and reference views shared by every client.
use crate::{common::*, App, Route};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

fn value(v: &Value) -> &str {
    v.as_str().unwrap_or("")
}
fn fallback<'a>(primary: &'a str, secondary: &'a str) -> &'a str {
    if primary.is_empty() {
        secondary
    } else {
        primary
    }
}
fn workspace<'a>(app: &'a App, job: &Value) -> &'a Value {
    &app.data["jobWorkspaces"]["jobs"][s(job, "id")]
}
fn quicks<'a>(app: &'a App, id: &str) -> Vec<&'a Value> {
    a(&app.data["quickStart"]["items"])
        .iter()
        .filter(|q| s(q, "jobId") == id)
        .collect()
}
fn notes<'a>(app: &'a App, id: &str) -> Vec<&'a Value> {
    a(&app.data["fieldNotes"]["notes"])
        .iter()
        .filter(|n| s(n, "jobId") == id)
        .collect()
}
fn quick_href(q: &Value) -> String {
    format!(
        "#job/{}/quick/{}",
        encode(s(q, "jobId")),
        encode(s(q, "id"))
    )
}
fn section_href(job: &Value, quick: &str, section: &str) -> String {
    format!(
        "#job/{}{}?section={}",
        encode(s(job, "id")),
        if quick.is_empty() {
            String::new()
        } else {
            format!("/quick/{}", encode(quick))
        },
        encode(section)
    )
}
fn block_title(id: &str, label: &str, kind: &str) -> String {
    format!(
        r##"<h2 class="block-heading" id="{}" tabindex="-1">{}{}</h2>"##,
        esc(id),
        icon(kind),
        esc(label)
    )
}
fn list(items: &Value, class: &str) -> String {
    format!(
        r##"<ul class="{}">{}</ul>"##,
        esc(class),
        a(items)
            .iter()
            .map(|x| format!("<li>{}</li>", formatted(&txt(x))))
            .collect::<String>()
    )
}
fn job_icon(job: &Value) -> &'static str {
    let text = format!(
        "{} {} {}",
        s(job, "id"),
        s(job, "title"),
        s(job, "category")
    )
    .to_lowercase();
    for (needles, name) in [
        ("malware virus ransom incident", "shield"),
        ("internet dns network", "network"),
        ("remote", "monitor"),
        ("mail outlook", "mail"),
        ("print", "printer"),
        ("backup restore migration imaging", "cloud"),
        ("disk drive storage recovery", "drive"),
        ("boot usb", "usb"),
        ("ai assistant", "terminal"),
        ("trust domain bitlocker account", "lock"),
        ("slow performance hardware bsod", "activity"),
        ("intake handoff onboard", "file"),
    ] {
        if needles.split_whitespace().any(|n| text.contains(n)) {
            return name;
        }
    }
    "wrench"
}
pub fn job_card(job: &Value) -> String {
    let href = format!("#job/{}", encode(s(job, "id")));
    format!(
        r##"<article class="job-row" role="listitem" data-job-route="{href}"><span class="job-row-icon">{}</span><div class="job-row-main"><h3><a href="{href}">{}</a></h3><p>{}</p></div><div class="job-row-meta"><span>{}</span><span>{}</span></div><a class="job-row-open" href="{href}" aria-label="Open {}">Open guide {}</a></article>"##,
        icon(job_icon(job)),
        esc(s(job, "title")),
        formatted(s(job, "summary")),
        esc(s(job, "estimatedTime")),
        esc(fallback(s(job, "difficulty"), s(job, "category"))),
        esc(s(job, "title")),
        icon("arrow")
    )
}
fn identity_entries(job: &Value) -> Vec<(String, usize, String, String)> {
    let mut out: Vec<_> = a(&job["steps"])
        .iter()
        .enumerate()
        .map(|(i, step)| {
            (
                "step".into(),
                i,
                s(step, "title").into(),
                s(step, "anchor").into(),
            )
        })
        .collect();
    out.extend(
        a(&job["verify"])
            .iter()
            .enumerate()
            .map(|(i, v)| ("verify".into(), i, txt(v), String::new())),
    );
    out
}
pub fn checklist_key(job: &Value, kind: &str, index: usize, title: &str) -> String {
    let entries = identity_entries(job);
    if let Some((_, _, actual, anchor)) =
        entries.iter().find(|(k, i, _, _)| k == kind && *i == index)
    {
        if !anchor.is_empty()
            && entries
                .iter()
                .filter(|(k, _, _, a)| k == kind && a == anchor)
                .count()
                == 1
        {
            return format!("{kind}:anchor:{anchor}");
        }
        if entries
            .iter()
            .filter(|(k, _, t, _)| k == kind && t == actual)
            .count()
            == 1
        {
            return format!("{kind}:title:{actual}");
        }
        return format!("{kind}:index:{index}:{actual}");
    }
    format!("{kind}:title:{title}")
}
pub fn migrate_checks(app: &mut App) {
    if !app.checks.is_object() {
        app.checks = json!({});
        return;
    }
    let jobs = app.jobs().to_vec();
    for job in &jobs {
        let Some(state) = app.checks[s(job, "id")].as_object_mut() else {
            continue;
        };
        let entries = identity_entries(job);
        for (kind, index, title, _) in &entries {
            if entries
                .iter()
                .filter(|(k, _, t, _)| k == kind && t == title)
                .count()
                != 1
            {
                continue;
            }
            let key = checklist_key(job, kind, *index, title);
            if state.contains_key(&key) {
                continue;
            }
            let candidates: Vec<_> = state
                .keys()
                .filter(|old| {
                    if **old == format!("{kind}:title:{title}") {
                        return **old != key;
                    }
                    let mut parts = old.splitn(3, ':');
                    parts.next() == Some(kind.as_str())
                        && parts
                            .next()
                            .is_some_and(|i| !i.is_empty() && i.chars().all(|c| c.is_ascii_digit()))
                        && parts.next() == Some(title.as_str())
                })
                .cloned()
                .collect();
            if candidates.len() == 1 && state[&candidates[0]].is_boolean() {
                if let Some(v) = state.remove(&candidates[0]) {
                    state.insert(key, v);
                }
            }
        }
    }
}
fn checked(app: &App, job: &Value, kind: &str, index: usize, title: &str) -> bool {
    app.checks[s(job, "id")][checklist_key(job, kind, index, title)] == true
}
fn closeout_rows(job: &Value) -> Vec<(usize, &Value)> {
    a(&job["steps"])
        .iter()
        .enumerate()
        .filter(|(_, s)| self::s(s, "anchor").starts_with("step-closeout-"))
        .collect()
}
pub fn progress(app: &App, job: &Value, focus: bool) -> (usize, usize) {
    let flags: Vec<bool> = if focus {
        closeout_rows(job)
            .iter()
            .map(|(i, x)| checked(app, job, "step", *i, s(x, "title")))
            .collect()
    } else {
        a(&job["steps"])
            .iter()
            .enumerate()
            .map(|(i, x)| checked(app, job, "step", i, s(x, "title")))
            .chain(
                a(&job["verify"])
                    .iter()
                    .enumerate()
                    .map(|(i, x)| checked(app, job, "verify", i, &txt(x))),
            )
            .collect()
    };
    (flags.iter().filter(|x| **x).count(), flags.len())
}
fn progress_card(app: &App, job: &Value, focus: bool) -> String {
    let (done, total) = progress(app, job, focus);
    let percent = if total == 0 {
        0
    } else {
        ((done as f64 / total as f64) * 100.0).round() as usize
    };
    format!(
        r##"<h3>Your checklist</h3><div class="progress-top"><strong id="progress-percentage">{percent}%</strong><span id="progress-count">{done} of {total} checks</span></div><div class="progress-track" role="progressbar" aria-label="Playbook completion" aria-valuemin="0" aria-valuemax="100" aria-valuenow="{percent}"><span style="width:{percent}%"></span></div><p class="progress-note" id="progress-note">{}</p><button class="checklist-reset" data-reset-checklist="{}">Start a fresh checklist</button>"##,
        if app.storage_available {
            "Progress stays in this browser. Start a fresh checklist for each new job."
        } else {
            "Progress lasts for this visit because browser storage is unavailable. Export your checklist to keep it."
        },
        esc(s(job, "id"))
    )
}
fn checkbox(app: &App, job: &Value, kind: &str, index: usize, title: &str) -> String {
    format!(
        r##"<input type="checkbox" data-check-job="{}" data-check-type="{}" data-check-index="{}" data-check-title="{}"{}>"##,
        esc(s(job, "id")),
        esc(kind),
        index,
        esc(title),
        if checked(app, job, kind, index, title) {
            " checked"
        } else {
            ""
        }
    )
}
fn playbooks(app: &App, route: &Route) -> String {
    let query = route.get("q");
    let category = route.get("category");
    let categories: BTreeSet<_> = app
        .jobs()
        .iter()
        .map(|j| s(j, "category").to_string())
        .collect();
    let found: Vec<_> = app
        .jobs()
        .iter()
        .filter(|job| {
            let mut haystack = job.to_string();
            haystack.push_str(&workspace(app, job).to_string());
            for q in quicks(app, s(job, "id")) {
                haystack.push_str(&q.to_string());
            }
            for n in notes(app, s(job, "id")) {
                haystack.push_str(&n.to_string());
            }
            matches(&haystack, query) && (category.is_empty() || s(job, "category") == category)
        })
        .collect();
    let rows = if found.is_empty() {
        empty(
            "Try another symptom",
            "Search a broad term such as network, boot, backup or malware.",
            "#playbooks",
            "Clear filters",
        )
    } else {
        found.iter().map(|j| job_card(j)).collect()
    };
    heading(
        "INTA field guide",
        "Job playbooks",
        "Choose a job for its procedure, working tools, verification and escalation path.",
    ) + &format!(
        r##"<div class="filters"><label class="input-wrap">{}<span class="sr-only">Search playbooks</span><input id="jobs-search" type="search" value="{}" placeholder="Describe the job or symptom…"></label><label class="sr-only" for="jobs-category">Playbook category</label><select class="filter-select" id="jobs-category">{}</select></div><div class="filter-result"><span id="jobs-result-count" aria-live="polite"><strong>{}</strong> of {} playbooks</span><button data-reset-jobs>Clear filters</button></div><div class="job-list" id="job-results" role="list">{rows}</div>"##,
        icon("search"),
        esc(query),
        options(
            &categories.into_iter().collect::<Vec<_>>(),
            "All job types",
            category
        ),
        found.len(),
        app.jobs().len()
    )
}
fn quick_answers(app: &App, job: &Value, quick: &str) -> String {
    let mut picks = quicks(app, s(job, "id"));
    if picks.is_empty() {
        return String::new();
    }
    let position = picks.iter().position(|q| s(q, "id") == quick).unwrap_or(0);
    let selected = picks.remove(position);
    picks.insert(0, selected);
    let rows=picks.iter().enumerate().map(|(i,item)|format!(r##"<article class="quick-answer" id="quick-{}" data-quick-answer="{}"{}><div class="quick-answer-steps"><h3 tabindex="-1">{} {}{}</h3><p class="answer-first-step">{}</p><ol>{}</ol></div><div class="quick-answer-tool">{}</div></article>"##,esc(s(item,"id")),esc(s(item,"id")),if i==0{" data-current-case=\"true\""}else{""},esc(s(item,"icon")),esc(s(item,"label")),if i==0&&picks.len()>1{"<span class=\"current-case\">Selected case</span>"}else{""},formatted(fallback(s(item,"firstStep"),s(job,"startHere"))),a(&item["quickSteps"]).iter().map(|v|format!("<li>{}</li>",formatted(&txt(v)))).collect::<String>(),app.quick_choices(item))).collect::<String>();
    format!(
        r##"<section class="job-answer-strip" aria-label="First actions"><h2>Do this first</h2>{rows}</section>"##
    )
}
#[derive(Clone)]
struct ToolEntry<'a> {
    tool: &'a Value,
    action: String,
    why: String,
    choice: Option<&'a Value>,
}
fn tool_entries<'a>(app: &'a App, job: &Value, quick: &str) -> Vec<ToolEntry<'a>> {
    let ws = workspace(app, job);
    let mut entries: Vec<_> = a(&ws["tools"])
        .iter()
        .filter_map(|x| {
            app.tool(s(x, "toolId")).map(|t| ToolEntry {
                tool: t,
                action: s(x, "actionId").into(),
                why: s(x, "why").into(),
                choice: None,
            })
        })
        .collect();
    for id in a(&job["tools"]) {
        if !entries.iter().any(|e| s(e.tool, "id") == value(id)) {
            if let Some(t) = app.tool(value(id)) {
                entries.push(ToolEntry {
                    tool: t,
                    action: String::new(),
                    why: fallback(s(t, "bestFor"), s(t, "summary")).into(),
                    choice: None,
                });
            }
        }
    }
    let picks = quicks(app, s(job, "id"));
    let selected = picks
        .iter()
        .copied()
        .find(|q| s(q, "id") == quick)
        .or_else(|| picks.first().copied());
    let mut preferred = Vec::new();
    if let Some(item) = selected {
        for choice in a(&item["choices"]) {
            if let Some(tool) = app.tool(s(choice, "toolId")) {
                let index = entries
                    .iter()
                    .position(|e| {
                        s(e.tool, "id") == s(choice, "toolId") && e.action == s(choice, "actionId")
                    })
                    .or_else(|| {
                        entries
                            .iter()
                            .position(|e| s(e.tool, "id") == s(choice, "toolId"))
                    });
                let mut entry = index.map(|i| entries.remove(i)).unwrap_or(ToolEntry {
                    tool,
                    action: String::new(),
                    why: s(choice, "why").into(),
                    choice: None,
                });
                entry.action = s(choice, "actionId").into();
                entry.choice = Some(choice);
                preferred.push(entry);
            }
        }
        // Preserve the scenario's original action even when the authored choice list has additional tools.
        if !s(item, "toolId").is_empty()
            && !preferred.iter().any(|e| {
                s(e.tool, "id") == s(item, "toolId")
                    && (s(item, "toolAction").is_empty() || e.action == s(item, "toolAction"))
            })
        {
            if let Some(tool) = app.tool(s(item, "toolId")) {
                let index = entries
                    .iter()
                    .position(|e| s(e.tool, "id") == s(tool, "id"));
                let mut entry = index.map(|i| entries.remove(i)).unwrap_or(ToolEntry {
                    tool,
                    action: String::new(),
                    why: s(item, "why").into(),
                    choice: None,
                });
                entry.action = s(item, "toolAction").into();
                entry.choice = Some(item);
                preferred.push(entry);
            }
        }
    }
    preferred.extend(entries);
    preferred
}
fn tool_access(app: &App, entry: &ToolEntry) -> String {
    format!(
        r##"{}<a class="workspace-options-link" href="#tool/{}">All options {}</a>"##,
        app.access_actions(entry.tool, &entry.action, true),
        encode(s(entry.tool, "id")),
        icon("arrow")
    )
}
fn resource_group(group: &str) -> String {
    let group = fallback(group, "Official resources").replace(['-', '_'], " ");
    let mut chars = group.chars();
    match chars.next() {
        Some(c) => format!("{}{}", c.to_uppercase(), chars.as_str()),
        None => String::new(),
    }
}
fn tool_panel(app: &App, job: &Value, quick: &str) -> String {
    let entries = tool_entries(app, job, quick);
    let ws = workspace(app, job);
    let resources = a(&ws["resources"]);
    let tool_count = entries
        .iter()
        .map(|e| s(e.tool, "id"))
        .collect::<BTreeSet<_>>()
        .len();
    let picks = quicks(app, s(job, "id"));
    let selected = picks
        .iter()
        .copied()
        .find(|q| s(q, "id") == quick)
        .or_else(|| picks.first().copied());
    let priority: Vec<_> = entries
        .iter()
        .filter(|e| {
            selected.is_none()
                || e.choice.is_some()
                || a(&e.tool["tags"]).iter().any(|t| value(t) == "discovery")
        })
        .collect();
    let priority_html = if priority.is_empty() {
        "<p class=\"no-extra-tool\">No extra application is required for this job.</p>".into()
    } else {
        format!(r##"<section class="job-priority-section"><h3 class="panel-group-heading">{}</h3><div class="job-priority-tools">{}</div></section>"##,if selected.is_some_and(|q|q["guideOnly"]==true){"Optional tools for this job"}else{"Choose by role"},priority.iter().map(|e|format!(r##"<article class="job-priority-tool" data-priority-tool="{}" data-priority-action="{}"><h4>{}</h4>{}{}</article>"##,esc(s(e.tool,"id")),esc(&e.action),app.tool_name(e.tool),e.choice.map(|c|format!("<p class=\"choice-role\">{}</p><p class=\"choice-use\">{}</p>",formatted(s(c,"role")),formatted(s(c,"freeUse")))).unwrap_or_default(),app.access_actions(e.tool,&e.action,true))).collect::<String>())
    };
    let drivers: Vec<_> = resources
        .iter()
        .filter(|r| {
            s(r, "kind") == "driver"
                || s(r, "group").to_lowercase().contains("manufacturer")
                || s(r, "group").to_lowercase().contains("oem")
        })
        .collect();
    let driver_html = if drivers.is_empty() {
        String::new()
    } else {
        format!(
            r##"<nav class="job-driver-shortcuts" aria-label="Manufacturer driver shortcuts"><h3 class="panel-group-heading">Get the driver</h3><div>{}</div></nav>"##,
            drivers
                .iter()
                .map(|r| external(
                    s(r, "url"),
                    &format!("<span>{}</span>{}", esc(s(r, "label")), icon("external")),
                    "driver-shortcut"
                ))
                .collect::<String>()
        )
    };
    let mut seen = BTreeSet::new();
    let supporting=entries.iter().map(|e|{let first=seen.insert(s(e.tool,"id"));format!(r##"<article class="job-workspace-tool" data-workspace-tool="{}" data-workspace-action="{}"{}><h4>{}</h4><p>{}</p>{}</article>"##,esc(s(e.tool,"id")),esc(&e.action),if first{format!(r##" data-recommended-tool="{}""##,esc(s(e.tool,"id")))}else{String::new()},app.tool_name(e.tool),formatted(&e.why),tool_access(app,e))}).collect::<String>();
    let mut groups: Vec<(&str, Vec<&Value>)> = Vec::new();
    for resource in resources {
        let label = fallback(s(resource, "group"), "Official resources");
        if let Some((_, items)) = groups.iter_mut().find(|(g, _)| *g == label) {
            items.push(resource);
        } else {
            groups.push((label, vec![resource]));
        }
    }
    let groups_html=groups.iter().enumerate().map(|(i,(group,items))|format!(r##"<section class="job-resource-group" data-resource-group="{}" aria-labelledby="job-resource-group-{i}"><h3 class="panel-group-heading" id="job-resource-group-{i}">{}</h3><div class="job-resource-list">{}</div></section>"##,esc(group),esc(&resource_group(group)),items.iter().map(|r|format!(r##"<article class="job-resource" id="job-resource-{}" data-workspace-resource="{}">{}<p>{}</p><div class="job-resource-meta"><span>{}</span>{}<time datetime="{}">Checked {}</time></div></article>"##,esc(s(r,"id")),esc(s(r,"id")),external(s(r,"url"),&format!("<span>{}</span>{}",esc(s(r,"label")),icon("external")),"job-resource-link"),formatted(s(r,"description")),esc(&resource_group(s(r,"kind"))),if s(r,"sourceUrl")!=s(r,"url"){external(s(r,"sourceUrl"),"Source","resource-source")}else{String::new()},esc(s(r,"checkedAt")),esc(s(r,"checkedAt")))).collect::<String>())).collect::<String>();
    format!(
        r##"<aside class="job-tool-panel" id="job-tools" aria-labelledby="job-tools-heading"><header class="job-tool-panel-heading"><div><h2 id="job-tools-heading" tabindex="-1">{}Tools &amp; resources</h2><p>{tool_count} tools · {} resources</p></div><a class="panel-procedure-link" href="{}">{}Steps</a></header><div class="job-panel-content" tabindex="0" role="region" aria-label="All tools and official resources for this job">{priority_html}{driver_html}<section class="job-panel-tools"><h3 class="panel-group-heading">Supporting tools &amp; usage</h3>{supporting}</section>{groups_html}</div></aside>"##,
        icon("wrench"),
        resources.len(),
        section_href(job, quick, "procedure"),
        icon("route")
    )
}
fn job_navigation(app: &App, job: &Value, quick: &str) -> String {
    let mut sections = vec![("tools", "Tools & resources")];
    if !a(&workspace(app, job)["decisions"]).is_empty() {
        sections.push(("decisions", "Decisions"));
    }
    sections.extend([("procedure", "Procedure"), ("verify", "Verify")]);
    if !notes(app, s(job, "id")).is_empty() {
        sections.push(("fixes", "Specific fixes"));
    }
    if !a(&job["sources"]).is_empty() {
        sections.push(("references", "Sources"));
    }
    let links = sections
        .iter()
        .map(|(id, label)| {
            format!(
                r##"<a href="{}">{}</a>"##,
                section_href(job, quick, id),
                esc(label)
            )
        })
        .collect::<String>();
    format!(
        r##"<nav class="job-section-nav" aria-label="Sections in this job guide">{links}</nav><nav class="job-mobile-jump" aria-label="Quick navigation in this job guide">{}</nav>"##,
        [
            ("tools", "Tools", "wrench"),
            ("procedure", "Steps", "route"),
            ("verify", "Verify", "checkCircle"),
            ("fixes", "Fixes", "life")
        ]
        .iter()
        .filter(|(id, _, _)| *id != "fixes" || !notes(app, s(job, "id")).is_empty())
        .map(|(id, label, kind)| format!(
            r##"<a href="{}">{}{label}</a>"##,
            section_href(job, quick, id),
            icon(kind)
        ))
        .collect::<String>()
    )
}
fn working_plan(ws: &Value) -> String {
    if !ws.is_object() {
        return String::new();
    }
    let mut html = format!(
        r##"<section class="workspace-start">{}<p class="workspace-goal">{}</p><p>{}</p></section>"##,
        block_title("workspace-start-heading", "Plan the job", "compass"),
        formatted(s(ws, "goal")),
        formatted(s(ws, "startHere"))
    );
    if !a(&ws["decisions"]).is_empty() {
        html.push_str(&format!(r##"<section class="workspace-decisions" id="job-decisions">{}<div class="decision-table-wrap"><table class="decision-table"><thead><tr><th scope="col">What you find</th><th scope="col">What to do</th><th scope="col">What to expect</th></tr></thead><tbody>{}</tbody></table></div></section>"##,block_title("job-decisions-heading","Choose the next action","route"),a(&ws["decisions"]).iter().map(|d|format!(r##"<tr><th scope="row" data-label="What you find">{}</th><td data-label="What to do">{}</td><td data-label="What to expect">{}</td></tr>"##,formatted(s(d,"symptom")),formatted(s(d,"action")),formatted(s(d,"expected")))).collect::<String>()));
    }
    html
}
fn command_block(command: &Value, job_id: &str, index: usize, quick: bool) -> String {
    let code = txt(&command["command"]);
    if code.is_empty() {
        return String::new();
    }
    format!(
        r##"<section class="{}" aria-label="{}"><div class="command-bar"><div><strong>{}</strong><span>{} · {}</span></div><button class="copy-command" data-copy-command="{}" data-command-index="{index}">{}Copy command</button></div><pre><code>{}</code></pre><p class="command-safety">{}<span>{}</span></p><p class="command-source">{}</p><p class="command-copy-note">Copy only. Read the matching procedure before execution.</p></section>"##,
        if quick {
            "job-quick-command"
        } else {
            "command-block"
        },
        if quick {
            "Quick command"
        } else {
            "Procedure command"
        },
        if quick {
            format!("Quick action · {}", esc(s(command, "title")))
        } else {
            String::new()
        },
        esc(fallback(s(command, "commandShell"), "Command")),
        formatted(s(command, "commandPurpose")),
        esc(job_id),
        icon("copy"),
        esc(&code),
        icon("info"),
        formatted(s(command, "commandSafety")),
        external(
            s(command, "commandUrl"),
            &format!(
                "Official reference: {} {}",
                esc(fallback(s(command, "commandName"), "Documentation")),
                icon("external")
            ),
            ""
        )
    )
}
fn shortcuts(app: &App, step: &Value) -> String {
    a(&step["shortcuts"])
        .iter()
        .map(|item| {
            if let Some(tool) = app.tool(s(item, "tool")) {
                format!(
                    r##"<a class="button ghost" href="#tool/{}">{}</a>{}"##,
                    encode(s(tool, "id")),
                    esc(s(item, "label")),
                    app.access_actions(tool, "", true)
                )
            } else if app.job(s(item, "job")).is_some() {
                format!(
                    r##"<a class="button ghost" href="#job/{}">{}</a>"##,
                    encode(s(item, "job")),
                    esc(s(item, "label"))
                )
            } else {
                external(s(item, "url"), &esc(s(item, "label")), "button ghost")
            }
        })
        .collect()
}
fn step_html(
    app: &App,
    job: &Value,
    step: &Value,
    index: usize,
    number: usize,
    quick: &str,
) -> String {
    let anchor = if s(step, "anchor").is_empty() {
        format!("step-{:02}", index + 1)
    } else {
        s(step, "anchor").into()
    };
    format!(
        r##"<section class="step" id="job-{}" data-step-anchor="{}"><div class="step-heading"><label class="step-label">{}<span><h3 tabindex="-1"><span class="step-number">{number}.</span>{}</h3></span></label><a class="step-permalink" href="{}" aria-label="Link to step {number}: {}">{}</a></div><div class="step-detail">{}</div>{}{}</section>"##,
        esc(&anchor),
        esc(&anchor),
        checkbox(app, job, "step", index, s(step, "title")),
        esc(s(step, "title")),
        section_href(job, quick, &anchor),
        esc(s(step, "title")),
        icon("link"),
        formatted(s(step, "detail")),
        if a(&step["shortcuts"]).is_empty() {
            String::new()
        } else {
            format!(
                r##"<div class="closeout-shortcuts" aria-label="Step shortcuts">{}</div>"##,
                shortcuts(app, step)
            )
        },
        command_block(step, s(job, "id"), index, false)
    )
}
fn specific_fixes(app: &App, job: &Value, quick: &str) -> String {
    let fixes = notes(app, s(job, "id"));
    if fixes.is_empty() {
        return String::new();
    }
    let mut html = format!(
        r##"<section class="field-notes" id="job-fixes"><header class="field-notes-header">{}<p>{} focused cases. Match the symptom, take the first action, and check the expected result.</p></header><nav class="field-note-jump" aria-label="Specific fixes in this guide">{}</nav>"##,
        block_title("job-fixes-heading", "Specific fixes", "wrench"),
        fixes.len(),
        fixes
            .iter()
            .map(|n| format!(
                r##"<a href="{}">{} {}</a>"##,
                section_href(job, quick, &format!("fix-{}", s(n, "id"))),
                esc(s(n, "title")),
                icon("arrow")
            ))
            .collect::<String>()
    );
    for note in fixes {
        let mut commands = String::new();
        for (index, c) in a(&note["commands"]).iter().enumerate() {
            commands.push_str(&format!(r##"<article class="field-note-command"><div class="field-note-command-heading"><div><strong>{}</strong><span>{} · {}</span></div><button class="copy-command" data-copy-field-command="{}" data-command-index="{index}">{}Copy</button></div><pre><code>{}</code></pre><p class="command-safety">{}<span>{}</span></p><p class="command-source">{}</p><p class="command-copy-note">Copy only. Inspect the result before continuing.</p></article>"##,esc(s(c,"label")),esc(s(c,"shell")),formatted(s(c,"purpose")),esc(s(note,"id")),icon("copy"),esc(s(c,"code")),icon("info"),formatted(s(c,"safety")),external(s(c,"sourceUrl"),&format!("Official reference: {} {}",esc(fallback(s(c,"sourceTitle"),"Documentation")),icon("external")),"")));
        }
        let tools=a(&note["toolIds"]).iter().filter_map(|id|app.tool(value(id))).map(|tool|{let authored=a(&workspace(app,job)["tools"]).iter().find(|e|s(e,"toolId")==s(tool,"id"));format!(r##"<div class="field-note-tool" data-field-tool="{}"><h4>{}</h4><div>{}</div></div>"##,esc(s(tool,"id")),app.tool_name(tool),tool_access(app,&ToolEntry{tool,action:authored.map(|e|s(e,"actionId")).unwrap_or("").into(),why:String::new(),choice:None}))}).collect::<String>();
        html.push_str(&format!(r##"<article class="field-note" id="job-fix-{}" data-field-note="{}" aria-labelledby="fix-heading-{}"><h3 id="fix-heading-{}" tabindex="-1"><a href="{}">{} {}</a></h3><p class="field-note-symptom"><strong>Symptom</strong> {}</p><p class="field-note-first"><strong>First action</strong> {}</p><ol class="field-note-steps">{}</ol><dl class="field-note-outcomes"><div><dt>Expected result</dt><dd>{}</dd></div><div><dt>If unresolved</dt><dd>{}</dd></div></dl>{}<section class="field-note-tools" aria-label="Tools for {}">{tools}</section><footer class="field-note-footer"><h4>Official sources</h4>{}<p>Reviewed <time datetime="{}">{}</time></p></footer></article>"##,esc(s(note,"id")),esc(s(note,"id")),esc(s(note,"id")),esc(s(note,"id")),section_href(job,quick,&format!("fix-{}",s(note,"id"))),esc(s(note,"title")),icon("link"),formatted(s(note,"symptom")),formatted(s(note,"firstAction")),a(&note["steps"]).iter().map(|step|format!("<li>{}</li>",formatted(&txt(step)))).collect::<String>(),formatted(s(note,"expected")),formatted(s(note,"ifUnresolved")),if commands.is_empty(){String::new()}else{format!(r##"<section class="field-note-commands" aria-label="Copy-ready commands for {}"><h4>{}Quick commands</h4>{commands}</section>"##,esc(s(note,"title")),icon("terminal"))},esc(s(note,"title")),source_list(&note["sourceUrls"]),esc(fallback(s(note,"reviewedAt"),s(&app.data["fieldNotes"],"reviewedAt"))),esc(fallback(s(note,"reviewedAt"),s(&app.data["fieldNotes"],"reviewedAt")))));
    }
    html.push_str("</section>");
    html
}
pub fn selection_notes(app: &App, items: &[&Value], title: &str) -> String {
    if items.is_empty() {
        return String::new();
    }
    let rows=items.iter().map(|item|{
        let details=[("Why this choice","why"),("Use and cost","freeUse"),("Before you act","boundary"),("Selection basis","qualityBasis")].iter().map(|(label,key)|format!("<div><dt>{label}</dt><dd>{}</dd></div>",formatted(s(item,key)))).collect::<String>();
        let choices=crate::catalog::normalized_choices(app,item).iter().filter_map(|choice|app.tool(s(choice,"toolId")).map(|tool|(choice,tool))).map(|(choice,tool)|format!(r##"<article class="selection-choice" data-choice-notes="{}"><h4>{}<span> · {}</span></h4><dl><div><dt>Best fit</dt><dd>{}</dd></div><div><dt>Use and cost</dt><dd>{}</dd></div></dl>{}{}<p class="sources-note">Reviewed {}.</p></article>"##,esc(s(tool,"id")),app.tool_name(tool),formatted(s(choice,"role")),formatted(s(choice,"why")),formatted(s(choice,"freeUse")),app.access_actions(tool,s(choice,"actionId"),true),source_list(&choice["evidenceUrls"]),esc(s(choice,"reviewedAt")))).collect::<String>();
        format!(r##"<article class="selection-note" data-selection-note="{}"><h3><a href="{}">{}</a></h3><dl>{details}</dl>{}<div class="selection-choices">{choices}</div></article>"##,esc(s(item,"id")),quick_href(item),esc(s(item,"label")),source_list(&item["evidenceUrls"]))
    }).collect::<String>();
    format!(
        r##"<section class="selection-notes"><h2 class="block-heading">{}{}</h2>{rows}</section>"##,
        icon("info"),
        esc(title)
    )
}
fn job_detail(app: &App, route: &Route) -> String {
    let Some(job) = app.job(&route.id) else {
        return empty(
            "Playbook not found",
            "Choose a guide from the complete index.",
            "#playbooks",
            "Open playbooks",
        );
    };
    if !route.quick.is_empty()
        && !quicks(app, s(job, "id"))
            .iter()
            .any(|q| s(q, "id") == route.quick)
    {
        return empty(
            "Scenario not found",
            "The complete procedure remains available.",
            &format!("#job/{}", encode(s(job, "id"))),
            "Open complete guide",
        );
    }
    let ws = workspace(app, job);
    let steps = a(&job["steps"]);
    let verifies = a(&job["verify"]);
    let quick = &route.quick;
    let stop_items: BTreeSet<_> = a(&job["stopIf"])
        .iter()
        .chain(a(&ws["escalateWhen"]))
        .map(txt)
        .collect();
    let stops = if stop_items.is_empty() {
        String::new()
    } else {
        format!(
            r##"<section class="job-stop-conditions" id="job-escalate">{}<ul class="stop-list">{}</ul></section>"##,
            block_title("job-escalate-heading", "Pause & escalate when", "warning"),
            stop_items
                .iter()
                .map(|s| format!("<li>{}</li>", formatted(s)))
                .collect::<String>()
        )
    };
    let quick_command = steps
        .iter()
        .enumerate()
        .find(|(_, step)| !txt(&step["command"]).is_empty())
        .map(|(i, step)| command_block(step, s(job, "id"), i, true))
        .unwrap_or_default();
    let checks = verifies
        .iter()
        .enumerate()
        .map(|(i, item)| {
            format!(
                r##"<label class="verify-item">{}<span>{}</span></label>"##,
                checkbox(app, job, "verify", i, &txt(item)),
                formatted(&txt(item))
            )
        })
        .collect::<String>();
    let related = a(&job["related"])
        .iter()
        .filter_map(|id| app.job(value(id)))
        .map(job_card)
        .collect::<String>();
    format!(
        r##"<div class="job-page" data-job-workspace="{}"><a class="back-link" href="#playbooks">{}All job playbooks</a><div class="job-work-layout"><header class="detail-header job-header"><div class="page-eyebrow">{}{} / Playbook</div><h1>{}</h1><p class="summary">{}</p><div class="detail-meta"><span class="pill">{}{}</span><span class="pill">{}{}</span><span class="pill">{}{} steps · {} verification checks</span></div><div class="detail-actions"><button class="button primary" data-export-job="{}">{}Export checklist</button><button class="button ghost" data-print>{}Print guide</button><button class="button ghost" data-copy-link>{}Copy link</button></div></header>{}{}<div class="detail-main job-guide-main">{}{quick_command}{}{stops}<div class="callout job-original-start">{}<div><h3>Start here</h3><p>{}</p></div></div><section class="aside-card job-progress-inline" id="job-progress">{}</section><section id="job-procedure">{}<div class="checklist">{}</div></section><section id="job-verify">{}<div class="verify-list">{checks}</div>{}</section>{}{}{}{}<section class="job-handoff"><h3>Leave a useful handoff</h3><p>Record the symptom, findings, changes and verification in your team's private ticket. Keep passwords, customer files and recovery keys out of the public wiki.</p></section></div></div></div>"##,
        esc(s(job, "id")),
        icon("back"),
        icon(job_icon(job)),
        esc(s(job, "category")),
        esc(s(job, "title")),
        formatted(s(job, "summary")),
        icon("clock"),
        esc(fallback(
            s(job, "estimatedTime"),
            "Work at the pace the job requires"
        )),
        icon("layers"),
        esc(fallback(s(job, "difficulty"), "Technician guide")),
        icon("checkCircle"),
        steps.len(),
        verifies.len(),
        esc(s(job, "id")),
        icon("download"),
        icon("printer"),
        icon("link"),
        job_navigation(app, job, quick),
        tool_panel(app, job, quick),
        quick_answers(app, job, quick),
        working_plan(ws),
        icon("compass"),
        formatted(fallback(s(job, "startHere"), s(job, "summary"))),
        progress_card(app, job, false),
        block_title("job-procedure-heading", "Work through the job", "route"),
        steps
            .iter()
            .enumerate()
            .map(|(i, step)| step_html(app, job, step, i, i + 1, quick))
            .collect::<String>(),
        block_title("job-verify-heading", "Verify the result", "checkCircle"),
        if a(&ws["doneWhen"]).is_empty() {
            String::new()
        } else {
            format!(
                "<div class=\"workspace-done\"><h3>Ready to close when</h3>{}</div>",
                list(&ws["doneWhen"], "bullet-list")
            )
        },
        specific_fixes(app, job, quick),
        selection_notes(app, &quicks(app, s(job, "id")), "Tool choice and use notes"),
        if a(&job["sources"]).is_empty() {
            String::new()
        } else {
            format!(
                r##"<section id="job-references">{}{}<p class="sources-note">Reviewed {}. Follow the vendor's current instructions when the product or environment differs.</p></section>"##,
                block_title("job-references-heading", "Official references", "book"),
                source_list(&job["sources"]),
                esc(s(job, "reviewedAt"))
            )
        },
        if related.is_empty() {
            String::new()
        } else {
            format!("<h2 class=\"block-heading\">{}Keep going</h2><div class=\"detail-related\">{related}</div>",icon("route"))
        }
    )
}
fn checklist(app: &App) -> String {
    let Some(job) = app.job("client-handoff") else {
        return empty(
            "Checklist unavailable",
            "Open a full job guide for its checks.",
            "#playbooks",
            "Open playbooks",
        );
    };
    heading(
        "INTA fieldwork",
        "Technician checklist",
        "One pass from protecting the data to handing the computer back.",
    ) + &format!(
        r##"<div class="detail-actions"><button class="button primary" data-export-job="client-handoff">{}Export checklist</button><button class="button ghost" data-print>{}Print</button><a class="button ghost" href="#job/client-handoff">Full handoff guide</a></div><div class="detail-layout"><div><div class="callout">{}<div><h3>Protect data before routine maintenance</h3><p>If a drive may be failing, recover first. Isolate a suspected active compromise before routine patching or optimization.</p></div></div><div class="checklist closeout-rows">{}</div></div><aside class="detail-aside"><section class="aside-card" id="job-progress">{}</section><section class="aside-card"><h3>Before closing</h3><p>Mark steps after you verify them. Record deferred work in the private ticket. Never mark an unresolved critical issue as passed.</p>{}</section></aside></div>"##,
        icon("download"),
        icon("printer"),
        icon("warning"),
        closeout_rows(job)
            .iter()
            .enumerate()
            .map(|(number, (index, step))| step_html(app, job, step, *index, number + 1, ""))
            .collect::<String>(),
        progress_card(app, job, true),
        external(
            &format!("{REPO}/wiki/Job-client-handoff"),
            "Open the Wiki guide",
            "access-link"
        )
    )
}
fn services(app: &App) -> String {
    let groups: Vec<_> = a(&app.data["layout"]["groups"])
        .iter()
        .filter(|g| g["jobs"].is_array())
        .collect();
    if groups.is_empty() {
        return empty(
            "Service map unavailable",
            "The complete job playbooks are available.",
            "#playbooks",
            "Browse job playbooks",
        );
    }
    let jobs: BTreeSet<_> = groups
        .iter()
        .flat_map(|g| a(&g["jobs"]))
        .filter_map(|id| app.job(value(id)))
        .map(|j| s(j, "id"))
        .collect();
    let jumps=groups.iter().map(|g|format!(r##"<a href="#services?area={}"><span aria-hidden="true">{}</span>{}<span class="service-area-count">{}</span></a>"##,encode(s(g,"id")),esc(s(g,"icon")),esc(fallback(s(g,"navigationLabel"),s(g,"title"))),a(&g["jobs"]).iter().filter(|id|app.job(value(id)).is_some()).count())).collect::<String>();
    let areas=groups.iter().map(|g|{
        let entries:Vec<_>=a(&g["jobs"]).iter().filter_map(|id|app.job(value(id))).collect();
        let count:usize=entries.iter().map(|j|notes(app,s(j,"id")).len()).sum();
        let rows=entries.iter().map(|job|{
            let picks=quicks(app,s(job,"id"));let pick=picks.first().copied();let presentation=&app.data["layout"]["jobs"][s(job,"id")];let fixes=notes(app,s(job,"id"));let title=fallback(s(presentation,"navigationLabel"),pick.map(|q|s(q,"label")).unwrap_or(s(job,"title")));
            let fixlinks=if fixes.is_empty(){String::new()}else{format!(r##"<nav class="service-note-links" aria-label="Specific fixes for {}"><span>Specific fixes</span>{}</nav>"##,esc(title),fixes.iter().map(|n|format!(r##"<a data-service-fix="{}" href="{}">{}</a>"##,esc(s(n,"id")),section_href(job,"",&format!("fix-{}",s(n,"id"))),esc(s(n,"title")))).collect::<String>())};
            format!(r##"<article class="service-job-row" role="listitem" data-service-job="{}"><div><h3><a href="#job/{}"><span aria-hidden="true">{}</span>{}</a></h3><p>{}</p><span class="service-job-coverage">{} tools · {} resources · {}</span>{fixlinks}</div><nav class="service-job-actions" aria-label="Open {}">{}{}<a href="#job/{}">Complete guide {}</a></nav></article>"##,esc(s(job,"id")),encode(s(job,"id")),esc(fallback(s(presentation,"navigationIcon"),s(presentation,"icon"))),esc(title),formatted(pick.map(|q|s(q,"firstStep")).unwrap_or(fallback(s(job,"startHere"),s(job,"summary")))),tool_entries(app,job,"").len(),a(&workspace(app,job)["resources"]).len(),esc(s(job,"estimatedTime")),esc(title),pick.map(|q|format!(r##"<a href="{}">First three actions {}</a>"##,quick_href(q),icon("arrow"))).unwrap_or_default(),if fixes.is_empty(){String::new()}else{format!(r##"<a href="{}">{} specific fixes {}</a>"##,section_href(job,"","fixes"),fixes.len(),icon("arrow"))},encode(s(job,"id")),icon("book"))
        }).collect::<String>();
        format!(r##"<section class="service-area" id="service-{}" data-service-area="{}" aria-labelledby="service-heading-{}"><header class="service-area-header"><div><h2 id="service-heading-{}" tabindex="-1"><span aria-hidden="true">{}</span>{}</h2><p>{}</p></div><span>{} guides · {count} specific fixes</span></header><div class="service-job-list" role="list">{rows}</div></section>"##,esc(s(g,"id")),esc(s(g,"id")),esc(s(g,"id")),esc(s(g,"id")),esc(s(g,"icon")),esc(s(g,"title")),formatted(s(g,"summary")),entries.len())
    }).collect::<String>();
    heading(
        "INTA service coverage",
        "Service map",
        "Choose the service area, then open a job guide or specific fix.",
    ) + &format!(
        r##"<div class="service-map-overview"><p>{} service areas · {} job guides · {} specific fixes</p>{}</div><nav class="service-area-jump" aria-label="Jump to a service area">{jumps}</nav><div class="service-map">{areas}</div>"##,
        groups.len(),
        jobs.len(),
        a(&app.data["fieldNotes"]["notes"]).len(),
        external(
            &format!("{REPO}/wiki/Service-Map"),
            &format!("Wiki service map {}", icon("external")),
            ""
        )
    )
}
const KNOWLEDGE:&[(&str,&str,&str,&str,&str)]=&[
("Online-Tool-Review","activity","Online tool review","Current service changes, practical selections and evidence from the November 2026 research.","Read the research"),
("#online","globe","Online tools","Open source-reviewed file, URL, network, email and website analysis services.","Browse online tools"),
("#services","route","Service map","Browse every service area, job guide and specific fix in one place.","Browse service areas"),
("Decision-Desk","compass","Decision desk","Start with the symptom. Follow the decision paths to the right playbook and tool.","Choose a path"),
("Methods","book","Methods library","Learn repeatable techniques behind diagnosis, repair and verified recovery.","Explore methods"),
("Knowledge-Standards","layers","Knowledge standards","Use one clear structure for evidence, links, diagrams, review dates and guidance.","Read the standards"),
("Knowledge-Organization","folder","Knowledge organization","See where guidance belongs and how source records stay connected.","Organization guide"),
("Knowledge-Health","activity","Knowledge health","Inspect coverage, open gaps and validation results for the knowledge base.","View knowledge health"),
("Research-Queue","search","Research queue","Find proposed improvements and evidence needed before they are recommended.","Review research queue"),
("Templates","file","Service templates","Turn a service visit, tool evaluation or proven fix into a complete record.","Choose a template"),
("Contribute","github","Contribute a fix","Bring a source, explain the improvement and help the next technician reproduce the result.","Contribution guide"),
("Continuous-Improvement","refresh","Continuous improvement","Review changes, close gaps in a method and use lessons from real jobs.","Review the process"),
("Interface-Design","grid","Interface design","Read layout, typography, interaction and accessibility standards.","Design standards"),
("Tool-Selection","filter","Tool selection","Compare tool fit, evidence and operating requirements before use.","Selection guide"),
("Current-Tool-Choices","activity","Current tool choices","Check current tool reviews, practical alternatives and recommendation reasons.","Review current choices"),
("Service-Evidence","checkCircle","Service evidence","Keep findings, changes and verification connected in a useful service record.","Evidence guide")];
fn reference_link(slug: &str, html: &str, class: &str) -> String {
    if slug.starts_with('#') {
        format!(
            r##"<a href="{}" class="{}">{html}</a>"##,
            esc(slug),
            esc(class)
        )
    } else {
        external(&format!("{REPO}/wiki/{slug}"), html, class)
    }
}
pub fn knowledge_desk(compact: bool) -> String {
    if compact {
        return format!(
            r##"<section class="home-knowledge"><h2>Knowledge desk</h2><nav aria-label="Knowledge desk references">{}</nav></section>"##,
            KNOWLEDGE
                .iter()
                .map(|(slug, kind, title, _, _)| reference_link(
                    slug,
                    &format!("{}<span>{}</span>", icon(kind), esc(title)),
                    ""
                ))
                .collect::<String>()
        );
    }
    format!(r##"<div class="knowledge-list" role="list">{}</div>"##,KNOWLEDGE.iter().map(|(slug,kind,title,body,label)|format!(r##"<article class="knowledge-row" role="listitem"><span class="reference-icon">{}</span><div><h2>{}</h2><p>{}</p></div>{}</article>"##,icon(kind),reference_link(slug,&esc(title),""),formatted(body),reference_link(slug,&format!("{} {}",esc(label),icon(if slug.starts_with('#'){"arrow"}else{"external"})),"row-action"))).collect::<String>())
}
fn knowledge() -> String {
    heading(
        "INTA field guide",
        "Knowledge desk",
        "Decision paths, methods, standards and templates for repeatable service work.",
    ) + &knowledge_desk(false)
        + r##"<section class="document-section"><h2>From service work to shared guidance</h2><ol class="process-list"><li><strong>Choose the path.</strong> Start at the decision desk and name the problem.</li><li><strong>Follow the method.</strong> Use the playbook, primary references and a measured next step.</li><li><strong>Verify the result.</strong> Reproduce the original task and record the outcome privately.</li><li><strong>Improve the guide.</strong> Share the reusable lesson with a source and clear verification.</li></ol></section>"##
}
fn official_sources(app: &App) -> String {
    let mut unique = BTreeMap::new();
    for source in app.jobs().iter().flat_map(|job| a(&job["sources"])).chain(
        a(&app.data["onlineTools"]["tools"])
            .iter()
            .flat_map(|tool| a(&tool["sources"])),
    ) {
        if safe_url(s(source, "url")) {
            unique.insert(s(source, "url"), source);
        }
    }
    let mut all: Vec<_> = unique.values().copied().collect();
    all.sort_by_key(|source| fallback(s(source, "title"), s(source, "url")).to_lowercase());
    let found: Vec<_> = all
        .iter()
        .filter(|source| {
            matches(
                &format!("{} {}", s(source, "title"), s(source, "url")),
                app.route.get("q"),
            )
        })
        .collect();
    let rows = if found.is_empty() {
        "<p class=\"prose\">No references match. Try another search.</p>".into()
    } else {
        found
            .iter()
            .map(|source| {
                let url = s(source, "url");
                let host = url
                    .split("://")
                    .nth(1)
                    .unwrap_or(url)
                    .split('/')
                    .next()
                    .unwrap_or(url);
                format!(
                    r##"<div class="source-list">{}</div>"##,
                    external(
                        url,
                        &format!(
                            r##"<span>{}<span class="source-domain">{}</span></span>{}"##,
                            esc(fallback(s(source, "title"), url)),
                            esc(host),
                            icon("external")
                        ),
                        ""
                    )
                )
            })
            .collect::<String>()
    };
    format!(
        r##"<section class="section-space"><div class="section-title"><div><h2>Official reference desk</h2><p>The primary references used by the job playbooks and online tools.</p></div></div><label class="input-wrap">{}<span class="sr-only">Filter official references</span><input type="search" id="source-search" value="{}" placeholder="Find a vendor, document or reference…"></label><div class="filter-result"><span id="source-count" aria-live="polite">{} of {} official references</span></div><div class="source-directory" id="source-directory">{rows}</div></section>"##,
        icon("search"),
        esc(app.route.get("q")),
        found.len(),
        all.len()
    )
}
fn resources(app: &App) -> String {
    let entries = [
        (
            "layers",
            "The INTA wiki",
            "Browse the complete field guide, follow primary sources and propose a correction.",
            format!("{REPO}/wiki"),
            "Open the wiki",
        ),
        (
            "usb",
            "Rescue & boot media",
            "Find releases and media tools for systems that need recovery.",
            format!("{REPO}/releases"),
            "View releases",
        ),
        (
            "book",
            "Athena research",
            "Explore the research behind specialist additions to the toolkit.",
            "https://github.com/Herbertofury/Athena".into(),
            "Explore Athena",
        ),
        (
            "github",
            "Source repository",
            "Source code, catalog, guides and rescue tooling.",
            REPO.into(),
            "Open the repository",
        ),
        (
            "activity",
            "Build & publication checks",
            "Review the actual results and artifacts for each build.",
            format!("{REPO}/actions"),
            "View workflow runs",
        ),
    ];
    let rows=entries.iter().map(|(kind,title,body,url,label)|format!(r##"<article class="resource-row"><span class="reference-icon">{}</span><div><h2>{}</h2><p>{}</p></div>{}</article>"##,icon(kind),external(url,&esc(title),""),formatted(body),external(url,&format!("{} {}",esc(label),icon("external")),"row-action"))).collect::<String>();
    heading(
        "INTA ecosystem",
        "Resources & wiki",
        "Project references, source history, releases and a complete offline copy.",
    ) + &format!(
        r##"<div class="resource-list">{rows}<article class="resource-row"><span class="reference-icon">{}</span><div><h2>Offline workspace</h2><p>Save the complete catalog, procedures and tools. Publisher downloads and external references need a connection.</p></div><button class="button primary" data-download-offline>{}Save offline workspace</button></article></div><section class="document-section"><h2>One shared field guide</h2><p>{} tools and {} complete job guides are included in this edition.</p><nav class="home-actions"><a href="#tools">Browse tools</a><a href="#playbooks">Open playbooks</a><a href="#services">Service map</a></nav></section>"##,
        icon("download"),
        icon("download"),
        app.tools().len(),
        app.jobs().len()
    ) + &format!(
        r##"<section class="resource-note"><h2>Current sources, honest expectations</h2><p>The source review date records when this collection was checked; it does not mean every program was installed or tested on every system. Use the official page to check the current release and compatibility.</p><p><strong>Everyday picks</strong> are practical first choices. <strong>Specialist</strong> tools serve a focused need. <strong>Evaluate first</strong> entries need local testing before routine use.</p><p>Favorites and checklists stay in this browser. Keep client records in your team's private ticketing system. {}</p></section>"##,
        if app.storage_available {
            "Export a checklist when you need a portable handoff."
        } else {
            "Browser storage is unavailable; export a checklist to preserve it."
        }
    ) + &official_sources(app)
}
pub fn render(app: &App, route: &Route) -> String {
    match route.name.as_str() {
        "playbooks" => playbooks(app, route),
        "job" => job_detail(app, route),
        "checklist" => checklist(app),
        "services" => services(app),
        "knowledge" => knowledge(),
        "resources" => resources(app),
        _ => empty(
            "Page not found",
            "Choose a guide in the workspace.",
            "#playbooks",
            "Open playbooks",
        ),
    }
}

fn markdown_tool(app: &App, tool: &Value, action: &str) -> String {
    let access = &app.data["toolAccess"][s(tool, "id")];
    let mut out = format!(
        "### {}\n\n[Official page]({})\n",
        s(tool, "name"),
        fallback(s(access, "officialUrl"), s(tool, "homepage"))
    );
    let mut options: Vec<&Value> = Vec::new();
    if access["primary"].is_object() {
        options.push(&access["primary"]);
    }
    options.extend(a(&access["alternates"]));
    if !action.is_empty() {
        options.sort_by_key(|o| s(o, "id") != action);
    }
    for o in options {
        if safe_url(s(o, "url")) {
            out.push_str(&format!("- [{}]({})\n", s(o, "label"), s(o, "url")));
        }
    }
    out.push('\n');
    out
}
fn markdown_step(app: &App, job: &Value, step: &Value, index: usize, number: usize) -> String {
    let mut out = format!(
        "- [{}] {}. {}\n\n{}\n\n",
        if checked(app, job, "step", index, s(step, "title")) {
            "x"
        } else {
            " "
        },
        number,
        s(step, "title"),
        s(step, "detail")
    );
    for item in a(&step["shortcuts"]) {
        if let Some(tool) = app.tool(s(item, "tool")) {
            out.push_str(&markdown_tool(app, tool, ""));
        } else {
            let href = if !s(item, "job").is_empty() {
                format!("{REPO}/wiki/Job-{}", s(item, "job"))
            } else {
                s(item, "url").into()
            };
            if safe_url(&href) {
                out.push_str(&format!("- [{}]({href})\n", s(item, "label")));
            }
        }
    }
    if !txt(&step["command"]).is_empty() {
        out.push_str(&format!(
            "\n{} · {}\n\n```text\n{}\n```\n\n{}\n\n[Official command reference]({})\n\n",
            s(step, "commandShell"),
            s(step, "commandPurpose"),
            txt(&step["command"]),
            s(step, "commandSafety"),
            s(step, "commandUrl")
        ));
    }
    out
}
pub fn export_job(app: &App, id: &str) -> String {
    let Some(job) = app.job(id) else {
        return String::new();
    };
    let focus = app.route.name == "checklist" && id == "client-handoff";
    let (done, total) = progress(app, job, focus);
    let mut out = format!(
        "# INTA · {}\n\n{}\n\nSource review: {}\nChecklist: {done}/{total} checks\n\n",
        if focus {
            "Technician checklist"
        } else {
            s(job, "title")
        },
        s(job, "summary"),
        s(job, "reviewedAt")
    );
    if focus {
        out.push_str("## Protect data before routine maintenance\n\nIf a drive may be failing, recover first. Isolate suspected active compromise before routine patching or optimization.\n\n## Work through the checklist\n\n");
        for (n, (i, step)) in closeout_rows(job).iter().enumerate() {
            out.push_str(&markdown_step(app, job, step, *i, n + 1));
        }
        out.push_str(&format!("## Before closing\n\nRecord deferred work privately. Never mark an unresolved critical issue as passed.\n\n[Full handoff guide]({REPO}/wiki/Job-{id})\n"));
        return out;
    }
    let ws = workspace(app, job);
    out.push_str(&format!(
        "## Working plan\n\n{}\n\n{}\n\n## Tools and resources for this job\n\n",
        s(ws, "goal"),
        s(ws, "startHere")
    ));
    for entry in tool_entries(app, job, &app.route.quick) {
        out.push_str(&markdown_tool(app, entry.tool, &entry.action));
        out.push_str(&format!("{}\n\n", entry.why));
    }
    for r in a(&ws["resources"]) {
        out.push_str(&format!(
            "- [{}]({}) — {}\n  - {} · {} · Checked {}\n",
            s(r, "label"),
            s(r, "url"),
            s(r, "description"),
            resource_group(s(r, "group")),
            resource_group(s(r, "kind")),
            s(r, "checkedAt")
        ));
        if safe_url(s(r, "sourceUrl")) {
            out.push_str(&format!("  - [Source]({})\n", s(r, "sourceUrl")));
        }
    }
    out.push_str("\n## Choose the next action\n\n");
    for d in a(&ws["decisions"]) {
        out.push_str(&format!(
            "### {}\n\nAction: {}\n\nExpected: {}\n\n",
            s(d, "symptom"),
            s(d, "action"),
            s(d, "expected")
        ));
    }
    let mut picks = quicks(app, id);
    if let Some(i) = picks.iter().position(|q| s(q, "id") == app.route.quick) {
        let q = picks.remove(i);
        picks.insert(0, q);
    }
    if !picks.is_empty() {
        out.push_str("## First actions\n\n");
        for item in &picks {
            out.push_str(&format!(
                "### {}\n\n{}\n\n",
                s(item, "label"),
                s(item, "firstStep")
            ));
            for (i, step) in a(&item["quickSteps"]).iter().enumerate() {
                out.push_str(&format!("{}. {}\n", i + 1, txt(step)));
            }
            if item["guideOnly"] == true {
                out.push_str("\nNo extra download needed; tools are optional aids.\n");
            }
            for choice in a(&item["choices"]) {
                if let Some(tool) = app.tool(s(choice, "toolId")) {
                    out.push_str(&markdown_tool(app, tool, s(choice, "actionId")));
                    out.push_str(&format!(
                        "Role: {}\n\nBest fit: {}\n\nUse and cost: {}\n\n",
                        s(choice, "role"),
                        s(choice, "why"),
                        s(choice, "freeUse")
                    ));
                    for url in a(&choice["evidenceUrls"]) {
                        out.push_str(&format!("- {}\n", txt(url)));
                    }
                    out.push_str(&format!("Reviewed: {}\n\n", s(choice, "reviewedAt")));
                }
            }
        }
    }
    out.push_str(&format!(
        "## Start here\n\n{}\n\n## Work through the job\n\n",
        s(job, "startHere")
    ));
    for (i, step) in a(&job["steps"]).iter().enumerate() {
        out.push_str(&markdown_step(app, job, step, i, i + 1));
    }
    out.push_str("## Verify the result\n\n");
    for (i, item) in a(&job["verify"]).iter().enumerate() {
        out.push_str(&format!(
            "- [{}] {}\n",
            if checked(app, job, "verify", i, &txt(item)) {
                "x"
            } else {
                " "
            },
            txt(item)
        ));
    }
    for (label, items) in [
        ("Ready to close when", &ws["doneWhen"]),
        ("Pause and escalate when", &job["stopIf"]),
        ("Escalation evidence", &ws["escalateWhen"]),
    ] {
        if !a(items).is_empty() {
            out.push_str(&format!("\n## {label}\n\n"));
            for item in a(items) {
                out.push_str(&format!("- {}\n", txt(item)));
            }
        }
    }
    let fixes = notes(app, id);
    if !fixes.is_empty() {
        out.push_str("\n## Specific fixes\n\n");
        for note in fixes {
            out.push_str(&format!(
                "### {}\n\nSymptom: {}\n\nFirst action: {}\n\n",
                s(note, "title"),
                s(note, "symptom"),
                s(note, "firstAction")
            ));
            for (i, step) in a(&note["steps"]).iter().enumerate() {
                out.push_str(&format!("{}. {}\n", i + 1, txt(step)));
            }
            out.push_str(&format!(
                "\nExpected result: {}\n\nIf unresolved: {}\n\n",
                s(note, "expected"),
                s(note, "ifUnresolved")
            ));
            for c in a(&note["commands"]) {
                out.push_str(&format!("#### {}\n\n{} · {}\n\n```text\n{}\n```\n\n{}\n\n[Official command reference]({})\n\n",s(c,"label"),s(c,"shell"),s(c,"purpose"),s(c,"code"),s(c,"safety"),s(c,"sourceUrl")));
            }
            for tool_id in a(&note["toolIds"]) {
                if let Some(tool) = app.tool(value(tool_id)) {
                    out.push_str(&markdown_tool(app, tool, ""));
                }
            }
            out.push_str("Official sources:\n");
            for url in a(&note["sourceUrls"]) {
                out.push_str(&format!("- {}\n", txt(url)));
            }
            out.push_str(&format!(
                "\nReviewed: {}\n\n",
                fallback(
                    s(note, "reviewedAt"),
                    s(&app.data["fieldNotes"], "reviewedAt")
                )
            ));
        }
    }
    if !picks.is_empty() {
        out.push_str("\n## Selection notes\n\n");
        for item in picks {
            out.push_str(&format!("### {}\n\nWhy: {}\n\nUse and cost: {}\n\nBefore you act: {}\n\nSelection basis: {}\n\n",s(item,"label"),s(item,"why"),s(item,"freeUse"),s(item,"boundary"),s(item,"qualityBasis")));
            for url in a(&item["evidenceUrls"]) {
                out.push_str(&format!("- {}\n", txt(url)));
            }
        }
    }
    out.push_str("\n## Official references\n\n");
    for source in a(&job["sources"]) {
        out.push_str(&format!(
            "- [{}]({})\n",
            fallback(s(source, "title"), s(source, "url")),
            s(source, "url")
        ));
    }
    out.push_str("\nRecord customer-specific details in your private service ticket.\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    fn app() -> App {
        App::new(
            json!({"jobs":serde_json::from_str::<Value>(include_str!("../../../data/jobs.json")).unwrap(),"tools":serde_json::from_str::<Value>(include_str!("../../../data/tools.json")).unwrap(),"quickStart":serde_json::from_str::<Value>(include_str!("../../../data/quick-start.json")).unwrap(),"jobWorkspaces":serde_json::from_str::<Value>(include_str!("../../../data/job-workspaces.json")).unwrap(),"fieldNotes":serde_json::from_str::<Value>(include_str!("../../../data/field-notes.json")).unwrap(),"layout":serde_json::from_str::<Value>(include_str!("../../../data/wiki-layout.json")).unwrap(),"toolAccess":serde_json::from_str::<Value>(include_str!("../../../data/tool-access.json")).unwrap()["tools"].clone()}),
        )
    }
    #[test]
    fn checklist_identity_survives_insertions() {
        let old = json!({"steps":[{"title":"Check","anchor":"verify-boot"}]});
        let new = json!({"steps":[{"title":"Added","anchor":"new"},{"title":"Check","anchor":"verify-boot"}]});
        assert_eq!(
            checklist_key(&old, "step", 0, "Check"),
            checklist_key(&new, "step", 1, "Check")
        );
    }
    #[test]
    fn ambiguous_legacy_titles_are_never_migrated() {
        let mut app = App::new(
            json!({"jobs":[{"id":"j","steps":[{"title":"Check","anchor":"a"},{"title":"Check","anchor":"b"}]}]}),
        );
        app.checks = json!({"j":{"step:0:Check":true}});
        migrate_checks(&mut app);
        assert!(app.checks["j"]["step:anchor:a"].is_null());
        assert_eq!(app.checks["j"]["step:0:Check"], true);
    }
    #[test]
    fn unique_legacy_title_migrates_to_anchor() {
        let mut app =
            App::new(json!({"jobs":[{"id":"j","steps":[{"title":"Check","anchor":"a"}]}]}));
        app.checks = json!({"j":{"step:4:Check":true}});
        migrate_checks(&mut app);
        assert_eq!(app.checks["j"]["step:anchor:a"], true);
        assert!(app.checks["j"]["step:4:Check"].is_null());
    }
    #[test]
    fn every_guide_retains_steps_verification_fixes_and_resources() {
        let app = app();
        for job in app.jobs() {
            let route = Route {
                name: "job".into(),
                id: s(job, "id").into(),
                ..Default::default()
            };
            let html = render(&app, &route);
            for step in a(&job["steps"]) {
                assert!(
                    html.contains(&esc(s(step, "title"))),
                    "missing step {}",
                    s(step, "title")
                );
            }
            for r in a(&workspace(&app, job)["resources"]) {
                assert!(html.contains(&format!("data-workspace-resource=\"{}\"", s(r, "id"))));
            }
            for note in notes(&app, s(job, "id")) {
                assert!(html.contains(&format!("data-field-note=\"{}\"", s(note, "id"))));
            }
            assert_eq!(
                html.matches("data-check-type=\"verify\"").count(),
                a(&job["verify"]).len()
            );
        }
    }
    #[test]
    fn malware_case_keeps_authored_choices_and_original_action() {
        let app = app();
        let html = render(
            &app,
            &Route::parse("#job/malware-removal/quick/suspected-malware"),
        );
        for id in [
            "malwarebytes",
            "adwcleaner",
            "bitdefender-free",
            "safety-scanner",
        ] {
            assert!(
                html.contains(&format!("data-priority-tool=\"{id}\"")),
                "{id}"
            );
        }
    }
    #[test]
    fn scenario_specific_acquisition_survives_rendering() {
        let app = app();
        for item in a(&app.data["quickStart"]["items"]) {
            let route = Route::parse(&quick_href(item));
            let html = render(&app, &route);
            for choice in a(&item["choices"]) {
                let record = &app.data["toolAccess"][s(choice, "toolId")];
                let mut choices = vec![&record["primary"]];
                choices.extend(a(&record["alternates"]));
                if let Some(action) = choices.iter().find(|a| s(a, "id") == s(choice, "actionId")) {
                    assert!(
                        html.contains(&esc(s(action, "url"))),
                        "{} lost {}",
                        s(item, "id"),
                        s(choice, "actionId")
                    );
                }
            }
        }
    }
    #[test]
    fn exported_fixes_include_commands_and_evidence() {
        let app = app();
        let out = export_job(&app, "intake-triage");
        for note in notes(&app, "intake-triage") {
            for command in a(&note["commands"]) {
                assert!(out.contains(s(command, "code")));
                assert!(out.contains(s(command, "sourceUrl")));
            }
        }
    }
    #[test]
    fn focused_checklist_exports_current_progress() {
        let mut app = app();
        app.route = Route::parse("#checklist");
        let job = app.job("client-handoff").unwrap().clone();
        let key = checklist_key(&job, "step", 0, s(&job["steps"][0], "title"));
        app.checks = json!({"client-handoff":{key:true}});
        let export = export_job(&app, "client-handoff");
        assert!(export.contains("Checklist: 1/9 checks"));
        assert!(export.contains("- [x] 1. Protect data"));
        assert!(!export.contains("## Specific fixes"));
    }
}
