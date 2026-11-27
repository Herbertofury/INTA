use std::{env, path::PathBuf, process::ExitCode};
fn run() -> Result<u8, String> {
    let mut args = env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "refresh".into());
    let mut output = None;
    let mut catalog = PathBuf::from("data/tools.json");
    let mut seed = None;
    let mut media = false;
    let mut workers = 6;
    let mut timeout = 15;
    while let Some(flag) = args.next() {
        match flag.as_str() {
            "--page-media" => media = true,
            "--output" => output = Some(PathBuf::from(args.next().ok_or("--output needs path")?)),
            "--seed" => seed = Some(PathBuf::from(args.next().ok_or("--seed needs path")?)),
            "--catalog" => catalog = PathBuf::from(args.next().ok_or("--catalog needs path")?),
            "--workers" => {
                workers = args
                    .next()
                    .ok_or("--workers needs count")?
                    .parse()
                    .map_err(|_| "invalid worker count")?
            }
            "--timeout" => {
                timeout = args
                    .next()
                    .ok_or("--timeout needs seconds")?
                    .parse()
                    .map_err(|_| "invalid timeout")?
            }
            _ => return Err(format!("Unknown option {flag}")),
        }
    }
    match command.as_str(){"refresh"=>{let path=output.unwrap_or_else(||"data/news-digest.json".into());let value=inta_news::refresh(&path,&inta_news::sources(),media,seed.as_deref(),&inta_news::fetch,chrono::Utc::now())?;println!("INTA News: {} retained metadata items; {} source states. {}",value["items"].as_array().unwrap().len(),value["sources"].as_array().unwrap().len(),path.display());Ok(0)},"upstreams"=>{let path=output.unwrap_or_else(||"reports/upstreams.json".into());let value=inta_news::upstream::report(&catalog,&path,workers,timeout)?;println!("Reported {}/{} tools; {} unresolved. No software installed or catalog entries changed.",value["reported"],value["expected"],value["unresolved"]);Ok(if value["unresolved"].as_u64().unwrap_or(0)>0{2}else{0})},_=>Err("Usage: inta-news refresh [--page-media] [--seed path] [--output path] | upstreams [--catalog path] [--output path] [--workers 1..16] [--timeout 1..60]".into())}
}
fn main() -> ExitCode {
    match run() {
        Ok(code) => ExitCode::from(code),
        Err(e) => {
            eprintln!("INTA News: {e}");
            ExitCode::FAILURE
        }
    }
}
