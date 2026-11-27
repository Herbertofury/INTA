use crate::{fetch,git,git_text,markdown,read_json,regular,require,sha,sha_file,stem,write_json,Result};
use serde_json::{json,Value};
use sha1::{Digest,Sha1};
use std::{collections::{BTreeMap,BTreeSet},env,fs::{self,File,OpenOptions},io::{Read,Write},path::{Path,PathBuf},thread,time::Duration};
use zip::{write::SimpleFileOptions,CompressionMethod,ZipArchive,ZipWriter};

pub fn validate_evidence(root:&Path,commit:&str)->Result<(Value,Value,Value,Vec<u8>,BTreeMap<String,Vec<u8>>)>{
    let runtime=read_json(&root.join("reports/online-runtime/online-runtime.json"))?;let wiki=read_json(&root.join("reports/wiki-publish.json"))?;let build=read_json(&root.join("site/build-manifest.json"))?;let html=regular(&root.join("site/index.html"))?;
    require(runtime["status"]=="passed"&&runtime["sha256"]==sha(&html),"Browser evidence does not cover this exact HTML")?;
    require(runtime["javascriptErrors"].as_array().is_some_and(Vec::is_empty)&&runtime["externalRequests"].as_array().is_some_and(Vec::is_empty),"Runtime evidence reports an error, an unexpected request or missing fields")?;
    let expected=markdown(&root.join("wiki"))?;require(wiki["status"]=="success"&&wiki["source_head"]==commit,"Wiki readback does not cover this source commit")?;
    require(wiki["expected_pages"]==expected.len()&&wiki["verified_pages"]==expected.len(),"Wiki readback is incomplete")?;
    let rows=wiki["files"].as_array().ok_or("Missing Wiki file inventory")?;let reported:BTreeMap<_,_>=rows.iter().filter_map(|row|row["name"].as_str().map(|name|(name,row))).collect();
    require(reported.len()==rows.len()&&reported.len()==expected.len()&&expected.iter().all(|(name,bytes)|reported.get(name.as_str()).is_some_and(|row|row["sha256"]==sha(bytes))),"Wiki hashes or file inventory differ from source")?;
    Ok((runtime,wiki,build,html,expected))
}
pub fn verify_deployment(root:&Path,commit:&str)->Result<Value>{
    let mut checks=serde_json::Map::new();for name in["index.html","manifest.webmanifest","sw.js","inta-app-icon.svg","runtime-provenance.json"]{
        let expected=regular(&root.join("site").join(name))?;let url=format!("https://herbertofury.github.io/INTA/{}?verify={commit}",if name=="index.html"{""}else{name});let expected_hash=sha(&expected);let mut actual=None;
        for attempt in 0..6{if let Ok(bytes)=fetch(&url,(expected.len()as u64).saturating_add(65536)){let digest=sha(&bytes);if digest==expected_hash{actual=Some(digest);break;}}if attempt<5{thread::sleep(Duration::from_secs(10));}}
        require(actual.as_deref()==Some(&expected_hash),&format!("Deployed bytes do not match verified source: {name}"))?;checks.insert(name.into(),json!({"url":url,"sha256":expected_hash,"bytes":expected.len()}));
    }Ok(Value::Object(checks))
}

pub fn archive(root:&Path,output:&Path,deployment:Value)->Result<Value>{
    let commit=git_text(root,&["rev-parse","HEAD"])?;let tree=git_text(root,&["rev-parse","HEAD^{tree}"])?;
    require(env::var("SOURCE_COMMIT").unwrap_or_else(|_|commit.clone())==commit,"Checkout differs from validated commit")?;
    require(git(root,&["diff","--name-only",&commit,"--",".",":(exclude)reports/**",":(exclude)site/validation/**"])?.is_empty(),"Tracked source changed after validation")?;
    let(runtime,wiki,build,html,expected)=validate_evidence(root,&commit)?;fs::create_dir_all(output)?;let output=output.canonicalize()?;let stem=stem(&commit)?;
    let source_zip=output.join(format!("{stem}-source.zip"));let offline=output.join(format!("{stem}-offline.html"));let manifest_path=output.join(format!("{stem}-manifest.json"));let checksums=output.join(format!("{stem}-SHA256SUMS.txt"));
    require([&source_zip,&offline,&manifest_path,&checksums].iter().all(|p|!p.exists()),"Release artifacts already exist")?;let archive_root=format!("INTA-{}/",&commit[..12]);
    git(root,&["archive","--format=zip","-9",&format!("--prefix={archive_root}"),&format!("--output={}",source_zip.display()),&commit])?;
    let listing=git(root,&["ls-tree","-rz","--full-tree",&commit])?;let mut tracked=BTreeMap::new();
    for row in listing.split(|byte|*byte==0).filter(|r|!r.is_empty()){
        let separator=row.iter().position(|byte|*byte==b'\t').ok_or("Invalid Git tree record")?;let metadata=std::str::from_utf8(&row[..separator])?.split_whitespace().collect::<Vec<_>>();require(metadata.len()==3&&metadata[1]=="blob","Unresolved submodule cannot enter complete source release")?;let path=std::str::from_utf8(&row[separator+1..])?.to_string();tracked.insert(path,json!({"mode":metadata[0],"gitBlob":metadata[2]}));
    }
    let mut inventory=Vec::new();let mut actual=BTreeMap::new();{
        let mut archive=ZipArchive::new(File::open(&source_zip)?)?;
        for index in 0..archive.len(){let mut entry=archive.by_index(index)?;if entry.is_dir(){continue;}require(entry.name().starts_with(&archive_root),"Unexpected source archive prefix")?;let name=entry.name()[archive_root.len()..].to_string();require(tracked.contains_key(&name)&&!actual.contains_key(&name),"Unexpected or duplicate source member")?;let mut bytes=Vec::new();entry.read_to_end(&mut bytes)?;let mut blob=Sha1::new();blob.update(format!("blob {}\0",bytes.len()));blob.update(&bytes);require(format!("{:x}",blob.finalize())==tracked[&name]["gitBlob"],&format!("Archived source differs from Git blob: {name}"))?;let digest=sha(&bytes);actual.insert(name.clone(),digest.clone());inventory.push(json!({"path":name,"bytes":bytes.len(),"sha256":digest,"mode":tracked[&name]["mode"],"gitBlob":tracked[&name]["gitBlob"]}));}
    }
    require(actual.len()==tracked.len(),"Archive omitted committed source")?;let generated=build["files"].as_object().ok_or("Build manifest file inventory missing")?;require(generated.iter().all(|(name,digest)|actual.get(name).is_some_and(|value|digest==value)),"Generated build manifest differs from committed source")?;
    let mut evidence_paths=vec![root.join("reports/wiki-publish.json")];let mut runtime_paths=fs::read_dir(root.join("reports/online-runtime"))?.map(|entry|entry.map(|e|e.path())).collect::<std::result::Result<Vec<_>,_>>()?;runtime_paths.sort();evidence_paths.extend(runtime_paths);
    let options=SimpleFileOptions::default().compression_method(CompressionMethod::Deflated).unix_permissions(0o644);let mut evidence=Vec::new();let mut zip=ZipWriter::new_append(OpenOptions::new().read(true).write(true).open(&source_zip)?)?;
    for path in evidence_paths{let bytes=regular(&path)?;let name=path.strip_prefix(root)?.to_string_lossy().replace('\\',"/");zip.start_file(format!("verification/{name}"),options)?;zip.write_all(&bytes)?;evidence.push(json!({"path":name,"bytes":bytes.len(),"sha256":sha(&bytes)}));}
    let counters:serde_json::Map<_,_>=build.as_object().ok_or("Invalid build manifest")?.iter().filter(|(_,value)|value.is_u64()).map(|(key,value)|(key.clone(),value.clone())).collect();
    let mut manifest=json!({"schemaVersion":1,"sourceCommit":commit,"sourceTree":tree,"sourceUrl":format!("https://github.com/Herbertofury/INTA/tree/{commit}"),"sourceFiles":tracked.len(),"wikiFiles":expected.len(),"htmlSha256":sha(&html),"browser":runtime["browser"],"runtimeStatus":runtime["status"],"wikiVerifiedHead":wiki["verified_head"],"wikiReadbackStatus":wiki["status"],"catalogCounts":counters,"files":inventory,"evidence":evidence,"deploymentChecks":deployment,"validationRun":env::var("VALIDATION_RUN_URL").unwrap_or_default(),"publicationRun":format!("{}/{}/actions/runs/{}",env::var("GITHUB_SERVER_URL").unwrap_or_else(|_|"https://github.com".into()),env::var("GITHUB_REPOSITORY").unwrap_or_else(|_|"Herbertofury/INTA".into()),env::var("GITHUB_RUN_ID").unwrap_or_default())});
    zip.start_file("verification/package-manifest.json",options)?;zip.write_all(&serde_json::to_vec_pretty(&manifest)?)?;zip.finish()?.sync_all()?;
    {let mut archive=ZipArchive::new(File::open(&source_zip)?)?;let mut names=BTreeSet::new();for index in 0..archive.len(){let mut entry=archive.by_index(index)?;require(names.insert(entry.name().to_string()),"ZIP contains duplicate entries")?;let mut bytes=Vec::new();entry.read_to_end(&mut bytes)?;if entry.name()==format!("{archive_root}site/index.html"){require(bytes==html,"Archived offline app differs")?;}}}
    fs::write(&offline,&html)?;let mut artifacts=serde_json::Map::new();for path in[&source_zip,&offline]{let(hash,bytes)=sha_file(path)?;artifacts.insert(path.file_name().unwrap().to_string_lossy().into_owned(),json!({"bytes":bytes,"sha256":hash}));}manifest["artifacts"]=json!(artifacts);write_json(&manifest_path,&manifest)?;
    let mut sums=String::new();for path in[&source_zip,&offline,&manifest_path]{let(hash,_)=sha_file(path)?;sums.push_str(&format!("{hash}  {}\n",path.file_name().unwrap().to_string_lossy()));}fs::write(&checksums,sums)?;
    fs::write(output.join("release-notes.md"),format!("# INTA Rust workspace\n\nComplete source, offline Site and canonical Wiki checkpoint: `{commit}`.\n\n{} catalog tools, {} online services and {} service playbooks are preserved in the shared data.\n\nThe package includes all committed source, assets, generated app, Wiki, tests and exact browser/Wiki readback evidence. The offline HTML is byte-identical to the tested build. The manifest records source/tree identities, every source blob, evidence hashes and deployed asset hashes. Native Desktop, Mobile, Rescue and local-agent acceptance remains tied to their separate build and device evidence; this package does not infer their hardware results from a browser pass.\n",build["tools"],build["onlineTools"],build["jobs"]))?;
    Ok(json!({"sourceCommit":commit,"sourceTree":tree,"sourceFiles":tracked.len(),"wikiFiles":expected.len(),"htmlSha256":sha(&html),"artifacts":manifest["artifacts"]}))
}
pub fn package(root:&Path,output:&Path)->Result<Value>{let commit=git_text(root,&["rev-parse","HEAD"])?;let deployment=verify_deployment(root,&commit)?;archive(root,output,deployment)}

pub fn media(binary:&Path,output:&Path)->Result<Value>{let bytes=regular(binary)?;require(!bytes.is_empty(),"Compiled Media Creator is empty")?;if let Some(parent)=output.parent(){fs::create_dir_all(parent)?;}require(!output.exists(),"Output package already exists")?;let mut zip=ZipWriter::new(OpenOptions::new().create_new(true).write(true).open(output)?);let name=binary.file_name().ok_or("Binary filename is missing")?.to_string_lossy();zip.start_file(&name,SimpleFileOptions::default().compression_method(CompressionMethod::Deflated).unix_permissions(0o755))?;zip.write_all(&bytes)?;zip.finish()?.sync_all()?;let(hash,size)=sha_file(output)?;fs::write(PathBuf::from(format!("{}.sha256",output.display())),format!("{hash}  {}\n",output.file_name().unwrap().to_string_lossy()))?;Ok(json!({"path":output,"bytes":size,"sha256":hash,"requires":"No Python or .NET runtime; native Rust executable with embedded brand images"}))}
