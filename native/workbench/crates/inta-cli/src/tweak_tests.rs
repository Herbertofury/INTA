use super::*;
use std::cell::Cell;

#[cfg(windows)]
#[test]
fn windows_registry_round_trip_in_disposable_key() -> Result<()> {
    use winreg::{RegKey, enums::HKEY_CURRENT_USER};
    struct TestKey(RegKey, &'static Spec);
    impl Setting for TestKey {
        fn spec(&self) -> &Spec {
            self.1
        }
        fn read(&self) -> Result<Option<u32>> {
            read_setting(&self.0, self.1.value)
        }
        fn write(&self, v: Option<u32>) -> Result<()> {
            write_setting(&self.0, self.1.value, v)
        }
    }
    let parent = RegKey::predef(HKEY_CURRENT_USER).open_subkey_with_flags(
        "Software",
        winreg::enums::KEY_READ | winreg::enums::KEY_WRITE,
    )?;
    let name = format!("INTA-Test-{}", uuid::Uuid::new_v4());
    let (key, _) = parent.create_subkey(&name)?;

    let dir = tempfile::tempdir()?;
    let c = Connection::open_in_memory()?;
    c.execute_batch("CREATE TABLE tweaks(id TEXT PRIMARY KEY,before_value TEXT,phase TEXT);CREATE TABLE events(at TEXT,operation TEXT,state TEXT);")?;
    for spec in SPECS {
        let registry = TestKey(
            key.open_subkey_with_flags("", winreg::enums::KEY_READ | winreg::enums::KEY_SET_VALUE)?,
            spec,
        );
        let other = spec
            .valid
            .iter()
            .copied()
            .find(|v| *v != spec.target)
            .unwrap();
        for previous in [None, Some(other)] {
            if previous.is_some() {
                registry.write(previous)?;
            }
            transact(&c, &registry, "apply", true, dir.path())?;
            assert_eq!(registry.read()?, Some(spec.target));
            transact(&c, &registry, "undo", true, dir.path())?;
            assert_eq!(registry.read()?, previous);
        }
    }
    drop(key);
    parent.delete_subkey(name)?;
    Ok(())
}

struct Fixture {
    value: Cell<Option<u32>>,
    fail: Cell<bool>,
}
impl Setting for Fixture {
    fn spec(&self) -> &Spec {
        &SPECS[0]
    }
    fn read(&self) -> Result<Option<u32>> {
        Ok(self.value.get())
    }
    fn write(&self, value: Option<u32>) -> Result<()> {
        if self.fail.get() {
            bail!("Fixture write unavailable")
        }
        self.value.set(value);
        Ok(())
    }
}
#[test]
fn snapshots_survive_failures_and_guard_against_conflicts() -> Result<()> {
    let dir = tempfile::tempdir()?;
    let conn = Connection::open_in_memory()?;
    conn.execute_batch("CREATE TABLE tweaks(id TEXT PRIMARY KEY,before_value TEXT NOT NULL,phase TEXT NOT NULL);
 CREATE TABLE events(seq INTEGER PRIMARY KEY,at TEXT NOT NULL,operation TEXT NOT NULL,state TEXT NOT NULL);")?;
    for original in [None, Some(1)] {
        let key = Fixture {
            value: Cell::new(original),
            fail: Cell::new(true),
        };
        let work = |op, yes| transact(&conn, &key, op, yes, dir.path());
        assert!(work("apply", false).is_err());
        assert!(work("apply", true).is_err());
        assert_eq!(work("status", false)?["phase"], "prepared");
        key.fail.set(false);
        assert_eq!(work("apply", true)?["verified"], true);
        assert_eq!(work("apply", true)?["state"], "already-applied");
        key.value.set(Some(1));
        assert!(work("undo", true).is_err());
        key.value.set(Some(0));
        key.fail.set(true);
        assert!(work("undo", true).is_err());
        assert_eq!(work("status", false)?["phase"], "restoring");
        key.fail.set(false);
        assert_eq!(work("undo", true)?["verified"], true);
        assert_eq!(key.value.get(), original);
        assert!(!work("status", false)?["undoAvailable"].as_bool().unwrap());
    }
    Ok(())
}
