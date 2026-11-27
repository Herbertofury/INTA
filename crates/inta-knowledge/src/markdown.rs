//! Publisher icon decoration that preserves contextual destinations.
use crate::{array,escape,fold,links::{balanced,boundary,protected_end,rewrite_prose},text,RAW_ASSETS};
use anyhow::{ensure,Result};
use regex::{Regex,RegexBuilder};
use serde_json::Value;
use std::{collections::{BTreeMap,BTreeSet},sync::LazyLock};
use url::Url;

static GENERATED:LazyLock<Regex>=LazyLock::new(||Regex::new(r#"(?i)<a\b[^>]*\bdata-inta-brand="[^"]+"[^>]*>\s*<img\b[^>]*>\s*</a> ?"#).unwrap());
static CLEAN:LazyLock<Regex>=LazyLock::new(||Regex::new(r"<[^>]*>|[*_`]").unwrap());
static REFERENCES:LazyLock<Regex>=LazyLock::new(||Regex::new(r"(?m)^ {0,3}\[([^\]]+)\]:\s*(\S+)").unwrap());
static HREF:LazyLock<Regex>=LazyLock::new(||Regex::new(r#"(?i)\bhref=["']([^"']+)["']"#).unwrap());
static TAGS:LazyLock<Regex>=LazyLock::new(||Regex::new(r"<[^>]*>").unwrap());
static ILLUSTRATED:LazyLock<Regex>=LazyLock::new(||Regex::new(r"(?i)<img\b|!\[").unwrap());
fn ambiguous(name:&str)->bool{["everything","windows","autopsy","chainsaw","terminal","edge","access","claude"].contains(&fold(name).as_str())}
fn clean(label:&str)->String{html_escape::decode_html_entities(&CLEAN.replace_all(label,"")).trim().to_string()}
pub fn strip_icons(text:&str)->String{GENERATED.replace_all(text,"").into_owned()}

pub struct BrandMarkdown {registry:Value,by_name:BTreeMap<String,usize>,by_url:BTreeMap<String,usize>,by_host:BTreeMap<String,usize>,matcher:Option<Regex>}
impl BrandMarkdown {
    pub fn new(registry:&Value)->Self {
        let mut by_name=BTreeMap::new();let mut by_url=BTreeMap::new();let mut by_host=BTreeMap::new();
        for (i,brand) in array(&registry["brands"]).iter().enumerate(){
            for name in std::iter::once(text(brand,"name")).chain(array(&brand["names"]).iter().filter_map(Value::as_str)){if !name.is_empty(){by_name.entry(fold(name)).or_insert(i);}}
            for url in array(&brand["urls"]).iter().filter_map(Value::as_str){by_url.entry(url.trim_end_matches('/').into()).or_insert(i);}
            for host in array(&brand["hosts"]).iter().filter_map(Value::as_str){let host=host.to_lowercase().trim_start_matches("www.").to_string();if !by_host.contains_key(&host)||text(brand,"kind")=="website"{by_host.insert(host,i);}}
        }
        let mut names=by_name.keys().cloned().collect::<Vec<_>>();names.sort_by(|l,r|r.chars().count().cmp(&l.chars().count()).then(l.cmp(r)));
        let matcher=if names.is_empty(){None}else{Some(RegexBuilder::new(&names.iter().map(|name|regex::escape(name)).collect::<Vec<_>>().join("|")).case_insensitive(true).size_limit(64*1024*1024).build().expect("escaped brand names form a valid pattern"))};
        Self{registry:registry.clone(),by_name,by_url,by_host,matcher}
    }
    fn brand(&self,index:usize)->&Value{&self.registry["brands"][index]}
    pub fn resolve(&self,label:&str,url:&str)->Option<&Value>{
        let clean=clean(label);let lowercase_ambiguous=ambiguous(&clean)&&clean==clean.to_lowercase();
        if !lowercase_ambiguous {if let Some(index)=self.by_name.get(&fold(&clean)){return Some(self.brand(*index))}}
        if let Some(matcher)=&self.matcher{if let Some(found)=matcher.find_iter(&clean).find(|found|boundary(&clean,found.start(),found.end())){
            if !ambiguous(found.as_str())||found.as_str().chars().next().is_some_and(char::is_uppercase){if let Some(index)=self.by_name.get(&fold(found.as_str())){return Some(self.brand(*index))}}
        }}
        let internal=url.starts_with("https://github.com/Herbertofury/INTA/")||url.starts_with("https://herbertofury.github.io/INTA/")||!url.starts_with("https://");if internal{return None}
        if let Some(index)=self.by_url.get(url.trim_end_matches('/')){return Some(self.brand(*index))}
        if let Some((_,index))=self.by_url.iter().filter(|(route,_)|url.len()>route.len()&&url.starts_with(route.as_str())&&url.as_bytes().get(route.len()).is_some_and(|c|b"/?#".contains(c))).max_by_key(|(route,_)|route.len()){return Some(self.brand(*index))}
        let parsed=Url::parse(url).ok()?;let host=parsed.host_str()?.trim_start_matches("www.");self.by_host.get(host).map(|index|self.brand(*index))
    }
    pub fn resolve_all(&self,label:&str,url:&str)->Vec<&Value>{
        let clean=clean(label);let mut found=Vec::new();let mut ids=BTreeSet::new();
        if let Some(matcher)=&self.matcher{for matched in matcher.find_iter(&clean){if !boundary(&clean,matched.start(),matched.end())||(ambiguous(matched.as_str())&&!matched.as_str().chars().next().is_some_and(char::is_uppercase)){continue}if let Some(index)=self.by_name.get(&fold(matched.as_str())){let brand=self.brand(*index);if ids.insert(text(brand,"id")){found.push(brand);}}}}
        if found.is_empty(){if let Some(brand)=self.resolve(label,url){found.push(brand);}}found
    }
    pub fn icon(&self,brand:Option<&Value>)->Result<String>{
        let Some(brand)=brand else{return Ok(String::new())};let asset=&self.registry["assets"][text(brand,"assetId")];if asset.is_null(){return Ok(String::new())}
        let page=text(brand,"downloadPage");ensure!(Url::parse(page).is_ok_and(|url|url.scheme()=="https"&&url.host_str().is_some()&&url.username().is_empty()&&url.password().is_none()),"Brand destination must be public HTTPS: {}",text(brand,"id"));
        let title=format!("Open {} official download or service page",text(brand,"name"));let source=format!("{RAW_ASSETS}{}",text(asset,"path"));
        Ok(format!("<a href=\"{}\" title=\"{}\" aria-label=\"{}\" data-inta-brand=\"{}\"><img src=\"{}\" alt=\"{}\" width=\"18\" height=\"18\" valign=\"middle\" /></a> ",escape(page),escape(&title),escape(&title),escape(text(brand,"id")),escape(&source),escape(&format!("{} official page",text(brand,"name")))))
    }
    fn inline(&self,text_value:&str,references:&BTreeMap<String,String>)->Result<String>{
        let reference_ids=references.keys().cloned().collect();let mut output=String::new();let mut index=0;
        while index<text_value.len(){
            let Some(end)=protected_end(text_value,index,&reference_ids)else{let c=text_value[index..].chars().next().unwrap();output.push(c);index+=c.len_utf8();continue};
            let token=&text_value[index..end];let mut label=String::new();let mut url=String::new();
            if token.starts_with('[')&&!token.starts_with("[["){if let Some(close)=balanced(token,0,b'[',b']'){
                label=token[1..close-1].to_string();let tail=&token[close..];
                if tail.starts_with('('){url=tail[1..tail.len()-1].split(" \"").next().unwrap_or("").trim().trim_matches(['<','>']).to_string();}
                else if tail.starts_with('['){let reference=tail[1..tail.len()-1].trim();url=references.get(&fold(if reference.is_empty(){&label}else{reference})).cloned().unwrap_or_default();}
                else{url=references.get(&fold(label.trim())).cloned().unwrap_or_default();}
            }}else if token.to_lowercase().starts_with("<a ")&&!token.contains("data-inta-brand"){if let Some(capture)=HREF.captures(token){url=html_escape::decode_html_entities(&capture[1]).into_owned();label=TAGS.replace_all(token,"").into_owned();}}
            if !label.is_empty()&&!url.is_empty()&&!ILLUSTRATED.is_match(token){for brand in self.resolve_all(&label,&url){output.push_str(&self.icon(Some(brand))?);}}
            output.push_str(token);index=end;
        }Ok(output)
    }
    pub fn decorate(&self,text:&str)->Result<String>{
        let text=strip_icons(text);let references=REFERENCES.captures_iter(&text).map(|capture|(fold(capture[1].trim()),capture[2].trim_matches(['<','>']).to_string())).collect();let mut error=None;
        let output=rewrite_prose(&text,|prose|match self.inline(prose,&references){Ok(value)=>value,Err(e)=>{error=Some(e);String::new()}});
        if let Some(error)=error{Err(error)}else{Ok(output)}
    }
}
