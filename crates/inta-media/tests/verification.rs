use inta_media::*;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    io::{Cursor, Read},
    sync::atomic::AtomicBool,
};
fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn asset(name: &str, size: u64) -> Asset {
    Asset {
        name: name.into(),
        size,
        browser_download_url: format!("{ASSET_ROOT}rescue-test/{name}"),
    }
}
struct Fixtures(BTreeMap<String, Vec<u8>>);
impl Fetcher for Fixtures {
    fn fetch(&self, url: &str) -> Result<Box<dyn Read>> {
        Ok(Box::new(Cursor::new(
            self.0.get(url).ok_or("Unknown fixture URL")?.clone(),
        )))
    }
}
fn multipart() -> (Release, Fixtures, Vec<u8>) {
    let parts = [b"first-part".to_vec(), b"second-part".to_vec()];
    let combined = parts.concat();
    let manifest = json!({"schemaVersion":1,"filename":ISO,"size":combined.len(),"sha256":hash(&combined),"parts":parts.iter().enumerate().map(|(i,p)|json!({"name":format!("{ISO}.part{:03}",i+1),"size":p.len(),"sha256":hash(p)})).collect::<Vec<_>>()});
    let bytes = serde_json::to_vec(&manifest).unwrap();
    let sums = format!("{}  {ISO}\n", hash(&combined));
    let mut assets = vec![
        asset(SUMS, sums.len() as u64),
        asset(PARTS, bytes.len() as u64),
    ];
    let mut fixtures = BTreeMap::new();
    fixtures.insert(assets[0].browser_download_url.clone(), sums.into_bytes());
    fixtures.insert(assets[1].browser_download_url.clone(), bytes);
    for (i, p) in parts.into_iter().enumerate() {
        let a = asset(&format!("{ISO}.part{:03}", i + 1), p.len() as u64);
        fixtures.insert(a.browser_download_url.clone(), p);
        assets.push(a);
    }
    (
        Release {
            assets,
            draft: false,
            prerelease: false,
        },
        Fixtures(fixtures),
        combined,
    )
}
#[test]
fn checksum_requires_exactly_one_image_record() {
    let digest = "a".repeat(64);
    assert_eq!(
        expected_digest(&format!("{digest}  {ISO}\n")).unwrap(),
        digest
    );
    assert!(expected_digest(&format!("{digest}  {ISO}\n{digest}  {ISO}\n")).is_err());
    assert!(expected_digest("bad  inta-rescue-amd64.iso").is_err());
    assert!(expected_digest(&format!("{digest}  other.iso")).is_err());
}
#[test]
fn release_assets_reject_foreign_origins_and_ambiguous_names() {
    let mut a = asset(ISO, 10);
    for url in["https://evil.example/inta-rescue-amd64.iso","https://github.com@evil.example/Herbertofury/INTA/releases/download/v/inta-rescue-amd64.iso","https://github.com/other/INTA/releases/download/v/inta-rescue-amd64.iso","https://github.com/Herbertofury/INTA/releases/download/v/inta-rescue-amd64.iso?redirect=x"]{a.browser_download_url=url.into();let map=BTreeMap::from([(ISO.into(),a.clone())]);assert!(release_asset(&map,ISO).is_err());}
    assert!(asset_map(Release {
        assets: vec![asset(ISO, 10), asset(ISO, 10)],
        draft: false,
        prerelease: false
    })
    .is_err());
}
#[test]
fn manifests_reject_duplicate_unknown_boolean_and_unordered_fields() {
    let (a, f, _) = multipart();
    let assets = asset_map(a).unwrap();
    let bytes = f.0.get(&assets[PARTS].browser_download_url).unwrap();
    let mut value: serde_json::Value = serde_json::from_slice(bytes).unwrap();
    value["parts"][0]["url"] = json!("https://evil.example");
    assert!(serde_json::from_value::<Manifest>(value).is_err());
    assert!(serde_json::from_str::<Manifest>(
        r#"{"schemaVersion":1,"schemaVersion":1,"filename":"x","size":1,"sha256":"x","parts":[]}"#
    )
    .is_err());
    let mut value: serde_json::Value = serde_json::from_slice(bytes).unwrap();
    value["size"] = json!(true);
    assert!(serde_json::from_value::<Manifest>(value).is_err());
    let mut manifest: Manifest = serde_json::from_slice(bytes).unwrap();
    manifest.parts.swap(0, 1);
    let expected = manifest.sha256.clone();
    assert!(verified_parts(manifest, &assets, &expected).is_err());
}
#[test]
fn verifies_every_part_and_combined_iso_before_replacement() {
    let folder = tempfile::tempdir().unwrap();
    let (release, fixtures, expected) = multipart();
    let plan = resolve_plan(release, &fixtures).unwrap();
    std::fs::write(folder.path().join(ISO), b"old-image").unwrap();
    let mut progress = Vec::new();
    let path = atomic_download(
        &plan,
        folder.path(),
        &fixtures,
        &AtomicBool::new(false),
        &mut |p| progress.push(p),
    )
    .unwrap();
    assert_eq!(std::fs::read(path).unwrap(), expected);
    assert_eq!(
        expected_digest(&std::fs::read_to_string(folder.path().join(SUMS)).unwrap()).unwrap(),
        hash(&expected)
    );
    assert_eq!(progress.last().unwrap().received, expected.len() as u64);
    assert_eq!(std::fs::read_dir(folder.path()).unwrap().count(), 2);
}
#[test]
fn corrupted_part_preserves_previous_image_and_removes_temporary_file() {
    let folder = tempfile::tempdir().unwrap();
    let (release, mut fixtures, _) = multipart();
    let plan = resolve_plan(release, &fixtures).unwrap();
    fixtures
        .0
        .insert(plan.parts[1].url.clone(), b"corruption!".to_vec());
    std::fs::write(folder.path().join(ISO), b"old-image").unwrap();
    assert!(atomic_download(
        &plan,
        folder.path(),
        &fixtures,
        &AtomicBool::new(false),
        &mut |_| {}
    )
    .is_err());
    assert_eq!(
        std::fs::read(folder.path().join(ISO)).unwrap(),
        b"old-image"
    );
    assert_eq!(std::fs::read_dir(folder.path()).unwrap().count(), 1);
}
#[test]
fn cancellation_and_size_overrun_keep_previous_image() {
    let folder = tempfile::tempdir().unwrap();
    let (release, fixtures, _) = multipart();
    let plan = resolve_plan(release, &fixtures).unwrap();
    std::fs::write(folder.path().join(ISO), b"old").unwrap();
    assert!(atomic_download(
        &plan,
        folder.path(),
        &fixtures,
        &AtomicBool::new(true),
        &mut |_| {}
    )
    .is_err());
    assert!(stream_verified(
        Cursor::new(b"oversized"),
        &folder.path().join(ISO),
        &hash(b"over"),
        4,
        &AtomicBool::new(false),
        &mut |_| {}
    )
    .is_err());
    assert_eq!(std::fs::read(folder.path().join(ISO)).unwrap(), b"old");
    assert_eq!(std::fs::read_dir(folder.path()).unwrap().count(), 1);
}
#[test]
fn copy_requires_mount_root_and_verified_source() {
    let folder = tempfile::tempdir().unwrap();
    std::fs::write(folder.path().join(ISO), b"source").unwrap();
    std::fs::write(
        folder.path().join(SUMS),
        format!("{}  {ISO}\n", hash(b"source")),
    )
    .unwrap();
    assert!(!is_mount_root(folder.path()));
    assert!(copy_to_ventoy(
        &folder.path().join(ISO),
        folder.path(),
        &AtomicBool::new(false),
        &mut |_| {}
    )
    .is_err());
}
#[test]
fn usb_launchers_accept_only_expected_platform_filenames() {
    use std::path::Path;
    assert!(valid_official_executable(
        "Ventoy",
        Path::new("Ventoy2Disk.exe"),
        true
    ));
    assert!(valid_official_executable(
        "Ventoy",
        Path::new("VentoyGUI.x86_64"),
        false
    ));
    assert!(valid_official_executable(
        "Rufus",
        Path::new("rufus-4.6p.exe"),
        true
    ));
    for name in ["cmd.exe", "rufus-.exe", "rufus-x.exe", "rufus.exe.ps1"] {
        assert!(!valid_official_executable("Rufus", Path::new(name), true));
    }
    assert!(!valid_official_executable(
        "Rufus",
        Path::new("rufus.exe"),
        false
    ));
}

#[test]
fn actual_release_directory_reconstructs_through_the_real_backend() {
    let directory = tempfile::tempdir().unwrap();
    let (release, fixtures, expected) = multipart();
    for asset in release.assets {
        std::fs::write(
            directory.path().join(asset.name),
            &fixtures.0[&asset.browser_download_url],
        )
        .unwrap();
    }
    let proof = verify_local_release_directory(directory.path(), &hash(&expected)).unwrap();
    assert_eq!(proof["sha256"], hash(&expected));
    assert_eq!(proof["size"], expected.len());
    assert!(verify_local_release_directory(directory.path(), &"a".repeat(64)).is_err());
}
