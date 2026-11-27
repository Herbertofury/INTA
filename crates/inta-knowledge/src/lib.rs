//! Canonical knowledge maintenance, implemented in Rust.
//! Content checks are deterministic and never imply a live publisher review.
pub mod brands;
pub mod health;
pub mod links;
pub mod markdown;
pub mod wiki;

use anyhow::{Context, Result};
use serde_json::Value;
use std::path::Path;
use unicode_casefold::UnicodeCaseFold;
pub const WIKI: &str = "https://github.com/Herbertofury/INTA/wiki";
pub const RAW_ASSETS: &str = "https://raw.githubusercontent.com/Herbertofury/INTA/main/";
pub fn read_json(path: &Path) -> Result<Value> {
    serde_json::from_slice(
        &std::fs::read(path).with_context(|| format!("Read {}", path.display()))?,
    )
    .with_context(|| format!("Parse {}", path.display()))
}
pub fn records(value: &Value, key: &str) -> Vec<Value> {
    if let Some(rows) = value.as_array() {
        rows.clone()
    } else {
        array(&value[key]).to_vec()
    }
}
pub(crate) fn text<'a>(value: &'a Value, key: &str) -> &'a str {
    value.get(key).and_then(Value::as_str).unwrap_or("")
}
pub(crate) fn array(value: &Value) -> &[Value] {
    value.as_array().map(Vec::as_slice).unwrap_or(&[])
}
pub(crate) fn fold(value: &str) -> String {
    value.case_fold().collect()
}
pub(crate) fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}
