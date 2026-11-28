//! Reversible current-user Run registration management using a protected write-ahead record.
use anyhow::{Context, Result, bail, ensure};
use rusqlite::Connection;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::io;
use winreg::{HKCU, RegKey, RegValue, enums::*};

mod protection;
#[cfg(test)]
mod tests;

const RUN: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
const UNDO: &str = r"Software\INTA\StartupQuarantine\v1";
const MAX_BYTES: usize = 65_536;
type Raw = RegValue<'static>;
#[derive(Debug, Clone, Copy, PartialEq)]
enum Phase {
    Prepared = 1,
    Held = 2,
    Restoring = 3,
}

struct Locations {
    run: String,
    undo: String,
}

impl Locations {
    fn current() -> Result<Self> {
        if let Some(fixture) = std::env::var_os("INTA_STARTUP_TEST_ID") {
            ensure!(
                cfg!(debug_assertions),
                "Fixture selection requires a debug build"
            );
            let id = uuid::Uuid::parse_str(fixture.to_str().context("Invalid startup fixture")?)?;
            return Ok(Self::fixture(id));
        }
        Ok(Self {
            run: RUN.into(),
            undo: UNDO.into(),
        })
    }
    fn fixture(id: uuid::Uuid) -> Self {
        let base = format!(r"Software\INTA-Lab\StartupTests\{id}");
        Self {
            run: format!(r"{base}\Run"),
            undo: format!(r"{base}\Undo"),
        }
    }
}

fn optional<T>(result: io::Result<T>) -> Result<Option<T>> {
    match result {
        Ok(value) => Ok(Some(value)),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e.into()),
    }
}
fn valid_id(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
}
fn identifier(loc: &Locations, name: &str) -> String {
    let mut hash = Sha256::new();
    hash.update(loc.run.as_bytes());
    hash.update([0]);
    hash.update(name.as_bytes());
    format!("{:x}", hash.finalize())
}
fn revision(id: &str, value: &Raw, state: &str, stamps: [u64; 2]) -> String {
    let mut hash = Sha256::new();
    hash.update(id);
    hash.update(state);
    for stamp in stamps {
        hash.update(stamp.to_le_bytes());
    }
    hash.update((value.vtype.clone() as u32).to_le_bytes());
    hash.update(value.bytes.as_ref());
    format!("{:x}", hash.finalize())
}
fn display_name(name: &str, id: &str) -> String {
    if name.len() <= 96
        && !name
            .split_whitespace()
            .skip(1)
            .any(|word| word.starts_with('-'))
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || " _.()-".contains(c))
    {
        name.to_owned()
    } else {
        format!("Startup entry {}", &id[..12])
    }
}
fn supported(raw: &Raw) -> bool {
    matches!(raw.vtype, REG_SZ | REG_EXPAND_SZ)
        && !raw.bytes.is_empty()
        && raw.bytes.len() <= MAX_BYTES
        && raw.bytes.len().is_multiple_of(2)
}
fn values(key: Option<&RegKey>) -> Result<Vec<(String, Raw)>> {
    let Some(key) = key else {
        return Ok(Vec::new());
    };
    let info = key.query_info()?;
    ensure!(
        info.values <= 256 && info.max_value_name_len <= 2048 && info.max_value_len <= 131_072,
        "Startup registry review exceeds its per-key limits"
    );
    let mut entries = Vec::new();
    let mut total = 0;
    for pair in key.enum_values().take(257) {
        let (name, raw) = pair?;
        total += raw.bytes.len();
        ensure!(
            total <= 4 * 1024 * 1024 && entries.len() < 256,
            "Startup registry review exceeds its bounds"
        );
        if name.is_empty() {
            continue;
        }
        ensure!(
            raw.vtype != REG_LINK,
            "A registry link requires separate review"
        );
        entries.push((name, raw));
    }
    Ok(entries)
}

struct Entry {
    name: String,
    live: Option<Raw>,
    saved: Option<Raw>,
    recovery_invalid: bool,
    phase: Option<Phase>,
}
impl Entry {
    fn state(&self) -> &'static str {
        if self.recovery_invalid {
            return "recovery-unavailable";
        }
        match (&self.live, &self.saved) {
            (Some(live), Some(saved)) if live == saved && self.phase == Some(Phase::Prepared) => {
                "disable-interrupted"
            }
            (Some(live), Some(saved)) if live == saved && self.phase == Some(Phase::Restoring) => {
                "restore-interrupted"
            }
            (Some(_), Some(_)) => "conflict",
            (None, Some(_)) => "quarantined",
            (Some(v), None) if supported(v) => "registered",
            _ => "unsupported",
        }
    }
    fn view(&self, id: &str, loc: &Locations, stamps: [u64; 2]) -> Value {
        let raw = self
            .saved
            .as_ref()
            .or(self.live.as_ref())
            .expect("Entry always carries a value");
        let state = self.state();
        let (action, effect) = match state {
            "disable-interrupted" => (
                Some("disable"),
                "Resume the interrupted removal of the verified registration. Its original protected recovery copy is present.",
            ),
            "restore-interrupted" => (
                Some("restore"),
                "Verify the previously restored registration and close its recovery record.",
            ),
            "recovery-unavailable" => (
                None,
                "Windows could not authenticate this entry's saved recovery copy. The current state is preserved.",
            ),
            "registered" => (
                Some("disable"),
                "Move this current-user sign-in registration to encrypted INTA recovery. Any existing process continues running.",
            ),
            "quarantined" => (
                Some("restore"),
                "Restore the saved registry bytes and value type. Windows may start this program at a future sign-in.",
            ),
            "conflict" => (
                None,
                "A registration exists alongside the INTA recovery copy. Both are preserved for review.",
            ),
            _ => (
                None,
                "Only named string or expandable-string registrations are eligible.",
            ),
        };
        json!({"id":id,"name":display_name(&self.name,id),"registryPath":format!(r"HKCU\{}",loc.run),
            "type":format!("{:?}",raw.vtype),"bytes":raw.bytes.len(),"state":state,"action":action,
            "revision":revision(id,raw,state,stamps),"effect":effect})
    }
}
struct Keys {
    run: Option<RegKey>,
    undo: Option<RegKey>,
}
fn stamp(key: Option<&RegKey>) -> Result<u64> {
    let Some(key) = key else { return Ok(0) };
    let time = key.query_info()?.last_write_time;
    Ok((u64::from(time.dwHighDateTime) << 32) | u64::from(time.dwLowDateTime))
}
impl Keys {
    fn stamps(&self) -> Result<[u64; 2]> {
        Ok([stamp(self.run.as_ref())?, stamp(self.undo.as_ref())?])
    }
    fn open(loc: &Locations, change: bool) -> Result<Self> {
        let flags = KEY_QUERY_VALUE | if change { KEY_SET_VALUE } else { 0 };
        let open =
            |s: &str| optional(HKCU.open_subkey_with_options_flags(s, REG_OPTION_OPEN_LINK, flags));
        Ok(Self {
            run: open(&loc.run)?,
            undo: open(&loc.undo)?,
        })
    }
    fn entries(&self, loc: &Locations) -> Result<BTreeMap<String, Entry>> {
        let mut entries = BTreeMap::new();
        for (name, live) in values(self.run.as_ref())? {
            if let Some(key) = &self.undo
                && optional(key.get_raw_value(&name))?.is_some()
            {
                continue;
            }
            entries.insert(
                identifier(loc, &name),
                Entry {
                    name,
                    live: Some(live),
                    saved: None,
                    recovery_invalid: false,
                    phase: None,
                },
            );
        }
        for (name, protected) in values(self.undo.as_ref())? {
            let id = identifier(loc, &name);
            let (saved, phase) = match protection::open(&id, &protected) {
                Ok((phase, original)) => (original, Some(phase)),
                Err(_) => (protected, None),
            };
            let live = match &self.run {
                Some(key) => optional(key.get_raw_value(&name))?,
                None => None,
            };
            entries.insert(
                id,
                Entry {
                    name,
                    live,
                    saved: Some(saved),
                    recovery_invalid: phase.is_none(),
                    phase,
                },
            );
        }
        Ok(entries)
    }
}
fn lock(loc: &Locations) -> Result<std::fs::File> {
    let filename = format!("startup-{}.lock", identifier(loc, "operation"));
    if loc.run != RUN {
        use std::os::windows::fs::OpenOptionsExt;
        use windows_sys::Win32::Storage::FileSystem::FILE_FLAG_DELETE_ON_CLOSE;
        return std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .custom_flags(FILE_FLAG_DELETE_ON_CLOSE)
            .open(std::env::temp_dir().join(filename))
            .context("Another fixture change is in progress");
    }
    let base = std::path::PathBuf::from(
        std::env::var_os("LOCALAPPDATA").context("Current user profile unavailable")?,
    )
    .join("INTA/transactions");
    std::fs::create_dir_all(&base)?;
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(base.join(filename))?;
    file.try_lock()
        .context("Another startup change is in progress")?;
    Ok(file)
}
fn flush(key: &RegKey) -> Result<()> {
    let status = unsafe { windows_sys::Win32::System::Registry::RegFlushKey(key.raw_handle()) };
    ensure!(
        status == 0,
        "Windows could not persist the startup recovery state (error {status})"
    );
    Ok(())
}
fn save(key: &RegKey, id: &str, name: &str, phase: Phase, raw: &Raw) -> Result<()> {
    key.set_raw_value(name, &protection::seal(id, phase, raw)?)?;
    flush(key)?;
    let (observed, value) = protection::open(id, &key.get_raw_value(name)?)?;
    ensure!(
        phase == observed && value == *raw,
        "Recovery verification failed"
    );
    Ok(())
}
fn execute(loc: &Locations, args: &[&str]) -> Result<Value> {
    let change =
        matches!(args,["disable"|"restore",id,rev,"--approve"] if valid_id(id) && valid_id(rev));
    ensure!(
        change || matches!(args, ["list"] | ["preview", _]),
        "Use a current preview ID and revision and --approve for one entry"
    );
    let _lock = if change { Some(lock(loc)?) } else { None };
    let mut keys = Keys::open(loc, change)?;
    let stamps = keys.stamps()?;
    let entries = keys.entries(loc)?;
    ensure!(
        keys.stamps()? == stamps,
        "Startup registration changed while reading. Refresh its preview."
    );
    if args == ["list"] {
        return Ok(
            json!({"scope":"Current user Run registrations","entries":entries.iter().map(|(id,e)|e.view(id,loc,stamps)).collect::<Vec<_>>()}),
        );
    }
    let id = args[1];
    let entry = entries
        .get(id)
        .context("Startup entry unavailable. Refresh the list.")?;
    let view = entry.view(id, loc, stamps);
    if !change {
        return Ok(view);
    }
    ensure!(
        view["action"] == args[0] && view["revision"] == args[2],
        "The entry changed or has a conflict. Refresh its preview."
    );
    let run = keys
        .run
        .as_ref()
        .context("The original Run key is missing. Restore requires the original location.")?;
    match args[0] {
        "disable" => {
            if keys.undo.is_none() {
                keys.undo = Some(
                    HKCU.create_subkey_with_flags(&loc.undo, KEY_QUERY_VALUE | KEY_SET_VALUE)?
                        .0,
                );
            }
            let undo = keys.undo.as_ref().context("Recovery key unavailable")?;
            let raw = entry.live.as_ref().context("Registration unavailable")?;
            if entry.phase.is_none() {
                ensure!(
                    optional(undo.get_raw_value(&entry.name))?.is_none(),
                    "A recovery record already exists"
                );
                save(undo, id, &entry.name, Phase::Prepared, raw)?;
            }
            // Keep the last identity check beside RegDeleteValue. Independent vendors do not share our lock.
            ensure!(
                stamp(Some(run))? == stamps[0] && run.get_raw_value(&entry.name)? == *raw,
                "The live registration changed. Its recovery record was retained."
            );
            run.delete_value(&entry.name)?;
            flush(run)?;
            ensure!(
                optional(run.get_raw_value(&entry.name))?.is_none(),
                "The registration was recreated. Both copies were retained."
            );
            save(undo, id, &entry.name, Phase::Held, raw)?;
        }
        "restore" => {
            let raw = entry.saved.as_ref().context("Recovery copy unavailable")?;
            let undo = keys.undo.as_ref().context("Recovery key unavailable")?;
            if entry.state() != "restore-interrupted" {
                save(undo, id, &entry.name, Phase::Restoring, raw)?;
                ensure!(
                    stamp(Some(run))? == stamps[0]
                        && optional(run.get_raw_value(&entry.name))?.is_none(),
                    "An external registration occupies the original name"
                );
                run.set_raw_value(&entry.name, raw)?;
                flush(run)?;
            }
            ensure!(
                run.get_raw_value(&entry.name)? == *raw,
                "The restored entry changed. The recovery copy was retained."
            );
            undo.delete_value(&entry.name)?;
            flush(undo)?;
        }
        _ => bail!("Unsupported startup action"),
    }
    Ok(
        json!({"id":id,"state":if args[0]=="disable" {"quarantined"} else {"restored"},"verified":true}),
    )
}
pub fn command(c: &Connection, args: &[&str]) -> Result<Value> {
    let loc = Locations::current()?;
    if let [op @ ("disable" | "restore"), id, rev, "--approve"] = args
        && valid_id(id)
        && valid_id(rev)
    {
        return crate::audited(c, &format!("startup/{op}/{id}"), || execute(&loc, args));
    }
    execute(&loc, args)
}
