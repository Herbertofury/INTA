use std::{env, fs, path::PathBuf};
fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("../..")
        .canonicalize()
        .unwrap();
    let registry_path = root.join("data/brand-icons.json");
    println!("cargo:rerun-if-changed={}", registry_path.display());
    let registry: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(registry_path).unwrap()).unwrap();
    let mut source = String::from("pub const ASSETS: &[(&str, &[u8], &str)] = &[\n");
    for (id, asset) in registry["assets"].as_object().unwrap() {
        let path = root.join(asset["path"].as_str().unwrap());
        println!("cargo:rerun-if-changed={}", path.display());
        source.push_str(&format!(
            "({:?}, include_bytes!({}), {:?}),\n",
            id,
            serde_json::to_string(&path.to_string_lossy()).unwrap(),
            asset["sha256"].as_str().unwrap()
        ));
    }
    source.push_str("];\n");
    fs::write(
        PathBuf::from(env::var("OUT_DIR").unwrap()).join("assets.rs"),
        source,
    )
    .unwrap();
}
