use crate::{git_text,markdown,network,require,run,sha,valid_repo,valid_sha,write_json,Result};
use base64::{engine::general_purpose::STANDARD,Engine};
use serde_json::{json,Value};
use std::{collections::BTreeMap,env,fs,path::Path,process::Command};

struct WikiGit{environment:BTreeMap<String,String>}
impl WikiGit{
    fn new(repository:&str,token:&str)->Self{
        let mut environment:BTreeMap<String,String>=env::vars().filter(|(key,_)|!key.starts_with("GIT_")&&key!="GITHUB_TOKEN"&&key!="GH_TOKEN").collect();
        let null=if cfg!(windows){"NUL"}else{"/dev/null"};
        for(key,value)in[("GIT_TERMINAL_PROMPT","0"),("GIT_CONFIG_NOSYSTEM","1"),("GIT_CONFIG_GLOBAL",null),("GIT_LITERAL_PATHSPECS","1")]{environment.insert(key.into(),value.into());}
        let url=format!("https://github.com/{repository}.wiki.git");let authorization=STANDARD.encode(format!("x-access-token:{token}"));
        let config=[(format!("http.{url}.extraheader"),format!("Authorization: Basic {authorization}")),("http.followRedirects".into(),"false".into()),("credential.helper".into(),String::new()),("core.hooksPath".into(),null.into()),("core.autocrlf".into(),"false".into()),("core.safecrlf".into(),"true".into()),("commit.gpgsign".into(),"false".into()),("protocol.allow".into(),"never".into()),("protocol.https.allow".into(),"always".into())];
        environment.insert("GIT_CONFIG_COUNT".into(),config.len().to_string());for(index,(key,value))in config.into_iter().enumerate(){environment.insert(format!("GIT_CONFIG_KEY_{index}"),key);environment.insert(format!("GIT_CONFIG_VALUE_{index}"),value);}Self{environment}
    }
    fn git(&self,report:&mut Value,phase:&str,cwd:&Path,args:&[&str])->Result<Vec<u8>>{
        report["phase"]=json!(phase);let mut command=Command::new("git");command.args(args).current_dir(cwd).env_clear().envs(&self.environment);
        let result=run(command).map_err(|_|"Git could not complete; captured details withheld")?;
        require(result.success,&format!("Git exited with status {:?}; captured output withheld",result.code))?;Ok(result.stdout)
    }
    fn head(&self,report:&mut Value,cwd:&Path,phase:&str)->Result<String>{let value=String::from_utf8(self.git(report,phase,cwd,&["rev-parse","--verify","HEAD"])?)?.trim().to_string();require(valid_sha(&value,40)||valid_sha(&value,64),"Invalid Git commit identity")?;Ok(value)}
}
fn current_main(report:&mut Value,repository:&str,token:&str)->Result<()>{report["phase"]=json!("check-current-main");let reply=network().get(&format!("https://api.github.com/repos/{repository}/git/ref/heads/main")).set("Authorization",&format!("Bearer {token}")).set("Accept","application/vnd.github+json").call().map_err(|_|"Could not read current main; credential details withheld")?;let value:Value=reply.into_json().map_err(|_|"Invalid current-main response")?;report["current_main"]=value["object"]["sha"].clone();require(report["current_main"]==report["source_head"],"A newer main revision exists; stale Wiki publication was stopped")}

pub fn publish(root:&Path,report:&mut Value)->Result<()>{
    let repository=env::var("GITHUB_REPOSITORY")?;let token=env::var("GITHUB_TOKEN")?;require(valid_repo(&repository)&&!token.is_empty()&&!token.chars().any(char::is_whitespace),"A valid repository and nonempty GITHUB_TOKEN are required")?;
    report["repository"]=json!(repository);report["phase"]=json!("snapshot-source");let expected=markdown(&root.join("wiki"))?;
    report["files"]=json!(expected.iter().map(|(name,bytes)|json!({"name":name,"bytes":bytes.len(),"sha256":sha(bytes)})).collect::<Vec<_>>());report["expected_pages"]=json!(expected.len());
    let git=WikiGit::new(&repository,&token);let source=git.head(report,root,"read-source-head")?;report["source_head"]=json!(source);report["changed_pages"]=json!([]);report["changes"]=json!([]);report["push_status"]=json!("not-needed");current_main(report,&repository,&token)?;
    let temporary=tempfile::tempdir()?;let work=temporary.path().join("publish");let readback=temporary.path().join("readback");let url=format!("https://github.com/{repository}.wiki.git");
    git.git(report,"clone-wiki",temporary.path(),&["clone","--quiet","--no-tags","--depth=1","--single-branch",&url,work.to_str().ok_or("Non-UTF8 temporary path")?])?;
    report["base_head"]=json!(git.head(report,&work,"read-wiki-head")?);let branch_ref=String::from_utf8(git.git(report,"read-wiki-branch",&work,&["symbolic-ref","--quiet","HEAD"])?)?.trim().to_string();
    require(branch_ref.starts_with("refs/heads/"),"Wiki checkout has no ordinary branch")?;git.git(report,"validate-wiki-branch",&work,&["check-ref-format",&branch_ref])?;let branch=branch_ref.trim_start_matches("refs/heads/");report["branch"]=json!(branch);
    let preserved:Vec<_>=fs::read_dir(&work)?.filter_map(std::result::Result::ok).map(|e|e.file_name().to_string_lossy().into_owned()).filter(|name|name.ends_with(".md")&&!expected.contains_key(name)).collect();report["preserved_live_pages"]=json!(preserved);
    report["phase"]=json!("copy-canonical-pages");let mut changed=Vec::new();let mut changes=Vec::new();
    for(name,content)in &expected{
        let destination=work.join(name);let before=match destination.symlink_metadata(){Ok(metadata)=>{require(metadata.is_file(),"Canonical destination is not a regular file")?;Some(fs::read(&destination)?)},Err(error)if error.kind()==std::io::ErrorKind::NotFound=>None,Err(error)=>return Err(error.into())};
        if before.as_ref()!=Some(content){fs::write(&destination,content)?;changed.push(name.as_str());changes.push(json!({"name":name,"before_sha256":before.as_ref().map(|bytes|sha(bytes)),"after_sha256":sha(content)}));}
    }
    report["changed_pages"]=json!(changed);report["changes"]=json!(changes);
    if !changed.is_empty(){let mut args=vec!["add","--"];args.extend(&changed);git.git(report,"stage-canonical-pages",&work,&args)?;}
    for(name,content)in &expected{let staged=git.git(report,"verify-staged-bytes",&work,&["cat-file","blob",&format!(":0:{name}")])?;if staged!=*content{report["staged_mismatch"]=json!({"name":name,"expected_sha256":sha(content),"actual_sha256":sha(&staged)});return Err("Staged Markdown differs from canonical bytes".into());}}
    if !changed.is_empty(){git.git(report,"commit-wiki",&work,&["-c","user.name=github-actions[bot]","-c","user.email=41898282+github-actions[bot]@users.noreply.github.com","commit","--quiet","-m",&format!("Sync canonical Wiki from {}",&source[..12])])?;report["published_head"]=json!(git.head(report,&work,"read-publish-head")?);current_main(report,&repository,&token)?;report["push_status"]=json!("attempted");git.git(report,"push-wiki",&work,&["push","--porcelain","origin",&format!("HEAD:{branch_ref}")])?;report["push_status"]=json!("confirmed");}else{report["published_head"]=report["base_head"].clone();}
    git.git(report,"clone-readback",temporary.path(),&["clone","--quiet","--no-tags","--depth=1","--single-branch",&format!("--branch={branch}"),&url,readback.to_str().ok_or("Non-UTF8 temporary path")?])?;report["verified_head"]=json!(git.head(report,&readback,"read-verified-head")?);report["phase"]=json!("verify-live-bytes");
    let mut mismatches=Vec::new();for(name,content)in &expected{let path=readback.join(name);let actual=crate::regular(&path).ok();if actual.as_ref()!=Some(content){mismatches.push(json!({"name":name,"expected_sha256":sha(content),"actual_sha256":actual.as_ref().map(|bytes|sha(bytes))}));}}
    if !mismatches.is_empty(){report["mismatches"]=json!(mismatches);return Err("Fresh Wiki clone differs from canonical bytes".into());}
    report["verified_pages"]=json!(expected.len());report["status"]=json!("success");report["phase"]=json!("complete");Ok(())
}
pub fn run_publish(root:&Path,path:Option<&Path>)->Result<Value>{let mut report=json!({"schemaVersion":1,"status":"failed","phase":"initialize"});if let Err(error)=publish(root,&mut report){report["error"]=json!(error.to_string());}if let Some(path)=path{write_json(path,&report)?;}Ok(report)}

#[cfg(test)]mod tests{use super::*;
#[test]fn rejects_unsafe_repository_and_symlink_source(){for repository in["../repo","x/y/z","x/..","x/a b","-x/y","https://github.com/x/y"]{assert!(!valid_repo(repository));}let temporary=tempfile::tempdir().unwrap();#[cfg(unix)]{std::os::unix::fs::symlink("/tmp",temporary.path().join("wiki")).unwrap();assert!(markdown(&temporary.path().join("wiki")).is_err());}}
#[test]fn canonical_snapshot_preserves_exact_bytes_and_names(){let temporary=tempfile::tempdir().unwrap();fs::write(temporary.path().join("Home.md"),b"# Home\r\n\n").unwrap();fs::write(temporary.path().join("Guide.md"),b"# Guide\n").unwrap();let rows=markdown(temporary.path()).unwrap();assert_eq!(rows["Home.md"],b"# Home\r\n\n");assert_eq!(rows.len(),2);}
#[test]fn wiki_transport_scopes_credentials_and_disables_hooks(){let config=WikiGit::new("owner/repo","test-token");assert_eq!(config.environment["GIT_TERMINAL_PROMPT"],"0");assert!(!config.environment.contains_key("GITHUB_TOKEN"));assert!(!config.environment.contains_key("GH_TOKEN"));assert_eq!(config.environment["GIT_CONFIG_KEY_0"],"http.https://github.com/owner/repo.wiki.git.extraheader");assert!(config.environment["GIT_CONFIG_VALUE_0"].starts_with("Authorization: Basic "));assert!(config.environment.values().any(|v|v=="core.hooksPath"));}
}
