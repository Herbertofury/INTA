use super::*;
use std::{
    ffi::{OsStr, OsString},
    fs::OpenOptions,
    os::windows::{
        ffi::{OsStrExt, OsStringExt},
        fs::{MetadataExt, OpenOptionsExt},
        io::AsRawHandle,
        process::CommandExt,
    },
    path::{Component, Prefix},
    process::{Command, Stdio},
    ptr, thread,
    time::{Duration, Instant},
};
use windows_sys::Win32::{
    Foundation::ERROR_INSUFFICIENT_BUFFER,
    Security::WinTrust::*,
    Storage::{
        FileSystem::*,
        Packaging::Appx::{GetPackagePathByFullName, GetPackagesByPackageFamily},
    },
    System::{Com::CoTaskMemFree, SystemInformation::GetSystemWindowsDirectoryW},
    UI::{Shell::*, WindowsAndMessaging::SW_SHOWNORMAL},
};

fn wide(s: &OsStr) -> Vec<u16> {
    s.encode_wide().chain([0]).collect()
}
pub(super) fn windows() -> Result<PathBuf> {
    let mut b = [0u16; 32768];
    let n = unsafe { GetSystemWindowsDirectoryW(b.as_mut_ptr(), b.len() as u32) } as usize;
    ensure!(n > 0 && n < b.len(), "Windows folder unavailable");
    Ok(OsString::from_wide(&b[..n]).into())
}
fn known(id: &windows_sys::core::GUID) -> Option<PathBuf> {
    let mut p = ptr::null_mut();
    if unsafe { SHGetKnownFolderPath(id, 0, ptr::null_mut(), &mut p) } != 0 || p.is_null() {
        return None;
    }
    let mut len = 0;
    unsafe {
        while *p.add(len) != 0 {
            len += 1
        }
        let path = OsString::from_wide(std::slice::from_raw_parts(p, len));
        CoTaskMemFree(p.cast());
        Some(path.into())
    }
}
pub(super) fn safe_local(p: &Path) -> Result<()> {
    ensure!(
        p.is_absolute()
            && matches!(p.components().next(),Some(Component::Prefix(pre)) if matches!(pre.kind(),Prefix::Disk(_))),
        "Choose an absolute local drive path"
    );
    for part in p.components().skip(1) {
        match part {
            Component::RootDir => (),
            Component::Normal(s) => {
                let n = s.to_str().context("Unsupported local path")?;
                let stem = n.split('.').next().unwrap_or("").to_ascii_uppercase();
                ensure!(
                    !n.contains([':', '"', '*', '?', '|', '<', '>'])
                        && !n.ends_with(['.', ' '])
                        && !n.chars().any(char::is_control)
                        && !matches!(
                            stem.as_str(),
                            "CON" | "NUL" | "PRN" | "AUX" | "CONIN$" | "CONOUT$"
                        )
                        && !["COM", "LPT"]
                            .iter()
                            .any(|v| stem.strip_prefix(v).is_some_and(|s| matches!(
                                s,
                                "1" | "2"
                                    | "3"
                                    | "4"
                                    | "5"
                                    | "6"
                                    | "7"
                                    | "8"
                                    | "9"
                                    | "¹"
                                    | "²"
                                    | "³"
                            ))),
                    "Unsafe file path"
                );
            }
            _ => bail!("Select a normalized path"),
        }
    }
    let drive = p.ancestors().last().context("Missing local drive")?;
    ensure!(
        matches!(
            unsafe { GetDriveTypeW(wide(drive.as_os_str()).as_ptr()) },
            2 | 3 | 6
        ),
        "Select a regular local volume"
    );
    Ok(())
}
pub(super) fn dirs(path: &Path, create: bool) -> Result<Vec<File>> {
    safe_local(path)?;
    let mut held = Vec::new();
    let mut chain = path.ancestors().collect::<Vec<_>>();
    chain.reverse();
    for dir in chain {
        if create && !dir.try_exists()? {
            match std::fs::create_dir(dir) {
                Ok(()) => (),
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => (),
                Err(e) => return Err(e.into()),
            }
        }
        let h = OpenOptions::new()
            .access_mode(FILE_READ_ATTRIBUTES)
            .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE)
            .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT | FILE_FLAG_BACKUP_SEMANTICS)
            .open(dir)?;
        let m = h.metadata()?;
        ensure!(
            m.is_dir() && m.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT == 0,
            "Linked directories are not accepted"
        );
        held.push(h);
    }
    Ok(held)
}
pub(super) struct Held {
    pub file: File,
    _parents: Vec<File>,
}
impl Held {
    pub fn open(p: &Path) -> Result<Self> {
        safe_local(p)?;
        let parents = dirs(p.parent().context("Missing parent directory")?, false)?;
        let file = OpenOptions::new()
            .read(true)
            .share_mode(FILE_SHARE_READ)
            .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
            .open(p)?;
        let m = file.metadata()?;
        ensure!(
            m.is_file() && m.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT == 0,
            "Expected a regular file"
        );
        Ok(Self {
            file,
            _parents: parents,
        })
    }
}
fn signature(path: &Path, file: &File) -> bool {
    let name = wide(path.as_os_str());
    let mut info = WINTRUST_FILE_INFO {
        cbStruct: size_of::<WINTRUST_FILE_INFO>() as u32,
        pcwszFilePath: name.as_ptr(),
        hFile: file.as_raw_handle(),
        ..Default::default()
    };
    let mut verify = WINTRUST_DATA {
        cbStruct: size_of::<WINTRUST_DATA>() as u32,
        dwUIChoice: WTD_UI_NONE,
        dwUnionChoice: WTD_CHOICE_FILE,
        Anonymous: WINTRUST_DATA_0 { pFile: &mut info },
        dwProvFlags: WTD_CACHE_ONLY_URL_RETRIEVAL,
        ..Default::default()
    };
    unsafe {
        WinVerifyTrust(
            ptr::null_mut(),
            &mut WINTRUST_ACTION_GENERIC_VERIFY_V2.clone(),
            (&mut verify as *mut WINTRUST_DATA).cast(),
        ) == 0
    }
}
fn registered(family: &str, exe: &str) -> Option<PathBuf> {
    let name = wide(OsStr::new(family));
    let mut count = 0;
    let mut size = 0;
    if unsafe {
        GetPackagesByPackageFamily(
            name.as_ptr(),
            &mut count,
            ptr::null_mut(),
            &mut size,
            ptr::null_mut(),
        )
    } != ERROR_INSUFFICIENT_BUFFER
        || count > 64
        || size > 32768
    {
        return None;
    }
    let mut pointers = vec![ptr::null_mut(); count as usize];
    let mut block = vec![0u16; size as usize];
    if unsafe {
        GetPackagesByPackageFamily(
            name.as_ptr(),
            &mut count,
            pointers.as_mut_ptr(),
            &mut size,
            block.as_mut_ptr(),
        )
    } != 0
    {
        return None;
    }
    for entry in pointers.into_iter().take(count as usize) {
        let mut n = 32768;
        let mut path = vec![0u16; n as usize];
        if unsafe { GetPackagePathByFullName(entry, &mut n, path.as_mut_ptr()) } == 0 && n > 1 {
            let p = PathBuf::from(OsString::from_wide(&path[..n as usize - 1])).join(exe);
            if p.is_file() {
                return Some(p);
            }
        }
    }
    None
}
pub(super) fn winget() -> Option<PathBuf> {
    registered("Microsoft.DesktopAppInstaller_8wekyb3d8bbwe", "winget.exe")
}
pub(super) fn packaged_app(id: &str) -> Option<&'static str> {
    if id == "chatgpt-desktop"
        && registered("OpenAI.ChatGPT-Desktop_2p2nqsd0c76g0", "AppxManifest.xml").is_some()
    {
        return Some(r"shell:AppsFolder\OpenAI.ChatGPT-Desktop_2p2nqsd0c76g0!ChatGPT");
    }
    None
}
fn known_install(rule: &Entry) -> Option<PathBuf> {
    rule.executables.iter().find_map(|s| {
        let (kind, rel) = s.split_once('/')?;
        if rel.contains('\\')
            || rel
                .split('/')
                .any(|s| s.is_empty() || s == "." || s == ".." || s.contains(':'))
        {
            return None;
        }
        let base = match kind {
            "pf" => known(&FOLDERID_ProgramFilesX64)?,
            "pf86" => known(&FOLDERID_ProgramFilesX86)?,
            "local" => known(&FOLDERID_LocalAppData)?,
            "system" => windows().ok()?.join("System32"),
            _ => return None,
        };
        let path = base.join(rel);
        let held = Held::open(&path).ok()?;
        if kind == "local" && !signature(&path, &held.file) {
            return None;
        }
        Some(path)
    })
}

pub(super) struct Installed {
    pub path: PathBuf,
    pub hash: Option<String>,
    pub provenance: &'static str,
}
pub(super) fn installed(rule: &Entry) -> Option<Installed> {
    known_install(rule)
        .map(|path| Installed {
            path,
            hash: None,
            provenance: "installed application",
        })
        .or_else(|| portable_install(rule))
}
fn portable_install(rule: &Entry) -> Option<Installed> {
    use winreg::{RegKey, enums::HKEY_CURRENT_USER};
    let package = rule.winget.as_deref().filter(|s| package_id(s))?;
    let source = "Microsoft.Winget.Source_8wekyb3d8bbwe";
    let key_name = format!("{package}_{source}");
    let key = RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey(format!(
            r"Software\Microsoft\Windows\CurrentVersion\Uninstall\{key_name}"
        ))
        .ok()?;
    let value = |s: &str| key.get_value::<String, _>(s).ok();
    if value("WinGetPackageIdentifier").as_deref() != Some(package)
        || value("WinGetSourceIdentifier").as_deref() != Some(source)
        || value("WinGetInstallerType").as_deref() != Some("portable")
    {
        return None;
    }
    let winget = known(&FOLDERID_LocalAppData)?.join("Microsoft/WinGet");
    let home = winget.join("Packages").join(key_name);
    if Path::new(&value("InstallLocation")?) != home {
        return None;
    }
    let _package = dirs(&home, false).ok()?;
    let declared = value("TargetFullPath").map(PathBuf::from);
    let digest = value("SHA256");
    let names = rule
        .filenames
        .iter()
        .map(String::as_str)
        .chain(rule.executables.iter().filter_map(|s| s.rsplit('/').next()));
    for name in names {
        if !base_allowed(rule, name) {
            continue;
        }
        let direct = home.join(name);
        let alias = winget.join("Links").join(name);
        // The link tells us which member WinGet exposed. Execute its verified regular target only.
        let via_link = std::fs::read_link(&alias).ok().and_then(|p| {
            let text = p.to_str()?;
            let path = PathBuf::from(text.strip_prefix(r"\\?\").unwrap_or(text));
            path.starts_with(&home).then_some(path)
        });
        for candidate in [declared.clone(), Some(direct), via_link]
            .into_iter()
            .flatten()
        {
            if !candidate.starts_with(&home)
                || !candidate
                    .file_name()
                    .is_some_and(|n| n.eq_ignore_ascii_case(name))
            {
                continue;
            }
            let Ok(mut held) = Held::open(&candidate) else {
                continue;
            };
            if !signature(&candidate, &held.file) {
                continue;
            }
            let mut hash = None;
            if declared.as_ref() == Some(&candidate) {
                let expected = digest.as_ref()?;
                if expected.len() != 64 || !sha(&mut held.file).ok()?.eq_ignore_ascii_case(expected)
                {
                    continue;
                }
                hash = Some(expected.to_ascii_lowercase());
            }
            return Some(Installed {
                path: candidate,
                hash,
                provenance: "verified WinGet portable package",
            });
        }
    }
    None
}
pub(super) fn shell(target: &OsStr, args: &str, cwd: Option<&Path>) -> Result<Value> {
    let target = wide(target);
    let arguments = wide(OsStr::new(args));
    let cwd = cwd.map(|v| wide(v.as_os_str()));
    let code = unsafe {
        ShellExecuteW(
            ptr::null_mut(),
            windows_sys::core::w!("open"),
            target.as_ptr(),
            arguments.as_ptr(),
            cwd.as_ref().map_or(ptr::null(), |v| v.as_ptr()),
            SW_SHOWNORMAL,
        )
    } as isize;
    ensure!(
        code > 32,
        "Windows could not open the application (shell code {code})"
    );
    Ok(json!({"state":"opened"}))
}
pub(super) fn open_uri(url: &str) -> Result<Value> {
    link(url)?;
    shell(OsStr::new(url), "", None)
}
// Registered MSIX directories may deny opening their parent WindowsApps directory even
// when the selected application is readable. The OS resolves the signed package family.
fn winget_file(path: &Path) -> Result<File> {
    ensure!(
        winget().as_deref() == Some(path),
        "Windows Package Manager registration changed"
    );
    let file = OpenOptions::new()
        .read(true)
        .share_mode(FILE_SHARE_READ)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
        .open(path)?;
    let m = file.metadata()?;
    ensure!(
        m.is_file() && m.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT == 0,
        "Invalid registered package file"
    );
    Ok(file)
}
pub(super) fn console(path: &Path, args: &str) -> Result<Value> {
    ensure!(
        args.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b" .+_-".contains(&b)),
        "Invalid fixed console operation"
    );
    independent_console(
        path,
        args,
        path.parent().context("Missing executable directory")?,
    )
}
pub(super) fn managed_terminal(id: &str, root: Option<&Path>) -> Result<Value> {
    ensure!(
        matches!(id, "7zip" | "restic" | "rclone" | "yara-x"),
        "Unknown managed program"
    );
    let mut args = format!("tool terminal {id} --approve");
    if let Some(folder) = root {
        safe_local(folder)?;
        // Slash form preserves a drive root inside Windows argument quotes.
        args.push_str(&format!(
            " --root \"{}\"",
            folder
                .to_str()
                .context("Invalid toolbox path")?
                .replace('\\', "/")
        ));
    }
    let path = std::env::current_exe()?;
    independent_console(
        &path,
        &args,
        path.parent().context("Missing executable directory")?,
    )
}
pub(super) fn arguments(input: &str) -> Result<Vec<OsString>> {
    ensure!(
        input.len() < 32_000 && !input.contains('\0'),
        "Argument line exceeds Windows limits"
    );
    let line = wide(OsStr::new(&format!("program {input}")));
    let mut count = 0;
    let ptr = unsafe { CommandLineToArgvW(line.as_ptr(), &mut count) };
    ensure!(!ptr.is_null(), "Invalid Windows argument line");
    let mut args = Vec::new();
    unsafe {
        for &s in std::slice::from_raw_parts(ptr, count as usize)
            .iter()
            .skip(1)
        {
            let mut len = 0;
            while *s.add(len) != 0 {
                len += 1;
            }
            args.push(OsString::from_wide(std::slice::from_raw_parts(s, len)));
        }
        windows_sys::Win32::Foundation::LocalFree(ptr.cast());
    }
    Ok(args)
}
fn independent_console(path: &Path, args: &str, cwd: &Path) -> Result<Value> {
    use windows_sys::Win32::{
        Foundation::CloseHandle,
        System::Threading::{
            CREATE_NEW_CONSOLE, CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW,
        },
    };
    ensure!(path.is_absolute(), "Expected an absolute executable path");
    let exe = wide(path.as_os_str());
    let directory = wide(cwd.as_os_str());
    let mut cmd = wide(OsStr::new(&format!("inta-tool {args}")));
    let startup = STARTUPINFOW {
        cb: size_of::<STARTUPINFOW>() as u32,
        ..Default::default()
    };
    let mut info = PROCESS_INFORMATION::default();
    let ok = unsafe {
        CreateProcessW(
            exe.as_ptr(),
            cmd.as_mut_ptr(),
            ptr::null(),
            ptr::null(),
            0,
            CREATE_NEW_CONSOLE,
            ptr::null(),
            directory.as_ptr(),
            &startup,
            &mut info,
        )
    };
    ensure!(
        ok != 0,
        "Opening the independent console failed: {}",
        io::Error::last_os_error()
    );
    unsafe {
        CloseHandle(info.hThread);
        CloseHandle(info.hProcess);
    }
    Ok(json!({"state":"opened","processId":info.dwProcessId}))
}
// The technician can read and accept the package and source terms in this console.
pub(super) fn install(path: &Path, id: &str) -> Result<Value> {
    ensure!(package_id(id), "Invalid package identifier");
    let _held = winget_file(path)?;
    let mut result = console(
        path,
        &format!("install --id {id} --exact --source winget --interactive --wait"),
    )?;
    result["state"] = json!("installer-opened");
    result["packageId"] = json!(id);
    Ok(result)
}
pub(super) fn bounded(mut cmd: Command, limit: Duration, cap: usize) -> Result<(i32, Vec<u8>)> {
    // A short-lived process can launch another process. A regular file lets the caller
    // finish reading even when the descendant keeps a duplicate output handle open.
    let mut output = tempfile::tempfile()?;
    cmd.creation_flags(0x08000000)
        .stdin(Stdio::null())
        .stdout(output.try_clone()?)
        .stderr(output.try_clone()?);
    let mut child = cmd.spawn()?;
    let until = Instant::now() + limit;
    let code = loop {
        if let Some(st) = child.try_wait()? {
            break st.code().unwrap_or(-1);
        }
        if Instant::now() >= until || output.metadata()?.len() > 2 * 1024 * 1024 {
            let _ = child.kill();
            let _ = child.wait();
            bail!("Tool exceeded its time or output limit");
        }
        thread::sleep(Duration::from_millis(35));
    };
    output.rewind()?;
    let mut bytes = Vec::new();
    output.take(cap as u64).read_to_end(&mut bytes)?;
    Ok((code, bytes))
}
fn help_args(id: &str) -> Option<&'static [&'static str]> {
    Some(match id {
        "procdump" | "sigcheck" => &["-?"],
        "pe-sieve" | "hollows-hunter" => &["/help"],
        "yara" | "wintools" => &["--help"],
        "restic" | "rclone" | "yara-x" | "clamav" | "smartmontools" | "fibratus" | "prx-sd"
        | "velociraptor" | "hayabusa" | "chainsaw" | "rustinel" | "sentinella" | "osquery"
        | "codex-cli" | "claude-code" | "gemini-cli" => &["--help"],
        _ => return None,
    })
}
pub(super) fn launch_file(path: &Path, id: &str, pin: Option<&String>) -> Result<Value> {
    let mut image = Held::open(path)?;
    check_pe(&mut image.file)?;
    if let Some(expected) = pin {
        ensure!(
            sha(&mut image.file)? == *expected,
            "The selected executable changed. Review and bind the updated file before launch."
        )
    } else if known(&FOLDERID_LocalAppData).is_some_and(|d| path.starts_with(d)) {
        ensure!(
            signature(path, &image.file),
            "The per-user app signature could not be verified"
        );
    }
    if matches!(id, "codex-cli" | "claude-code" | "gemini-cli") {
        let workspace = crate::db::data_dir()?.join("assistant-workspace");
        let _directory = dirs(&workspace, true)?;
        return independent_console(path, "", &workspace);
    }
    if let Some(args) = help_args(id) {
        return help(path, args);
    }
    if id == "python" {
        return console(path, "-I");
    }
    if id == "powershell" {
        return console(path, "-NoLogo -NoProfile");
    }
    let args = match id {
        "procmon" => "/NoConnect",
        "powershell" => "-NoLogo -NoProfile",
        _ => "",
    };
    shell(path.as_os_str(), args, path.parent())
}
fn help(path: &Path, args: &[&str]) -> Result<Value> {
    let _held = if path
        .file_name()
        .is_some_and(|n| n.eq_ignore_ascii_case("winget.exe"))
    {
        (None, Some(winget_file(path)?))
    } else {
        (Some(Held::open(path)?), None)
    };
    let mut cmd = Command::new(path);
    cmd.args(args)
        .current_dir(path.parent().context("Missing application folder")?);
    let (code, bytes) = bounded(cmd, Duration::from_secs(12), 48 * 1024)?;
    let text = if bytes.starts_with(&[0xff, 0xfe]) || bytes.get(1) == Some(&0) {
        String::from_utf16_lossy(
            &bytes
                .as_chunks::<2>()
                .0
                .iter()
                .map(|b| u16::from_le_bytes([b[0], b[1]]))
                .collect::<Vec<_>>(),
        )
    } else {
        String::from_utf8_lossy(&bytes).into_owned()
    };
    ensure!(
        !text.trim().is_empty(),
        "The tool returned no command help (exit {code})"
    );
    Ok(json!({"state":"command-help","output":text.trim(),"exitCode":code}))
}
pub(super) struct Builtin {
    pub label: &'static str,
    pub detail: &'static str,
    target: Target,
}
enum Target {
    Uri(&'static str),
    Reference(PathBuf, &'static str),
    Exe(Option<PathBuf>, &'static [&'static str]),
    Management(PathBuf, &'static str),
    Fixed(PathBuf, &'static str),
}
impl Builtin {
    pub fn kind(&self) -> &'static str {
        if matches!(&self.target, Target::Reference(..)) {
            "windows-reference"
        } else {
            "windows-component"
        }
    }
    pub fn available(&self) -> bool {
        match &self.target {
            Target::Uri(_) => true,
            Target::Reference(path, _) => path.is_file(),
            Target::Exe(p, _) => p.as_ref().is_some_and(|p| p.is_file()),
            Target::Management(dir, name) => {
                dir.join(name).is_file() && dir.join("mmc.exe").is_file()
            }
            Target::Fixed(path, _) => path.is_file(),
        }
    }
    pub fn run(&self) -> Result<Value> {
        match &self.target {
            Target::Uri(uri) => shell(OsStr::new(uri), "", None),
            Target::Reference(path, synopsis) => {
                let mut f = Held::open(path)?;
                Ok(
                    json!({"state":"local-command-reference","path":path,"sha256":sha(&mut f.file)?,"output":synopsis}),
                )
            }
            Target::Exe(Some(path), args) if !args.is_empty() => help(path, args),
            Target::Exe(Some(path), _) => shell(path.as_os_str(), "", path.parent()),
            Target::Fixed(path, args) => shell(path.as_os_str(), args, path.parent()),
            Target::Management(dir, name) => {
                let target = dir.join(name);
                let _snap = Held::open(&target)?;
                let mmc = dir.join("mmc.exe");
                let _engine = Held::open(&mmc)?;
                shell(
                    mmc.as_os_str(),
                    &format!("\"{}\"", target.display()),
                    Some(dir),
                )
            }
            _ => bail!("Windows component unavailable"),
        }
    }
}
pub(super) fn builtin(name: &str) -> Option<Builtin> {
    let system = windows().ok()?.join("System32");
    // These binaries require elevation even for native usage text on supported hosts.
    // Preserve the active token and provide local command syntax together with installed-file evidence.
    if unsafe { IsUserAnAdmin() } == 0 {
        let synopsis = match name {
            "diskpart" => Some(
                "DiskPart is the Windows disk partition command interpreter. An authorized administrator can request native command help using diskpart /?.",
            ),
            "winsat" => Some(
                "WinSAT reports Windows system assessment capabilities. An authorized administrator can inspect its native command reference using winsat -?.",
            ),
            "sfc" => Some(
                "System File Checker verifies or repairs Windows protected files. An authorized administrator can request native command help using sfc /?.",
            ),
            "dism" => Some(
                "Deployment Image Servicing and Management provides servicing commands for Windows images. An authorized administrator can inspect its native command groups using dism /?.",
            ),
            _ => None,
        };
        if let Some(text) = synopsis {
            return Some(Builtin {
                label: "Read local command reference",
                detail: "Show command syntax and verify the installed system binary. Native help requires an elevated Windows token.",
                target: Target::Reference(system.join(format!("{name}.exe")), text),
            });
        }
    }
    let (label, detail, target) = match name {
        "windows-security" => (
            "Open Windows Security",
            "Open the Windows Security dashboard.",
            Target::Uri("windowsdefender:"),
        ),
        "windows-update" => (
            "Open Windows Update",
            "Review Windows updates in Settings.",
            Target::Uri("ms-settings:windowsupdate"),
        ),
        "recovery-settings" => (
            "Open Recovery settings",
            "Review Windows recovery options in Settings.",
            Target::Uri("ms-settings:recovery"),
        ),
        "quick-assist" => (
            "Open Quick Assist",
            "Open the installed app without a connection or invitation.",
            Target::Uri("ms-quick-assist:"),
        ),
        "task-manager" => (
            "Open Task Manager",
            "Review applications and resource usage.",
            Target::Exe(Some(system.join("Taskmgr.exe")), &[]),
        ),
        "active-directory"
        | "event-viewer"
        | "device-manager"
        | "performance-monitor"
        | "hyper-v" => {
            let snap = match name {
                "event-viewer" => "eventvwr.msc",
                "device-manager" => "devmgmt.msc",
                "performance-monitor" => "perfmon.msc",
                "active-directory" => "dsa.msc",
                _ => "virtmgmt.msc",
            };
            (
                "Open management console",
                "Open the installed Windows management snap-in.",
                Target::Management(system, snap),
            )
        }
        "system-restore" => (
            "Open System Restore",
            "Review available restore options in the interactive wizard.",
            Target::Exe(Some(system.join("rstrui.exe")), &[]),
        ),
        "resource-monitor" => (
            "Open Resource Monitor",
            "Review system resource activity.",
            Target::Exe(Some(system.join("resmon.exe")), &[]),
        ),
        "reliability-monitor" => (
            "Open Reliability Monitor",
            "Inspect Windows reliability history.",
            Target::Fixed(system.join("perfmon.exe"), "/rel"),
        ),
        "bitlocker" => (
            "Open BitLocker settings",
            "Review disk encryption settings in Control Panel.",
            Target::Fixed(
                system.join("control.exe"),
                "/name Microsoft.BitLockerDriveEncryption",
            ),
        ),
        "command-prompt" => (
            "Open Command Prompt",
            "Open an interactive shell with AutoRun commands disabled.",
            Target::Fixed(system.join("cmd.exe"), "/d"),
        ),
        "chkdsk" | "bcdboot" | "diskpart" => (
            "Show command reference",
            "Show the Windows command help without selecting any physical disk or volume.",
            Target::Exe(Some(system.join(format!("{name}.exe"))), &["/?"]),
        ),
        "windows-terminal" => (
            "Open Windows Terminal",
            "Open a new interactive terminal.",
            Target::Exe(
                registered(
                    "Microsoft.WindowsTerminal_8wekyb3d8bbwe",
                    "WindowsTerminal.exe",
                ),
                &[],
            ),
        ),
        "powershell" => (
            "Open PowerShell",
            "Open an interactive Windows PowerShell console.",
            Target::Exe(
                Some(system.join("WindowsPowerShell/v1.0/powershell.exe")),
                &[],
            ),
        ),
        "winget" => (
            "Show WinGet commands",
            "Read Windows Package Manager command help.",
            Target::Exe(winget(), &["--help"]),
        ),
        "dism" | "sfc" | "winsat" | "robocopy" | "pktmon" | "openssh" => {
            let (exe, args): (&str, &[&str]) = match name {
                "dism" => ("dism.exe", &["/?"]),
                "sfc" => ("sfc.exe", &["/?"]),
                "winsat" => ("winsat.exe", &["-?"]),
                "robocopy" => ("robocopy.exe", &["/?"]),
                "pktmon" => ("pktmon.exe", &["help"]),
                _ => ("OpenSSH/ssh.exe", &["-Q", "cipher"]),
            };
            (
                "Show command reference",
                "Run the built-in read-only help or supported cipher listing.",
                Target::Exe(Some(system.join(exe)), args),
            )
        }
        _ => return None,
    };
    Some(Builtin {
        label,
        detail,
        target,
    })
}
