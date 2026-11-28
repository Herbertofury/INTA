use super::*;
use std::io::Write;

struct Lab {
    _tmp: tempfile::TempDir,
    c: Connection,
    root: PathBuf,
}
impl Lab {
    fn new() -> Self {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path().join("sample-project");
        fs::create_dir(&root).unwrap();
        let c = Connection::open(tmp.path().join("fixture.sqlite3")).unwrap();
        c.execute_batch("CREATE TABLE events(seq INTEGER PRIMARY KEY,at TEXT NOT NULL,operation TEXT NOT NULL,state TEXT NOT NULL)").unwrap();
        Self { _tmp: tmp, c, root }
    }
    fn cache(&self, name: &str) -> PathBuf {
        let d = self.root.join("__pycache__");
        fs::create_dir_all(&d).unwrap();
        fs::write(self.root.join(format!("{name}.py")), b"x=42\n").unwrap();
        let p = d.join(format!("{name}.cpython-313.pyc"));
        fs::write(&p, b"\xf3\r\r\n\0\0\0\0abcdefghiINTERNAL GENERATED FIXTURE").unwrap();
        p
    }
    fn plan(&self) -> Review {
        serde_json::from_value(command(&self.c, &["plan", self.root.to_str().unwrap()]).unwrap())
            .unwrap()
    }
    fn act(&self, r: &Review, op: &str, item: &Item) -> Value {
        command(&self.c, &[op, &r.id, &item.id, "--approve"]).unwrap()
    }
}

#[test]
fn excludes_documents_credentials_cloud_hardlinks_open_files_and_named_streams() {
    let l = Lab::new();
    let good = l.cache("module");
    let no_source = l.cache("orphan");
    fs::rename(l.root.join("orphan.py"), l.root.join("moved.py")).unwrap();
    let secret = l.cache("password");
    let locked = l.cache("busy");
    let _busy = OpenOptions::new()
        .read(true)
        .share_mode(0)
        .open(&locked)
        .unwrap();
    let hard = l.cache("hard");
    fs::hard_link(&hard, l.root.join("saved-copy")).unwrap();
    let ads = l.cache("stream");
    fs::write(format!("{}:private", ads.display()), b"unrelated data").unwrap();
    let offline = l.cache("offline");
    let wide = offline
        .as_os_str()
        .encode_wide()
        .chain([0])
        .collect::<Vec<_>>();
    assert_ne!(
        unsafe { SetFileAttributesW(wide.as_ptr(), FILE_ATTRIBUTE_OFFLINE) },
        0
    );
    for name in [
        "Documents",
        "Models",
        "OneDrive - Company",
        "Backups",
        ".ssh",
        "Clients",
    ] {
        let d = l.root.join(name);
        fs::create_dir_all(d.join("__pycache__")).unwrap();
        fs::write(d.join("module.py"), "x=42").unwrap();
        fs::copy(&good, d.join("__pycache__/module.cpython-313.pyc")).unwrap();
    }
    fs::write(l.root.join("invoice.docx.tmp"), "customer document").unwrap();
    let r = l.plan();
    assert_eq!(r.files.len(), 1, "{}", serde_json::to_string(&r).unwrap());
    assert_eq!(r.files[0].path, good);
    for file in [secret, no_source, hard, ads, offline] {
        assert!(file.exists())
    }
    assert!(r.vault.is_none());
}

#[test]
fn exact_selected_file_roundtrip_audit_and_collision() {
    let l = Lab::new();
    let p = l.cache("alpha");
    let unselected = l.cache("beta");
    let raw = fs::read(&p).unwrap();
    let r = l.plan();
    let first = &r.files[0];
    assert!(command(&l.c, &["quarantine", &r.id, &first.id]).is_err());
    assert!(command(&l.c, &["quarantine", &r.id, &uuid(), "--approve"]).is_err());
    assert_eq!(l.act(&r, "quarantine", first)["moved"], 1);
    assert!(!p.exists() && unselected.exists());
    fs::write(&p, b"new original at same path").unwrap();
    assert_eq!(l.act(&r, "restore", first)["preserved"], 1);
    assert_eq!(fs::read(&p).unwrap(), b"new original at same path");
    fs::rename(&p, l.root.join("collision-copy")).unwrap();
    assert_eq!(l.act(&r, "restore", first)["moved"], 1);
    assert_eq!(fs::read(&p).unwrap(), raw);
    let audit = command(&l.c, &["audit", &r.id]).unwrap();
    assert!(audit.as_array().unwrap().len() >= 5);
}

#[test]
fn stale_same_name_files_and_edited_content_are_preserved() {
    let l = Lab::new();
    let a = l.cache("alpha");
    let b = l.cache("beta");
    let r = l.plan();
    fs::rename(&a, l.root.join("old-alpha")).unwrap();
    fs::copy(l.root.join("old-alpha"), &a).unwrap();
    OpenOptions::new()
        .append(true)
        .open(&b)
        .unwrap()
        .write_all(b"new content")
        .unwrap();
    for i in &r.files {
        assert_eq!(l.act(&r, "quarantine", i)["preserved"], 1);
    }
    assert!(a.exists() && b.exists());
}

#[test]
fn reconciles_interrupted_renames_from_both_directions() {
    let l = Lab::new();
    let p = l.cache("alpha");
    let r = l.plan();
    let f = &r.files[0];
    assert_eq!(l.act(&r, "quarantine", f)["moved"], 1);
    let mut interrupted = load(&l.c, &r.id).unwrap();
    interrupted.files[0].state = "quarantining".into();
    save(&l.c, &interrupted, None).unwrap();
    let recovered = command(&l.c, &["review", &r.id]).unwrap();
    assert_eq!(recovered["files"][0]["state"], "quarantined");
    interrupted.files[0].state = "restoring".into();
    save(&l.c, &interrupted, None).unwrap();
    let slot = l
        .root
        .join(format!(".inta-quarantine-{}", interrupted.vault.unwrap()))
        .join(format!("{}.bin", f.id));
    fs::rename(slot, p).unwrap();
    assert_eq!(
        command(&l.c, &["review", &r.id]).unwrap()["files"][0]["state"],
        "restored"
    );
}

#[test]
fn redirected_parent_cannot_move_an_external_file() {
    let l = Lab::new();
    let file = l.cache("alpha");
    let r = l.plan();
    let cache = file.parent().unwrap();
    let outside = l._tmp.path().join("external-cache");
    fs::rename(cache, &outside).unwrap();
    let powershell = crate::tool_actions::system_dir()
        .unwrap()
        .join("System32/WindowsPowerShell/v1.0/powershell.exe");
    let status=std::process::Command::new(powershell).args(["-NoProfile","-NonInteractive","-Command","$ErrorActionPreference='Stop'; New-Item -ItemType Junction -Path $env:INTA_TEST_LINK -Target $env:INTA_TEST_TARGET | Out-Null"]).env("INTA_TEST_LINK",cache).env("INTA_TEST_TARGET",&outside).status().unwrap();
    assert!(status.success());
    let result = l.act(&r, "quarantine", &r.files[0]);
    assert_eq!(result["preserved"], 1);
    assert!(outside.join(file.file_name().unwrap()).is_file());
    assert!(l.plan().files.is_empty());
}

#[test]
fn recognizes_declared_caches_and_leaves_unmarked_temporary_files() {
    let l = Lab::new();
    let declared = l.root.join("scratch-cache");
    let pytest = l.root.join(".pytest_cache");
    fs::create_dir(&declared).unwrap();
    fs::create_dir_all(pytest.join("v/cache")).unwrap();
    fs::write(declared.join("CACHEDIR.TAG"), TAG).unwrap();
    fs::write(pytest.join("CACHEDIR.TAG"), TAG).unwrap();
    let name = "1234567890abcdef.tmp";
    fs::write(declared.join(name), b"scratch").unwrap();
    fs::write(l.root.join(name), b"unmarked").unwrap();
    fs::write(pytest.join("v/cache/nodeids"), b"[]").unwrap();
    assert_eq!(l.plan().files.len(), 2);
}
