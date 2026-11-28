use anyhow::{Context, Result, ensure};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap},
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
use walkdir::WalkDir;

const ENTRY_LIMIT: usize = 100_000;
const HASH_BUDGET: u64 = 128 * 1024 * 1024;
#[path = "model_inventory.rs"]
mod model_inventory;
pub use model_inventory::models;
fn classification(path: &Path) -> &'static str {
    let ext = path
        .extension()
        .and_then(|v| v.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    match ext.as_str() {
        "gguf" | "safetensors" | "onnx" => return "AI model files",
        "vhd" | "vhdx" | "vmdk" | "qcow2" | "wim" | "esd" | "iso" => {
            return "Disk and deployment images";
        }
        "bak" | "tib" | "tibx" | "mrimg" => return "Possible backups",
        _ => (),
    }
    for ancestor in path.ancestors().skip(1) {
        match ancestor
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_ascii_lowercase()
            .as_str()
        {
            "__pycache__" | ".pytest_cache" | ".cache" | "gpucache" | "shadercache" => {
                return "Recognized cache directories";
            }
            "node_modules" | ".venv" => return "Project dependencies",
            _ => (),
        }
    }
    "Other files"
}
#[cfg(windows)]
fn open_for_hash(path: &Path, root: &Path) -> Result<File> {
    use std::os::windows::{ffi::OsStringExt, fs::OpenOptionsExt, io::AsRawHandle};
    use windows_sys::Win32::Storage::FileSystem::{
        FILE_FLAG_OPEN_REPARSE_POINT, GetFinalPathNameByHandleW,
    };
    let f = fs::OpenOptions::new()
        .read(true)
        .share_mode(1)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
        .open(path)?;
    let md = f.metadata()?;
    use std::os::windows::fs::MetadataExt;
    ensure!(
        md.is_file() && md.file_attributes() & (0x400 | 0x1000 | 0x400000) == 0,
        "Skipped link or offline item"
    );
    let mut buf = vec![0u16; 32768];
    let n = unsafe {
        GetFinalPathNameByHandleW(f.as_raw_handle(), buf.as_mut_ptr(), buf.len() as u32, 0)
    } as usize;
    ensure!(n > 0 && n < buf.len(), "Cannot resolve open file");
    let actual = PathBuf::from(std::ffi::OsString::from_wide(&buf[..n]));
    ensure!(
        actual.starts_with(root),
        "Skipped file outside selected directory"
    );
    Ok(f)
}
#[cfg(not(windows))]
fn open_for_hash(path: &Path, root: &Path) -> Result<File> {
    ensure!(
        path.canonicalize()?.starts_with(root),
        "Skipped file outside selected directory"
    );
    Ok(File::open(path)?)
}
fn reparse(m: &fs::Metadata) -> bool {
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        m.file_attributes() & 0x400 != 0
    }
    #[cfg(not(windows))]
    {
        m.file_type().is_symlink()
    }
}
pub fn scan(dir: &Path) -> Result<Value> {
    ensure!(dir.is_absolute(), "Choose an absolute directory path");
    #[cfg(windows)]
    {
        use std::path::{Component, Prefix};
        use windows_sys::Win32::Storage::FileSystem::GetDriveTypeW;
        let drive = match dir.components().next() {
            Some(Component::Prefix(p)) => match p.kind() {
                Prefix::Disk(d) | Prefix::VerbatimDisk(d) => d,
                _ => anyhow::bail!("Choose a local disk"),
            },
            _ => anyhow::bail!("Choose a local disk"),
        };
        let volume = [drive as u16, 58, 92, 0];
        ensure!(
            matches!(unsafe { GetDriveTypeW(volume.as_ptr()) }, 2 | 3 | 6),
            "Choose a local fixed or removable volume"
        );
    }
    let md = fs::symlink_metadata(dir).context("Cannot read the selected directory")?;
    ensure!(
        md.is_dir() && !reparse(&md),
        "Choose a local directory without a reparse point"
    );
    for ancestor in dir.ancestors() {
        ensure!(
            !reparse(&fs::symlink_metadata(ancestor)?),
            "Choose a path without reparse points"
        );
    }
    let root = dir.canonicalize()?;
    let begin = Instant::now();
    let mut largest = BinaryHeap::new();
    let mut sizes: BTreeMap<u64, Vec<PathBuf>> = BTreeMap::new();
    let mut total = 0u64;
    let mut count = 0;
    let mut categories: BTreeMap<&'static str, u64> = BTreeMap::new();
    let mut directories: BTreeMap<String, u64> = BTreeMap::new();
    let mut errors = 0;
    let mut links = 0;
    let mut truncated = false;
    let mut iter = WalkDir::new(&root).follow_links(false).into_iter();
    let mut entries = 0;
    while let Some(entry) = iter.next() {
        entries += 1;
        if entries > ENTRY_LIMIT || begin.elapsed() > Duration::from_secs(25) {
            truncated = true;
            break;
        }
        let entry = match entry {
            Ok(x) => x,
            Err(_) => {
                errors += 1;
                continue;
            }
        };
        let md = match fs::symlink_metadata(entry.path()) {
            Ok(x) => x,
            Err(_) => {
                errors += 1;
                continue;
            }
        };
        if reparse(&md) {
            links += 1;
            if entry.file_type().is_dir() {
                iter.skip_current_dir()
            }
            continue;
        }
        if !md.is_file() {
            continue;
        }
        let size = md.len();
        count += 1;
        total = total.saturating_add(size);
        let relative = entry.path().strip_prefix(&root)?.to_owned();
        *categories.entry(classification(&relative)).or_default() += size;
        let folder = if relative.components().count() == 1 {
            ".".into()
        } else {
            relative
                .components()
                .next()
                .unwrap()
                .as_os_str()
                .to_string_lossy()
                .into_owned()
        };
        *directories.entry(folder).or_default() += size;
        largest.push(Reverse((size, relative.clone())));
        if largest.len() > 25 {
            largest.pop();
        }
        if size > 0 && size <= HASH_BUDGET {
            sizes.entry(size).or_default().push(relative);
        }
    }
    let mut groups = Vec::new();
    let mut hashed = 0;
    let mut incomplete = false;
    let mut hash_files = 0;
    'outer: for (size, paths) in sizes.into_iter().rev().filter(|(_, x)| x.len() > 1) {
        let mut digests: BTreeMap<String, Vec<PathBuf>> = BTreeMap::new();
        for path in paths {
            if hashed + size > HASH_BUDGET
                || hash_files >= 2048
                || begin.elapsed() > Duration::from_secs(30)
            {
                incomplete = true;
                for (sha, files) in digests {
                    if files.len() > 1 {
                        groups.push(json!({"sha256":sha,"sizeBytes":size,"files":files}));
                    }
                }
                break 'outer;
            }
            let abs = root.join(&path);
            if fs::symlink_metadata(&abs)
                .map(|m| reparse(&m) || m.len() != size)
                .unwrap_or(true)
            {
                errors += 1;
                continue;
            }
            let mut file = match open_for_hash(&abs, &root) {
                Ok(f) => f,
                Err(_) => {
                    errors += 1;
                    continue;
                }
            };
            let before = file.metadata()?;
            let mut hash = Sha256::new();
            let amount = std::io::copy(&mut file.by_ref().take(size + 1), &mut hash);
            hashed += size;
            hash_files += 1;
            let after = file.metadata()?;
            if amount.ok() != Some(size)
                || after.len() != size
                || before.modified().ok() != after.modified().ok()
            {
                errors += 1;
                continue;
            }
            digests
                .entry(format!("{:x}", hash.finalize()))
                .or_default()
                .push(path);
        }
        for (sha, files) in digests {
            if files.len() > 1 {
                groups.push(json!({"sha256":sha,"sizeBytes":size,"files":files}));
            }
        }
    }
    let largest: Vec<_> = largest
        .into_sorted_vec()
        .into_iter()
        .map(|Reverse((size, path))| json!({"path":path,"sizeBytes":size}))
        .collect();
    let mut folders: Vec<_> = directories.into_iter().collect();
    folders.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    let top: Vec<_> = folders
        .iter()
        .take(50)
        .map(|(name, bytes)| json!({"directory":name,"logicalBytes":bytes}))
        .collect();
    Ok(
        json!({"kind":"space-analysis","schemaVersion":1,"observedAt":crate::db::now(),"fileCount":count,"logicalBytes":total,"largest":largest,"sizeByCategory":categories,"topDirectories":top,"directoryGroupCount":folders.len(),"duplicateGroups":groups,"readErrors":errors,"skippedReparsePoints":links,"scanTruncated":truncated,"duplicateHashBudgetBytes":HASH_BUDGET,"duplicateBytesRead":hashed,"duplicateScanTruncated":incomplete,"scope":"Relative file names, logical sizes, and equal-content candidates. Hard links may refer to one allocation. Files over the hash budget are excluded from duplicate detection. Categories identify file types and directory names for review."}),
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_only_equal_content() -> Result<()> {
        let d = tempfile::tempdir()?;
        for (n, s) in [("a", "abc"), ("b", "abc"), ("c", "xyz"), ("d", "large")] {
            fs::write(d.path().join(n), s)?
        }
        let v = scan(d.path())?;
        assert_eq!(v["duplicateGroups"].as_array().unwrap().len(), 1);
        assert_eq!(v["largest"][0]["sizeBytes"], 5);
        assert_eq!(v["logicalBytes"], 14);
        Ok(())
    }
}
