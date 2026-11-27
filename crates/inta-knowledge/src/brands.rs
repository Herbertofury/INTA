use crate::{array, escape, fold, read_json, text, RAW_ASSETS};
use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD, Engine};
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};
use url::Url;

pub fn load_registry(root: &Path) -> Result<Value> {
    read_json(&root.join("data/brand-icons.json"))
}
pub fn url_key(value: &str) -> String {
    if let Ok(url) = Url::parse(value) {
        let host = match url.port() {
            Some(port) => format!("{}:{port}", url.host_str().unwrap_or("")),
            None => url.host_str().unwrap_or("").into(),
        };
        format!(
            "{}://{}{}{}",
            url.scheme().to_lowercase(),
            host.to_lowercase(),
            url.path().trim_end_matches('/'),
            url.query().map(|q| format!("?{q}")).unwrap_or_default()
        )
    } else {
        value.into()
    }
}
pub struct BrandIndex<'a> {
    tools: BTreeMap<String, &'a Value>,
    names: BTreeMap<String, &'a Value>,
    urls: BTreeMap<String, &'a Value>,
    hosts: BTreeMap<String, &'a Value>,
}
impl<'a> BrandIndex<'a> {
    pub fn new(registry: &'a Value) -> Self {
        let mut result = Self {
            tools: BTreeMap::new(),
            names: BTreeMap::new(),
            urls: BTreeMap::new(),
            hosts: BTreeMap::new(),
        };
        for brand in array(&registry["brands"]) {
            if !text(brand, "toolId").is_empty() {
                result
                    .tools
                    .entry(text(brand, "toolId").into())
                    .or_insert(brand);
            }
            for name in array(&brand["names"]).iter().filter_map(Value::as_str) {
                result.names.entry(fold(name.trim())).or_insert(brand);
            }
            for url in array(&brand["urls"]).iter().filter_map(Value::as_str) {
                let key = url_key(url);
                if !result.urls.contains_key(&key)
                    || (text(brand, "kind") == "tool" && text(result.urls[&key], "kind") != "tool")
                {
                    result.urls.insert(key, brand);
                }
            }
            for host in array(&brand["hosts"]).iter().filter_map(Value::as_str) {
                if !result.hosts.contains_key(host)
                    || (text(brand, "kind") == "website"
                        && text(result.hosts[host], "kind") != "website")
                {
                    result.hosts.insert(host.into(), brand);
                }
            }
        }
        result
    }
    pub fn resolve(
        &self,
        name: Option<&str>,
        url: Option<&str>,
        tool_id: Option<&str>,
    ) -> Option<&'a Value> {
        if let Some(brand) = tool_id.and_then(|id| self.tools.get(id)) {
            return Some(brand);
        }
        if let Some(brand) = name.and_then(|name| self.names.get(&fold(name.trim()))) {
            return Some(brand);
        }
        if let Some(url) = url {
            if let Some(brand) = self.urls.get(&url_key(url)) {
                return Some(brand);
            }
            if let Ok(url) = Url::parse(url) {
                return url
                    .host_str()
                    .and_then(|host| self.hosts.get(&host.to_lowercase()))
                    .copied();
            }
        }
        None
    }
}
pub fn resolve_brand<'a>(
    registry: &'a Value,
    name: Option<&str>,
    url: Option<&str>,
    tool_id: Option<&str>,
) -> Option<&'a Value> {
    BrandIndex::new(registry).resolve(name, url, tool_id)
}
pub fn asset_data_url(registry: &Value, asset_id: &str, root: &Path) -> Result<String> {
    let asset = &registry["assets"][asset_id];
    let bytes = std::fs::read(root.join(text(asset, "path")))
        .with_context(|| format!("Read brand asset {asset_id}"))?;
    Ok(format!(
        "data:{};base64,{}",
        text(asset, "mimeType"),
        STANDARD.encode(bytes)
    ))
}
pub fn portal_icons(registry: &Value, root: &Path) -> Result<Value> {
    let mut assets = Map::new();
    let mut urls = Map::new();
    if let Some(records) = registry["assets"].as_object() {
        for (id, asset) in records {
            assets.insert(id.clone(), asset_data_url(registry, id, root)?.into());
            urls.insert(
                id.clone(),
                format!("{RAW_ASSETS}{}", text(asset, "path")).into(),
            );
        }
    }
    Ok(
        json!({"assets":assets,"assetUrls":urls,"brands":registry["brands"],"checkedAt":registry["checkedAt"]}),
    )
}
pub fn icon_html(
    brand: Option<&Value>,
    registry: &Value,
    embedded: bool,
    size: u32,
    root: &Path,
    css_class: &str,
) -> Result<String> {
    let Some(brand) = brand.filter(|b| !text(b, "assetId").is_empty()) else {
        return Ok(String::new());
    };
    let asset_id = text(brand, "assetId");
    let src = if embedded {
        asset_data_url(registry, asset_id, root)?
    } else {
        format!(
            "{RAW_ASSETS}{}",
            text(&registry["assets"][asset_id], "path")
        )
    };
    Ok(format!("<img class=\"{}\" src=\"{}\" width=\"{size}\" height=\"{size}\" alt=\"\" aria-hidden=\"true\" data-brand-id=\"{}\">",escape(css_class),escape(&src),escape(text(brand,"id"))))
}
pub fn validate_registry(registry: &Value, root: &Path) -> Vec<String> {
    let mut errors = Vec::new();
    let brands = array(&registry["brands"]);
    let ids = brands
        .iter()
        .map(|b| text(b, "id"))
        .collect::<BTreeSet<_>>();
    if ids.len() != brands.len() {
        errors.push("Duplicate brand identity".into());
    }
    for brand in brands {
        let id = text(brand, "id");
        if registry["assets"].get(text(brand, "assetId")).is_none() {
            errors.push(format!("{id}: missing authentic cached icon"));
        }
        for url in std::iter::once(text(brand, "downloadPage"))
            .chain(array(&brand["urls"]).iter().filter_map(Value::as_str))
        {
            if !Url::parse(url).is_ok_and(|p| {
                p.scheme() == "https"
                    && p.host_str().is_some()
                    && p.username().is_empty()
                    && p.password().is_none()
            }) {
                errors.push(format!("{id}: unsafe brand URL"));
            }
        }
    }
    if let Some(assets) = registry["assets"].as_object() {
        for (id, asset) in assets {
            let path = root.join(text(asset, "path"));
            let Ok(bytes) = std::fs::read(&path) else {
                errors.push(format!("{id}: missing asset file"));
                continue;
            };
            if format!("{:x}", Sha256::digest(&bytes)) != text(asset, "sha256") {
                errors.push(format!("{id}: asset hash mismatch"));
            }
            if !bytes.starts_with(b"\x89PNG\r\n\x1a\n") || text(asset, "mimeType") != "image/png" {
                errors.push(format!("{id}: non-PNG cached image"));
            }
            if !["width", "height"]
                .iter()
                .all(|key| asset[*key].as_u64().is_some_and(|v| (8..=64).contains(&v)))
            {
                errors.push(format!("{id}: invalid thumbnail dimensions"));
            }
            if bytes.len() >= 24 && bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
                let width = u32::from_be_bytes(bytes[16..20].try_into().unwrap());
                let height = u32::from_be_bytes(bytes[20..24].try_into().unwrap());
                if asset["width"].as_u64() != Some(u64::from(width))
                    || asset["height"].as_u64() != Some(u64::from(height))
                {
                    errors.push(format!("{id}: PNG dimensions differ from provenance"));
                }
            }
            if !text(asset, "sourceUrl").starts_with("https://") {
                errors.push(format!("{id}: missing acquisition provenance"));
            }
        }
    }
    errors
}
