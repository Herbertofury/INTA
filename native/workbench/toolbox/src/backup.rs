use super::Answer;
#[cfg(windows)]
use super::{archive, checked_root, err, inside, linked, stage_verified};
use std::path::Path;

#[cfg(windows)]
mod windows {
    use super::*;
    use serde_json::json;
    use sha2::{Digest, Sha256};
    use std::{
        fs::{self, File, OpenOptions},
        io::{self, Read, Seek},
        mem::{offset_of, size_of},
        os::windows::{ffi::OsStrExt, fs::OpenOptionsExt, io::AsRawHandle},
        path::PathBuf,
        time::Instant,
    };
    use windows_sys::Win32::{
        Foundation::{GENERIC_READ, GENERIC_WRITE},
        Storage::FileSystem::{
            FileRenameInfo, GetDriveTypeW, SetFileInformationByHandle, DELETE,
            FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OPEN_REPARSE_POINT, FILE_READ_ATTRIBUTES,
            FILE_RENAME_INFO, FILE_SHARE_READ,
        },
    };
    const MAX_INPUT: u64 = 64 * 1024 * 1024;
    const MAX_ARCHIVE: u64 = MAX_INPUT + 1024 * 1024;

    /// Hold each ancestor against deletion or reparse-point changes until publication is over.
    fn guard_directories(parent: &Path) -> Result<Vec<File>, String> {
        checked_root(parent)?;
        let mut dirs: Vec<_> = parent.ancestors().collect();
        dirs.reverse();
        let volume: Vec<_> = dirs[0].as_os_str().encode_wide().chain(Some(0)).collect();
        if !matches!(unsafe { GetDriveTypeW(volume.as_ptr()) }, 2 | 3 | 6) {
            return Err("Backup requires local fixed, removable or RAM storage".into());
        }
        let mut guards = Vec::with_capacity(dirs.len());
        for dir in dirs {
            let handle = OpenOptions::new()
                .access_mode(FILE_READ_ATTRIBUTES)
                .share_mode(FILE_SHARE_READ)
                .custom_flags(FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT)
                .open(dir)
                .map_err(|e| format!("Unable to pin a backup directory: {e}"))?;
            let meta = handle.metadata().map_err(err)?;
            if linked(&meta) || !meta.is_dir() {
                return Err("Reparse points are outside the backup boundary".into());
            }
            guards.push(handle);
        }
        Ok(guards)
    }
    fn open_regular(path: &Path) -> Result<File, String> {
        let f = OpenOptions::new()
            .read(true)
            .share_mode(FILE_SHARE_READ)
            .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
            .open(path)
            .map_err(err)?;
        let m = f.metadata().map_err(err)?;
        if !m.is_file() || linked(&m) {
            return Err("Select a regular file outside reparse points".into());
        }
        Ok(f)
    }
    fn validate_input_stream(file: &File) -> Result<(), String> {
        use std::os::windows::fs::MetadataExt;
        use windows_sys::Win32::Storage::FileSystem::{
            FileStreamInfo, GetFileInformationByHandleEx, FILE_ATTRIBUTE_ENCRYPTED,
            FILE_STREAM_INFO,
        };
        if file.metadata().map_err(err)?.file_attributes() & FILE_ATTRIBUTE_ENCRYPTED != 0 {
            return Err("Encrypted files require a backup that preserves their protection".into());
        }
        // Query the held source handle. A single unnamed stream fits comfortably in this buffer.
        // On an unsupported filesystem or oversized list, require a filesystem-aware backup.
        let mut buffer = [0usize; 128];
        if unsafe {
            GetFileInformationByHandleEx(
                file.as_raw_handle(),
                FileStreamInfo,
                buffer.as_mut_ptr().cast(),
                size_of::<[usize; 128]>() as u32,
            )
        } == 0
        {
            return Err(
                "Unable to verify the file's stream layout; use a filesystem-aware backup".into(),
            );
        }
        let header = unsafe { &*buffer.as_ptr().cast::<FILE_STREAM_INFO>() };
        let expected: Vec<u16> = "::$DATA".encode_utf16().collect();
        if header.NextEntryOffset != 0 || header.StreamNameLength as usize != expected.len() * 2 {
            return Err("The file contains named streams; use a filesystem-aware backup".into());
        }
        // FileStreamInfo returns UTF-16 data immediately after this variable-sized header.
        let name = unsafe {
            std::slice::from_raw_parts(
                std::ptr::addr_of!(header.StreamName).cast::<u16>(),
                expected.len(),
            )
        };
        if name != expected {
            return Err("Unsupported input data stream".into());
        }
        Ok(())
    }
    fn file_hash(f: &mut File) -> Result<String, String> {
        f.rewind().map_err(err)?;
        let mut h = Sha256::new();
        io::copy(f, &mut h).map_err(err)?;
        Ok(format!("{:x}", h.finalize()))
    }
    fn split(path: &Path) -> Result<(&Path, &str), String> {
        let parent = path.parent().ok_or("Choose an absolute local file path")?;
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or("Invalid file name")?;
        // A trailing separator or dot could otherwise be normalized into a different Win32 object.
        if parent.join(name).as_os_str() != path.as_os_str() {
            let supplied = path.as_os_str().to_string_lossy().replace('/', "\\");
            let normalized = parent
                .join(name)
                .as_os_str()
                .to_string_lossy()
                .replace('/', "\\");
            if supplied != normalized {
                return Err("Use a normalized file path".into());
            }
        }
        Ok((parent, name))
    }
    /// Rename by handle with ReplaceIfExists=false, so an output collision cannot replace another file.
    fn publish(handle: &File, target: &Path) -> Result<(), String> {
        let name: Vec<u16> = target.as_os_str().encode_wide().collect();
        let bytes = name
            .len()
            .checked_mul(2)
            .ok_or("Destination name is too long")?;
        let length =
            (offset_of!(FILE_RENAME_INFO, FileName) + bytes + 2).max(size_of::<FILE_RENAME_INFO>());
        let mut buffer = vec![0usize; length.div_ceil(size_of::<usize>())];
        let ptr = buffer.as_mut_ptr().cast::<FILE_RENAME_INFO>();
        // Zero-initialization gives a null root handle and ReplaceIfExists=false. The aligned
        // buffer contains enough bytes for the flexible FileName field in full.
        let ok = unsafe {
            (*ptr).FileNameLength = bytes.try_into().map_err(err)?;
            std::ptr::copy_nonoverlapping(
                name.as_ptr(),
                std::ptr::addr_of_mut!((*ptr).FileName).cast::<u16>(),
                name.len(),
            );
            SetFileInformationByHandle(
                handle.as_raw_handle(),
                FileRenameInfo,
                ptr.cast(),
                length.try_into().map_err(err)?,
            )
        };
        if ok == 0 {
            return Err(format!(
                "Could not publish the new archive: {}",
                io::Error::last_os_error()
            ));
        }
        Ok(())
    }
    pub(super) fn create(root: &Path, source: &Path, destination: &Path) -> Answer {
        if !cfg!(target_arch = "x86_64") {
            return Err("Backup requires x64 Windows".into());
        }
        let start = Instant::now();
        let (src_parent, name) = split(source)?;
        let (dst_parent, dst_name) = split(destination)?;
        let _source_dirs = guard_directories(src_parent)?;
        let _destination_dirs = guard_directories(dst_parent)?;
        let src = inside(src_parent, name)?;
        let dst = inside(dst_parent, dst_name)?;
        if !dst
            .extension()
            .is_some_and(|s| s.eq_ignore_ascii_case("7z"))
        {
            return Err("Choose a new .7z destination".into());
        }
        match fs::symlink_metadata(&dst) {
            Ok(_) => return Err("Destination already exists; choose a new archive name".into()),
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(err(e)),
        }
        let mut input = open_regular(&src)?;
        validate_input_stream(&input)?;
        let bytes = input.metadata().map_err(err)?.len();
        if bytes > MAX_INPUT {
            return Err("Single-file backup supports inputs up to 64 MiB".into());
        }
        let (pin, work, exe, _sealed) = stage_verified(root, "7zip")?;
        let _private_dirs = guard_directories(work.path())?;
        let input_dir = work.path().join("selected");
        fs::create_dir(&input_dir).map_err(err)?;
        let snapshot = input_dir.join(name);
        let mut local = File::create_new(&snapshot).map_err(err)?;
        let copied = io::copy(&mut (&mut input).take(MAX_INPUT + 1), &mut local).map_err(err)?;
        local.sync_all().map_err(err)?;
        drop(local);
        if copied != bytes {
            return Err("The selected file changed during backup".into());
        }
        let input_sha = file_hash(&mut input)?;
        let mut snap = open_regular(&snapshot)?;
        if file_hash(&mut snap)? != input_sha {
            return Err("Private snapshot failed verification".into());
        }
        archive::run(&exe, &input_dir, archive::Operation::Create)?;
        let mut output = open_regular(&work.path().join("input.7z"))?;
        let size = output.metadata().map_err(err)?.len();
        if !(32..=MAX_ARCHIVE).contains(&size) {
            return Err("Archive exceeded its published size limit".into());
        }
        let archive_sha = file_hash(&mut output)?;
        fs::create_dir(work.path().join("restored")).map_err(err)?;
        archive::run(&exe, work.path(), archive::Operation::Restore)?;
        let mut recovered = open_regular(&work.path().join("restored").join(name))?;
        if fs::read_dir(work.path().join("restored"))
            .map_err(err)?
            .count()
            != 1
            || recovered.metadata().map_err(err)?.len() != bytes
            || file_hash(&mut recovered)? != input_sha
        {
            return Err("Archive restore failed the byte-for-byte check".into());
        }
        // Same-volume staging allows a single native no-clobber rename to publish verified bytes.
        let out_dir = tempfile::Builder::new()
            .prefix(".inta-backup-")
            .tempdir_in(dst_parent)
            .map_err(err)?;
        let _output_dirs = guard_directories(out_dir.path())?;
        let temp: PathBuf = out_dir.path().join("verified.7z");
        let mut staged = OpenOptions::new()
            .read(true)
            .write(true)
            .access_mode(GENERIC_READ | GENERIC_WRITE | DELETE)
            .share_mode(FILE_SHARE_READ)
            .create_new(true)
            .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
            .open(temp)
            .map_err(err)?;
        output.rewind().map_err(err)?;
        let written = io::copy(&mut output, &mut staged).map_err(err)?;
        staged.sync_all().map_err(err)?;
        if written != size || file_hash(&mut staged)? != archive_sha {
            return Err("Publication copy failed SHA-256 verification".into());
        }
        // Normalize separators via the canonical guarded parent. Windows native rename expects backslashes.
        let canonical = dst_parent.canonicalize().map_err(err)?.join(dst_name);
        publish(&staged, &canonical)?;
        Ok(
            json!({"state":"verified","inputSha256":input_sha,"archiveSha256":archive_sha,"inputBytes":bytes,
            "archiveBytes":size,"engine":"7zip","engineVersion":pin.version,"roundTripVerified":true,"durationMs":start.elapsed().as_millis()}),
        )
    }
}

/// Create a new verified 7z containing the selected regular file's default data stream.
pub fn create_backup(root: &Path, source: &Path, destination: &Path) -> Answer {
    #[cfg(windows)]
    {
        windows::create(root, source, destination)
    }
    #[cfg(not(windows))]
    {
        let _ = (root, source, destination);
        Err("Backup requires x64 Windows".into())
    }
}
