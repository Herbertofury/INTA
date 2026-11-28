#[path = "detached.rs"]
mod detached;
#[path = "images.rs"]
mod images;
#[path = "../cloud/bridge.rs"]
mod repair;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::HashSet,
    ffi::OsStr,
    fs::{self, File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    os::windows::{
        ffi::{OsStrExt, OsStringExt},
        fs::{MetadataExt, OpenOptionsExt},
        io::{AsRawHandle, FromRawHandle, OwnedHandle},
        process::CommandExt,
    },
    path::{Component, Path, PathBuf, Prefix},
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use windows_sys::Win32::{
    Foundation::{CloseHandle, GetLastError, ERROR_ALREADY_EXISTS, WAIT_OBJECT_0},
    Storage::FileSystem::{
        GetDriveTypeW, FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OPEN_REPARSE_POINT,
        FILE_READ_ATTRIBUTES, FILE_SHARE_READ, FILE_SHARE_WRITE,
    },
    System::{
        JobObjects::{
            AssignProcessToJobObject, CreateJobObjectW, JobObjectExtendedLimitInformation,
            SetInformationJobObject, JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
            JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
        },
        SystemInformation::GetSystemWindowsDirectoryW,
        Threading::{
            CreateEventW, OpenEventW, SetEvent, WaitForSingleObject, CREATE_NO_WINDOW,
            EVENT_MODIFY_STATE, SYNCHRONIZATION_SYNCHRONIZE,
        },
    },
};

type R<T> = Result<T, String>;
const LOCK: &str = include_str!("../runtime-lock.json");
const MAX_BASE: u64 = 8 * 1024 * 1024 * 1024;
const CPU: u32 = 2;
const MEMORY: u32 = 4096;
const DISK_GIB: u32 = 64;
fn err(e: impl std::fmt::Display) -> String {
    e.to_string()
}
fn time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
fn wide(s: &OsStr) -> Vec<u16> {
    s.encode_wide().chain(Some(0)).collect()
}
fn regular_comps(path: &Path) -> bool {
    path.components().all(|c| match c {
        Component::Prefix(p) => matches!(p.kind(), Prefix::Disk(_)),
        Component::RootDir => true,
        Component::Normal(s) => s.to_str().is_some_and(|s| {
            let stem = s.split('.').next().unwrap_or_default().to_uppercase();
            let device = matches!(
                stem.as_str(),
                "CON" | "PRN" | "AUX" | "NUL" | "CONIN$" | "CONOUT$"
            ) || ["COM", "LPT"].iter().any(|prefix| {
                stem.strip_prefix(prefix).is_some_and(|n| {
                    matches!(
                        n,
                        "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "¹" | "²" | "³"
                    )
                })
            });
            !device
                && !s.is_empty()
                && !s.ends_with(['.', ' '])
                && !s.contains([':', '*', '?', '"', '<', '>', '|'])
                && !s.chars().any(char::is_control)
        }),
        _ => false,
    })
}
fn local(path: &Path) -> R<()> {
    if !path.is_absolute() || !regular_comps(path) {
        return Err("Select a normalized absolute local drive path".into());
    }
    let volume = path.ancestors().last().ok_or("Missing local drive")?;
    if !matches!(
        unsafe { GetDriveTypeW(wide(volume.as_os_str()).as_ptr()) },
        2 | 3 | 6
    ) {
        return Err("Network and device paths are outside the lab boundary".into());
    }
    Ok(())
}
fn guarded_dirs(path: &Path) -> R<Vec<File>> {
    local(path)?;
    let mut parents: Vec<_> = path.ancestors().collect();
    parents.reverse();
    parents
        .into_iter()
        .map(|dir| {
            let f = OpenOptions::new()
                .access_mode(FILE_READ_ATTRIBUTES)
                .share_mode(FILE_SHARE_READ)
                .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT | FILE_FLAG_BACKUP_SEMANTICS)
                .open(dir)
                .map_err(err)?;
            let m = f.metadata().map_err(err)?;
            if !m.is_dir() || m.file_attributes() & 0x400 != 0 {
                return Err(
                    "Links and reparse-point directories are outside the lab boundary".into(),
                );
            }
            Ok(f)
        })
        .collect()
}
fn read_file(path: &Path) -> R<File> {
    let f = OpenOptions::new()
        .read(true)
        .share_mode(FILE_SHARE_READ)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
        .open(path)
        .map_err(err)?;
    let m = f.metadata().map_err(err)?;
    if !m.is_file() || m.file_attributes() & 0x400 != 0 {
        return Err("Expected a regular file without reparse points".into());
    }
    Ok(f)
}
fn safe_rel(rel: &str) -> R<&Path> {
    if rel.is_empty()
        || rel.contains('\\')
        || rel
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == "..")
    {
        return Err("Invalid relative runtime location".into());
    }
    let p = Path::new(rel);
    if !regular_comps(p) || !p.components().all(|c| matches!(c, Component::Normal(_))) {
        return Err("Invalid relative runtime location".into());
    }
    Ok(p)
}
fn read_json<T: for<'a> Deserialize<'a>>(p: &Path) -> R<T> {
    let mut raw = vec![];
    let meta = fs::symlink_metadata(p).map_err(err)?;
    if !meta.is_file() || meta.file_attributes() & 0x400 != 0 {
        return Err("Untrusted session metadata path".into());
    }
    OpenOptions::new()
        .read(true)
        .share_mode(FILE_SHARE_READ | windows_sys::Win32::Storage::FileSystem::FILE_SHARE_DELETE)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
        .open(p)
        .map_err(err)?
        .take(32769)
        .read_to_end(&mut raw)
        .map_err(err)?;
    if raw.len() > 32768 {
        return Err("Session metadata exceeds its bound".into());
    }
    serde_json::from_slice(&raw).map_err(err)
}
fn sha(f: &mut File) -> R<String> {
    f.rewind().map_err(err)?;
    let mut h = Sha256::new();
    io::copy(f, &mut h).map_err(err)?;
    f.rewind().map_err(err)?;
    Ok(format!("{:x}", h.finalize()))
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Lock {
    version: String,
    default_location: String,
    files: Vec<Artifact>,
}
#[derive(Deserialize)]
struct Artifact {
    path: String,
    sha256: String,
    bytes: u64,
}
struct Runtime {
    home: PathBuf,
    version: String,
    _guard: Vec<File>,
}
fn runtime(toolbox: &Path) -> R<Runtime> {
    let lock: Lock = serde_json::from_str(LOCK).map_err(err)?;
    let mut guard = guarded_dirs(toolbox)?;
    let index = toolbox.join(".inta-vm-runtime.json");
    let rel = if index.try_exists().map_err(err)? {
        let r: Value = read_json(&index)?;
        if r["schemaVersion"] != 1 {
            return Err("Unsupported VM runtime marker".into());
        }
        r["location"]
            .as_str()
            .ok_or("Invalid VM runtime marker")?
            .to_owned()
    } else {
        lock.default_location
    };
    let home = toolbox.join(safe_rel(&rel)?);
    for (parent, allowed) in [
        (&home, vec!["bin", "share", "notices"]),
        (&home.join("share"), vec!["qemu"]),
    ] {
        guard.extend(guarded_dirs(parent)?);
        for entry in fs::read_dir(parent).map_err(err)? {
            let e = entry.map_err(err)?;
            if !allowed.iter().any(|name| e.file_name() == OsStr::new(name)) {
                return Err("Unexpected path in VM runtime layout".into());
            }
        }
    }
    let mut seen = HashSet::new();
    let allowed: HashSet<_> = lock.files.iter().map(|f| f.path.clone()).collect();
    for f in lock.files {
        let p = home.join(safe_rel(&f.path)?);
        let parent = p.parent().ok_or("Invalid runtime pin")?;
        if seen.insert(parent.to_owned()) {
            guard.extend(guarded_dirs(parent)?);
            for e in fs::read_dir(parent).map_err(err)? {
                let entry = e.map_err(err)?;
                let add = entry
                    .path()
                    .strip_prefix(&home)
                    .map_err(err)?
                    .to_string_lossy()
                    .replace('\\', "/");
                if !allowed.contains(&add) {
                    return Err("Unexpected file or folder in executable/firmware directory".into());
                }
            }
        }
        let mut file = read_file(&p)?;
        if file.metadata().map_err(err)?.len() != f.bytes || sha(&mut file)? != f.sha256 {
            return Err(format!("VM runtime integrity check failed for {}", f.path));
        }
        guard.push(file);
    }
    Ok(Runtime {
        home,
        version: lock.version,
        _guard: guard,
    })
}
fn state_root(toolbox: &Path, create: bool) -> R<PathBuf> {
    let p = if let Some(s) = std::env::var_os("INTA_VM_LAB_DIR") {
        PathBuf::from(s)
    } else {
        toolbox
            .parent()
            .ok_or("Select a toolbox stored in a folder")?
            .join("INTA VM Labs")
    };
    local(&p)?;
    let parent = p.parent().ok_or("Select a dedicated lab directory")?;
    let _guard = guarded_dirs(parent)?;
    if create && !p.try_exists().map_err(err)? {
        fs::create_dir(&p)
            .or_else(|e| {
                if e.kind() == io::ErrorKind::AlreadyExists {
                    Ok(())
                } else {
                    Err(e)
                }
            })
            .map_err(err)?;
    }
    if p.try_exists().map_err(err)? {
        guarded_dirs(&p)?;
    }
    Ok(p)
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Session {
    id: String,
    iso: PathBuf,
    iso_sha256: Option<String>,
    definitions: Option<images::Definitions>,
    base: Option<PathBuf>,
    base_sha256: Option<String>,
    network: String,
    created_at: u64,
    state: String,
    detail: Option<String>,
    accelerator: Option<String>,
}
fn session_id(id: &str) -> R<&str> {
    let parsed = uuid::Uuid::parse_str(id).map_err(|_| "Invalid session ID")?;
    if parsed.to_string() != id {
        return Err("Invalid session ID".into());
    }
    Ok(id)
}
fn save(dir: &Path, s: &Session) -> R<()> {
    let mut tmp = tempfile::NamedTempFile::new_in(dir).map_err(err)?;
    serde_json::to_writer(&mut tmp, &s).map_err(err)?;
    tmp.flush().map_err(err)?;
    tmp.as_file().sync_all().map_err(err)?;
    let until = Instant::now() + Duration::from_secs(2);
    let target = dir.join("session.json");
    loop {
        match tmp.persist(&target) {
            Ok(_) => return Ok(()),
            Err(failure)
                if matches!(failure.error.raw_os_error(), Some(32 | 33))
                    && Instant::now() < until =>
            {
                tmp = failure.file;
                thread::sleep(Duration::from_millis(20));
            }
            Err(failure) => return Err(err(failure.error)),
        }
    }
}
fn owned_session(root: &Path, id: &str) -> R<(PathBuf, Session)> {
    let until = Instant::now() + Duration::from_secs(2);
    loop {
        match read_owned_session(root, id) {
            Err(e)
                if (e.ends_with("(os error 32)") || e.ends_with("(os error 33)"))
                    && Instant::now() < until =>
            {
                thread::sleep(Duration::from_millis(20));
            }
            result => return result,
        }
    }
}
fn read_owned_session(root: &Path, id: &str) -> R<(PathBuf, Session)> {
    let dir = root.join(session_id(id)?);
    let _g = guarded_dirs(&dir)?;
    let s: Session = read_json(&dir.join("session.json"))?;
    if s.id != id || !matches!(s.network.as_str(), "none" | "user-nat") {
        return Err("Invalid session identity".into());
    }
    Ok((dir, s))
}
fn summary(dir: &Path, s: &Session) -> Value {
    let mut v = serde_json::to_value(s).unwrap_or(Value::Null);
    v["directory"] = json!(dir);
    if s.state == "running" && event(&s.id, false).is_err() {
        v["state"] = json!("interrupted");
    }
    v
}
fn event(id: &str, create: bool) -> R<OwnedHandle> {
    session_id(id)?;
    let name = wide(OsStr::new(&format!("Local\\INTA-Lab-Stop-{id}")));
    let handle = unsafe {
        if create {
            CreateEventW(std::ptr::null(), 1, 0, name.as_ptr())
        } else {
            OpenEventW(
                EVENT_MODIFY_STATE | SYNCHRONIZATION_SYNCHRONIZE,
                0,
                name.as_ptr(),
            )
        }
    };
    if handle.is_null() {
        return Err("The INTA supervisor is offline".into());
    }
    if create && unsafe { GetLastError() } == ERROR_ALREADY_EXISTS {
        unsafe { CloseHandle(handle) };
        return Err("An active supervisor already owns this lab".into());
    }
    Ok(unsafe { OwnedHandle::from_raw_handle(handle) })
}
struct Medium {
    _dirs: Vec<File>,
    file: File,
    bytes: u64,
}
fn medium(p: &Path, iso: bool) -> R<Medium> {
    local(p)?;
    let ext = if iso { "iso" } else { "img" };
    if !p.extension().is_some_and(|s| s.eq_ignore_ascii_case(ext)) {
        return Err(format!("Select a regular local .{ext} file"));
    }
    let guards = guarded_dirs(p.parent().ok_or("Missing image directory")?)?;
    let mut f = read_file(p)?;
    let bytes = f.metadata().map_err(err)?.len();
    if iso {
        if !(32774..=64 * 1024 * 1024 * 1024).contains(&bytes) {
            return Err("Invalid or oversized optical image".into());
        }
        let mut block = [0u8; 6];
        let mut recognized = false;
        for sector in 16..32 {
            if sector * 2048 + 6 > bytes {
                break;
            }
            f.seek(SeekFrom::Start(sector * 2048)).map_err(err)?;
            f.read_exact(&mut block).map_err(err)?;
            if [&b"CD001"[..], &b"BEA01"[..], &b"NSR02"[..], &b"NSR03"[..]].contains(&&block[1..]) {
                recognized = true;
                break;
            }
        }
        if !recognized {
            return Err("ISO9660 or UDF volume descriptor missing".into());
        }
    } else if !(512..=MAX_BASE).contains(&bytes) || bytes % 512 != 0 {
        return Err("Raw test disks must be sector aligned and between 512 bytes and 8 GiB".into());
    }
    f.rewind().map_err(err)?;
    Ok(Medium {
        file: f,
        bytes,
        _dirs: guards,
    })
}
fn win_dir() -> R<PathBuf> {
    let mut b = [0u16; 32768];
    let n = unsafe { GetSystemWindowsDirectoryW(b.as_mut_ptr(), b.len() as u32) } as usize;
    if n == 0 || n >= b.len() {
        return Err("Windows directory unavailable".into());
    }
    Ok(std::ffi::OsString::from_wide(&b[..n]).into())
}
fn child_cmd(rt: &Runtime, bin: &str, cwd: &Path) -> R<Command> {
    let system = win_dir()?;
    let mut cmd = Command::new(rt.home.join("bin").join(bin));
    cmd.env_clear()
        .env("SystemRoot", &system)
        .env(
            "SystemDrive",
            system
                .to_string_lossy()
                .get(..2)
                .ok_or("Invalid Windows directory")?,
        )
        .env("ProgramData", cwd)
        .env("USERPROFILE", cwd)
        .env("APPDATA", cwd)
        .env("LOCALAPPDATA", cwd)
        .env("WINDIR", &system)
        .env(
            "PATH",
            std::env::join_paths([rt.home.join("bin"), system.join("System32")]).map_err(err)?,
        )
        .env("TEMP", cwd)
        .env("TMP", cwd)
        .env("SDL_VIDEODRIVER", "windows")
        .env("SDL_RENDER_DRIVER", "software")
        .current_dir(cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .creation_flags(CREATE_NO_WINDOW);
    Ok(cmd)
}
fn disk_arg(node: &str, path: &Path, ro: bool) -> String {
    json!({"driver":"file","filename":path,"node-name":node,"read-only":ro}).to_string()
}
fn qemu_args(rt: &Runtime, dir: &Path, s: &Session, accel: &str) -> Vec<String> {
    let bios = rt.home.join("share/qemu");
    let mut a = vec![];
    let mut add = |k: &str, v: String| {
        a.push(k.to_string());
        a.push(v)
    };
    add("-name", format!("INTA lab {}", s.id));
    add("-machine", "q35,vmport=off".into());
    add("-accel", accel.into());
    add("-m", MEMORY.to_string());
    add("-smp", CPU.to_string());
    add("-display", "sdl".into());
    add("-vga", "std".into());
    add("-L", bios.to_string_lossy().into());
    add(
        "-blockdev",
        disk_arg("codeFile", &bios.join("edk2-x86_64-code.fd"), true),
    );
    add(
        "-blockdev",
        json!({"driver":"raw","file":"codeFile","node-name":"code","read-only":true}).to_string(),
    );
    add(
        "-blockdev",
        disk_arg("varsFile", &dir.join("vars.fd"), false),
    );
    add(
        "-blockdev",
        json!({"driver":"raw","file":"varsFile","node-name":"vars"}).to_string(),
    );
    add(
        "-global",
        "driver=cfi.pflash01,property=secure,value=off".into(),
    );
    // Attach flash nodes to the machine through documented machine properties.
    add("-machine", "pflash0=code,pflash1=vars".into());
    add("-blockdev", disk_arg("isoFile", &s.iso, true));
    add(
        "-blockdev",
        json!({"driver":"raw","file":"isoFile","node-name":"bootmedia","read-only":true})
            .to_string(),
    );
    add(
        "-device",
        "ide-cd,bus=ide.0,unit=0,drive=bootmedia,bootindex=1".into(),
    );
    if let Some(definitions) = &s.definitions {
        add(
            "-blockdev",
            disk_arg("definitionsFile", &definitions.path, true),
        );
        add("-blockdev", json!({"driver":"raw","file":"definitionsFile","node-name":"definitions","read-only":true}).to_string());
        add(
            "-device",
            "ide-cd,bus=ide.2,unit=0,drive=definitions".into(),
        );
    }
    add(
        "-blockdev",
        disk_arg("diskFile", &dir.join("disk.qcow2"), false),
    );
    let backing = if s.base.is_some() {
        add(
            "-blockdev",
            disk_arg("baseFile", &dir.join("base.raw"), true),
        );
        add(
            "-blockdev",
            json!({"driver":"raw","file":"baseFile","node-name":"baseRaw","read-only":true})
                .to_string(),
        );
        json!("baseRaw")
    } else {
        Value::Null
    };
    add(
        "-blockdev",
        json!({"driver":"qcow2","file":"diskFile","backing":backing,"node-name":"scratch"})
            .to_string(),
    );
    add(
        "-device",
        "ide-hd,bus=ide.1,unit=0,drive=scratch,bootindex=2".into(),
    );
    add(
        "-nic",
        if s.network == "user-nat" {
            "user,model=e1000e,ipv6=off".into()
        } else {
            "none".into()
        },
    );
    add("-monitor", "none".into());
    add("-serial", "none".into());
    add("-parallel", "none".into());
    a.push("-no-user-config".into());
    a.push("-nodefaults".into());
    a
}
fn die_on_close(child: &mut Child) -> R<OwnedHandle> {
    let h = unsafe { CreateJobObjectW(std::ptr::null(), std::ptr::null()) };
    if h.is_null() {
        return Err(err(io::Error::last_os_error()));
    }
    let job = unsafe { OwnedHandle::from_raw_handle(h) };
    let mut limits = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
    limits.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
    if unsafe {
        SetInformationJobObject(
            h,
            JobObjectExtendedLimitInformation,
            (&limits as *const JOBOBJECT_EXTENDED_LIMIT_INFORMATION).cast(),
            std::mem::size_of_val(&limits) as u32,
        )
    } == 0
        || unsafe { AssignProcessToJobObject(h, child.as_raw_handle()) } == 0
    {
        return Err(err(io::Error::last_os_error()));
    }
    Ok(job)
}
fn run_img(mut cmd: Command) -> R<()> {
    let mut p = cmd.spawn().map_err(err)?;
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        if let Some(s) = p.try_wait().map_err(err)? {
            return if s.success() {
                Ok(())
            } else {
                Err(format!("qemu-img failed ({s})"))
            };
        }
        if Instant::now() > deadline {
            let _ = p.kill();
            let _ = p.wait();
            return Err("qemu-img timed out".into());
        }
        thread::sleep(Duration::from_millis(60));
    }
}
fn pin_new_file(p: &Path) -> R<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
        .open(p)
        .map_err(err)
}
fn copy_and_hash(src: &mut File, dst: &mut File) -> R<String> {
    src.rewind().map_err(err)?;
    let mut digest = Sha256::new();
    let mut bytes = [0u8; 128 * 1024];
    loop {
        let n = src.read(&mut bytes).map_err(err)?;
        if n == 0 {
            break;
        }
        dst.write_all(&bytes[..n]).map_err(err)?;
        digest.update(&bytes[..n]);
    }
    dst.sync_all().map_err(err)?;
    Ok(format!("{:x}", digest.finalize()))
}
fn supervise(rt: &Runtime, dir: &Path, s: &mut Session) -> R<()> {
    let _guards = guarded_dirs(dir)?;
    let mut iso = medium(&s.iso, true)?;
    if let Some(digest) = &s.iso_sha256 {
        verify_image(&mut iso.file, digest)?;
    }
    let _definitions = if let Some(definitions) = &s.definitions {
        let mut input = medium(&definitions.path, true)?;
        verify_image(&mut input.file, &definitions.sha256)?;
        Some(input)
    } else {
        None
    };
    let quit = event(&s.id, true)?;
    let mut vars = pin_new_file(&dir.join("vars.fd"))?;
    let mut template = read_file(&rt.home.join("share/qemu/edk2-i386-vars.fd"))?;
    io::copy(&mut template, &mut vars).map_err(err)?;
    vars.sync_all().map_err(err)?;
    let scratch = dir.join("runtime-cache");
    fs::create_dir(&scratch).map_err(err)?;
    let mut img = child_cmd(rt, "qemu-img.exe", &scratch)?;
    let disk = dir.join("disk.qcow2");
    let _disk = pin_new_file(&disk)?;
    let _base_lock;
    img.args(["create", "-q", "-f", "qcow2"]);
    if let Some(b) = s.base.as_ref() {
        let mut base = medium(b, false)?;
        let raw = dir.join("base.raw");
        let mut out = pin_new_file(&raw)?;
        s.base_sha256 = Some(copy_and_hash(&mut base.file, &mut out)?);
        drop(out);
        let mut locked = read_file(&raw)?;
        if Some(sha(&mut locked)?) != s.base_sha256 {
            return Err("Disk snapshot failed readback verification".into());
        }
        _base_lock = Some(locked);
        img.args(["-F", "raw", "-b", "base.raw"]).arg(&disk);
    } else {
        _base_lock = None;
        img.arg(&disk).arg(format!("{DISK_GIB}G"));
    }
    run_img(img)?;
    let session_log = pin_new_file(&dir.join("engine.log"))?;
    let mut last_error = String::new();
    for accel in ["whpx,kernel-irqchip=off", "tcg"] {
        let mut cmd = child_cmd(rt, "qemu-system-x86_64.exe", &scratch)?;
        let mut child = cmd
            .args(qemu_args(rt, dir, s, accel))
            .stderr(Stdio::piped())
            .spawn()
            .map_err(err)?;
        let mut pipe = child
            .stderr
            .take()
            .ok_or("Cannot open guest diagnostic channel")?;
        let mut log = session_log.try_clone().map_err(err)?;
        let start_size = log.metadata().map_err(err)?.len();
        thread::spawn(move || {
            let _ = io::copy(
                &mut (&mut pipe).take(131072u64.saturating_sub(start_size)),
                &mut log,
            );
            let _ = io::copy(&mut pipe, &mut io::sink());
        });
        let job = match die_on_close(&mut child) {
            Ok(j) => j,
            Err(e) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(format!("Cannot constrain guest process: {e}"));
            }
        };
        thread::sleep(Duration::from_millis(1500));
        if let Some(st) = child.try_wait().map_err(err)? {
            if st.success() {
                s.accelerator = Some(accel.to_owned());
                s.state = "exited".into();
                return Ok(());
            }
            last_error = format!("{accel} startup exited ({st})");
            continue;
        }
        s.state = "running".into();
        s.accelerator = Some(accel.to_string());
        save(dir, s)?;
        loop {
            if unsafe { WaitForSingleObject(quit.as_raw_handle(), 200) } == WAIT_OBJECT_0 {
                drop(job);
                let _ = child.wait();
                s.state = "powered-off".into();
                break;
            }
            if let Some(st) = child.try_wait().map_err(err)? {
                s.state = if st.success() { "exited" } else { "failed" }.into();
                break;
            }
        }
        return Ok(());
    }
    Err(format!(
        "QEMU failed to start ({last_error}). See the new session's engine.log."
    ))
}
fn verify_image(file: &mut File, digest: &str) -> R<()> {
    if sha(file)? != digest {
        return Err("The boot image failed its expected SHA-256 check".into());
    }
    Ok(())
}

fn plan(
    toolbox: &Path,
    args: &[&str],
    starting: bool,
) -> R<(PathBuf, Option<PathBuf>, bool, Value)> {
    let (iso, mut tail) = args.split_first().ok_or("Select an ISO")?;
    let iso = PathBuf::from(iso);
    let mut boot = medium(&iso, true)?;
    let mut iso_digest = None;
    let mut with_definitions = false;
    let mut base = None;
    let mut nat = false;
    let mut approved = false;
    let mut base_bytes = 0;
    while let Some((arg, rest)) = tail.split_first() {
        tail = rest;
        match *arg {
            "--clamav" if !with_definitions => with_definitions = true,
            "--sha256" if iso_digest.is_none() => {
                let (digest, next) = tail
                    .split_first()
                    .ok_or("Supply the expected image SHA-256")?;
                if digest.len() != 64 || !digest.bytes().all(|b| b.is_ascii_hexdigit()) {
                    return Err("Invalid boot image SHA-256".into());
                }
                let digest = digest.to_ascii_lowercase();
                if !starting {
                    verify_image(&mut boot.file, &digest)?;
                }
                iso_digest = Some(digest);
                tail = next;
            }
            "--approve" if !approved && starting => approved = true,
            "--nat" if !nat => nat = true,
            "--base" if base.is_none() => {
                let (f, next) = tail.split_first().ok_or("Choose a raw .img test disk")?;
                tail = next;
                let p = PathBuf::from(f);
                base_bytes = medium(&p, false)?.bytes;
                base = Some(p);
            }
            _ => return Err("Invalid or duplicate VM option".into()),
        }
    }
    if starting && !approved {
        return Err("Starting a disposable lab requires --approve".into());
    }
    let definitions = if with_definitions {
        let entry = images::definitions(toolbox)?
            .ok_or("The verified ClamAV definition CD is unavailable")?;
        if iso_digest.as_deref() != Some(entry.target.as_str()) {
            return Err("Use the INTA Linux Rescue image validated with this definition CD".into());
        }
        if !starting {
            verify_image(&mut medium(&entry.path, true)?.file, &entry.sha256)?;
        }
        Some(entry)
    } else {
        None
    };
    let rt = runtime(toolbox)?;
    let root = state_root(toolbox, false)?;
    let destination = if root.exists() {
        root.as_path()
    } else {
        root.parent().ok_or("Invalid lab directory")?
    };
    let mut free = 0u64;
    if unsafe {
        windows_sys::Win32::Storage::FileSystem::GetDiskFreeSpaceExW(
            wide(destination.as_os_str()).as_ptr(),
            &mut free,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    } == 0
    {
        return Err(err(io::Error::last_os_error()));
    }
    let required = 1024u64.pow(3)
        + if base.is_some() {
            2 * base_bytes
        } else {
            u64::from(DISK_GIB) * 1024u64.pow(3)
        };
    let v = json!({"ready":free>=required,"freeBytes":free,"requiredFreeBytes":required,"version":rt.version,"iso":iso,"isoBytes":boot.bytes,"base":base,"baseBytes":base_bytes,"network":if nat{"user-nat"}else{"none"},"firmware":"EDK II UEFI x64","memoryMiB":MEMORY,"cpus":CPU,"diskGiB":if base.is_some(){base_bytes as f64/(1024.0_f64.powi(3))}else{DISK_GIB as f64},"accelerator":"WHPX with TCG fallback","storageRoot":root});
    let mut v = v;
    v["isoSha256"] = json!(iso_digest);
    v["definitions"] = json!(definitions);
    Ok((iso, base, nat, v))
}
fn start_session(toolbox: &Path, args: &[&str]) -> R<Value> {
    let (iso, base, nat, preview) = plan(toolbox, args, true)?;
    if preview["ready"] != true {
        return Err("Insufficient free space for the selected test disk and lab".into());
    }
    let root = state_root(toolbox, true)?;
    let _root_guard = guarded_dirs(&root)?;
    let id = uuid::Uuid::new_v4().to_string();
    let dir = root.join(&id);
    fs::create_dir(&dir).map_err(err)?;
    let mut s = Session {
        id: id.clone(),
        iso,
        iso_sha256: preview["isoSha256"].as_str().map(str::to_owned),
        definitions: serde_json::from_value(preview["definitions"].clone()).map_err(err)?,
        base,
        base_sha256: None,
        network: if nat { "user-nat" } else { "none" }.into(),
        created_at: time(),
        state: "preparing".into(),
        detail: None,
        accelerator: None,
    };
    save(&dir, &s)?;
    let supervisor = match detached::Supervisor::start(toolbox, &root, &id) {
        Ok(p) => p,
        Err(e) => {
            s.state = "failed".into();
            s.detail = Some(e.to_string());
            save(&dir, &s)?;
            return Err(err(e));
        }
    };
    let start = Instant::now();
    loop {
        let (_, record) = owned_session(&root, &id)?;
        if record.state == "failed" {
            return Err(record.detail.unwrap_or_else(|| "VM startup failed".into()));
        }
        if record.state != "preparing" || start.elapsed() > Duration::from_secs(300) {
            return Ok(summary(&dir, &record));
        }
        if let Some(exit) = supervisor.try_wait().map_err(err)? {
            // Read again in case the worker atomically published its result just before exiting.
            let (_, end) = owned_session(&root, &id)?;
            if end.state != "preparing" {
                return Ok(summary(&dir, &end));
            }
            s.state = "failed".into();
            s.detail = Some(format!("Supervisor exited before startup ({exit})"));
            save(&dir, &s)?;
            return Err(s.detail.unwrap());
        }
        thread::sleep(Duration::from_millis(200));
    }
}

fn worker(toolbox: &Path, id: &str) -> R<Value> {
    if std::env::var("INTA_VM_WORKER_ID").ok().as_deref() != Some(id) {
        return Err("VM supervisor requires a freshly approved session".into());
    }
    let root = state_root(toolbox, false)?;
    let (dir, mut s) = owned_session(&root, id)?;
    if s.state != "preparing" {
        return Err("A session already initialized cannot be reused by the supervisor".into());
    }
    if let Err(e) = runtime(toolbox).and_then(|rt| supervise(&rt, &dir, &mut s)) {
        s.state = "failed".into();
        s.detail = Some(e);
    }
    save(&dir, &s)?;
    Ok(summary(&dir, &s))
}

fn sessions(toolbox: &Path) -> R<Value> {
    let root = state_root(toolbox, false)?;
    let mut rows = vec![];
    if root.is_dir() {
        for entry in fs::read_dir(&root).map_err(err)?.take(10000) {
            let e = entry.map_err(err)?;
            let name = e.file_name();
            if let Some(id) = name.to_str().filter(|id| session_id(id).is_ok()) {
                let (dir, s) = owned_session(&root, id)?;
                rows.push(summary(&dir, &s));
            }
        }
    }
    rows.sort_by_key(|s| std::cmp::Reverse(s["createdAt"].as_u64().unwrap_or_default()));
    Ok(json!({"sessions":rows,"storageRoot":root}))
}

fn stop(toolbox: &Path, id: &str) -> R<Value> {
    let root = state_root(toolbox, false)?;
    let (dir, s) = owned_session(&root, id)?;
    if s.state != "running" {
        return Err("Only a running INTA lab can be powered off".into());
    }
    let handle = event(id, false)?;
    if unsafe { SetEvent(handle.as_raw_handle()) } == 0 {
        return Err(err(io::Error::last_os_error()));
    }
    drop(handle);
    for _ in 0..50 {
        thread::sleep(Duration::from_millis(100));
        let (_, new) = owned_session(&root, id)?;
        if new.state != "running" {
            return Ok(summary(&dir, &new));
        }
    }
    Err("Requested power off; check the session state".into())
}

pub(super) fn rust_lab_enabled() -> bool {
    repair::enabled()
}
pub(super) fn command(toolbox: &Path, args: &[&str]) -> R<Value> {
    match args {
        ["images"] => return images::list(toolbox),
        ["repair-test"] => return repair::run(toolbox, false, None),
        ["console", "--approve"] => return repair::run(toolbox, true, None),
        ["console", "--approve", "--base", path] | ["console", "--base", path, "--approve"] => {
            return repair::run(toolbox, true, Some(Path::new(path)))
        }
        ["console", ..] => return Err("Expected vm console --approve [--base FILE.img]".into()),
        _ => {}
    }
    match args {
        ["status"]=>match runtime(toolbox){
            Ok(rt)=>Ok(json!({"ready":true,"version":rt.version,"backend":"QEMU","platform":"Windows x64","runtimePath":rt.home})),
            Err(e)=>Ok(json!({"ready":false,"backend":"QEMU","detail":e})),
        },
        ["plan",tail @ ..]=>Ok(plan(toolbox,tail,false)?.3),
        ["start",tail @ ..]=>start_session(toolbox,tail),
        ["supervise",id]=>worker(toolbox,id),
        ["sessions"]=>sessions(toolbox),
        ["stop",id,"--approve"]=>stop(toolbox,id),
        _=>Err("Expected vm status, plan ISO, start ISO --approve [--base FILE.img] [--nat], sessions or stop ID --approve".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn raw_devices_uris_ads_and_escape_paths_are_rejected() {
        for path in [
            r"\\.\PhysicalDrive0",
            r"\\host\share\disk.img",
            r"C:\dir\..\boot.iso",
            r"C:\dir\NUL.iso",
            r"C:\dir\a.img:stream",
            r"nbd:localhost",
        ] {
            assert!(local(Path::new(path)).is_err(), "{path}");
        }
        for rel in ["../bin", "/bin", "C:/bin", "nested//bin", "./bin"] {
            assert!(safe_rel(rel).is_err(), "{rel}");
        }
        assert!(session_id("../another-vm").is_err());
    }

    #[test]
    fn paths_can_only_be_block_filenames_and_user_disk_never_attaches() {
        let rt = Runtime {
            home: PathBuf::from("C:/test/runtime"),
            version: "fixture".into(),
            _guard: vec![],
        };
        let session = Session {
            id: uuid::Uuid::new_v4().to_string(),
            iso: PathBuf::from("C:/test/file,cache=unsafe.iso"),
            iso_sha256: None,
            definitions: None,
            base: Some(PathBuf::from("C:/test/private.img")),
            base_sha256: None,
            network: "none".into(),
            created_at: 0,
            state: "preparing".into(),
            detail: None,
            accelerator: None,
        };
        let args = qemu_args(&rt, Path::new("C:/test/session"), &session, "tcg");
        assert!(args.windows(2).any(|w| w == ["-nic", "none"]));
        assert!(!args.iter().any(|v| v.contains("private.img")));
        let nodes: Vec<Value> = args
            .windows(2)
            .filter(|pair| pair[0] == "-blockdev")
            .map(|pair| serde_json::from_str(&pair[1]).unwrap())
            .collect();
        let iso = nodes
            .iter()
            .find(|node| node["node-name"] == "isoFile")
            .unwrap();
        assert_eq!(iso["filename"], "C:/test/file,cache=unsafe.iso");
        assert_eq!(iso["read-only"], true);
        assert_eq!(
            nodes.iter().find(|n| n["node-name"] == "baseFile").unwrap()["read-only"],
            true
        );
        assert_eq!(
            nodes.iter().find(|n| n["node-name"] == "scratch").unwrap()["backing"],
            "baseRaw"
        );
    }
    #[test]
    fn corrupt_optical_and_unaligned_raw_input_are_refused() {
        let tmp = tempfile::tempdir().unwrap();
        let iso = tmp.path().join("invalid.iso");
        fs::write(&iso, vec![0; 65536]).unwrap();
        assert!(medium(&iso, true).is_err());
        let raw = tmp.path().join("invalid.img");
        fs::write(&raw, [0; 513]).unwrap();
        assert!(medium(&raw, false).is_err());
    }

    #[test]
    fn prepared_image_identity_is_checked_even_if_the_length_matches() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("fixture.iso");
        let mut data = vec![0u8; 65536];
        data[32769..32774].copy_from_slice(b"CD001");
        let expected = format!("{:x}", Sha256::digest(&data));
        fs::write(&path, &data).unwrap();
        let mut valid = medium(&path, true).unwrap();
        verify_image(&mut valid.file, &expected).unwrap();
        assert!(OpenOptions::new().write(true).open(&path).is_err());
        drop(valid);
        data[27000] = 10;
        fs::write(&path, data).unwrap();
        let mut changed = medium(&path, true).unwrap();
        assert!(verify_image(&mut changed.file, &expected).is_err());
    }
}

#[cfg(test)]
mod session_contention {
    use super::*;
    #[test]
    fn session_read_survives_a_brief_exclusive_file_lock() {
        let root = tempfile::tempdir().unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let dir = root.path().join(&id);
        fs::create_dir(&dir).unwrap();
        let s = Session {
            id: id.clone(),
            iso: PathBuf::from("C:/test/boot.iso"),
            iso_sha256: None,
            definitions: None,
            base: None,
            base_sha256: None,
            network: "none".into(),
            state: "powered-off".into(),
            created_at: 0,
            detail: None,
            accelerator: None,
        };
        save(&dir, &s).unwrap();
        let locked = OpenOptions::new()
            .read(true)
            .share_mode(0)
            .open(dir.join("session.json"))
            .unwrap();
        let release = thread::spawn(move || {
            thread::sleep(Duration::from_millis(120));
            drop(locked);
        });
        let (_, observed) = owned_session(root.path(), &id).unwrap();
        assert_eq!(observed.state, "powered-off");
        release.join().unwrap();
    }
}
