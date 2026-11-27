use crate::{array, fold, read_json, records, text, WIKI};
use anyhow::{ensure, Context, Result};
use regex::{Regex, RegexBuilder};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
    sync::LazyLock,
};
use url::Url;

static REFERENCE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^ {0,3}\[([^\]]+)\]:").unwrap());
static ANCHOR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?is)^<a\b[^>]*>.*?</a\s*>").unwrap());
static BARE_URL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^https?://[^\s<>]+").unwrap());

pub fn build_registry(root: &Path, tools: &[Value], jobs: &[Value]) -> Result<Vec<Value>> {
    let mut entries = BTreeMap::<String, Value>::new();
    fn add(
        entries: &mut BTreeMap<String, Value>,
        name: &str,
        url: &str,
        kind: &str,
        id: &str,
        case_sensitive: bool,
    ) -> Result<()> {
        let name = name.trim();
        if name.is_empty() {
            return Ok(());
        }
        let parsed =
            Url::parse(url).with_context(|| format!("Unsafe registry URL for {name}: {url}"))?;
        ensure!(
            parsed.scheme() == "https"
                && parsed.host_str().is_some()
                && parsed.username().is_empty()
                && parsed.password().is_none(),
            "Unsafe registry URL for {name}: {url}"
        );
        let query = parsed.query().unwrap_or("").to_lowercase();
        ensure!(
            !["utm_", "fbclid=", "gclid="]
                .iter()
                .any(|v| query.contains(v)),
            "Tracking registry URL for {name}"
        );
        let key = fold(name);
        if let Some(previous) = entries.get(&key) {
            ensure!(text(previous, "url") == url, "Ambiguous link alias: {name}");
            return Ok(());
        }
        let mut value = json!({"name":name,"url":url,"kind":kind,"id":id});
        if case_sensitive {
            value["caseSensitive"] = true.into();
        }
        entries.insert(key, value);
        Ok(())
    }
    for tool in tools {
        add(
            &mut entries,
            text(tool, "name"),
            text(tool, "homepage"),
            "tool",
            text(tool, "id"),
            ["Everything", "Autopsy", "Chainsaw"].contains(&text(tool, "name")),
        )?;
        for alias in array(&tool["aliases"]) {
            add(
                &mut entries,
                alias.as_str().unwrap_or(""),
                text(tool, "homepage"),
                "tool",
                text(tool, "id"),
                false,
            )?;
        }
    }
    for job in jobs {
        add(
            &mut entries,
            text(job, "title"),
            &format!("{WIKI}/Job-{}", text(job, "id")),
            "playbook",
            text(job, "id"),
            false,
        )?;
    }
    let methods = root.join("data/methods.json");
    if methods.exists() {
        for method in records(&read_json(&methods)?, "methods") {
            for name in std::iter::once(text(&method, "name"))
                .chain(array(&method["aliases"]).iter().filter_map(Value::as_str))
            {
                add(
                    &mut entries,
                    name,
                    text(&method, "url"),
                    "method",
                    text(&method, "id"),
                    method["caseSensitive"] == true,
                )?;
            }
        }
    }
    let aliases = root.join("data/link-aliases.json");
    if aliases.exists() {
        for item in records(&read_json(&aliases)?, "links") {
            let (url, kind, id) = if let Some(tool_id) = item["toolId"].as_str() {
                let tool = tools
                    .iter()
                    .find(|tool| text(tool, "id") == tool_id)
                    .with_context(|| format!("Unknown alias tool {tool_id}"))?;
                (text(tool, "homepage"), "tool", text(tool, "id"))
            } else {
                (
                    text(&item, "url"),
                    if text(&item, "kind").is_empty() {
                        "application"
                    } else {
                        text(&item, "kind")
                    },
                    text(&item, "id"),
                )
            };
            for name in std::iter::once(text(&item, "name"))
                .chain(array(&item["aliases"]).iter().filter_map(Value::as_str))
            {
                add(
                    &mut entries,
                    name,
                    url,
                    kind,
                    id,
                    item["caseSensitive"] == true,
                )?;
            }
        }
    }
    let brand_path = root.join("data/brand-icons.json");
    if brand_path.exists() {
        let brands = read_json(&brand_path)?;
        for brand in array(&brands["brands"]) {
            for name in std::iter::once(text(brand, "name"))
                .chain(array(&brand["names"]).iter().filter_map(Value::as_str))
            {
                let key = fold(name);
                if name.chars().count() < 2
                    || entries.contains_key(&key)
                    || [
                        "home",
                        "support",
                        "download",
                        "downloads",
                        "documentation",
                        "source",
                        "help",
                        "free",
                        "tools",
                        "media",
                    ]
                    .contains(&key.as_str())
                {
                    continue;
                }
                add(
                    &mut entries,
                    name,
                    text(brand, "downloadPage"),
                    if text(brand, "kind").is_empty() {
                        "website"
                    } else {
                        text(brand, "kind")
                    },
                    text(brand, "id"),
                    true,
                )?;
            }
        }
    }
    let mut registry = entries.into_values().collect::<Vec<_>>();
    registry.sort_by(|l, r| {
        text(r, "name")
            .chars()
            .count()
            .cmp(&text(l, "name").chars().count())
            .then_with(|| fold(text(l, "name")).cmp(&fold(text(r, "name"))))
    });
    Ok(registry)
}
pub fn portal_links(registry: &[Value]) -> Vec<Value> {
    registry.to_vec()
}
pub fn balanced(value: &str, start: usize, opening: u8, closing: u8) -> Option<usize> {
    let bytes = value.as_bytes();
    let mut depth = 0usize;
    let mut i = start;
    while i < bytes.len() {
        if bytes[i] == b'\\' {
            i += 2;
            continue;
        }
        if bytes[i] == opening {
            depth += 1
        } else if bytes[i] == closing {
            depth = depth.checked_sub(1)?;
            if depth == 0 {
                return Some(i + 1);
            }
        }
        i += 1;
    }
    None
}
pub fn protected_end(value: &str, index: usize, references: &BTreeSet<String>) -> Option<usize> {
    let tail = &value[index..];
    let bytes = tail.as_bytes();
    if tail.starts_with("<!--") {
        return Some(
            tail.find("-->")
                .map(|v| index + v + 3)
                .unwrap_or(value.len()),
        );
    }
    if bytes.first() == Some(&b'`') {
        let run = bytes.iter().take_while(|&&c| c == b'`').count();
        if let Some(end) = tail[run..].find(&tail[..run]) {
            return Some(index + run + end + run);
        }
    }
    if bytes.first() == Some(&b'<') {
        if let Some(m) = ANCHOR.find(tail) {
            return Some(index + m.end());
        }
        if let Some(end) = tail.find('>') {
            return Some(index + end + 1);
        }
    }
    if bytes.first() == Some(&b'h') {
        if let Some(url) = BARE_URL.find(tail) {
            return Some(index + url.end());
        }
    }
    let bracket = if tail.starts_with("![") {
        index + 1
    } else {
        index
    };
    if value.as_bytes().get(bracket) == Some(&b'[') {
        if let Some(end) = balanced(value, bracket, b'[', b']') {
            if value.as_bytes().get(end) == Some(&b'(') {
                return balanced(value, end, b'(', b')');
            }
            if value.as_bytes().get(end) == Some(&b'[') {
                return balanced(value, end, b'[', b']');
            }
            let label = fold(value[bracket + 1..end - 1].trim());
            if references.contains(&label) || value[bracket..].starts_with("[[") {
                return Some(end);
            }
        }
    }
    None
}
pub(crate) fn fence(line: &str) -> Option<(u8, usize, &str)> {
    let line = line
        .trim_end_matches(['\r', '\n'])
        .trim_start_matches([' ', '>', '\t']);
    let marker = *line.as_bytes().first()?;
    if marker != b'`' && marker != b'~' {
        return None;
    }
    let len = line.bytes().take_while(|c| *c == marker).count();
    (len >= 3).then_some((marker, len, &line[len..]))
}
pub(crate) fn rewrite_prose(text: &str, mut rewrite: impl FnMut(&str) -> String) -> String {
    let mut result = String::new();
    let mut prose = String::new();
    let mut fenced = None;
    fn flush(result: &mut String, prose: &mut String, rewrite: &mut impl FnMut(&str) -> String) {
        if !prose.is_empty() {
            result.push_str(&rewrite(prose));
            prose.clear();
        }
    }
    for line in text.split_inclusive('\n') {
        let marker = fence(line);
        if let Some((kind, length)) = fenced {
            result.push_str(line);
            if marker.is_some_and(|(c, l, tail)| c == kind && l >= length && tail.trim().is_empty())
            {
                fenced = None;
            }
        } else if let Some((kind, len, _)) = marker {
            flush(&mut result, &mut prose, &mut rewrite);
            fenced = Some((kind, len));
            result.push_str(line);
        } else if line.starts_with("    ") || line.starts_with('\t') || REFERENCE.is_match(line) {
            flush(&mut result, &mut prose, &mut rewrite);
            result.push_str(line);
        } else {
            prose.push_str(line);
        }
    }
    flush(&mut result, &mut prose, &mut rewrite);
    result
}
pub(crate) fn word(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}
pub(crate) fn boundary(text: &str, start: usize, end: usize) -> bool {
    !text[..start].chars().next_back().is_some_and(word)
        && !text[end..].chars().next().is_some_and(word)
}
pub struct Linker {
    registry: Vec<Value>,
    by_name: BTreeMap<String, usize>,
    matcher: Option<Regex>,
}
impl Linker {
    pub fn new(registry: &[Value]) -> Self {
        let mut registry = registry.to_vec();
        registry.sort_by_key(|item| std::cmp::Reverse(text(item, "name").chars().count()));
        let by_name = registry
            .iter()
            .enumerate()
            .map(|(i, item)| (fold(text(item, "name")), i))
            .collect();
        let pattern = registry
            .iter()
            .map(|item| regex::escape(text(item, "name")))
            .filter(|v| !v.is_empty())
            .collect::<Vec<_>>()
            .join("|");
        let matcher = if pattern.is_empty() {
            None
        } else {
            Some(
                RegexBuilder::new(&pattern)
                    .case_insensitive(true)
                    .size_limit(64 * 1024 * 1024)
                    .build()
                    .expect("escaped registry names form a valid pattern"),
            )
        };
        Self {
            registry,
            by_name,
            matcher,
        }
    }
    fn replace(&self, prose: &str) -> String {
        let Some(matcher) = &self.matcher else {
            return prose.into();
        };
        let mut out = String::new();
        let mut start = 0;
        for found in matcher.find_iter(prose) {
            if !boundary(prose, found.start(), found.end()) {
                continue;
            }
            let Some(index) = self.by_name.get(&fold(found.as_str())) else {
                continue;
            };
            let item = &self.registry[*index];
            if item["caseSensitive"] == true && found.as_str() != text(item, "name") {
                continue;
            }
            out.push_str(&prose[start..found.start()]);
            out.push_str(&format!("[{}]({})", found.as_str(), text(item, "url")));
            start = found.end();
        }
        out.push_str(&prose[start..]);
        out
    }
    fn inline(&self, prose: &str, references: &BTreeSet<String>) -> String {
        let mut out = String::new();
        let mut index = 0;
        let mut start = 0;
        while index < prose.len() {
            if let Some(end) = protected_end(prose, index, references) {
                out.push_str(&self.replace(&prose[start..index]));
                out.push_str(&prose[index..end]);
                index = end;
                start = end;
            } else {
                index += prose[index..].chars().next().unwrap().len_utf8();
            }
        }
        out.push_str(&self.replace(&prose[start..]));
        out
    }
    pub fn link(&self, text: &str) -> String {
        let references = REFERENCE.captures_iter(text).map(|c| fold(&c[1])).collect();
        rewrite_prose(text, |prose| self.inline(prose, &references))
    }
}
pub fn link_markdown(text: &str, registry: &[Value]) -> String {
    Linker::new(registry).link(text)
}
pub fn plain_mentions(text_value: &str, registry: &[Value]) -> Vec<String> {
    let linked = link_markdown(text_value, registry);
    if linked == text_value {
        return Vec::new();
    }
    registry
        .iter()
        .filter(|item| {
            let target = format!("]({})", text(item, "url"));
            linked.matches(&target).count() > text_value.matches(&target).count()
        })
        .map(|item| text(item, "name").to_string())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}
