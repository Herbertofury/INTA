use crate::models::OperationRecord;
use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

pub struct OperationStore {
    records: Mutex<BTreeMap<String, OperationRecord>>,
    locks: Mutex<HashMap<String, Arc<Mutex<()>>>>,
    write_lock: Mutex<()>,
    path: PathBuf,
}

impl OperationStore {
    pub fn open(directory: impl AsRef<Path>) -> io::Result<Self> {
        fs::create_dir_all(&directory)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&directory, fs::Permissions::from_mode(0o700))?;
        }
        let path = directory.as_ref().join("operations.jsonl");
        let mut records = BTreeMap::new();
        if path.is_file() {
            let mut lines = VecDeque::new();
            for line in BufReader::new(File::open(&path)?)
                .lines()
                .map_while(Result::ok)
            {
                if lines.len() == 500 {
                    lines.pop_front();
                }
                lines.push_back(line);
            }
            for line in lines {
                if let Ok(record) = serde_json::from_str::<OperationRecord>(&line) {
                    records.insert(record.id.clone(), record);
                }
            }
        }
        let store = Self {
            records: Mutex::new(records),
            locks: Mutex::new(HashMap::new()),
            write_lock: Mutex::new(()),
            path,
        };
        let interrupted: Vec<_> = store.records.lock().unwrap().values().filter(|r| r.state == "queued" || r.state == "running").cloned().map(|mut r| {
            r.state = "interrupted".into();
            r.error = Some("The INTA agent restarted while this operation was active. Inspect the installer and logs before retrying; an installer may still be running.".into());
            r
        }).collect();
        if !interrupted.is_empty() {
            store.save_batch(&interrupted)?;
        }
        Ok(store)
    }
    pub fn recent(&self) -> Vec<OperationRecord> {
        let mut records: Vec<_> = self.records.lock().unwrap().values().cloned().collect();
        records.sort_by(|a, b| {
            b.started_at
                .cmp(&a.started_at)
                .then_with(|| b.id.cmp(&a.id))
        });
        records.truncate(100);
        records
    }
    pub fn get(&self, id: &str) -> Option<OperationRecord> {
        self.records.lock().unwrap().get(id).cloned()
    }
    pub fn lock_for(&self, manager: &str) -> Arc<Mutex<()>> {
        self.locks
            .lock()
            .unwrap()
            .entry(manager.into())
            .or_insert_with(|| Arc::new(Mutex::new(())))
            .clone()
    }
    pub fn save(&self, record: &OperationRecord) -> io::Result<()> {
        self.save_batch(std::slice::from_ref(record))
    }
    pub fn save_batch(&self, records: &[OperationRecord]) -> io::Result<()> {
        let _guard = self.write_lock.lock().unwrap();
        let mut bytes = Vec::new();
        for record in records {
            serde_json::to_writer(&mut bytes, record)?;
            bytes.push(b'\n');
        }
        let mut options = OpenOptions::new();
        options.create(true).append(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let mut file = options.open(&self.path)?;
        file.write_all(&bytes)?;
        file.sync_data()?;
        let mut current = self.records.lock().unwrap();
        for record in records {
            current.insert(record.id.clone(), record.clone());
        }
        Ok(())
    }
}
