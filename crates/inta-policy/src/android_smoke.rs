//! One-shot, real Android emulator smoke. All authored test orchestration is Rust.
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
    thread,
    time::Duration,
};
const PACKAGE: &str = "org.inta.toolbox";
const ACTIVITY: &str = "org.inta.toolbox/android.app.NativeActivity";
type Result<T> = std::result::Result<T, String>;
fn adb(args: &[&str]) -> Result<Vec<u8>> {
    let result = Command::new("adb")
        .args(args)
        .output()
        .map_err(|e| format!("adb {args:?}: {e}"))?;
    if !result.status.success() {
        return Err(format!(
            "adb {args:?}: {} {}",
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr)
        ));
    }
    Ok(result.stdout)
}
fn text(args: &[&str]) -> Result<String> {
    adb(args).map(|s| String::from_utf8_lossy(&s).into_owned())
}
fn write(path: &Path, bytes: impl AsRef<[u8]>) -> Result<()> {
    fs::write(path, bytes).map_err(|e| format!("{}: {e}", path.display()))
}
fn start(report: &Path, label: &str) -> Result<String> {
    let launch = text(&["shell", "am", "start", "-W", "-n", ACTIVITY])?;
    write(&report.join(format!("{label}-launch.txt")), &launch)?;
    if !launch.contains("Status: ok") || launch.contains("Error:") {
        return Err(format!(
            "Activity did not report a successful launch: {launch}"
        ));
    }
    thread::sleep(Duration::from_secs(5));
    let pid = text(&["shell", "pidof", PACKAGE])?.trim().to_string();
    if pid.is_empty() {
        return Err("Native process is absent after launch".into());
    }
    let activity = text(&["shell", "dumpsys", "activity", "activities"])?;
    write(&report.join(format!("{label}-activities.txt")), &activity)?;
    if !activity.lines().any(|line| {
        line.contains(PACKAGE)
            && (line.contains("mResumedActivity") || line.contains("topResumedActivity"))
    }) {
        return Err("INTA is not the resumed foreground Activity".into());
    }
    Ok(pid)
}
fn screenshot(report: &Path, label: &str) -> Result<serde_json::Value> {
    let bytes = adb(&["exec-out", "screencap", "-p"])?;
    write(&report.join(format!("{label}.png")), &bytes)?;
    if bytes.len() < 10_000
        || !bytes.starts_with(b"\x89PNG\r\n\x1a\n")
        || bytes.get(12..16) != Some(b"IHDR")
    {
        return Err("Android screen capture did not produce a substantive PNG".into());
    }
    let width = u32::from_be_bytes(bytes[16..20].try_into().unwrap());
    let height = u32::from_be_bytes(bytes[20..24].try_into().unwrap());
    if width < 300 || height < 300 {
        return Err("Android screenshot dimensions are unexpectedly small".into());
    }
    Ok(
        json!({"path":format!("{label}.png"),"width":width,"height":height,"sha256":format!("{:x}",Sha256::digest(&bytes))}),
    )
}
fn fatal(log: &str) -> bool {
    [
        "FATAL EXCEPTION",
        "Fatal signal",
        "ANR in org.inta.toolbox",
        "Native UI failed:",
        "panicked at",
    ]
    .iter()
    .any(|needle| log.contains(needle))
}
fn logs(report: &Path, pid: &str, label: &str) -> Result<()> {
    let process = text(&["logcat", "-d", "--pid", pid])?;
    write(&report.join(format!("{label}-process.log")), &process)?;
    let crash = text(&["logcat", "-b", "crash", "-d"])?;
    write(&report.join(format!("{label}-crash.log")), &crash)?;
    if fatal(&process) || fatal(&crash) {
        return Err(format!(
            "Fatal, panic or ANR evidence found in {label} logs"
        ));
    }
    Ok(())
}
fn run(apk_dir: &Path, report: &Path) -> Result<()> {
    fs::create_dir_all(report).map_err(|e| e.to_string())?;
    let mut apks: Vec<_> = fs::read_dir(apk_dir)
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|e| e == "apk"))
        .collect();
    apks.sort();
    if apks.len() != 1 {
        return Err(format!(
            "Expected one universal APK in {}, found {}",
            apk_dir.display(),
            apks.len()
        ));
    }
    let apk = &apks[0];
    let apk_path = apk.to_str().ok_or("Non-UTF8 APK path")?;
    write(&report.join("adb-version.txt"), text(&["version"])?)?;
    write(&report.join("device.txt"), text(&["shell", "getprop"])?)?;
    write(
        &report.join("install.txt"),
        text(&["install", "-r", "-t", apk_path])?,
    )?;
    adb(&["logcat", "-c"])?;
    adb(&["logcat", "-b", "crash", "-c"])?;
    let pid = start(report, "cold-start")?;
    let cold = screenshot(report, "cold-start")?;
    logs(report, &pid, "cold-start")?;
    adb(&["shell", "input", "keyevent", "KEYCODE_HOME"])?;
    thread::sleep(Duration::from_secs(1));
    let resumed_pid = start(report, "resume")?;
    let resumed = screenshot(report, "resume")?;
    logs(report, &resumed_pid, "resume")?;
    adb(&["shell", "am", "force-stop", PACKAGE])?;
    let restarted_pid = start(report, "restart")?;
    let restarted = screenshot(report, "restart")?;
    logs(report, &restarted_pid, "restart")?;
    let bytes = fs::read(apk).map_err(|e| e.to_string())?;
    let evidence = json!({"status":"passed","scope":"Android API 35 x86_64 emulator: install, native foreground launch, background/resume and cold restart; no physical-device or account-sync claim","package":PACKAGE,"activity":ACTIVITY,"apk":apk_path,"apkSha256":format!("{:x}",Sha256::digest(&bytes)),"screenshots":[cold,resumed,restarted],"processIds":[pid,resumed_pid,restarted_pid],"fatalPanicOrAnrDetected":false});
    write(
        &report.join("result.json"),
        serde_json::to_vec_pretty(&evidence).unwrap(),
    )?;
    println!(
        "Android native emulator smoke passed; screenshots and logs in {}",
        report.display()
    );
    Ok(())
}
fn main() -> ExitCode {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() != 2 {
        eprintln!("Usage: inta-android-smoke APK_DIRECTORY REPORT_DIRECTORY");
        return ExitCode::from(2);
    }
    let report = PathBuf::from(&args[1]);
    match run(Path::new(&args[0]), &report) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let _ = fs::create_dir_all(&report);
            let _ = write(
                &report.join("result.json"),
                serde_json::to_vec_pretty(&json!({"status":"failed","error":error})).unwrap(),
            );
            if let Ok(log) = text(&["logcat", "-d"]) {
                let _ = write(&report.join("failure.log"), log);
            }
            eprintln!("Android native smoke failed: {error}");
            ExitCode::FAILURE
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fatal_and_anr_are_failures() {
        for log in [
            "E AndroidRuntime: FATAL EXCEPTION: main",
            "Fatal signal 11",
            "ANR in org.inta.toolbox",
            "Native UI failed: no renderer",
            "thread panicked at memory",
        ] {
            assert!(fatal(log));
        }
        assert!(!fatal("W EGL: optional extension unavailable"));
    }
}
