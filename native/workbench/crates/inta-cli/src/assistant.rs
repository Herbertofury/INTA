//! A deliberately small Ollama client for explaining published catalog records.
//! The production endpoint, request paths, context fields, and prompt are fixed.
pub(crate) mod server;
use anyhow::{Context, Result, bail, ensure};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    ffi::c_void,
    ptr,
    time::{Duration, Instant},
};
use windows_sys::Win32::Networking::WinHttp::*;

const PORT: u16 = 11434;
const RESPONSE_LIMIT: usize = 2 * 1024 * 1024;

struct HttpHandle(*mut c_void);
impl HttpHandle {
    fn checked(handle: *mut c_void) -> Result<Self> {
        if handle.is_null() {
            return Err(std::io::Error::last_os_error().into());
        }
        Ok(Self(handle))
    }
}
impl Drop for HttpHandle {
    fn drop(&mut self) {
        unsafe { WinHttpCloseHandle(self.0) };
    }
}
fn checked(result: i32) -> Result<()> {
    if result == 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok(())
}
fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().chain([0]).collect()
}

struct Ollama(u16);
impl Ollama {
    fn request(&self, path: &str, body: Option<&Value>) -> Result<Value> {
        let timeout_ms = if path == "/api/generate" {
            120_000
        } else {
            8_000
        };
        self.request_with_timeout(path, body, timeout_ms)
    }

    fn request_with_timeout(&self, path: &str, body: Option<&Value>, limit: i32) -> Result<Value> {
        let verb = wide(if body.is_some() { "POST" } else { "GET" });
        let bytes = body
            .map(serde_json::to_vec)
            .transpose()?
            .unwrap_or_default();
        let length = u32::try_from(bytes.len())?;
        let started = Instant::now();
        // Each RAII handle remains live until its dependents have been dropped. All
        // wide-string pointers and byte slices stay valid for their synchronous call.
        unsafe {
            let session = HttpHandle::checked(WinHttpOpen(
                wide("INTA Desktop").as_ptr(),
                WINHTTP_ACCESS_TYPE_NO_PROXY,
                ptr::null(),
                ptr::null(),
                0,
            ))?;
            checked(WinHttpSetTimeouts(session.0, 2000, 2000, 5000, limit))?;
            let connection = HttpHandle::checked(WinHttpConnect(
                session.0,
                wide("127.0.0.1").as_ptr(),
                self.0,
                0,
            ))?;
            let request = HttpHandle::checked(WinHttpOpenRequest(
                connection.0,
                verb.as_ptr(),
                wide(path).as_ptr(),
                ptr::null(),
                ptr::null(),
                ptr::null(),
                0,
            ))?;
            checked(WinHttpSetTimeouts(request.0, 2000, 2000, 5000, limit))?;
            for (option, value) in [
                (WINHTTP_OPTION_RECEIVE_RESPONSE_TIMEOUT, limit as u32),
                (
                    WINHTTP_OPTION_REDIRECT_POLICY,
                    WINHTTP_OPTION_REDIRECT_POLICY_NEVER,
                ),
                (
                    WINHTTP_OPTION_DISABLE_FEATURE,
                    WINHTTP_DISABLE_COOKIES | WINHTTP_DISABLE_AUTHENTICATION,
                ),
            ] {
                checked(WinHttpSetOption(
                    request.0,
                    option,
                    (&value as *const u32).cast(),
                    size_of::<u32>() as u32,
                ))?;
            }
            let headers = wide("Content-Type: application/json\r\nAccept: application/json\r\n");
            checked(WinHttpSendRequest(request.0, headers.as_ptr(), (headers.len()-1) as u32,
                if length == 0 { ptr::null() } else { bytes.as_ptr().cast() }, length, length, 0))
                .context("Unable to contact local Ollama. Start a local-only Ollama server with OLLAMA_NO_CLOUD=1")?;
            checked(WinHttpReceiveResponse(request.0, ptr::null_mut()))?;
            let mut status = 0u32;
            let mut size = size_of::<u32>() as u32;
            checked(WinHttpQueryHeaders(
                request.0,
                WINHTTP_QUERY_STATUS_CODE | WINHTTP_QUERY_FLAG_NUMBER,
                ptr::null(),
                (&mut status as *mut u32).cast(),
                &mut size,
                ptr::null_mut(),
            ))?;
            ensure!(
                status == 200,
                "Local Ollama returned HTTP {status} for {path}"
            );
            let mut result = Vec::new();
            let mut buffer = [0u8; 8192];
            loop {
                ensure!(
                    started.elapsed() < Duration::from_millis(limit as u64 + 8_000),
                    "Local Ollama exceeded the request deadline"
                );
                let mut read = 0u32;
                checked(WinHttpReadData(
                    request.0,
                    buffer.as_mut_ptr().cast(),
                    buffer.len() as u32,
                    &mut read,
                ))?;
                if read == 0 {
                    break;
                }
                ensure!(
                    result.len() + read as usize <= RESPONSE_LIMIT,
                    "Local Ollama response exceeded the size limit"
                );
                result.extend_from_slice(&buffer[..read as usize]);
            }
            serde_json::from_slice(&result).context("Local Ollama returned invalid JSON")
        }
    }

    fn models(&self) -> Result<Vec<Value>> {
        let status = self.request("/api/status", None)?;
        ensure!(
            status["cloud"]["disabled"] == true,
            "Local-only generation requires Ollama with cloud disabled. Start Ollama with OLLAMA_NO_CLOUD=1 and refresh."
        );
        let result = self.request("/api/tags", None)?;
        let records = result["models"]
            .as_array()
            .context("Invalid local model inventory")?;
        ensure!(
            records.len() <= 4096,
            "Local model inventory exceeded the limit"
        );
        let mut models = Vec::new();
        for item in records {
            let name = item["name"].as_str().unwrap_or_default();
            let digest = item["digest"].as_str().unwrap_or_default();
            if valid_name(name)
                && valid_digest(digest)
                && local_metadata(item)
                && item["size"].as_u64().is_some_and(|n| n >= 1024 * 1024)
            {
                models.push(json!({"name":name,"digest":digest,"bytes":item["size"]}));
            }
        }
        models.sort_by(|a, b| a["name"].as_str().cmp(&b["name"].as_str()));
        Ok(models)
    }

    fn explain(
        &self,
        kind: &str,
        id: &str,
        model: &str,
        digest: &str,
        prompt_hash: &str,
    ) -> Result<Value> {
        let reviewed = preview(kind, id)?;
        ensure!(
            reviewed["promptSha256"] == prompt_hash,
            "The prompt changed. Review the current prompt before generating."
        );
        self.verify_model(model, digest)?;
        let result = self.request(
            "/api/generate",
            Some(
                &json!({"model":model,"prompt":reviewed["prompt"],"stream":false,
            "keep_alive":0,"options":{"temperature":0.1,"num_predict":240,"num_ctx":2048}}),
            ),
        )?;
        ensure!(
            result["done"] == true && result["model"] == model && !remote(&result),
            "Local generation did not complete with the selected model"
        );
        let answer = result["response"]
            .as_str()
            .context("Ollama returned no answer")?
            .trim();
        ensure!(
            !answer.is_empty() && answer.len() <= 32 * 1024,
            "Ollama returned an empty or oversized answer"
        );
        Ok(
            json!({"kind":kind,"id":id,"model":model,"modelDigest":digest,"promptSha256":prompt_hash,"answer":answer}),
        )
    }

    fn verify_model(&self, model: &str, digest: &str) -> Result<()> {
        ensure!(
            valid_name(model) && valid_digest(digest),
            "Invalid local model selection"
        );
        ensure!(
            self.models()?
                .iter()
                .any(|m| m["name"] == model && m["digest"] == digest),
            "The selected model is absent or changed. Refresh the installed models."
        );
        let metadata = self.request("/api/show", Some(&json!({"model":model})))?;
        ensure!(
            local_metadata(&metadata)
                && metadata["model_info"]["general.architecture"]
                    .as_str()
                    .is_some_and(|s| !s.is_empty())
                && metadata["capabilities"]
                    .as_array()
                    .is_some_and(|c| c.iter().any(|x| x == "completion")),
            "Select an installed, local text-generation model with verified model metadata"
        );
        Ok(())
    }
}

/// A model can suggest one existing local inspection. The executable plan and
/// its digest are constructed here from the reviewed scenario registry.
pub fn triage(c: &rusqlite::Connection, args: &[&str]) -> Result<Value> {
    use crate::scenario;
    fn reviewed(c: &rusqlite::Connection, id: &str) -> Result<(Value, String)> {
        let plan = scenario::preview(c, id)?;
        let digest = format!("{:x}", Sha256::digest(serde_json::to_vec(&plan)?));
        Ok((plan, digest))
    }
    match args {
        ["run-plan", id, digest, "--approve"] => {
            ensure!(
                reviewed(c, id)?.1 == *digest,
                "Inspection plan changed. Request a new preview."
            );
            scenario::run(c, id, &[], None)
        }
        ["ask", model, digest, request] => {
            ensure!(
                !request.trim().is_empty() && request.len() <= 2048,
                "Use a question of 1–2048 UTF-8 bytes"
            );
            let choices = scenario::list(c)?;
            let shortlist = scenario_matches(request);
            let candidates: Vec<_> = choices
                .as_array()
                .context("Invalid scenario registry")?
                .iter()
                .filter(|row| {
                    shortlist.is_empty()
                        || row["id"].as_str().is_some_and(|id| shortlist.contains(&id))
                })
                .map(|row| json!({"id":row["id"],"title":row["title"],"checks":row["stages"]}))
                .collect();
            let mut ids = vec!["unmatched"];
            for item in &candidates {
                ids.push(item["id"].as_str().context("Missing scenario identifier")?);
            }
            let provider = Ollama(PORT);
            provider.verify_model(model, digest)?;
            let instruction = "Select the most relevant local Windows diagnostic scenario ID for the technician's description. Use unmatched when none fits. Choose only from the supplied IDs.";
            let result = provider.request("/api/generate", Some(&json!({
                "model":model, "system":instruction,
                "prompt":format!("Local diagnostic options: {}\nTechnician description: {}",serde_json::to_string(&candidates)?,request),
                "format":{"type":"object","properties":{"scenario":{"type":"string","enum":ids}},
                    "required":["scenario"],"additionalProperties":false},
                "stream":false,"keep_alive":0,"options":{"temperature":0,"num_predict":64,"num_ctx":2048}
            })))?;
            ensure!(
                result["done"] == true && result["model"] == *model && !remote(&result),
                "Local scenario selection did not finish"
            );
            #[derive(serde::Deserialize)]
            #[serde(deny_unknown_fields)]
            struct Suggestion {
                scenario: String,
            }
            let suggestion: Suggestion = serde_json::from_str(
                result["response"]
                    .as_str()
                    .context("Missing model response")?,
            )?;
            ensure!(
                ids.contains(&suggestion.scenario.as_str()),
                "Model returned an unknown scenario"
            );
            if suggestion.scenario == "unmatched" {
                return Ok(
                    json!({"schemaVersion":1,"state":"unmatched","model":model,"executed":false}),
                );
            }
            let (plan, hash) = reviewed(c, &suggestion.scenario)?;
            Ok(
                json!({"schemaVersion":1,"state":"awaiting-review","model":model,"modelDigest":digest,
                "plan":plan,"planSha256":hash,"executed":false}),
            )
        }
        _ => bail!(
            "Use ai ask MODEL MODEL_DIGEST QUESTION or ai run-plan SCENARIO PLAN_SHA256 --approve"
        ),
    }
}

fn scenario_matches(request: &str) -> Vec<&'static str> {
    let words: Vec<_> = request
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(str::to_lowercase)
        .collect();
    [
        (
            "slow-pc",
            &["slow", "slowly", "sluggish", "lagging", "performance"][..],
        ),
        ("printer-repair", &["printer", "printing", "spooler"]),
        ("network-dns", &["dns", "wifi", "internet", "connectivity"]),
        ("hardware-bsod", &["bsod", "bluescreen"]),
        ("outlook-email", &["outlook", "email"]),
        ("windows-update", &["updates", "updating"]),
        ("backup-restore", &["backup", "backups"]),
        ("malware-removal", &["malware", "virus"]),
    ]
    .into_iter()
    .filter(|(_, terms)| words.iter().any(|word| terms.contains(&word.as_str())))
    .map(|(id, _)| id)
    .collect()
}

fn remote(item: &Value) -> bool {
    ["remote_model", "remote_host"]
        .iter()
        .any(|key| !item[key].is_null() && item[key].as_str() != Some(""))
}
fn local_metadata(item: &Value) -> bool {
    !remote(item)
        && matches!(
            item["details"]["format"].as_str(),
            Some("gguf" | "safetensors")
        )
}
fn valid_digest(digest: &str) -> bool {
    digest.len() == 64
        && digest
            .bytes()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
}
fn valid_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    !name.is_empty()
        && name.len() <= 160
        && name
            .bytes()
            .next()
            .is_some_and(|b| b.is_ascii_alphanumeric())
        && name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-' | b'_' | b'/' | b':'))
        && name.split('/').count() <= 2
        && !name.contains("..")
        && !lower.starts_with("cloud/")
        && !lower.ends_with(":cloud")
        && !lower.ends_with("-cloud")
}
fn preview(kind: &str, id: &str) -> Result<Value> {
    let source = match kind {
        "tool" => include_str!(concat!(env!("OUT_DIR"), "/tools.json")),
        "job" => include_str!("../../../../../data/jobs.json"),
        "method" => include_str!("../../../../../data/methods.json"),
        _ => bail!("Choose a public tool, playbook, or method"),
    };
    let data: Value = serde_json::from_str(source)?;
    let rows = data
        .as_array()
        .or_else(|| data["methods"].as_array())
        .context("Invalid embedded catalog")?;
    let record = rows
        .iter()
        .find(|r| r["id"] == id)
        .context("Unknown published catalog record")?;
    let title = record["name"]
        .as_str()
        .or_else(|| record["title"].as_str())
        .context("Catalog title unavailable")?;
    let mut prompt = format!(
        "Explain this published INTA {kind} record to a technician in plain English. Summarize its purpose, appropriate use, and one verification or caution supported by the record. Do not invent capabilities, commands, or host findings.\n\nTitle: {title}"
    );
    for (key, label) in [
        ("summary", "Summary"),
        ("bestFor", "Best use"),
        ("when", "When to use"),
        ("startHere", "Starting point"),
        ("proof", "Verification"),
        ("cautions", "Caution"),
        ("stopIf", "Stop conditions"),
    ] {
        let value = &record[key];
        let content = if let Some(s) = value.as_str() {
            s.to_owned()
        } else if let Some(a) = value.as_array() {
            a.iter()
                .filter_map(Value::as_str)
                .take(3)
                .collect::<Vec<_>>()
                .join(" ")
        } else {
            continue;
        };
        if !content.is_empty() {
            prompt.push_str(&format!(
                "\n{label}: {}",
                content.chars().take(650).collect::<String>()
            ));
        }
    }
    let hash = format!("{:x}", Sha256::digest(prompt.as_bytes()));
    Ok(
        json!({"kind":kind,"id":id,"title":title,"prompt":prompt,"promptSha256":hash,"source":"Published INTA catalog"}),
    )
}

pub fn command(args: &[&str]) -> Result<Value> {
    command_at(args, PORT)
}

// The production dispatcher supplies the fixed port. Integration harnesses call
// the same command path with their independently bound loopback test server.
pub(crate) fn command_at(args: &[&str], port: u16) -> Result<Value> {
    match args {
        ["models"] => Ok(json!({"models":Ollama(port).models()?})),
        ["preview", kind, id] => {
            let mut record = preview(kind, id)?;
            record["endpoint"] = json!(format!("http://127.0.0.1:{port}"));
            Ok(record)
        }
        ["explain", kind, id, model, digest, hash, "--approve"] => {
            Ollama(port).explain(kind, id, model, digest, hash)
        }
        _ => bail!(
            "Use assistant models | assistant preview tool|job|method ID | assistant explain KIND ID MODEL MODEL_DIGEST PROMPT_SHA256 --approve"
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::{BufRead, BufReader, Read, Write},
        net::TcpListener,
        thread,
    };
    const NAME: &str = "fixture:local";
    const SHA: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    fn item() -> Value {
        json!({"name":NAME,"digest":SHA,"size":2_000_000,"details":{"format":"gguf"}})
    }
    fn status() -> Value {
        json!({"cloud":{"disabled":true}})
    }
    fn show() -> Value {
        json!({"details":{"format":"gguf"},"model_info":{"general.architecture":"test"},"capabilities":["completion"]})
    }

    type Reply = (&'static str, u16, String);
    fn ok(path: &'static str, body: Value) -> Reply {
        (path, 200, body.to_string())
    }
    fn fixture(replies: Vec<Reply>) -> (Ollama, thread::JoinHandle<Vec<Value>>) {
        let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let port = listener.local_addr().unwrap().port();
        listener.set_nonblocking(true).unwrap();
        let worker = thread::spawn(move || {
            let deadline = Instant::now() + Duration::from_secs(15);
            let mut bodies = Vec::new();
            for (path, status, reply) in replies {
                let mut client = loop {
                    if let Ok((client, _)) = listener.accept() {
                        break client;
                    }
                    assert!(
                        Instant::now() < deadline,
                        "fixture timed out waiting for {path}"
                    );
                    thread::sleep(Duration::from_millis(3));
                };
                // Winsock can propagate the listener's nonblocking mode to accepted sockets.
                client.set_nonblocking(false).unwrap();
                client
                    .set_read_timeout(Some(Duration::from_secs(3)))
                    .unwrap();
                client
                    .set_write_timeout(Some(Duration::from_secs(3)))
                    .unwrap();
                let mut reader = BufReader::new(client.try_clone().unwrap());
                let mut line = String::new();
                reader.read_line(&mut line).unwrap();
                assert_eq!(line.split_ascii_whitespace().nth(1), Some(path));
                let mut size = 0;
                loop {
                    line.clear();
                    assert!(reader.read_line(&mut line).unwrap() > 0);
                    if line == "\r\n" {
                        break;
                    }
                    if let Some(v) = line.to_ascii_lowercase().strip_prefix("content-length:") {
                        size = v.trim().parse::<usize>().unwrap();
                    }
                }
                assert!(size < 32_768);
                let mut bytes = vec![0; size];
                reader.read_exact(&mut bytes).unwrap();
                bodies.push(if bytes.is_empty() {
                    Value::Null
                } else {
                    serde_json::from_slice(&bytes).unwrap()
                });
                let redir = if status == 302 {
                    "Location: http://127.0.0.1:1/should-not-follow\r\n"
                } else {
                    ""
                };
                write!(client,"HTTP/1.1 {status} Response\r\nContent-Type: application/json\r\n{redir}Content-Length: {}\r\nConnection: close\r\n\r\n",reply.len()).unwrap();
                let _ = client.write_all(reply.as_bytes());
            }
            bodies
        });
        (Ollama(port), worker)
    }
    #[test]
    fn verified_model_receives_only_reviewed_catalog_prompt() {
        let prompt = preview("tool", "7zip").unwrap();
        let (provider, worker) = fixture(vec![
            ok("/api/status", status()),
            ok("/api/tags", json!({"models":[item()]})),
            ok("/api/show", show()),
            ok(
                "/api/generate",
                json!({"model":NAME,"done":true,"response":"Plain [link](https://example.invalid)."}),
            ),
        ]);
        let response = provider
            .explain(
                "tool",
                "7zip",
                NAME,
                SHA,
                prompt["promptSha256"].as_str().unwrap(),
            )
            .unwrap();
        assert_eq!(response["answer"], "Plain [link](https://example.invalid).");
        let requests = worker.join().unwrap();
        assert_eq!(requests[3]["prompt"], prompt["prompt"]);
        assert_eq!(requests[3]["stream"], false);
        assert_eq!(requests[3]["model"], NAME);
        assert!(requests[3].get("tools").is_none());
    }
    #[test]
    fn cloud_alias_redirect_oversize_and_unreviewed_inputs_are_refused() {
        let (provider, worker) =
            fixture(vec![ok("/api/status", json!({"cloud":{"disabled":false}}))]);
        assert!(
            provider
                .models()
                .unwrap_err()
                .to_string()
                .contains("cloud disabled")
        );
        worker.join().unwrap();
        let (provider, worker) = fixture(vec![("/api/status", 302, "{}".into())]);
        assert!(provider.models().unwrap_err().to_string().contains("302"));
        worker.join().unwrap();
        let (provider, worker) =
            fixture(vec![("/api/status", 200, " ".repeat(RESPONSE_LIMIT + 1))]);
        assert!(
            provider
                .models()
                .unwrap_err()
                .to_string()
                .contains("size limit")
        );
        worker.join().unwrap();
        let mut alias = show();
        alias["remote_host"] = json!("https://example.invalid");
        let (provider, worker) = fixture(vec![
            ok("/api/status", status()),
            ok("/api/tags", json!({"models":[item()]})),
            ok("/api/show", alias),
        ]);
        let prompt = preview("tool", "7zip").unwrap();
        assert!(
            provider
                .explain(
                    "tool",
                    "7zip",
                    NAME,
                    SHA,
                    prompt["promptSha256"].as_str().unwrap()
                )
                .unwrap_err()
                .to_string()
                .contains("local text-generation")
        );
        worker.join().unwrap();
        assert!(
            Ollama(1)
                .explain("tool", "7zip", NAME, SHA, "unreviewed")
                .unwrap_err()
                .to_string()
                .contains("prompt changed")
        );
        assert!(command(&["explain", "tool", "7zip", NAME, SHA, "hash"]).is_err());
        assert!(preview("tool", "unknown").is_err());
        assert!(preview("path", r"C:\private.txt").is_err());
        assert!(!valid_name("gpt-oss:120b-cloud"));
    }
    #[test]
    fn stalled_local_server_has_a_receive_deadline() {
        let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let port = listener.local_addr().unwrap().port();
        let (finish, done) = std::sync::mpsc::channel();
        let worker = thread::spawn(move || {
            let (mut connection, _) = listener.accept().unwrap();
            connection
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            let mut request = BufReader::new(connection.try_clone().unwrap());
            let mut line = String::new();
            loop {
                line.clear();
                if request.read_line(&mut line).unwrap() == 0 || line == "\r\n" {
                    break;
                }
            }
            connection
                .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 4096\r\nConnection: close\r\n\r\n{")
                .unwrap();
            let _ = done.recv_timeout(Duration::from_secs(12));
        });
        let start = Instant::now();
        let error = Ollama(port)
            .request_with_timeout("/api/status", None, 80)
            .unwrap_err();
        let _ = finish.send(());
        worker.join().unwrap();
        assert!(
            error.chain().any(|cause| cause
                .downcast_ref::<std::io::Error>()
                .and_then(std::io::Error::raw_os_error)
                == Some(12002)),
            "expected WinHTTP timeout: {error:#}"
        );
        assert!(start.elapsed() < Duration::from_secs(12));
    }
    #[test]
    fn changed_model_digest_blocks_generation_before_show() {
        let (provider, worker) = fixture(vec![
            ok("/api/status", status()),
            ok("/api/tags", json!({"models":[item()]})),
        ]);
        let p = preview("job", "windows-repair").unwrap();
        assert!(
            provider
                .explain(
                    "job",
                    "windows-repair",
                    NAME,
                    &"b".repeat(64),
                    p["promptSha256"].as_str().unwrap()
                )
                .unwrap_err()
                .to_string()
                .contains("absent or changed")
        );
        worker.join().unwrap();
    }
    #[test]
    #[ignore = "requires the separately prepared local-only Qwen2.5 test server"]
    fn actual_local_ollama_generates() {
        let port = std::env::var("INTA_TEST_OLLAMA_PORT")
            .unwrap()
            .parse()
            .unwrap();
        let provider = Ollama(port);
        let models = provider.models().unwrap();
        let model = models
            .iter()
            .find(|m| m["name"] == "qwen2.5:0.5b")
            .expect("Qwen2.5 local fixture");
        let p = preview("tool", "7zip").unwrap();
        let result = provider
            .explain(
                "tool",
                "7zip",
                "qwen2.5:0.5b",
                model["digest"].as_str().unwrap(),
                p["promptSha256"].as_str().unwrap(),
            )
            .unwrap();
        assert!(result["answer"].as_str().unwrap().len() > 30);
        println!("{}", result);
    }
}
