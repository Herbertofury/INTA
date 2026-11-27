use aho_corasick::{AhoCorasick, MatchKind};
use serde_json::Value;
use std::sync::OnceLock;
pub const REPO: &str = "https://github.com/Herbertofury/INTA";
pub fn s<'a>(v: &'a Value, key: &str) -> &'a str {
    v.get(key).and_then(Value::as_str).unwrap_or("")
}
pub fn a(v: &Value) -> &[Value] {
    v.as_array().map(Vec::as_slice).unwrap_or(&[])
}
pub fn txt(v: &Value) -> String {
    match v {
        Value::String(x) => x.clone(),
        Value::Array(x) => x.iter().map(txt).collect::<Vec<_>>().join("\n"),
        Value::Null => String::new(),
        _ => v.to_string(),
    }
}
pub fn esc(v: &str) -> String {
    v.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
pub fn encode(v: &str) -> String {
    urlencoding::encode(v).into_owned()
}
pub fn decode(v: &str) -> String {
    urlencoding::decode(&v.replace('+', " "))
        .map(|v| v.into_owned())
        .unwrap_or_else(|_| v.into())
}
pub fn safe_url(v: &str) -> bool {
    (v.starts_with("https://") || v.starts_with("http://")) && !v.chars().any(char::is_control)
}
pub fn external(url: &str, html: &str, class: &str) -> String {
    if !safe_url(url) {
        return html.into();
    }
    format!(
        "<a href=\"{}\" class=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{}</a>",
        esc(url),
        esc(class),
        html
    )
}
struct Linker {
    matcher: AhoCorasick,
    entries: Vec<(String, String, bool)>,
}
static LINKS: OnceLock<Option<Linker>> = OnceLock::new();
pub fn init_links(values: &Value) {
    LINKS.get_or_init(|| {
        let mut entries: Vec<_> = a(values)
            .iter()
            .filter(|x| !s(x, "name").is_empty() && safe_url(s(x, "url")))
            .map(|x| {
                (
                    s(x, "name").to_string(),
                    s(x, "url").to_string(),
                    x["caseSensitive"] == true,
                )
            })
            .collect();
        entries.sort_by_key(|x| std::cmp::Reverse(x.0.len()));
        entries.dedup_by(|a, b| a.0.eq_ignore_ascii_case(&b.0));
        let matcher = AhoCorasick::builder()
            .ascii_case_insensitive(true)
            .match_kind(MatchKind::LeftmostLongest)
            .build(entries.iter().map(|x| &x.0))
            .ok()?;
        Some(Linker { matcher, entries })
    });
}
fn prose(value: &str) -> String {
    let Some(Some(linker)) = LINKS.get() else {
        return esc(value);
    };
    let mut result = String::new();
    let mut offset = 0;
    for m in linker.matcher.find_iter(value) {
        let (name, url, case) = &linker.entries[m.pattern().as_usize()];
        let before = value[..m.start()].chars().next_back();
        let after = value[m.end()..].chars().next();
        if before.is_some_and(|c| c.is_alphanumeric() || c == '_')
            || after.is_some_and(|c| c.is_alphanumeric() || c == '_')
            || (*case && &value[m.start()..m.end()] != name)
        {
            continue;
        }
        result.push_str(&esc(&value[offset..m.start()]));
        result.push_str(&external(
            url,
            &esc(&value[m.start()..m.end()]),
            "prose-link",
        ));
        offset = m.end();
    }
    result.push_str(&esc(&value[offset..]));
    result
}
pub fn formatted(value: &str) -> String {
    let mut html = String::new();
    let mut rest = value;
    while let Some(i) = rest.find('`') {
        html.push_str(&bold(&rest[..i]));
        let next = &rest[i + 1..];
        if let Some(j) = next.find('`') {
            html.push_str("<code>");
            html.push_str(&esc(&next[..j]));
            html.push_str("</code>");
            rest = &next[j + 1..];
        } else {
            html.push_str(&bold(&rest[i..]));
            return html;
        }
    }
    html.push_str(&bold(rest));
    html
}
fn bold(value: &str) -> String {
    let mut parts = value.split("**");
    let mut out = prose(parts.next().unwrap_or(""));
    let mut strong = true;
    for part in parts {
        out.push_str(if strong { "<strong>" } else { "</strong>" });
        out.push_str(&prose(part));
        strong = !strong;
    }
    if !strong {
        out.push_str("</strong>");
    }
    out
}
pub fn icon(name: &str) -> String {
    static ICONS: OnceLock<Value> = OnceLock::new();
    let icons = ICONS.get_or_init(|| {
        serde_json::from_str(include_str!("../../../portal/icons.json"))
            .expect("valid bundled icons")
    });
    let key = match name {
        "refresh-cw" => "refresh",
        "alert-triangle" | "shield-alert" => "warning",
        "package" => "layers",
        "plug" => "usb",
        _ => name,
    };
    format!("<svg viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.55\" stroke-linecap=\"round\" stroke-linejoin=\"round\" aria-hidden=\"true\">{}</svg>",icons[key].as_str().unwrap_or_else(||icons["wrench"].as_str().unwrap_or("")))
}
pub fn heading(eyebrow: &str, title: &str, subtitle: &str) -> String {
    format!("<header class=\"page-heading\"><div><p class=\"page-label\">{}</p><h1>{}</h1><p class=\"page-description\">{}</p></div></header>",esc(eyebrow),esc(title),formatted(subtitle))
}
pub fn source_list(sources: &Value) -> String {
    format!(
        "<ul class=\"source-list\">{}</ul>",
        a(sources)
            .iter()
            .filter_map(|x| {
                let url = if x.is_string() {
                    x.as_str().unwrap()
                } else {
                    s(x, "url")
                };
                if !safe_url(url) {
                    return None;
                }
                let label = if s(x, "title").is_empty() {
                    url
                } else {
                    s(x, "title")
                };
                Some(format!(
                    "<li>{}</li>",
                    external(url, &format!("{} {}", esc(label), icon("external")), "")
                ))
            })
            .collect::<String>()
    )
}
pub fn empty(title: &str, summary: &str, href: &str, label: &str) -> String {
    format!("<div class=\"empty-state\"><h2>{}</h2><p>{}</p><a class=\"button primary\" href=\"{}\">{} {}</a></div>",esc(title),esc(summary),esc(href),esc(label),icon("arrow"))
}
pub fn category(value: &str) -> &str {
    match value {
        "ai" => "AI workspace",
        "boot" => "Boot & rescue",
        "hardware" => "Hardware diagnostics",
        "malware" => "Malware removal",
        "network" => "Networking",
        "productivity" => "Technician essentials",
        "remote" => "Remote support",
        "security" => "Security & investigation",
        "storage" => "Storage & recovery",
        "windows" => "Windows repair",
        "online" => "Online tools",
        _ => value,
    }
}
pub fn tier(value: &str) -> &str {
    match value {
        "core" => "Everyday pick",
        "specialist" => "Specialist",
        "evaluate" => "Evaluate first",
        _ => "Reference",
    }
}
pub fn mode(value: &str) -> &str {
    match value {
        "built-in" => "Built into the system",
        "portable" => "Portable",
        "installed" => "Installed application",
        "web" => "Web application",
        "boot" => "Bootable media",
        _ => value,
    }
}
pub fn tier_pill(value: &str) -> String {
    format!(
        "<span class=\"recommendation {}\">{}</span>",
        esc(value),
        esc(tier(value))
    )
}
pub fn matches(haystack: &str, query: &str) -> bool {
    let lower = haystack.to_lowercase();
    query
        .to_lowercase()
        .split_whitespace()
        .all(|term| lower.contains(term))
}
pub fn command_search<'a>(commands: &'a Value, query: &str) -> Vec<&'a Value> {
    a(commands)
        .iter()
        .filter(|item| {
            matches(
                &[
                    "title",
                    "summary",
                    "code",
                    "safety",
                    "shell",
                    "sourceTitle",
                    "category",
                ]
                .iter()
                .map(|key| s(item, key))
                .collect::<Vec<_>>()
                .join(" "),
                query,
            )
        })
        .collect()
}
pub fn options(values: &[String], any: &str, selected: &str) -> String {
    format!(
        "<option value=\"\">{}</option>{}",
        esc(any),
        values
            .iter()
            .map(|v| format!(
                "<option value=\"{}\"{}>{}</option>",
                esc(v),
                if v == selected { " selected" } else { "" },
                esc(v)
            ))
            .collect::<String>()
    )
}
