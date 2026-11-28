use super::{err, guarded_dirs, local, read_file, read_json, safe_rel, sha, MAX_BASE};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    env,
    fs::{self, File},
    os::windows::process::CommandExt,
    path::{Path, PathBuf},
    process::Command,
    time::Instant,
};

#[derive(Deserialize)]
struct ImagePin {
    bytes: u64,
    sha256: String,
}
#[derive(Deserialize)]
struct RescuePins {
    images: Vec<ImagePin>,
}
const RECOVERED: &str = "14516d423e8f53c66e0af6eec66cf942606e571efe2861058ac8e78911507881";
const ASSETS: &[(&str, &[u8])] = &[
    ("runner.py", include_bytes!("runner.py")),
    (
        "scripts/smoke-rescue-repair.py",
        include_bytes!("../../scripts/smoke-rescue-repair.py"),
    ),
    (
        "scripts/cloud-rescue-console.py",
        include_bytes!("../../scripts/cloud-rescue-console.py"),
    ),
    (
        "media/inta-media.py",
        include_bytes!("../../media/inta-media.py"),
    ),
    (
        "media/rescue_iso.py",
        include_bytes!("../../media/rescue_iso.py"),
    ),
    (
        "media/rescue_devices.py",
        include_bytes!("../../media/rescue_devices.py"),
    ),
    (
        "tests/rescue-lab/console.sh",
        include_bytes!("../../tests/rescue-lab/console.sh"),
    ),
    (
        "tests/rescue-lab/repair.sh",
        include_bytes!("../../tests/rescue-lab/repair.sh"),
    ),
];

#[derive(Deserialize)]
struct Layout {
    iso: String,
}

fn wsl() -> Result<PathBuf, String> {
    env::var_os("SystemRoot")
        .map(|p| PathBuf::from(p).join("System32/wsl.exe"))
        .filter(|p| p.is_file())
        .ok_or("WSL2 is unavailable on this Windows host.".into())
}
pub(super) fn enabled() -> bool {
    wsl().is_ok()
        && env::var_os("LOCALAPPDATA").is_some_and(|p| {
            PathBuf::from(p)
                .join("INTA/lab/cloud-hypervisor-v55.enabled")
                .is_file()
        })
}
fn linux(path: &Path) -> Result<String, String> {
    local(path)?;
    let value = path
        .to_str()
        .ok_or("Use a Unicode local drive path")?
        .replace('\\', "/");
    let bytes = value.as_bytes();
    if bytes.len() < 3 || !bytes[0].is_ascii_alphabetic() || &bytes[1..3] != b":/" {
        return Err("Expected a normalized drive path".into());
    }
    Ok(format!(
        "/mnt/{}/{}",
        (bytes[0] as char).to_ascii_lowercase(),
        &value[3..]
    ))
}
pub(super) fn run(toolbox: &Path, console: bool, base: Option<&Path>) -> Result<Value, String> {
    if !enabled() {
        return Err(
            "The prepared Cloud Hypervisor WSL2 profile is unavailable on this host.".into(),
        );
    }
    let started = Instant::now();
    let _toolbox = guarded_dirs(toolbox)?;
    let layout: Layout = read_json(&toolbox.join(".inta-cloud-lab.json"))
        .map_err(|_| "Select a toolbox containing the INTA Rescue Cloud Hypervisor profile.")?;
    let image = toolbox.join(safe_rel(&layout.iso)?);
    let _parents = guarded_dirs(image.parent().ok_or("Invalid ISO path")?)?;
    let mut input = read_file(&image)?;
    let pins: RescuePins =
        serde_json::from_str(include_str!("../../media/rescue-releases.json")).map_err(err)?;
    let size = input.metadata().map_err(err)?.len();
    if !pins.images.iter().any(|p| p.bytes == size) {
        return Err("The rescue ISO has an unrecognized size.".into());
    }
    let image_hash = sha(&mut input)?;
    if !pins
        .images
        .iter()
        .any(|p| p.bytes == size && p.sha256 == image_hash)
    {
        return Err("The rescue ISO failed its pinned integrity check.".into());
    }
    let _sums = read_file(&image.with_file_name("SHA256SUMS"))?;
    let scratch = tempfile::Builder::new()
        .prefix("INTA-Cloud-")
        .tempdir()
        .map_err(err)?;
    let mut guards = Vec::<File>::new();
    for (name, bytes) in ASSETS {
        let p = scratch.path().join(name);
        let dir = p.parent().ok_or("Invalid runner asset path")?;
        fs::create_dir_all(dir).map_err(err)?;
        fs::write(&p, bytes).map_err(err)?;
        guards.extend(guarded_dirs(dir)?);
        guards.push(read_file(&p)?);
    }
    let mut cmd = Command::new(wsl()?);
    cmd.args([
        "--distribution",
        "INTA-Rescue-Builder",
        "--user",
        "inta-lab",
        "--cd",
        "/",
        "--exec",
        "/usr/bin/python3",
        "-I",
        "-B",
    ])
    .arg(linux(&scratch.path().join("runner.py"))?)
    .arg(if console { "console" } else { "repair" })
    .arg(linux(&image)?);
    if let Some(path) = base {
        if !path
            .extension()
            .is_some_and(|s| s.eq_ignore_ascii_case("img"))
        {
            return Err("Select a flat .img disk".into());
        }
        guards.extend(guarded_dirs(path.parent().ok_or("Missing image parent")?)?);
        let file = read_file(path)?;
        let n = file.metadata().map_err(err)?.len();
        if n == 0 || n > MAX_BASE || n % 512 != 0 {
            return Err("Select a sector-aligned .img file up to 8 GiB".into());
        }
        guards.push(file);
        cmd.arg(linux(path)?);
    }
    if console {
        if !cmd.status().map_err(err)?.success() {
            return Err("The Cloud Hypervisor guest failed to start or complete.".into());
        }
        return Ok(
            json!({"engine":"Cloud Hypervisor v55.0","state":"powered-off","guestChanges":"discarded","network":"offline"}),
        );
    }
    let run = cmd.creation_flags(0x08000000).output().map_err(err)?;
    if !run.status.success() {
        return Err(format!(
            "Cloud Hypervisor repair failed: {}",
            String::from_utf8_lossy(&run.stderr)
                .lines()
                .rev()
                .take(3)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
                .join(" ")
        ));
    }
    let proof: Value = String::from_utf8_lossy(&run.stdout)
        .lines()
        .rev()
        .find_map(|line| serde_json::from_str(line).ok())
        .ok_or("Missing guest evidence")?;
    if proof["passed"] != true
        || proof["rescueISO"] != image_hash
        || proof["recoveredFileSHA256"] != RECOVERED
        || proof["networkDevices"] != 0
        || !proof["sourceSHA256BeforeAndAfter"]
            .as_str()
            .is_some_and(|s| s.len() == 64)
    {
        return Err("Guest evidence did not match the verified recovery profile.".into());
    }
    Ok(
        json!({"schemaVersion":1,"engine":"Cloud Hypervisor v55.0","backend":"Linux KVM in WSL2","guest":"INTA Rescue amd64","network":"offline","passed":true,"sourcePreserved":true,"recoveredSha256":RECOVERED,"isoSha256":image_hash,"elapsedMs":started.elapsed().as_millis(),"evidence":proof}),
    )
}
