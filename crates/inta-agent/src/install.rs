use std::{env, fs, io, path::PathBuf};

/// Installs this already-verified executable for the current user. It never requests elevation.
/// Package checksums remain release artifacts and must be verified before running a download.
pub fn install_current_user() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let current = env::current_exe()?;
    #[cfg(windows)]
    let directory =
        PathBuf::from(env::var_os("LOCALAPPDATA").ok_or("LOCALAPPDATA is unavailable")?)
            .join("Programs")
            .join("INTA")
            .join("Agent");
    #[cfg(not(windows))]
    let directory = PathBuf::from(env::var_os("HOME").ok_or("HOME is unavailable")?)
        .join(".local")
        .join("bin");
    fs::create_dir_all(&directory)?;
    let destination = directory.join(if cfg!(windows) {
        "inta-agent.exe"
    } else {
        "inta-agent"
    });
    if current.canonicalize()? != destination.canonicalize().unwrap_or_default() {
        let temporary = directory.join(format!(".inta-agent-{}.tmp", crate::random_id(8)));
        let result: io::Result<()> = (|| {
            fs::copy(&current, &temporary)?;
            fs::rename(&temporary, &destination)?;
            Ok(())
        })();
        if result.is_err() {
            let _ = fs::remove_file(&temporary);
        }
        result?;
    }
    #[cfg(windows)]
    create_shortcuts(&destination)?;
    Ok(destination)
}

#[cfg(windows)]
fn create_shortcuts(executable: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    use windows::{
        core::{Interface, HSTRING},
        Win32::{
            System::Com::{
                CoCreateInstance, CoInitializeEx, CoUninitialize, IPersistFile,
                CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED,
            },
            UI::Shell::{IShellLinkDataList, IShellLinkW, ShellLink, SLDF_RUNAS_USER},
        },
    };
    let directory = PathBuf::from(env::var_os("APPDATA").ok_or("APPDATA is unavailable")?)
        .join("Microsoft")
        .join("Windows")
        .join("Start Menu")
        .join("Programs")
        .join("INTA");
    fs::create_dir_all(&directory)?;
    // COM objects are scoped and dropped before balancing initialization.
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()?;
        let result: Result<(), windows::core::Error> = (|| {
            for (name, elevated) in [
                ("INTA Local Agent.lnk", false),
                ("INTA Local Agent (Administrator).lnk", true),
            ] {
                let link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)?;
                link.SetPath(&HSTRING::from(executable.as_os_str()))?;
                link.SetWorkingDirectory(&HSTRING::from(executable.parent().unwrap().as_os_str()))?;
                link.SetDescription(&HSTRING::from(
                    "Start the INTA Rust local maintenance agent",
                ))?;
                if elevated {
                    let flags: IShellLinkDataList = link.cast()?;
                    flags.SetFlags(flags.GetFlags()? | SLDF_RUNAS_USER.0 as u32)?;
                }
                let persisted: IPersistFile = link.cast()?;
                persisted.Save(&HSTRING::from(directory.join(name).as_os_str()), true)?;
            }
            Ok(())
        })();
        CoUninitialize();
        result?;
    }
    Ok(())
}
