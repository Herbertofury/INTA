use std::{fs, path::PathBuf};
#[test]
#[ignore = "requires the acquired Windows toolbox; set INTA_TEST_TOOLBOX"]
fn verified_archive_operation_and_corruption() {
    let root =
        PathBuf::from(std::env::var_os("INTA_TEST_TOOLBOX").expect("select the fixture toolbox"));
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("-- @Ü customer fixture.7z");
    let valid = include_bytes!("fixtures/valid.7z");
    fs::write(&path, valid).unwrap();
    let result = inta_toolbox::inventory_archive(&root, &path).unwrap();
    assert_eq!(result["state"], "verified");
    assert_eq!(result["sizeBytes"], valid.len());
    assert_eq!(fs::read(&path).unwrap(), valid);
    let mut broken = *valid;
    broken[32] ^= 0x40;
    fs::write(&path, broken).unwrap();
    assert!(inta_toolbox::inventory_archive(&root, &path)
        .unwrap_err()
        .contains("could not be verified"));
    assert_eq!(temp.path().read_dir().unwrap().count(), 1);
}
