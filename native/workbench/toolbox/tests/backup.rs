#![cfg(windows)]
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Barrier},
};
fn toolbox() -> PathBuf {
    PathBuf::from(std::env::var_os("INTA_TEST_TOOLBOX").expect("select acquired toolbox"))
}
fn archived_fixture(root: &Path, dir: &Path, bytes: &[u8]) -> (PathBuf, serde_json::Value) {
    let input = dir.join("-@检查 report.txt");
    let output = dir.join("result.7z");
    fs::write(&input, bytes).unwrap();
    let report = inta_toolbox::create_backup(root, &input, &output).unwrap();
    assert_eq!(report["roundTripVerified"], true);
    assert_eq!(report["inputBytes"], bytes.len());
    assert_eq!(
        report["inputSha256"],
        format!("{:x}", Sha256::digest(bytes))
    );
    assert_eq!(
        report["archiveSha256"],
        format!("{:x}", Sha256::digest(fs::read(&output).unwrap()))
    );
    assert_eq!(fs::read(&input).unwrap(), bytes);
    (output, report)
}

#[test]
#[ignore = "requires the acquired Windows toolbox; set INTA_TEST_TOOLBOX"]
fn new_backup_round_trip_collision_and_corrupt_archive() {
    let root = toolbox();
    let work = tempfile::tempdir().unwrap();
    let input = b"INTA offline backup fixture";
    let (out, report) = archived_fixture(&root, work.path(), input);
    let stored = fs::read(&out).unwrap();
    let checked = inta_toolbox::inventory_archive(&root, &out).unwrap();
    assert_eq!(checked["sha256"], report["archiveSha256"]);
    assert!(
        inta_toolbox::create_backup(&root, &work.path().join("-@检查 report.txt"), &out)
            .unwrap_err()
            .contains("exists")
    );
    assert_eq!(fs::read(&out).unwrap(), stored);
    let mut bad = stored;
    bad[32] ^= 0x40;
    fs::write(&out, &bad).unwrap();
    assert!(inta_toolbox::inventory_archive(&root, &out).is_err());
    assert_eq!(fs::read_dir(work.path()).unwrap().count(), 2);
}
#[test]
#[ignore = "requires the acquired Windows toolbox; set INTA_TEST_TOOLBOX"]
fn empty_file_and_concurrent_publication() {
    let root = toolbox();
    let work = tempfile::tempdir().unwrap();
    let file = work.path().join("empty.txt");
    fs::write(&file, b"").unwrap();
    let output = work.path().join("one.7z");
    let barrier = Arc::new(Barrier::new(2));
    let attempt = || {
        let (root, input, dest, go) = (root.clone(), file.clone(), output.clone(), barrier.clone());
        std::thread::spawn(move || {
            go.wait();
            inta_toolbox::create_backup(&root, &input, &dest)
        })
    };
    let a = attempt();
    let b = attempt();
    let results = [a.join().unwrap(), b.join().unwrap()];
    assert_eq!(
        results.iter().filter(|r| r.is_ok()).count(),
        1,
        "{results:?}"
    );
    assert_eq!(fs::metadata(&file).unwrap().len(), 0);
    assert_eq!(fs::read_dir(work.path()).unwrap().count(), 2);
    assert_eq!(
        inta_toolbox::inventory_archive(&root, &output).unwrap()["state"],
        "verified"
    );
}

#[test]
fn unexpected_named_stream_is_not_silently_dropped() {
    let work = tempfile::tempdir().unwrap();
    let input = work.path().join("fixture.bin");
    fs::write(&input, b"named-stream fixture").unwrap();
    fs::write(
        work.path().join("fixture.bin:INTA-test"),
        b"additional data",
    )
    .unwrap();
    let error = inta_toolbox::create_backup(work.path(), &input, &work.path().join("result.7z"))
        .unwrap_err();
    assert!(error.contains("stream"), "{error}");
    assert!(!work.path().join("result.7z").exists());
}
#[test]
#[ignore = "requires the acquired Windows toolbox; set INTA_TEST_TOOLBOX"]
fn backup_uses_relocated_package_by_uuid_and_pin() {
    let original = toolbox();
    let location = tempfile::tempdir().unwrap();
    let copy = location.path().join("@portable 工具");
    fs::create_dir(&copy).unwrap();
    let marker = fs::read(original.join(".inta-toolbox.json")).unwrap();
    fs::write(copy.join(".inta-toolbox.json"), &marker).unwrap();
    let id: serde_json::Value = serde_json::from_slice(&marker).unwrap();
    let idx: serde_json::Value =
        serde_json::from_slice(&fs::read(original.join(".inta-packages.json")).unwrap()).unwrap();
    let rel = idx["packages"]["7zip"].as_str().unwrap();
    let rearranged = "Tools/zip/PORTABLE/7zr.exe";
    let binary = copy.join(rearranged);
    fs::create_dir_all(binary.parent().unwrap()).unwrap();
    fs::copy(original.join(rel), &binary).unwrap();
    fs::write(copy.join(".inta-packages.json"),serde_json::json!({"schemaVersion":1,"toolboxId":id["toolboxId"],"packages":{"7zip":rearranged}}).to_string()).unwrap();
    let work = tempfile::tempdir().unwrap();
    let (out, _) = archived_fixture(&copy, work.path(), b"relocated");
    assert_eq!(
        inta_toolbox::inventory_archive(&copy, &out).unwrap()["state"],
        "verified"
    );
}
