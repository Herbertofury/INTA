//! Complete source-owned INTA content validation and generation in Rust.
//! No interpreter, subprocess, or historical non-Rust generator is used.
use serde_json::Value;
use std::{collections::{BTreeMap, BTreeSet},path::Path};
pub mod validate;
pub mod markdown;
pub mod online;
mod build;
pub use build::build_outputs;
pub use validate::*;
pub type Result<T> = std::result::Result<T,String>;
pub type Outputs = BTreeMap<String,String>;
pub const REPO:&str="https://github.com/Herbertofury/INTA";
pub const WIKI:&str="https://github.com/Herbertofury/INTA/wiki";
pub const HUB:&str="https://herbertofury.github.io/INTA/";
pub const QUICK_START_START:&str="<!-- INTA:QUICK-START:START -->";
pub const QUICK_START_END:&str="<!-- INTA:QUICK-START:END -->";
pub const WIKI_MIRRORS:&[(&str,&str)]=&[
("Start-Here","start-here.md"),("Maintaining-the-Hub","maintaining-the-hub.md"),("Rescue-Media","rescue-media.md"),("Technician-Handoff","technician-handoff.md"),("Knowledge-Standards","knowledge-standards.md"),("Contribute","contribute.md"),("Quick-Commands","quick-commands.md"),("Command-Center","command-center.md"),("Updates-and-Maintenance","updates-center.md"),("Ecosystem-Development","ecosystem-development.md"),("Templates","contribution-template.md"),("Continuous-Improvement","continuous-improvement.md"),("Methods","methods.md"),("Decision-Desk","decision-desk.md"),("Interface-Design","interface-design.md"),("Service-Evidence","service-evidence.md"),("Tool-Selection","tool-selection.md"),("Free-Tool-Picks","free-tool-picks.md"),("Current-Tool-Choices","reference/scenario-tool-review-2026-11-26.md"),("Malware-Tool-Choices","reference/malware-tool-review-2026-11-26.md"),("Knowledge-Organization","knowledge-organization.md"),("Research-Updates","reference/catalog-expansion-2026-11-26.md"),("Online-Tool-Review","reference/online-tools-review-2026-11-26.md")];
pub fn category_name(category:&str)->&str {match category {"malware"=>"Malware removal & endpoint protection","remote"=>"Remote support","windows"=>"Windows diagnosis & repair","storage"=>"Storage, recovery & backup","hardware"=>"Hardware diagnostics","network"=>"Networking","boot"=>"Bootable media","productivity"=>"Technician productivity","ai"=>"AI assistance","security"=>"Advanced defensive security","online"=>"Online website tools",_=>category}}
pub fn s<'a>(v:&'a Value,key:&str)->&'a str {v[key].as_str().unwrap_or("")}
pub fn a(v:&Value)->&[Value] {v.as_array().map(Vec::as_slice).unwrap_or(&[])}
pub fn t(v:&Value)->&str {v.as_str().unwrap_or("")}
pub fn truth(v:&Value)->bool {match v {Value::Null=>false,Value::Bool(b)=>*b,Value::String(s)=>!s.is_empty(),Value::Array(v)=>!v.is_empty(),Value::Object(v)=>!v.is_empty(),Value::Number(n)=>n.as_f64()!=Some(0.0)}}
pub fn ids(v:&[Value],key:&str)->BTreeSet<String>{v.iter().map(|v|s(v,key).to_string()).collect()}
pub fn index(v:&[Value],key:&str)->Value {Value::Object(v.iter().map(|v|(s(v,key).to_string(),v.clone())).collect())}
pub fn read_json(root:&Path,path:&str)->Result<Value>{serde_json::from_slice(&std::fs::read(root.join(path)).map_err(|e|format!("{path}: {e}"))?).map_err(|e|format!("{path}: {e}"))}
pub fn records(value:&Value,key:&str)->Vec<Value>{if value.is_array(){a(value).to_vec()}else{a(&value[key]).to_vec()}}
pub fn slug(value:&str)->String {regex::Regex::new("[^a-z0-9]+").unwrap().replace_all(&value.to_lowercase(),"-").trim_matches('-').to_string()}
pub fn display(v:&Value)->String {match v {Value::String(s)=>s.clone(),Value::Array(v)=>v.iter().map(display).collect::<Vec<_>>().join(", "),Value::Null=>"None".into(),Value::Bool(b)=>if *b {"True"}else{"False"}.into(),_=>v.to_string()}}
pub fn cell(v:&Value)->String {display(v).replace('|',"\\|").replace('\n'," ")}
pub fn cs(v:&str)->String {v.replace('|',"\\|").replace('\n'," ")}
#[macro_export]
macro_rules! lines {($($x:expr),* $(,)?)=>{vec![$($x.to_string()),*]};}
pub fn sha(bytes:&[u8])->String {use sha2::{Sha256,Digest};format!("{:x}",Sha256::digest(bytes))}
/// Python's historical sorted JSON hashes use a space after commas and colons.
/// Preserve that wire format so the port does not rewrite content identities.
pub fn canonical_json(v:&Value)->String {match v {Value::Array(a)=>format!("[{}]",a.iter().map(canonical_json).collect::<Vec<_>>().join(", ")),Value::Object(m)=>{let mut pairs:Vec<_>=m.iter().collect();pairs.sort_by_key(|(k,_)|*k);format!("{{{}}}",pairs.into_iter().map(|(k,v)|format!("{}: {}",serde_json::to_string(k).unwrap(),canonical_json(v))).collect::<Vec<_>>().join(", "))},_=>serde_json::to_string(v).unwrap()}}
