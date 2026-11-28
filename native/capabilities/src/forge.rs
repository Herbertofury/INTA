use crate::{
    Result,
    catalog::{Catalog, valid_id},
    err, expect, files, invalid, operations,
    registry::Op,
    strict_json,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    io::{ErrorKind, Write},
    path::Path,
};

const MAX: u64 = 64 * 1024;
pub fn schema() -> Value {
    let steps:Vec<_>=Op::ALL.into_iter().filter(|op|op.forge()).map(|op|{
        let desc=crate::registry::describe(op.id()).expect("registered operation");
        json!({"type":"object","additionalProperties":false,"required":["operation","version","input"],
            "properties":{"operation":{"const":op.id()},"version":{"const":1},"input":desc["inputSchema"]}})
    }).collect();
    json!({"$schema":"https://json-schema.org/draft/2020-12/schema","title":"INTA offline Forge recipe v1",
        "type":"object","additionalProperties":false,"required":["schemaVersion","id","title","version","steps","tests"],
        "properties":{"schemaVersion":{"const":1},"version":{"const":1},"id":{"type":"string","maxLength":64,"pattern":"^[a-z0-9](?:[a-z0-9-]*[a-z0-9])?$"},
          "title":{"type":"string","minLength":1,"maxLength":120},
          "steps":{"type":"array","minItems":1,"maxItems":8,"items":{"oneOf":steps}},
          "tests":{"type":"array","minItems":1,"maxItems":24,"items":{"type":"object","additionalProperties":false,"required":["step","pointer","equals"],
            "properties":{"step":{"type":"integer","minimum":0,"maximum":7},"pointer":{"type":"string","maxLength":96,"pattern":"^/[A-Za-z0-9_/-]*$"},
            "equals":{"type":["boolean","number","string"]}}}}}})
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Manifest {
    schema_version: u32,
    id: String,
    title: String,
    version: u32,
    steps: Vec<Step>,
    tests: Vec<Vector>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Step {
    operation: String,
    version: u32,
    input: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Vector {
    step: usize,
    pointer: String,
    equals: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Stored {
    store_version: u32,
    manifest_sha256: String,
    manifest: Manifest,
}

fn decode(input: Value) -> Result<Manifest> {
    let m: Manifest = serde_json::from_value(input)
        .map_err(|_| invalid("Malformed Forge manifest or unexpected field."))?;
    expect(
        m.schema_version == 1 && m.version == 1 && valid_id(&m.id) && m.id.len() <= 64,
        "Forge requires manifest v1, revision 1, and a valid lower-case ID.",
    )?;
    expect(
        !m.title.trim().is_empty()
            && m.title.len() <= 120
            && !m.title.chars().any(char::is_control),
        "Invalid Forge title.",
    )?;
    expect(
        (1..=8).contains(&m.steps.len()) && (1..=24).contains(&m.tests.len()),
        "Use one to eight steps and one to twenty-four tests.",
    )?;
    for s in &m.steps {
        expect(
            s.version == 1 && Op::find(&s.operation).is_some_and(Op::forge),
            "Only listed, Forge-eligible operation versions may be imported.",
        )?;
        expect(s.input.is_object(), "Forge step input must be an object.")?;
    }
    let mut covered = vec![false; m.steps.len()];
    for v in &m.tests {
        expect(
            v.step < m.steps.len()
                && v.pointer.len() <= 96
                && v.pointer.starts_with('/')
                && v.pointer
                    .bytes()
                    .all(|b| b == b'/' || b == b'-' || b == b'_' || b.is_ascii_alphanumeric()),
            "A test must select an existing step and a simple JSON pointer.",
        )?;
        expect(
            matches!(
                v.equals,
                Value::Bool(_) | Value::Number(_) | Value::String(_)
            ),
            "Tests compare a scalar expected value.",
        )?;
        covered[v.step] = true;
    }
    expect(
        covered.into_iter().all(|x| x),
        "Every step requires at least one test vector.",
    )?;
    Ok(m)
}
fn digest(m: &Manifest) -> String {
    format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(m).expect("serialized manifest"))
    )
}
fn exercise(c: &Catalog, m: &Manifest, retain: bool) -> Result<Value> {
    let mut observations = Vec::new();
    for step in &m.steps {
        observations.push(operations::run(c, &step.operation, step.input.clone())?);
    }
    for (index, v) in m.tests.iter().enumerate() {
        if observations[v.step].pointer(&v.pointer) != Some(&v.equals) {
            return Err(err(
                "test-failed",
                format!("Forge test {} failed on step {}.", index + 1, v.step + 1),
            ));
        }
    }
    let mut result = json!({"id":m.id,"title":m.title,"manifestSha256":digest(m),"stepsVerified":m.steps.len(),"testsPassed":m.tests.len()});
    if retain {
        result["observations"] = observations.into();
    }
    Ok(result)
}
fn read_manifest(path: &Path) -> Result<Manifest> {
    decode(strict_json::parse(&files::read(path, MAX)?)?)
}
pub(super) fn verify_value(c: &Catalog, value: Value) -> Result<Value> {
    exercise(c, &decode(value)?, false)
}
pub fn verify_file(c: &Catalog, path: &Path) -> Result<Value> {
    exercise(c, &read_manifest(path)?, false)
}

fn stored(path: &Path) -> Result<Manifest> {
    let bytes = files::read(path, MAX * 2)?;
    let loaded: Stored = serde_json::from_value(strict_json::parse(&bytes)?)
        .map_err(|_| invalid("Invalid local Forge record."))?;
    expect(
        loaded.store_version == 1 && digest(&loaded.manifest) == loaded.manifest_sha256,
        "Forge record failed integrity verification.",
    )?;
    decode(serde_json::to_value(loaded.manifest).expect("serialized manifest"))
}
pub fn import(c: &Catalog, path: &Path, store: &Path) -> Result<Value> {
    let m = read_manifest(path)?;
    let mut receipt = exercise(c, &m, false)?;
    let folder = files::directory(store, true)?;
    let destination = folder.path.join(format!("{}.json", m.id));
    let sha = digest(&m);
    let record = serde_json::to_vec(&Stored {
        store_version: 1,
        manifest_sha256: sha.clone(),
        manifest: m,
    })
    .expect("serialized Forge record");
    let mut tmp = tempfile::NamedTempFile::new_in(&folder.path)
        .map_err(|_| err("io", "Could not stage the Forge record."))?;
    tmp.write_all(&record)
        .and_then(|_| tmp.as_file().sync_all())
        .map_err(|_| err("io", "Could not flush the Forge record."))?;
    let state = match tmp.persist_noclobber(&destination) {
        Ok(_) => "imported",
        Err(e) if e.error.kind() == ErrorKind::AlreadyExists => {
            let prior = stored(&destination)?;
            if digest(&prior) != sha {
                return Err(err(
                    "conflict",
                    "This recipe ID already has a different manifest.",
                ));
            }
            "already-imported"
        }
        Err(_) => return Err(err("io", "Could not publish the verified Forge record.")),
    };
    receipt["state"] = state.into();
    Ok(receipt)
}
pub fn run_stored(c: &Catalog, id: &str, store: &Path) -> Result<Value> {
    expect(valid_id(id) && id.len() <= 64, "Invalid Forge recipe ID.")?;
    let folder = files::directory(store, false)?;
    let m = stored(&folder.path.join(format!("{id}.json")))?;
    expect(m.id == id, "Stored recipe identity mismatch.")?;
    exercise(c, &m, true)
}
