use super::{Ollama, remote};
use anyhow::{Context, Result, ensure};
use inta_capabilities::{BaselineDestination, BaselineObservation};
use serde::Deserialize;
use serde_json::{Value, json};
use std::path::Path;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Selection {
    operations: Vec<BaselineObservation>,
}

fn selection(response: &Value, model: &str) -> Result<Vec<BaselineObservation>> {
    ensure!(
        response["model"] == model
            && response["done"] == true
            && !remote(response)
            && response["tool_calls"].is_null()
            && response["done_reason"] != "length",
        "Local recipe selection did not finish with the verified model."
    );
    let raw = response["response"]
        .as_str()
        .context("Missing recipe selection")?;
    ensure!(raw.len() <= 512, "Oversized recipe selection");
    let chosen: Selection =
        serde_json::from_str(raw).context("The model did not return reviewed operation IDs")?;
    ensure!(
        !chosen.operations.is_empty(),
        "The local model found no supported baseline observation for this request."
    );
    ensure!(
        chosen.operations.len() <= BaselineObservation::ALL.len()
            && chosen
                .operations
                .iter()
                .enumerate()
                .all(|(i, op)| !chosen.operations[..i].contains(op)),
        "The local model returned an invalid or repeated operation."
    );
    Ok(chosen.operations)
}

pub(super) fn compose(
    provider: &Ollama,
    model: &str,
    digest: &str,
    request: &str,
    path: &Path,
) -> Result<Value> {
    ensure!(
        !request.trim().is_empty()
            && request.len() <= 512
            && !request.chars().any(char::is_control),
        "Use one line of 1–512 UTF-8 bytes describing the baseline."
    );
    let destination = BaselineDestination::new(path)?;
    provider.verify_model(model, digest)?;
    let instruction = "Classify the technician's request. Return only a JSON object with an operations array. Include only directly requested checks. RAM or memory capacity means system.memory. A Windows power plan or scheme means system.power. Startup or logon registration counts mean startup.counts. Multiple checks may be requested. Unsupported tasks get an empty array. Do not invent operations or parameters. Example: 'Record RAM and active power plan' => {\"operations\":[\"system.memory\",\"system.power\"]}. Example: 'Inspect startup registrations' => {\"operations\":[\"startup.counts\"]}. Example: 'Test a web server' => {\"operations\":[]}.";
    let generated = provider.request("/api/generate", Some(&json!({
        "model":model,"system":instruction,"prompt":request.trim(),"stream":false,"keep_alive":0,
        "format":{"type":"object","properties":{"operations":{"type":"array","minItems":0,"maxItems":3,
            "items":{"type":"string","enum":BaselineObservation::ALL}}},"required":["operations"],"additionalProperties":false},
        "options":{"temperature":0,"num_ctx":2048,"num_predict":72,"seed":1}
    })))?;
    let mut receipt = destination.export(&selection(&generated, model)?)?;
    receipt["model"] = model.into();
    receipt["modelDigest"] = digest.into();
    Ok(receipt)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn response(raw: &str) -> Value {
        json!({"model":"local","done":true,"response":raw,"done_reason":"stop"})
    }
    #[test]
    fn limits_model_output_to_complete_reviewed_local_selections() {
        let good = response(r#"{"operations":["system.power","system.memory"]}"#);
        assert_eq!(
            selection(&good, "local").unwrap(),
            [BaselineObservation::Power, BaselineObservation::Memory]
        );
        for bad in [
            r#"{"operations":[]}"#,
            r#"{"operations":["system.memory","system.memory"]}"#,
            r#"{"operations":["file.sha256"]}"#,
            r#"{"operations":["system.memory"],"input":{"path":"T:/file"}}"#,
            r#"{"operations":[],"operations":["system.memory"]}"#,
            r#"{"operations":[{"operation":"system.memory","command":"anything"}]}"#,
            r#"{"operations":["system.memory"]} other text"#,
        ] {
            assert!(selection(&response(bad), "local").is_err());
        }
        for (field, value) in [
            ("model", json!("other")),
            ("done", json!(false)),
            ("done_reason", json!("length")),
            ("remote_host", json!("elsewhere")),
            ("tool_calls", json!([])),
        ] {
            let mut changed = good.clone();
            changed[field] = value;
            assert!(selection(&changed, "local").is_err(), "{field}");
        }
    }
}
