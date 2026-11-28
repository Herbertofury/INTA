use anyhow::{Context, Result, anyhow, bail, ensure};
use rusqlite::{Connection, OptionalExtension};
use serde_json::{Value, json};
use std::{
    env,
    path::{Path, PathBuf},
};

fn checked(value: std::result::Result<Value, String>) -> Result<Value> {
    value.map_err(|s| anyhow!(s))
}
fn anchors() -> Result<Vec<PathBuf>> {
    let parent = env::current_exe()?
        .parent()
        .context("Application path unavailable")?
        .to_owned();
    let mut dirs = vec![
        parent.clone(),
        parent.parent().unwrap_or(&parent).to_owned(),
    ];
    #[cfg(windows)]
    {
        use windows_sys::Win32::Storage::FileSystem::{GetDriveTypeW, GetLogicalDrives};
        let mask = unsafe { GetLogicalDrives() };
        for i in 0..26 {
            if mask & (1 << i) == 0 {
                continue;
            }
            let root = format!("{}:\\", (b'A' + i) as char);
            let wide = root.encode_utf16().chain([0]).collect::<Vec<_>>();
            let kind = unsafe { GetDriveTypeW(wide.as_ptr()) };
            if matches!(kind, 2 | 3) {
                dirs.push(root.into());
            }
        }
    }
    Ok(dirs)
}
fn saved(c: &Connection) -> Result<Option<(PathBuf, String)>> {
    let raw: Option<String> = c
        .query_row("SELECT value FROM metadata WHERE key='toolbox'", [], |r| {
            r.get(0)
        })
        .optional()?;
    Ok(raw.map(|val| serde_json::from_str(&val)).transpose()?)
}
pub(crate) fn select(c: &Connection, explicit: Option<&str>) -> Result<PathBuf> {
    if let Some(p) = explicit
        .map(PathBuf::from)
        .or_else(|| env::var_os("INTA_TOOLBOX").map(PathBuf::from))
    {
        ensure!(p.is_absolute(), "Toolbox path must be absolute");
        return Ok(p);
    }
    let selection = saved(c)?;
    if let Some((p, id)) = &selection
        && checked(inta_toolbox::list(p)).is_ok_and(|v| v["toolboxId"] == *id)
    {
        return Ok(p.clone());
    }
    let mut roots = anchors()?;
    if let Some((p, _)) = &selection
        && let Some(parent) = p.parent()
    {
        roots.insert(0, parent.to_owned())
    }
    let discovered = checked(inta_toolbox::discover(&roots))?;
    let mut matches = discovered["roots"]
        .as_array()
        .context("Invalid discovery results")?
        .iter()
        .filter(|v| {
            selection
                .as_ref()
                .is_none_or(|(_, id)| v["toolboxId"] == *id)
        });
    let p = matches
        .next()
        .and_then(|v| v["path"].as_str())
        .context("Toolbox unavailable. Connect it or run inta toolbox use DIRECTORY.")?;
    ensure!(
        matches.next().is_none(),
        "Multiple toolbox roots match. Choose one with inta toolbox use DIRECTORY."
    );
    Ok(PathBuf::from(p))
}
pub fn command(c: &Connection, args: &[&str]) -> Result<Value> {
    let (args, root) = match args {
        [args @ .., "--root", p] => (args, Some(*p)),
        _ => (args, None),
    };
    if let ["discover"] = args {
        return checked(inta_toolbox::discover(&anchors()?));
    }
    if let ["use", p] = args {
        let p = Path::new(p);
        ensure!(p.is_absolute(), "Toolbox path must be absolute");
        let data = checked(inta_toolbox::list(p))?;
        let id = data["toolboxId"]
            .as_str()
            .context("Missing toolbox identity")?;
        let value = serde_json::to_string(&(p, id))?;
        c.execute(
            "INSERT OR REPLACE INTO metadata(key,value) VALUES('toolbox',?)",
            [value],
        )?;
        return Ok(json!({"toolboxId":id,"root":p,"state":"selected"}));
    }
    let p = select(c, root)?;
    match args {
        ["list"] => checked(inta_toolbox::list(&p)),
        ["backup", source, destination, "--approve"] => {
            crate::audited(c, "toolbox/verified-backup", || {
                checked(inta_toolbox::create_backup(
                    &p,
                    Path::new(source),
                    Path::new(destination),
                ))
            })
        }
        ["backup", ..] => bail!("Expected toolbox backup SOURCE NEW.7z --approve"),
        ["archive", file] => crate::audited(c, "toolbox/archive-test", || {
            checked(inta_toolbox::inventory_archive(&p, Path::new(file)))
        }),
        ["verify", id] => crate::audited(c, "toolbox/verify", || {
            checked(inta_toolbox::verify(&p, id))
        }),
        ["probe", id] => {
            crate::audited(c, "toolbox/probe", || checked(inta_toolbox::probe(&p, id)))
        }
        _ => bail!("Expected toolbox list, discover, use, verify or probe"),
    }
}
