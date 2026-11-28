use anyhow::{Context, Result, bail, ensure};
use rusqlite::{Connection, OptionalExtension, params};
use serde::Deserialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::{self, Read, Seek},
    path::{Path, PathBuf},
};

#[cfg(windows)]
mod host;
#[cfg(windows)]
mod managed;
#[cfg(windows)]
pub(crate) fn hold_local_directories(path: &Path, create: bool) -> Result<Vec<File>> {
    host::dirs(path, create)
}

#[cfg(windows)]
pub(crate) fn system_dir() -> Result<PathBuf> {
    host::windows()
}
#[cfg(windows)]
pub(crate) fn open_console(exe: &Path, args: &str) -> Result<Value> {
    host::console(exe, args)
}

#[derive(Deserialize, Default)]
struct Entry {
    id: String,
    winget: Option<String>,
    builtin: Option<String>,
    web: Option<String>,
    managed: Option<String>,
    #[serde(default)]
    executables: Vec<String>,
    #[serde(default)]
    filenames: Vec<String>,
    #[serde(default)]
    boot: Value,
    #[serde(default)]
    linux: Value,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Catalog {
    id: String,
    name: String,
    homepage: String,
    download_url: Option<String>,
    mode: String,
    platforms: Vec<String>,
}

fn route(id: &str) -> Result<(Catalog, Entry)> {
    let catalog = serde_json::from_str::<Vec<Catalog>>(include_str!(concat!(
        env!("OUT_DIR"),
        "/tools.json"
    )))?
    .into_iter()
    .find(|r| r.id == id)
    .context("Unknown catalog tool ID")?;
    let rule = serde_json::from_str::<Vec<Entry>>(include_str!("../../../data/tool-actions.json"))?
        .into_iter()
        .find(|r| r.id == id)
        .unwrap_or_else(|| Entry {
            id: id.to_owned(),
            ..Default::default()
        });
    Ok((catalog, rule))
}
fn enabled(value: &Value) -> bool {
    !value.is_null() && value != &Value::Bool(false)
}
fn link(s: &str) -> Result<&str> {
    let host = s
        .strip_prefix("https://")
        .and_then(|s| s.split(['/', '?', '#']).next())
        .context("A fixed HTTPS vendor page is required")?;
    ensure!(
        !host.is_empty()
            && host.contains('.')
            && host
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, b'.' | b'-'))
            && !s.contains(['\\', '"', '<', '>'])
            && !s.chars().any(|c| c.is_control() || c.is_whitespace()),
        "Invalid vendor page"
    );
    Ok(s)
}
fn base_allowed(rule: &Entry, name: &str) -> bool {
    name.to_ascii_lowercase().ends_with(".exe")
        && !name.contains(['\\', '/', ':'])
        && rule
            .filenames
            .iter()
            .map(String::as_str)
            .chain(rule.executables.iter().filter_map(|s| s.rsplit('/').next()))
            .any(|v| v.eq_ignore_ascii_case(name))
}
fn sha(f: &mut File) -> Result<String> {
    f.rewind()?;
    let mut hash = Sha256::new();
    io::copy(f, &mut hash)?;
    f.rewind()?;
    Ok(format!("{:x}", hash.finalize()))
}
fn check_pe(f: &mut File) -> Result<()> {
    let len = f.metadata()?.len();
    ensure!(
        (64..=1024 * 1024 * 1024).contains(&len),
        "Invalid executable size"
    );
    let mut hdr = [0; 64];
    f.read_exact(&mut hdr)?;
    ensure!(&hdr[..2] == b"MZ", "Expected a Windows PE executable");
    let offset = u32::from_le_bytes(hdr[60..].try_into()?);
    ensure!(
        u64::from(offset) + 24 <= len,
        "Truncated Windows executable"
    );
    f.seek(io::SeekFrom::Start(offset.into()))?;
    let mut pe = [0; 24];
    f.read_exact(&mut pe)?;
    let flags = u16::from_le_bytes([pe[22], pe[23]]);
    ensure!(
        &pe[..4] == b"PE\0\0" && flags & 2 != 0 && flags & 0x2000 == 0,
        "Expected a Windows executable application"
    );
    f.rewind()?;
    Ok(())
}
fn action(kind: &str, label: &str, available: bool, detail: impl AsRef<str>) -> Value {
    json!({"kind":kind,"label":label,"available":available,"detail":detail.as_ref()})
}
fn package_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 150
        && id.as_bytes()[0].is_ascii_alphanumeric()
        && id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-' | b'_' | b'+'))
}
fn binding(c: &Connection, id: &str) -> Result<Option<(PathBuf, String)>> {
    let raw: Option<String> = c
        .query_row(
            "SELECT value FROM metadata WHERE key=?",
            [format!("tool-binding/{id}")],
            |r| r.get(0),
        )
        .optional()?;
    raw.map(|v| serde_json::from_str(&v))
        .transpose()
        .map_err(Into::into)
}
#[cfg(windows)]
fn tenant_url(url: &str) -> Result<&str> {
    ensure!(
        url.len() <= 2048
            && !url.contains(['?', '#', '@', '\\', '"', '<', '>', '%'])
            && !url.chars().any(|ch| ch.is_whitespace() || ch.is_control()),
        "Use the HTTPS console address without a login, query, fragment or token"
    );
    let address = url
        .strip_prefix("https://")
        .context("The console requires HTTPS")?
        .split('/')
        .next()
        .context("Missing console hostname")?;
    let host = if let Some((h, port)) = address.split_once(':') {
        ensure!(
            port.parse::<u16>().is_ok_and(|p| p > 0),
            "Invalid console port"
        );
        h
    } else {
        address
    };
    ensure!(
        !host.is_empty()
            && host.len() <= 253
            && host.split('.').all(|label| !label.is_empty()
                && label.len() <= 63
                && label
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'-')
                && !label.starts_with('-')
                && !label.ends_with('-')),
        "Invalid console hostname"
    );
    Ok(url)
}
#[cfg(windows)]
pub fn command(c: &Connection, args: &[&str]) -> Result<Value> {
    let (args, root) = match args {
        [a @ .., "--root", p] => (a, Some(*p)),
        _ => (args, None),
    };
    if let Some(root) = root {
        host::safe_local(Path::new(root))?;
    }
    if args == ["actions"] {
        let catalog: Vec<Catalog> =
            serde_json::from_str(include_str!(concat!(env!("OUT_DIR"), "/tools.json")))?;
        let profiles: Vec<Entry> =
            serde_json::from_str(include_str!("../../../data/tool-actions.json"))?;
        let mut rules: std::collections::HashMap<_, _> = profiles
            .into_iter()
            .map(|entry| (entry.id.clone(), entry))
            .collect();
        let mut states = Vec::with_capacity(catalog.len());
        for tool in catalog {
            let rule = rules.remove(&tool.id).unwrap_or_else(|| Entry {
                id: tool.id.clone(),
                ..Default::default()
            });
            states.push(execute(c, "actions", tool, rule, None, root)?);
        }
        return Ok(Value::Array(states));
    }
    if let ["configure", id, url, "--approve"] = args {
        ensure!(
            matches!(*id, "screenconnect" | "meshcentral" | "wazuh"),
            "This tool does not support a tenant console"
        );
        route(id)?;
        let url = tenant_url(url)?;
        return crate::audited(c, &format!("tool/configure/{id}"), || {
            c.execute("INSERT INTO metadata(key,value) VALUES(?,?) ON CONFLICT(key) DO UPDATE SET value=excluded.value",params![format!("tool-console/{id}"),url])?;
            Ok(json!({"state":"configured","id":id,"console":url}))
        });
    }
    let (verb, id, selection) = match args {
        ["actions", id] => ("actions", *id, None),
        [
            verb @ ("install" | "launch" | "linux" | "workflow" | "terminal"),
            id,
            "--approve",
        ] => (*verb, *id, None),
        [verb @ ("bind" | "launch"), id, p, "--approve"] => (*verb, *id, Some(Path::new(p))),
        _ => bail!(
            "Expected tool actions ID, install|launch ID --approve, bind ID FILE.exe --approve or launch ID FILE.iso --approve"
        ),
    };
    let (cat, rule) = route(id)?;
    execute(c, verb, cat, rule, selection, root)
}

#[cfg(windows)]
fn execute(
    c: &Connection,
    verb: &str,
    cat: Catalog,
    rule: Entry,
    selection: Option<&Path>,
    root: Option<&str>,
) -> Result<Value> {
    let id = cat.id.as_str();
    if verb == "bind" {
        let path = selection.context("Select a Windows executable")?;
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .context("Invalid executable name")?;
        ensure!(
            base_allowed(&rule, name),
            "This executable basename is outside the catalog allowlist"
        );
        return crate::audited(c, &format!("tool/bind/{id}"), || {
            let mut image = host::Held::open(path)?;
            check_pe(&mut image.file)?;
            let digest = sha(&mut image.file)?;
            c.execute("INSERT INTO metadata(key,value) VALUES(?,?) ON CONFLICT(key) DO UPDATE SET value=excluded.value",params![format!("tool-binding/{id}"),serde_json::to_string(&(path,&digest))?])?;
            Ok(json!({"id":id,"state":"bound","path":path,"sha256":digest}))
        });
    }
    if verb == "linux" {
        ensure!(
            enabled(&rule.linux) && crate::linux_tools::available(id),
            "Linux environment is unavailable for this tool"
        );
        return crate::audited(c, &format!("tool/linux/{id}"), || {
            crate::linux_tools::launch(id)
        });
    }
    let toolbox = if rule.managed.is_some() || enabled(&rule.boot) {
        crate::toolbox::select(c, root)
            .ok()
            .filter(|p| host::dirs(p, false).is_ok())
    } else {
        None
    };
    let binding = binding(c, id)?;
    if verb == "terminal" {
        return managed::terminal(
            rule.managed
                .as_deref()
                .context("This tool has no managed console")?,
            toolbox.as_deref(),
        );
    }
    if verb == "workflow" {
        let pin = rule
            .managed
            .as_deref()
            .context("This catalog record has no managed workflow")?;
        return crate::audited(c, &format!("tool/workflow/{id}"), || {
            managed::workflow(pin, toolbox.as_deref())
        });
    }
    if let Some((p, _)) = &binding {
        host::safe_local(p)?;
    }
    let supports_tenant = matches!(id, "screenconnect" | "meshcentral" | "wazuh");
    let tenant: Option<String> = if supports_tenant {
        c.query_row(
            "SELECT value FROM metadata WHERE key=?",
            [format!("tool-console/{id}")],
            |r| r.get(0),
        )
        .optional()?
    } else {
        None
    };
    let tenant = tenant.as_deref().map(tenant_url).transpose()?;
    let configure = action(
        "configure-console",
        "Set console address",
        supports_tenant,
        "Choose your organization's HTTPS management console. Credentials and access tokens stay in the browser.",
    );
    let installed = host::installed(&rule);
    let registered_app = host::packaged_app(id);
    let pkg = rule.winget.as_deref().filter(|v| package_id(v));
    let winget = host::winget();
    let vendor = link(&cat.homepage)?;
    let acquire = cat
        .download_url
        .as_deref()
        .and_then(|u| link(u).ok())
        .filter(|u| {
            !u.split(['?', '#'])
                .next()
                .unwrap_or("")
                .to_ascii_lowercase()
                .ends_with(".exe")
        })
        .unwrap_or(vendor);
    let managed = rule.managed.as_deref();
    let builtin = rule.builtin.as_deref().and_then(host::builtin);
    let web = rule
        .web
        .as_deref()
        .map(link)
        .transpose()?
        .or_else(|| (cat.mode == "web").then_some(vendor));
    let portable = !rule.executables.is_empty() || !rule.filenames.is_empty();
    let bind = action(
        "bind-executable",
        "Choose installed executable",
        portable,
        "Choose a trusted vendor program with an approved filename. INTA records its SHA-256 and checks it before every launch.",
    );
    let boot = enabled(&rule.boot);
    let linux = enabled(&rule.linux)
        || cat
            .platforms
            .iter()
            .all(|s| s.eq_ignore_ascii_case("linux"));
    let panel = match rule.builtin.as_deref() {
        Some("inta-rescue") => Some(("rescue-media", "Open INTA Rescue")),
        Some("media-creator") => Some(("media-creator", "Open Media Creator")),
        Some("qemu" | "cloud-hypervisor") => Some(("vm-workspace", "Open Virtual machines")),
        _ => None,
    };
    let install = if let Some(m) = managed {
        action(
            "managed-install",
            "Install verified tool",
            true,
            format!(
                "Install the reviewed {m} binary for this computer from the toolbox, local package cache, or SHA-256-pinned official release."
            ),
        )
    } else if cat.mode == "web" {
        action(
            "web-service",
            "Browser service",
            false,
            "Open the official service with the launch control.",
        )
    } else if builtin.is_some() || panel.is_some() {
        action(
            "included",
            "Included component",
            false,
            "This component is available through its launch control when present.",
        )
    } else if let Some(p) = pkg.filter(|_| winget.is_some()) {
        action(
            "winget",
            "Install with WinGet",
            true,
            format!(
                "Install the exact package {p} from the winget community source. The installer may show its own prompts."
            ),
        )
    } else if enabled(&rule.linux) && crate::linux_tools::can_install(id) {
        action(
            "linux-install",
            "Install in Linux environment",
            true,
            "Install the exact distribution package in INTA's dedicated Linux environment.",
        )
    } else {
        action(
            "vendor-page",
            "Open official acquisition page",
            true,
            format!(
                "Open {acquire} to review the vendor's licensing, platform requirements and acquisition steps."
            ),
        )
    };
    let launch = if let Some((kind, label)) = panel {
        action(
            kind,
            label,
            true,
            match rule.builtin.as_deref() {
                Some("qemu") => {
                    "Choose the verified QEMU runtime and a local rescue ISO in Virtual machines."
                }
                Some("cloud-hypervisor") => {
                    "Use the Rust Cloud Hypervisor repair test or console with the prepared Linux KVM environment."
                }
                _ => "Open this workspace in INTA Desktop.",
            },
        )
    } else if supports_tenant {
        action(
            "configured-console",
            "Open configured console",
            tenant.is_some(),
            tenant.unwrap_or("Set the HTTPS address of your organization's existing console."),
        )
    } else if let Some((path, _)) = &binding {
        action(
            "bound-executable",
            "Launch selected executable",
            path.is_file(),
            format!("Check the recorded SHA-256 and launch {}.", path.display()),
        )
    } else if registered_app.is_some() {
        action(
            "registered-app",
            "Open installed desktop app",
            true,
            "Open the publisher's app registered with Windows for this account.",
        )
    } else if managed == Some("7zip") && installed.is_some() {
        action(
            "installed-executable",
            "Open 7-Zip File Manager",
            true,
            "Open the installed file manager to choose your own archive.",
        )
    } else if let Some(m) = managed {
        let exists = managed::available(m, toolbox.as_deref())?;
        action(
            "managed-terminal",
            "Open verified tool console",
            exists,
            "Open an interactive console for the verified program. Enter arguments and select your own working directory.",
        )
    } else if let Some(installation) = &installed {
        action(
            "installed-executable",
            "Launch installed tool",
            true,
            format!(
                "Open the {} or display command help at {}.",
                installation.provenance,
                installation.path.display()
            ),
        )
    } else if let Some(b) = &builtin {
        action(b.kind(), b.label, b.available(), b.detail)
    } else if let Some(w) = web {
        if cat.mode == "source project" {
            action("project-source", "Open project source", true, w)
        } else {
            action("web-app", "Open application", true, w)
        }
    } else if boot {
        action(
            "boot-media",
            "Boot selected ISO in lab",
            toolbox.is_some(),
            "Select a local optical ISO. Start an offline UEFI guest with fresh, file-backed writable storage.",
        )
    } else if linux {
        let ready = enabled(&rule.linux) && crate::linux_tools::available(id);
        action(
            "linux",
            if ready {
                "Open Linux tool"
            } else {
                "Linux environment required"
            },
            ready,
            "Open the prepared program in the dedicated Linux environment.",
        )
    } else {
        action(
            "unavailable",
            "Launch unavailable",
            false,
            if portable {
                "Install the official program or choose its trusted executable first."
            } else if cat.mode == "commercial" {
                "Use the licensed vendor deployment and management console."
            } else {
                "The catalog has no verified unattended launch route for this installation."
            },
        )
    };
    if verb == "actions" {
        let workflow = managed.map(|m| {
            action(
                "managed-workflow",
                managed::label(m),
                managed::available(m, toolbox.as_deref()).unwrap_or(false),
                "Run the pinned program on a temporary local sample and verify its result.",
            )
        });
        let alternative = action(
            "linux",
            "Open in Linux",
            enabled(&rule.linux) && crate::linux_tools::available(id),
            "Open this tool in INTA's prepared Linux environment.",
        );
        return Ok(
            json!({"id":id,"name":cat.name,"install":install,"launch":launch,"bind":bind,"configure":configure,"linux":alternative,"workflow":workflow}),
        );
    }
    let chosen = if verb == "install" { &install } else { &launch };
    ensure!(
        selection.is_none() || chosen["kind"] == "boot-media",
        "This tool does not accept file arguments"
    );
    ensure!(chosen["available"] == true, "{}", chosen["detail"]);
    crate::audited(c, &format!("tool/{verb}/{id}"), || {
        let mut result = match chosen["kind"].as_str().context("Unknown action")? {
            "managed-install" => managed::install(managed.unwrap(), toolbox.as_deref())?,
            "managed-terminal" => host::managed_terminal(
                managed.context("Managed tool unavailable")?,
                toolbox.as_deref(),
            )?,
            "winget" => host::install(
                &winget.context("Windows Package Manager unavailable")?,
                pkg.context("Package ID unavailable")?,
            )?,
            "vendor-page" => host::open_uri(acquire)?,
            "linux-install" => crate::linux_tools::install(id)?,
            "registered-app" => host::shell(
                std::ffi::OsStr::new(registered_app.context("Desktop app is unavailable")?),
                "",
                None,
            )?,
            "web-app" | "project-source" => host::open_uri(web.unwrap())?,
            "configured-console" => host::shell(
                std::ffi::OsStr::new(tenant.context("Set the console address")?),
                "",
                None,
            )?,
            "installed-executable" => {
                let app = installed.as_ref().unwrap();
                host::launch_file(&app.path, id, app.hash.as_ref())?
            }
            "bound-executable" => {
                let (path, digest) = binding.as_ref().unwrap();
                ensure!(
                    base_allowed(
                        &rule,
                        path.file_name().and_then(|s| s.to_str()).unwrap_or("")
                    ),
                    "Unrecognized bound program"
                );
                host::launch_file(path, id, Some(digest))?
            }
            "windows-component" | "windows-reference" => builtin.unwrap().run()?,
            "linux" => crate::linux_tools::launch(id)?,
            "rescue-media" | "media-creator" | "vm-workspace" => {
                json!({"state":"workspace","workspace":chosen["kind"],"detail":chosen["detail"]})
            }
            "boot-media" => {
                let source = selection.context("Select an absolute local ISO file")?;
                ensure!(
                    source
                        .extension()
                        .is_some_and(|x| x.eq_ignore_ascii_case("iso")),
                    "Select an ISO optical image"
                );
                inta_vm_lab::command(
                    toolbox.as_deref().context("Select a toolbox")?,
                    &[
                        "start",
                        source.to_str().context("Invalid ISO path")?,
                        "--approve",
                    ],
                )
                .map_err(anyhow::Error::msg)?
            }
            _ => bail!("Unavailable action"),
        };
        result["id"] = json!(id);
        result["name"] = json!(cat.name);
        result["action"] = chosen["kind"].clone();
        Ok(result)
    })
}

#[cfg(not(windows))]
pub fn command(_: &Connection, _: &[&str]) -> Result<Value> {
    bail!("Catalog actions require Windows")
}

#[cfg(all(test, windows))]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn tenant_configuration_stays_in_the_per_tool_metadata() {
        let c = Connection::open_in_memory().unwrap();
        c.execute_batch("CREATE TABLE metadata(key TEXT PRIMARY KEY,value TEXT NOT NULL);CREATE TABLE events(seq INTEGER PRIMARY KEY,at TEXT NOT NULL,operation TEXT NOT NULL,state TEXT NOT NULL);").unwrap();
        for url in [
            "http://example.org",
            "https://name:secret@example.org",
            "https://example.org/?token=abc",
            "https://example.org/#session",
            "https://example.org:0/",
            "https:///",
            "https://example.org/%0D",
        ] {
            assert!(tenant_url(url).is_err(), "{url}");
        }
        let addr = "https://technician.example.org:8443/helpdesk";
        assert!(command(&c, &["configure", "meshcentral", addr]).is_err());
        assert!(command(&c, &["configure", "dism", addr, "--approve"]).is_err());
        command(&c, &["configure", "meshcentral", addr, "--approve"]).unwrap();
        let route = command(&c, &["actions", "meshcentral"]).unwrap();
        assert_eq!(route["launch"]["detail"], addr);
        assert_eq!(route["launch"]["available"], true);
        assert_eq!(
            command(&c, &["actions", "screenconnect"]).unwrap()["launch"]["available"],
            false
        );
    }

    #[test]
    fn catalog_ids_and_routes_are_closed() {
        let entries: Vec<Entry> =
            serde_json::from_str(include_str!("../../../data/tool-actions.json")).unwrap();
        let catalog: Vec<Catalog> =
            serde_json::from_str(include_str!(concat!(env!("OUT_DIR"), "/tools.json"))).unwrap();
        let ids = entries
            .iter()
            .map(|v| &v.id)
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(ids.len(), entries.len());
        assert!(ids.is_subset(&catalog.iter().map(|v| &v.id).collect()));
        for e in entries {
            if let Some(s) = e.winget {
                assert!(package_id(&s));
            }
            if let Some(s) = e.web {
                link(&s).unwrap();
            }
        }
        for invalid in ["--id", "a b", "A&B", "A/../../B", ""] {
            assert!(!package_id(invalid));
        }
        for invalid in [
            "http://vendor.test",
            "https://trusted.test@file.invalid",
            "file:///C:/abc",
            "https://a.test/ \r--run",
        ] {
            assert!(link(invalid).is_err());
        }
        assert!(route("unknown-id").is_err());
    }

    #[test]
    fn binding_requires_consent_an_allowed_name_and_unmodified_bytes() {
        let c = Connection::open_in_memory().unwrap();
        c.execute_batch("CREATE TABLE metadata(key TEXT PRIMARY KEY,value TEXT NOT NULL);CREATE TABLE events(id INTEGER PRIMARY KEY,at TEXT NOT NULL,operation TEXT NOT NULL,state TEXT NOT NULL);").unwrap();
        let tmp = tempfile::tempdir().unwrap();
        let p = tmp.path().join("clamscan.exe");
        std::fs::copy(host::windows().unwrap().join("System32/where.exe"), &p).unwrap();
        let path = p.to_str().unwrap();
        assert!(command(&c, &["bind", "clamav", path]).is_err());
        assert!(command(&c, &["bind", "putty", path, "--approve"]).is_err());
        command(&c, &["bind", "clamav", path, "--approve"]).unwrap();
        std::fs::OpenOptions::new()
            .append(true)
            .open(&p)
            .unwrap()
            .write_all(b"modified")
            .unwrap();
        let failure = command(&c, &["launch", "clamav", "--approve"])
            .unwrap_err()
            .to_string();
        assert!(failure.contains("changed"), "{failure}");
        assert!(
            command(
                &c,
                &["install", "clamav", "--approve", "--override", "test"]
            )
            .is_err()
        );
    }
}
