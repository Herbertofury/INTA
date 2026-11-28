use crate::{db, inspect};
use anyhow::{Context, Result, ensure};
use rusqlite::{Connection, params};
use serde_json::{Value, json};
use std::fs::OpenOptions;
use uuid::Uuid;

const SCENARIOS: &[(&str, &[&str])] = &[
    ("intake-triage", &["overview", "storage"]),
    ("new-pc", &["overview", "storage", "security", "network"]),
    (
        "slow-pc",
        &["overview", "performance", "storage", "startup"],
    ),
    ("hardware-bsod", &["overview", "drivers", "events"]),
    ("network-dns", &["network"]),
    ("windows-update", &["updates", "storage", "events"]),
    (
        "preventive-maintenance",
        &["overview", "storage", "security", "startup"],
    ),
    ("malware-removal", &["security", "startup"]),
    (
        "windows-repair",
        &["overview", "storage", "events", "drivers"],
    ),
    ("failing-drive", &["storage"]),
    ("backup-restore", &["storage", "recovery"]),
    ("boot-recovery", &["recovery", "disks", "events"]),
    ("boot-usb", &["recovery", "disks"]),
    ("disk-migration", &["disks", "storage", "recovery"]),
    ("outage-recovery", &["power", "disks", "events"]),
    ("printer-repair", &["printing", "drivers"]),
    ("ad-trust", &["directory", "network"]),
    ("outlook-email", &["network", "applications"]),
    ("remote-support", &["network", "security"]),
    ("ransomware-response", &["security", "events", "recovery"]),
    ("client-handoff", &["overview", "storage", "updates"]),
    ("ai-triage", &["models", "overview"]),
];
fn stages(id: &str) -> Result<&'static [&'static str]> {
    SCENARIOS
        .iter()
        .find(|p| p.0 == id)
        .map(|p| p.1)
        .context("Unknown inspection scenario")
}
pub fn preview(c: &Connection, id: &str) -> Result<Value> {
    let guide = db::record(c, "job", id)?;
    Ok(
        json!({"id":id,"title":guide["title"],"stages":stages(id)?,"mode":"inspect-only","effects":"Read local Windows providers. Save stage progress for 30 days. Raw observations are returned to the caller.","stopIf":guide["stopIf"],"verify":guide["verify"]}),
    )
}
pub fn list(c: &Connection) -> Result<Value> {
    Ok(Value::Array(
        SCENARIOS
            .iter()
            .map(|(id, _)| preview(c, id))
            .collect::<Result<_>>()?,
    ))
}
pub fn run(c: &Connection, id: &str, skip: &[String], resume: Option<&str>) -> Result<Value> {
    let run_id = if let Some(id) = resume {
        Uuid::parse_str(id)?.to_string()
    } else {
        Uuid::new_v4().to_string()
    };
    let gates = stages(id)?;
    let plan = serde_json::to_string(gates)?;
    for s in skip {
        ensure!(
            gates.contains(&s.as_str()),
            "Unknown stage for this scenario"
        )
    }
    let lock = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(db::data_dir()?.join("scenario.lock"))?;
    lock.try_lock().context("Another scenario is running")?;
    let mut completed: serde_json::Map<String, Value> = if resume.is_some() {
        let text: String = c
            .query_row(
                "SELECT steps FROM runs JOIN run_plans USING(id) WHERE id=? AND scenario=? AND run_plans.stages=?",
                params![run_id, id, plan],
                |r| r.get(0),
            )
            .context("Checkpoint unavailable for this scenario version. Start a new run.")?;
        serde_json::from_str(&text)?
    } else {
        let tx = c.unchecked_transaction()?;
        tx.execute(
            "INSERT INTO runs VALUES(?,?,?,'{}')",
            params![run_id, id, db::now()],
        )?;
        tx.execute("INSERT INTO run_plans VALUES(?,?)", params![run_id, plan])?;
        tx.commit()?;
        serde_json::Map::new()
    };
    let mut observations = vec![];
    let mut stopped = false;
    for stage in gates {
        if matches!(
            completed.get(*stage).and_then(Value::as_str),
            Some("complete" | "skipped")
        ) {
            continue;
        }
        let status = if skip.iter().any(|s| s == stage) {
            "skipped"
        } else {
            match inspect::run(stage) {
                Ok(value) => {
                    observations.push(value);
                    "complete"
                }
                Err(e) => {
                    observations.push(json!({"kind":stage,"error":e.to_string()}));
                    stopped = true;
                    "failed"
                }
            }
        };
        completed.insert(stage.to_string(), json!(status));
        let tx = c.unchecked_transaction()?;
        tx.execute(
            "UPDATE runs SET steps=? WHERE id=?",
            params![serde_json::to_string(&completed)?, run_id],
        )?;
        db::event(&tx, &format!("scenario/{id}/{stage}"), status)?;
        tx.commit()?;
        if stopped {
            break;
        }
    }
    Ok(
        json!({"schemaVersion":1,"id":id,"runId":run_id,"mode":"inspect-only","status":if stopped{"stopped"}else{"complete"},"stages":completed,"observations":observations}),
    )
}
pub fn checkpoints(c: &Connection) -> Result<Value> {
    let mut s = c.prepare(
        "SELECT id,scenario,created,steps FROM runs ORDER BY created DESC,rowid DESC LIMIT 100",
    )?;
    let mut rs = s.query([])?;
    let mut items = vec![];
    while let Some(r) = rs.next()? {
        items.push(json!({"runId":r.get::<_,String>(0)?,"scenario":r.get::<_,String>(1)?,"created":r.get::<_,String>(2)?,"stages":serde_json::from_str::<Value>(&r.get::<_,String>(3)?)?}));
    }
    Ok(json!(items))
}
