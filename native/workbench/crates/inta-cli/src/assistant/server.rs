use anyhow::{Context, Result, bail, ensure};
use rusqlite::{Connection, OptionalExtension};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::{self, ErrorKind},
    net::{Ipv4Addr, SocketAddr, TcpStream},
    os::windows::fs::{MetadataExt, OpenOptionsExt},
    path::{Component, Path, PathBuf, Prefix},
    time::Duration,
};
use windows_sys::Win32::Storage::FileSystem::{
    FILE_ATTRIBUTE_REPARSE_POINT, FILE_FLAG_OPEN_REPARSE_POINT, FILE_SHARE_READ, GetDriveTypeW,
};

const KEY: &str = "assistant/verified-store-v1";
const MODEL: &str = "qwen2.5:0.5b";
const MANIFEST: &str = "a8b0c51577010a279d933d14c2a8ab4b268079d44c5c8830c0a93900f1827c67";
fn stored(c: &Connection) -> Result<Option<PathBuf>> {
    let s: Option<String> = c
        .query_row("SELECT value FROM metadata WHERE key=?", [KEY], |r| {
            r.get(0)
        })
        .optional()?;
    Ok(s.map(PathBuf::from))
}
fn local_path(path: &Path) -> Result<()> {
    ensure!(
        path.is_absolute()
            && !path
                .components()
                .any(|c| matches!(c, Component::ParentDir | Component::CurDir)),
        "Choose an absolute, local model store"
    );
    let Some(Component::Prefix(prefix)) = path.components().next() else {
        bail!("Choose a local drive")
    };
    let Prefix::Disk(drive) = prefix.kind() else {
        bail!("Choose a local drive letter")
    };
    let root = super::wide(&format!("{}:\\", char::from(drive)));
    // GetDriveTypeW reports 2 (removable) or 3 (fixed) for local media.
    ensure!(
        matches!(unsafe { GetDriveTypeW(root.as_ptr()) }, 2 | 3),
        "Choose a local model store on a fixed or removable drive"
    );
    for p in path.ancestors() {
        ensure!(
            fs::symlink_metadata(p)?.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT == 0,
            "A linked model-store path cannot be prepared"
        )
    }
    Ok(())
}
fn regular(path: &Path, expected: u64) -> Result<File> {
    local_path(path)?;
    let file = OpenOptions::new()
        .read(true)
        .share_mode(FILE_SHARE_READ)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
        .open(path)?;
    let meta = file.metadata()?;
    ensure!(
        meta.is_file()
            && meta.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT == 0
            && meta.len() == expected,
        "Model-store file has unexpected type or length"
    );
    Ok(file)
}
fn inspect(path: &Path) -> Result<Value> {
    local_path(path)?;
    let manifest_path = path.join("manifests/registry.ollama.ai/library/qwen2.5/0.5b");
    let mut file = regular(&manifest_path, 857)
        .context("The reviewed Qwen2.5:0.5b manifest is missing or invalid")?;
    let mut bytes = Vec::new();
    io::copy(&mut file, &mut bytes)?;
    ensure!(
        format!("{:x}", Sha256::digest(&bytes)) == MANIFEST,
        "The model manifest differs from the reviewed upstream release"
    );
    let manifest: Value = serde_json::from_slice(&bytes)?;
    let layers = manifest["layers"]
        .as_array()
        .context("Invalid reviewed model layers")?;
    let mut total = 0u64;
    for descriptor in std::iter::once(&manifest["config"]).chain(layers) {
        let sha = descriptor["digest"]
            .as_str()
            .and_then(|s| s.strip_prefix("sha256:"))
            .filter(|s| super::valid_digest(s))
            .context("Invalid manifest digest")?;
        let size = descriptor["size"].as_u64().context("Invalid layer size")?;
        let mut blob = regular(&path.join("blobs").join(format!("sha256-{sha}")), size)?;
        let mut hasher = Sha256::new();
        io::copy(&mut blob, &mut hasher)?;
        ensure!(
            format!("{:x}", hasher.finalize()) == sha,
            "A local model layer failed SHA-256 verification"
        );
        total += size;
    }
    Ok(
        json!({"store":path,"model":MODEL,"digest":MANIFEST,"verifiedBytes":total,"license":"Apache-2.0","source":"https://ollama.com/library/qwen2.5:0.5b"}),
    )
}
fn listening() -> Result<bool> {
    let endpoint = SocketAddr::from((Ipv4Addr::LOCALHOST, super::PORT));
    match TcpStream::connect_timeout(&endpoint, Duration::from_secs(3)) {
        Ok(_) => Ok(true),
        Err(e) if e.kind() == ErrorKind::ConnectionRefused => Ok(false),
        Err(e) => Err(e).context("Could not establish that the local Ollama port is free"),
    }
}
pub fn prepare(c: &Connection, path: &Path) -> Result<Value> {
    let receipt = inspect(path)?;
    c.execute(
        "INSERT OR REPLACE INTO metadata(key,value) VALUES(?1,?2)",
        [KEY, path.to_str().context("Model path is not UTF-8")?],
    )?;
    Ok(receipt)
}
pub fn status(c: &Connection) -> Result<Value> {
    let store = stored(c)?;
    Ok(
        json!({"listening":listening()?,"configured":store.is_some(),"store":store,"model":MODEL,"endpoint":"http://127.0.0.1:11434"}),
    )
}
pub fn plan(c: &Connection) -> Result<Value> {
    ensure!(
        !listening()?,
        "A service already owns the local Ollama endpoint"
    );
    let store = stored(c)?.context("Prepare the verified Qwen2.5:0.5b model store first")?;
    let mut receipt = inspect(&store)?;
    ensure!(
        !listening()?,
        "Another service opened the Ollama endpoint during verification"
    );
    receipt["endpoint"] = json!("http://127.0.0.1:11434");
    Ok(receipt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unverified_store_cannot_replace_an_existing_configuration() {
        let c = Connection::open_in_memory().unwrap();
        c.execute_batch("CREATE TABLE metadata(key TEXT PRIMARY KEY,value TEXT NOT NULL)")
            .unwrap();
        c.execute(
            "INSERT INTO metadata VALUES(?1,?2)",
            [KEY, "previous-setting"],
        )
        .unwrap();
        let fixture =
            tempfile::tempdir_in(std::env::current_exe().unwrap().parent().unwrap()).unwrap();
        let model = fixture
            .path()
            .join("manifests/registry.ollama.ai/library/qwen2.5");
        fs::create_dir_all(&model).unwrap();
        fs::write(model.join("0.5b"), vec![b' '; 857]).unwrap();
        assert!(
            prepare(&c, fixture.path())
                .unwrap_err()
                .to_string()
                .contains("manifest differs")
        );
        assert_eq!(stored(&c).unwrap(), Some(PathBuf::from("previous-setting")));
        assert!(inspect(Path::new(r"\\server\models")).is_err());
        assert!(inspect(Path::new("relative")).is_err());
    }
}
