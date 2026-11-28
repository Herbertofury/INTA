use serde_json::Value;
use std::process::{Command, Output};
use tempfile::TempDir;

fn cli(data: &TempDir, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_inta"))
        .args(args)
        .env("INTA_DATA_DIR", data.path())
        .output()
        .expect("start INTA")
}
#[test]
fn durable_checkpoint_and_inspection_boundaries() {
    let state = tempfile::tempdir().unwrap();
    let run = cli(
        &state,
        &["scenario", "run", "network-dns", "--skip", "network"],
    );
    assert!(run.status.success());
    let row: Value = serde_json::from_slice(&run.stdout).unwrap();
    assert_eq!(row["stages"]["network"], "skipped");
    let id = row["runId"].as_str().unwrap();
    assert!(
        cli(&state, &["scenario", "resume", "network-dns", id])
            .status
            .success()
    );
    assert!(
        !cli(&state, &["scenario", "resume", "slow-pc", id])
            .status
            .success()
    );
    for args in [
        vec!["scenario", "run", "new-pc", "--skip", "unrecognized"],
        vec!["tweak", "apply"],
        vec!["tweak", "undo", "--yes"],
        vec!["inspect", "bad; whoami"],
        vec!["storage", "relative-path"],
    ] {
        assert!(!cli(&state, &args).status.success(), "{args:?}");
    }
    let h: Value = serde_json::from_slice(&cli(&state, &["history"]).stdout).unwrap();
    assert!(
        h.as_array()
            .unwrap()
            .iter()
            .any(|v| v["operation"] == "scenario/network-dns/network")
    );
    assert!(
        !h.to_string()
            .contains(state.path().to_string_lossy().as_ref())
    );
}
#[test]
fn all_source_fields_survive_ingestion() {
    let state = tempfile::tempdir().unwrap();
    let jobs: Value = serde_json::from_str(include_str!("../../../../../data/jobs.json")).unwrap();
    for job in jobs.as_array().unwrap() {
        let out = cli(&state, &["record", "job", job["id"].as_str().unwrap()]);
        assert!(out.status.success());
        assert_eq!(serde_json::from_slice::<Value>(&out.stdout).unwrap(), *job);
    }
}
