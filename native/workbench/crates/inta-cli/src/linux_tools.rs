use anyhow::{Context, Result, ensure};
use serde_json::{Value, json};
use std::{env, fs, path::PathBuf};

const DISTRO: &str = "INTA-Rescue-Builder";

fn cache() -> Result<PathBuf> {
    Ok(
        PathBuf::from(env::var_os("LOCALAPPDATA").context("Windows user profile unavailable")?)
            .join("INTA/lab/linux-tool-inventory.json"),
    )
}
fn supports(id: &str, field: &str) -> bool {
    if !id
        .bytes()
        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
    {
        return false;
    }
    let known: Vec<Value> =
        serde_json::from_str(include_str!("../../../data/tool-actions.json")).unwrap_or_default();
    known
        .iter()
        .any(|r| r["id"] == id && r["linux"].is_object())
        && cache()
            .ok()
            .and_then(|p| fs::read(p).ok())
            .and_then(|b| serde_json::from_slice::<Value>(&b).ok())
            .is_some_and(|v| {
                v[field]
                    .as_array()
                    .is_some_and(|a| a.iter().any(|v| v == id))
            })
}
pub fn available(id: &str) -> bool {
    supports(id, "installed")
}
pub fn can_install(id: &str) -> bool {
    supports(id, "packages")
}

#[cfg(windows)]
fn open(id: &str, verb: &str) -> Result<Value> {
    ensure!(
        if verb == "install" {
            can_install(id)
        } else {
            available(id)
        },
        "Linux tool unavailable in the prepared inventory"
    );
    crate::tool_actions::open_console(
        &env::current_exe()?,
        &format!("linux-session {verb} {id} --approve"),
    )
}
#[cfg(not(windows))]
fn open(_: &str, _: &str) -> Result<Value> {
    anyhow::bail!("The WSL bridge requires Windows")
}
pub fn launch(id: &str) -> Result<Value> {
    open(id, "launch")
}
pub fn install(id: &str) -> Result<Value> {
    open(id, "install")
}

pub fn session(verb: &str, id: &str) -> Result<Value> {
    ensure!(matches!(verb, "launch" | "install"), "Invalid Linux action");
    let installing = verb == "install";
    ensure!(
        if installing {
            can_install(id)
        } else {
            available(id)
        },
        "Unrecognized Linux tool or environment"
    );
    #[cfg(windows)]
    let wsl = crate::tool_actions::system_dir()?.join("System32/wsl.exe");
    #[cfg(not(windows))]
    let wsl = PathBuf::from("/nonexistent/wsl.exe");
    let mut run = std::process::Command::new(&wsl);
    let flag = if installing { "--install" } else { "--open" };
    let user = if installing { "root" } else { "inta-lab" };
    let status = run
        .args([
            "-d",
            DISTRO,
            "-u",
            user,
            "--cd",
            "~",
            "--exec",
            "/usr/local/bin/inta-tool",
            flag,
            id,
        ])
        .status()?;
    if installing {
        let report = std::process::Command::new(wsl)
            .args([
                "-d",
                DISTRO,
                "-u",
                "inta-lab",
                "--exec",
                "/usr/local/bin/inta-tool",
                "--inventory",
            ])
            .output()?;
        ensure!(
            report.status.success(),
            "The Linux inventory refresh failed"
        );
        let inventory: Value = serde_json::from_slice(&report.stdout)?;
        ensure!(
            inventory["installed"].is_array() && inventory["packages"].is_array(),
            "Unexpected Linux inventory"
        );
        let target = cache()?;
        let mut tmp =
            tempfile::NamedTempFile::new_in(target.parent().context("Invalid lab folder")?)?;
        serde_json::to_writer(&mut tmp, &inventory)?;
        tmp.as_file().sync_all()?;
        tmp.persist(target)?;
    }
    ensure!(status.success(), "The Linux tool returned {status}");
    Ok(json!({"state":"complete","tool":id,"environment":DISTRO}))
}
