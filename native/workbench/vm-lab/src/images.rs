use super::{guarded_dirs, medium, read_json, safe_rel, R};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct Definitions {
    pub(super) path: PathBuf,
    pub(super) sha256: String,
    pub(super) bytes: u64,
    pub(super) collected_utc: String,
    #[serde(rename = "targetRescueSHA256")]
    pub(super) target: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DefinitionRelease {
    toolbox_relative_directory: String,
    filename: String,
    sha256: String,
    bytes: u64,
    collected_utc: String,
    #[serde(rename = "targetRescueSHA256")]
    target: String,
}

pub(super) fn definitions(toolbox: &Path) -> R<Option<Definitions>> {
    let data: Value = serde_json::from_str(include_str!("../../media/clamav-definitions.json"))
        .map_err(super::err)?;
    if data["available"] == false {
        return Ok(None);
    }
    let pin: DefinitionRelease = serde_json::from_value(data).map_err(super::err)?;
    let override_file = toolbox.join(".inta-clamav.json");
    let rel = if override_file.try_exists().map_err(super::err)? {
        read_json::<Location>(&override_file)?.iso
    } else {
        format!("{}/{}", pin.toolbox_relative_directory, pin.filename)
    };
    let path = toolbox.join(safe_rel(&rel)?);
    if !path.try_exists().map_err(super::err)? {
        return Ok(None);
    }
    let iso = medium(&path, true)?;
    if iso.bytes != pin.bytes {
        return Err("The local ClamAV definition CD has an unexpected size".into());
    }
    Ok(Some(Definitions {
        path,
        bytes: pin.bytes,
        sha256: pin.sha256,
        collected_utc: pin.collected_utc,
        target: pin.target,
    }))
}

#[derive(Deserialize)]
struct Registry<T> {
    images: Vec<T>,
}
#[derive(Deserialize)]
struct Pin {
    bytes: u64,
    sha256: String,
    architecture: Option<String>,
}
#[derive(Deserialize)]
struct Location {
    iso: String,
    architecture: Option<String>,
}

pub(super) fn list(toolbox: &Path) -> R<Value> {
    let _root = guarded_dirs(toolbox)?;
    let mut images = Vec::new();
    let mut issues = Vec::new();
    for (marker, name, manifest) in [
        (
            ".inta-recovery-lab.json",
            "INTA Rescue Recovery",
            include_str!("../../../../data/rescue-recovery-download-channel.json"),
        ),
        (
            ".inta-cloud-lab.json",
            "INTA Linux Rescue",
            include_str!("../../media/rescue-releases.json"),
        ),
        (
            ".inta-winpe-lab.json",
            "INTA Windows PE",
            include_str!("../../media/windows-rescue-releases.json"),
        ),
    ] {
        let file = toolbox.join(marker);
        if !file.try_exists().map_err(super::err)? {
            continue;
        }
        let result: R<()> = (|| {
            let locations = if marker == ".inta-winpe-lab.json" {
                read_json::<Registry<Location>>(&file)?.images
            } else {
                vec![read_json::<Location>(&file)?]
            };
            let pins = if marker == ".inta-recovery-lab.json" {
                vec![serde_json::from_str::<Pin>(manifest).map_err(super::err)?]
            } else {
                serde_json::from_str::<Registry<Pin>>(manifest).map_err(super::err)?.images
            };
            for entry in locations {
                let path = toolbox.join(safe_rel(&entry.iso)?);
                let mut boot = medium(&path, true)?;
                let mut candidates = pins
                    .iter()
                    .filter(|p| p.bytes == boot.bytes && p.architecture == entry.architecture)
                    .peekable();
                let first = candidates
                    .next()
                    .ok_or("The local image has no matching release record")?;
                let pin = if candidates.peek().is_some() {
                    let digest = super::sha(&mut boot.file)?;
                    std::iter::once(first)
                        .chain(candidates)
                        .find(|p| p.sha256 == digest)
                        .ok_or("The local image does not match a release of the same size")?
                } else {
                    first
                };
                let arch = entry.architecture.as_deref().unwrap_or("amd64");
                images.push(
                    json!({"name":name, "architecture":arch, "path":path, "sha256":pin.sha256,
                    "bytes":boot.bytes, "canBoot":arch=="amd64"}),
                );
            }
            Ok(())
        })();
        if let Err(error) = result {
            issues.push(format!("{name}: {error}"));
        }
    }
    let module = match definitions(toolbox) {
        Ok(item) => item,
        Err(error) => {
            issues.push(error);
            None
        }
    };
    Ok(json!({"images":images,"definitions":module,"issues":issues}))
}
