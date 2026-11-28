use super::{host, *};
use std::{
    fs,
    io::Write,
    os::windows::process::CommandExt,
    process::{Command, Stdio},
    time::{Duration, Instant},
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Pin {
    id: String,
    version: String,
    repo: String,
    tag: String,
    asset: String,
    asset_bytes: u64,
    asset_sha256: String,
    member: Option<String>,
    executable: String,
    exe_sha256: String,
    homepage: String,
    license: String,
}
fn pin(id: &str) -> Result<Pin> {
    serde_json::from_str::<Vec<Pin>>(include_str!("../../../../toolbox/pins.json"))?
        .into_iter()
        .find(|p| p.id == id)
        .context("Unknown managed tool")
}
fn home(p: &Pin) -> Result<PathBuf> {
    Ok(crate::db::data_dir()?
        .join("installed-tools")
        .join(&p.id)
        .join(&p.version))
}
fn toolpath(root: Option<&Path>, id: &str) -> Option<PathBuf> {
    let root = root?;
    let listed = inta_toolbox::list(root).ok()?;
    let rel = listed["packages"]
        .as_array()?
        .iter()
        .find(|v| v["id"] == id)?["relativePath"]
        .as_str()?;
    if !rel
        .split(['/', '\\'])
        .all(|s| !s.is_empty() && s != "." && s != ".." && !s.contains(':'))
    {
        return None;
    }
    let file = root.join(rel);
    file.is_file().then_some(file)
}
fn source(p: &Pin, root: Option<&Path>) -> Result<Option<PathBuf>> {
    let local = home(p)?.join(&p.executable);
    Ok(if local.try_exists()? {
        Some(local)
    } else {
        toolpath(root, &p.id)
    })
}
pub fn available(id: &str, root: Option<&Path>) -> Result<bool> {
    Ok(source(&pin(id)?, root)?.is_some())
}
pub fn terminal(id: &str, root: Option<&Path>) -> Result<Value> {
    use std::io::IsTerminal;
    ensure!(
        io::stdin().is_terminal(),
        "An interactive terminal is required"
    );
    let pin = pin(id)?;
    let exe = source(&pin, root)?.context("Install the verified program first")?;
    let _held = verified(&exe, &pin.exe_sha256)?;
    let mut folder = crate::db::data_dir()?.join("tool-workspace");
    let _workspace = host::dirs(&folder, true)?;
    println!(
        "INTA • {} {}\nEnter arguments for this program. Use help, pwd, cd \"absolute folder\" or exit.\nWorking directory: {}",
        id,
        pin.version,
        folder.display()
    );
    loop {
        print!("{id}> ");
        io::stdout().flush()?;
        let mut input = String::new();
        if io::stdin().read_line(&mut input)? == 0 {
            break;
        }
        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        if matches!(input, "exit" | "quit") {
            break;
        }
        if input == "pwd" {
            println!("{}", folder.display());
            continue;
        }
        let mut arguments = match host::arguments(input) {
            Ok(args) => args,
            Err(error) => {
                eprintln!("{error}");
                continue;
            }
        };
        if arguments.first().is_some_and(|v| v == "cd") {
            if arguments.len() != 2 {
                eprintln!("Use cd followed by one quoted absolute local directory.");
                continue;
            }
            let selected = PathBuf::from(&arguments[1]);
            match host::dirs(&selected, false) {
                Ok(_) => folder = selected,
                Err(e) => eprintln!("{e}"),
            };
            continue;
        }
        if input == "help" {
            arguments = vec![if id == "7zip" { "-h" } else { "--help" }.into()]
        }
        match Command::new(&exe)
            .args(arguments)
            .current_dir(&folder)
            .status()
        {
            Ok(status) => println!("Exit code {}", status.code().unwrap_or(-1)),
            Err(error) => eprintln!("Unable to run {id}: {error}"),
        }
    }
    Ok(json!({"state":"closed","tool":id}))
}
fn verified(path: &Path, expected: &str) -> Result<host::Held> {
    let mut f = host::Held::open(path)?;
    ensure!(
        f.file.metadata()?.len() <= 256 * 1024 * 1024 && sha(&mut f.file)? == expected,
        "SHA-256 mismatch. The original is preserved for inspection."
    );
    Ok(f)
}
fn notices(id: &str) -> &'static [(&'static str, &'static [u8])] {
    match id {
        "7zip" => &[
            ("License.txt", include_bytes!("notices/7zip-License.txt")),
            (
                "unRarLicense.txt",
                include_bytes!("notices/7zip-unRarLicense.txt"),
            ),
        ],
        "restic" => &[("LICENSE", include_bytes!("notices/restic-LICENSE"))],
        "rclone" => &[("COPYING", include_bytes!("notices/rclone-COPYING"))],
        "yara-x" => &[("LICENSE", include_bytes!("notices/yara-x-LICENSE"))],
        _ => &[],
    }
}
fn open_copy(from: &Path, to: &Path, expected: &str) -> Result<()> {
    let mut got = verified(from, expected)?;
    let mut dest = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(to)?;
    io::copy(&mut got.file, &mut dest)?;
    dest.sync_all()?;
    Ok(())
}
fn run_file(mut cmd: Command, out: File, seconds: u64) -> Result<()> {
    let mut child = cmd
        .creation_flags(0x08000000)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .stdout(out)
        .spawn()?;
    let until = Instant::now() + Duration::from_secs(seconds);
    loop {
        if let Some(status) = child.try_wait()? {
            ensure!(
                status.success(),
                "Verified acquisition utility returned {status}"
            );
            return Ok(());
        }
        if Instant::now() > until {
            let _ = child.kill();
            let _ = child.wait();
            bail!("Acquisition timed out")
        }
        std::thread::sleep(Duration::from_millis(70));
    }
}
fn acquire(p: &Pin, root: Option<&Path>, stage: &Path, to: &Path) -> Result<&'static str> {
    if let Some(exe) = toolpath(root, &p.id) {
        open_copy(&exe, to, &p.exe_sha256)?;
        return Ok("verified-toolbox");
    }
    let cache = root
        .map(|r| r.join("93 - Offline Package Cache").join(&p.asset_sha256))
        .filter(|f| f.is_file());
    let (artifact, origin) = if let Some(cache) = cache {
        (cache, "verified-offline-cache")
    } else {
        let curl = host::windows()?.join("System32/curl.exe");
        let _curl = host::Held::open(&curl)?;
        let url = format!(
            "https://github.com/{}/releases/download/{}/{}",
            p.repo, p.tag, p.asset
        );
        let payload = stage.join("download");
        let mut cmd = Command::new(curl);
        cmd.current_dir(stage).args([
            "--disable",
            "--silent",
            "--fail",
            "--proto",
            "=https",
            "--proto-redir",
            "=https",
            "--location",
            "--max-redirs",
            "5",
            "--max-time",
            "100",
            "--max-filesize",
            &p.asset_bytes.to_string(),
            "--",
            &url,
        ]);
        run_file(cmd, File::create_new(&payload)?, 110)?;
        (payload, "verified-official-release")
    };
    let mut asset = verified(&artifact, &p.asset_sha256)?;
    ensure!(
        asset.file.metadata()?.len() == p.asset_bytes,
        "Unexpected release asset length"
    );
    let mut out = File::create_new(to)?;
    if let Some(member) = &p.member {
        let tar = host::windows()?.join("System32/tar.exe");
        let _tar = host::Held::open(&tar)?;
        let mut cmd = Command::new(tar);
        cmd.current_dir(stage)
            .arg("-xOf")
            .arg(&artifact)
            .arg("--")
            .arg(member);
        run_file(cmd, out, 35)?;
    } else {
        io::copy(&mut asset.file, &mut out)?;
        out.sync_all()?;
    }
    Ok(origin)
}
pub fn install(id: &str, root: Option<&Path>) -> Result<Value> {
    let p = pin(id)?;
    let dest = home(&p)?;
    let parent = dest.parent().context("Missing installation folder")?;
    let _parents = host::dirs(parent, true)?;
    let exe = dest.join(&p.executable);
    let mut origin = "existing-verified-install";
    if !dest.try_exists()? {
        let stage = tempfile::Builder::new()
            .prefix(".inta-package-")
            .tempdir_in(parent)?;
        let publish = stage.path().join("install");
        fs::create_dir(&publish)?;
        let output = publish.join(&p.executable);
        origin = acquire(&p, root, stage.path(), &output)?;
        let _ = verified(&output, &p.exe_sha256)?;
        for (name, data) in notices(id) {
            File::create_new(publish.join(name))?.write_all(data)?;
        }
        let receipt = json!({"id":id,"version":p.version,"sha256":p.exe_sha256,"source":p.homepage,"upstreamRelease":format!("https://github.com/{}/releases/tag/{}",p.repo,p.tag),"license":p.license});
        File::create_new(publish.join("tool.json"))?
            .write_all(&serde_json::to_vec_pretty(&receipt)?)?;
        if let Err(e) = fs::rename(&publish, &dest)
            && !dest.try_exists()?
        {
            return Err(e.into());
        }
    }
    let _held = verified(&exe, &p.exe_sha256)?;
    for (name, expected) in notices(id) {
        let mut f = host::Held::open(&dest.join(name))?;
        ensure!(
            f.file.metadata()?.len() == expected.len() as u64
                && sha(&mut f.file)? == format!("{:x}", Sha256::digest(expected)),
            "Managed tool notice integrity mismatch"
        );
    }
    Ok(
        json!({"state":"installed","version":p.version,"sha256":p.exe_sha256,"path":exe,"source":origin}),
    )
}
pub fn label(id: &str) -> &'static str {
    match id {
        "restic" => "Run local backup and restore",
        "rclone" => "Copy and verify sample files",
        "7zip" => "Create and verify a sample archive",
        "yara-x" => "Test a sample detection rule",
        _ => "Run verified workflow",
    }
}
pub fn workflow(id: &str, root: Option<&Path>) -> Result<Value> {
    let p = pin(id)?;
    let src = source(&p, root)?.context("Install the verified tool first")?;
    let temp = tempfile::Builder::new()
        .prefix("inta-tool-job-")
        .tempdir_in(crate::db::data_dir()?)?;
    let dir = temp.path();
    let _parents = host::dirs(dir, false)?;
    let exe = dir.join(&p.executable);
    open_copy(&src, &exe, &p.exe_sha256)?;
    let _seal = verified(&exe, &p.exe_sha256)?;
    let payload = b"INTA local workflow sample\r\n";
    fs::write(dir.join("sample.txt"), payload)?;
    let system = host::windows()?;
    let mut operations = Vec::new();
    let mut run = |args: &[&str]| -> Result<Vec<u8>> {
        let mut cmd = Command::new(&exe);
        cmd.args(args)
            .current_dir(dir)
            .env_clear()
            .env("SystemRoot", &system)
            .env("WINDIR", &system)
            .env("PATH", system.join("System32"))
            .env("TEMP", dir)
            .env("TMP", dir)
            .env("USERPROFILE", dir)
            .env("APPDATA", dir)
            .env("LOCALAPPDATA", dir)
            .env("RESTIC_REPOSITORY", dir.join("repo"))
            .env("RESTIC_CACHE_DIR", dir.join("cache"));
        let (rc, out) = host::bounded(cmd, Duration::from_secs(90), 16384)?;
        ensure!(rc == 0, "{} returned exit code {rc} for {}", p.id, args[0]);
        operations.push(args[0].to_owned());
        Ok(out)
    };
    match id {
        "7zip" => {
            run(&["a", "-t7z", "sample.7z", "sample.txt"])?;
            run(&["t", "sample.7z"])?;
            ensure!(
                run(&["x", "-so", "sample.7z", "sample.txt"])? == payload,
                "Archived content differed"
            );
        }
        "rclone" => {
            fs::create_dir(dir.join("input"))?;
            fs::rename(dir.join("sample.txt"), dir.join("input/sample.txt"))?;
            fs::write(dir.join("empty.conf"), [])?;
            run(&["copy", "input", "copied", "--config", "empty.conf"])?;
            run(&[
                "check",
                "--download",
                "input",
                "copied",
                "--config",
                "empty.conf",
            ])?;
            ensure!(
                fs::read(dir.join("copied/sample.txt"))? == payload,
                "Copied content differed"
            );
        }
        "restic" => {
            run(&["init", "--insecure-no-password"])?;
            run(&["backup", "--insecure-no-password", "sample.txt"])?;
            run(&["check", "--insecure-no-password", "--read-data"])?;
            run(&[
                "restore",
                "--insecure-no-password",
                "latest",
                "--target",
                "restored",
            ])?;
            let sample = walkdir::WalkDir::new(dir.join("restored"))
                .into_iter()
                .filter_map(Result::ok)
                .find(|e| e.file_name() == "sample.txt")
                .context("Restored sample missing")?;
            ensure!(
                fs::read(sample.path())? == payload,
                "Restored content differed"
            );
        }
        "yara-x" => {
            fs::write(dir.join("sample.yar"),br#"rule INTA_fixture { strings: $s = "INTA local workflow sample" condition: $s }"#)?;
            fs::write(dir.join("negative.txt"), b"ordinary negative control")?;
            ensure!(
                String::from_utf8_lossy(&run(&["scan", "sample.yar", "sample.txt"])?)
                    .contains("INTA_fixture"),
                "Positive control missing"
            );
            ensure!(
                !String::from_utf8_lossy(&run(&["scan", "sample.yar", "negative.txt"])?)
                    .contains("INTA_fixture"),
                "Negative control matched"
            );
        }
        _ => bail!("Unknown managed workflow"),
    }
    Ok(
        json!({"state":"completed","workflow":label(id),"scope":"temporary local sample","operations":operations,"sampleSha256":format!("{:x}",Sha256::digest(payload)),"toolSha256":p.exe_sha256,"version":p.version}),
    )
}
