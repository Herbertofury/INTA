mod build_knowledge;

use serde_json::Value;
use std::{collections::BTreeSet, env, fs, path::PathBuf};

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut rows = Vec::new();
    let mut ids = BTreeSet::new();
    for path in [
        root.join("../../../../data/tools.json"),
        root.join("../../data/additional-tools.json"),
    ] {
        println!("cargo:rerun-if-changed={}", path.display());
        let bytes = fs::read(path).expect("read INTA catalog");
        for row in serde_json::from_slice::<Vec<Value>>(&bytes).expect("parse catalog") {
            let id = row["id"].as_str().expect("catalog ID");
            if ids.insert(id.to_string()) {
                rows.push(row);
            }
        }
    }
    let out = PathBuf::from(env::var_os("OUT_DIR").expect("cargo output"));
    fs::write(out.join("tools.json"), serde_json::to_vec(&rows).unwrap()).unwrap();
    build_knowledge::build(&root.join("../../../.."), &out.join("knowledge.sqlite3"))
        .expect("build the offline INTA passage index");
}
