use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OperationRequest {
    pub manager: String,
    pub action: String,
    #[serde(default)]
    pub package_id: String,
    #[serde(default)]
    pub confirmed: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BatchRequest {
    pub operations: Vec<OperationRequest>,
    #[serde(default)]
    pub confirmed: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationRecord {
    pub id: String,
    pub manager: String,
    pub action: String,
    pub package_id: String,
    pub state: String,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub exit_code: Option<i32>,
    pub output: String,
    pub reboot_likely: bool,
    pub error: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstalledApp {
    pub name: String,
    pub version: Option<String>,
    pub publisher: Option<String>,
    pub manager: String,
    pub package_id: Option<String>,
    pub scope: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagerPreview {
    pub manager: String,
    pub command: String,
    pub exit_code: i32,
    pub output: String,
    pub checked_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryResult {
    pub collected_at: DateTime<Utc>,
    pub platform: String,
    pub applications: Vec<InstalledApp>,
    pub managers: Vec<String>,
    pub update_previews: Vec<ManagerPreview>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandSpec {
    pub program: String,
    pub arguments: Vec<String>,
}

impl CommandSpec {
    pub fn new(program: &str, arguments: &[&str]) -> Self {
        Self {
            program: program.into(),
            arguments: arguments.iter().map(|v| (*v).into()).collect(),
        }
    }
    pub fn display(&self) -> String {
        format!("{} {}", self.program, self.arguments.join(" "))
    }
}

#[derive(Clone, Debug)]
pub struct ProcessResult {
    pub exit_code: i32,
    pub output: String,
}

#[derive(Debug)]
pub struct Verification {
    pub verified: bool,
    pub output: String,
}
