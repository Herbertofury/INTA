use serde_json::{Value, json};

#[derive(Clone, Copy, PartialEq)]
pub enum Op {
    Search,
    Inspect,
    Memory,
    Power,
    Startup,
    HashFile,
    HashText,
}
impl Op {
    pub const ALL: [Self; 7] = [
        Self::Search,
        Self::Inspect,
        Self::Memory,
        Self::Power,
        Self::Startup,
        Self::HashFile,
        Self::HashText,
    ];
    pub fn id(self) -> &'static str {
        match self {
            Self::Search => "tools.search",
            Self::Inspect => "tools.inspect",
            Self::Memory => "system.memory",
            Self::Power => "system.power",
            Self::Startup => "startup.counts",
            Self::HashFile => "file.sha256",
            Self::HashText => "text.sha256",
        }
    }
    pub fn find(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.id() == s)
    }
    pub fn forge(self) -> bool {
        self != Self::HashFile
    }
}
fn object(fields: Value, required: &[&str]) -> Value {
    json!({"type":"object","properties":fields,"required":required,"additionalProperties":false})
}
fn number() -> Value {
    json!({"type":"integer","minimum":0})
}
fn string() -> Value {
    json!({"type":"string"})
}
fn array(item: Value) -> Value {
    json!({"type":"array","items":item})
}
fn digest() -> Value {
    json!({"type":"string","pattern":"^[0-9a-f]{64}$"})
}
pub fn describe(id: &str) -> Option<Value> {
    let op = Op::find(id)?;
    let empty = || object(json!({}), &[]);
    let any_record = || json!({"type":"object"});
    let (title, description, input, output, backend, platform, ids) = match op {
        Op::Search => (
            "Search catalog",
            "Search the merged canonical and workstation records by text.",
            object(
                json!({"query":{"type":"string","maxLength":240},"limit":{"type":"integer","minimum":1,"maximum":500,"default":50}}),
                &["query"],
            ),
            object(
                json!({"total":number(),"returned":number(),"items":array(any_record())}),
                &["total", "returned", "items"],
            ),
            "embedded-catalog",
            "any",
            vec![],
        ),
        Op::Inspect => (
            "Inspect catalog tool",
            "Read one canonical record, its provenance, prior coverage and separately registered native operations.",
            object(
                json!({"id":{"type":"string","pattern":"^[a-z0-9][a-z0-9-]{0,99}$"}}),
                &["id"],
            ),
            object(
                json!({"tool":any_record(),"origin":string(),"coverage":any_record(),"publisherAccess":{"type":["object","null"]},"operations":array(any_record())}),
                &[
                    "tool",
                    "origin",
                    "coverage",
                    "publisherAccess",
                    "operations",
                ],
            ),
            "embedded-catalog",
            "any",
            vec![],
        ),
        Op::Memory => (
            "Read physical memory",
            "Read total and available physical bytes and memory load from Windows.",
            empty(),
            object(
                json!({"totalPhysicalBytes":number(),"availablePhysicalBytes":number(),"loadPercent":{"type":"integer","minimum":0,"maximum":100}}),
                &[
                    "totalPhysicalBytes",
                    "availablePhysicalBytes",
                    "loadPercent",
                ],
            ),
            "Windows GlobalMemoryStatusEx",
            "windows-x64",
            vec![],
        ),
        Op::Power => (
            "Inventory power plans",
            "Read available power-plan IDs and the active selection. Confirm the selection stays stable during enumeration.",
            empty(),
            object(
                json!({"activeId":string(),"planIds":array(string()),"count":number()}),
                &["activeId", "planIds", "count"],
            ),
            "Windows PowerEnumerate and PowerGetActiveScheme",
            "windows-x64",
            vec!["powercfg"],
        ),
        Op::Startup => (
            "Count sign-in registrations",
            "Read counts for Run and RunOnce from the documented user and machine registry views. Entry names and command lines stay private.",
            empty(),
            object(
                json!({"sources":array(object(json!({"scope":string(),"view":string(),"key":string(),"state":string(),"registered":number()}),&["scope","view","key","state","registered"])),"registeredValues":number()}),
                &["sources", "registeredValues"],
            ),
            "Windows registry metadata",
            "windows-x64",
            vec![],
        ),
        Op::HashFile => (
            "Verify a local file",
            "Hash a selected regular local file up to 256 MiB. The result contains its byte count and SHA-256.",
            object(
                json!({"path":{"type":"string","maxLength":1024,"description":"Absolute local filesystem path"}}),
                &["path"],
            ),
            object(
                json!({"sha256":digest(),"bytes":number()}),
                &["sha256", "bytes"],
            ),
            "SHA-256 over a held regular file",
            "windows-x64",
            vec![],
        ),
        Op::HashText => (
            "Verify UTF-8 text",
            "Hash the exact UTF-8 bytes of up to 8 KiB of input text without line-ending or Unicode normalization.",
            object(
                json!({"text":{"type":"string","maxLength":8192}}),
                &["text"],
            ),
            object(
                json!({"sha256":digest(),"bytes":number()}),
                &["sha256", "bytes"],
            ),
            "SHA-256",
            "any",
            vec![],
        ),
    };
    Some(
        json!({"id":id,"title":title,"description":description,"version":1,"effect":"read-only","platform":platform,
        "requiresElevation":false,"network":false,"remoteDispatch":false,"forgeEligible":op.forge(),"backend":backend,"catalogIds":ids,
        "inputSchema":input,"outputSchema":output}),
    )
}
pub fn all() -> Vec<Value> {
    Op::ALL
        .into_iter()
        .map(|op| describe(op.id()).expect("registered operation"))
        .collect()
}
