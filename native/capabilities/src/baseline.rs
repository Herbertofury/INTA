//! Only these parameter-free observations can be proposed by the local model.
use crate::{
    Result, catalog::Catalog, err, expect, files, forge, invalid, operations, registry::Op,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BaselineObservation {
    #[serde(rename = "system.memory")]
    Memory,
    #[serde(rename = "system.power")]
    Power,
    #[serde(rename = "startup.counts")]
    Startup,
}

impl BaselineObservation {
    pub const ALL: [Self; 3] = [Self::Memory, Self::Power, Self::Startup];
    fn operation(self) -> Op {
        match self {
            Self::Memory => Op::Memory,
            Self::Power => Op::Power,
            Self::Startup => Op::Startup,
        }
    }
    fn pointers(self) -> &'static [&'static str] {
        match self {
            Self::Memory => &["/totalPhysicalBytes"],
            Self::Power => &["/activeId", "/count"],
            Self::Startup => &[
                "/sources/0/registered",
                "/sources/1/registered",
                "/sources/2/registered",
                "/sources/3/registered",
                "/sources/4/registered",
                "/sources/5/registered",
            ],
        }
    }
}

/// Holds local parent directories against substitution for the entire generation.
pub struct BaselineDestination {
    folder: files::Directory,
    path: PathBuf,
}

impl BaselineDestination {
    pub fn new(path: &Path) -> Result<Self> {
        expect(
            path.is_absolute(),
            "Select a new, absolute local .json file.",
        )?;
        #[cfg(windows)]
        let path = files::host::local(path)?;
        #[cfg(not(windows))]
        let path = path.to_owned();
        expect(
            path.extension()
                .and_then(|s| s.to_str())
                .is_some_and(|s| s.eq_ignore_ascii_case("json")),
            "The new Forge recipe must have a .json extension.",
        )?;
        let folder = files::directory(
            path.parent().ok_or_else(|| invalid("Select a file."))?,
            false,
        )?;
        match fs::symlink_metadata(&path) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Self { folder, path }),
            Ok(_) => Err(err(
                "conflict",
                "Choose a new recipe filename. The destination already exists.",
            )),
            Err(_) => Err(err("io", "Cannot inspect the recipe destination.")),
        }
    }

    pub fn export(&self, selection: &[BaselineObservation]) -> Result<Value> {
        expect(
            !selection.is_empty() && selection.len() <= BaselineObservation::ALL.len(),
            "Select one to three baseline observations.",
        )?;
        let ordered: Vec<_> = BaselineObservation::ALL
            .into_iter()
            .filter(|op| selection.contains(op))
            .collect();
        expect(
            ordered.len() == selection.len(),
            "Each observation must appear only once.",
        )?;
        let catalog = Catalog::load()?;
        let mut steps = Vec::new();
        let mut tests = Vec::new();
        for (index, item) in ordered.iter().enumerate() {
            let operation = item.operation().id();
            let observation = operations::run(&catalog, operation, json!({}))?;
            steps.push(json!({"operation":operation,"version":1,"input":{}}));
            for pointer in item.pointers() {
                let observed = observation
                    .pointer(pointer)
                    .ok_or_else(|| err("incomplete", "The baseline field was absent."))?;
                tests.push(json!({"step":index,"pointer":pointer,"equals":observed}));
            }
        }
        let hash = format!(
            "{:x}",
            Sha256::digest(serde_json::to_vec(&(&steps, &tests)).expect("serializable baseline"))
        );
        let manifest = json!({"schemaVersion":1,"id":format!("local-baseline-{}",&hash[..20]),"title":"Local workstation baseline","version":1,"steps":steps,"tests":tests});
        // A second independent native observation must still satisfy the baseline before publication.
        let mut result = forge::verify_value(&catalog, manifest.clone())?;
        let mut file = tempfile::NamedTempFile::new_in(&self.folder.path)
            .map_err(|_| err("io", "Cannot stage the recipe."))?;
        let data = serde_json::to_vec_pretty(&manifest).expect("serializable manifest");
        file.write_all(&data)
            .and_then(|_| file.as_file().sync_all())
            .map_err(|_| err("io", "Could not flush the recipe."))?;
        file.persist_noclobber(&self.path).map_err(|_| {
            err(
                "conflict",
                "Could not publish the new recipe. Check that the destination is still unused.",
            )
        })?;
        result["selectedOperations"] = json!(ordered);
        result["state"] = json!("exported");
        Ok(result)
    }
}
