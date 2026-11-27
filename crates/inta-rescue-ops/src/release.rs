use crate::{sha256, Result};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::Path,
};
const ISO: &str = "inta-rescue-amd64.iso";
const SUMS: &str = "SHA256SUMS";
const PARTS: &str = "inta-rescue-amd64.iso.parts.json";
const HELP: &str = "REASSEMBLE.txt";
const INSTRUCTIONS:&str="This release preserves the original verified ISO in numbered pieces.\n\nOpen the Rust INTA Media Creator and choose Download / update ISO. It verifies every piece and the complete image, then saves inta-rescue-amd64.iso. Individual pieces are not bootable. Keep SHA256SUMS beside the completed ISO for verification and copying.\n\nThe .parts.json manifest records exact order, sizes and SHA-256 checksums. SHA256SUMS retains the original complete-image checksum.\n";
fn regular(path: &Path) -> Result<fs::Metadata> {
    let metadata = path.symlink_metadata()?;
    if !metadata.is_file() || metadata.len() == 0 {
        return Err(format!("Expected nonempty regular file: {}", path.display()).into());
    }
    Ok(metadata)
}
fn same_snapshot(a: &fs::Metadata, b: &fs::Metadata) -> bool {
    let basic = a.len() == b.len() && a.modified().ok() == b.modified().ok();
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        basic && a.ino() == b.ino() && a.dev() == b.dev()
    }
    #[cfg(not(unix))]
    {
        basic
    }
}
fn checksums(path: &Path) -> Result<BTreeMap<String, String>> {
    if regular(path)?.len() > 1024 * 1024 {
        return Err("Input SHA256SUMS is unexpectedly large".into());
    }
    let mut result = BTreeMap::new();
    for line in fs::read_to_string(path)?
        .lines()
        .filter(|s| !s.trim().is_empty())
    {
        let fields: Vec<_> = line.split_whitespace().collect();
        if fields.len() != 2 {
            return Err("Invalid SHA256SUMS entry".into());
        }
        let digest = fields[0];
        let name = fields[1].trim_start_matches('*');
        if digest.len() != 64
            || !digest.bytes().all(|c| c.is_ascii_hexdigit())
            || name.is_empty()
            || !name.as_bytes()[0].is_ascii_alphanumeric()
            || !name
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || b"._-".contains(&c))
            || result.contains_key(name)
        {
            return Err("Invalid or duplicate SHA256SUMS entry".into());
        }
        result.insert(name.to_string(), digest.to_ascii_lowercase());
    }
    if !result.contains_key(ISO) {
        return Err("Missing original ISO checksum".into());
    }
    Ok(result)
}
fn finish(path: &Path, expected: &str, size: u64) -> Result<Value> {
    if path.metadata()?.len() != size || sha256(path)? != expected {
        return Err(format!("Written asset failed verification: {}", path.display()).into());
    }
    Ok(json!({"name":path.file_name().unwrap().to_string_lossy(),"size":size,"sha256":expected}))
}
fn copy(source: &Path, target: &Path, expected: Option<&String>) -> Result<Value> {
    let before = regular(source)?;
    let mut incoming = File::open(source)?;
    let mut outgoing = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(target)?;
    let mut digest = Sha256::new();
    let mut total = 0u64;
    let mut buffer = vec![0; 1024 * 1024];
    loop {
        let n = incoming.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        outgoing.write_all(&buffer[..n])?;
        digest.update(&buffer[..n]);
        total += n as u64;
    }
    outgoing.flush()?;
    outgoing.sync_all()?;
    let digest = format!("{:x}", digest.finalize());
    if !same_snapshot(&before, &source.metadata()?)
        || total != before.len()
        || expected.is_some_and(|v| v != &digest)
    {
        return Err("Source changed or its checksum did not match".into());
    }
    fs::set_permissions(target, before.permissions())?;
    finish(target, &digest, total)
}
fn write(target: &Path, bytes: &[u8]) -> Result<Value> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(target)?;
    file.write_all(bytes)?;
    file.flush()?;
    file.sync_all()?;
    finish(
        target,
        &format!("{:x}", Sha256::digest(bytes)),
        bytes.len() as u64,
    )
}
fn split(source: &Path, staging: &Path, expected: &str, part_size: u64) -> Result<Value> {
    let before = regular(source)?;
    let mut incoming = File::open(source)?;
    let mut full = Sha256::new();
    let mut total = 0u64;
    let mut parts = Vec::new();
    let mut buffer = vec![0; 1024 * 1024];
    while total < before.len() {
        let name = format!("{ISO}.part{:03}", parts.len() + 1);
        let path = staging.join(&name);
        let mut outgoing = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)?;
        let mut digest = Sha256::new();
        let mut size = 0u64;
        let expected_size = part_size.min(before.len() - total);
        while size < expected_size {
            let remaining = (expected_size - size).min(buffer.len() as u64) as usize;
            let n = incoming.read(&mut buffer[..remaining])?;
            if n == 0 {
                return Err("ISO ended before its recorded size".into());
            }
            outgoing.write_all(&buffer[..n])?;
            digest.update(&buffer[..n]);
            full.update(&buffer[..n]);
            size += n as u64;
            total += n as u64;
        }
        outgoing.flush()?;
        outgoing.sync_all()?;
        parts.push(finish(&path, &format!("{:x}", digest.finalize()), size)?);
    }
    let mut tail = [0];
    if incoming.read(&mut tail)? != 0
        || !same_snapshot(&before, &source.metadata()?)
        || format!("{:x}", full.finalize()) != expected
    {
        return Err(
            "The complete ISO differs from its original checksum or changed while packaging".into(),
        );
    }
    Ok(json!({"schemaVersion":1,"filename":ISO,"size":total,"sha256":expected,"parts":parts}))
}

pub fn package(input: &Path, output: &Path, part_size: u64, asset_limit: u64) -> Result<Value> {
    if part_size == 0
        || part_size > 1024 * 1024 * 1024
        || asset_limit == 0
        || asset_limit > 2 * 1024 * 1024 * 1024
        || part_size >= asset_limit
    {
        return Err("Part size must be positive, at most 1 GiB and below the asset limit".into());
    }
    let source = input.canonicalize()?;
    if output.try_exists()? {
        return Err("Output directory already exists; it was preserved".into());
    }
    let parent = output
        .parent()
        .ok_or("Output requires a parent directory")?;
    fs::create_dir_all(parent)?;
    let sums = checksums(&source.join(SUMS))?;
    let iso = regular(&source.join(ISO))?;
    let mut companions: Vec<_> = fs::read_dir(&source)?
        .filter_map(std::result::Result::ok)
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|name| name.starts_with("inta-media") || name == "inta-rescue-amd64.packages")
        .collect();
    companions.sort();
    if !companions.iter().any(|name| name.starts_with("inta-media")) {
        return Err("Include a compiled Rust Media Creator release artifact".into());
    }
    for name in &companions {
        if regular(&source.join(name))?.len() >= asset_limit {
            return Err("Companion asset exceeds the release size limit".into());
        }
        if name.starts_with("inta-media") && !sums.contains_key(name) {
            return Err("Compiled Media Creator requires an input SHA-256 record".into());
        }
    }
    let split_needed = iso.len() >= asset_limit;
    let count = if split_needed {
        iso.len().div_ceil(part_size)
    } else {
        0
    };
    if companions.len() as u64 + count + 4 > 1000 {
        return Err("Release would exceed 1000 assets".into());
    }
    let staging = tempfile::tempdir_in(parent)?;
    let mut assets = Vec::new();
    for name in &companions {
        assets.push(copy(
            &source.join(name),
            &staging.path().join(name),
            sums.get(name),
        )?);
    }
    if split_needed {
        let manifest = split(&source.join(ISO), staging.path(), &sums[ISO], part_size)?;
        assets.extend(manifest["parts"].as_array().unwrap().iter().cloned());
        assets.push(write(
            &staging.path().join(PARTS),
            &serde_json::to_vec_pretty(&manifest)?,
        )?);
        assets.push(write(&staging.path().join(HELP), INSTRUCTIONS.as_bytes())?);
    } else {
        assets.push(copy(
            &source.join(ISO),
            &staging.path().join(ISO),
            sums.get(ISO),
        )?);
    }
    let mut hashes: BTreeMap<String, String> = assets
        .iter()
        .map(|a| {
            (
                a["name"].as_str().unwrap().to_string(),
                a["sha256"].as_str().unwrap().to_string(),
            )
        })
        .collect();
    hashes.insert(ISO.into(), sums[ISO].clone());
    let contents: String = hashes
        .iter()
        .map(|(name, digest)| format!("{digest}  {name}\n"))
        .collect();
    assets.push(write(&staging.path().join(SUMS), contents.as_bytes())?);
    if assets.iter().any(|a| {
        a["size"].as_u64().unwrap_or(0) == 0 || a["size"].as_u64().unwrap_or(0) >= asset_limit
    }) {
        return Err("Prepared release asset exceeds the publication limit".into());
    }
    if output.try_exists()? {
        return Err("Output appeared during packaging; preserved it".into());
    }
    fs::rename(staging.path(), output)?;
    Ok(
        json!({"output_dir":output,"iso":{"name":ISO,"size":iso.len(),"sha256":sums[ISO]},"split":split_needed,"part_count":count,"assets":assets}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture(folder: &Path, size: usize) {
        fs::write(folder.join(ISO), vec![0x5a; size]).unwrap();
        fs::write(folder.join("inta-media-linux-x64"), b"native-media-fixture").unwrap();
        fs::write(
            folder.join(SUMS),
            format!(
                "{}  {ISO}\n{}  inta-media-linux-x64\n",
                sha256(&folder.join(ISO)).unwrap(),
                sha256(&folder.join("inta-media-linux-x64")).unwrap()
            ),
        )
        .unwrap();
    }
    #[test]
    fn split_keeps_exact_original_bytes_and_hash() {
        let input = tempfile::tempdir().unwrap();
        let root = tempfile::tempdir().unwrap();
        fixture(input.path(), 4500);
        let destination = root.path().join("release");
        let report = package(input.path(), &destination, 1000, 4000).unwrap();
        assert!(report["split"].as_bool().unwrap());
        let mut reassembled = Vec::new();
        let manifest: Value =
            serde_json::from_slice(&fs::read(destination.join(PARTS)).unwrap()).unwrap();
        for part in manifest["parts"].as_array().unwrap() {
            let bytes = fs::read(destination.join(part["name"].as_str().unwrap())).unwrap();
            assert_eq!(
                format!("{:x}", Sha256::digest(&bytes)),
                part["sha256"].as_str().unwrap()
            );
            reassembled.extend(bytes);
        }
        assert_eq!(reassembled, fs::read(input.path().join(ISO)).unwrap());
        assert!(fs::read_to_string(destination.join(SUMS))
            .unwrap()
            .contains(&sha256(&input.path().join(ISO)).unwrap()));
    }
    #[test]
    fn corrupt_source_and_existing_destination_are_never_published() {
        let input = tempfile::tempdir().unwrap();
        let root = tempfile::tempdir().unwrap();
        fixture(input.path(), 500);
        fs::write(input.path().join(ISO), b"corrupt").unwrap();
        let destination = root.path().join("release");
        assert!(package(input.path(), &destination, 1000, 4000).is_err());
        assert!(!destination.exists());
        fs::create_dir(&destination).unwrap();
        fs::write(destination.join("keep"), b"keep").unwrap();
        assert!(package(input.path(), &destination, 1000, 4000).is_err());
        assert_eq!(fs::read(destination.join("keep")).unwrap(), b"keep");
    }
    #[test]
    fn ordinary_image_stays_unsplit() {
        let input = tempfile::tempdir().unwrap();
        let root = tempfile::tempdir().unwrap();
        fixture(input.path(), 800);
        let destination = root.path().join("release");
        assert_eq!(
            package(input.path(), &destination, 1000, 4000).unwrap()["split"],
            false
        );
        assert_eq!(
            sha256(&destination.join(ISO)).unwrap(),
            sha256(&input.path().join(ISO)).unwrap()
        );
        assert!(!destination.join(PARTS).exists());
    }
}
