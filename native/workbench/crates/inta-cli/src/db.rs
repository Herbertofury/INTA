use anyhow::{Context, Result, bail};
use rusqlite::{Connection, TransactionBehavior, params};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{env, fs, path::PathBuf, time::Duration};

const TOOLS: &str = include_str!(concat!(env!("OUT_DIR"), "/tools.json"));
const JOBS: &str = include_str!("../../../../../data/jobs.json");
const METHODS: &str = include_str!("../../../../../data/methods.json");

pub fn data_dir() -> Result<PathBuf> {
    if let Some(path) = env::var_os("INTA_DATA_DIR") {
        let path = PathBuf::from(path);
        anyhow::ensure!(path.is_absolute(), "INTA_DATA_DIR must be absolute");
        return Ok(path);
    }
    let exe = env::current_exe()?;
    let base = exe.parent().context("Application directory missing")?;
    if base.join("inta-portable.marker").is_file() {
        return Ok(base.join("state"));
    }
    Ok(PathBuf::from(
        env::var_os("LOCALAPPDATA").context("Set INTA_DATA_DIR for this environment")?,
    )
    .join("INTA")
    .join("state"))
}
pub fn open() -> Result<Connection> {
    let dir = data_dir()?;
    fs::create_dir_all(&dir)?;
    let mut c = Connection::open(dir.join("technician.sqlite3"))?;
    c.busy_timeout(Duration::from_secs(12))?;
    prepare(&mut c)?;
    Ok(c)
}

fn prepare(c: &mut Connection) -> Result<()> {
    let version: i64 = c.query_row("PRAGMA user_version", [], |r| r.get(0))?;
    if version > 1 {
        bail!("This database requires a newer INTA build")
    }
    let mut hasher = Sha256::new();
    for input in [
        b"catalog-schema-1".as_slice(),
        TOOLS.as_bytes(),
        JOBS.as_bytes(),
        METHODS.as_bytes(),
    ] {
        hasher.update(input);
    }
    let fingerprint = format!("{:x}", hasher.finalize());
    if version == 1
        && c.query_row(
            "SELECT EXISTS(SELECT 1 FROM metadata WHERE key='catalog' AND value=?)
        AND EXISTS(SELECT 1 FROM metadata WHERE key='maintenance-v2' AND value=date('now'))",
            [&fingerprint],
            |r| r.get::<_, bool>(0),
        )?
    {
        return Ok(());
    }
    let tx = c.transaction_with_behavior(TransactionBehavior::Immediate)?;
    tx.execute_batch("CREATE TABLE IF NOT EXISTS metadata (key TEXT PRIMARY KEY,value TEXT NOT NULL);
        CREATE TABLE IF NOT EXISTS records (kind TEXT NOT NULL,id TEXT NOT NULL,title TEXT NOT NULL,summary TEXT NOT NULL,category TEXT NOT NULL,payload TEXT NOT NULL,PRIMARY KEY(kind,id));
        CREATE VIRTUAL TABLE IF NOT EXISTS search USING fts5(title,id,body,tokenize='unicode61');
        CREATE TABLE IF NOT EXISTS events(seq INTEGER PRIMARY KEY,at TEXT NOT NULL,operation TEXT NOT NULL,state TEXT NOT NULL);
        CREATE TABLE IF NOT EXISTS runs(id TEXT PRIMARY KEY,scenario TEXT NOT NULL,created TEXT NOT NULL,steps TEXT NOT NULL);
        CREATE TABLE IF NOT EXISTS run_plans(id TEXT PRIMARY KEY,stages TEXT NOT NULL);
        CREATE TABLE IF NOT EXISTS tweaks(id TEXT PRIMARY KEY,before_value TEXT NOT NULL,phase TEXT NOT NULL);
        PRAGMA user_version=1;")?;
    let current: String = tx.query_row(
        "SELECT COALESCE((SELECT value FROM metadata WHERE key='catalog'),'')",
        [],
        |r| r.get(0),
    )?;
    if current != fingerprint {
        tx.execute_batch("DELETE FROM records; DELETE FROM search;")?;
        for (kind, src) in [("tool", TOOLS), ("job", JOBS), ("method", METHODS)] {
            let root: Value = serde_json::from_str(src)?;
            let rows = if kind == "method" {
                &root["methods"]
            } else {
                &root
            };
            for row in rows
                .as_array()
                .context("Catalog source must contain a list")?
            {
                let id = row["id"].as_str().context("Missing catalog ID")?;
                let title = row["name"]
                    .as_str()
                    .or(row["title"].as_str())
                    .context("Missing catalog title")?;
                let category = row["category"].as_str().unwrap_or("");
                tx.execute("INSERT INTO records(kind,id,title,summary,category,payload) VALUES(?,?,?,?,?,?)",params![kind,id,title,row["summary"].as_str().unwrap_or(""),category,row.to_string()])?;
                let rowid = tx.last_insert_rowid();
                let mut terms = String::new();
                collect_text(row, &mut terms);
                tx.execute(
                    "INSERT INTO search(rowid,title,id,body) VALUES(?,?,?,?)",
                    params![rowid, title, id, terms],
                )?;
            }
        }
        tx.execute(
            "INSERT OR REPLACE INTO metadata VALUES('catalog',?)",
            [fingerprint],
        )?;
    }
    tx.execute(
        "DELETE FROM events WHERE at < strftime('%Y-%m-%dT%H:%M:%SZ','now','-30 days')",
        [],
    )?;
    tx.execute(
        "DELETE FROM runs WHERE created < strftime('%Y-%m-%dT%H:%M:%SZ','now','-30 days')",
        [],
    )?;
    tx.execute(
        "DELETE FROM run_plans WHERE id NOT IN (SELECT id FROM runs)",
        [],
    )?;
    tx.execute(
        "INSERT OR REPLACE INTO metadata VALUES('maintenance-v2',date('now'))",
        [],
    )?;
    tx.commit()?;
    Ok(())
}
fn collect_text(v: &Value, out: &mut String) {
    match v {
        Value::String(s) => {
            out.push(' ');
            out.push_str(s);
        }
        Value::Object(m) => m.values().for_each(|x| collect_text(x, out)),
        Value::Array(a) => a.iter().for_each(|x| collect_text(x, out)),
        _ => (),
    }
}
pub fn catalog(c: &Connection, q: &str) -> Result<Value> {
    anyhow::ensure!(q.len() < 4096, "Search is limited to 4095 bytes");
    let words = q
        .split(|v: char| !v.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|s| format!("\"{s}\"*"))
        .collect::<Vec<_>>()
        .join(" AND ");
    if words.is_empty() && !q.trim().is_empty() {
        return Ok(json!([]));
    }
    let all = words.is_empty();
    let mut s=c.prepare(if all {"SELECT kind,id,title,summary,category FROM records ORDER BY title COLLATE NOCASE,id"}else{"SELECT kind,records.id,records.title,summary,category FROM search JOIN records ON records.rowid=search.rowid WHERE search MATCH ?1 ORDER BY bm25(search,8,5,1),records.title COLLATE NOCASE,kind,records.id"})?;
    let mut rs = if all { s.query([])? } else { s.query([words])? };
    let mut res = Vec::new();
    while let Some(row) = rs.next()? {
        res.push(json!({"kind":row.get::<_,String>(0)?,"id":row.get::<_,String>(1)?,"title":row.get::<_,String>(2)?,"summary":row.get::<_,String>(3)?,"category":row.get::<_,String>(4)?}));
    }
    Ok(json!(res))
}
pub fn record(c: &Connection, kind: &str, id: &str) -> Result<Value> {
    let payload: String = c
        .query_row(
            "SELECT payload FROM records WHERE kind=? AND id=?",
            params![kind, id],
            |r| r.get(0),
        )
        .context("Unknown record")?;
    Ok(serde_json::from_str(&payload)?)
}
pub fn now() -> String {
    time::OffsetDateTime::now_utc()
        .replace_nanosecond(0)
        .expect("zero nanoseconds are valid")
        .format(&time::format_description::well_known::Rfc3339)
        .expect("system time is representable as an RFC 3339 timestamp")
}
pub fn event(c: &Connection, op: &str, state: &str) -> Result<()> {
    c.execute(
        "INSERT INTO events(at,operation,state) VALUES(?,?,?)",
        params![now(), op, state],
    )?;
    Ok(())
}
pub fn history(c: &Connection) -> Result<Value> {
    let mut s =
        c.prepare("SELECT seq,at,operation,state FROM events ORDER BY seq DESC LIMIT 500")?;
    let rows=s.query_map([],|r|Ok(json!({"sequence":r.get::<_,i64>(0)?,"observedAt":r.get::<_,String>(1)?,"operation":r.get::<_,String>(2)?,"state":r.get::<_,String>(3)?})))?.collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(json!(rows))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ingestion_is_idempotent_and_complete() -> Result<()> {
        let mut c = Connection::open_in_memory()?;
        prepare(&mut c)?;
        prepare(&mut c)?;
        let n: usize = serde_json::from_str::<Vec<Value>>(TOOLS)?.len()
            + serde_json::from_str::<Vec<Value>>(JOBS)?.len()
            + serde_json::from_str::<Value>(METHODS)?["methods"]
                .as_array()
                .unwrap()
                .len();
        assert_eq!(catalog(&c, "")?.as_array().unwrap().len(), n);
        assert_eq!(
            catalog(&c, "Defender Offline")?[0]["id"],
            "defender-offline"
        );
        assert_eq!(record(&c, "job", "intake-triage")?["id"], "intake-triage");
        assert_eq!(catalog(&c, "---")?, json!([]));
        Ok(())
    }
}
