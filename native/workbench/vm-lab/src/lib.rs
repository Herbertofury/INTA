#[cfg(windows)]
mod windows;
use serde_json::Value;
use std::path::Path;

pub fn rust_lab_enabled() -> bool {
    #[cfg(windows)]
    {
        windows::rust_lab_enabled()
    }
    #[cfg(not(windows))]
    {
        false
    }
}

/// Operate a private local UEFI lab using verified QEMU and session-owned storage.
pub fn command(toolbox: &Path, args: &[&str]) -> Result<Value, String> {
    #[cfg(windows)]
    {
        windows::command(toolbox, args)
    }
    #[cfg(not(windows))]
    {
        let _ = (toolbox, args);
        Err("The native UEFI lab manager requires Windows x64".into())
    }
}
