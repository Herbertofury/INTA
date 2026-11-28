//! Reversible, per-file quarantine on a local Windows volume. SQLite stores the intent
//! before a no-replace handle rename and the observed outcome afterwards.
use crate::tool_actions::hold_local_directories as directories;
use anyhow::{Context, Result, bail, ensure};
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::HashSet,
    fs::{self, File, OpenOptions},
    io::{self, Read, Seek},
    os::windows::{
        ffi::OsStrExt,
        fs::{MetadataExt, OpenOptionsExt},
        io::AsRawHandle,
    },
    path::{Component, Path, PathBuf},
    time::{Duration, Instant},
};
use windows_sys::Win32::Storage::FileSystem::*;

const MAX_FILE: u64 = 32 * 1024 * 1024;
const HASH_BUDGET: u64 = 128 * 1024 * 1024;
const MAX_ITEMS: usize = 256;
const MAX_ENTRIES: usize = 15000;
const CLOUD: u32 = FILE_ATTRIBUTE_REPARSE_POINT
    | FILE_ATTRIBUTE_OFFLINE
    | FILE_ATTRIBUTE_RECALL_ON_OPEN
    | FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS;
const TAG: &[u8] = b"Signature: 8a477f597d28d172789f06886806bc55";
fn uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}
fn valid_id(id: &str) -> bool {
    uuid::Uuid::parse_str(id).is_ok_and(|v| v.to_string() == id)
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Item {
    id: String,
    path: PathBuf,
    rule: String,
    bytes: u64,
    sha256: String,
    identity: String,
    parent_identity: String,
    modified: u64,
    state: String,
    detail: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Review {
    id: String,
    root: PathBuf,
    root_identity: String,
    created: String,
    examined: usize,
    excluded: usize,
    bounded: bool,
    files: Vec<Item>,
    vault: Option<String>,
    vault_identity: Option<String>,
}
fn identity(f: &File) -> Result<(String, u32)> {
    let mut i = BY_HANDLE_FILE_INFORMATION::default();
    ensure!(
        unsafe { GetFileInformationByHandle(f.as_raw_handle(), &mut i) } != 0,
        "File identity unavailable"
    );
    let index = (u64::from(i.nFileIndexHigh) << 32) | u64::from(i.nFileIndexLow);
    ensure!(index != 0, "The filesystem provides no stable file ID");
    Ok((
        format!("{:08x}-{index:016x}", i.dwVolumeSerialNumber),
        i.nNumberOfLinks,
    ))
}
fn checked_dirs(dir: &Path) -> Result<Vec<File>> {
    let chain = directories(dir, false)?;
    for f in &chain {
        ensure!(
            f.metadata()?.file_attributes() & CLOUD == 0,
            "Cloud or linked path excluded"
        );
    }
    Ok(chain)
}
fn sensitive(path: &Path) -> bool {
    let blocked = [
        "documents",
        "desktop",
        "downloads",
        "pictures",
        "videos",
        "music",
        "clients",
        "customers",
        "credentials",
        "credential manager",
        "protect",
        "user data",
        "keychains",
        "browser profiles",
        ".password-store",
        ".cargo",
        ".nuget",
        ".codex",
        ".claude",
        "secrets",
        "passwords",
        "backups",
        "backup",
        "model",
        "models",
        "blobs",
        "huggingface",
        "ollama",
        ".ollama",
        ".ssh",
        ".aws",
        ".azure",
        ".gnupg",
        ".kube",
        "keyrings",
        "vault",
        "vaults",
        "appdata credentials",
        "wallets",
        ".git",
        "windowsapps",
        "program files",
        "program files (x86)",
        "system volume information",
        "$recycle.bin",
    ];
    path.components().enumerate().any(|(pos, p)| {
        let Component::Normal(s) = p else {
            return false;
        };
        let n = s.to_string_lossy().to_lowercase();
        blocked.contains(&n.as_str())
            || n.starts_with(".inta-quarantine-")
            || n.starts_with("onedrive")
            || n.starts_with("dropbox")
            || n.starts_with("icloud")
            || n == "google drive"
            || n.starts_with("client ")
            || n.starts_with("customer ")
            || (pos == 2 && n == "windows")
    })
}
fn conservative_name(n: &str) -> bool {
    ![
        "secret",
        "credential",
        "password",
        "token",
        "wallet",
        "private",
        "customer",
        "client",
        "backup",
        "model",
    ]
    .iter()
    .any(|s| n.to_ascii_lowercase().contains(s))
}
// Reads of package markers are exclusive and never follow a reparse point or hydrate a file.
fn open_regular(p: &Path, move_access: bool) -> Result<File> {
    let attrs = fs::symlink_metadata(p)?.file_attributes();
    ensure!(
        attrs
            & (CLOUD | FILE_ATTRIBUTE_ENCRYPTED | FILE_ATTRIBUTE_SYSTEM | FILE_ATTRIBUTE_READONLY)
            == 0,
        "Protected, linked or cloud file"
    );
    let access = GENERIC_READ_ACCESS | if move_access { DELETE } else { 0 };
    let f = OpenOptions::new()
        .access_mode(access)
        .share_mode(0)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT | FILE_FLAG_OPEN_NO_RECALL)
        .open(p)?;
    let m = f.metadata()?;
    ensure!(
        m.is_file()
            && m.file_attributes()
                & (CLOUD
                    | FILE_ATTRIBUTE_ENCRYPTED
                    | FILE_ATTRIBUTE_SYSTEM
                    | FILE_ATTRIBUTE_READONLY)
                == 0,
        "Protected file"
    );
    ensure!(identity(&f)?.1 == 1, "Hard-linked file excluded");
    Ok(f)
}
const GENERIC_READ_ACCESS: u32 = 0x80000000;
fn only_default_stream(f: &File) -> Result<()> {
    let mut buffer = [0u64; 8192];
    let ok = unsafe {
        GetFileInformationByHandleEx(
            f.as_raw_handle(),
            FileStreamInfo,
            buffer.as_mut_ptr().cast(),
            size_of_val(&buffer) as u32,
        )
    };
    ensure!(ok != 0, "Cannot inspect Windows data streams");
    let p = buffer.as_ptr().cast::<FILE_STREAM_INFO>();
    let (next, size) = unsafe { ((*p).NextEntryOffset, (*p).StreamNameLength as usize) };
    ensure!(next == 0 && size == 14, "Named streams are excluded");
    let name = unsafe {
        std::slice::from_raw_parts(std::ptr::addr_of!((*p).StreamName).cast::<u16>(), size / 2)
    };
    ensure!(
        name.iter().copied().eq("::$DATA".encode_utf16()),
        "Named streams are excluded"
    );
    Ok(())
}
fn marker(dir: &Path) -> bool {
    let Ok(mut f) = open_regular(&dir.join("CACHEDIR.TAG"), false) else {
        return false;
    };
    let mut b = [0u8; 43];
    f.read_exact(&mut b).is_ok() && b == TAG
}
fn class(p: &Path, f: &mut File) -> Option<&'static str> {
    let n = p.file_name()?.to_str()?;
    let d = p.parent()?;
    let parent = d.file_name()?.to_str()?;
    if !conservative_name(n) {
        return None;
    }
    if parent.eq_ignore_ascii_case("__pycache__") && n.ends_with(".pyc") {
        let (module, version) = n.strip_suffix(".pyc")?.split_once(".cpython-")?;
        let version = version
            .strip_suffix(".opt-1")
            .or_else(|| version.strip_suffix(".opt-2"))
            .unwrap_or(version);
        if module.is_empty()
            || !module
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || c == b'_')
            || !matches!(version.len(), 2 | 3)
            || !version.bytes().all(|b| b.is_ascii_digit())
        {
            return None;
        }
        let source = d.parent()?.join(format!("{module}.py"));
        let m = fs::symlink_metadata(source).ok()?;
        let mut magic = [0; 16];
        f.read_exact(&mut magic).ok()?;
        f.rewind().ok()?;
        return (m.is_file()
            && m.file_attributes() & CLOUD == 0
            && (3000..4000).contains(&u16::from_le_bytes([magic[0], magic[1]]))
            && magic[2..4] == [13, 10]
            && u32::from_le_bytes(magic[4..8].try_into().ok()?) <= 3)
            .then_some("Python bytecode with source present");
    }
    if matches!(n, "nodeids" | "lastfailed" | "stepwise")
        && d.ends_with(".pytest_cache/v/cache")
        && marker(d.parent()?.parent()?)
    {
        return Some("pytest state in a declared cache");
    }
    let (stem, ext) = n.rsplit_once('.')?;
    if matches!(ext, "tmp" | "cache")
        && (16..=64).contains(&stem.len())
        && stem.bytes().all(|c| c.is_ascii_hexdigit())
        && marker(d)
    {
        return Some("Hash-named temporary file in a declared cache");
    }
    None
}
fn snapshot(f: &mut File) -> Result<(u64, u64, String, String)> {
    only_default_stream(f)?;
    let (id, links) = identity(f)?;
    let before = f.metadata()?;
    ensure!(
        links == 1 && before.len() <= MAX_FILE,
        "File exceeds review limits"
    );
    f.rewind()?;
    let mut hash = Sha256::new();
    let bytes = io::copy(&mut f.take(MAX_FILE + 1), &mut hash)?;
    let after = f.metadata()?;
    ensure!(
        bytes == before.len()
            && after.len() == bytes
            && before.last_write_time() == after.last_write_time(),
        "File changed during hash"
    );
    Ok((
        bytes,
        after.last_write_time(),
        id,
        format!("{:x}", hash.finalize()),
    ))
}
fn db(c: &Connection) -> Result<()> {
    c.execute_batch("CREATE TABLE IF NOT EXISTS cleanup_reviews(id TEXT PRIMARY KEY,created TEXT NOT NULL,root TEXT NOT NULL,manifest TEXT NOT NULL);
    CREATE TABLE IF NOT EXISTS cleanup_audit(seq INTEGER PRIMARY KEY,review TEXT NOT NULL,item TEXT NOT NULL,at TEXT NOT NULL,state TEXT NOT NULL,detail TEXT NOT NULL);")?;
    Ok(())
}
fn save(c: &Connection, r: &Review, event: Option<(&str, &str, &str)>) -> Result<()> {
    let tx = c.unchecked_transaction()?;
    tx.execute("INSERT INTO cleanup_reviews(id,created,root,manifest) VALUES(?,?,?,?) ON CONFLICT(id) DO UPDATE SET manifest=excluded.manifest",params![r.id,r.created,r.root.to_str(),serde_json::to_string(r)?])?;
    if let Some((item, state, note)) = event {
        tx.execute(
            "INSERT INTO cleanup_audit(review,item,at,state,detail) VALUES(?,?,?,?,?)",
            params![r.id, item, crate::db::now(), state, note],
        )?;
    }
    tx.commit()?;
    Ok(())
}
fn load(c: &Connection, id: &str) -> Result<Review> {
    ensure!(valid_id(id), "Invalid review identity");
    let raw: String = c
        .query_row(
            "SELECT manifest FROM cleanup_reviews WHERE id=?",
            [id],
            |r| r.get(0),
        )
        .context("Unknown cleanup review")?;
    ensure!(raw.len() < 4 * 1024 * 1024, "Invalid cleanup manifest");
    let r: Review = serde_json::from_str(&raw)?;
    ensure!(
        r.id == id && r.files.len() <= MAX_ITEMS,
        "Invalid cleanup review"
    );
    Ok(r)
}
fn candidate(path: &Path) -> bool {
    path.file_name().and_then(|s| s.to_str()).is_some_and(|s| {
        s.ends_with(".pyc")
            || s.ends_with(".tmp")
            || s.ends_with(".cache")
            || matches!(s, "nodeids" | "lastfailed" | "stepwise")
    })
}
fn plan(c: &Connection, root: &Path) -> Result<Value> {
    ensure!(
        root.parent().is_some() && !sensitive(root),
        "Choose a dedicated folder outside protected locations"
    );
    let root_handles = checked_dirs(root)?;
    let root_id = identity(root_handles.last().context("Missing folder identity")?)?.0;
    let mut r = Review {
        id: uuid(),
        root: root.to_owned(),
        root_identity: root_id,
        created: crate::db::now(),
        examined: 0,
        excluded: 0,
        bounded: false,
        files: vec![],
        vault: None,
        vault_identity: None,
    };
    let mut queue = vec![root.to_owned()];
    let mut budget = 0;
    let deadline = Instant::now() + Duration::from_secs(12);
    'scan: while let Some(dir) = queue.pop() {
        let Ok(dirs) = checked_dirs(&dir) else {
            r.excluded += 1;
            continue;
        };
        let parent = identity(dirs.last().context("Missing folder identity")?)?.0;
        let Ok(entries) = fs::read_dir(&dir) else {
            r.excluded += 1;
            continue;
        };
        for entry in entries {
            if r.examined == MAX_ENTRIES || r.files.len() == MAX_ITEMS || Instant::now() >= deadline
            {
                r.bounded = true;
                break 'scan;
            }
            r.examined += 1;
            let Ok(entry) = entry else {
                r.excluded += 1;
                continue;
            };
            let path = entry.path();
            let Ok(md) = fs::symlink_metadata(&path) else {
                r.excluded += 1;
                continue;
            };
            if path.as_os_str().len() > 1024
                || sensitive(&path)
                || md.file_attributes() & CLOUD != 0
            {
                r.excluded += 1;
                continue;
            }
            if md.is_dir() {
                queue.push(path);
                continue;
            }
            if !candidate(&path) || md.len() > MAX_FILE {
                r.excluded += 1;
                continue;
            }
            let Ok(mut f) = open_regular(&path, false) else {
                r.excluded += 1;
                continue;
            };
            let Some(rule) = class(&path, &mut f) else {
                r.excluded += 1;
                continue;
            };
            if budget + md.len() > HASH_BUDGET {
                r.bounded = true;
                break 'scan;
            }
            let Ok((bytes, modified, identity, sha256)) = snapshot(&mut f) else {
                r.excluded += 1;
                continue;
            };
            budget += bytes;
            r.files.push(Item {
                id: uuid(),
                path,
                rule: rule.into(),
                bytes,
                sha256,
                identity,
                parent_identity: parent.clone(),
                modified,
                state: "planned".into(),
                detail: String::new(),
            });
        }
    }
    r.files.sort_by(|a, b| a.path.cmp(&b.path));
    save(c, &r, None)?;
    Ok(serde_json::to_value(r)?)
}
fn list(c: &Connection) -> Result<Value> {
    let mut stmt =
        c.prepare("SELECT id,created,root FROM cleanup_reviews ORDER BY rowid DESC LIMIT 200")?;
    let rows=stmt.query_map([],|r|Ok(json!({"id":r.get::<_,String>(0)?,"created":r.get::<_,String>(1)?,"root":r.get::<_,String>(2)?})))?.collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(json!(rows))
}
#[path = "cleanup/moves.rs"]
mod moves;
#[cfg(test)]
#[path = "cleanup/tests.rs"]
mod tests;
pub fn command(c: &Connection, args: &[&str]) -> Result<Value> {
    db(c)?;
    match args {
        ["plan", dir] => plan(c, Path::new(dir)),
        ["reviews"] => list(c),
        ["audit", id] => {
            let review = load(c, id)?;
            let mut query=c.prepare("SELECT item,at,state,detail FROM cleanup_audit WHERE review=? ORDER BY seq DESC LIMIT 512")?;
            let records=query.query_map([id],|row|{
                let item:String=row.get(0)?;let path=review.files.iter().find(|f|f.id==item).map(|f|&f.path);
                Ok(json!({"item":item,"path":path,"at":row.get::<_,String>(1)?,"state":row.get::<_,String>(2)?,"detail":row.get::<_,String>(3)?}))
            })?.collect::<rusqlite::Result<Vec<_>>>()?;
            Ok(json!(records))
        }
        ["review", id] => moves::operate(c, id, None),
        [op @ ("quarantine" | "restore"), id, items, "--approve"] => {
            let chosen = items.split(',').collect::<Vec<_>>();
            let unique = chosen.iter().collect::<HashSet<_>>();
            ensure!(
                !chosen.is_empty()
                    && chosen.len() <= 32
                    && unique.len() == chosen.len()
                    && chosen.iter().all(|i| valid_id(i)),
                "Select 1–32 unique exact file IDs"
            );
            crate::audited(c, &format!("cleanup/{op}"), || {
                moves::operate(c, id, Some((op, &chosen)))
            })
        }
        _ => bail!(
            "Expected cleanup plan FOLDER, reviews, review ID, or quarantine|restore REVIEW ITEM[,ITEM] --approve"
        ),
    }
}
