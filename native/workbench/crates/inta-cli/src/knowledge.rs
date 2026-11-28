use anyhow::{Context, Result, ensure};
use rusqlite::Connection;
use serde_json::{Value, json};
use std::collections::HashSet;

const INDEX: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/knowledge.sqlite3"));

const STOP_WORDS: &[&str] = &[
    "an", "and", "are", "as", "at", "be", "by", "can", "check", "checks", "do", "does", "for",
    "from", "has", "have", "help", "how", "in", "is", "it", "me", "my", "of", "on", "or", "should",
    "the", "their", "this", "to", "what", "when", "which", "with", "would", "why",
];

fn terms(question: &str) -> Result<String> {
    ensure!(
        !question.trim().is_empty() && question.len() <= 2048,
        "Use a question of 1–2048 UTF-8 bytes."
    );
    let mut tokens = Vec::new();
    for word in question
        .split(|c: char| !c.is_alphanumeric())
        .map(str::to_lowercase)
    {
        if word.len() < 2 || STOP_WORDS.contains(&word.as_str()) {
            continue;
        }
        if !tokens.contains(&word) {
            tokens.push(word);
        }
        if tokens.len() == 24 {
            break;
        }
    }
    Ok(tokens
        .into_iter()
        .map(|w| format!("\"{w}\""))
        .collect::<Vec<_>>()
        .join(" OR "))
}

pub fn search(question: &str) -> Result<Value> {
    ranked(question, None)
}

fn index() -> Result<Connection> {
    let mut con = Connection::open_in_memory()?;
    con.deserialize_bytes("main", INDEX)?;
    Ok(con)
}

pub fn context(source: &str, location: &str) -> Result<Value> {
    let con = index()?;
    let (document, anchor, digest): (i64, i64, String) = con
        .query_row(
            "SELECT s.rowid,p.rowid,s.sha256 FROM sources s JOIN passages p ON p.source=s.rowid
         WHERE s.kind IN ('runbook','wiki') AND s.path=?1 AND p.location=?2",
            [source, location],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )
        .context("This guide passage is absent from the installed index")?;
    // The build inserts each document in reading order. Restrict both directions to
    // this source so a passage at the edge cannot pull in unrelated instructions.
    let mut query = con.prepare(
        "SELECT section,passage,location FROM passages WHERE source=?1
         AND rowid BETWEEN ?2-3 AND ?2+3 ORDER BY rowid",
    )?;
    let passages = query
        .query_map([document, anchor], |r| {
            Ok(
                json!({"section": r.get::<_,String>(0)?, "passage": r.get::<_,String>(1)?,
                  "location": r.get::<_,String>(2)?}),
            )
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(
        json!({"schemaVersion":1,"source":source,"location":location,
              "sourceTextSha256":digest,"passages":passages}),
    )
}

pub fn ranked(question: &str, scenario: Option<&str>) -> Result<Value> {
    let mut expressions = vec![terms(question)?];
    if let Some(id) = scenario {
        expressions.push(terms(id)?);
    }
    expressions.retain(|term| !term.is_empty());
    let query = expressions.join(" OR ");
    let con = index()?;
    let mut hits = Vec::new();
    if !query.is_empty() {
        let mut statement = con.prepare(
            "SELECT s.path,s.kind,s.sha256,p.title,p.section,p.passage,p.location
             FROM passages AS p JOIN sources AS s ON s.rowid=p.source
             WHERE passages MATCH ?1 AND rank MATCH 'bm25(4,2,1,0,0)'
             ORDER BY rank * CASE WHEN s.path = 'docs/runbooks/' || ?2 || '.md'
                  OR s.path = 'wiki/Job-' || ?2 || '.md'
                  OR p.location GLOB 'job/' || ?2 || '/*' THEN 2.5 ELSE 1 END,
                  s.path,p.location,p.rowid LIMIT 120",
        )?;
        let mut rows = statement.query(rusqlite::params![query, scenario])?;
        let mut seen = HashSet::new();
        let mut sources = HashSet::new();
        while let Some(row) = rows.next()? {
            let source: String = row.get(0)?;
            let location: String = row.get(6)?;
            let title: String = row.get(3)?;
            let kind: String = row.get(1)?;
            let passage: String = row.get(5)?;
            let identity = if matches!(kind.as_str(), "tool" | "job" | "method") {
                format!("{source}/{}", location.split('/').nth(1).unwrap_or(""))
            } else {
                // The generated job Wiki and its canonical runbook often share the same page.
                title.to_lowercase()
            };
            if sources.contains(&identity) || seen.contains(&passage) {
                continue;
            }
            sources.insert(identity);
            seen.insert(passage.clone());
            hits.push(json!({"title":title,"section":row.get::<_,String>(4)?,
                "passage":passage,"source":source,"kind":kind,"location":location,
                "sourceTextSha256":row.get::<_,String>(2)?}));
            if hits.len() == 5 {
                break;
            }
        }
    }
    Ok(json!({"schemaVersion":1,"citations":hits,"sourceHashFormat":"sha256-utf8-lf"}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};
    use std::{fs, path::Path};

    #[test]
    fn relevant_sources_are_reproducible_in_a_standalone_binary() {
        let answers = search("Why is the printer queue stuck?").unwrap();
        assert_eq!(search("Why is the printer queue stuck?").unwrap(), answers);
        let citations = answers["citations"].as_array().unwrap();
        assert!(!citations.is_empty() && citations.len() <= 5);
        let best = &citations[0];
        assert!(best["source"].as_str().unwrap().contains("printer"));
        assert!(best["passage"].as_str().unwrap().contains("stuck job"));
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../..");
        for item in citations {
            let source = fs::read_to_string(root.join(item["source"].as_str().unwrap()))
                .unwrap()
                .replace("\r\n", "\n");
            assert_eq!(
                item["sourceTextSha256"],
                format!("{:x}", Sha256::digest(source.as_bytes()))
            );
            assert!(
                item["passage"]
                    .as_str()
                    .is_some_and(|text| !text.is_empty())
            );
            if let Some(line) = item["location"].as_str().unwrap().strip_prefix('L') {
                assert!(
                    source
                        .lines()
                        .nth(line.parse::<usize>().unwrap() - 1)
                        .is_some()
                );
            }
        }
        let dns = search("DNS lookup fails but IP addresses work").unwrap();
        assert!(dns["citations"].as_array().unwrap().iter().any(|item| {
            item["kind"] == "method"
                && item["location"]
                    .as_str()
                    .unwrap()
                    .contains("dns-connectivity-isolation")
        }));
    }

    #[test]
    fn query_limits_and_non_matches() {
        assert!(search("").is_err());
        assert!(search(&"x".repeat(2049)).is_err());
        for query in ["what is the", "qzxvnopearlmnop", "\"\"** -–<>"] {
            assert_eq!(search(query).unwrap()["citations"], json!([]));
        }
        assert!(search("DNS \" OR 1=1 --").is_ok());
    }

    #[test]
    fn context_resolves_the_original_guide_and_location() {
        let result = search("Why is the printer queue stuck?").unwrap();
        let hit = &result["citations"][0];
        let source = hit["source"].as_str().unwrap();
        let mark = hit["location"].as_str().unwrap();
        let result = context(source, mark).unwrap();
        assert_eq!(result["sourceTextSha256"], hit["sourceTextSha256"]);
        let rows = result["passages"].as_array().unwrap();
        assert!((2..=7).contains(&rows.len()));
        assert!(
            rows.iter()
                .any(|r| r["location"] == mark && r["passage"] == hit["passage"])
        );
        assert!(context("../local-file", mark).is_err());
        assert!(context(source, "L0").is_err());
    }

    #[test]
    fn a_reviewed_scenario_prefers_its_own_sources() {
        let result = ranked(
            "My computer is running extremely slowly. Find out why.",
            Some("slow-pc"),
        )
        .unwrap();
        let citation = &result["citations"][0];
        assert!(
            citation["source"].as_str().unwrap().contains("slow-pc")
                || citation["location"]
                    .as_str()
                    .unwrap()
                    .starts_with("job/slow-pc/")
        );
    }
}
