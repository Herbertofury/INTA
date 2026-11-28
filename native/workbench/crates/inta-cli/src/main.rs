#[cfg(windows)]
mod assistant;
#[cfg(windows)]
mod cleanup;
mod db;
mod health;
mod inspect;
mod linux_tools;
mod scenario;
#[cfg(windows)]
mod startup;
mod storage;
mod system_inspection;
mod tool_actions;
mod toolbox;
mod tweak;

use anyhow::{Result, bail, ensure};
use rusqlite::Connection;
use serde_json::{Value, json};
use std::{env, path::Path};

const HELP: &str = "INTA technician CLI. Results are JSON.
  capabilities | tools search QUERY | tools inspect ID | forge verify FILE
  ai ask MODEL MODEL_DIGEST QUESTION | ai run-plan SCENARIO PLAN_SHA256 --approve
  ai forge MODEL MODEL_DIGEST REQUEST NEW_ABSOLUTE_RECIPE.json --approve
  catalog [query] | record tool|job|method ID
  tool actions [ID] | tool install|launch ID --approve | tool bind ID FILE.exe --approve
  tool configure ID HTTPS_URL --approve | tool workflow|terminal|linux ID --approve [--root DIRECTORY]
  assistant prepare MODEL_STORE --approve | assistant server-status | assistant server-plan
  inspect overview|storage|network|security|startup|drivers|events|performance|applications|updates|power|printing|directory|recovery|disks|models
  storage DIRECTORY
  startup list | startup preview ID | startup disable|restore ID REVISION --approve
  cleanup plan DIRECTORY | cleanup reviews | cleanup review ID | cleanup quarantine|restore REVIEW ITEM[,ITEM] --approve
  scenario list|checkpoints | scenario preview ID | scenario run ID [--skip STAGE] | scenario resume ID RUN_ID [--skip STAGE]
  toolbox list|discover|use DIRECTORY|verify ID|probe ID [--root DIRECTORY]
  toolbox archive FILE.7z [--root DIRECTORY]
  toolbox backup SOURCE NEW.7z --approve [--root DIRECTORY]
  vm repair-test | vm console --approve [--base FILE.img] [--root DIRECTORY]
  vm status|sessions | vm plan ISO [--base FILE.img] | vm start ISO --approve [--base FILE.img] [--nat] | vm stop ID --approve
  tweak list | tweak status|preview [ID] | tweak apply|undo [ID] --approve
  tweak profiles | tweak profile preview ID | tweak profile apply|undo ID --approve
  history | version | build-info";
fn action(c: &Connection, args: &[String]) -> Result<Value> {
    let a = args.iter().map(String::as_str).collect::<Vec<_>>();
    match a.as_slice() {
        #[cfg(windows)]
        ["ai", args @ ..] => assistant::triage(c, args),
        #[cfg(windows)]
        ["startup", args @ ..] => startup::command(c, args),
        #[cfg(windows)]
        ["cleanup", args @ ..] => cleanup::command(c, args),
        #[cfg(windows)]
        ["assistant", "prepare", path, "--approve"] => {
            assistant::server::prepare(c, Path::new(path))
        }
        #[cfg(windows)]
        ["assistant", "server-status"] => assistant::server::status(c),
        #[cfg(windows)]
        ["assistant", "server-plan"] => assistant::server::plan(c),
        #[cfg(windows)]
        ["assistant", args @ ..] => assistant::command(args),
        ["catalog", q @ ..] => db::catalog(c, &q.join(" ")),
        ["record", kind, id] => db::record(c, kind, id),
        ["inspect", kind] if inspect::KINDS.contains(kind) => {
            audited(c, &format!("inspect/{kind}"), || inspect::run(kind))
        }
        ["storage", dir] => audited(c, "storage/space-analysis", || {
            storage::scan(Path::new(dir))
        }),
        ["scenario", "list"] => scenario::list(c),
        ["scenario", "checkpoints"] => scenario::checkpoints(c),
        ["scenario", "preview", id] => scenario::preview(c, id),
        ["scenario", verb @ ("run" | "resume"), id, rest @ ..] => {
            let (checkpoint, flags) = if *verb == "resume" {
                let (r, flags) = rest
                    .split_first()
                    .ok_or_else(|| anyhow::anyhow!("Missing run ID"))?;
                (Some(*r), flags)
            } else {
                (None, rest)
            };
            let mut skip = vec![];
            for pair in flags.chunks(2) {
                ensure!(
                    pair.len() == 2 && pair[0] == "--skip",
                    "Expected --skip STAGE"
                );
                skip.push(pair[1].to_owned());
            }
            scenario::run(c, id, &skip, checkpoint)
        }
        ["toolbox", args @ ..] => toolbox::command(c, args),
        ["tool", args @ ..] => tool_actions::command(c, args),
        ["linux-session", verb, id, "--approve"] => {
            audited(c, "linux/session", || linux_tools::session(verb, id))
        }
        ["vm", args @ ..] => {
            let (commands, root) = match args {
                [commands @ .., "--root", p] => (commands, Some(*p)),
                _ => (args, None),
            };
            let toolbox = toolbox::select(c, root)?;
            let execute = || inta_vm_lab::command(&toolbox, commands).map_err(anyhow::Error::msg);
            match commands.first() {
                Some(&"start") => audited(c, "vm/start", execute),
                Some(&"stop") => audited(c, "vm/stop", execute),
                Some(&"repair-test") => audited(c, "vm/repair-test", execute),
                Some(&"console") => audited(c, "vm/console", execute),
                _ => execute(),
            }
        }
        ["tweak", "profiles"] => Ok(tweak::profiles::list()),
        #[cfg(windows)]
        ["tweak", "profile", verb, id, flags @ ..] => {
            ensure!(
                flags.is_empty() || *flags == ["--approve"],
                "Unknown profile option"
            );
            let action = || tweak::profiles::run(c, verb, id, !flags.is_empty());
            if matches!(*verb, "apply" | "undo") {
                audited(c, &format!("tweak/profile/{verb}/{id}"), action)
            } else {
                action()
            }
        }
        ["tweak", "list"] => Ok(tweak::list()),
        #[cfg(windows)]
        ["tweak", op, flags @ ..] => {
            let (id, approved) = match flags {
                [] => (tweak::SPECS[0].id, false),
                ["--approve"] => (tweak::SPECS[0].id, true),
                [id] => (*id, false),
                [id, "--approve"] => (*id, true),
                _ => bail!("Unknown tweak option"),
            };
            let spec = tweak::SPECS
                .iter()
                .find(|s| s.id == id)
                .ok_or_else(|| anyhow::anyhow!("Unknown setting"))?;
            let run = || tweak::operation(c, &tweak::ExplorerSetting(spec), op, approved);
            if matches!(*op, "apply" | "undo") {
                audited(c, &format!("tweak/{op}/{id}"), run)
            } else {
                run()
            }
        }
        ["history"] => db::history(c),
        _ => bail!("Unknown command or arguments. Run inta --help."),
    }
}
fn audited(c: &Connection, name: &str, f: impl FnOnce() -> Result<Value>) -> Result<Value> {
    db::event(c, name, "started")?;
    let result = f();
    db::event(c, name, if result.is_ok() { "complete" } else { "failed" })?;
    result
}
fn build_info() -> Result<Value> {
    let exe = env::current_exe()?;
    let file = exe.with_file_name("manifest.json");
    if !file.exists() {
        return Ok(
            json!({"channel":"developer-build","packaged":false,"version":env!("CARGO_PKG_VERSION")}),
        );
    }
    let reader = std::fs::File::open(file)?;
    let manifest: Value = serde_json::from_reader(reader)?;
    Ok(
        json!({"channel":manifest["channel"],"version":env!("CARGO_PKG_VERSION"),"revision":manifest["revision"],"sourceDigest":manifest["sourceDigest"],"generation":manifest["generation"],"packaged":true}),
    )
}
fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() || args == ["--help"] || args == ["help"] {
        println!("{HELP}");
        return;
    }
    let value = if args == ["build-info"] {
        build_info()
    } else if args == ["version"] || args == ["--version"] {
        let mut capabilities = vec!["vm", "tool-actions"];
        if inta_vm_lab::rust_lab_enabled() {
            capabilities.extend(["rust-repair-lab", "rust-rescue-console"]);
        }
        Ok(
            json!({"name":"INTA","version":env!("CARGO_PKG_VERSION"),"channel":"desktop","schemaVersion":1,"capabilities":capabilities}),
        )
    } else {
        db::open().and_then(|c| action(&c, &args))
    };
    match value {
        Ok(v) => {
            println!("{v}");
            if v.get("status") == Some(&json!("stopped")) {
                std::process::exit(3)
            }
        }
        Err(e) => {
            eprintln!("INTA: {e:#}");
            std::process::exit(1)
        }
    }
}
