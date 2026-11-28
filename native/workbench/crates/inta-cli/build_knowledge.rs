use pulldown_cmark::{Event, Parser};
use rusqlite::{Connection, Transaction, params};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, error::Error, fs, path::Path};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

struct Source<'a> {
    transaction: &'a Transaction<'a>,
    id: i64,
}

impl Source<'_> {
    fn passage(&self, title: &str, section: &str, text: &str, location: &str) -> Result<()> {
        if text.is_empty() {
            return Ok(());
        }
        self.transaction.execute(
            "INSERT INTO passages(title,section,passage,source,location) VALUES(?,?,?,?,?)",
            params![title, section, text, self.id, location],
        )?;
        Ok(())
    }
}

fn plain(markdown: &str) -> String {
    let mut out = String::new();
    for event in Parser::new(markdown) {
        match event {
            Event::Text(text) | Event::Code(text) => out.push_str(&text),
            Event::SoftBreak | Event::HardBreak => out.push(' '),
            _ => {}
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn document(source: &Source, data: &str, path: &Path) -> Result<()> {
    let fallback = path
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .replace('-', " ");
    let mut title = fallback.clone();
    let mut section = String::new();
    let mut fence: Option<(char, usize)> = None;
    for (index, raw) in data.lines().enumerate() {
        let line = raw.trim();
        if let Some(ch) = line.chars().next().filter(|c| *c == '`' || *c == '~') {
            let length = line.chars().take_while(|c| *c == ch).count();
            if length >= 3 {
                if let Some((open, count)) = fence {
                    if open == ch && length >= count && line[length..].trim().is_empty() {
                        fence = None;
                    }
                } else {
                    fence = Some((ch, length));
                }
                continue;
            }
        }
        if fence.is_some() || line.is_empty() || line.starts_with('<') || line.starts_with("![") {
            continue;
        }
        if line.starts_with('#') {
            let heading = plain(line.trim_start_matches('#').trim());
            let heading = heading
                .trim_start_matches(|c: char| !c.is_alphanumeric())
                .trim();
            if line.starts_with("# ") {
                title = heading.to_string();
            }
            section = heading.to_string();
            continue;
        }
        if line.contains("](/") && line.contains(" · ") || line.matches("](#").count() > 1 {
            continue;
        }
        let text = plain(line.trim_matches('|')).replace(" | ", " · ");
        let words = text.split_whitespace().count();
        if !(7..=115).contains(&words)
            || text.chars().count() > 760
            || text.contains("https://")
            || text.starts_with("Open working view")
            || text.starts_with("Choose by the task shown")
        {
            continue;
        }
        source.passage(&title, &section, &text, &format!("L{}", index + 1))?;
    }
    Ok(())
}

fn records(source: &Source, data: &str, kind: &str, seen: &mut BTreeSet<String>) -> Result<()> {
    let value: Value = serde_json::from_str(data)?;
    let records = if kind == "method" {
        &value["methods"]
    } else {
        &value
    };
    for row in records.as_array().ok_or("Invalid source records")? {
        let id = row["id"].as_str().ok_or("Missing record ID")?;
        if !seen.insert(format!("{kind}/{id}")) {
            continue;
        }
        let title = row["name"]
            .as_str()
            .or_else(|| row["title"].as_str())
            .ok_or("Missing record title")?;
        for field in [
            "summary",
            "bestFor",
            "cautions",
            "startHere",
            "when",
            "proof",
        ] {
            if let Some(text) = row[field].as_str() {
                source.passage(title, field, text, &format!("{kind}/{id}/{field}"))?;
            }
        }
        for step in row["steps"].as_array().into_iter().flatten() {
            if let (Some(name), Some(detail), Some(anchor)) = (
                step["title"].as_str(),
                step["detail"].as_str(),
                step["anchor"].as_str(),
            ) {
                source.passage(title, name, detail, &format!("{kind}/{id}/{anchor}"))?;
            }
        }
    }
    Ok(())
}

pub fn build(root: &Path, output: &Path) -> Result<()> {
    let mut db = Connection::open_in_memory()?;
    db.execute_batch("CREATE TABLE sources(path TEXT UNIQUE NOT NULL,kind TEXT NOT NULL,sha256 TEXT NOT NULL);
        CREATE VIRTUAL TABLE passages USING fts5(title,section,passage,source UNINDEXED,location UNINDEXED,tokenize='porter unicode61');")?;
    let tx = db.transaction()?;
    let mut paths = vec![
        ("data/tools.json".into(), "tool"),
        ("native/workbench/data/additional-tools.json".into(), "tool"),
        ("data/jobs.json".into(), "job"),
        ("data/methods.json".into(), "method"),
    ];
    for (folder, kind) in [("docs/runbooks", "runbook"), ("wiki", "wiki")] {
        let directory = root.join(folder);
        println!("cargo:rerun-if-changed={}", directory.display());
        let mut markdown = Vec::new();
        for entry in fs::read_dir(directory)? {
            let path = entry?.path();
            if path.extension().is_some_and(|ext| ext == "md")
                && !path.file_name().unwrap().to_string_lossy().starts_with('_')
            {
                markdown.push(format!(
                    "{folder}/{}",
                    path.file_name().unwrap().to_string_lossy()
                ));
            }
        }
        markdown.sort();
        paths.extend(markdown.into_iter().map(|p| (p, kind)));
    }
    let mut seen = BTreeSet::new();
    for (rel, kind) in paths {
        let path = root.join(&rel);
        println!("cargo:rerun-if-changed={}", path.display());
        // Git's text checkout policy can change line endings without changing the document.
        let data = fs::read_to_string(&path)?.replace("\r\n", "\n");
        let digest = format!("{:x}", Sha256::digest(data.as_bytes()));
        tx.execute(
            "INSERT INTO sources VALUES(?,?,?)",
            params![rel, kind, digest],
        )?;
        let source = Source {
            transaction: &tx,
            id: tx.last_insert_rowid(),
        };
        if matches!(kind, "runbook" | "wiki") {
            document(&source, &data, &path)?;
        } else {
            records(&source, &data, kind, &mut seen)?;
        }
    }
    tx.execute("INSERT INTO passages(passages) VALUES('optimize')", [])?;
    tx.commit()?;
    db.execute_batch("VACUUM")?;
    fs::write(output, &*db.serialize("main")?)?;
    Ok(())
}
