//! Structural and editorial review-age checks. No dates are rewritten.
use crate::{array,brands::url_key,fold,text,WIKI};
use anyhow::{ensure,Context,Result};
use chrono::{Days,NaiveDate};
use regex::Regex;
use serde_json::{json,Value};
use std::{collections::{BTreeMap,BTreeSet},sync::LazyLock};
use unicode_normalization::UnicodeNormalization;
use url::Url;

static ID:LazyLock<Regex>=LazyLock::new(||Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap());
pub fn public_url(value:&str)->bool{
    Url::parse(value).is_ok_and(|url|url.scheme()=="https"&&url.host_str().is_some()&&url.username().is_empty()&&url.password().is_none()&&! ["utm_","fbclid=","gclid="].iter().any(|part|url.query().unwrap_or("").to_lowercase().contains(part)))
}
pub fn normalized_name(value:&str)->String{fold(&value.nfkc().collect::<String>()).chars().filter(|c|c.is_alphanumeric()||*c=='_').collect()}
pub fn canonical_url(value:&str)->String{url_key(value)}
fn date(value:&str)->Result<NaiveDate>{ensure!(value.len()==10&&value.as_bytes().get(4)==Some(&b'-')&&value.as_bytes().get(7)==Some(&b'-'),"Invalid ISO date: {value}");NaiveDate::parse_from_str(value,"%Y-%m-%d").with_context(||format!("Invalid ISO date: {value}"))}
fn identities(rows:&[Value])->BTreeSet<&str>{rows.iter().map(|row|text(row,"id")).collect()}
fn unique_strings(value:&Value)->Option<BTreeSet<&str>>{let values=value.as_array()?;let set=values.iter().map(Value::as_str).collect::<Option<BTreeSet<_>>>()?;(set.len()==values.len()).then_some(set)}
pub fn validate_field_notes(field_notes:&Value,jobs:&[Value],tools:&[Value])->Result<()> {
    ensure!(field_notes["schemaVersion"]==1&&field_notes["notes"].is_array(),"Field notes require schemaVersion 1 and a notes list");date(text(field_notes,"reviewedAt"))?;
    let job_ids=identities(jobs);let tool_ids=identities(tools);let mut ids=BTreeSet::new();let mut titles=BTreeSet::new();let mut covered=BTreeSet::new();
    for note in array(&field_notes["notes"]){
        let id=text(note,"id");ensure!(ID.is_match(id)&&ids.insert(id),"Duplicate or invalid field-note ID: {id}");
        ensure!(job_ids.contains(text(note,"jobId")),"{id}: unknown field-note job");covered.insert(text(note,"jobId"));
        for field in ["title","symptom","firstAction","expected","ifUnresolved"]{ensure!(note[field].is_string()&&!text(note,field).trim().is_empty(),"{id}: field note needs {field}");}
        ensure!(titles.insert(normalized_name(text(note,"title"))),"Duplicate field-note title: {}",text(note,"title"));
        ensure!(note["steps"].is_array()&&!array(&note["steps"]).is_empty()&&array(&note["steps"]).iter().all(|s|s.as_str().is_some_and(|s|!s.trim().is_empty())),"{id}: concrete field-note steps are required");
        ensure!(unique_strings(&note["toolIds"]).is_some_and(|set|set.is_subset(&tool_ids)),"{id}: unknown or duplicate field-note tools");
        ensure!(unique_strings(&note["sourceUrls"]).is_some_and(|set|!set.is_empty()&&set.iter().all(|url|public_url(url))),"{id}: distinct public HTTPS evidence is required");
        date(text(note,"reviewedAt"))?;
    }
    ensure!(covered==job_ids,"Field notes must cover every job; missing {:?}",job_ids.difference(&covered).collect::<Vec<_>>());Ok(())
}
pub fn validate_organization(policy:&Value,queue:&Value,tools:&[Value],jobs:&[Value],layout:&Value,field_notes:&Value)->Result<()> {
    ensure!(policy["schemaVersion"]==1&&text(policy,"taxonomySource")=="data/wiki-layout.json","Knowledge policy must use the existing canonical layout taxonomy");
    for key in ["coreTool","specialistTool","evaluateTool","guide","fieldNote","resource","acquisition"]{ensure!(policy["reviewIntervalsDays"][key].as_u64().is_some_and(|v|v>0),"Invalid knowledge review interval: {key}");}
    let groups=array(&layout["groups"]);let group_ids=groups.iter().map(|g|text(g,"id")).collect::<BTreeSet<_>>();let mut grouped=BTreeMap::<String,usize>::new();let mut expected=BTreeMap::<String,usize>::new();
    for job in jobs{*expected.entry(text(job,"id").into()).or_default()+=1;}
    for id in groups.iter().flat_map(|g|array(&g["jobs"]).iter()){*grouped.entry(id.as_str().unwrap_or("").into()).or_default()+=1;}
    ensure!(!groups.is_empty()&&group_ids.len()==groups.len()&&group_ids.iter().all(|id|ID.is_match(id))&&grouped==expected,"Every job needs one service home with a distinct stable area ID");
    let mut names=BTreeMap::<String,Vec<&str>>::new();let mut urls=BTreeMap::<String,Vec<&str>>::new();
    for tool in tools{names.entry(normalized_name(text(tool,"name"))).or_default().push(text(tool,"id"));urls.entry(canonical_url(text(tool,"homepage"))).or_default().push(text(tool,"id"));}
    let duplicate_names=names.values().filter(|ids|ids.len()>1).collect::<Vec<_>>();ensure!(duplicate_names.is_empty(),"Duplicate normalized tool name: {duplicate_names:?}");
    let tool_ids=identities(tools);let mut allowed=BTreeSet::<BTreeSet<&str>>::new();
    for exception in array(&policy["duplicateOfficialPageAllowlist"]){let ids=unique_strings(&exception["toolIds"]).unwrap_or_default();ensure!(ids.len()>=2&&ids.is_subset(&tool_ids)&&!text(exception,"reason").trim().is_empty(),"A shared official-page exception needs known tools and a reason");allowed.insert(ids);}
    let duplicate_urls=urls.values().filter(|ids|ids.len()>1&&!allowed.contains(&ids.iter().copied().collect::<BTreeSet<_>>())).collect::<Vec<_>>();ensure!(duplicate_urls.is_empty(),"Duplicate official tool page needs review: {duplicate_urls:?}");
    ensure!(queue["schemaVersion"]==1&&queue["items"].is_array(),"Research queue requires schemaVersion 1 and items");
    let job_ids=identities(jobs);let category_ids=layout["categories"].as_object().map(|v|v.keys().map(String::as_str).collect::<BTreeSet<_>>()).unwrap_or_default();
    let targets=BTreeMap::from([("job",job_ids),("category",category_ids)]);let mut covered=BTreeMap::from([("job",BTreeSet::new()),("category",BTreeSet::new())]);let mut ids=BTreeSet::new();let mut questions=BTreeSet::new();
    for item in array(&queue["items"]){
        let id=text(item,"id");ensure!(ID.is_match(id)&&ids.insert(id),"Duplicate or invalid research ID: {id}");let kind=text(item,"kind");let target=text(item,"targetId");
        ensure!(targets.get(kind).is_some_and(|ids|ids.contains(target)),"{id}: unknown research target");covered.get_mut(kind).unwrap().insert(target);
        ensure!(["queued","investigating","ready","done","blocked"].contains(&text(item,"status")),"{id}: unknown research status");
        ensure!(item["priority"].as_u64().is_some_and(|p|(1..=3).contains(&p)),"{id}: research priority must be 1, 2 or 3");
        for field in ["question","deliverable","acceptance"]{ensure!(item[field].is_string()&&!text(item,field).trim().is_empty(),"{id}: research needs {field}");}
        ensure!(questions.insert((kind,target,normalized_name(text(item,"question")))),"{id}: duplicate research question");
        ensure!(!array(&item["sourceUrls"]).is_empty()&&array(&item["sourceUrls"]).iter().all(|url|url.as_str().is_some_and(public_url)),"{id}: official research starting sources are required");
        ensure!(text(item,"status")!="done"||public_url(text(item,"resultUrl")),"{id}: completed research needs a result URL with public HTTPS");
    }
    for (kind,targets) in targets{ensure!(covered[kind]==targets,"Research queue must cover every {kind}: missing {:?}",targets.difference(&covered[kind]).collect::<Vec<_>>());}
    validate_field_notes(field_notes,jobs,tools)
}
#[allow(clippy::too_many_arguments)]
pub fn knowledge_report(policy:&Value,queue:&Value,tools:&[Value],jobs:&[Value],layout:&Value,field_notes:&Value,workspaces:&Value,access:&Value,as_of:&str,quick_start:Option<&Value>)->Result<Value>{
    validate_organization(policy,queue,tools,jobs,layout,field_notes)?;let as_of=date(as_of)?;let intervals=&policy["reviewIntervalsDays"];let mut reviews=Vec::<Value>::new();
    let interval=|key:&str|->Result<u64>{intervals[key].as_u64().filter(|v|*v>0).with_context(||format!("Missing review interval {key}"))};
    let mut review=|kind:&str,id:&str,title:&str,reviewed_at:&str,interval:u64,url:&str|->Result<()>{
        let reviewed=date(reviewed_at)?;ensure!(reviewed<=as_of,"Future review date: {kind}/{id} ({reviewed})");let due=reviewed.checked_add_days(Days::new(interval)).context("Review date interval overflow")?;
        reviews.push(json!({"kind":kind,"id":id,"title":title,"reviewedAt":reviewed.to_string(),"dueAt":due.to_string(),"status":if due<as_of{"overdue"}else if due==as_of{"due"}else{"current"},"daysOverdue":(as_of-due).num_days().max(0),"url":url}));Ok(())
    };
    for tool in tools{
        let id=text(tool,"id");review("tool",id,text(tool,"name"),text(tool,"reviewedAt"),interval(&format!("{}Tool",text(tool,"tier")))?,text(tool,"homepage"))?;
        let entry=&access[id];for (index,action) in std::iter::once(&entry["primary"]).chain(array(&entry["alternates"]).iter()).enumerate(){let action_id=if text(action,"id").is_empty(){index.to_string()}else{text(action,"id").to_string()};review("acquisition",&format!("{id}/{action_id}"),text(action,"label"),text(action,"checkedAt"),interval("acquisition")?,text(action,"evidenceUrl"))?;}
    }
    for job in jobs{review("guide",text(job,"id"),text(job,"title"),text(job,"reviewedAt"),interval("guide")?,&format!("{WIKI}/Job-{}",text(job,"id")))?;}
    for note in array(&field_notes["notes"]){review("field-note",text(note,"id"),text(note,"title"),text(note,"reviewedAt"),interval("fieldNote")?,note["sourceUrls"][0].as_str().unwrap_or(""))?;}
    if let Some(workspaces)=workspaces["jobs"].as_object(){for (job_id,workspace) in workspaces{for resource in array(&workspace["resources"]){review("resource",&format!("{job_id}/{}",text(resource,"id")),text(resource,"label"),text(resource,"checkedAt"),interval("resource")?,text(resource,"sourceUrl"))?;}}}
    let mut choice_count=0;
    if let Some(quick_start)=quick_start{for item in array(&quick_start["items"]){for choice in array(&item["choices"]){choice_count+=1;let tool=tools.iter().find(|tool|text(tool,"id")==text(choice,"toolId")).with_context(||format!("Unknown selected tool {}",text(choice,"toolId")))?;let action=if text(choice,"actionId").is_empty(){"primary"}else{text(choice,"actionId")};review("tool-choice",&format!("{}/{}/{action}",text(item,"id"),text(choice,"toolId")),&format!("{}: {} ({})",text(item,"label"),text(tool,"name"),text(choice,"role")),text(choice,"reviewedAt"),interval("coreTool")?,choice["evidenceUrls"][0].as_str().unwrap_or(""))?;}}}
    let mut due=reviews.iter().filter(|record|text(record,"status")!="current").cloned().collect::<Vec<_>>();due.sort_by(|l,r|r["daysOverdue"].as_i64().cmp(&l["daysOverdue"].as_i64()).then(text(l,"kind").cmp(text(r,"kind"))).then(text(l,"id").cmp(text(r,"id"))));
    let mut research=array(&queue["items"]).to_vec();research.sort_by(|l,r|l["priority"].as_u64().cmp(&r["priority"].as_u64()).then(text(l,"id").cmp(text(r,"id"))));
    let coverage=jobs.iter().map(|job|{let id=text(job,"id");json!({"jobId":id,"fieldNotes":array(&field_notes["notes"]).iter().filter(|note|text(note,"jobId")==id).count(),"resources":array(&workspaces["jobs"][id]["resources"]).len(),"decisions":array(&workspaces["jobs"][id]["decisions"]).len()})}).collect::<Vec<_>>();
    Ok(json!({"schemaVersion":1,"asOf":as_of.to_string(),"structuralStatus":"passed","counts":{"serviceAreas":array(&layout["groups"]).len(),"jobs":jobs.len(),"tools":tools.len(),"toolCategories":layout["categories"].as_object().map(|c|c.len()).unwrap_or(0),"fieldNotes":array(&field_notes["notes"]).len(),"researchItems":array(&queue["items"]).len(),"scenarioToolChoices":choice_count,"reviewedRecords":reviews.len(),"dueReviews":due.len()},"coverage":coverage,"dueReviews":due,"reviews":reviews,"researchQueue":research,"limits":"Structural validation and editorial-age arithmetic only. No network, hardware test, source rereview or date updates."}))
}
