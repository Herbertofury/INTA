mod baseline;
pub use baseline::{BaselineDestination, BaselineObservation};
mod catalog;
mod files;
mod forge;
mod operations;
mod registry;
mod strict_json;

use serde_json::{Value, json};
use std::path::Path;

type Result<T> = std::result::Result<T, Error>;
#[derive(Debug)]
pub struct Error {
    code: &'static str,
    message: String,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl std::error::Error for Error {}
fn err(code: &'static str, message: impl Into<String>) -> Error {
    Error {
        code,
        message: message.into(),
    }
}
fn invalid(message: impl Into<String>) -> Error {
    err("invalid-input", message)
}
fn expect(condition: bool, message: &'static str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(invalid(message))
    }
}
fn unknown() -> Error {
    err(
        "unknown-operation",
        "Unknown capability. Use discover to see supported operations.",
    )
}
fn dispatch(args: &[&str]) -> Result<Value> {
    let catalog = catalog::Catalog::load()?;
    match args {
        ["discover"] => Ok(
            json!({"product":"INTA Capabilities","version":env!("CARGO_PKG_VERSION"),"catalogCount":catalog.len(),
            "operations":registry::all(),"forge":{"schemaVersion":1,"operations":registry::all().into_iter().filter(|v|v["forgeEligible"]==true).map(|v|v["id"].clone()).collect::<Vec<_>>()},
            "executionScope":"local","network":false,"remoteDispatch":false}),
        ),
        ["tools", "search", q] => catalog.search(q, 50),
        ["tools", "search", q, n] => catalog.search(
            q,
            n.parse()
                .map_err(|_| invalid("Limit must be an integer from 1 to 500."))?,
        ),
        ["tools", "inspect", id] => catalog.inspect(id),
        ["schema", id] => registry::describe(id).ok_or_else(unknown),
        ["run", id] => operations::run(&catalog, id, json!({})),
        ["run", id, "--input", path] => operations::run(
            &catalog,
            id,
            strict_json::parse(&files::read(Path::new(path), 64 * 1024)?)?,
        ),
        ["forge", "schema"] => Ok(forge::schema()),
        ["forge", "verify", path] => forge::verify_file(&catalog, Path::new(path)),
        ["forge", "import", path, "--store", store] => {
            forge::import(&catalog, Path::new(path), Path::new(store))
        }
        ["forge", "run", id, "--store", store] => forge::run_stored(&catalog, id, Path::new(store)),
        ["--help"] | ["help"] | [] => Ok(
            json!({"commands":["discover","tools search QUERY [LIMIT]","tools inspect ID","schema OP","run OP [--input FILE]","forge schema","forge verify MANIFEST","forge import MANIFEST --store DIRECTORY","forge run ID --store DIRECTORY"],"output":"JSON","exitCodes":{"0":"success","1":"unavailable or failed observation","2":"refused request"}}),
        ),
        ["--version"] => Ok(json!({"version":env!("CARGO_PKG_VERSION"),"schemaVersion":1})),
        _ => Err(invalid("Unsupported arguments. Use --help.")),
    }
}
pub fn invoke(arguments: impl Iterator<Item = std::ffi::OsString>) -> (i32, Value) {
    let args: std::result::Result<Vec<_>, _> = arguments.map(|v| v.into_string()).collect();
    let result = args
        .map_err(|_| invalid("Arguments must be Unicode."))
        .and_then(|args| {
            expect(
                args.len() <= 8 && args.iter().all(|s| s.len() <= 8192),
                "Arguments exceed the supported bounds.",
            )?;
            let mut words: Vec<_> = args.iter().map(String::as_str).collect();
            if words.first() == Some(&"--json") {
                words.remove(0);
            }
            dispatch(&words)
        });
    match result {
        Ok(value) => (0, json!({"schemaVersion":1,"ok":true,"result":value})),
        Err(e) => (
            if matches!(e.code, "unavailable" | "io" | "incomplete") {
                1
            } else {
                2
            },
            json!({"schemaVersion":1,"ok":false,"error":{"code":e.code,"message":e.message}}),
        ),
    }
}
