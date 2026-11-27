//! Freshness metadata never upgrades an editorial review or installs a package.
use crate::{field, items, Result};
use chrono::Utc;
use serde_json::{json, Value};
use std::{fs, path::Path, time::Duration};
use url::Url;
pub fn github_repo(tool: &Value) -> Option<String> {
    for key in ["sourceUrl", "homepage", "downloadUrl"] {
        let Ok(u) = Url::parse(field(tool, key)) else {
            continue;
        };
        if u.host_str() != Some("github.com") {
            continue;
        }
        let p: Vec<_> = u.path().trim_matches('/').split('/').collect();
        if p.len() >= 2
            && p[..2].iter().all(|s| {
                !s.is_empty()
                    && s.chars()
                        .all(|c| c.is_alphanumeric() || matches!(c, '_' | '.' | '-'))
            })
        {
            return Some(format!(
                "{}/{}",
                p[0],
                p[1].strip_suffix(".git").unwrap_or(p[1])
            ));
        }
    }
    None
}
#[derive(Debug)]
pub struct Observation {
    pub status: u16,
    pub resolved_url: String,
    pub body: Option<Value>,
}
pub type Observer<'a> = dyn Fn(&str, &str, Duration) -> Result<Observation> + Sync + 'a;
pub fn observe(url: &str, method: &str, timeout: Duration) -> Result<Observation> {
    let agent = ureq::AgentBuilder::new()
        .timeout(timeout)
        .try_proxy_from_env(true)
        .redirects(3)
        .build();
    let response = match agent
        .request(method, url)
        .set("User-Agent", "INTA-Technician-Hub/2.0")
        .set(
            "Accept",
            if url.starts_with("https://api.github.com/") {
                "application/vnd.github+json"
            } else {
                "text/html,*/*;q=0.5"
            },
        )
        .call()
    {
        Ok(r) => r,
        Err(ureq::Error::Status(_, r)) => r,
        Err(e) => return Err(e.to_string()),
    };
    let status = response.status();
    let resolved_url = response.get_url().to_string();
    let body = if url.starts_with("https://api.github.com/") && status == 200 {
        use std::io::Read;
        let mut data = Vec::new();
        response
            .into_reader()
            .take(2 * 1024 * 1024 + 1)
            .read_to_end(&mut data)
            .map_err(|e| e.to_string())?;
        if data.len() > 2 * 1024 * 1024 {
            return Err("Release metadata response exceeds size budget".into());
        }
        Some(serde_json::from_slice(&data).map_err(|e| e.to_string())?)
    } else {
        None
    };
    Ok(Observation {
        status,
        resolved_url,
        body,
    })
}
pub fn inspect(tool: &Value, timeout: Duration, observer: &Observer<'_>) -> Value {
    let mut result = json!({"id":tool["id"],"name":tool["name"],"editorialReview":tool["reviewedAt"],"downloadUrl":tool["downloadUrl"]});
    let repo = github_repo(tool);
    let url = repo
        .as_ref()
        .map(|r| format!("https://api.github.com/repos/{r}/releases/latest"))
        .unwrap_or_else(|| field(tool, "downloadUrl").to_string());
    let mut response = observer(&url, if repo.is_some() { "GET" } else { "HEAD" }, timeout);
    if repo.is_none()
        && response
            .as_ref()
            .is_ok_and(|r| matches!(r.status, 405 | 501))
    {
        response = observer(&url, "GET", timeout);
    }
    match response {
        Ok(r) => {
            result["httpStatus"] = json!(r.status);
            result["resolvedUrl"] = json!(r.resolved_url);
            if repo.is_some() && r.status == 404 {
                result["status"] = json!("manual-version-review");
                result["detail"]=json!("No latest stable GitHub release is visible. Check the official download source manually.");
            } else if !(200..400).contains(&r.status) {
                result["status"] = json!("unresolved");
                result["detail"] = json!(format!("HTTP {}", r.status));
            } else if repo.is_some() {
                if let Some(release) = r.body {
                    if release["draft"] == true || release["prerelease"] == true {
                        result["status"] = json!("review-required");
                        result["detail"] =
                            json!("The returned release is not a stable public release.");
                    } else {
                        result["status"] = json!("release-observed");
                        result["version"] = release["tag_name"].clone();
                        result["publishedAt"] = release["published_at"].clone();
                        result["releaseUrl"] = release["html_url"].clone();
                        result["detail"]=json!("Upstream metadata only. Review, test and verify the download before adopting.");
                    }
                } else {
                    result["status"] = json!("unresolved");
                    result["detail"] = json!("Missing release metadata");
                }
            } else {
                result["status"] = json!("manual-version-review");
                result["detail"] = json!(
                    "Official link responded. A version has not been inferred from reachability."
                );
            }
        }
        Err(e) => {
            result["status"] = json!("unresolved");
            result["detail"] = json!(e);
        }
    }
    result
}
pub fn report(catalog: &Path, output: &Path, workers: usize, timeout: u64) -> Result<Value> {
    if !(1..=16).contains(&workers) || !(1..=60).contains(&timeout) {
        return Err("Use 1–16 workers and a 1–60 second request timeout".into());
    }
    let data: Value = serde_json::from_slice(&fs::read(catalog).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    let tools = data
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_else(|| items(&data, "tools"));
    let mut results = Vec::new();
    for chunk in tools.chunks(workers) {
        let rows = std::thread::scope(|scope| {
            let handles: Vec<_> = chunk
                .iter()
                .map(|tool| {
                    scope.spawn(move || inspect(tool, Duration::from_secs(timeout), &observe))
                })
                .collect();
            handles
                .into_iter()
                .map(|h| {
                    h.join()
                        .map_err(|_| "Upstream observer panicked".to_string())
                })
                .collect::<Result<Vec<_>>>()
        })?;
        results.extend(rows);
    }
    let unresolved = results
        .iter()
        .filter(|r| r["status"] == "unresolved")
        .count();
    let value = json!({"checkedAt":Utc::now().to_rfc3339(),"expected":tools.len(),"reported":results.len(),"unresolved":unresolved,"installed":false,"results":results});
    if let Some(p) = output.parent() {
        fs::create_dir_all(p).map_err(|e| e.to_string())?;
    }
    fs::write(
        output,
        serde_json::to_string_pretty(&value).map_err(|e| e.to_string())? + "\n",
    )
    .map_err(|e| e.to_string())?;
    Ok(value)
}
#[cfg(test)]
mod tests {
    use super::*;
    fn tool() -> Value {
        json!({"id":"fixture","name":"Fixture","reviewedAt":"2026-11-27","sourceUrl":"https://github.com/example/tool","homepage":"https://example.com","downloadUrl":"https://example.com/download"})
    }
    #[test]
    fn observed_release_preserves_version() {
        let v = inspect(&tool(), Duration::from_secs(1), &|u, m, _| {
            assert_eq!(m, "GET");
            assert!(u.ends_with("/example/tool/releases/latest"));
            Ok(Observation {
                status: 200,
                resolved_url: u.into(),
                body: Some(
                    json!({"tag_name":"v2.0","html_url":"https://github.com/example/tool/releases/tag/v2.0"}),
                ),
            })
        });
        assert_eq!(v["version"], "v2.0");
        assert_eq!(v["status"], "release-observed");
        assert_eq!(v["editorialReview"], "2026-11-27");
    }
    #[test]
    fn missing_release_and_network_failure_never_invent_versions() {
        let missing = inspect(&tool(), Duration::from_secs(1), &|u, _, _| {
            Ok(Observation {
                status: 404,
                resolved_url: u.into(),
                body: None,
            })
        });
        assert_eq!(missing["status"], "manual-version-review");
        assert!(missing.get("version").is_none());
        let failure = inspect(&tool(), Duration::from_secs(1), &|_, _, _| {
            Err("unavailable".into())
        });
        assert_eq!(failure["status"], "unresolved");
    }
    #[test]
    fn reachable_vendor_is_not_version_verification_and_head_falls_back() {
        let mut t = tool();
        t["sourceUrl"] = json!("https://example.com/source");
        let v = inspect(&t, Duration::from_secs(1), &|u, m, _| {
            Ok(Observation {
                status: if m == "HEAD" { 405 } else { 200 },
                resolved_url: u.into(),
                body: None,
            })
        });
        assert_eq!(v["status"], "manual-version-review");
        assert_eq!(v["httpStatus"], 200);
        assert!(v.get("version").is_none());
    }
    #[test]
    fn repository_detection_requires_explicit_publisher_url() {
        let mut t = tool();
        t["sourceUrl"] = json!("https://docs.example.org/github.com/owner/repo");
        assert!(github_repo(&t).is_none());
        t["sourceUrl"] = json!("https://github.com/example/tool.git/releases");
        assert_eq!(github_repo(&t).as_deref(), Some("example/tool"));
    }
}
