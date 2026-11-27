mod release;
use sha2::{Digest, Sha256};
use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::Path,
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn command(program: &str, args: &[&str], cwd: Option<&Path>) -> Result<()> {
    let mut cmd = Command::new(program);
    cmd.args(args);
    if let Some(cwd) = cwd {
        cmd.current_dir(cwd);
    }
    let status = cmd.status()?;
    if !status.success() {
        return Err(format!("{program} failed with {status}").into());
    }
    Ok(())
}
fn text_command(program: &str, args: &[&str]) -> Result<String> {
    let out = Command::new(program).args(args).output()?;
    if !out.status.success() {
        return Err(format!("{program} failed: {}", String::from_utf8_lossy(&out.stderr)).into());
    }
    Ok(String::from_utf8(out.stdout)?.trim().to_owned())
}
fn sha256(path: &Path) -> Result<String> {
    let mut input = File::open(path)?;
    let mut hash = Sha256::new();
    let mut buffer = vec![0u8; 1024 * 1024];
    loop {
        let n = input.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hash.update(&buffer[..n]);
    }
    Ok(format!("{:x}", hash.finalize()))
}
fn copy_tree(from: &Path, to: &Path) -> Result<()> {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let source = entry.path();
        let destination = to.join(entry.file_name());
        let kind = entry.file_type()?;
        if kind.is_dir() {
            copy_tree(&source, &destination)?;
        } else if kind.is_file() {
            fs::copy(source, destination)?;
        } else {
            return Err("Unexpected non-regular file in Rescue source tree".into());
        }
    }
    Ok(())
}
fn executable_copy(from: &Path, to: &Path) -> Result<()> {
    if let Some(parent) = to.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(from, to)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(to, fs::Permissions::from_mode(0o755))?;
    }
    Ok(())
}
fn chroot(build: &Path, program: &str, args: &[&str], cwd: Option<&str>) -> Result<()> {
    let root = build.join("chroot");
    let mut cmd = Command::new("chroot");
    cmd.arg(&root);
    if let Some(cwd) = cwd {
        cmd.arg("/usr/bin/env").arg(format!("--chdir={cwd}"));
    }
    cmd.arg(program).args(args);
    let status = cmd.status()?;
    if !status.success() {
        return Err(format!("chroot {program} failed with {status}").into());
    }
    Ok(())
}

fn iso(source: &Path, out: &Path, binaries: &Path) -> Result<()> {
    if text_command("id", &["-u"])? != "0"
        || text_command("dpkg", &["--print-architecture"])? != "amd64"
    {
        return Err("Run on an amd64 Debian 13 build host as root.".into());
    }
    let release = fs::read_to_string("/etc/os-release")?;
    if !release.lines().any(|s| s == "ID=debian")
        || !release
            .lines()
            .any(|s| s == "VERSION_ID=\"13\"" || s == "VERSION_ID=13")
    {
        return Err("The Rescue recipe requires Debian 13.".into());
    }
    let source = source.canonicalize()?;
    let binaries = binaries.canonicalize()?;
    fs::create_dir_all(out)?;
    let out = out.canonicalize()?;
    for binary in ["inta-rescue", "inta-rescue-ops"] {
        if !binaries.join(binary).is_file() {
            return Err(format!("Build the Rust binary first: {binary}").into());
        }
    }
    let temporary = tempfile::tempdir()?;
    let build = temporary.path().to_owned();
    let result = (|| -> Result<()> {
        command(
            "lb",
            &[
                "config",
                "--mode",
                "debian",
                "--distribution",
                "trixie",
                "--architectures",
                "amd64",
                "--archive-areas",
                "main contrib non-free-firmware",
                "--binary-images",
                "iso-hybrid",
                "--bootloaders",
                "grub-pc grub-efi",
                "--debian-installer",
                "none",
                "--firmware-chroot",
                "true",
                "--apt-recommends",
                "false",
                "--security",
                "true",
                "--updates",
                "true",
                "--memtest",
                "none",
                "--iso-application",
                "INTA Rescue",
                "--iso-volume",
                "INTA_RESCUE",
                "--bootappend-live",
                "boot=live components username=inta hostname=inta quiet",
            ],
            Some(&build),
        )?;
        copy_tree(&source.join("live"), &build.join("config"))?;
        // Old hook scripts are superseded by exact Rust-controlled chroot steps below.
        for relative in [
            "hooks/live/0900-ai.hook.chroot",
            "hooks/live/0950-session.hook.chroot",
        ] {
            let path = build.join("config").join(relative);
            if path.exists() {
                fs::remove_file(path)?;
            }
        }
        let installed = build.join("config/includes.chroot");
        executable_copy(
            &binaries.join("inta-rescue"),
            &installed.join("usr/local/bin/inta-rescue"),
        )?;
        for name in ["inta-rescue-ops", "inta-self-test", "inta-boot-smoke"] {
            executable_copy(
                &binaries.join("inta-rescue-ops"),
                &installed.join("usr/local/bin").join(name),
            )?;
        }
        let resources = installed.join("usr/share/inta");
        fs::create_dir_all(resources.join("data"))?;
        fs::copy(
            source.join("data/brand-icons.json"),
            resources.join("data/brand-icons.json"),
        )?;
        copy_tree(
            &source.join("assets/brand-icons"),
            &resources.join("assets/brand-icons"),
        )?;
        fs::create_dir_all(installed.join("usr/share/icons/hicolor/scalable/apps"))?;
        fs::copy(
            source.join("docs/assets/inta-mark.svg"),
            installed.join("usr/share/icons/hicolor/scalable/apps/inta-rescue.svg"),
        )?;
        command("lb", &["bootstrap"], Some(&build))?;
        command("lb", &["chroot"], Some(&build))?;
        fs::create_dir_all(build.join("chroot/opt/inta-ai"))?;
        chroot(&build, "npm", &["init", "--yes"], Some("/opt/inta-ai"))?;
        chroot(
            &build,
            "npm",
            &[
                "install",
                "--save-exact",
                "--ignore-scripts",
                "--no-audit",
                "--no-fund",
                "@openai/codex",
                "@anthropic-ai/claude-code",
            ],
            Some("/opt/inta-ai"),
        )?;
        if build
            .join("chroot/opt/inta-ai/node_modules/@anthropic-ai/claude-code/install.cjs")
            .is_file()
        {
            chroot(
                &build,
                "node",
                &["node_modules/@anthropic-ai/claude-code/install.cjs"],
                Some("/opt/inta-ai"),
            )?;
        }
        for client in ["codex", "claude"] {
            chroot(
                &build,
                &format!("/opt/inta-ai/node_modules/.bin/{client}"),
                &["--version"],
                None,
            )?;
        }
        // Read-only scans are on demand; signature updates never start in the background.
        chroot(
            &build,
            "systemctl",
            &["disable", "clamav-freshclam.service"],
            None,
        )?;
        command("lb", &["binary"], Some(&build))?;
        let mut images: Vec<_> = fs::read_dir(&build)?
            .filter_map(std::result::Result::ok)
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|s| s == "iso"))
            .collect();
        images.sort();
        if images.len() != 1 {
            return Err("Expected exactly one built ISO.".into());
        }
        fs::copy(&images[0], out.join("inta-rescue-amd64.iso"))?;
        let mut manifests: Vec<_> = fs::read_dir(&build)?
            .filter_map(std::result::Result::ok)
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|s| s == "packages"))
            .collect();
        manifests.sort();
        if let Some(manifest) = manifests.first() {
            fs::copy(manifest, out.join("inta-rescue-amd64.packages"))?;
        }
        let digest = sha256(&out.join("inta-rescue-amd64.iso"))?;
        fs::write(
            out.join("SHA256SUMS"),
            format!("{digest}  inta-rescue-amd64.iso\n"),
        )?;
        println!("Built complete Rescue image: SHA-256 {digest}");
        Ok(())
    })();
    let cleaned = command("lb", &["clean"], Some(&build)).is_ok();
    let mounted = ["chroot/proc", "chroot/sys", "chroot/dev"]
        .iter()
        .any(|path| {
            Command::new("mountpoint")
                .arg("-q")
                .arg(build.join(path))
                .status()
                .map(|s| s.success())
                .unwrap_or(true)
        });
    if !cleaned || mounted {
        let retained = temporary.keep();
        eprintln!(
            "Build directory retained for safe cleanup: {}",
            retained.display()
        );
    }
    result
}

fn installed_test() -> Result<()> {
    for exe in [
        "xfce4-session",
        "lightdm",
        "firefox-esr",
        "gparted",
        "gsmartcontrol",
        "smartctl",
        "nvme",
        "clonezilla",
        "testdisk",
        "photorec",
        "ddrescue",
        "mmls",
        "dislocker",
        "hivexsh",
        "clamscan",
        "yara",
        "tcpdump",
    ] {
        if !env::split_paths(&env::var_os("PATH").unwrap_or_default())
            .any(|path| path.join(exe).is_file())
        {
            return Err(format!("Missing required installed tool: {exe}").into());
        }
    }
    command("/usr/local/bin/inta-rescue", &["--self-test"], None)?;
    if !Path::new("/usr/share/icons/hicolor/scalable/apps/inta-rescue.svg").is_file() {
        return Err("Missing INTA desktop icon".into());
    }
    for client in ["codex", "claude"] {
        command(
            &format!("/opt/inta-ai/node_modules/.bin/{client}"),
            &["--version"],
            None,
        )?;
    }
    Ok(())
}
fn boot_smoke() -> Result<()> {
    let serial = fs::read_to_string("/sys/class/dmi/id/product_serial").unwrap_or_default();
    if serial.trim() != "INTA_CI_SMOKE" {
        return Ok(());
    }
    let mut log = OpenOptions::new().write(true).open("/dev/ttyS0")?;
    writeln!(log, "\nINTA_SMOKE_START")?;
    let result = (|| -> Result<()> {
        for _ in 0..90 {
            let desktop = Command::new("pgrep")
                .args(["-u", "inta", "-x", "xfce4-session"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()?
                .success();
            let app = Command::new("pgrep")
                .args(["-u", "inta", "-f", "/usr/local/bin/inta-rescue"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()?
                .success();
            if desktop && app {
                let status = Command::new("runuser")
                    .args(["-u", "inta", "--", "/usr/local/bin/inta-self-test"])
                    .stdout(log.try_clone()?)
                    .stderr(log.try_clone()?)
                    .status()?;
                if status.success() {
                    return Ok(());
                }
                return Err("Installed application self-test failed".into());
            }
            thread::sleep(Duration::from_secs(2));
        }
        Err("Desktop or native Rescue app did not become ready.".into())
    })();
    if result.is_ok() {
        writeln!(log, "\nINTA_SMOKE_PASS")?;
    } else {
        writeln!(log, "\nINTA_SMOKE_FAIL: {:?}", result.as_ref().err())?;
        for (program, args) in [
            (
                "systemctl",
                vec!["--no-pager", "status", "lightdm", "live-config"],
            ),
            (
                "journalctl",
                vec![
                    "--no-pager",
                    "-b",
                    "-u",
                    "lightdm",
                    "-u",
                    "live-config",
                    "-n",
                    "80",
                ],
            ),
        ] {
            let _ = Command::new(program)
                .args(args)
                .stdout(log.try_clone()?)
                .stderr(log.try_clone()?)
                .status();
        }
    }
    log.flush()?;
    let _ = Command::new("systemctl")
        .args(["poweroff", "--no-block"])
        .status();
    result
}

#[cfg(unix)]
fn monitor(socket: &Path, command: &str) -> Result<String> {
    use std::os::unix::net::UnixStream;
    let mut stream = UnixStream::connect(socket)?;
    stream.set_read_timeout(Some(Duration::from_secs(3)))?;
    stream.set_write_timeout(Some(Duration::from_secs(3)))?;
    fn prompt(stream: &mut UnixStream) -> Result<String> {
        let mut output = Vec::new();
        let mut buffer = [0u8; 4096];
        while !output.windows(6).any(|s| s == b"(qemu)") {
            let n = stream.read(&mut buffer)?;
            if n == 0 {
                return Err("QEMU monitor closed before acknowledging the command".into());
            }
            output.extend_from_slice(&buffer[..n]);
            if output.len() > 65536 {
                return Err("QEMU monitor response exceeds limit".into());
            }
        }
        Ok(String::from_utf8_lossy(&output).into_owned())
    }
    prompt(&mut stream)?;
    writeln!(stream, "{command}")?;
    let response = prompt(&mut stream)?;
    if ["error", "unknown", "invalid"]
        .iter()
        .any(|s| response.to_lowercase().contains(s))
    {
        return Err(format!("QEMU rejected command: {response}").into());
    }
    Ok(response)
}
#[cfg(not(unix))]
fn monitor(_socket: &Path, _command: &str) -> Result<String> {
    Err("QEMU smoke verification requires a Unix build host.".into())
}
fn smoke_iso(iso: &Path, mode: &str, diagnostics: &Path) -> Result<()> {
    if !matches!(mode, "bios" | "uefi") {
        return Err("Firmware must be bios or uefi".into());
    }
    let iso = iso.canonicalize()?;
    fs::create_dir_all(diagnostics)?;
    let diagnostics = diagnostics.canonicalize()?;
    let work = tempfile::tempdir()?;
    let serial = diagnostics.join(format!("{mode}-serial.log"));
    let socket = work.path().join("monitor");
    let acceleration = if OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/kvm")
        .is_ok()
    {
        "kvm"
    } else {
        "tcg"
    };
    let cpu = if acceleration == "kvm" { "host" } else { "max" };
    let mut cmd = Command::new("qemu-system-x86_64");
    cmd.args([
        "-machine",
        &format!("q35,accel={acceleration}"),
        "-cpu",
        cpu,
        "-smp",
        "2",
        "-m",
        "3072",
        "-nic",
        "none",
        "-display",
        "none",
        "-monitor",
        &format!("unix:{},server=on,wait=off", socket.display()),
        "-serial",
        &format!("file:{}", serial.display()),
        "-boot",
        "order=d,strict=on",
        "-smbios",
        "type=1,serial=INTA_CI_SMOKE",
        "-cdrom",
    ])
    .arg(&iso);
    if mode == "uefi" {
        let vars = work.path().join("vars.fd");
        fs::copy("/usr/share/OVMF/OVMF_VARS_4M.fd", &vars)?;
        cmd.args([
            "-drive",
            "if=pflash,unit=0,format=raw,readonly=on,file=/usr/share/OVMF/OVMF_CODE_4M.fd",
            "-drive",
            &format!("if=pflash,unit=1,format=raw,file={}", vars.display()),
        ]);
    }
    println!("Booting INTA via {mode} ({acceleration})");
    let mut guest = cmd.spawn()?;
    let start = Instant::now();
    let mut selected = false;
    let mut pictures = [false; 3];
    let result = (|| -> Result<()> {
        loop {
            if let Some(status) = guest.try_wait()? {
                let log = fs::read_to_string(&serial).unwrap_or_default();
                print!("{log}");
                if !status.success()
                    || !log
                        .lines()
                        .any(|line| line.trim_end_matches('\r') == "INTA_SMOKE_PASS")
                {
                    return Err(format!(
                        "{mode} boot did not pass the complete desktop/application gate"
                    )
                    .into());
                }
                return Ok(());
            }
            let seconds = start.elapsed().as_secs();
            if seconds >= 340 {
                print!("{}", fs::read_to_string(&serial).unwrap_or_default());
                return Err(format!("Timed out waiting for {mode} desktop readiness").into());
            }
            if seconds >= 35 && !selected {
                monitor(&socket, "sendkey ret")?;
                selected = true;
            }
            for (index, threshold) in [35, 175, 325].into_iter().enumerate() {
                if seconds >= threshold && !pictures[index] {
                    let path = diagnostics.join(format!("{mode}-{threshold}.ppm"));
                    let _ = monitor(&socket, &format!("screendump \"{}\"", path.display()));
                    pictures[index] = true;
                }
            }
            thread::sleep(Duration::from_secs(5));
        }
    })();
    if result.is_err() {
        let _ = guest.kill();
        let _ = guest.wait();
    }
    result
}

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    let name = Path::new(&args[0])
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    if name == "inta-self-test" {
        return installed_test();
    }
    if name == "inta-boot-smoke" {
        return boot_smoke();
    }
    match args.get(1).map(String::as_str){
        Some("iso")if args.len()==5=>iso(Path::new(&args[2]),Path::new(&args[3]),Path::new(&args[4])),
        Some("smoke")if args.len()==5=>smoke_iso(Path::new(&args[2]),&args[3],Path::new(&args[4])),
        Some("package")if args.len()==4=>{println!("{}",serde_json::to_string_pretty(&release::package(Path::new(&args[2]),Path::new(&args[3]),1024*1024*1024,2*1024*1024*1024)?)?);Ok(())},
        Some("installed-test")=>installed_test(),Some("boot-smoke")=>boot_smoke(),
        Some("sha256")if args.len()==3=>{println!("{}",sha256(Path::new(&args[2]))?);Ok(())},
        _=>Err("Usage: inta-rescue-ops iso <source> <output> <native-binaries> | smoke <iso> <bios|uefi> <diagnostics> | installed-test | boot-smoke | sha256 <file>".into()),
    }
}
