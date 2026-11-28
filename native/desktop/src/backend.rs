use std::{
    ffi::OsString,
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

use serde_json::Value;

pub const CORE: &str = "inta.exe";
pub const CAPABILITIES: &str = "inta-capabilities.exe";

#[derive(Clone)]
pub struct Bridge {
    pub directory: PathBuf,
}

pub fn field(data: &Value, key: &str) -> String {
    data.get(key)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned()
}

fn bounded(mut stream: impl Read, cap: usize) -> io::Result<(Vec<u8>, bool)> {
    let mut kept = Vec::new();
    let mut buf = [0u8; 16384];
    let mut truncated = false;
    loop {
        let n = stream.read(&mut buf)?;
        if n == 0 {
            return Ok((kept, truncated));
        }
        let count = n.min(cap - kept.len());
        kept.extend_from_slice(&buf[..count]);
        truncated |= count < n;
    }
}

impl Bridge {
    fn path(&self, name: &str) -> Result<PathBuf, String> {
        let p = self.directory.join(name);
        let m = fs::symlink_metadata(&p)
            .map_err(|_| format!("{name} is missing from this installation."))?;
        #[cfg(windows)]
        {
            use std::os::windows::fs::MetadataExt;
            if m.file_attributes() & 0x400 != 0 {
                return Err(format!("{name} is a redirected file."));
            }
        }
        if !m.is_file() {
            return Err(format!("{name} is not a regular application file."));
        }
        Ok(p)
    }
    pub fn json(&self, name: &str, args: &[&str], action: bool) -> Result<Value, String> {
        if !matches!(name, CORE | CAPABILITIES) {
            return Err("Unknown local backend.".into());
        }
        let mut cmd = Command::new(self.path(name)?);
        cmd.args(args)
            .current_dir(&self.directory)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(0x08000000);
        }
        let mut child = cmd.spawn().map_err(|e| e.to_string())?;
        let out = child.stdout.take().expect("configured stdout");
        let err = child.stderr.take().expect("configured stderr");
        let stdout = thread::spawn(move || bounded(out, 6 * 1024 * 1024));
        let stderr = thread::spawn(move || bounded(err, 16384));
        let started = Instant::now();
        let status = loop {
            if let Some(s) = child.try_wait().map_err(|e| e.to_string())? {
                break s;
            }
            if !action && started.elapsed() > Duration::from_secs(40) {
                let _ = child.kill();
                let _ = child.wait();
                return Err("The local query exceeded 40 seconds.".into());
            }
            thread::sleep(Duration::from_millis(35));
        };
        let (body, overflow) = stdout
            .join()
            .map_err(|_| "Output reader stopped")?
            .map_err(|e| e.to_string())?;
        let (errors, _) = stderr
            .join()
            .map_err(|_| "Error reader stopped")?
            .map_err(|e| e.to_string())?;
        if overflow {
            return Err("The local response exceeded the display limit.".into());
        }
        let value = serde_json::from_slice::<Value>(&body);
        if !status.success() {
            let msg = value
                .as_ref()
                .ok()
                .and_then(|v| v.pointer("/error/message"))
                .and_then(Value::as_str)
                .unwrap_or("");
            let e = String::from_utf8_lossy(&errors);
            let message = if msg.is_empty() { e.trim() } else { msg };
            return Err(if message.is_empty() {
                format!("{name} exited with {status}")
            } else {
                message.chars().take(2000).collect()
            });
        }
        let data = value.map_err(|_| format!("{name} returned an unreadable response."))?;
        if name == CAPABILITIES {
            if data.get("ok") != Some(&Value::Bool(true)) {
                return Err(field(&data["error"], "message"));
            }
            return data
                .get("result")
                .cloned()
                .ok_or_else(|| "The local capability response had no result.".into());
        }
        Ok(data)
    }
}

pub fn prefs_path(folder: &Path) -> PathBuf {
    if let Some(root) = std::env::var_os("INTA_DATA_DIR")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
    {
        return root.join("appearance.json");
    }
    if folder.join("inta-portable.marker").is_file() {
        return folder.join("state/appearance.json");
    }
    let base = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|| folder.to_owned());
    base.join("INTA/Desktop/appearance.json")
}

pub fn save(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let directory = path.parent().ok_or(io::ErrorKind::InvalidInput)?;
    fs::create_dir_all(directory)?;
    let mut filename = OsString::from("settings-");
    filename.push(std::process::id().to_string());
    filename.push(".tmp");
    let tmp = directory.join(filename);
    use std::io::Write;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&tmp)?;
    let result = (|| {
        file.write_all(bytes)?;
        file.sync_all()?;
        drop(file);
        fs::rename(&tmp, path)
    })();
    if result.is_err() {
        let _ = fs::remove_file(tmp);
    }
    result
}
