use crate::{inventory, managers, models::*, random_id, store::OperationStore, PORT, VERSION};
use chrono::Utc;
use serde_json::{json, Value};
use std::{
    io::{self, Read},
    net::{IpAddr, SocketAddr},
    path::Path,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread,
};
use tiny_http::{Header, Request, Response, Server, StatusCode};

pub const ALLOWED_ORIGINS: &[&str] = &[
    "https://herbertofury.github.io",
    "https://intacomputerservices.com",
    "https://www.intacomputerservices.com",
    "http://localhost:5173",
    "http://127.0.0.1:5173",
];
pub const BODY_LIMIT: usize = 65_536;
const QUEUE_LIMIT: usize = 128;

pub trait Backend: Send + Sync {
    fn managers(&self) -> Vec<String>;
    fn inventory(&self) -> InventoryResult;
    fn execute(&self, request: &OperationRequest) -> Result<ProcessResult, String>;
    fn verify(&self, request: &OperationRequest) -> Verification;
    fn pending_reboot(&self) -> bool;
}
pub struct HostBackend;
impl Backend for HostBackend {
    fn managers(&self) -> Vec<String> {
        managers::detect()
    }
    fn inventory(&self) -> InventoryResult {
        inventory::collect()
    }
    fn execute(&self, r: &OperationRequest) -> Result<ProcessResult, String> {
        managers::execute(r)
    }
    fn verify(&self, r: &OperationRequest) -> Verification {
        managers::verify(r)
    }
    fn pending_reboot(&self) -> bool {
        inventory::pending_reboot()
    }
}

pub struct AppState {
    pub store: OperationStore,
    backend: Arc<dyn Backend>,
    token: String,
    queued: AtomicUsize,
}
impl AppState {
    pub fn new(data_dir: impl AsRef<Path>, backend: Arc<dyn Backend>) -> io::Result<Self> {
        Ok(Self {
            store: OperationStore::open(data_dir)?,
            backend,
            token: random_id(32),
            queued: AtomicUsize::new(0),
        })
    }
    fn reserve(&self, count: usize) -> bool {
        let mut current = self.queued.load(Ordering::SeqCst);
        loop {
            if current + count > QUEUE_LIMIT {
                return false;
            }
            match self.queued.compare_exchange(
                current,
                current + count,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => return true,
                Err(observed) => current = observed,
            }
        }
    }
    fn enqueue(
        self: &Arc<Self>,
        requests: Vec<OperationRequest>,
    ) -> Result<Vec<OperationRecord>, (u16, String)> {
        let installed = self.backend.managers();
        for request in &requests {
            managers::validate(request, &installed).map_err(|e| (400, e))?;
        }
        if !self.reserve(requests.len()) {
            return Err((
                429,
                "The operation queue is full. Wait for current work to finish.".into(),
            ));
        }
        let records: Vec<_> = requests
            .iter()
            .map(|request| OperationRecord {
                id: random_id(16),
                manager: request.manager.clone(),
                action: request.action.clone(),
                package_id: request.package_id.clone(),
                state: "queued".into(),
                started_at: Utc::now(),
                finished_at: None,
                exit_code: None,
                output: String::new(),
                reboot_likely: false,
                error: None,
            })
            .collect();
        if let Err(error) = self.store.save_batch(&records) {
            self.queued.fetch_sub(requests.len(), Ordering::SeqCst);
            return Err((
                500,
                format!("Could not save operation history; nothing was launched: {error}"),
            ));
        }
        for (request, record) in requests.into_iter().zip(records.iter().cloned()) {
            let state = self.clone();
            thread::spawn(move || {
                state.execute(request, record);
                state.queued.fetch_sub(1, Ordering::SeqCst);
            });
        }
        Ok(records)
    }
    fn execute(&self, request: OperationRequest, mut record: OperationRecord) {
        let manager_lock = self.store.lock_for(&request.manager);
        let _guard = manager_lock.lock().unwrap();
        record.state = "running".into();
        if let Err(error) = self.store.save(&record) {
            eprintln!("Could not record running state; operation was not launched: {error}");
            return;
        }
        match self.backend.execute(&request) {
            Ok(result) => {
                let success = result.exit_code == 0
                    || (request.manager == "windows-update" && result.exit_code == 3010);
                record.exit_code = Some(result.exit_code);
                record.output = result.output;
                record.reboot_likely = self.backend.pending_reboot() || result.exit_code == 3010;
                if success {
                    let verification = self.backend.verify(&request);
                    record.output.push_str("\n\n===== INTA POST-CHECK =====\n");
                    record.output.push_str(&verification.output);
                    record.state = if record.reboot_likely {
                        "awaiting-reboot"
                    } else if verification.verified {
                        "verified"
                    } else {
                        "completed-review"
                    }
                    .into();
                    if record.state == "completed-review" {
                        record.error = Some("The operation exited successfully. Review its post-check and verify the original task; completion is not yet proven.".into());
                    }
                } else {
                    record.state = "failed".into();
                    record.error = Some(format!("The package manager returned exit code {}. Review the output before retrying.", result.exit_code));
                }
            }
            Err(error) => {
                record.state = "failed".into();
                record.error = Some(error);
            }
        }
        record.finished_at = Some(Utc::now());
        if let Err(error) = self.store.save(&record) {
            eprintln!("Could not save final operation record: {error}");
        }
    }
}

pub struct RequestContext<'a> {
    pub remote: IpAddr,
    pub method: &'a str,
    pub path: &'a str,
    pub host: &'a str,
    pub origin: &'a str,
    pub authorization: &'a str,
    pub body: &'a [u8],
}
#[derive(Debug)]
pub struct ApiResponse {
    pub status: u16,
    pub value: Value,
}
fn error(status: u16, message: impl Into<String>) -> ApiResponse {
    ApiResponse {
        status,
        value: json!({"error":message.into()}),
    }
}
fn ok(value: Value) -> ApiResponse {
    ApiResponse { status: 200, value }
}

pub fn valid_host(host: &str, port: u16) -> bool {
    [
        format!("127.0.0.1:{port}"),
        format!("localhost:{port}"),
        format!("[::1]:{port}"),
    ]
    .iter()
    .any(|value| host.eq_ignore_ascii_case(value))
}
fn same_token(received: &str, expected: &str) -> bool {
    if received.len() != expected.len() {
        return false;
    }
    received
        .bytes()
        .zip(expected.bytes())
        .fold(0u8, |diff, (a, b)| diff | (a ^ b))
        == 0
}

pub fn handle(state: &Arc<AppState>, context: &RequestContext<'_>) -> ApiResponse {
    if !context.remote.is_loopback() || !valid_host(context.host, PORT) {
        return error(403, "Loopback host required.");
    }
    if !ALLOWED_ORIGINS
        .iter()
        .any(|origin| context.origin.eq_ignore_ascii_case(origin))
    {
        return error(403, "This browser origin is not allowed.");
    }
    if context.method == "OPTIONS" {
        return ApiResponse {
            status: 204,
            value: Value::Null,
        };
    }
    if context.path != "/api/v1/pair"
        && !same_token(context.authorization, &format!("Bearer {}", state.token))
    {
        return error(401, "Pair this browser tab with the local agent.");
    }
    if context.body.len() > BODY_LIMIT {
        return error(413, "Request body is too large.");
    }
    match (context.method, context.path) {
        ("POST", "/api/v1/pair") => {
            ok(json!({"token":state.token,"version":VERSION,"platform":inventory::platform()}))
        }
        ("GET", "/api/v1/health") => {
            ok(json!({"status":"ready","version":VERSION,"platform":inventory::platform()}))
        }
        ("GET", "/api/v1/status") => ok(
            json!({"version":VERSION,"platform":inventory::platform(),"hostname":inventory::hostname(),"elevated":inventory::elevated(),"managers":state.backend.managers(),"capabilities":["inventory","update-preview","update","repair","uninstall","operation-history"]}),
        ),
        ("GET", "/api/v1/inventory") => ok(json!(state.backend.inventory())),
        ("GET", "/api/v1/operations") => ok(json!(state.store.recent())),
        ("POST", "/api/v1/operations") => {
            match serde_json::from_slice::<OperationRequest>(context.body) {
                Ok(request) => match state.enqueue(vec![request]) {
                    Ok(records) => ApiResponse {
                        status: 202,
                        value: json!(records[0]),
                    },
                    Err((status, message)) => error(status, message),
                },
                Err(_) => error(400, "Invalid typed operation request."),
            }
        }
        ("POST", "/api/v1/operations/batch") => {
            match serde_json::from_slice::<BatchRequest>(context.body) {
                Ok(mut request) => {
                    if !request.confirmed {
                        return error(400, "Confirm once to run this update batch.");
                    }
                    if request.operations.is_empty()
                        || request.operations.len() > 30
                        || request.operations.iter().any(|r| r.action != "update")
                    {
                        return error(400, "A batch accepts 1–30 exact package updates only.");
                    }
                    for operation in &mut request.operations {
                        operation.confirmed = true;
                    }
                    match state.enqueue(request.operations) {
                        Ok(records) => ApiResponse {
                            status: 202,
                            value: json!(records),
                        },
                        Err((status, message)) => error(status, message),
                    }
                }
                Err(_) => error(400, "Invalid typed update batch."),
            }
        }
        ("GET", path) if path.starts_with("/api/v1/operations/") => {
            let id = path.trim_start_matches("/api/v1/operations/");
            match state.store.get(id) {
                Some(record) => ok(json!(record)),
                None => error(404, "Operation not found."),
            }
        }
        _ => error(404, "Endpoint not found."),
    }
}

pub fn bind(address: SocketAddr) -> io::Result<Server> {
    if !address.ip().is_loopback() {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Only loopback listeners are permitted.",
        ));
    }
    Server::http(address).map_err(io::Error::other)
}
pub fn serve(server: Server, state: Arc<AppState>) {
    let (sender, receiver) = std::sync::mpsc::sync_channel::<Request>(32);
    let receiver = Arc::new(std::sync::Mutex::new(receiver));
    for _ in 0..8 {
        let receiver = receiver.clone();
        let state = state.clone();
        thread::spawn(move || loop {
            let request = receiver.lock().unwrap().recv();
            match request {
                Ok(request) => respond(request, &state),
                Err(_) => return,
            }
        });
    }
    for request in server.incoming_requests() {
        if sender.send(request).is_err() {
            break;
        }
    }
}
pub fn respond(mut request: Request, state: &Arc<AppState>) {
    let header = |name: &'static str| {
        request
            .headers()
            .iter()
            .find(|h| h.field.equiv(name))
            .map(|h| h.value.to_string())
            .unwrap_or_default()
    };
    let origin = header("Origin");
    let host = header("Host");
    let authorization = header("Authorization");
    let content_type = header("Content-Type");
    let method = request.method().as_str().to_string();
    let path = request.url().to_string();
    let remote = request
        .remote_addr()
        .map(|a| a.ip())
        .unwrap_or_else(|| "0.0.0.0".parse().unwrap());
    let mut body = Vec::new();
    // Refuse foreign origins and hosts before reading a client body.
    let permitted = remote.is_loopback()
        && valid_host(&host, PORT)
        && ALLOWED_ORIGINS
            .iter()
            .any(|v| origin.eq_ignore_ascii_case(v));
    let body_result = if permitted {
        request
            .as_reader()
            .take((BODY_LIMIT + 1) as u64)
            .read_to_end(&mut body)
    } else {
        Ok(0)
    };
    let response = if body_result.is_err() {
        error(400, "Could not read request body.")
    } else if permitted
        && method == "POST"
        && path != "/api/v1/pair"
        && !content_type
            .split(';')
            .next()
            .is_some_and(|v| v.trim().eq_ignore_ascii_case("application/json"))
    {
        error(415, "Use application/json for typed operations.")
    } else {
        handle(
            state,
            &RequestContext {
                remote,
                method: &method,
                path: &path,
                host: &host,
                origin: &origin,
                authorization: &authorization,
                body: &body,
            },
        )
    };
    let bytes = if response.status == 204 {
        Vec::new()
    } else {
        serde_json::to_vec(&response.value).unwrap_or_default()
    };
    let mut outgoing = Response::from_data(bytes).with_status_code(StatusCode(response.status));
    for (name, value) in [
        ("Content-Type", "application/json; charset=utf-8"),
        ("Cache-Control", "no-store"),
        ("X-Content-Type-Options", "nosniff"),
        ("Vary", "Origin"),
    ] {
        outgoing.add_header(Header::from_bytes(name, value).unwrap());
    }
    if permitted {
        for (name, value) in [
            ("Access-Control-Allow-Origin", origin.as_str()),
            ("Access-Control-Allow-Methods", "GET, POST, OPTIONS"),
            (
                "Access-Control-Allow-Headers",
                "Authorization, Content-Type",
            ),
            ("Access-Control-Max-Age", "600"),
            ("Access-Control-Allow-Private-Network", "true"),
        ] {
            outgoing.add_header(Header::from_bytes(name, value).unwrap());
        }
    }
    if response.status == 202 {
        if let Some(id) = response.value.get("id").and_then(Value::as_str) {
            outgoing.add_header(
                Header::from_bytes("Location", format!("/api/v1/operations/{id}")).unwrap(),
            );
        }
    }
    let _ = request.respond(outgoing);
}
