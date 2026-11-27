use chrono::Utc;
use inta_agent::{
    host, managers,
    models::*,
    process, random_id,
    server::{self, AppState, Backend, RequestContext},
    store::OperationStore,
    PORT,
};
use serde_json::{json, Value};
use std::{
    fs,
    io::{Read, Write},
    net::TcpStream,
    path::PathBuf,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!("inta-rust-agent-test-{}", random_id(10)));
        fs::create_dir_all(&p).unwrap();
        Self(p)
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

struct FakeBackend {
    active: AtomicUsize,
    max_active: AtomicUsize,
    executed: AtomicUsize,
}
impl FakeBackend {
    fn new() -> Self {
        Self {
            active: AtomicUsize::new(0),
            max_active: AtomicUsize::new(0),
            executed: AtomicUsize::new(0),
        }
    }
}
impl Backend for FakeBackend {
    fn managers(&self) -> Vec<String> {
        vec!["apt".into(), "windows-update".into()]
    }
    fn inventory(&self) -> InventoryResult {
        InventoryResult {
            collected_at: Utc::now(),
            platform: "fixture".into(),
            applications: vec![],
            managers: self.managers(),
            update_previews: vec![],
        }
    }
    fn execute(&self, _: &OperationRequest) -> Result<ProcessResult, String> {
        let active = self.active.fetch_add(1, Ordering::SeqCst) + 1;
        self.max_active.fetch_max(active, Ordering::SeqCst);
        thread::sleep(Duration::from_millis(15));
        self.active.fetch_sub(1, Ordering::SeqCst);
        self.executed.fetch_add(1, Ordering::SeqCst);
        Ok(ProcessResult {
            exit_code: 0,
            output: "Fixture: no package manager invoked.".into(),
        })
    }
    fn verify(&self, _: &OperationRequest) -> Verification {
        Verification {
            verified: false,
            output: "Fixture requires review.".into(),
        }
    }
    fn pending_reboot(&self) -> bool {
        false
    }
}
fn state(scratch: &Scratch) -> (Arc<AppState>, Arc<FakeBackend>) {
    let backend = Arc::new(FakeBackend::new());
    (
        Arc::new(AppState::new(&scratch.0, backend.clone()).unwrap()),
        backend,
    )
}
fn request(
    state: &Arc<AppState>,
    method: &str,
    path: &str,
    token: &str,
    body: Value,
) -> server::ApiResponse {
    let body = serde_json::to_vec(&body).unwrap();
    server::handle(
        state,
        &RequestContext {
            remote: "127.0.0.1".parse().unwrap(),
            method,
            path,
            host: "127.0.0.1:47821",
            origin: "https://herbertofury.github.io",
            authorization: token,
            body: &body,
        },
    )
}
fn pair(state: &Arc<AppState>) -> String {
    let response = request(state, "POST", "/api/v1/pair", "", Value::Null);
    assert_eq!(response.status, 200);
    format!("Bearer {}", response.value["token"].as_str().unwrap())
}
fn operation(manager: &str, action: &str, package: &str) -> OperationRequest {
    OperationRequest {
        manager: manager.into(),
        action: action.into(),
        package_id: package.into(),
        confirmed: true,
    }
}

#[test]
fn rejects_shell_switches_and_malformed_package_ids() {
    for id in [
        "",
        "--all",
        "-y",
        "$(touch nope)",
        "x;y",
        "x\ny",
        "a b",
        "`id`",
        "x|y",
        "é",
        "a'",
        "x\\y",
        "<x>",
    ] {
        assert!(managers::validate_id(id).is_err(), "{id:?}");
    }
    assert!(managers::validate_id(&"a".repeat(181)).is_err());
    for id in [
        "Microsoft.PowerToys",
        "libc6:amd64",
        "org.example.App",
        "app/bucket",
        "name@version",
        "libstdc++",
    ] {
        assert!(managers::validate_id(id).is_ok());
    }
    assert_eq!(
        managers::normalize_hotfix("kb5035845").unwrap(),
        "KB5035845"
    );
    for id in ["KB123", "KB123456;id", "123456789", "KB-123456"] {
        assert!(managers::normalize_hotfix(id).is_err());
    }
}

#[test]
fn every_existing_manager_action_is_mapped_to_fixed_arguments() {
    let cases: &[(&str, &[&str])] = &[
        ("winget", &["update", "update-all", "repair", "uninstall"]),
        ("choco", &["update", "update-all", "repair", "uninstall"]),
        ("scoop", &["update", "update-all", "repair", "uninstall"]),
        ("apt", &["update", "update-all", "repair", "uninstall"]),
        ("dnf", &["update", "update-all", "repair", "uninstall"]),
        ("pacman", &["update", "update-all", "repair", "uninstall"]),
        ("brew", &["update", "update-all", "repair", "uninstall"]),
        ("flatpak", &["update", "update-all", "repair", "uninstall"]),
        ("snap", &["update", "update-all", "rollback", "uninstall"]),
        ("uniget", &["update-all"]),
        ("windows-update", &["repair", "uninstall", "rollback"]),
    ];
    let mut count = 0;
    for (manager, actions) in cases {
        assert!(managers::preview_command(manager).is_ok());
        for action in *actions {
            let plan = managers::plan(&operation(
                manager,
                action,
                if *manager == "windows-update" {
                    "KB5035845"
                } else {
                    "test-package"
                },
            ))
            .unwrap();
            assert!(!plan.is_empty());
            assert!(!plan
                .iter()
                .any(|c| c.program == "sh" || c.program == "cmd.exe"));
            count += 1;
        }
    }
    assert_eq!(count, 40);
    let repair = managers::plan(&operation("windows-update", "repair", "*")).unwrap();
    assert_eq!(
        repair
            .iter()
            .map(|c| c.program.as_str())
            .collect::<Vec<_>>(),
        ["DISM.exe", "sfc.exe"]
    );
    let apt = managers::plan(&operation("apt", "update", "test-package")).unwrap();
    assert_eq!(apt[0], CommandSpec::new("apt-get", &["update"]));
    assert_eq!(
        apt[1],
        CommandSpec::new(
            "apt-get",
            &["install", "--only-upgrade", "-y", "test-package"]
        )
    );
}

#[test]
fn confirmations_and_unsupported_actions_fail_before_execution() {
    let installed = vec![
        "apt".into(),
        "windows-update".into(),
        "snap".into(),
        "uniget".into(),
    ];
    let mut unconfirmed = operation("apt", "update", "test");
    unconfirmed.confirmed = false;
    assert!(managers::validate(&unconfirmed, &installed).is_err());
    for request in [
        operation("apt", "rollback", "x"),
        operation("windows-update", "update", "KB5035845"),
        operation("windows-update", "update-all", "*"),
        operation("snap", "repair", "x"),
        operation("uniget", "uninstall", "x"),
        operation("unknown", "update", "x"),
        operation("apt", "shell", "x"),
    ] {
        assert!(managers::validate(&request, &installed).is_err());
    }
}

#[test]
fn session_pairing_requires_loopback_host_and_exact_origin() {
    let scratch = Scratch::new();
    let (state, backend) = state(&scratch);
    let body = b"";
    for (remote, host, origin) in [
        (
            "10.0.0.1",
            "127.0.0.1:47821",
            "https://herbertofury.github.io",
        ),
        (
            "127.0.0.1",
            "evil.example:47821",
            "https://herbertofury.github.io",
        ),
        (
            "127.0.0.1",
            "127.0.0.1:47821",
            "https://herbertofury.github.io.evil.example",
        ),
        ("127.0.0.1", "127.0.0.1:47821", "null"),
        ("127.0.0.1", "127.0.0.1:47821", ""),
    ] {
        assert_eq!(
            server::handle(
                &state,
                &RequestContext {
                    remote: remote.parse().unwrap(),
                    method: "POST",
                    path: "/api/v1/pair",
                    host,
                    origin,
                    authorization: "",
                    body
                }
            )
            .status,
            403
        );
    }
    assert!(server::valid_host("[::1]:47821", PORT));
    assert!(!server::valid_host("127.0.0.1:47821@evil.example", PORT));
    assert_eq!(
        request(&state, "GET", "/api/v1/operations", "", Value::Null).status,
        401
    );
    let token = pair(&state);
    assert_eq!(
        request(&state, "GET", "/api/v1/operations", &token, Value::Null).status,
        200
    );
    assert_eq!(
        request(
            &state,
            "GET",
            "/api/v1/operations",
            &(token + "x"),
            Value::Null
        )
        .status,
        401
    );
    assert_eq!(backend.executed.load(Ordering::SeqCst), 0);
}

#[test]
fn invalid_batch_is_atomic_and_cannot_bypass_windows_update_boundary() {
    let scratch = Scratch::new();
    let (state, backend) = state(&scratch);
    let token = pair(&state);
    let response = request(
        &state,
        "POST",
        "/api/v1/operations/batch",
        &token,
        json!({"confirmed":true,"operations":[{"manager":"apt","action":"update","packageId":"safe"},{"manager":"windows-update","action":"update","packageId":"KB5035845"}]}),
    );
    assert_eq!(response.status, 400);
    assert!(state.store.recent().is_empty());
    assert_eq!(backend.executed.load(Ordering::SeqCst), 0);
}

#[test]
fn same_manager_serializes_and_persists_real_states() {
    let scratch = Scratch::new();
    let (state, backend) = state(&scratch);
    let token = pair(&state);
    let response = request(
        &state,
        "POST",
        "/api/v1/operations/batch",
        &token,
        json!({"confirmed":true,"operations":[{"manager":"apt","action":"update","packageId":"one"},{"manager":"apt","action":"update","packageId":"two"}]}),
    );
    assert_eq!(response.status, 202);
    for _ in 0..100 {
        if state.store.recent().iter().all(|r| r.finished_at.is_some()) {
            break;
        }
        thread::sleep(Duration::from_millis(10));
    }
    let rows = state.store.recent();
    assert_eq!(rows.len(), 2);
    assert!(rows.iter().all(|r| r.state == "completed-review"
        && r.finished_at.is_some()
        && r.exit_code == Some(0)));
    assert_eq!(backend.max_active.load(Ordering::SeqCst), 1);
    assert_eq!(backend.executed.load(Ordering::SeqCst), 2);
    let restored = OperationStore::open(&scratch.0).unwrap();
    assert_eq!(restored.recent().len(), 2);
    assert!(restored
        .recent()
        .iter()
        .all(|r| r.state == "completed-review"));
}

#[test]
fn restart_marks_active_legacy_history_interrupted_and_rotates_token() {
    let scratch = Scratch::new();
    let record = json!({"id":"legacy-csharp-record","manager":"winget","action":"update","packageId":"Microsoft.PowerToys","state":"running","startedAt":"2026-11-26T12:30:00+00:00","finishedAt":null,"exitCode":null,"output":"existing evidence","rebootLikely":false,"error":null});
    fs::write(scratch.0.join("operations.jsonl"), format!("{}\n", record)).unwrap();
    let (first, _) = state(&scratch);
    let first_token = pair(&first);
    let restored = first.store.get("legacy-csharp-record").unwrap();
    assert_eq!(restored.state, "interrupted");
    assert_eq!(restored.output, "existing evidence");
    drop(first);
    let (second, _) = state(&scratch);
    assert_ne!(first_token, pair(&second));
    assert_eq!(
        request(
            &second,
            "GET",
            "/api/v1/operations",
            &first_token,
            Value::Null
        )
        .status,
        401
    );
}

#[test]
fn postcheck_needs_installed_identity_and_successful_complete_preview() {
    let installed = ProcessResult {
        exit_code: 0,
        output: "test\t1.2\tii \n".into(),
    };
    let mut preview = ManagerPreview {
        manager: "apt".into(),
        command: "apt list --upgradable".into(),
        exit_code: 0,
        output: "Listing...\n".into(),
        checked_at: Utc::now(),
    };
    assert!(managers::evaluate_apt_update("test", &installed, &preview));
    assert!(!managers::evaluate_apt_update(
        "absent", &installed, &preview
    ));
    preview.output = "test/stable 1.3 amd64 [upgradable from: 1.2]\n".into();
    assert!(!managers::evaluate_apt_update("test", &installed, &preview));
    preview.output.clear();
    preview.exit_code = 1;
    assert!(!managers::evaluate_apt_update("test", &installed, &preview));
    preview.exit_code = 0;
    preview.output = "[INTA output limit reached]".into();
    assert!(!managers::evaluate_apt_update("test", &installed, &preview));
}

#[test]
fn portable_host_rejects_traversal_hidden_files_and_external_symlinks() {
    let scratch = Scratch::new();
    fs::write(scratch.0.join("index.html"), "<title>INTA</title>").unwrap();
    assert_eq!(
        host::resolve_file(&scratch.0, "/").unwrap(),
        scratch.0.join("index.html")
    );
    for path in [
        "/../index.html",
        "/%2e%2e/index.html",
        "/.git/config",
        "/\\..\\test",
        "/secret.txt",
    ] {
        assert!(host::resolve_file(&scratch.0, path).is_none());
    }
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink("/etc/passwd", scratch.0.join("outside.html")).unwrap();
        assert!(host::resolve_file(&scratch.0, "/outside.html").is_none());
    }
}

#[test]
fn process_runner_executes_real_binary_and_captures_both_streams() {
    let spec = CommandSpec::new(env!("CARGO_BIN_EXE_inta-agent"), &["--version"]);
    let result = process::run(&spec).unwrap();
    assert_eq!(result.exit_code, 0);
    assert!(result.output.contains("INTA Rust Local Agent"));
    let invalid = process::run(&CommandSpec::new(
        env!("CARGO_BIN_EXE_inta-agent"),
        &["--not-a-real-option"],
    ))
    .unwrap();
    assert_ne!(invalid.exit_code, 0);
    assert!(invalid.output.contains("Unknown argument"));
}

#[test]
fn live_http_pairing_cors_authentication_and_no_execution() {
    let scratch = Scratch::new();
    let (state, backend) = state(&scratch);
    let listener = server::bind("127.0.0.1:0".parse().unwrap()).unwrap();
    let address = listener.server_addr().to_ip().unwrap();
    let state_copy = state.clone();
    let worker = thread::spawn(move || {
        for _ in 0..3 {
            server::respond(listener.recv().unwrap(), &state_copy);
        }
    });
    let send = |request: &str| {
        let mut stream = TcpStream::connect(address).unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        stream.write_all(request.as_bytes()).unwrap();
        let mut output = String::new();
        stream.read_to_string(&mut output).unwrap();
        output
    };
    let pair_response = send("POST /api/v1/pair HTTP/1.1\r\nHost: 127.0.0.1:47821\r\nOrigin: https://herbertofury.github.io\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
    assert!(pair_response.starts_with("HTTP/1.1 200"));
    assert!(pair_response
        .to_lowercase()
        .contains("access-control-allow-origin: https://herbertofury.github.io"));
    let body: Value =
        serde_json::from_str(pair_response.split("\r\n\r\n").nth(1).unwrap()).unwrap();
    let token = body["token"].as_str().unwrap();
    let history = send(&format!("GET /api/v1/operations HTTP/1.1\r\nHost: 127.0.0.1:47821\r\nOrigin: https://herbertofury.github.io\r\nAuthorization: Bearer {token}\r\nConnection: close\r\n\r\n"));
    assert!(history.starts_with("HTTP/1.1 200"));
    let foreign = send("POST /api/v1/pair HTTP/1.1\r\nHost: 127.0.0.1:47821\r\nOrigin: https://evil.example\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
    assert!(foreign.starts_with("HTTP/1.1 403"));
    assert!(!foreign
        .to_lowercase()
        .contains("access-control-allow-origin:"));
    worker.join().unwrap();
    assert_eq!(backend.executed.load(Ordering::SeqCst), 0);
    assert!(server::bind("0.0.0.0:0".parse().unwrap()).is_err());
}

#[test]
fn rust_packager_writes_verified_archive_and_sha256_sidecar() {
    use sha2::{Digest, Sha256};
    let scratch = Scratch::new();
    let publish = scratch.0.join("publish");
    fs::create_dir(&publish).unwrap();
    fs::write(publish.join("inta-agent.exe"), b"fixture-binary").unwrap();
    let archive = scratch.0.join("inta-agent-test.zip");
    let result = process::run(&CommandSpec::new(
        env!("CARGO_BIN_EXE_inta-agent-package"),
        &[publish.to_str().unwrap(), archive.to_str().unwrap()],
    ))
    .unwrap();
    assert_eq!(result.exit_code, 0);
    let digest = format!("{:x}", Sha256::digest(fs::read(&archive).unwrap()));
    assert_eq!(
        fs::read_to_string(scratch.0.join("inta-agent-test.zip.sha256")).unwrap(),
        format!("{digest}  inta-agent-test.zip\n")
    );
    let mut zip = zip::ZipArchive::new(fs::File::open(archive).unwrap()).unwrap();
    assert_eq!(zip.len(), 1);
    let mut contents = String::new();
    zip.by_name("inta-agent.exe")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    assert_eq!(contents, "fixture-binary");
}
