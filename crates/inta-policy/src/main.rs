//! Changes and finished-state gates for INTA's mandatory Rust implementation rule.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::{Component, Path, PathBuf},
    process::{Command, ExitCode},
};

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn field<'a>(v: &'a Value, key: &str) -> &'a str {
    v.get(key).and_then(Value::as_str).unwrap_or("")
}
fn records<'a>(v: &'a Value, key: &str) -> &'a [Value] {
    v.get(key)
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or(&[])
}
fn safe_path(s: &str) -> bool {
    !s.is_empty()
        && !s.contains(['*', '?', '\\', '\0'])
        && Path::new(s)
            .components()
            .all(|c| matches!(c, Component::Normal(_)))
}
fn file(root: &Path, name: &str) -> Result<Vec<u8>, String> {
    if !safe_path(name) {
        return Err(format!("Invalid exact repository path: {name}"));
    }
    let path = root.join(name);
    let canonical_root = root.canonicalize().map_err(|e| e.to_string())?;
    let canonical = path.canonicalize().map_err(|e| format!("{name}: {e}"))?;
    if !canonical.starts_with(canonical_root) {
        return Err(format!("Path leaves checkout: {name}"));
    }
    fs::read(path).map_err(|e| format!("{name}: {e}"))
}
fn checked_file(root: &Path, record: &Value) -> Result<Vec<u8>, String> {
    let name = field(record, "path");
    let bytes = file(root, name)?;
    if field(record, "sha256") != digest(&bytes) {
        return Err(format!("Evidence/source hash changed: {name}"));
    }
    Ok(bytes)
}
fn git(root: &Path, args: &[&str]) -> Result<Vec<u8>, String> {
    let out = Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_string());
    }
    Ok(out.stdout)
}
fn source_language(name: &str, bytes: &[u8]) -> Option<&'static str> {
    let ext = Path::new(name)
        .extension()
        .and_then(|v| v.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    let known = match ext.as_str() {
        "js" | "mjs" | "cjs" | "jsx" => Some("JavaScript"),
        "ts" | "tsx" => Some("TypeScript"),
        "py" => Some("Python"),
        "cs" => Some("C#"),
        "ps1" | "psm1" => Some("PowerShell"),
        "sh" | "bash" | "zsh" => Some("Shell"),
        "c" | "h" => Some("C"),
        "cc" | "cpp" | "hpp" => Some("C++"),
        "go" => Some("Go"),
        "java" => Some("Java"),
        "kt" | "kts" => Some("Kotlin"),
        "swift" => Some("Swift"),
        "rb" => Some("Ruby"),
        "php" => Some("PHP"),
        "lua" => Some("Lua"),
        "pl" => Some("Perl"),
        "bat" | "cmd" => Some("Batch"),
        _ => None,
    };
    if known.is_some() {
        return known;
    }
    let text = String::from_utf8_lossy(bytes);
    if bytes.starts_with(b"#!") {
        let line = text.lines().next().unwrap_or("");
        if line.contains("python") {
            return Some("Python");
        }
        if line.contains("bash") || line.contains("/sh") || line.contains("/env sh") {
            return Some("Shell");
        }
        if line.contains("node") {
            return Some("JavaScript");
        }
    }
    if ["html", "htm", "svg"].contains(&ext.as_str()) && embedded_script(&text) {
        return Some("Embedded script");
    }
    None
}
fn embedded_script(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    let mut offset = 0;
    while let Some(relative) = lower[offset..].find("<script") {
        let start = offset + relative + 7;
        let Some(end_relative) = lower[start..].find('>') else {
            return true;
        };
        let end = start + end_relative;
        let Some(close_relative) = lower[end + 1..].find("</script") else {
            return true;
        };
        let close = end + 1 + close_relative;
        let tag = &lower[start..end];
        let body = text[end + 1..close].trim();
        let literal_build_placeholder = (body == "{{DATA}}" || body == "{{APP}}")
            && !tag.contains("src=")
            && !tag.contains("src =");
        if !tag.contains("application/json")
            && !tag.contains("application/ld+json")
            && !literal_build_placeholder
        {
            return true;
        }
        offset = close + 8;
    }
    // Inline handlers and javascript URLs are executable even without a script tag.
    lower.contains("javascript:")
        || lower.split_whitespace().any(|word| {
            let name = word.split('=').next().unwrap_or("");
            name.starts_with("on") && name.len() > 3 && word.contains('=')
        })
}
fn require_text(record: &Value, keys: &[&str]) -> Result<(), String> {
    for key in keys {
        if field(record, key).trim().is_empty() {
            return Err(format!("Missing required evidence field {key}"));
        }
    }
    Ok(())
}
fn paired_lower_bound(samples: &Value) -> Result<f64, String> {
    let rows = samples
        .as_array()
        .ok_or("Raw benchmark samples must be an array")?;
    if rows.len() < 20 {
        return Err("At least 20 paired independent measurements are required".into());
    }
    let mut differences = Vec::new();
    for row in rows {
        let r = row
            .get("rustMilliseconds")
            .and_then(Value::as_f64)
            .ok_or("Missing Rust measurement")?;
        let a = row
            .get("alternativeMilliseconds")
            .and_then(Value::as_f64)
            .ok_or("Missing alternative measurement")?;
        if !r.is_finite() || !a.is_finite() || r <= 0.0 || a <= 0.0 {
            return Err("Measurements must be finite positive latencies".into());
        }
        differences.push(r - a);
    }
    let n = differences.len() as f64;
    let mean = differences.iter().sum::<f64>() / n;
    let variance = differences.iter().map(|d| (d - mean).powi(2)).sum::<f64>() / (n - 1.0);
    // Conservative t critical value for df >=19, rather than assuming infinite samples.
    Ok(mean - 2.094 * (variance / n).sqrt())
}
fn performance_exception(root: &Path, e: &Value) -> Result<BTreeSet<String>, String> {
    require_text(
        e,
        &[
            "id",
            "language",
            "workload",
            "target",
            "reviewedBy",
            "reviewedAt",
            "recheckWhen",
        ],
    )?;
    let sources = records(e, "sources");
    if sources.is_empty() {
        return Err("Exception needs exact alternative sources".into());
    }
    let mut paths = BTreeSet::new();
    for s in sources {
        checked_file(root, s)?;
        paths.insert(field(s, "path").to_owned());
    }
    let rust = e
        .get("rustImplementation")
        .ok_or("Missing optimized Rust implementation")?;
    if !field(rust, "path").ends_with(".rs") {
        return Err("Comparison needs actual Rust source".into());
    }
    checked_file(root, rust)?;
    let bench = e.get("benchmark").ok_or("Missing benchmark record")?;
    require_text(
        bench,
        &[
            "command",
            "buildSettings",
            "toolchains",
            "warmup",
            "runOrder",
        ],
    )?;
    for key in ["input", "environment", "correctness", "report"] {
        checked_file(
            root,
            bench
                .get(key)
                .ok_or_else(|| format!("Missing benchmark {key} evidence"))?,
        )?;
    }
    let raw = checked_file(
        root,
        bench
            .get("rawSamples")
            .ok_or("Missing raw paired samples")?,
    )?;
    let samples: Value = serde_json::from_slice(&raw).map_err(|e| e.to_string())?;
    if paired_lower_bound(&samples)? <= 0.0 {
        return Err(
            "Recorded comparison does not establish that Rust is slower beyond noise".into(),
        );
    }
    Ok(paths)
}
fn generated_interop(root: &Path, g: &Value) -> Result<String, String> {
    require_text(g, &["generator", "generatorVersion", "command"])?;
    if g.get("containsApplicationLogic") != Some(&Value::Bool(false)) {
        return Err("Generated boundary must declare no application logic".into());
    }
    let sources = records(g, "rustSources");
    if sources.is_empty() {
        return Err("Generated interop needs Rust provenance".into());
    }
    for source in sources {
        if !field(source, "path").ends_with(".rs") {
            return Err("Interop source must be Rust".into());
        }
        checked_file(root, source)?;
    }
    checked_file(root, g)?;
    checked_file(
        root,
        g.get("reproductionEvidence")
            .ok_or("Missing generator reproduction evidence")?,
    )?;
    Ok(field(g, "path").to_string())
}
fn unchanged_inventory(old: &Value, new: &Value) -> Vec<String> {
    let initial: BTreeMap<_, _> = records(old, "legacyInventory")
        .iter()
        .map(|v| (field(v, "path"), v))
        .collect();
    records(new, "legacyInventory")
        .iter()
        .filter_map(|v| {
            let p = field(v, "path");
            match initial.get(p) {
                None => Some(format!("New legacy debt is forbidden: {p}")),
                Some(previous) if previous != &v => {
                    Some(format!("Legacy debt cannot be rebaselined: {p}"))
                }
                _ => None,
            }
        })
        .collect()
}
fn run(root: &Path, base: Option<&str>, strict: bool) -> Result<Value, String> {
    let bytes = file(root, "data/language-exceptions.json")?;
    let registry: Value = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    if registry.get("schemaVersion").and_then(Value::as_u64) != Some(1)
        || field(&registry, "defaultLanguage") != "Rust"
    {
        return Err("Unsupported policy schema or default language; Rust is mandatory".into());
    }
    for key in ["exceptions", "generatedInterop", "legacyInventory"] {
        if !registry.get(key).map(Value::is_array).unwrap_or(false) {
            return Err(format!("{key} must be an array"));
        }
    }
    let mut violations = Vec::<String>::new();
    if let Some(base) = base {
        git(
            root,
            &["rev-parse", "--verify", &format!("{base}^{{commit}}")],
        )?;
        let old = git(
            root,
            &["show", &format!("{base}:data/language-exceptions.json")],
        );
        if let Ok(old) = old {
            let value: Value = serde_json::from_slice(&old).map_err(|e| e.to_string())?;
            violations.extend(unchanged_inventory(&value, &registry));
        } else {
            // First installation: each recorded legacy digest must match the actual base, not a new edit.
            for entry in records(&registry, "legacyInventory") {
                let name = field(entry, "path");
                match git(root, &["show", &format!("{base}:{name}")]) {
                    Ok(content) if digest(&content) == field(entry, "sha256") => {}
                    _ => violations.push(format!(
                        "Initial legacy inventory does not match reviewed base: {name}"
                    )),
                }
            }
        }
    }
    let mut approved = BTreeSet::new();
    for e in records(&registry, "exceptions") {
        match performance_exception(root, e) {
            Ok(paths) => approved.extend(paths),
            Err(err) => violations.push(format!("Exception {}: {err}", field(e, "id"))),
        }
    }
    for g in records(&registry, "generatedInterop") {
        match generated_interop(root, g) {
            Ok(path) => {
                approved.insert(path);
            }
            Err(err) => violations.push(format!("Generated interop {}: {err}", field(g, "path"))),
        }
    }
    let mut legacy = BTreeMap::new();
    for e in records(&registry, "legacyInventory") {
        let name = field(e, "path");
        if !safe_path(name)
            || field(e, "status") != "unapproved-legacy"
            || field(e, "migrationTarget").is_empty()
            || field(e, "sha256").len() != 64
        {
            violations.push(format!("Invalid unresolved legacy record: {name}"));
        }
        if legacy.insert(name, e).is_some() {
            violations.push(format!("Duplicate legacy record: {name}"));
        }
    }
    let mut files = BTreeSet::new();
    for args in [
        &["ls-files", "-z"][..],
        &["ls-files", "--others", "--exclude-standard", "-z"][..],
    ] {
        for name in git(root, args)?
            .split(|b| *b == 0)
            .filter(|s| !s.is_empty())
        {
            files.insert(String::from_utf8(name.to_vec()).map_err(|e| e.to_string())?);
        }
    }
    let mut remaining = Vec::new();
    for name in files {
        if !root.join(&name).exists() {
            continue;
        }
        let content = file(root, &name)?;
        let Some(language) = source_language(&name, &content) else {
            continue;
        };
        if approved.contains(&name) {
            continue;
        }
        if let Some(entry) = legacy.get(name.as_str()) {
            if digest(&content) != field(entry, "sha256") {
                violations.push(format!(
                    "Modified non-Rust source requires migration or measured exception: {name}"
                ));
            }
            remaining.push(json!({"path":name,"language":language,"status":"unapproved-legacy"}));
        } else {
            violations.push(format!(
                "Unapproved non-Rust executable source: {name} ({language})"
            ));
        }
    }
    if strict && !remaining.is_empty() {
        violations.push(format!(
            "Full Rust compliance blocked by {} unresolved legacy files",
            remaining.len()
        ));
    }
    let complete = violations.is_empty() && remaining.is_empty();
    Ok(
        json!({"mode":if strict {"strict"}else{"migration-guard"},"passed":violations.is_empty(),"fullyRustCompliant":complete,"unresolvedLegacyCount":remaining.len(),"unresolvedLegacy":remaining,"violations":violations,"approvedExceptionCount":records(&registry,"exceptions").len()}),
    )
}
fn main() -> ExitCode {
    let mut root = PathBuf::from(".");
    let mut base = None;
    let mut strict = false;
    let mut as_json = false;
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--root" => match args.next() {
                Some(v) => root = PathBuf::from(v),
                None => {
                    eprintln!("--root needs a path");
                    return ExitCode::from(2);
                }
            },
            "--base" => match args.next() {
                Some(v) => base = Some(v),
                None => {
                    eprintln!("--base needs a commit");
                    return ExitCode::from(2);
                }
            },
            "--strict" => strict = true,
            "--json" => as_json = true,
            "--help" | "-h" => {
                println!("inta-policy [--root PATH] [--base COMMIT] [--strict] [--json]");
                return ExitCode::SUCCESS;
            }
            _ => {
                eprintln!("Unknown argument: {arg}");
                return ExitCode::from(2);
            }
        }
    }
    match run(&root, base.as_deref(), strict) {
        Ok(report) => {
            if as_json {
                println!("{}", serde_json::to_string_pretty(&report).unwrap());
            } else {
                println!(
                    "{}: {}",
                    field(&report, "mode"),
                    if report["passed"] == true {
                        "passed"
                    } else {
                        "FAILED"
                    }
                );
                println!(
                    "Unresolved non-Rust legacy files: {}. Full Rust requirement satisfied: {}.",
                    report["unresolvedLegacyCount"], report["fullyRustCompliant"]
                );
                for v in records(&report, "violations") {
                    println!("- {}", v.as_str().unwrap_or(""));
                }
            }
            if report["passed"] == true {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            }
        }
        Err(e) => {
            eprintln!("Policy check failed: {e}");
            ExitCode::from(2)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_paths_only() {
        for p in ["../x", "/x", "foo/../x", "x*.js", "x?", "x\\y", ""] {
            assert!(!safe_path(p), "{p}");
        }
        assert!(safe_path("crates/ui/src/main.rs"));
    }
    #[test]
    fn script_cannot_hide_as_content() {
        assert!(embedded_script("<button onclick=\"f()\">Run</button>"));
        assert!(embedded_script("<script type=module>doWork()</script>"));
        assert!(!embedded_script(
            "<script type=\"application/json\">{}</script><p>Guide</p>"
        ));
    }
    #[test]
    fn only_literal_build_placeholders_are_declarative() {
        assert!(!embedded_script(
            "<script id=\"inta-data\">{{DATA}}</script><script id=\"inta-app\"> {{APP}} </script>"
        ));
        assert!(embedded_script("<script>{{APP}};doWork()</script>"));
        assert!(embedded_script("<script>{{app}}</script>"));
        assert!(embedded_script("<script src=\"other.js\">{{APP}}</script>"));
    }
    #[test]
    fn extensionless_scripts_are_detected() {
        assert_eq!(
            source_language("live/tool", b"#!/usr/bin/python3\nprint('x')"),
            Some("Python")
        );
        assert_eq!(source_language("guide.md", b"# Guide"), None);
    }
    #[test]
    fn noise_is_not_proof() {
        let rows=json!((0..20).map(|i|json!({"rustMilliseconds":if i%2==0 {9.0}else{11.0},"alternativeMilliseconds":10.0})).collect::<Vec<_>>());
        assert!(paired_lower_bound(&rows).unwrap() <= 0.0);
    }
    #[test]
    fn comparison_needs_enough_pairs() {
        assert!(
            paired_lower_bound(&json!([{ "rustMilliseconds":2,"alternativeMilliseconds":1 }]))
                .is_err()
        );
    }
    #[test]
    fn consistent_slower_pairs_are_detected() {
        let rows = json!((0..20)
            .map(|_| json!({"rustMilliseconds":12.0,"alternativeMilliseconds":10.0}))
            .collect::<Vec<_>>());
        assert!(paired_lower_bound(&rows).unwrap() > 0.0);
    }
    #[test]
    fn legacy_hashes_cannot_be_refreshed() {
        let old = json!({"legacyInventory":[{"path":"app.js","sha256":"old"}]});
        let new = json!({"legacyInventory":[{"path":"app.js","sha256":"new"},{"path":"new.py"}]});
        assert_eq!(unchanged_inventory(&old, &new).len(), 2);
        assert!(unchanged_inventory(&old, &json!({"legacyInventory":[]})).is_empty());
    }
}
