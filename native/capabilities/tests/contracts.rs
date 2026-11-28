use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, path::Path, process::Command};
use tempfile::tempdir;

fn invoke(args: &[&str]) -> (i32, Value) {
    let out = Command::new(env!("CARGO_BIN_EXE_inta-capabilities"))
        .args(args)
        .output()
        .unwrap();
    assert!(
        out.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let v: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(v["schemaVersion"], 1);
    assert_eq!(v["ok"], out.status.success());
    (out.status.code().unwrap(), v)
}
fn ok(args: &[&str]) -> Value {
    let (code, v) = invoke(args);
    assert_eq!(code, 0, "{v}");
    v["result"].clone()
}
fn refused(args: &[&str]) {
    let (code, v) = invoke(args);
    assert_ne!(code, 0, "{v}");
    assert!(v["error"]["code"].is_string());
}
fn write(path: &Path, v: &Value) {
    fs::write(path, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn call_input(file: &Path, id: &str, value: &Value, success: bool) -> Value {
    write(file, value);
    let args = ["run", id, "--input", file.to_str().unwrap()];
    if success {
        ok(&args)
    } else {
        refused(&args);
        Value::Null
    }
}
fn recipe() -> Value {
    serde_json::from_str(include_str!("../examples/verify-workstation-basics.json")).unwrap()
}

#[test]
fn full_catalog_and_typed_discovery_preserve_sources() {
    let actual = ok(&["tools", "search", "", "500"]);
    let public: Vec<Value> =
        serde_json::from_str(include_str!("../../../data/tools.json")).unwrap();
    let supplemental: Vec<Value> =
        serde_json::from_str(include_str!("../../workbench/data/additional-tools.json")).unwrap();
    let expected: BTreeSet<_> = public
        .iter()
        .chain(&supplemental)
        .map(|r| r["id"].as_str().unwrap())
        .collect();
    let ids: BTreeSet<_> = actual["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["id"].as_str().unwrap())
        .collect();
    assert_eq!(ids, expected);
    assert_eq!(actual["total"], expected.len());
    let info = ok(&["--json", "discover"]);
    assert_eq!(info["catalogCount"], expected.len());
    for op in info["operations"].as_array().unwrap() {
        let desc = ok(&["schema", op["id"].as_str().unwrap()]);
        assert_eq!(desc, *op);
        assert_eq!(desc["effect"], "read-only");
        assert_eq!(desc["network"], false);
        assert_eq!(desc["inputSchema"]["additionalProperties"], false);
    }
    let exact = ok(&["tools", "search", "POWERCFG", "1"]);
    assert_eq!(exact["items"][0]["id"], "powercfg");
    let record = ok(&["tools", "inspect", "powercfg"]);
    assert_eq!(
        record["tool"],
        *public.iter().find(|r| r["id"] == "powercfg").unwrap()
    );
    assert_eq!(record["coverage"]["scope"], "partial");
    assert_eq!(record["operations"][0]["id"], "system.power");
    assert_eq!(ok(&["tools", "inspect", "7zip"])["operations"], json!([]));
    for args in [
        &["schema", "x"][..],
        &["tools", "inspect", "../../x"],
        &["tools", "search", "", "501"],
        &["run", "process.exec"],
        &["run", "system.power", "--host", "127.0.0.1"],
    ] {
        refused(args)
    }
}
#[cfg(windows)]
#[test]
fn file_identity_strict_input_and_text_vectors() {
    let tmp = tempdir().unwrap();
    let input = tmp.path().join("input.json");
    let file = tmp.path().join("-fixture Ω.bin");
    let bytes = b"\0abc\r\n\xff";
    fs::write(&file, bytes).unwrap();
    let got = call_input(&input, "file.sha256", &json!({"path":file}), true);
    assert_eq!(got["bytes"], bytes.len());
    assert_eq!(got["sha256"], format!("{:x}", Sha256::digest(bytes)));
    assert_eq!(fs::read(&file).unwrap(), bytes);
    for v in [
        json!({"path":"relative.bin"}),
        json!({"path":r"\\server\share\anything"}),
        json!({"path":r"\\.\PhysicalDrive0"}),
        json!({"path":format!("{}:stream",file.display())}),
        json!({"path":tmp.path().join("CON.txt")}),
        json!({"path":file,"arguments":["--version"]}),
    ] {
        call_input(&input, "file.sha256", &v, false);
    }
    fs::write(&input, br#"{"text":"a","text":"b"}"#).unwrap();
    refused(&["run", "text.sha256", "--input", input.to_str().unwrap()]);
    call_input(
        &input,
        "tools.search",
        &json!({"query":"","limit":null}),
        false,
    );
    let got = call_input(&input, "text.sha256", &json!({"text":""}), true);
    assert_eq!(
        got["sha256"],
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
    call_input(
        &input,
        "system.memory",
        &json!({"url":"https://example.invalid/"}),
        false,
    );
}
#[cfg(windows)]
#[test]
fn forge_holds_both_receipts_and_never_clobbers() {
    let tmp = tempdir().unwrap();
    let path = tmp.path().join("source.json");
    let store = tmp.path().join("new store");
    let input = path.to_str().unwrap();
    let dir = store.to_str().unwrap();
    let id = "verify-workstation-basics";
    write(&path, &recipe());
    let preview = ok(&["forge", "verify", input]);
    assert_eq!(preview["testsPassed"], 4);
    assert!(!store.exists());
    let save = || ok(&["forge", "import", input, "--store", dir]);
    assert_eq!(save()["state"], "imported");
    assert_eq!(save()["state"], "already-imported");
    let run = ok(&["forge", "run", id, "--store", dir]);
    assert_eq!(run["manifestSha256"], preview["manifestSha256"]);
    assert_eq!(run["observations"][1]["bytes"], 3);
    let mut different = recipe();
    different["title"] = "Changed title".into();
    write(&path, &different);
    refused(&["forge", "import", input, "--store", dir]);
    assert_eq!(
        ok(&["forge", "run", id, "--store", dir])["manifestSha256"],
        preview["manifestSha256"]
    );
    let record = store.join(format!("{id}.json"));
    let mut damaged: Value = serde_json::from_slice(&fs::read(&record).unwrap()).unwrap();
    damaged["manifest"]["title"] = "Edited after import".into();
    write(&record, &damaged);
    refused(&["forge", "run", id, "--store", dir]);
}
#[cfg(windows)]
#[test]
fn malformed_recipes_never_create_store() {
    let tmp = tempdir().unwrap();
    let input = tmp.path().join("recipe.json");
    let store = tmp.path().join("unused");
    let run = |v: Value| {
        write(&input, &v);
        refused(&[
            "forge",
            "import",
            input.to_str().unwrap(),
            "--store",
            store.to_str().unwrap(),
        ]);
        assert!(!store.exists())
    };
    let alter = |path: &str, v: Value| {
        let mut m = recipe();
        *m.pointer_mut(path).unwrap() = v;
        run(m)
    };
    alter("/steps/1/operation", "file.sha256".into());
    alter("/steps/1/operation", "shell.exec".into());
    alter("/version", 2.into());
    alter("/steps/1/version", 2.into());
    alter("/steps/0/input", json!({"id":"7zip","command":"extra"}));
    alter("/id", "../unsafe".into());
    alter("/tests/3/equals", "bad hash".into());
    alter("/tests", json!([]));
    alter(
        "/tests",
        json!([{"step":0,"pointer":"/tool/id","equals":"powercfg"}]),
    );
    let mut m = recipe();
    m["url"] = "https://example.invalid".into();
    run(m);
    fs::write(&input, vec![b' '; 65_537]).unwrap();
    refused(&["forge", "verify", input.to_str().unwrap()]);
}
#[cfg(windows)]
#[test]
fn owner_file_and_ancestor_links_are_refused() {
    let tmp = tempdir().unwrap();
    let outside = tempdir().unwrap();
    let actual = outside.path().join("real.json");
    write(&actual, &recipe());
    let link = tmp.path().join("junction");
    // Create only a generated directory junction; Directory.CreateSymbolicLink may require a privilege.
    let os = std::env::var_os("SystemRoot").unwrap();
    let cmd = std::path::PathBuf::from(os).join("System32/WindowsPowerShell/v1.0/powershell.exe");
    let status=Command::new(cmd).args(["-NoProfile","-NonInteractive","-Command","$ErrorActionPreference='Stop'; New-Item -ItemType Junction -Path $env:INTA_TEST_LINK -Target $env:INTA_TEST_TARGET | Out-Null"])
        .env("INTA_TEST_LINK",&link).env("INTA_TEST_TARGET",outside.path()).status().unwrap();
    assert!(status.success());
    refused(&["forge", "verify", link.join("real.json").to_str().unwrap()]);
    refused(&[
        "forge",
        "import",
        actual.to_str().unwrap(),
        "--store",
        link.to_str().unwrap(),
    ]);
    assert_eq!(fs::read_dir(outside.path()).unwrap().count(), 1);
    fs::remove_dir(link).unwrap();
}
