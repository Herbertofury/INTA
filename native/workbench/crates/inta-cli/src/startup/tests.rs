use super::*;

struct Fixture {
    locations: Locations,
    path: String,
    run: RegKey,
}
impl Fixture {
    fn new() -> Result<Self> {
        let id = uuid::Uuid::new_v4();
        let locations = Locations::fixture(id);
        let run = HKCU.create_subkey(&locations.run)?.0;
        Ok(Self {
            locations,
            path: format!(r"Software\INTA-Lab\StartupTests\{id}"),
            run,
        })
    }
    fn preview(&self, name: &str) -> Result<Value> {
        execute(
            &self.locations,
            &["preview", &identifier(&self.locations, name)],
        )
    }
    fn apply(&self, op: &str, view: &Value) -> Result<Value> {
        execute(
            &self.locations,
            &[
                op,
                view["id"].as_str().unwrap(),
                view["revision"].as_str().unwrap(),
                "--approve",
            ],
        )
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = HKCU.delete_subkey_all(&self.path);
    }
}
fn sample(kind: RegType) -> Raw {
    let bytes = r#""%LOCALAPPDATA%\sample.exe" --token=fixture-private"#
        .encode_utf16()
        .chain([0, 0])
        .flat_map(u16::to_le_bytes)
        .collect::<Vec<_>>();
    RegValue {
        vtype: kind,
        bytes: bytes.into(),
    }
}
#[test]
fn exact_bytes_remain_private_and_recover_after_reopen() -> Result<()> {
    let f = Fixture::new()?;
    for (name, kind) in [("Example App", REG_EXPAND_SZ), ("Other App", REG_SZ)] {
        let raw = sample(kind);
        f.run.set_raw_value(name, &raw)?;
        let p = f.preview(name)?;
        assert_eq!(p["state"], "registered");
        assert!(!p.to_string().contains("fixture-private"));
        assert!(
            execute(
                &f.locations,
                &[
                    "disable",
                    p["id"].as_str().unwrap(),
                    p["revision"].as_str().unwrap()
                ]
            )
            .is_err()
        );
        assert_eq!(f.apply("disable", &p)?["verified"], true);
        assert!(optional(f.run.get_raw_value(name))?.is_none());
        let store = HKCU.open_subkey(&f.locations.undo)?;
        let encrypted = store.get_raw_value(name)?;
        assert!(encrypted.vtype == REG_BINARY && encrypted.bytes != raw.bytes);
        assert!(protection::open("other entry", &encrypted).is_err());
        let resume = f.preview(name)?;
        assert_eq!(resume["state"], "quarantined");
        assert_eq!(f.apply("restore", &resume)?["verified"], true);
        assert!(f.run.get_raw_value(name)? == raw);
        assert!(optional(store.get_raw_value(name))?.is_none());
    }
    Ok(())
}
#[test]
fn stale_preview_external_replacement_and_unowned_backup_are_refused() -> Result<()> {
    let f = Fixture::new()?;
    f.run.set_raw_value("Owner", &sample(REG_SZ))?;
    let p = f.preview("Owner")?;
    f.run.set_value("Owner", &"changed")?;
    assert!(f.apply("disable", &p).is_err());
    let current = f.preview("Owner")?;
    f.apply("disable", &current)?;
    let saved = f.preview("Owner")?;
    f.run.set_value("owner", &"third party")?;
    assert_eq!(f.preview("Owner")?["state"], "conflict");
    assert!(f.apply("restore", &saved).is_err());
    assert_eq!(f.run.get_value::<String, _>("Owner")?, "third party");
    let store = HKCU.open_subkey_with_flags(&f.locations.undo, KEY_QUERY_VALUE | KEY_SET_VALUE)?;
    let protected = store.get_raw_value("Owner")?;
    store.set_raw_value("Forged", &protected)?;
    let invalid = f.preview("Forged")?;
    assert_eq!(invalid["state"], "recovery-unavailable");
    assert!(f.apply("restore", &invalid).is_err());
    assert_eq!(f.preview("Owner")?["state"], "conflict");
    store.delete_value("Forged")?;
    f.run.delete_value("owner")?;
    assert_eq!(
        f.apply("restore", &f.preview("Owner")?)?["state"],
        "restored"
    );
    Ok(())
}
#[test]
fn write_ahead_records_recover_both_interrupted_directions() -> Result<()> {
    let f = Fixture::new()?;
    let raw = sample(REG_SZ);
    f.run.set_raw_value("Keep", &raw)?;
    let id = identifier(&f.locations, "Keep");
    let undo = HKCU.create_subkey(&f.locations.undo)?.0;
    save(&undo, &id, "Keep", Phase::Prepared, &raw)?;
    let pending = f.preview("Keep")?;
    assert_eq!(pending["state"], "disable-interrupted");
    f.apply("disable", &pending)?;
    save(&undo, &id, "Keep", Phase::Restoring, &raw)?;
    f.run.set_raw_value("Keep", &raw)?;
    let restore = f.preview("Keep")?;
    assert_eq!(restore["state"], "restore-interrupted");
    f.apply("restore", &restore)?;
    assert!(f.run.get_raw_value("Keep")? == raw);
    assert!(optional(undo.get_raw_value("Keep"))?.is_none());
    Ok(())
}
#[test]
fn unsupported_and_unnamed_values_cannot_be_selected() -> Result<()> {
    let f = Fixture::new()?;
    f.run.set_value("", &"ignored")?;
    f.run.set_value("unsupported", &3u32)?;
    let result = execute(&f.locations, &["list"])?;
    let items = result["entries"].as_array().unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["state"], "unsupported");
    assert!(f.apply("disable", &items[0]).is_err());
    Ok(())
}

#[test]
fn overlapping_inta_operations_are_refused_and_foreign_values_preserved() -> Result<()> {
    let f = Fixture::new()?;
    f.run.set_raw_value("Keep", &sample(REG_SZ))?;
    let preview = f.preview("Keep")?;
    let held = lock(&f.locations)?;
    assert!(f.apply("disable", &preview).is_err());
    drop(held);
    f.apply("disable", &preview)?;
    let r = f.preview("Keep")?;
    f.run.set_raw_value("Keep", &sample(REG_SZ))?;
    assert!(f.apply("restore", &r).is_err());
    assert_eq!(f.preview("Keep")?["state"], "conflict");
    Ok(())
}
