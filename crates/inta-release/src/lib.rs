pub mod candidate;
pub mod publication;
pub mod package;
pub mod wiki;
pub mod reconcile;
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs::{self,File},io::{Read,Write},path::Path,process::{Command,Stdio},thread,time::{Duration,Instant}};
use serde_json::Value;
pub type Result<T>=std::result::Result<T,Box<dyn std::error::Error + Send + Sync>>;
pub fn require(value:bool,message:&str)->Result<()>{if value{Ok(())}else{Err(message.into())}}
pub fn sha(bytes:&[u8])->String{format!("{:x}",Sha256::digest(bytes))}
pub fn sha_file(path:&Path)->Result<(String,u64)>{let mut file=File::open(path)?;let mut digest=Sha256::new();let mut total=0;let mut bytes=vec![0u8;1024*1024];loop{let n=file.read(&mut bytes)?;if n==0{break;}digest.update(&bytes[..n]);total+=n as u64;}Ok((format!("{:x}",digest.finalize()),total))}
pub fn read_json(path:&Path)->Result<Value>{Ok(serde_json::from_slice(&fs::read(path)?)?)}
pub fn write_json(path:&Path,value:&Value)->Result<()>{if let Some(parent)=path.parent(){fs::create_dir_all(parent)?;}let mut file=File::create(path)?;serde_json::to_writer_pretty(&mut file,value)?;file.write_all(b"\n")?;Ok(())}
pub fn valid_sha(value:&str,digits:usize)->bool{value.len()==digits&&value.bytes().all(|c|c.is_ascii_digit()||(b'a'..=b'f').contains(&c))}
pub fn valid_repo(value:&str)->bool{let fields:Vec<_>=value.split('/').collect();fields.len()==2&&!fields[0].is_empty()&&!fields[1].is_empty()&&fields[0].as_bytes()[0].is_ascii_alphanumeric()&&fields[0].bytes().all(|c|c.is_ascii_alphanumeric()||c==b'-')&&fields[1]!="."&&fields[1]!=".."&&fields[1].bytes().all(|c|c.is_ascii_alphanumeric()||b"_.-".contains(&c))}
pub fn regular(path:&Path)->Result<Vec<u8>>{let metadata=path.symlink_metadata()?;require(metadata.is_file(),"Evidence/source entries must be regular files")?;Ok(fs::read(path)?)}
pub fn markdown(directory:&Path)->Result<BTreeMap<String,Vec<u8>>>{require(directory.symlink_metadata()?.is_dir(),"Canonical Markdown directory is missing or unsafe")?;let mut rows=BTreeMap::new();for entry in fs::read_dir(directory)?{let path=entry?.path();if path.extension().is_some_and(|ext|ext=="md"){let name=path.file_name().unwrap().to_str().ok_or("Non-UTF8 Markdown filename")?.to_string();rows.insert(name,regular(&path)?);}}require(!rows.is_empty(),"Canonical Markdown directory is empty")?;Ok(rows)}
pub struct Output{pub success:bool,pub code:Option<i32>,pub stdout:Vec<u8>,pub stderr:Vec<u8>}
pub fn run(mut command:Command)->Result<Output>{
    command.stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped());let mut child=command.spawn()?;
    let read=|mut stream:Box<dyn Read+Send>|thread::spawn(move||{let mut bytes=Vec::new();let result=stream.by_ref().take(64*1024*1024+1).read_to_end(&mut bytes);(result,bytes)});
    let out=read(Box::new(child.stdout.take().unwrap()));let err=read(Box::new(child.stderr.take().unwrap()));let start=Instant::now();
    let status=loop{if let Some(status)=child.try_wait()?{break status;}if start.elapsed()>Duration::from_secs(240){let _=child.kill();let _=child.wait();return Err("Command timed out; inspect the current publication phase before retrying".into());}thread::sleep(Duration::from_millis(20));};
    let(ra,stdout)=out.join().map_err(|_|"Output capture failed")?;let(rb,stderr)=err.join().map_err(|_|"Error capture failed")?;ra?;rb?;require(stdout.len()<=64*1024*1024&&stderr.len()<=64*1024*1024,"Command output exceeded evidence limit")?;
    Ok(Output{success:status.success(),code:status.code(),stdout,stderr})
}
pub fn git(root:&Path,args:&[&str])->Result<Vec<u8>>{let mut cmd=Command::new("git");cmd.arg("-C").arg(root).args(args);let output=run(cmd)?;require(output.success,"Git command failed; captured output withheld")?;Ok(output.stdout)}
pub fn git_text(root:&Path,args:&[&str])->Result<String>{Ok(String::from_utf8(git(root,args)?)?.trim().to_string())}
pub fn network()->ureq::Agent{ureq::AgentBuilder::new().timeout_connect(Duration::from_secs(15)).timeout_read(Duration::from_secs(60)).timeout_write(Duration::from_secs(30)).user_agent("INTA-Rust-Publication/0.2.0").build()}
pub fn fetch(url:&str,limit:u64)->Result<Vec<u8>>{let response=network().get(url).set("Cache-Control","no-cache").call()?;let mut bytes=Vec::new();response.into_reader().take(limit+1).read_to_end(&mut bytes)?;require(bytes.len()as u64<=limit,"Response exceeds evidence size limit")?;Ok(bytes)}
pub fn stem(commit:&str)->Result<String>{require(valid_sha(commit,40),"Complete source commit required")?;Ok(format!("INTA-workspace-{}",&commit[..12]))}
