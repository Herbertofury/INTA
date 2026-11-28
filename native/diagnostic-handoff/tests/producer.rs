use serde_json::Value;
use std::process::Command;

#[test]
fn preview_names_only_the_approved_fields_and_rejects_unapproved_export() {
    let exe = env!("CARGO_BIN_EXE_inta-diagnostic-handoff");
    let preview = Command::new(exe).arg("preview").output().unwrap();
    assert!(preview.status.success());
    let v: Value = serde_json::from_slice(&preview.stdout).unwrap();
    assert_eq!(v["result"]["lifetimeSeconds"], 600);
    assert_eq!(v["result"]["requiresApproval"], true);
    assert_eq!(v["result"]["fields"].as_array().unwrap().len(), 6);
    for args in [
        vec!["export", "sample.intadiag"],
        vec!["export", "sample.intadiag", "--approve", "--key", "fixture"],
        vec!["listen"],
        vec!["send", "host"],
    ] {
        let out = Command::new(exe).args(args).output().unwrap();
        assert!(!out.status.success());
        assert_eq!(
            serde_json::from_slice::<Value>(&out.stdout).unwrap()["ok"],
            false
        );
    }
}
