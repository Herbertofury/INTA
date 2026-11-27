use crate::{
    models::{CommandSpec, ManagerPreview, OperationRequest, ProcessResult, Verification},
    process,
};
use chrono::Utc;
use std::{env, path::PathBuf};

pub fn validate_id(value: &str) -> Result<&str, String> {
    if value.is_empty()
        || value.len() > 180
        || !value.as_bytes()[0].is_ascii_alphanumeric()
        || !value
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"._+@:/-".contains(&c))
    {
        return Err("Enter the exact package ID from the selected manager. Shell syntax and command-line switches are not accepted.".into());
    }
    Ok(value)
}

pub fn normalize_hotfix(value: &str) -> Result<String, String> {
    let digits = if value.get(..2).is_some_and(|s| s.eq_ignore_ascii_case("kb")) {
        &value[2..]
    } else {
        value
    };
    if !(6..=8).contains(&digits.len()) || !digits.bytes().all(|c| c.is_ascii_digit()) {
        return Err(
            "Windows Update rollback requires an exact KB number, such as KB5035845.".into(),
        );
    }
    Ok(format!("KB{digits}"))
}

pub fn find_on_path(name: &str) -> Option<PathBuf> {
    env::split_paths(&env::var_os("PATH").unwrap_or_default())
        .filter(|p| !p.as_os_str().is_empty())
        .map(|p| p.join(name))
        .find(|p| p.is_file())
}

pub fn detect() -> Vec<String> {
    let candidates: &[&str] = if cfg!(windows) {
        &["winget", "choco", "scoop", "uniget"]
    } else if cfg!(target_os = "macos") {
        &["brew", "flatpak"]
    } else {
        &["apt", "dnf", "pacman", "flatpak", "snap"]
    };
    let mut detected: Vec<String> = candidates
        .iter()
        .filter(|name| available(name))
        .map(|s| (*s).into())
        .collect();
    if cfg!(windows) {
        detected.push("windows-update".into());
    }
    detected
}

fn available(name: &str) -> bool {
    if cfg!(windows) {
        if name == "scoop" {
            return find_on_path("scoop.ps1").is_some();
        }
        return find_on_path(&format!("{name}.exe")).is_some();
    }
    find_on_path(name).is_some()
}

pub fn validate(request: &OperationRequest, installed: &[String]) -> Result<(), String> {
    if !request.confirmed {
        return Err(
            "This action changes the computer. Confirm once in INTA before continuing.".into(),
        );
    }
    if !installed.contains(&request.manager) {
        return Err("That package manager is not installed on this computer.".into());
    }
    if !matches!(
        request.action.as_str(),
        "update" | "update-all" | "repair" | "uninstall" | "rollback"
    ) {
        return Err("Unsupported operation.".into());
    }
    // Construct the complete plan before accepting the operation: no deferred validation after queueing.
    let _ = plan(request)?;
    Ok(())
}

pub fn preview_command(manager: &str) -> Result<CommandSpec, String> {
    let command = match manager {
        "windows-update" => CommandSpec::new("powershell.exe", &["-NoProfile", "-NonInteractive", "-Command", "Get-HotFix | Sort-Object InstalledOn -Descending | Select-Object -First 40 HotFixID,Description,InstalledOn | Format-Table -AutoSize"]),
        "winget" => CommandSpec::new("winget", &["upgrade", "--accept-source-agreements", "--disable-interactivity"]),
        "choco" => CommandSpec::new("choco", &["outdated", "--limit-output"]),
        "scoop" => CommandSpec::new("scoop", &["status"]),
        "uniget" => CommandSpec::new("uniget.exe", &["package", "updates", "--manager", "winget"]),
        "apt" => CommandSpec::new("apt", &["list", "--upgradable"]),
        "dnf" => CommandSpec::new("dnf", &["check-update"]),
        "pacman" => CommandSpec::new("checkupdates", &[]),
        "flatpak" => CommandSpec::new("flatpak", &["remote-ls", "--updates"]),
        "snap" => CommandSpec::new("snap", &["refresh", "--list"]),
        "brew" => CommandSpec::new("brew", &["outdated", "--verbose"]),
        _ => return Err("Unsupported package manager.".into()),
    };
    Ok(command)
}

pub fn resolve(mut spec: CommandSpec) -> Result<CommandSpec, String> {
    if cfg!(windows) && spec.program == "scoop" {
        let script = find_on_path("scoop.ps1").ok_or("Scoop's PowerShell shim was not found on PATH. Repair Scoop using its official installer.")?;
        let mut arguments = vec![
            "-NoProfile".into(),
            "-NonInteractive".into(),
            "-File".into(),
            script.to_string_lossy().into_owned(),
        ];
        arguments.extend(spec.arguments);
        spec = CommandSpec {
            program: "powershell.exe".into(),
            arguments,
        };
    }
    Ok(spec)
}

pub fn previews(managers: &[String]) -> Vec<ManagerPreview> {
    managers
        .iter()
        .map(|manager| {
            let spec = preview_command(manager).and_then(resolve);
            let (command, result) = match spec {
                Ok(spec) => (
                    spec.display(),
                    process::run(&spec).map_err(|e| e.to_string()),
                ),
                Err(error) => (manager.to_string(), Err(error)),
            };
            let result = result.unwrap_or_else(|output| ProcessResult {
                exit_code: -1,
                output,
            });
            ManagerPreview {
                manager: manager.clone(),
                command,
                exit_code: result.exit_code,
                output: result.output,
                checked_at: Utc::now(),
            }
        })
        .collect()
}

pub fn plan(request: &OperationRequest) -> Result<Vec<CommandSpec>, String> {
    let m = request.manager.as_str();
    let a = request.action.as_str();
    let p = if a == "update-all" || (m == "windows-update" && a == "repair") {
        "*"
    } else {
        validate_id(&request.package_id)?
    };
    let mut result = Vec::new();
    if a == "update" || a == "update-all" {
        match m {
            "apt" => result.push(CommandSpec::new("apt-get", &["update"])),
            "brew" => result.push(CommandSpec::new("brew", &["update"])),
            "scoop" => result.push(CommandSpec::new("scoop", &["update"])),
            _ => {}
        }
    }
    let command = match (m, a) {
        ("windows-update", "repair") => {
            // State and exit-code control stays in Rust; no PowerShell orchestration script.
            return Ok(vec![CommandSpec::new("DISM.exe", &["/Online", "/Cleanup-Image", "/RestoreHealth"]), CommandSpec::new("sfc.exe", &["/scannow"])]);
        },
        ("windows-update", "uninstall" | "rollback") => CommandSpec::new("wusa.exe", &["/uninstall", &format!("/kb:{}", &normalize_hotfix(p)?[2..]), "/quiet", "/norestart"]),
        ("windows-update", _) => return Err("Use Windows Update Settings to check and install OS updates. INTA can inspect, repair servicing files and uninstall an exact KB.".into()),
        ("winget", "update-all") => CommandSpec::new("winget", &["upgrade", "--all", "--silent", "--accept-source-agreements", "--accept-package-agreements"]),
        ("winget", "update") => CommandSpec::new("winget", &["upgrade", "--id", p, "--exact", "--silent", "--accept-source-agreements", "--accept-package-agreements"]),
        ("winget", "repair") => CommandSpec::new("winget", &["repair", "--id", p, "--exact", "--silent"]),
        ("winget", "uninstall") => CommandSpec::new("winget", &["uninstall", "--id", p, "--exact", "--silent"]),
        ("choco", "update-all") => CommandSpec::new("choco", &["upgrade", "all", "--yes"]),
        ("choco", "update") => CommandSpec::new("choco", &["upgrade", p, "--yes"]),
        ("choco", "repair") => CommandSpec::new("choco", &["install", p, "--force", "--yes"]),
        ("choco", "uninstall") => CommandSpec::new("choco", &["uninstall", p, "--confirm"]),
        ("scoop", "update-all") => CommandSpec::new("scoop", &["update", "*"]),
        ("scoop", "update") => CommandSpec::new("scoop", &["update", p]),
        ("scoop", "repair") => CommandSpec::new("scoop", &["reinstall", p]),
        ("scoop", "uninstall") => CommandSpec::new("scoop", &["uninstall", p]),
        ("apt", "update-all") => CommandSpec::new("apt-get", &["upgrade", "-y"]),
        ("apt", "update") => CommandSpec::new("apt-get", &["install", "--only-upgrade", "-y", p]),
        ("apt", "repair") => CommandSpec::new("apt-get", &["install", "--reinstall", "-y", p]),
        ("apt", "uninstall") => CommandSpec::new("apt-get", &["remove", "-y", p]),
        ("dnf", "update-all") => CommandSpec::new("dnf", &["upgrade", "-y"]),
        ("dnf", "update") => CommandSpec::new("dnf", &["upgrade", "-y", p]),
        ("dnf", "repair") => CommandSpec::new("dnf", &["reinstall", "-y", p]),
        ("dnf", "uninstall") => CommandSpec::new("dnf", &["remove", "-y", p]),
        ("pacman", "update-all") => CommandSpec::new("pacman", &["-Syu", "--noconfirm"]),
        ("pacman", "update") => CommandSpec::new("pacman", &["-S", "--needed", "--noconfirm", p]),
        ("pacman", "repair") => CommandSpec::new("pacman", &["-S", "--noconfirm", p]),
        ("pacman", "uninstall") => CommandSpec::new("pacman", &["-R", "--noconfirm", p]),
        ("brew", "update-all") => CommandSpec::new("brew", &["upgrade"]),
        ("brew", "update") => CommandSpec::new("brew", &["upgrade", p]),
        ("brew", "repair") => CommandSpec::new("brew", &["reinstall", p]),
        ("brew", "uninstall") => CommandSpec::new("brew", &["uninstall", p]),
        ("flatpak", "update-all") => CommandSpec::new("flatpak", &["update", "-y"]),
        ("flatpak", "update") => CommandSpec::new("flatpak", &["update", "-y", p]),
        ("flatpak", "repair") => CommandSpec::new("flatpak", &["repair"]),
        ("flatpak", "uninstall") => CommandSpec::new("flatpak", &["uninstall", "-y", p]),
        ("snap", "update-all") => CommandSpec::new("snap", &["refresh"]),
        ("snap", "update") => CommandSpec::new("snap", &["refresh", p]),
        ("snap", "rollback") => CommandSpec::new("snap", &["revert", p]),
        ("snap", "uninstall") => CommandSpec::new("snap", &["remove", p]),
        ("uniget", "update-all") => CommandSpec::new("uniget.exe", &["package", "update-all", "--wait", "true", "--timeout", "3600"]),
        _ => return Err(format!("The {m} manager does not support {a}.")),
    };
    result.push(command);
    Ok(result)
}

pub fn execute(request: &OperationRequest) -> Result<ProcessResult, String> {
    let mut output = String::new();
    let mut exit_code = 0;
    for spec in plan(request)? {
        let spec = resolve(spec)?;
        output.push_str(&format!("===== {} =====\n", spec.display()));
        let result = process::run(&spec).map_err(|e| e.to_string())?;
        output.push_str(&result.output);
        output.push('\n');
        exit_code = result.exit_code;
        if exit_code != 0 {
            break;
        }
    }
    Ok(ProcessResult { exit_code, output })
}

pub fn verification_command(manager: &str, package: &str) -> Result<CommandSpec, String> {
    validate_id(package)?;
    Ok(match manager {
        "windows-update" => CommandSpec::new(
            "powershell.exe",
            &[
                "-NoProfile",
                "-NonInteractive",
                "-Command",
                &format!(
                    "Get-HotFix -Id {} -ErrorAction Stop",
                    normalize_hotfix(package)?
                ),
            ],
        ),
        "winget" => CommandSpec::new(
            "winget",
            &[
                "list",
                "--id",
                package,
                "--exact",
                "--disable-interactivity",
            ],
        ),
        "choco" => CommandSpec::new("choco", &["list", "--limit-output", "--exact", package]),
        "scoop" => CommandSpec::new("scoop", &["list", package]),
        "apt" => CommandSpec::new(
            "dpkg-query",
            &[
                "-W",
                "-f=${Package}\t${Version}\t${db:Status-Abbrev}\n",
                package,
            ],
        ),
        "dnf" => CommandSpec::new("rpm", &["-q", package]),
        "pacman" => CommandSpec::new("pacman", &["-Q", package]),
        "brew" => CommandSpec::new("brew", &["list", "--versions", package]),
        "flatpak" => CommandSpec::new("flatpak", &["info", package]),
        "snap" => CommandSpec::new("snap", &["list", package]),
        _ => return Err("This manager has no definitive package query.".into()),
    })
}

/// Verification stays conservative: command exit success alone is not repaired software.
/// APT exposes an unambiguous installed-package status plus an update listing. Other output
/// formats are retained for review until their exact semantics have integration evidence.
pub fn evaluate_apt_update(
    package: &str,
    installed: &ProcessResult,
    preview: &ManagerPreview,
) -> bool {
    if installed.exit_code != 0
        || preview.exit_code != 0
        || installed.output.contains("[INTA output limit")
        || preview.output.contains("[INTA output limit")
    {
        return false;
    }
    let present = installed.output.lines().any(|line| {
        let fields: Vec<&str> = line.split('\t').collect();
        fields.len() == 3
            && fields[0] == package
            && !fields[1].is_empty()
            && fields[2].starts_with("ii")
    });
    let still_pending = preview
        .output
        .lines()
        .any(|line| line.split('/').next() == Some(package));
    present && !still_pending
}

pub fn verify(request: &OperationRequest) -> Verification {
    if request.manager == "windows-update" && request.action == "repair" {
        return Verification { verified: false, output: "DISM and SFC exited. Review servicing output and test the original fault after any required restart.".into() };
    }
    let mut output = String::new();
    let mut installed = ProcessResult {
        exit_code: -1,
        output: String::new(),
    };
    if request.action != "update-all" {
        match verification_command(&request.manager, &request.package_id)
            .and_then(resolve)
            .and_then(|c| process::run(&c).map_err(|e| e.to_string()))
        {
            Ok(result) => {
                output.push_str(&result.output);
                installed = result;
            }
            Err(error) => output.push_str(&error),
        }
    }
    let mut verified = false;
    if request.action == "update" || request.action == "update-all" {
        if let Some(preview) = previews(std::slice::from_ref(&request.manager)).first() {
            output.push_str("\n===== Current update preview =====\n");
            output.push_str(&preview.output);
            if request.action == "update" && request.manager == "apt" {
                verified = evaluate_apt_update(&request.package_id, &installed, preview);
            }
        }
    }
    if !verified {
        output.push_str("\nReview the post-check and verify the original task. A successful exit is not proof of a repair, uninstall or rollback.");
    }
    Verification { verified, output }
}
