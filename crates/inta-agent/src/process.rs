use crate::models::{CommandSpec, ProcessResult};
use std::{
    io::{self, Read},
    process::{Command, Stdio},
    sync::{Arc, Mutex},
    thread,
};

pub const OUTPUT_LIMIT: usize = 64_000;

/// Launches an exact executable plus argument vector. No API request supplies a shell command.
/// Running installers are never killed because a browser disconnects or stops observing.
pub fn run(spec: &CommandSpec) -> io::Result<ProcessResult> {
    let mut command = Command::new(&spec.program);
    command
        .args(&spec.arguments)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }
    let mut child = command.spawn().map_err(|e| {
        io::Error::new(
            e.kind(),
            format!(
                "Could not launch {}: {}. Check the selected manager and current privilege level.",
                spec.program, e
            ),
        )
    })?;
    let captured = Arc::new(Mutex::new(Vec::new()));
    let out = reader(child.stdout.take().expect("piped stdout"), captured.clone());
    let err = reader(child.stderr.take().expect("piped stderr"), captured.clone());
    let status = child.wait()?;
    let _ = out.join();
    let _ = err.join();
    let bytes = captured.lock().unwrap();
    let mut output = String::from_utf8_lossy(&bytes).into_owned();
    if bytes.len() == OUTPUT_LIMIT {
        output.push_str("\n[INTA output limit reached; inspect the manager's full logs.]");
    }
    Ok(ProcessResult {
        exit_code: status.code().unwrap_or(-1),
        output,
    })
}

fn reader(
    mut pipe: impl Read + Send + 'static,
    output: Arc<Mutex<Vec<u8>>>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut buffer = [0; 4096];
        loop {
            match pipe.read(&mut buffer) {
                Ok(0) | Err(_) => break,
                Ok(n) => {
                    let mut output = output.lock().unwrap();
                    let remaining = OUTPUT_LIMIT.saturating_sub(output.len());
                    output.extend_from_slice(&buffer[..n.min(remaining)]);
                    // Continue draining both pipes after truncation to avoid blocking the child.
                }
            }
        }
    })
}
