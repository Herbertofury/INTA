use inta_agent::{
    host,
    server::{self, AppState, HostBackend},
    PORT, VERSION,
};
use std::{env, io, net::SocketAddr, path::PathBuf, sync::Arc, thread};

fn data_directory() -> PathBuf {
    if let Some(path) = env::var_os("INTA_DATA_DIR") {
        return PathBuf::from(path).join("agent");
    }
    if cfg!(windows) {
        return PathBuf::from(env::var_os("LOCALAPPDATA").unwrap_or_else(|| ".".into()))
            .join("INTA")
            .join("agent");
    }
    if let Some(path) = env::var_os("XDG_DATA_HOME") {
        return PathBuf::from(path).join("INTA").join("agent");
    }
    PathBuf::from(env::var_os("HOME").unwrap_or_else(|| ".".into()))
        .join(".local")
        .join("share")
        .join("INTA")
        .join("agent")
}

fn main() -> io::Result<()> {
    let mut args = env::args().skip(1);
    let mut directory = data_directory();
    let mut site = None;
    let mut open = false;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--install" => {
                let path = inta_agent::install::install_current_user()
                    .map_err(|e| io::Error::other(e.to_string()))?;
                println!(
                    "Installed INTA Local Agent to {}. Start it explicitly to connect.",
                    path.display()
                );
                return Ok(());
            }
            "--version" => {
                println!("INTA Rust Local Agent {VERSION}");
                return Ok(());
            }
            "--help" | "-h" => {
                println!("INTA Rust Local Agent {VERSION}\n\nUsage: inta-agent [--data-dir PATH] [--serve-site PATH] [--open] | --install\n\nRuns the local maintenance bridge on loopback port {PORT}.\n--install          Install this verified binary for the current user.\n--data-dir PATH    Keep history in a chosen portable directory.\n--serve-site PATH  Serve the bundled Site in the local browser on port 5173.\n--open             Open that browser workspace after startup.\n\nStart and connect explicitly. No background inventory, update polling, or automatic package changes.\nStop the agent only after installers finish; stopping observation is not installer cancellation.");
                return Ok(());
            }
            "--data-dir" => {
                directory = PathBuf::from(args.next().ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "--data-dir requires a path")
                })?)
            }
            "--serve-site" => {
                site = Some(PathBuf::from(args.next().ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "--serve-site requires a path")
                })?))
            }
            "--open" => open = true,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Unknown argument: {arg}"),
                ))
            }
        }
    }
    if open && site.is_none() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "--open requires --serve-site PATH",
        ));
    }
    let state = Arc::new(AppState::new(&directory, Arc::new(HostBackend))?);
    let ipv4 = server::bind(SocketAddr::from(([127, 0, 0, 1], PORT)))?;
    match server::bind(format!("[::1]:{PORT}").parse().unwrap()) {
        Ok(ipv6) => {
            let state = state.clone();
            thread::spawn(move || server::serve(ipv6, state));
        }
        Err(error) => eprintln!("IPv6 loopback unavailable; IPv4 remains active: {error}"),
    }
    if let Some(path) = site {
        let (site_server, path) = host::bind(path)?;
        thread::spawn(move || {
            if let Err(error) = host::serve(site_server, path) {
                eprintln!("Could not start the bundled workspace: {error}");
            }
        });
        if open {
            if let Err(error) = host::open_browser() {
                eprintln!("Open http://127.0.0.1:5173/ in your browser: {error}");
            }
        }
    }
    println!("INTA Rust Local Agent {VERSION} — loopback port {PORT}");
    println!("Open INTA → Updates & Maintenance → Connect this computer.");
    server::serve(ipv4, state);
    Ok(())
}
