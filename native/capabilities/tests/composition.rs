#[cfg(windows)]
#[test]
fn verified_baseline_is_portable_between_cli_versions_and_never_clobbers() {
    use inta_capabilities::{BaselineDestination, BaselineObservation as Op};
    use serde_json::Value;
    use std::{fs, path::Path};
    let fixture = tempfile::tempdir().unwrap();
    let path = fixture.path().join("review.json");
    let destination = BaselineDestination::new(&path).unwrap();
    assert!(destination.export(&[]).is_err());
    assert!(destination.export(&[Op::Startup, Op::Startup]).is_err());
    assert!(!path.exists());
    let summary = destination
        .export(&[Op::Startup, Op::Power, Op::Memory])
        .unwrap();
    assert_eq!(summary["stepsVerified"], 3);
    assert_eq!(summary["testsPassed"], 9);
    let bytes = fs::read(&path).unwrap();
    let recipe: Value = serde_json::from_slice(&bytes).unwrap();
    assert!(
        recipe["steps"]
            .as_array()
            .unwrap()
            .iter()
            .all(|s| s["input"] == serde_json::json!({}))
    );
    assert!(destination.export(&[Op::Memory]).is_err());
    assert_eq!(fs::read(&path).unwrap(), bytes);
    assert!(BaselineDestination::new(&path).is_err());
    for p in [
        Path::new("relative.json"),
        Path::new(r"\\server\share\recipe.json"),
        &fixture.path().join("reserved.json:stream.json"),
        &fixture.path().join("CON.json"),
        &fixture.path().join("script.exe"),
    ] {
        assert!(BaselineDestination::new(p).is_err(), "{p:?}");
    }
}

#[cfg(windows)]
#[test]
fn intervening_destination_is_preserved() {
    use inta_capabilities::{BaselineDestination, BaselineObservation};
    let fixture = tempfile::tempdir().unwrap();
    let path = fixture.path().join("race.json");
    let target = BaselineDestination::new(&path).unwrap();
    std::fs::write(&path, b"external owner").unwrap();
    assert!(target.export(&[BaselineObservation::Memory]).is_err());
    assert_eq!(std::fs::read(path).unwrap(), b"external owner");
}
