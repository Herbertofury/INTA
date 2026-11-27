use crate::server::valid_host;
use std::{
    fs, io,
    path::{Component, Path, PathBuf},
    process::Command,
};
use tiny_http::{Header, Response, Server, StatusCode};

pub const HOST_PORT: u16 = 5173;

/// A portable local browser host, shared by Desktop launch and Rescue. This is not a native
/// mobile client: it serves the actual bundled app without inventing account/device sync.
pub fn bind(directory: PathBuf) -> io::Result<(Server, PathBuf)> {
    let root = directory.canonicalize()?;
    if !root.join("index.html").is_file() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "The app directory must contain index.html.",
        ));
    }
    let server = Server::http(("127.0.0.1", HOST_PORT)).map_err(io::Error::other)?;
    Ok((server, root))
}

pub fn serve(server: Server, root: PathBuf) -> io::Result<()> {
    println!("INTA workspace: http://127.0.0.1:{HOST_PORT}/ (bundled browser app)");
    for request in server.incoming_requests() {
        let host = request
            .headers()
            .iter()
            .find(|h| h.field.equiv("Host"))
            .map(|h| h.value.as_str())
            .unwrap_or("");
        let allowed = valid_host(host, HOST_PORT)
            && request.remote_addr().is_some_and(|a| a.ip().is_loopback());
        let method = request.method().as_str();
        let path = request.url().split('?').next().unwrap_or("/");
        let resolved = resolve_file(&root, path);
        if !allowed {
            let _ = request.respond(Response::empty(StatusCode(403)));
            continue;
        }
        if !matches!(method, "GET" | "HEAD") {
            let _ = request.respond(Response::empty(StatusCode(405)));
            continue;
        }
        let Some(path) = resolved else {
            let _ = request.respond(Response::empty(StatusCode(404)));
            continue;
        };
        let content_type = match path.extension().and_then(|s| s.to_str()).unwrap_or("") {
            "html" => "text/html; charset=utf-8",
            "css" => "text/css; charset=utf-8",
            "js" => "text/javascript; charset=utf-8",
            "json" => "application/json",
            "wasm" => "application/wasm",
            "svg" => "image/svg+xml",
            "png" => "image/png",
            "ico" => "image/x-icon",
            "webmanifest" => "application/manifest+json",
            "jpg" | "jpeg" => "image/jpeg",
            _ => "application/octet-stream",
        };
        match fs::read(path) {
            Ok(bytes) => {
                let mut response =
                    Response::from_data(if method == "HEAD" { Vec::new() } else { bytes });
                for (name, value) in [
                    ("Content-Type", content_type),
                    ("Cache-Control", "no-cache"),
                    ("X-Content-Type-Options", "nosniff"),
                    ("Referrer-Policy", "no-referrer"),
                ] {
                    response.add_header(Header::from_bytes(name, value).unwrap());
                }
                let _ = request.respond(response);
            }
            Err(_) => {
                let _ = request.respond(Response::empty(StatusCode(404)));
            }
        }
    }
    Ok(())
}

pub fn resolve_file(root: &Path, request: &str) -> Option<PathBuf> {
    let request = if request == "/" {
        "index.html"
    } else {
        request.strip_prefix('/')?
    };
    // The bundled app uses plain static filenames. Encoded traversal and hidden files are rejected.
    if request.contains('%')
        || request.contains('\\')
        || request.split('/').any(|s| s.starts_with('.'))
    {
        return None;
    }
    let relative = Path::new(request);
    if relative
        .components()
        .any(|part| !matches!(part, Component::Normal(_)))
    {
        return None;
    }
    let path = root.join(relative).canonicalize().ok()?;
    if !path.starts_with(root) || !path.is_file() {
        return None;
    }
    if !matches!(
        path.extension().and_then(|s| s.to_str()),
        Some(
            "html"
                | "css"
                | "js"
                | "json"
                | "wasm"
                | "svg"
                | "png"
                | "ico"
                | "webmanifest"
                | "jpg"
                | "jpeg"
        )
    ) {
        return None;
    }
    Some(path)
}

pub fn open_browser() -> io::Result<()> {
    let url = format!("http://127.0.0.1:{HOST_PORT}/");
    #[cfg(windows)]
    {
        Command::new("rundll32.exe")
            .args(["url.dll,FileProtocolHandler", &url])
            .spawn()?;
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg(&url).spawn()?;
    }
    #[cfg(all(not(windows), not(target_os = "macos")))]
    {
        Command::new("xdg-open").arg(&url).spawn()?;
    }
    Ok(())
}
