use crate::{
    managers,
    models::{InstalledApp, InventoryResult},
};
use chrono::Utc;

pub fn platform() -> String {
    format!("{} {}", std::env::consts::OS, std::env::consts::ARCH)
}

pub fn collect() -> InventoryResult {
    let detected = managers::detect();
    let applications = installed_apps();
    let update_previews = managers::previews(&detected);
    InventoryResult {
        collected_at: Utc::now(),
        platform: platform(),
        applications,
        managers: detected,
        update_previews,
    }
}

#[cfg(not(windows))]
fn installed_apps() -> Vec<InstalledApp> {
    Vec::new()
}

#[cfg(windows)]
fn installed_apps() -> Vec<InstalledApp> {
    use winreg::{enums::*, RegKey};
    let mut rows = Vec::new();
    let roots = [
        (HKEY_LOCAL_MACHINE, KEY_WOW64_64KEY, "Machine"),
        (HKEY_LOCAL_MACHINE, KEY_WOW64_32KEY, "Machine"),
        (HKEY_CURRENT_USER, 0, "User"),
    ];
    for (root, view, scope) in roots {
        let key = RegKey::predef(root).open_subkey_with_flags(
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
            KEY_READ | view,
        );
        if let Ok(key) = key {
            for name in key.enum_keys().flatten() {
                if let Ok(app) = key.open_subkey(&name) {
                    let display: Result<String, _> = app.get_value("DisplayName");
                    if let Ok(name_display) = display {
                        if name_display.trim().is_empty() {
                            continue;
                        }
                        rows.push(InstalledApp {
                            name: name_display,
                            version: app.get_value("DisplayVersion").ok(),
                            publisher: app.get_value("Publisher").ok(),
                            manager: "registry".into(),
                            package_id: Some(name),
                            scope: scope.into(),
                        });
                    }
                }
            }
        }
    }
    rows.sort_by(|a, b| {
        (a.name.to_lowercase(), &a.version, &a.publisher, &a.scope).cmp(&(
            b.name.to_lowercase(),
            &b.version,
            &b.publisher,
            &b.scope,
        ))
    });
    rows.dedup_by(|a, b| {
        a.name == b.name
            && a.version == b.version
            && a.publisher == b.publisher
            && a.scope == b.scope
    });
    rows
}

#[cfg(windows)]
pub fn pending_reboot() -> bool {
    use winreg::{enums::*, RegKey};
    RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Component Based Servicing\\RebootPending").is_ok()
}
#[cfg(not(windows))]
pub fn pending_reboot() -> bool {
    std::path::Path::new("/var/run/reboot-required").exists()
}

pub fn elevated() -> bool {
    #[cfg(windows)]
    {
        use crate::{models::CommandSpec, process};
        return process::run(&CommandSpec::new("whoami.exe", &["/groups"]))
            .is_ok_and(|r| r.exit_code == 0 && r.output.contains("S-1-16-12288"));
    }
    #[cfg(not(windows))]
    {
        use crate::{models::CommandSpec, process};
        process::run(&CommandSpec::new("id", &["-u"]))
            .is_ok_and(|r| r.exit_code == 0 && r.output.trim() == "0")
    }
}

pub fn hostname() -> String {
    std::env::var("COMPUTERNAME")
        .or_else(|_| std::env::var("HOSTNAME"))
        .unwrap_or_else(|_| {
            std::fs::read_to_string("/proc/sys/kernel/hostname")
                .unwrap_or_else(|_| "This computer".into())
                .trim()
                .into()
        })
}
