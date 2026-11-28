use super::*;
use rusqlite::params;
use std::path::Path;

struct Profile {
    id: &'static str,
    name: &'static str,
    settings: &'static [&'static str],
}
const PROFILES: &[Profile] = &[
    Profile {
        id: "file-inspection",
        name: "File inspection",
        settings: &[
            "explorer-file-extensions",
            "explorer-hidden-files",
            "explorer-type-overlay",
        ],
    },
    Profile {
        id: "folder-details",
        name: "Folder details",
        settings: &[
            "explorer-status-bar",
            "explorer-info-tips",
            "explorer-type-overlay",
        ],
    },
];
const SCHEMA: &str = "CREATE TABLE IF NOT EXISTS tweak_profiles(id TEXT PRIMARY KEY, phase TEXT NOT NULL);
CREATE TABLE IF NOT EXISTS tweak_profile_items(setting TEXT PRIMARY KEY, profile TEXT NOT NULL, original TEXT NOT NULL);";
pub fn list() -> Value {
    json!(
        PROFILES
            .iter()
            .map(|p| json!({"id":p.id,"name":p.name,"settings":p.settings}))
            .collect::<Vec<_>>()
    )
}
fn initialized(c: &Connection) -> Result<bool> {
    Ok(c.query_row("SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='tweak_profile_items')",[],|r|r.get(0))?)
}
pub(super) fn owner(c: &Connection, setting: &str) -> Result<Option<String>> {
    if !initialized(c)? {
        return Ok(None);
    }
    Ok(c.query_row(
        "SELECT profile FROM tweak_profile_items WHERE setting=?",
        [setting],
        |r| r.get(0),
    )
    .optional()?)
}
fn phase(c: &Connection, id: &str) -> Result<Option<String>> {
    if !initialized(c)? {
        return Ok(None);
    }
    Ok(
        c.query_row("SELECT phase FROM tweak_profiles WHERE id=?", [id], |r| {
            r.get(0)
        })
        .optional()?,
    )
}
#[cfg(windows)]
pub fn run(c: &Connection, verb: &str, id: &str, approved: bool) -> Result<Value> {
    let profile = PROFILES
        .iter()
        .find(|p| p.id == id)
        .context("Unknown Windows setting profile")?;
    let settings = profile
        .settings
        .iter()
        .map(|id| {
            SPECS
                .iter()
                .find(|s| s.id == *id)
                .map(ExplorerSetting)
                .context("Unknown profile setting")
        })
        .collect::<Result<Vec<_>>>()?;
    let refs = settings
        .iter()
        .map(|s| s as &dyn Setting)
        .collect::<Vec<_>>();
    execute(c, profile, &refs, verb, approved, &lock_dir()?)
}
fn execute(
    c: &Connection,
    profile: &Profile,
    keys: &[&dyn Setting],
    verb: &str,
    approved: bool,
    dir: &Path,
) -> Result<Value> {
    ensure!(
        matches!(verb, "preview" | "apply" | "undo"),
        "Unknown profile operation"
    );
    let id = profile.id;
    let _lock = if verb == "preview" {
        None
    } else {
        ensure!(approved, "Explicit --approve is required");
        Some(lock(dir)?)
    };
    let prior = phase(c, id)?;
    let mut changes = Vec::new();
    let mut preview = Vec::new();
    for &key in keys {
        let state = transact_locked(c, key, "preview")?;
        let spec = key.spec();
        let current: Option<u32> = serde_json::from_value(state["registryValue"].clone())?;
        let target = Some(spec.target);
        let owner = owner(c, spec.id)?;
        let disposition = if owner.as_deref() == Some(id) {
            "owned by this profile"
        } else if state["phase"].is_string() || owner.is_some() {
            "managed separately"
        } else if current == target {
            "already configured"
        } else {
            "change"
        };
        if disposition == "change" {
            changes.push((spec.id, current));
        }
        if verb == "apply" && disposition == "managed separately" {
            ensure!(
                state["phase"] == "applied" && current == target,
                "{} has a conflicting or interrupted transaction",
                spec.name
            );
        }
        preview.push(json!({"setting":spec.name,"id":spec.id,"registryValue":current,"targetRegistryValue":spec.target,"disposition":disposition,"effect":spec.effect,"registryPath":state["registryPath"],"valueName":spec.value}));
    }
    if verb == "preview" {
        return Ok(
            json!({"id":id,"name":profile.name,"phase":prior,"undoAvailable":prior.is_some(),"settings":preview}),
        );
    }
    if verb == "undo" {
        ensure!(prior.is_some(), "No changes owned by this profile");
        restore(c, id, keys)?;
        return Ok(json!({"id":id,"state":"restored","verified":true}));
    }
    ensure!(
        prior.as_deref() != Some("restoring"),
        "Finish restoring this profile before applying it again"
    );
    ensure!(
        prior.is_none() || changes.is_empty(),
        "A setting outside this profile's snapshot has changed. Undo this profile and preview a new transaction."
    );
    if prior.is_none() {
        if changes.is_empty() {
            return Ok(json!({"id":id,"state":"already-configured","undoAvailable":false}));
        }
        let tx = c.unchecked_transaction()?;
        tx.execute_batch(SCHEMA)?;
        tx.execute("INSERT INTO tweak_profiles VALUES(?,'prepared')", [id])?;
        for (setting, before) in changes {
            let original = serde_json::to_string(&before)?;
            tx.execute(
                "INSERT INTO tweak_profile_items VALUES(?,?,?)",
                params![setting, id, original],
            )?;
            tx.execute(
                "INSERT INTO tweaks VALUES(?,?,'prepared')",
                params![setting, original],
            )?;
        }
        tx.commit()?;
    }
    // A restarted process may resume the exact snapshot captured by the first attempt.
    for &key in keys {
        if owner(c, key.spec().id)?.as_deref() != Some(id) {
            continue;
        }
        if let Err(error) = transact_locked(c, key, "apply") {
            if prior.as_deref() == Some("applied") {
                return Err(error);
            }
            match restore(c, id, keys) {
                Ok(()) => bail!("Apply failed. The captured values were restored. {error}"),
                Err(recovery) => {
                    bail!("Apply failed. Recovery retained for profile undo. {error}. {recovery}")
                }
            }
        }
    }
    c.execute("UPDATE tweak_profiles SET phase='applied' WHERE id=?", [id])?;
    Ok(json!({"id":id,"state":"applied","verified":true,"undoAvailable":true}))
}
fn restore(c: &Connection, id: &str, keys: &[&dyn Setting]) -> Result<()> {
    c.execute(
        "UPDATE tweak_profiles SET phase='restoring' WHERE id=?",
        [id],
    )?;
    for &key in keys.iter().rev() {
        let setting = key.spec().id;
        if owner(c, setting)?.as_deref() != Some(id) {
            continue;
        }
        let saved: String = c.query_row(
            "SELECT original FROM tweak_profile_items WHERE setting=? AND profile=?",
            params![setting, id],
            |r| r.get(0),
        )?;
        let original: Option<u32> = serde_json::from_str(&saved)?;
        let journal = transact_locked(c, key, "status")?;
        if journal["phase"].is_string() {
            transact_locked(c, key, "undo")?;
        }
        ensure!(
            key.read()? == original,
            "{} changed externally during restore. Snapshot retained.",
            key.spec().name
        );
        c.execute(
            "DELETE FROM tweak_profile_items WHERE setting=? AND profile=?",
            params![setting, id],
        )?;
    }
    c.execute("DELETE FROM tweak_profiles WHERE id=?", [id])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    struct Fake(&'static Spec, Cell<Option<u32>>, Cell<bool>);
    impl Setting for Fake {
        fn spec(&self) -> &Spec {
            self.0
        }
        fn read(&self) -> Result<Option<u32>> {
            Ok(self.1.get())
        }
        fn write(&self, value: Option<u32>) -> Result<()> {
            ensure!(!self.2.replace(false), "blocked fixture write");
            self.1.set(value);
            Ok(())
        }
    }
    #[test]
    fn failed_batch_rolls_back_and_preserves_preexisting_settings() -> Result<()> {
        let c = Connection::open_in_memory()?;
        c.execute_batch("CREATE TABLE tweaks(id TEXT PRIMARY KEY,before_value TEXT,phase TEXT);CREATE TABLE events(at TEXT,operation TEXT,state TEXT);")?;
        let dir = tempfile::tempdir()?;
        let a = Fake(&SPECS[0], Cell::new(None), Cell::new(false));
        let b = Fake(&SPECS[1], Cell::new(Some(2)), Cell::new(true));
        let d = Fake(&SPECS[4], Cell::new(Some(1)), Cell::new(false));
        let run = |op, yes| execute(&c, &PROFILES[0], &[&a, &b, &d], op, yes, dir.path());
        assert!(run("apply", false).is_err());
        assert!(run("apply", true).is_err());
        assert_eq!((a.read()?, b.read()?, d.read()?), (None, Some(2), Some(1)));
        assert!(phase(&c, PROFILES[0].id)?.is_none());
        run("apply", true)?;
        assert!(transact(&c, &a, "undo", true, dir.path()).is_err());
        assert_eq!(owner(&c, d.spec().id)?, None);
        b.1.set(None);
        assert!(run("undo", true).is_err());
        assert_eq!(phase(&c, PROFILES[0].id)?.as_deref(), Some("restoring"));
        b.1.set(Some(1));
        run("undo", true)?;
        assert_eq!((a.read()?, b.read()?, d.read()?), (None, Some(2), Some(1)));
        Ok(())
    }
}
