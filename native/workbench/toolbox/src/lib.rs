mod archive;
mod backup;
pub use backup::create_backup;
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File, OpenOptions},
    io::{self, Read, Seek},
    path::{Component, Path, PathBuf},
    process::{Command, Stdio},
    time::{Duration, Instant},
};

const PINS: &str = include_str!("../pins.json");
const MARKER: &str = ".inta-toolbox.json";
const INDEX: &str = ".inta-packages.json";
type Answer = Result<Value, String>;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Pin {
    id: String,
    name: String,
    version: String,
    homepage: String,
    license: String,
    exe_sha256: String,
    executable: String,
    probe: Vec<String>,
    architecture: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Marker {
    schema_version: u32,
    toolbox_id: String,
    display_name: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Index {
    schema_version: u32,
    toolbox_id: String,
    packages: BTreeMap<String, String>,
}
fn err(e: impl std::fmt::Display) -> String {
    e.to_string()
}
fn pins() -> Result<Vec<Pin>, String> {
    serde_json::from_str(PINS).map_err(err)
}
fn linked(meta: &fs::Metadata) -> bool {
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        meta.file_attributes() & 0x400 != 0
    }
    #[cfg(not(windows))]
    meta.file_type().is_symlink()
}
fn checked_root(root: &Path) -> Result<PathBuf, String> {
    if !root.is_absolute()
        || root
            .components()
            .any(|c| matches!(c, Component::ParentDir | Component::CurDir))
    {
        return Err("Select an absolute local toolbox path".into());
    }
    #[cfg(windows)]
    if !matches!(root.components().next(), Some(Component::Prefix(prefix)) if matches!(prefix.kind(), std::path::Prefix::Disk(_)))
    {
        return Err("Select a local drive path".into());
    }
    for part in root.ancestors().filter(|p| !p.as_os_str().is_empty()) {
        let meta = fs::symlink_metadata(part).map_err(err)?;
        if linked(&meta) || !meta.is_dir() {
            return Err("Toolbox directories cannot be links or reparse points".into());
        }
    }
    Ok(root.to_owned())
}
fn inside(root: &Path, relative: &str) -> Result<PathBuf, String> {
    let mut target = root.to_owned();
    if relative.is_empty() {
        return Err("Missing relative path".into());
    }
    for piece in relative.split(['/', '\\']) {
        let stem = piece.split('.').next().unwrap_or_default().to_uppercase();
        let reserved = matches!(
            stem.as_str(),
            "CON" | "AUX" | "PRN" | "NUL" | "CONIN$" | "CONOUT$"
        ) || ["COM", "LPT"].iter().any(|prefix| {
            stem.strip_prefix(prefix).is_some_and(|n| {
                matches!(
                    n,
                    "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "¹" | "²" | "³"
                )
            })
        });
        if reserved
            || piece
                .chars()
                .any(|ch| ch.is_control() || matches!(ch, '<' | '>' | '"' | '|' | '?' | '*'))
            || piece.is_empty()
            || piece == "."
            || piece == ".."
            || piece.contains([':', '\0'])
            || piece.ends_with([' ', '.'])
        {
            return Err("Invalid relative toolbox path".into());
        }
        let p = Path::new(piece);
        if !matches!(p.components().next(), Some(Component::Normal(_))) {
            return Err("Expected a normal relative path".into());
        }
        target.push(p);
        match fs::symlink_metadata(&target) {
            Ok(meta) if linked(&meta) => {
                return Err("Toolbox path contains a link or reparse point".into())
            }
            Ok(_) => {}
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(err(e)),
        }
    }
    Ok(target)
}
fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, String> {
    let file = File::open(path).map_err(err)?;
    let mut bytes = Vec::new();
    file.take(128 * 1024 + 1)
        .read_to_end(&mut bytes)
        .map_err(err)?;
    if bytes.len() > 128 * 1024 {
        return Err("Toolbox metadata exceeds size limit".into());
    }
    serde_json::from_slice(&bytes).map_err(err)
}
fn marker(root: &Path) -> Result<Marker, String> {
    let m: Marker = read_json(&inside(root, MARKER)?)?;
    let id = m.toolbox_id.as_bytes();
    if m.schema_version != 1
        || id.len() != 36
        || !id.iter().enumerate().all(|(i, b)| {
            if [8, 13, 18, 23].contains(&i) {
                *b == b'-'
            } else {
                b.is_ascii_hexdigit()
            }
        })
        || m.display_name.is_empty()
        || m.display_name.chars().any(char::is_control)
    {
        return Err("Invalid toolbox identity or unsupported schema version".into());
    }
    Ok(m)
}
fn open_root(root: &Path) -> Result<(Marker, Index), String> {
    let root = checked_root(root)?;
    let m = marker(&root)?;
    let index: Index = read_json(&inside(&root, INDEX)?)?;
    if index.schema_version != 1 || m.toolbox_id != index.toolbox_id {
        return Err("Index identity differs from toolbox marker; reindex this root".into());
    }
    Ok((m, index))
}
pub fn list(root: &Path) -> Answer {
    let (m, index) = open_root(root)?;
    let mut packages = vec![];
    for pin in pins()? {
        let relative = index.packages.get(&pin.id);
        let state = match relative {
            Some(s) => match inside(root, s) {
                Ok(p) if p.is_file() => "present",
                Ok(_) => "missing",
                Err(_) => "blocked",
            },
            None => "missing",
        };
        packages.push(json!({"id":pin.id,"name":pin.name,"version":pin.version,"homepage":pin.homepage,"license":pin.license,
            "architecture":pin.architecture,"relativePath":relative,"state":state}));
    }
    Ok(
        json!({"schemaVersion":1,"toolboxId":m.toolbox_id,"displayName":m.display_name,"packages":packages}),
    )
}
fn open_verified(root: &Path, id: &str) -> Result<(Pin, File), String> {
    let (_, idx) = open_root(root)?;
    let pin = pins()?
        .into_iter()
        .find(|p| p.id == id)
        .ok_or("Unknown tool ID")?;
    let rel = idx
        .packages
        .get(id)
        .ok_or("Package has not been acquired")?;
    let path = inside(root, rel)?;
    let mut opts = OpenOptions::new();
    opts.read(true);
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        opts.share_mode(1);
    }
    let mut file = opts.open(path).map_err(err)?;
    if !file.metadata().map_err(err)?.is_file() {
        return Err("Expected a regular file".into());
    }
    if file.metadata().map_err(err)?.len() > 256 * 1024 * 1024 {
        return Err("Package exceeds size limit".into());
    }
    let mut sha = Sha256::new();
    io::copy(&mut file, &mut sha).map_err(err)?;
    if format!("{:x}", sha.finalize()) != pin.exe_sha256 {
        return Err(format!("SHA-256 mismatch for {id}; acquisition required"));
    }
    file.rewind().map_err(err)?;
    Ok((pin, file))
}
pub fn verify(root: &Path, id: &str) -> Answer {
    let (pin, _) = open_verified(root, id)?;
    Ok(json!({"id":id,"version":pin.version,"sha256":pin.exe_sha256,"state":"verified"}))
}
fn stage_verified(
    root: &Path,
    id: &str,
) -> Result<(Pin, tempfile::TempDir, PathBuf, File), String> {
    let (pin, mut source) = open_verified(root, id)?;
    let work = tempfile::Builder::new()
        .prefix("inta-worker-")
        .tempdir()
        .map_err(err)?;
    let exe = work.path().join(&pin.executable);
    {
        let mut copy = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&exe)
            .map_err(err)?;
        io::copy(&mut source, &mut copy).map_err(err)?;
        copy.sync_all().map_err(err)?;
    }
    let mut opts = OpenOptions::new();
    opts.read(true);
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        opts.share_mode(1);
    }
    let mut sealed = opts.open(&exe).map_err(err)?;
    let mut hash = Sha256::new();
    io::copy(&mut sealed, &mut hash).map_err(err)?;
    if format!("{:x}", hash.finalize()) != pin.exe_sha256 {
        return Err("Launch copy integrity failed".into());
    }
    Ok((pin, work, exe, sealed))
}

/// Run a pinned version probe in an empty directory, isolated from toolbox DLLs.
pub fn probe(root: &Path, id: &str) -> Answer {
    if !cfg!(all(target_os = "windows", target_arch = "x86_64")) {
        return Err("Probes require x64 Windows".into());
    }
    let (pin, work, exe, _sealed) = stage_verified(root, id)?;
    let mut cmd = Command::new(&exe);
    cmd.args(&pin.probe)
        .current_dir(work.path())
        .env_clear()
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    if let Some(windows) = std::env::var_os("SystemRoot") {
        cmd.env("SystemRoot", &windows)
            .env("PATH", Path::new(&windows).join("System32"));
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x08000000);
    }
    let mut child = cmd.spawn().map_err(err)?;
    let mut pipe = child.stdout.take().ok_or("Missing output pipe")?;
    let output = std::thread::spawn(move || {
        let mut kept = Vec::new();
        let mut buf = [0u8; 4096];
        loop {
            let n = pipe.read(&mut buf)?;
            if n == 0 {
                break;
            }
            kept.extend_from_slice(&buf[..n.min(8192 - kept.len())]);
        }
        Ok::<_, io::Error>(kept)
    });
    let start = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if start.elapsed() < Duration::from_secs(12) => {
                std::thread::sleep(Duration::from_millis(30))
            }
            other => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(other
                    .err()
                    .map(err)
                    .unwrap_or_else(|| "Probe timed out".into()));
            }
        }
    };
    let bytes = output
        .join()
        .map_err(|_| "Output reader failed")?
        .map_err(err)?;
    if !status.success() {
        return Err(format!("Verified probe exited with {status}"));
    }
    Ok(
        json!({"id":id,"version":pin.version,"state":"tested","sha256":pin.exe_sha256,"exitCode":status.code(),"output":String::from_utf8_lossy(&bytes).trim()}),
    )
}

/// Verify every stream in a selected .7z archive. File names and contents stay out of the result.
pub fn inventory_archive(root: &Path, source: &Path) -> Answer {
    if !cfg!(all(target_os = "windows", target_arch = "x86_64")) {
        return Err("Archive test requires x64 Windows".into());
    }
    let start = Instant::now();
    let parent = source.parent().ok_or("Select a local .7z file")?;
    checked_root(parent)?;
    let name = source
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or("Invalid archive name")?;
    let path = inside(parent, name)?;
    if !path
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("7z"))
    {
        return Err("Select a single-volume .7z archive".into());
    }
    let mut opt = OpenOptions::new();
    opt.read(true);
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        opt.share_mode(1);
    }
    let mut file = opt.open(path).map_err(err)?;
    let m = file.metadata().map_err(err)?;
    let limit = 256 * 1024 * 1024;
    if !m.is_file() || m.len() > limit || m.len() < 32 {
        return Err("Select a regular .7z file between 32 bytes and 256 MiB".into());
    }
    let (pin, work, exe, _sealed) = stage_verified(root, "7zip")?;
    let snapshot = work.path().join("input.7z");
    let mut dest = File::create_new(&snapshot).map_err(err)?;
    let actual = io::copy(&mut (&mut file).take(limit + 1), &mut dest).map_err(err)?;
    if actual != m.len() {
        return Err("Archive changed during snapshot".into());
    }
    drop(dest);
    let mut locked = opt.open(&snapshot).map_err(err)?;
    let mut sha = Sha256::new();
    io::copy(&mut locked, &mut sha).map_err(err)?;
    let code = archive::test(&exe, work.path())?;
    Ok(
        json!({"state":"verified","sizeBytes":actual,"sha256":format!("{:x}",sha.finalize()),"engine":"7zip","engineVersion":pin.version,"exitCode":code,"durationMs":start.elapsed().as_millis()}),
    )
}
/// Bounded, on-demand inspection of explicit locations and direct children. Return every candidate for operator selection.
pub fn discover(anchors: &[PathBuf]) -> Answer {
    let mut paths = BTreeSet::new();
    for anchor in anchors.iter().take(64) {
        if checked_root(anchor).is_err() {
            continue;
        }
        paths.insert(anchor.clone());
        if let Ok(entries) = fs::read_dir(anchor) {
            for entry in entries.take(512).flatten() {
                if entry.file_type().is_ok_and(|t| t.is_dir()) {
                    paths.insert(entry.path());
                }
            }
        }
    }
    let mut roots = vec![];
    for p in paths {
        if p.join(MARKER).is_file() && checked_root(&p).is_ok() {
            if let Ok(m) = marker(&p) {
                roots.push(json!({"path":p,"toolboxId":m.toolbox_id,"displayName":m.display_name}));
            }
        }
    }
    Ok(json!({"schemaVersion":1,"roots":roots}))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_escapes_streams_and_windows_names_with_trailing_dots() {
        let dir = tempfile::tempdir().unwrap();
        for bad in [
            "../file",
            "a//b",
            "/root",
            "C:\\foo",
            "x:stream",
            "thing./ok",
            "a/./b",
        ] {
            assert!(inside(dir.path(), bad).is_err(), "{bad}");
        }
    }
    #[test]
    fn mismatched_identity_and_tamper_are_rejected() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let id = "a1234567-0123-4567-89ab-0123456789ab";
        fs::write(
            root.join(MARKER),
            json!({"schemaVersion":1,"displayName":"Local","toolboxId":id}).to_string(),
        )
        .unwrap();
        fs::write(
            root.join(INDEX),
            json!({"schemaVersion":1,"toolboxId":"other","packages":{}}).to_string(),
        )
        .unwrap();
        assert!(list(root).is_err());
        fs::write(
            root.join(INDEX),
            json!({"schemaVersion":1,"toolboxId":id,"packages":{"7zip":"file.exe"}}).to_string(),
        )
        .unwrap();
        fs::write(root.join("file.exe"), b"bad").unwrap();
        assert!(verify(root, "7zip").unwrap_err().contains("mismatch"));
    }
}
