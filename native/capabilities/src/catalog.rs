use crate::{Result, err, expect, invalid, registry};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

const TOOLS: &str = include_str!("../../../data/tools.json");
const ADDITIONAL: &str = include_str!("../../workbench/data/additional-tools.json");
const COVERAGE: &str = include_str!("../../../data/tool-cli-coverage.json");
const ACCESS: &str = include_str!("../../../data/tool-access.json");
pub(crate) fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 100
        && !id.starts_with('-')
        && !id.ends_with('-')
        && id
            .bytes()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-')
}
pub struct Catalog {
    rows: BTreeMap<String, Value>,
    public: BTreeSet<String>,
    coverage: Value,
    access: Value,
}
fn data(s: &str) -> Result<Value> {
    serde_json::from_str(s).map_err(|_| err("catalog", "Invalid embedded catalog."))
}
impl Catalog {
    pub fn load() -> Result<Self> {
        let mut rows = BTreeMap::new();
        let mut public = BTreeSet::new();
        for (source, canonical) in [(TOOLS, true), (ADDITIONAL, false)] {
            let items = data(source)?;
            let items = items
                .as_array()
                .ok_or_else(|| err("catalog", "Expected an embedded catalog array."))?;
            let mut seen = BTreeSet::new();
            for row in items {
                let id = row["id"]
                    .as_str()
                    .ok_or_else(|| err("catalog", "Missing catalog ID."))?;
                if !valid_id(id) || !seen.insert(id) || row["name"].as_str().is_none() {
                    return Err(err("catalog", "Invalid or duplicate catalog entry."));
                }
                if canonical {
                    public.insert(id.to_owned());
                }
                rows.entry(id.to_owned()).or_insert_with(|| row.clone());
            }
        }
        Ok(Self {
            rows,
            public,
            coverage: data(COVERAGE)?,
            access: data(ACCESS)?,
        })
    }
    pub fn len(&self) -> usize {
        self.rows.len()
    }
    pub fn search(&self, query: &str, limit: usize) -> Result<Value> {
        expect(
            query.len() <= 240 && !query.chars().any(char::is_control),
            "Query must be at most 240 bytes without control characters.",
        )?;
        expect(
            (1..=500).contains(&limit),
            "Limit must be from 1 through 500.",
        )?;
        let q = query.trim().to_lowercase();
        let terms: Vec<_> = q.split_whitespace().collect();
        let mut found = Vec::new();
        for (id, row) in &self.rows {
            let name = row["name"].as_str().unwrap_or("").to_lowercase();
            let mut corpus =
                format!("{} {} {} {}", id, name, row["category"], row["summary"]).to_lowercase();
            if let Some(tags) = row["tags"].as_array() {
                for tag in tags.iter().filter_map(Value::as_str) {
                    corpus.push(' ');
                    corpus.push_str(&tag.to_lowercase());
                }
            }
            if terms.iter().all(|t| corpus.contains(t)) {
                let priority = if id == &q || name == q {
                    0
                } else if !q.is_empty() && name.starts_with(&q) {
                    1
                } else {
                    2
                };
                found.push((priority, name, id, row));
            }
        }
        found.sort_by(|a, b| (a.0, &a.1, a.2).cmp(&(b.0, &b.1, b.2)));
        let items:Vec<_>=found.iter().take(limit).map(|(_,_,id,r)|json!({"id":id,"name":r["name"],"summary":r["summary"],"category":r["category"],"mode":r["mode"],"platforms":r["platforms"],"origin":self.origin(id)})).collect();
        Ok(json!({"total":found.len(),"returned":items.len(),"items":items}))
    }
    fn origin(&self, id: &str) -> &'static str {
        if self.public.contains(id) {
            "public-catalog"
        } else {
            "workstation-catalog"
        }
    }
    pub fn inspect(&self, id: &str) -> Result<Value> {
        if !valid_id(id) {
            return Err(invalid("Invalid catalog ID."));
        }
        let tool = self
            .rows
            .get(id)
            .ok_or_else(|| err("not-found", "Catalog ID not found."))?;
        let coverage = self.coverage["tools"]
            .get(id)
            .cloned()
            .unwrap_or_else(|| json!({"scope":"needs-tool-review","operations":{}}));
        Ok(
            json!({"tool":tool,"origin":self.origin(id),"coverage":coverage,"publisherAccess":self.access["tools"].get(id),
            "operations":registry::all().into_iter().filter(|v|v["catalogIds"].as_array().is_some_and(|a|a.iter().any(|v|v==id))).collect::<Vec<_>>()}),
        )
    }
}
