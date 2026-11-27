//! Read-only, bounded publisher metadata. No cookie jar, account session, article
//! bodies or media downloads are used. Partial failures preserve checked metadata.
use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, Utc};
use regex::Regex;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{Read, Write},
    path::Path,
    time::{Duration, Instant},
};
use url::Url;
pub type Result<T> = std::result::Result<T, String>;
pub const FEED_BYTES: usize = 1024 * 1024;
pub const PAGE_BYTES: usize = 384 * 1024;
pub const OUTPUT_BYTES: usize = 512 * 1024;
pub const MAX_ITEMS: usize = 16;
pub const MAX_MEDIA: usize = 3;
const ATOM: &str = "http://www.w3.org/2005/Atom";
const MEDIA: &str = "http://search.yahoo.com/mrss/";
const EMBED_HOSTS: &[&str] = &[
    "www.youtube.com",
    "youtube.com",
    "youtu.be",
    "www.youtube-nocookie.com",
];
#[derive(Clone, Debug)]
pub struct Source {
    pub id: String,
    pub feed: String,
    pub hosts: Vec<String>,
    pub article_prefix: Vec<String>,
    pub media_hosts: Vec<String>,
    pub byte_limit: usize,
}
fn source(
    id: &str,
    feed: &str,
    hosts: &[&str],
    prefix: &[&str],
    media: &[&str],
    byte_limit: usize,
) -> Source {
    Source {
        id: id.into(),
        feed: feed.into(),
        hosts: hosts.iter().map(|s| s.to_string()).collect(),
        article_prefix: prefix.iter().map(|s| s.to_string()).collect(),
        media_hosts: media.iter().map(|s| s.to_string()).collect(),
        byte_limit,
    }
}
pub fn sources() -> Vec<Source> {
    vec![
        source(
            "bleeping",
            "https://www.bleepingcomputer.com/feed/",
            &["www.bleepingcomputer.com"],
            &["/news/"],
            &["www.bleepstatic.com"],
            FEED_BYTES,
        ),
        source(
            "krebs",
            "https://krebsonsecurity.com/feed/",
            &["krebsonsecurity.com"],
            &[],
            &[],
            FEED_BYTES,
        ),
        source(
            "securityweek",
            "https://www.securityweek.com/feed/",
            &["www.securityweek.com"],
            &[],
            &[],
            FEED_BYTES,
        ),
        source(
            "github-status",
            "https://www.githubstatus.com/history.rss",
            &["www.githubstatus.com"],
            &[],
            &[],
            FEED_BYTES,
        ),
        source(
            "cert-fr",
            "https://cert.cyber.gouv.fr/feed/",
            &["cert.cyber.gouv.fr", "www.cert.cyber.gouv.fr"],
            &[],
            &[],
            FEED_BYTES,
        ),
        source(
            "sans-isc",
            "https://isc.sans.edu/dailypodcast.xml",
            &["isc.sans.edu"],
            &[],
            &["traffic.libsyn.com"],
            7 * FEED_BYTES,
        ),
    ]
}
pub fn seed_sources() -> Vec<Source> {
    vec![source(
        "windows-health",
        "",
        &[
            "learn.microsoft.com",
            "techcommunity.microsoft.com",
            "blogs.windows.com",
        ],
        &[],
        &[],
        FEED_BYTES,
    )]
}
pub fn safe_url(raw: &str, base: &str, hosts: &[String], canonical: bool) -> Option<String> {
    if raw.is_empty()
        || raw.len() > 2048
        || raw.chars().any(|c| c <= ' ' || c == '\u{7f}' || c == '\\')
    {
        return None;
    }
    // Inspect before URL normalization, which otherwise erases encoded dot segments.
    let raw_path = raw.split(['?', '#']).next()?;
    if Regex::new(r"(?i)%(?:00|0a|0d|2e|2f|5c)")
        .ok()?
        .is_match(raw_path)
    {
        return None;
    }
    if let Some(authority) = raw
        .strip_prefix("https://")
        .or_else(|| raw.strip_prefix("//"))
    {
        if authority
            .split(['/', '?', '#'])
            .next()?
            .contains([':', '@'])
        {
            return None;
        }
    }
    let mut url = Url::parse(raw)
        .or_else(|_| Url::parse(base).and_then(|b| b.join(raw)))
        .ok()?;
    if url.scheme() != "https"
        || !url.username().is_empty()
        || url.password().is_some()
        || url.port().is_some()
        || !hosts.iter().any(|h| Some(h.as_str()) == url.host_str())
    {
        return None;
    }
    if url.path().split('/').any(|s| s == "." || s == "..") {
        return None;
    }
    if canonical {
        let pairs: Vec<(String, String)> = url
            .query_pairs()
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect();
        if pairs.len() > 32 {
            return None;
        }
        url.set_query(None);
        let keep: Vec<_> = pairs
            .into_iter()
            .filter(|(k, _)| {
                let k = k.to_ascii_lowercase();
                !k.starts_with("utm_")
                    && !matches!(
                        k.as_str(),
                        "fbclid" | "gclid" | "mc_cid" | "mc_eid" | "ref" | "ref_src"
                    )
            })
            .collect();
        if !keep.is_empty() {
            url.query_pairs_mut().extend_pairs(keep);
        }
    }
    url.set_fragment(None);
    Some(url.to_string())
}
pub fn fetch(
    url: &str,
    hosts: &[String],
    limit: usize,
    deadline: Instant,
    page: bool,
) -> Result<(Vec<u8>, String)> {
    if safe_url(url, url, hosts, false).as_deref() != Some(url) {
        return Err("unapproved request URL".into());
    }
    let mut current = url.to_owned();
    for redirects in 0..=3 {
        let remaining = deadline
            .checked_duration_since(Instant::now())
            .ok_or("source time budget exhausted")?;
        let agent = ureq::AgentBuilder::new()
            .redirects(0)
            .try_proxy_from_env(true)
            .timeout(remaining.min(Duration::from_secs(7)))
            .build();
        let response = agent
            .get(&current)
            .set("User-Agent", "INTA-News/2.0 (public metadata reader)")
            .set(
                "Accept",
                if page {
                    "text/html"
                } else {
                    "application/atom+xml, application/rss+xml, application/xml, text/xml"
                },
            )
            .set("Accept-Encoding", "identity")
            .call()
            .map_err(|e| e.to_string())?;
        if (300..400).contains(&response.status()) {
            if redirects == 3 {
                return Err("redirect limit exceeded".into());
            }
            current = safe_url(
                response
                    .header("Location")
                    .ok_or("redirect without Location")?,
                &current,
                hosts,
                false,
            )
            .ok_or("redirect outside the approved HTTPS hosts")?;
            continue;
        }
        if response.status() != 200 {
            return Err("unexpected final response".into());
        }
        let mime = response
            .header("Content-Type")
            .unwrap_or("")
            .split(';')
            .next()
            .unwrap_or("")
            .trim()
            .to_ascii_lowercase();
        let allowed = if page {
            vec!["text/html", "application/xhtml+xml"]
        } else {
            vec![
                "application/atom+xml",
                "application/rss+xml",
                "application/xml",
                "text/xml",
                "application/rdf+xml",
            ]
        };
        if !allowed.contains(&mime.as_str())
            || !response
                .header("Content-Encoding")
                .unwrap_or("identity")
                .eq_ignore_ascii_case("identity")
        {
            return Err("unexpected response type or encoding".into());
        }
        if let Some(length) = response.header("Content-Length") {
            if length.is_empty()
                || !length.bytes().all(|b| b.is_ascii_digit())
                || length
                    .parse::<usize>()
                    .map_err(|_| "invalid content length")?
                    > limit
            {
                return Err("declared response too large".into());
            }
        }
        let mut bytes = Vec::new();
        let mut reader = response.into_reader();
        let mut block = [0u8; 16384];
        loop {
            if Instant::now() >= deadline {
                return Err("source time budget exhausted".into());
            }
            let n = reader.read(&mut block).map_err(|e| e.to_string())?;
            if n == 0 {
                break;
            }
            if bytes.len() + n > limit {
                return Err("response too large".into());
            }
            bytes.extend_from_slice(&block[..n]);
        }
        return Ok((bytes, current));
    }
    Err("redirect limit exceeded".into())
}
pub fn plain_title(raw: &str) -> String {
    let raw: String = raw.chars().take(4096).collect();
    let decoded = html_escape::decode_html_entities(&raw);
    let html = scraper::Html::parse_fragment(&decoded);
    let mut parts = Vec::new();
    for node in html.tree.root().descendants() {
        if let Some(text) = node.value().as_text() {
            if !node.ancestors().any(|p| {
                p.value()
                    .as_element()
                    .is_some_and(|e| matches!(e.name(), "script" | "style"))
            }) {
                parts.push(text.to_string());
            }
        }
    }
    parts
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .filter(|c| *c >= ' ' && *c != '\u{7f}')
        .take(220)
        .collect::<String>()
        .trim()
        .into()
}
pub fn item_date(raw: &str) -> Option<String> {
    if raw.is_empty() || raw.len() > 100 {
        return None;
    }
    let raw = raw.trim();
    let date = DateTime::parse_from_rfc3339(raw)
        .or_else(|_| DateTime::parse_from_rfc2822(raw))
        .ok()
        .map(|x| x.with_timezone(&Utc).date_naive())
        .or_else(|| NaiveDate::parse_from_str(raw, "%Y-%m-%d").ok())
        .or_else(|| {
            NaiveDateTime::parse_from_str(raw, "%Y-%m-%dT%H:%M:%S")
                .ok()
                .map(|x| x.date())
        })?;
    (date.year() >= 1990).then(|| date.format("%Y-%m-%d").to_string())
}
pub fn snapshot_time(raw: &str) -> Option<DateTime<Utc>> {
    if !Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d{1,6})?Z$")
        .ok()?
        .is_match(raw)
    {
        return None;
    }
    DateTime::parse_from_rfc3339(raw)
        .ok()
        .map(|x| x.with_timezone(&Utc))
}
pub fn embed_url(raw: &str) -> Option<String> {
    let u = Url::parse(raw).ok()?;
    if !EMBED_HOSTS.contains(&u.host_str()?) {
        return None;
    }
    let id = if u.path() == "/watch" {
        let pairs: Vec<_> = u.query_pairs().collect();
        if pairs.len() > 32 {
            return None;
        }
        pairs.iter().find(|(k, _)| k == "v")?.1.to_string()
    } else {
        u.path().rsplit('/').next()?.into()
    };
    if (u.path().starts_with("/embed/")
        || u.path().starts_with("/shorts/")
        || u.path() == "/watch"
        || u.host_str() == Some("youtu.be"))
        && Regex::new(r"^[A-Za-z0-9_-]{11}$").ok()?.is_match(&id)
    {
        Some(format!("https://www.youtube.com/watch?v={id}"))
    } else {
        None
    }
}
fn mime_types(kind: &str) -> &'static [&'static str] {
    match kind {
        "image" => &[
            "image/jpeg",
            "image/png",
            "image/webp",
            "image/gif",
            "image/avif",
        ],
        "audio" => &[
            "audio/mpeg",
            "audio/mp4",
            "audio/ogg",
            "audio/wav",
            "audio/x-wav",
            "audio/aac",
        ],
        "video" => &["video/mp4", "video/webm", "video/ogg"],
        _ => &[],
    }
}
fn extensions(kind: &str) -> &'static [&'static str] {
    match kind {
        "image" => &[".jpg", ".jpeg", ".png", ".webp", ".gif", ".avif"],
        "audio" => &[".mp3", ".m4a", ".wav", ".ogg", ".oga", ".aac"],
        "video" => &[".mp4", ".webm", ".ogv"],
        _ => &[],
    }
}
pub fn pick_media(source: &Source, rows: &[(String, String, String)], base: &str) -> Vec<Value> {
    let mut result = Vec::new();
    let mut seen = BTreeSet::new();
    let embed_hosts: Vec<_> = EMBED_HOSTS.iter().map(|s| s.to_string()).collect();
    let hosts: Vec<_> = source
        .hosts
        .iter()
        .chain(&source.media_hosts)
        .cloned()
        .collect();
    for (k, href, mime) in rows {
        let outbound = safe_url(href, base, &embed_hosts, false)
            .and_then(|u| embed_url(&u))
            .filter(|_| matches!(k.as_str(), "video" | "audio" | "embed" | ""));
        let (kind, url) = if let Some(url) = outbound {
            ("embed".to_string(), url)
        } else {
            let Some(url) = safe_url(href, base, &hosts, false) else {
                continue;
            };
            let Ok(parsed) = Url::parse(&url) else {
                continue;
            };
            let path = parsed.path().to_ascii_lowercase();
            let mime = mime
                .split(';')
                .next()
                .unwrap_or("")
                .trim()
                .to_ascii_lowercase();
            if parsed.host_str() == Some("traffic.libsyn.com")
                && !Regex::new(r"^/securitypodcast/\d+\.mp3$")
                    .unwrap()
                    .is_match(&path)
            {
                continue;
            }
            if [".svg", ".svgz", ".html", ".htm", ".php", ".js", ".xml"]
                .iter()
                .any(|x| path.ends_with(x))
            {
                continue;
            }
            let kind = if mime_types(k).is_empty() {
                ["image", "audio", "video"]
                    .into_iter()
                    .find(|c| {
                        mime_types(c).contains(&mime.as_str())
                            || extensions(c).iter().any(|x| path.ends_with(x))
                    })
                    .unwrap_or("")
                    .to_string()
            } else {
                k.clone()
            };
            if kind.is_empty()
                || !(mime_types(&kind).contains(&mime.as_str())
                    || (mime.is_empty() && extensions(&kind).iter().any(|x| path.ends_with(x))))
            {
                continue;
            }
            (kind, url)
        };
        if seen.insert(url.clone()) {
            result.push(json!({"kind":kind,"url":url}));
        }
        if result.len() == MAX_MEDIA {
            break;
        }
    }
    result
}
fn text(node: roxmltree::Node<'_, '_>, names: &[(&str, &str)]) -> String {
    for (ns, name) in names {
        if let Some(c) = node.children().find(|c| {
            c.is_element()
                && c.tag_name().name() == *name
                && c.tag_name().namespace().unwrap_or("") == *ns
        }) {
            let text = c
                .descendants()
                .filter_map(|n| n.is_text().then(|| n.text()).flatten())
                .collect::<String>();
            if !text.trim().is_empty() {
                return text.trim().into();
            }
        }
    }
    String::new()
}
fn field<'a>(v: &'a Value, k: &str) -> &'a str {
    v.get(k).and_then(Value::as_str).unwrap_or("")
}
fn items<'a>(v: &'a Value, k: &str) -> &'a [Value] {
    v.get(k)
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or(&[])
}
fn ident(source: &str, url: &str) -> String {
    format!("{:x}", Sha256::digest(format!("{source}|{url}")))[..24].to_string()
}
pub fn promotional(source: &Source, categories: &[String], url: &str, title: &str) -> bool {
    if source.id != "bleeping" {
        return false;
    }
    let path = Url::parse(url)
        .map(|u| u.path().to_ascii_lowercase())
        .unwrap_or_default();
    path.starts_with("/news/deals/")
        || path.starts_with("/deals/")
        || categories.iter().any(|c| {
            matches!(
                c.trim().to_ascii_lowercase().as_str(),
                "deals" | "sponsored" | "sponsored content"
            )
        })
        || (Regex::new(r"(?i)\bblack friday\b").unwrap().is_match(title)
            && Regex::new(r"(?i)(?:\b(?:deals?|discount|saving|sale|shop)\b|\$\d)")
                .unwrap()
                .is_match(title))
}
pub fn parse_feed(source: &Source, data: &[u8], base: &str) -> Result<Vec<Value>> {
    if data.len() > source.byte_limit {
        return Err("oversized feed".into());
    }
    let xml = std::str::from_utf8(data)
        .map_err(|e| e.to_string())?
        .trim_start_matches('\u{feff}');
    if Regex::new(r"(?i)<!\s*(?:DOCTYPE|ENTITY)")
        .unwrap()
        .is_match(xml)
    {
        return Err("DTD and entity declarations are not permitted".into());
    }
    let document = roxmltree::Document::parse(xml).map_err(|e| e.to_string())?;
    let root = document.root_element();
    let atom = root.has_tag_name((ATOM, "feed"));
    if !atom && !root.has_tag_name("rss") {
        return Err("unrecognized feed root".into());
    }
    let nodes: Vec<_> = if atom {
        root.children()
            .filter(|n| n.has_tag_name((ATOM, "entry")))
            .collect()
    } else {
        root.children()
            .filter(|n| n.has_tag_name("channel"))
            .flat_map(|n| n.children().filter(|c| c.has_tag_name("item")))
            .collect()
    };
    let mut seen = BTreeSet::new();
    let mut result = Vec::new();
    for node in nodes.into_iter().take(96) {
        if result.len() >= MAX_ITEMS {
            break;
        }
        let href = if atom {
            node.children()
                .find(|n| {
                    n.has_tag_name((ATOM, "link"))
                        && n.attribute("rel").unwrap_or("alternate") == "alternate"
                        && matches!(
                            n.attribute("type").unwrap_or("text/html"),
                            "text/html" | "application/xhtml+xml"
                        )
                })
                .and_then(|n| n.attribute("href"))
                .unwrap_or("")
                .to_string()
        } else {
            text(node, &[("", "link")])
        };
        let Some(url) = safe_url(&href, base, &source.hosts, true) else {
            continue;
        };
        let title = plain_title(&text(
            node,
            if atom {
                &[(ATOM, "title")]
            } else {
                &[("", "title")]
            },
        ));
        let Some(date) = item_date(&text(
            node,
            if atom {
                &[(ATOM, "published"), (ATOM, "updated")]
            } else {
                &[
                    ("", "pubDate"),
                    ("http://purl.org/dc/elements/1.1/", "date"),
                    ("", "date"),
                ]
            },
        )) else {
            continue;
        };
        let cats: Vec<_> = node
            .children()
            .filter(|n| n.has_tag_name("category"))
            .map(|n| n.text().unwrap_or("").to_string())
            .collect();
        if title.is_empty() || seen.contains(&url) || promotional(source, &cats, &url, &title) {
            continue;
        }
        seen.insert(url.clone());
        let rows: Vec<_> = node
            .descendants()
            .filter_map(|c| {
                let (kind, href, mime) = if c.has_tag_name((MEDIA, "thumbnail")) {
                    ("image", c.attribute("url"), "")
                } else if c.has_tag_name((MEDIA, "content")) {
                    (
                        c.attribute("medium").unwrap_or(""),
                        c.attribute("url"),
                        c.attribute("type").unwrap_or(""),
                    )
                } else if c.has_tag_name((MEDIA, "player")) {
                    ("embed", c.attribute("url"), "")
                } else if c.has_tag_name("enclosure") {
                    ("", c.attribute("url"), c.attribute("type").unwrap_or(""))
                } else if atom
                    && c.has_tag_name((ATOM, "link"))
                    && c.attribute("rel") == Some("enclosure")
                {
                    ("", c.attribute("href"), c.attribute("type").unwrap_or(""))
                } else {
                    return None;
                };
                Some((kind.into(), href.unwrap_or("").into(), mime.into()))
            })
            .take(24)
            .collect();
        result.push(json!({"id":ident(&source.id,&url),"sourceId":source.id,"title":title,"url":url,"date":date,"media":pick_media(source,&rows,&url)}));
    }
    Ok(result)
}
pub fn head_metadata(html: &str) -> Vec<(String, String, String)> {
    let lower = html.to_ascii_lowercase();
    let end = [lower.find("</head"), lower.find("<body")]
        .into_iter()
        .flatten()
        .min()
        .unwrap_or(html.len());
    let document = scraper::Html::parse_document(&html[..end]);
    let selector = scraper::Selector::parse("meta").unwrap();
    document
        .select(&selector)
        .filter_map(|m| {
            let key = m
                .value()
                .attr("property")
                .or_else(|| m.value().attr("name"))
                .unwrap_or("")
                .to_ascii_lowercase();
            let kind = match key.as_str() {
                "og:image" | "og:image:url" | "og:image:secure_url" | "twitter:image" => "image",
                "og:video" | "og:video:url" | "og:video:secure_url" => "video",
                "twitter:player" => "embed",
                "og:audio" | "og:audio:url" | "og:audio:secure_url" => "audio",
                _ => return None,
            };
            Some((
                kind.into(),
                m.value().attr("content").unwrap_or("").into(),
                String::new(),
            ))
        })
        .take(24)
        .collect()
}
pub type Fetcher<'a> =
    dyn Fn(&str, &[String], usize, Instant, bool) -> Result<(Vec<u8>, String)> + Sync + 'a;
pub fn refresh_source(
    source: &Source,
    page_media: bool,
    fetcher: &Fetcher<'_>,
) -> (String, String, Vec<Value>) {
    let deadline = Instant::now() + Duration::from_secs(16);
    let Ok((data, end)) = fetcher(
        &source.feed,
        &source.hosts,
        source.byte_limit,
        deadline,
        false,
    ) else {
        return (source.id.clone(), "unavailable".into(), vec![]);
    };
    let Ok(mut entries) = parse_feed(source, &data, &end) else {
        return (source.id.clone(), "unavailable".into(), vec![]);
    };
    if page_media && !source.article_prefix.is_empty() {
        let mut count = 0;
        for item in &mut entries {
            let url = field(item, "url").to_string();
            let path = Url::parse(&url)
                .map(|u| u.path().to_string())
                .unwrap_or_default();
            if items(item, "media").len() >= MAX_MEDIA
                || !source.article_prefix.iter().any(|p| path.starts_with(p))
            {
                continue;
            }
            if count >= 2 || Instant::now() + Duration::from_millis(250) >= deadline {
                break;
            }
            count += 1;
            let Ok((html, end)) = fetcher(&url, &source.hosts, PAGE_BYTES, deadline, true) else {
                continue;
            };
            let end_path = Url::parse(&end)
                .map(|u| u.path().to_string())
                .unwrap_or_default();
            if !source
                .article_prefix
                .iter()
                .any(|p| end_path.starts_with(p))
            {
                continue;
            }
            let extra = pick_media(
                source,
                &head_metadata(&String::from_utf8_lossy(&html)),
                &end,
            );
            let media = item["media"].as_array_mut().unwrap();
            for m in extra {
                if !media.iter().any(|old| old["url"] == m["url"]) && media.len() < MAX_MEDIA {
                    media.push(m);
                }
            }
        }
    }
    (source.id.clone(), "ok".into(), entries)
}
pub fn normalize_saved_items(rows: &[Value], source: &Source) -> Vec<Value> {
    let mut result = Vec::new();
    let mut seen = BTreeSet::new();
    for saved in rows.iter().take(sources().len().max(1) * MAX_ITEMS) {
        if field(saved, "sourceId") != source.id {
            continue;
        }
        let Some(url) = safe_url(field(saved, "url"), &source.feed, &source.hosts, true) else {
            continue;
        };
        let title = plain_title(field(saved, "title"));
        let Some(date) = item_date(field(saved, "date")) else {
            continue;
        };
        if title.is_empty() || seen.contains(&url) || promotional(source, &[], &url, &title) {
            continue;
        }
        seen.insert(url.clone());
        let media_rows = items(saved, "media")
            .iter()
            .take(MAX_MEDIA)
            .filter(|m| m["kind"].is_string() && m["url"].is_string())
            .map(|m| {
                (
                    field(m, "kind").into(),
                    field(m, "url").into(),
                    String::new(),
                )
            })
            .collect::<Vec<_>>();
        result.push(json!({"id":ident(&source.id,&url),"sourceId":source.id,"title":title,"url":url,"date":date,"media":pick_media(source,&media_rows,&url)}));
        if result.len() >= MAX_ITEMS {
            break;
        }
    }
    result
}
pub fn read_bounded_json(path: &Path) -> Result<Value> {
    let mut data = Vec::new();
    fs::File::open(path)
        .map_err(|e| e.to_string())?
        .take((OUTPUT_BYTES + 1) as u64)
        .read_to_end(&mut data)
        .map_err(|e| e.to_string())?;
    if data.len() > OUTPUT_BYTES {
        return Err("local snapshot exceeds read budget".into());
    }
    serde_json::from_slice(&data).map_err(|e| e.to_string())
}
pub fn load_seed(path: &Path) -> Result<Vec<(String, String, Vec<Value>)>> {
    let data = read_bounded_json(path)?;
    let rows = data
        .get("items")
        .unwrap_or(&data)
        .as_array()
        .ok_or("editorial seed needs a short items list")?;
    if rows.len() > MAX_ITEMS {
        return Err("editorial seed needs a short items list".into());
    }
    let sources = seed_sources();
    if rows
        .iter()
        .any(|r| !sources.iter().any(|s| s.id == field(r, "sourceId")))
    {
        return Err("editorial seed has an unapproved publisher".into());
    }
    let output: Vec<_> = sources
        .iter()
        .map(|s| (s.id.clone(), "ok".into(), normalize_saved_items(rows, s)))
        .filter(|(_, _, r)| !r.is_empty())
        .collect();
    if output.iter().map(|(_, _, r)| r.len()).sum::<usize>() != rows.len() {
        return Err(
            "editorial seed entries must have unique approved article URLs, dates and titles"
                .into(),
        );
    }
    Ok(output)
}
pub fn refresh(
    output: &Path,
    sources: &[Source],
    page_media: bool,
    seed: Option<&Path>,
    fetcher: &Fetcher<'_>,
    now: DateTime<Utc>,
) -> Result<Value> {
    let curated = seed.map(load_seed).transpose()?.unwrap_or_default();
    let mut gathered = Vec::new();
    for batch in sources.chunks(4) {
        let rows = std::thread::scope(|scope| {
            let joins: Vec<_> = batch
                .iter()
                .map(|s| scope.spawn(move || refresh_source(s, page_media, fetcher)))
                .collect();
            joins
                .into_iter()
                .map(|j| j.join().map_err(|_| "source worker failed".to_string()))
                .collect::<Result<Vec<_>>>()
        })?;
        gathered.extend(rows);
    }
    if !gathered.iter().any(|(_, s, _)| s == "ok") {
        return Err("all public sources unavailable; previous output left unchanged".into());
    }
    let previous = read_bounded_json(output)
        .ok()
        .filter(|v| v["schemaVersion"] == 1)
        .unwrap_or(json!({}));
    let generated = now.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let previous_generated = snapshot_time(field(&previous, "generatedAt"));
    let current_generated = snapshot_time(&generated).unwrap();
    let mut prior_checks = BTreeMap::new();
    let mut seen = BTreeSet::new();
    if let Some(old) = previous_generated.filter(|old| *old <= current_generated) {
        for row in items(&previous, "sources") {
            let id = field(row, "id");
            if !sources.iter().any(|s| s.id == id) {
                continue;
            }
            if !seen.insert(id.to_string()) {
                prior_checks.remove(id);
                continue;
            }
            if row.get("origin").map_or(true, |v| v == "feed")
                && matches!(field(row, "status"), "ok" | "unavailable")
                && snapshot_time(field(row, "lastSuccessfulCheckAt")).is_some_and(|t| t <= old)
            {
                prior_checks.insert(
                    id.to_string(),
                    field(row, "lastSuccessfulCheckAt").to_string(),
                );
            }
        }
    }
    let mut states = Vec::new();
    let mut entries = Vec::new();
    for (id, status, mut rows) in gathered {
        let mut state = json!({"id":id,"status":status,"origin":"feed"});
        if status == "ok" {
            state["lastSuccessfulCheckAt"] = json!(generated);
        } else {
            let s = sources
                .iter()
                .find(|s| s.id == id)
                .ok_or("source disappeared")?;
            rows = normalize_saved_items(items(&previous, "items"), s);
            if let Some(check) = prior_checks.get(&id) {
                state["lastSuccessfulCheckAt"] = json!(check);
            }
        }
        states.push(state);
        entries.extend(rows);
    }
    for (id, status, rows) in curated {
        states.push(json!({"id":id,"status":status,"origin":"editorial"}));
        entries.extend(rows);
    }
    entries.sort_by(|a, b| {
        (field(b, "date"), field(b, "id")).cmp(&(field(a, "date"), field(a, "id")))
    });
    let value = json!({"schemaVersion":1,"generatedAt":generated,"items":entries,"sources":states});
    let mut bytes = serde_json::to_vec(&value).map_err(|e| e.to_string())?;
    bytes.push(b'\n');
    if bytes.len() > OUTPUT_BYTES {
        return Err("digest exceeds output budget".into());
    }
    let parent = output
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    let mut temp = tempfile::Builder::new()
        .prefix(".news-")
        .tempfile_in(parent)
        .map_err(|e| e.to_string())?;
    temp.write_all(&bytes).map_err(|e| e.to_string())?;
    temp.as_file().sync_all().map_err(|e| e.to_string())?;
    temp.persist(output).map_err(|e| e.to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(output, fs::Permissions::from_mode(0o644))
            .map_err(|e| e.to_string())?;
    }
    Ok(value)
}

pub fn validate_digest(digest: &Value, news: &Value) -> Result<()> {
    let error = || "Invalid News digest contract".to_string();
    let known: BTreeSet<_> = items(news, "sources")
        .iter()
        .map(|s| field(s, "id"))
        .collect();
    let policies: Vec<_> = sources().into_iter().chain(seed_sources()).collect();
    if digest["schemaVersion"] != 1
        || !digest["items"].is_array()
        || items(digest, "items").len() > 100
        || !digest["sources"].is_array()
        || items(digest, "sources").len() > known.len()
    {
        return Err(error());
    }
    let generated = snapshot_time(field(digest, "generatedAt")).ok_or_else(error)?;
    let mut statuses = BTreeSet::new();
    for row in items(digest, "sources") {
        let id = field(row, "id");
        let editorial = seed_sources().iter().any(|s| s.id == id);
        let origin = if editorial { "editorial" } else { "feed" };
        if !known.contains(id)
            || !policies.iter().any(|p| p.id == id)
            || !statuses.insert(id)
            || !matches!(field(row, "status"), "ok" | "unavailable")
            || row.get("origin").is_some_and(|o| o != origin)
            || (editorial && (row["status"] != "ok" || row.get("lastSuccessfulCheckAt").is_some()))
        {
            return Err(error());
        }
        if let Some(v) = row.get("lastSuccessfulCheckAt") {
            let check = snapshot_time(v.as_str().unwrap_or("")).ok_or_else(error)?;
            if check > generated || (row["status"] == "ok" && check != generated) {
                return Err(error());
            }
        }
    }
    let mut ids = BTreeSet::new();
    let mut links = BTreeSet::new();
    for item in items(digest, "items") {
        let id = field(item, "id");
        let title = field(item, "title");
        let date = field(item, "date");
        if !statuses.contains(field(item, "sourceId"))
            || !Regex::new(r"^[A-Za-z0-9_-]{1,96}$").unwrap().is_match(id)
            || !ids.insert(id)
            || title.trim().is_empty()
            || title.chars().count() > 300
            || item_date(date).as_deref() != Some(date)
            || date > &field(digest, "generatedAt")[..10]
            || !links.insert(field(item, "url"))
            || !item["media"].is_array()
            || items(item, "media").len() > 4
        {
            return Err(error());
        }
        let policy = policies
            .iter()
            .find(|p| p.id == field(item, "sourceId"))
            .ok_or_else(error)?;
        if normalize_saved_items(std::slice::from_ref(item), policy) != [item.clone()] {
            return Err(error());
        }
    }
    Ok(())
}
pub mod upstream;

pub fn validate_news(catalog: &Value) -> Result<()> {
    fn public_https(value: &str) -> bool {
        !value.is_empty()
            && !value.chars().any(char::is_whitespace)
            && Url::parse(value).is_ok_and(|u| {
                u.scheme() == "https"
                    && u.host_str().is_some()
                    && u.username().is_empty()
                    && u.password().is_none()
            })
    }
    let date = field(catalog, "reviewedAt");
    if catalog["schemaVersion"] != 1
        || item_date(date).as_deref() != Some(date)
        || items(catalog, "sources").is_empty()
    {
        return Err("News requires a versioned and dated source catalog".into());
    }
    let mut ids = BTreeSet::new();
    let mut links = BTreeSet::new();
    let id_pattern = Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap();
    for source in items(catalog, "sources") {
        if !id_pattern.is_match(field(source, "id"))
            || !ids.insert(field(source, "id"))
            || !links.insert(field(source, "url"))
            || !public_https(field(source, "url"))
            || !matches!(
                field(source, "category"),
                "advisory" | "compliance" | "vendor" | "reporting" | "community" | "status"
            )
            || field(source, "name").trim().is_empty()
            || field(source, "description").trim().is_empty()
            || source
                .get("feed")
                .is_some_and(|v| !public_https(v.as_str().unwrap_or("")))
        {
            return Err("Invalid or duplicate News source".into());
        }
    }
    for item in items(catalog, "briefing") {
        let date = field(item, "date");
        if !ids.contains(field(item, "sourceId"))
            || !public_https(field(item, "url"))
            || item_date(date).as_deref() != Some(date)
            || date > field(catalog, "reviewedAt")
            || field(item, "title").is_empty()
            || field(item, "description").is_empty()
        {
            return Err("Invalid News briefing item".into());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
