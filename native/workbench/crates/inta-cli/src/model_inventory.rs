use super::{ENTRY_LIMIT, reparse};
use anyhow::{Context, Result, ensure};
use serde_json::{Value, json};
use std::{
    env, fs,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
use walkdir::WalkDir;

fn setting(key: &str, default: PathBuf) -> PathBuf {
    env::var_os(key)
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .unwrap_or(default)
}
fn locations(home: &Path) -> Vec<(&'static str, PathBuf)> {
    let cache = setting("XDG_CACHE_HOME", home.join(".cache"));
    let hf = setting("HF_HOME", cache.join("huggingface"));
    vec![
        (
            "Ollama",
            setting("OLLAMA_MODELS", home.join(".ollama/models")),
        ),
        ("Hugging Face", setting("HF_HUB_CACHE", hf.join("hub"))),
        (
            "LM Studio",
            setting("INTA_LMSTUDIO_MODELS", home.join(".lmstudio/models")),
        ),
        (
            "LM Studio (legacy location)",
            home.join(".cache/lm-studio/models"),
        ),
        (
            "ComfyUI",
            setting("INTA_COMFYUI_MODELS", home.join("Documents/ComfyUI/models")),
        ),
    ]
}
fn check_path(p: &Path) -> Result<()> {
    ensure!(
        p.is_absolute() && p.parent().is_some(),
        "Expected a local model-store directory"
    );
    #[cfg(windows)]
    {
        use std::path::{Component, Prefix};
        use windows_sys::Win32::Storage::FileSystem::GetDriveTypeW;
        let Some(Component::Prefix(prefix)) = p.components().next() else {
            anyhow::bail!("Expected a local path")
        };
        let Prefix::Disk(d) = prefix.kind() else {
            anyhow::bail!("Network and device paths excluded")
        };
        ensure!(
            matches!(
                unsafe { GetDriveTypeW([u16::from(d), 58, 92, 0].as_ptr()) },
                2 | 3 | 6
            ),
            "Network volumes excluded"
        );
    }
    for dir in p.ancestors() {
        match fs::symlink_metadata(dir) {
            Ok(m) => ensure!(
                m.is_dir() && !reparse(&m),
                "Linked model-store paths excluded"
            ),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}
fn inventory(name: &str, path: &Path) -> Result<Value> {
    check_path(path)?;
    if !path.try_exists()? {
        return Ok(json!({"provider":name,"status":"location-absent","location":path}));
    }
    let mut files = 0usize;
    let mut bytes = 0u64;
    let errors = std::cell::Cell::new(0u64);
    let mut partial = false;
    let start = Instant::now();
    let entries = WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            if name == "Hugging Face"
                && e.depth() == 1
                && !e.file_name().to_string_lossy().starts_with("models--")
            {
                return false;
            }
            match e.metadata() {
                Ok(m) => !reparse(&m),
                Err(_) => {
                    errors.set(errors.get() + 1);
                    false
                }
            }
        });
    for (index, entry) in entries.enumerate() {
        if index >= ENTRY_LIMIT || start.elapsed() > Duration::from_secs(6) {
            partial = true;
            break;
        }
        match entry.and_then(|e| e.metadata()) {
            Ok(m) if m.is_file() => {
                files += 1;
                bytes = bytes.saturating_add(m.len());
            }
            Ok(_) => (),
            Err(_) => errors.set(errors.get() + 1),
        }
    }
    Ok(
        json!({"provider":name,"status":if partial || errors.get()>0 {"partial"}else{"observed"},"location":path,"fileCount":files,"logicalBytes":bytes,"readErrors":errors.get()}),
    )
}
pub fn models() -> Result<Value> {
    let home = PathBuf::from(
        env::var_os(if cfg!(windows) { "USERPROFILE" } else { "HOME" })
            .context("User profile unavailable")?,
    );
    let rows: Vec<_> = locations(&home)
        .iter()
        .map(|(name, path)| {
            inventory(name, path).unwrap_or_else(
                |e| json!({"provider":name,"status":"unavailable","detail":e.to_string()}),
            )
        })
        .collect();
    Ok(
        json!({"schemaVersion":1,"kind":"models","observedAt":crate::db::now(),"observations":rows,"scope":"Local model-store contents, including metadata. Logical bytes count each regular file entry. Reparse points and links are skipped. Custom model paths can be selected through folder space analysis."}),
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn separates_model_repositories_from_datasets_and_secrets() -> Result<()> {
        let dir = tempfile::tempdir()?;
        for name in ["models--publisher--model", "datasets--publisher--data"] {
            fs::create_dir(dir.path().join(name))?;
            fs::write(dir.path().join(name).join("blob"), b"12345")?;
        }
        fs::write(dir.path().join("token"), b"ignore")?;
        let result = inventory("Hugging Face", dir.path())?;
        assert_eq!(result["fileCount"], 1);
        assert_eq!(result["logicalBytes"], 5);
        assert_eq!(
            inventory("missing", &dir.path().join("missing"))?["status"],
            "location-absent"
        );
        Ok(())
    }
}
