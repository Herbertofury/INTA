use crate::{Result, err, expect, invalid};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

pub struct Directory {
    pub path: PathBuf,
    _handles: Vec<File>,
}
fn io_error(_: std::io::Error) -> crate::Error {
    err(
        "io",
        "The selected local file or directory is unavailable or busy.",
    )
}

#[cfg(windows)]
pub(super) mod host {
    use super::*;
    use std::ffi::OsStr;
    use std::os::windows::{
        ffi::OsStrExt,
        fs::{MetadataExt, OpenOptionsExt},
        io::AsRawHandle,
    };
    use std::path::{Component, Prefix};
    use windows_sys::Win32::Storage::FileSystem::*;

    fn safe_part(p: &OsStr) -> bool {
        let Some(s) = p.to_str() else { return false };
        let stem = s
            .split('.')
            .next()
            .unwrap_or("")
            .trim_end()
            .to_ascii_uppercase();
        let reserved = ["CON", "CONIN$", "CONOUT$", "PRN", "NUL", "AUX", "CLOCK$"]
            .contains(&stem.as_str())
            || ["COM", "LPT"].iter().any(|k| {
                stem.strip_prefix(k).is_some_and(|n| {
                    n.len() == 1 && n.as_bytes()[0].is_ascii_digit() || ["¹", "²", "³"].contains(&n)
                })
            });
        !reserved
            && !s.is_empty()
            && !s.ends_with([' ', '.'])
            && !s.chars().any(|c| c.is_control() || "<>:\"|?*".contains(c))
    }
    pub fn local(path: &Path) -> Result<PathBuf> {
        let raw = path
            .to_str()
            .ok_or_else(|| invalid("Select a Unicode local path."))?;
        expect(
            !raw.is_empty()
                && raw.len() <= 1024
                && !raw.starts_with(['\\', '/'])
                && !path.components().any(|c| matches!(c, Component::ParentDir)),
            "Use a local drive path without traversal or device syntax.",
        )?;
        let p = if path.is_absolute() {
            path.to_owned()
        } else {
            expect(
                !raw.contains(':'),
                "Use an absolute drive path or a plain relative path.",
            )?;
            std::env::current_dir().map_err(io_error)?.join(path)
        };
        let mut pieces = p.components();
        let Some(Component::Prefix(prefix)) = pieces.next() else {
            return Err(invalid("A local drive is required."));
        };
        let Prefix::Disk(drive) = prefix.kind() else {
            return Err(invalid("UNC and device paths are unsupported."));
        };
        expect(
            pieces.next() == Some(Component::RootDir),
            "An absolute drive is required.",
        )?;
        let mut output = PathBuf::from(format!("{}:\\", drive as char));
        let root: Vec<_> = output.as_os_str().encode_wide().chain([0]).collect();
        // Win32 drive types 2, 3, 5 and 6 denote removable, fixed, optical and RAM media.
        expect(
            matches!(unsafe { GetDriveTypeW(root.as_ptr()) }, 2 | 3 | 5 | 6),
            "Select a locally attached drive.",
        )?;
        for component in pieces {
            match component {
                Component::Normal(c) if safe_part(c) => output.push(c),
                Component::CurDir => {}
                _ => return Err(invalid("A path component is unsupported.")),
            }
        }
        Ok(output)
    }
    fn checked(path: &Path, dir: bool) -> Result<File> {
        let mut options = fs::OpenOptions::new();
        options
            .read(true)
            .share_mode(if dir {
                FILE_SHARE_READ | FILE_SHARE_WRITE
            } else {
                FILE_SHARE_READ
            })
            .custom_flags(
                FILE_FLAG_OPEN_REPARSE_POINT | if dir { FILE_FLAG_BACKUP_SEMANTICS } else { 0 },
            );
        if dir {
            options.access_mode(FILE_READ_ATTRIBUTES);
        }
        let file = options.open(path).map_err(io_error)?;
        let meta = file.metadata().map_err(io_error)?;
        let dangerous = FILE_ATTRIBUTE_REPARSE_POINT
            | FILE_ATTRIBUTE_OFFLINE
            | FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS
            | FILE_ATTRIBUTE_RECALL_ON_OPEN;
        expect(
            meta.file_attributes() & dangerous == 0
                && if dir { meta.is_dir() } else { meta.is_file() },
            "Only regular local files and directories are supported. Links and cloud placeholders are refused.",
        )?;
        expect(
            unsafe { GetFileType(file.as_raw_handle()) } == FILE_TYPE_DISK,
            "Only filesystem objects are supported.",
        )?;
        Ok(file)
    }
    pub fn directory(path: &Path, create: bool) -> Result<Directory> {
        let path = local(path)?;
        let mut ancestors: Vec<_> = path.ancestors().collect();
        ancestors.reverse();
        let mut handles = Vec::new();
        for p in ancestors {
            if create && p == path && p.parent().is_some() {
                match fs::create_dir(p) {
                    Ok(()) => {}
                    Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
                    Err(e) => return Err(io_error(e)),
                }
            }
            handles.push(checked(p, true)?);
        }
        Ok(Directory {
            path,
            _handles: handles,
        })
    }
    pub fn open(path: &Path) -> Result<(Directory, File)> {
        let path = local(path)?;
        let parents = directory(
            path.parent()
                .ok_or_else(|| invalid("Select a file, not a drive root."))?,
            false,
        )?;
        Ok((parents, checked(&path, false)?))
    }
}
#[cfg(not(windows))]
mod host {
    use super::*;
    pub fn directory(_: &Path, _: bool) -> Result<Directory> {
        Err(err(
            "unavailable",
            "Local file access requires the Windows x64 build.",
        ))
    }
    pub fn open(_: &Path) -> Result<(Directory, File)> {
        Err(err(
            "unavailable",
            "Local file access requires the Windows x64 build.",
        ))
    }
}
pub use host::directory;
pub fn read(path: &Path, max: u64) -> Result<Vec<u8>> {
    let (_guard, file) = host::open(path)?;
    expect(
        file.metadata().map_err(io_error)?.len() <= max,
        "File exceeds the size limit.",
    )?;
    let mut bytes = Vec::new();
    file.take(max + 1)
        .read_to_end(&mut bytes)
        .map_err(io_error)?;
    expect(bytes.len() as u64 <= max, "File exceeds the size limit.")?;
    Ok(bytes)
}
pub fn digest_file(path: &Path) -> Result<(u64, String)> {
    const MAX: u64 = 256 * 1024 * 1024;
    let (_guard, file) = host::open(path)?;
    let len = file.metadata().map_err(io_error)?.len();
    expect(len <= MAX, "Selected file exceeds 256 MiB.")?;
    let mut reader = file.take(MAX + 1);
    let mut hash = Sha256::new();
    let mut total = 0;
    let mut buf = [0; 65536];
    loop {
        let n = reader.read(&mut buf).map_err(io_error)?;
        if n == 0 {
            break;
        }
        total += n as u64;
        hash.update(&buf[..n]);
    }
    expect(total == len, "The selected file changed during hashing.")?;
    Ok((total, format!("{:x}", hash.finalize())))
}
