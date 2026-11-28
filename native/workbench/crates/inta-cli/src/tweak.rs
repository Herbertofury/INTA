use crate::db;
#[path = "tweak_profiles.rs"]
pub mod profiles;
use anyhow::{Context, Result, bail, ensure};
use rusqlite::{Connection, OptionalExtension};
use serde_json::{Value, json};

#[cfg(test)]
#[path = "tweak_tests.rs"]
mod tests;

pub trait Setting {
    fn spec(&self) -> &Spec;
    fn read(&self) -> Result<Option<u32>>;
    fn write(&self, value: Option<u32>) -> Result<()>;
}
pub struct Spec {
    pub id: &'static str,
    name: &'static str,
    value: &'static str,
    target: u32,
    valid: &'static [u32],
    effect: &'static str,
}
const ADVANCED: &str = r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced";
const FLAGS: &[u32] = &[0, 1];
pub static SPECS: &[Spec] = &[
    Spec {
        id: "explorer-file-extensions",
        name: "Show known file extensions",
        value: "HideFileExt",
        target: 0,
        valid: FLAGS,
        effect: "Show file-name extensions. Reopen Explorer windows to see the change.",
    },
    Spec {
        id: "explorer-hidden-files",
        name: "Show hidden files",
        value: "Hidden",
        target: 1,
        valid: &[1, 2],
        effect: "Display hidden files and folders while preserving the separate protected operating system files preference.",
    },
    Spec {
        id: "explorer-status-bar",
        name: "Show the Explorer status bar",
        value: "ShowStatusBar",
        target: 1,
        valid: FLAGS,
        effect: "Display selection and folder information in Explorer versions that support the status bar.",
    },
    Spec {
        id: "explorer-info-tips",
        name: "Show file information tips",
        value: "ShowInfoTip",
        target: 1,
        valid: FLAGS,
        effect: "Display file information when the pointer rests on a supported item.",
    },
    Spec {
        id: "explorer-type-overlay",
        name: "Show file icons on thumbnails",
        value: "ShowTypeOverlay",
        target: 1,
        valid: FLAGS,
        effect: "Display file-type icons on supported thumbnail previews.",
    },
    Spec {
        id: "explorer-separate-process",
        name: "Open folder windows in a separate process",
        value: "SeparateProcess",
        target: 1,
        valid: FLAGS,
        effect: "Request a separate Explorer process for folder windows. This can use additional memory.",
    },
];
fn metadata(s: &Spec) -> Value {
    json!({"id":s.id,"scope":"Current user / File Explorer","setting":s.name,"effect":s.effect,
    "registryPath":format!(r"HKCU\{ADVANCED}"),"valueName":s.value,"targetRegistryValue":s.target,
    "type":"REG_DWORD","risk":"Low","requiresElevation":false,"requiresRestart":false,
    "activation":"Reopen Explorer windows or sign in again for the preference to take effect.",
    "compatibility":"Windows Explorer on Windows 10 and 11. Shell builds may render options differently.",
    "policy":"Requires access to Explorer folder options.","performanceEvidence":"No measured speed improvement claimed."})
}
pub fn list() -> Value {
    Value::Array(SPECS.iter().map(metadata).collect())
}
pub fn operation(
    c: &Connection,
    key: &impl Setting,
    action: &str,
    approved: bool,
) -> Result<Value> {
    let locks = lock_dir()?;
    transact(c, key, action, approved, &locks)
}
fn lock_dir() -> Result<std::path::PathBuf> {
    Ok(std::path::PathBuf::from(
        std::env::var_os("LOCALAPPDATA").context("Current-user application data unavailable")?,
    )
    .join("INTA/transactions"))
}
fn lock(state: &std::path::Path) -> Result<std::fs::File> {
    std::fs::create_dir_all(state)?;
    let f = std::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(state.join("tweak.lock"))?;
    f.try_lock()
        .context("Another setting transaction is running")?;
    Ok(f)
}
fn transact(
    c: &Connection,
    key: &impl Setting,
    action: &str,
    approved: bool,
    state: &std::path::Path,
) -> Result<Value> {
    ensure!(
        matches!(action, "status" | "preview" | "apply" | "undo"),
        "Unknown tweak operation"
    );
    let _lock = if matches!(action, "apply" | "undo") {
        ensure!(approved, "Explicit --approve is required");
        Some(lock(state)?)
    } else {
        None
    };
    let owner = profiles::owner(c, key.spec().id)?;
    if matches!(action, "apply" | "undo") {
        ensure!(
            owner.is_none(),
            "This setting belongs to the {} profile. Use its undo control.",
            owner.as_deref().unwrap_or("")
        );
    }
    let mut value = transact_locked(c, key, action)?;
    if owner.is_some() {
        value["profileOwner"] = json!(owner);
        value["undoAvailable"] = json!(false);
    }
    Ok(value)
}
fn transact_locked(c: &Connection, key: &dyn Setting, action: &str) -> Result<Value> {
    let spec = key.spec();
    let id = spec.id;
    let target = Some(spec.target);
    let current = key.read()?;
    ensure!(
        current.is_none_or(|v| spec.valid.contains(&v)),
        "Unexpected registry value. Review the setting manually."
    );
    let journal: Option<(String, String)> = c
        .query_row(
            "SELECT before_value,phase FROM tweaks WHERE id=?",
            [id],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .optional()?;
    if action == "preview" || action == "status" {
        let mut record = metadata(spec);
        record["registryValue"] = json!(current);
        record["phase"] = json!(journal.as_ref().map(|r| &r.1));
        record["undoAvailable"] = json!(journal.is_some());
        return Ok(record);
    }
    if action == "apply" {
        if let Some((original, phase)) = &journal {
            let before: Option<u32> = serde_json::from_str(original)?;
            ensure!(
                phase != "restoring",
                "Interrupted undo. Complete undo before applying."
            );
            if phase == "applied" {
                ensure!(
                    current == target,
                    "Setting changed externally. Resolve the conflict before applying."
                );
                return Ok(json!({"id":id,"state":"already-applied"}));
            }
            ensure!(
                current == before || current == target,
                "Pending change conflicts with the current setting"
            );
        } else {
            if current == target {
                return Ok(json!({"id":id,"state":"already-configured","undoAvailable":false}));
            }
            c.execute(
                "INSERT INTO tweaks VALUES(?,?,'prepared')",
                rusqlite::params![id, serde_json::to_string(&current)?],
            )?;
        }
        if current != target {
            key.write(target)?;
        }
        ensure!(
            key.read()? == target,
            "Setting verification failed. The recovery snapshot is retained."
        );
        let tx = c.unchecked_transaction()?;
        tx.execute("UPDATE tweaks SET phase='applied' WHERE id=?", [id])?;
        db::event(&tx, id, "applied")?;
        tx.commit()?;
        return Ok(json!({"id":id,"state":"applied","verified":true,"undoAvailable":true}));
    }
    let (original, phase) = journal.context("No change owned by INTA to undo")?;
    let original: Option<u32> = serde_json::from_str(&original)?;
    if current != target && !(current == original && (phase == "prepared" || phase == "restoring"))
    {
        bail!("Setting changed outside INTA. Undo stopped. Snapshot retained.")
    }
    c.execute("UPDATE tweaks SET phase='restoring' WHERE id=?", [id])?;
    if current != original {
        key.write(original)?;
    }
    ensure!(
        key.read()? == original,
        "Undo verification failed. The recovery snapshot is retained."
    );
    let tx = c.unchecked_transaction()?;
    tx.execute("DELETE FROM tweaks WHERE id=?", [id])?;
    db::event(&tx, id, "restored")?;
    tx.commit()?;
    Ok(json!({"id":id,"state":"restored","verified":true}))
}

#[cfg(windows)]
pub struct ExplorerSetting(pub &'static Spec);
#[cfg(windows)]
impl ExplorerSetting {
    fn key(&self, write: bool) -> Result<winreg::RegKey> {
        use winreg::{RegKey, enums::*};
        let policy = RegKey::predef(HKEY_CURRENT_USER);
        for hive in [&policy, &RegKey::predef(HKEY_LOCAL_MACHINE)] {
            match hive.open_subkey(r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer") {
                Ok(k) => match k.get_value::<u32, _>("NoFolderOptions") {
                    Ok(0) => (),
                    Ok(_) => bail!("Explorer options are managed"),
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
                    Err(e) => return Err(e.into()),
                },
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
                Err(e) => return Err(e.into()),
            }
        }
        Ok(policy.open_subkey_with_flags(
            ADVANCED,
            if write {
                KEY_READ | KEY_SET_VALUE
            } else {
                KEY_READ
            },
        )?)
    }
}
#[cfg(windows)]
impl Setting for ExplorerSetting {
    fn spec(&self) -> &Spec {
        self.0
    }
    fn read(&self) -> Result<Option<u32>> {
        read_setting(&self.key(false)?, self.0.value)
    }
    fn write(&self, value: Option<u32>) -> Result<()> {
        write_setting(&self.key(true)?, self.0.value, value)
    }
}
#[cfg(windows)]
fn read_setting(key: &winreg::RegKey, name: &str) -> Result<Option<u32>> {
    match key.get_value(name) {
        Ok(x) => Ok(Some(x)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e.into()),
    }
}
#[cfg(windows)]
fn write_setting(key: &winreg::RegKey, name: &str, value: Option<u32>) -> Result<()> {
    match value {
        Some(v) => key.set_value(name, &v)?,
        None => key.delete_value(name)?,
    };
    Ok(())
}
