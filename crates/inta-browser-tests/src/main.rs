//! Browser acceptance for the exact built INTA artifact.
//! All assertions, sequencing, evidence handling and exports checks run in Rust.
//! JavaScript expressions are limited to CDP DOM inspection and user-event probes.

use anyhow::{bail, ensure, Context, Result};
use headless_chrome::protocol::cdp::{
    types::Event, Browser as BrowserProtocol, Emulation, Network, Page, Runtime,
};
use headless_chrome::{Browser, LaunchOptions, Tab};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    env, fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

#[derive(Debug)]
struct Args {
    html: PathBuf,
    browser: Option<PathBuf>,
    output: PathBuf,
}
fn args() -> Result<Args> {
    let mut result = Args {
        html: "site/index.html".into(),
        browser: None,
        output: "reports/rust-runtime".into(),
    };
    let mut input = env::args().skip(1);
    while let Some(flag) = input.next() {
        let value = input
            .next()
            .with_context(|| format!("{flag} needs a value"))?;
        match flag.as_str() {
            "--html" => result.html = value.into(),
            "--browser" => result.browser = Some(value.into()),
            "--output" => result.output = value.into(),
            _ => bail!("Unknown argument {flag}; use --html, --browser and --output"),
        }
    }
    Ok(result)
}
fn probe(tab: &Tab, expression: &str) -> Result<Value> {
    let result=tab.call_method(serde_json::from_value::<Runtime::Evaluate>(json!({"expression":expression,"returnByValue":true,"awaitPromise":true,"userGesture":true,"timeout":20000}))?).with_context(||format!("Browser probe failed: {expression}"))?;
    ensure!(
        result.exception_details.is_none(),
        "Probe exception for {expression}: {:?}",
        result.exception_details
    );
    Ok(result.result.value.unwrap_or(Value::Null))
}
fn wait(tab: &Tab, expression: &str) -> Result<()> {
    let until = Instant::now() + Duration::from_secs(20);
    loop {
        if probe(tab, expression).ok().and_then(|v| v.as_bool()) == Some(true) {
            return Ok(());
        }
        ensure!(Instant::now() < until, "Timed out: {expression}");
        thread::sleep(Duration::from_millis(40));
    }
}
fn expect(tab: &Tab, expression: &str, label: &str) -> Result<()> {
    ensure!(probe(tab, expression)? == Value::Bool(true), "{label}");
    Ok(())
}
fn string(tab: &Tab, expression: &str) -> Result<String> {
    probe(tab, expression)?
        .as_str()
        .map(str::to_string)
        .with_context(|| format!("Expected string from {expression}"))
}
fn quoted(value: &str) -> String {
    serde_json::to_string(value).unwrap()
}
fn click(tab: &Tab, selector: &str) -> Result<()> {
    tab.wait_for_element(selector)?.click()?;
    Ok(())
}
fn input(tab: &Tab, selector: &str, value: &str, event: &str) -> Result<()> {
    probe(tab,&format!("(()=>{{const e=document.querySelector({});e.focus();e.value={};e.dispatchEvent(new Event({},{{bubbles:true}}));return true}})()",quoted(selector),quoted(value),quoted(event)))?;
    Ok(())
}
fn route(tab: &Tab, fragment: &str, selector: &str) -> Result<()> {
    if string(tab, "location.hash")? != fragment {
        probe(tab,&format!("window.__intaRouteBefore=document.querySelector('#app').firstElementChild;location.hash={}",quoted(fragment)))?;
        wait(tab,&format!("location.hash==={} && document.querySelector('#app').firstElementChild!==window.__intaRouteBefore && document.querySelector({})!==null",quoted(fragment),quoted(selector)))?;
    } else {
        wait(
            tab,
            &format!("document.querySelector({})!==null", quoted(selector)),
        )?;
    }
    Ok(())
}
fn ids(tab: &Tab, selector: &str, attribute: &str) -> Result<Vec<String>> {
    let value = probe(
        tab,
        &format!(
            "Array.from(document.querySelectorAll({}),e=>e.getAttribute({}))",
            quoted(selector),
            quoted(attribute)
        ),
    )?;
    serde_json::from_value(value).context("Reading rendered identities")
}
fn viewport(tab: &Tab, width: u32, height: u32) -> Result<()> {
    tab.call_method(
        serde_json::from_value::<Emulation::SetDeviceMetricsOverride>(
            json!({"width":width,"height":height,"deviceScaleFactor":1,"mobile":false}),
        )?,
    )?;
    wait(tab, &format!("innerWidth==={width}"))?;
    Ok(())
}
fn screenshot(tab: &Tab, output: &Path, name: &str) -> Result<()> {
    probe(tab, "window.scrollTo(0,0)")?;
    probe(
        tab,
        "new Promise(resolve=>requestAnimationFrame(()=>requestAnimationFrame(()=>resolve(true))))",
    )?;
    fs::write(
        output.join(format!("{name}.png")),
        tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Png, None, None, true)?,
    )?;
    Ok(())
}
fn read_data(tab: &Tab) -> Result<Value> {
    probe(tab, "window.INTA_DATA")
}
fn data_ids(data: &Value, path: &str, key: &str) -> Vec<String> {
    data.pointer(path)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|v| v[key].as_str().map(str::to_string))
        .collect()
}
fn push_check(report: &mut Value, label: &str) {
    report["checks"].as_array_mut().unwrap().push(json!(label));
    println!("PASS {label}");
}
fn text<'a>(v: &'a Value, key: &str) -> &'a str {
    v.get(key).and_then(Value::as_str).unwrap_or("")
}
fn array(v: &Value) -> &[Value] {
    v.as_array().map(Vec::as_slice).unwrap_or(&[])
}
fn set(values: Vec<String>) -> BTreeSet<String> {
    values.into_iter().collect()
}
fn link_present(tab: &Tab, selector: &str, url: &str) -> Result<bool> {
    Ok(probe(
        tab,
        &format!(
            "Array.from(document.querySelectorAll({}),e=>e.getAttribute('href')).includes({})",
            quoted(selector),
            quoted(url)
        ),
    )? == Value::Bool(true))
}
fn acquisition<'a>(data: &'a Value, tool: &str, action: &str) -> &'a str {
    let record = &data["toolAccess"][tool];
    if action.is_empty() {
        return text(&record["primary"], "url");
    }
    std::iter::once(&record["primary"])
        .chain(array(&record["alternates"]).iter())
        .find(|option| text(option, "id") == action)
        .map(|option| text(option, "url"))
        .unwrap_or("")
}
fn reload(tab: &Tab, selector: &str) -> Result<()> {
    tab.reload(false, None)?.wait_until_navigated()?;
    wait(
        tab,
        &format!("document.querySelector({})!==null", quoted(selector)),
    )
}
fn assert_clipboard(tab: &Tab, selector: &str, expected: &str) -> Result<()> {
    click(tab, selector)?;
    wait(
        tab,
        &format!(
            "navigator.clipboard.readText().then(value=>value==={})",
            quoted(expected)
        ),
    )
}
fn validate_library(tab: &Tab, data: &Value, report: &mut Value) -> Result<()> {
    let expected = data_ids(data, "/tools", "id");
    ensure!(
        set(ids(tab, "#tool-results [data-tool-id]", "data-tool-id")?) == set(expected.clone()),
        "Home must render the complete tool catalog"
    );
    expect(tab,"document.getElementById('directory-heading').compareDocumentPosition(document.getElementById('online-start-heading')) & Node.DOCUMENT_POSITION_FOLLOWING ? true : false","Tools must precede online services")?;
    ensure!(
        string(tab, "document.getElementById('tool-count').textContent")?
            == expected.len().to_string(),
        "Sidebar tool count differs from catalog"
    );
    let home_scenarios = ids(tab, "[data-quick-start]", "data-quick-start")?;
    ensure!(
        home_scenarios == data_ids(data, "/quickStart/items", "id"),
        "Home lost a scenario or its authored order"
    );
    for item in array(&data["quickStart"]["items"]) {
        let selector = format!("[data-quick-start='{}'] a[href]", text(item, "id"));
        let mut choices = array(&item["choices"]).to_vec();
        if !text(item, "toolId").is_empty() {
            choices.push(json!({"toolId":text(item,"toolId"),"actionId":text(item,"toolAction")}));
        }
        for choice in choices {
            let id = text(&choice, "toolId");
            let url = acquisition(data, id, text(&choice, "actionId"));
            ensure!(
                !url.is_empty() && link_present(tab, &selector, url)?,
                "Home scenario {} lost acquisition for {id}",
                text(item, "id")
            );
            ensure!(
                link_present(tab, &selector, text(&data["toolAccess"][id], "officialUrl"))?,
                "Home scenario {} lost official page for {id}",
                text(item, "id")
            );
        }
    }
    push_check(report,&format!("Home preserves all {} tools, all {} scenarios, exact original acquisitions, every authored choice, and Tools before Online",expected.len(),home_scenarios.len()));
    route(tab, "#tools?view=list", "#library-search")?;
    let before = ids(tab, "#tool-results [data-tool-id]", "data-tool-id")?;
    click(tab, "[data-library-view-toggle='gallery']")?;
    expect(
        tab,
        "document.querySelector('.library-workspace').dataset.libraryView==='gallery'",
        "Gallery switch failed",
    )?;
    ensure!(
        before == ids(tab, "#tool-results [data-tool-id]", "data-tool-id")?,
        "Switching gallery changed result IDs or order"
    );
    expect(
        tab,
        "getComputedStyle(document.querySelector('.library-column-head')).display==='none'",
        "Gallery retained the list column header",
    )?;
    reload(tab, "#library-search")?;
    expect(
        tab,
        "document.querySelector('.library-workspace').dataset.libraryView==='gallery'",
        "Gallery preference did not survive reload",
    )?;
    click(tab, "[data-library-view-toggle='list']")?;
    click(tab, "[data-library-filters]")?;
    expect(tab,"document.querySelector('[data-library-filters]').getAttribute('aria-expanded')==='true' && getComputedStyle(document.getElementById('tool-filters')).display!=='none'","Filters disclosure did not expose its controls")?;
    let categories = array(&data["tools"])
        .iter()
        .map(|t| text(t, "category"))
        .collect::<BTreeSet<_>>();
    for category in categories {
        input(tab, "#category-filter", category, "change")?;
        let expected = array(&data["tools"])
            .iter()
            .filter(|t| text(t, "category") == category)
            .map(|t| text(t, "id").to_string())
            .collect();
        ensure!(
            set(ids(tab, "#tool-results [data-tool-id]", "data-tool-id")?) == set(expected),
            "Category {category} did not retain its exact tool set"
        );
    }
    click(tab, "[data-reset-filters]")?;
    for (id, field, value) in [
        ("platform-filter", "platforms", "Windows"),
        ("tier-filter", "tier", "core"),
        ("mode-filter", "mode", "portable"),
    ] {
        input(tab, &format!("#{id}"), value, "change")?;
        let actual = ids(tab, "#tool-results [data-tool-id]", "data-tool-id")?;
        let expected = array(&data["tools"])
            .iter()
            .filter(|t| match field {
                "platforms" => array(&t[field]).iter().any(|p| p == value),
                "mode" => text(t, field).split('/').any(|part| part.trim() == value),
                _ => text(t, field) == value,
            })
            .map(|t| text(t, "id").to_string())
            .collect();
        ensure!(
            set(actual) == set(expected),
            "{field} filter produced an incorrect result set"
        );
        click(tab, "[data-reset-filters]")?;
    }
    input(tab, "#library-search", "network Windows", "input")?;
    input(tab, "#category-filter", "network", "change")?;
    input(tab, "#platform-filter", "Windows", "change")?;
    let filtered = ids(tab, "#tool-results [data-tool-id]", "data-tool-id")?;
    ensure!(
        !filtered.is_empty(),
        "Combined multi-term/platform/category filters lost the network tools"
    );
    click(tab, "[data-library-view-toggle='gallery']")?;
    ensure!(
        filtered == ids(tab, "#tool-results [data-tool-id]", "data-tool-id")?,
        "Filtered result identities changed with layout"
    );
    let filtered_url = string(tab, "location.hash")?;
    let first = &filtered[0];
    click(tab, &format!("[data-library-detail='{first}']"))?;
    wait(tab,"document.querySelector('.detail-header')!==null && document.querySelector('#library-search')===null")?;
    probe(tab, "history.back()")?;
    wait(
        tab,
        &format!(
            "location.hash==={} && document.getElementById('library-search')!==null",
            quoted(&filtered_url)
        ),
    )?;
    ensure!(
        filtered == ids(tab, "#tool-results [data-tool-id]", "data-tool-id")?,
        "Browser Back lost filtered result set"
    );
    click(tab, "[data-reset-filters]")?;
    input(
        tab,
        "#library-search",
        "no-tool-can-match-this-inta-957218",
        "input",
    )?;
    ensure!(
        ids(tab, "#tool-results [data-tool-id]", "data-tool-id")?.is_empty(),
        "Impossible query returned tools"
    );
    expect(
        tab,
        "document.getElementById('library-result-count').textContent.trim().startsWith('0 ')",
        "Empty-result count is inaccurate",
    )?;
    click(tab, "[data-reset-filters]")?;
    ensure!(
        ids(tab, "#tool-results [data-tool-id]", "data-tool-id")?.len() == expected.len(),
        "Clear filters did not restore all tools"
    );
    push_check(report,"List/gallery parity, all category/platform/recommendation/use filters, combined search, empty/reset, browser Back and persistent layout");

    let favorite = expected.first().context("Empty catalog")?;
    click(tab, &format!("#tool-results [data-save='{favorite}']"))?;
    route(tab, "#saved", "#library-search")?;
    ensure!(
        ids(tab, "#tool-results [data-tool-id]", "data-tool-id")? == vec![favorite.clone()],
        "Saved toolkit differs from saved choice"
    );
    reload(tab, "#library-search")?;
    ensure!(
        ids(tab, "#tool-results [data-tool-id]", "data-tool-id")? == vec![favorite.clone()],
        "Saved toolkit lost on reload"
    );
    click(tab, &format!("[data-save='{favorite}']"))?;
    ensure!(
        ids(tab, "#tool-results [data-tool-id]", "data-tool-id")?.is_empty(),
        "Last unsaved result remained visible"
    );
    expect(
        tab,
        "document.activeElement.id==='library-search'",
        "Removing final saved result lost keyboard focus",
    )?;
    push_check(report,"Saved toolkit persists across reload, removes immediately, and restores focus when emptied");
    for tool in array(&data["tools"]) {
        let id = text(tool, "id");
        route(tab, &format!("#tool/{id}"), ".detail-header")?;
        ensure!(
            string(
                tab,
                "document.querySelector('.detail-header h1').textContent"
            )?
            .trim()
                == text(tool, "name"),
            "Tool {id} detail title differs from source"
        );
        let record = &data["toolAccess"][id];
        for action in std::iter::once(&record["primary"]).chain(array(&record["alternates"]).iter())
        {
            ensure!(
                link_present(tab, ".detail-actions a[href]", text(action, "url"))?,
                "Detail {id} lost acquisition {}",
                text(action, "id")
            );
        }
        for field in ["homepage", "docsUrl", "sourceUrl"] {
            let url = text(tool, field);
            if !url.is_empty() {
                ensure!(
                    link_present(tab, "#app a[href]", url)?,
                    "Detail {id} lost {field}"
                );
            }
        }
    }
    push_check(report,&format!("All {} tool detail routes retain names, every acquisition alternative and official/documentation/source links",expected.len()));
    Ok(())
}

fn validate_guides(tab: &Tab, data: &Value, report: &mut Value) -> Result<()> {
    route(tab, "#playbooks", "#jobs-search")?;
    let jobs = array(&data["jobs"]);
    ensure!(
        ids(tab, "[data-job-route]", "data-job-route")?.len() == jobs.len(),
        "Playbook directory does not contain every guide"
    );
    for job in jobs {
        let id = text(job, "id");
        route(tab, &format!("#job/{id}"), "[data-job-workspace]")?;
        ensure!(
            string(
                tab,
                "document.querySelector('[data-job-workspace]').dataset.jobWorkspace"
            )? == id,
            "Wrong job rendered"
        );
        ensure!(
            ids(tab, "[data-step-anchor]", "data-step-anchor")?.len() == array(&job["steps"]).len(),
            "{id} lost procedure steps"
        );
        ensure!(
            ids(tab, "[data-check-type='verify']", "data-check-index")?.len()
                == array(&job["verify"]).len(),
            "{id} lost completion checks"
        );
        let resource_ids = data_ids(data, &format!("/jobWorkspaces/jobs/{id}/resources"), "id");
        ensure!(
            set(ids(
                tab,
                "[data-workspace-resource]",
                "data-workspace-resource"
            )?) == set(resource_ids),
            "{id} lost working-kit resources"
        );
        let codes = probe(
            tab,
            "Array.from(document.querySelectorAll('#app pre code'),e=>e.textContent)",
        )?;
        for step in array(&job["steps"]) {
            if let Some(command) = step["command"].as_str() {
                ensure!(
                    array(&codes).iter().any(|v| v == command),
                    "{id} lost exact command text"
                );
            }
        }
        let fixes = array(&data["fieldNotes"]["notes"])
            .iter()
            .filter(|note| text(note, "jobId") == id)
            .map(|note| text(note, "id").to_string())
            .collect::<Vec<_>>();
        ensure!(
            ids(tab, "[data-field-note]", "data-field-note")? == fixes,
            "{id} lost specific fixes"
        );
    }
    for item in array(&data["quickStart"]["items"]) {
        let id = text(item, "id");
        let job = text(item, "jobId");
        route(
            tab,
            &format!("#job/{job}/quick/{id}"),
            "[data-quick-answer]",
        )?;
        let selected = format!("[data-quick-answer='{id}']");
        let steps = probe(
            tab,
            &format!(
                "Array.from(document.querySelectorAll({}),e=>e.textContent)",
                quoted(&format!("{selected} .quick-answer-steps ol > li"))
            ),
        )?;
        ensure!(
            array(&steps).len() == 3,
            "{id} must have exactly three first actions"
        );
        let mut choices = array(&item["choices"]).to_vec();
        if !text(item, "toolId").is_empty() {
            choices.push(json!({"toolId":text(item,"toolId"),"actionId":text(item,"toolAction")}));
        }
        for choice in choices {
            let tool = text(&choice, "toolId");
            let url = acquisition(data, tool, text(&choice, "actionId"));
            ensure!(
                link_present(tab, &format!("{selected} a[href]"), url)?,
                "{id} quick answer lost {tool} action"
            );
            ensure!(
                link_present(tab, "#job-tools a[href]", url)?,
                "{id} working kit lost {tool} action"
            );
        }
    }
    push_check(report,&format!("All {} guides retain procedure/checks/resources/commands/fixes; every scenario has three first actions and complete exact tool choices",jobs.len()));
    Ok(())
}

fn validate_commands(tab: &Tab, data: &Value, report: &mut Value) -> Result<()> {
    tab.call_method(serde_json::from_value::<BrowserProtocol::GrantPermissions>(
        json!({"permissions":["clipboardReadWrite","clipboardSanitizedWrite"]}),
    )?)?;
    for (name, key, attribute, copy_attribute, workflow_attribute, input_id) in [
        (
            "command-center",
            "commandCenter",
            "data-command-item",
            "data-copy-center-command",
            "data-copy-center-workflow",
            "command-center-search",
        ),
        (
            "updates",
            "updatesCenter",
            "data-update-command-card",
            "data-copy-update-command",
            "data-copy-update-workflow",
            "updates-search",
        ),
    ] {
        route(tab, &format!("#{name}"), &format!("#{input_id}"))?;
        let commands = array(&data[key]["commands"]);
        ensure!(
            ids(tab, &format!("[{attribute}]"), attribute)?
                == commands
                    .iter()
                    .map(|item| text(item, "id").to_string())
                    .collect::<Vec<_>>(),
            "{name} did not render all commands in source order"
        );
        for command in commands {
            let id = text(command, "id");
            let selector = format!("[{attribute}='{id}']");
            let shown = string(
                tab,
                &format!(
                    "document.querySelector({}).textContent",
                    quoted(&format!("{selector} pre code"))
                ),
            )?;
            ensure!(
                shown == text(command, "code"),
                "{name}/{id} changed command text"
            );
            ensure!(
                link_present(
                    tab,
                    &format!("{selector} a[href]"),
                    text(command, "sourceUrl")
                )?,
                "{name}/{id} lost official evidence"
            );
            assert_clipboard(
                tab,
                &format!("{selector} [{copy_attribute}]"),
                text(command, "code"),
            )?;
        }
        for workflow in array(&data[key]["workflows"]) {
            click(
                tab,
                &format!("[{workflow_attribute}='{}']", text(workflow, "id")),
            )?;
            let expected = array(&workflow["commandIds"])
                .iter()
                .map(|id| {
                    commands
                        .iter()
                        .find(|command| command["id"] == *id)
                        .context("Workflow has unknown command")
                })
                .collect::<Result<Vec<_>>>()?;
            let needed = expected
                .iter()
                .flat_map(|command| {
                    [
                        text(command, "code"),
                        text(command, "safety"),
                        text(command, "sourceUrl"),
                    ]
                })
                .collect::<Vec<_>>();
            wait(
                tab,
                &format!(
                    "navigator.clipboard.readText().then(value=>{}.every(part=>value.includes(part)))",
                    serde_json::to_string(&needed)?
                ),
            )?;
            let copied = string(tab, "navigator.clipboard.readText()")?;
            for id in array(&workflow["commandIds"]) {
                let command = commands
                    .iter()
                    .find(|command| command["id"] == *id)
                    .context("Workflow command missing")?;
                for field in ["code", "safety", "sourceUrl"] {
                    ensure!(
                        copied.contains(text(command, field)),
                        "{name} workflow {} lost {field}",
                        text(workflow, "id")
                    );
                }
            }
        }
        input(
            tab,
            &format!("#{input_id}"),
            "no-command-matches-957218",
            "input",
        )?;
        ensure!(
            ids(tab, &format!("[{attribute}]"), attribute)?.is_empty(),
            "{name} impossible search returned commands"
        );
        input(tab, &format!("#{input_id}"), "", "input")?;
        input(tab, &format!("#{name}-risk"), "read-only", "change")?;
        let expected = commands
            .iter()
            .filter(|command| text(command, "risk") == "read-only")
            .map(|command| text(command, "id").to_string())
            .collect();
        ensure!(
            set(ids(tab, &format!("[{attribute}]"), attribute)?) == set(expected),
            "{name} risk filter differs from authored impact"
        );
        push_check(report,&format!("{name}: all {} commands and {} workflows copy exact code with source/safety retained; search and impact filter operate",commands.len(),array(&data[key]["workflows"]).len()));
    }
    let step_job = array(&data["jobs"])
        .iter()
        .find(|job| {
            array(&job["steps"])
                .iter()
                .any(|step| !text(step, "command").is_empty())
        })
        .context("No command-bearing job")?;
    let index = array(&step_job["steps"])
        .iter()
        .position(|step| !text(step, "command").is_empty())
        .unwrap();
    route(
        tab,
        &format!("#job/{}", text(step_job, "id")),
        "[data-job-workspace]",
    )?;
    assert_clipboard(
        tab,
        &format!(
            "[data-copy-command='{}'][data-command-index='{index}']",
            text(step_job, "id")
        ),
        text(&step_job["steps"][index], "command"),
    )?;
    let note = array(&data["fieldNotes"]["notes"])
        .iter()
        .find(|note| !array(&note["commands"]).is_empty())
        .context("No field command fixture")?;
    route(
        tab,
        &format!("#job/{}", text(note, "jobId")),
        "[data-job-workspace]",
    )?;
    assert_clipboard(
        tab,
        &format!(
            "[data-copy-field-command='{}'][data-command-index='0']",
            text(note, "id")
        ),
        text(&note["commands"][0], "code"),
    )?;
    push_check(
        report,
        "Procedure and specific-fix command buttons copy their exact authored command",
    );
    Ok(())
}

fn wait_file(path: &Path) -> Result<Vec<u8>> {
    let until = Instant::now() + Duration::from_secs(20);
    loop {
        if let Ok(bytes) = fs::read(path) {
            if !bytes.is_empty() {
                return Ok(bytes);
            }
        }
        ensure!(
            Instant::now() < until,
            "No completed download: {}",
            path.display()
        );
        thread::sleep(Duration::from_millis(50));
    }
}
fn validate_checklist_and_exports(
    tab: &Tab,
    data: &Value,
    output: &Path,
    report: &mut Value,
) -> Result<()> {
    route(tab, "#checklist", "[data-check-job='client-handoff']")?;
    let selector = "[data-check-job='client-handoff']";
    let first_title = string(
        tab,
        &format!(
            "document.querySelector({}).dataset.checkTitle",
            quoted(selector)
        ),
    )?;
    click(tab, selector)?;
    expect(
        tab,
        &format!("document.querySelector({}).checked", quoted(selector)),
        "Checklist click did not check the item",
    )?;
    reload(tab, selector)?;
    expect(
        tab,
        &format!("document.querySelector({}).checked", quoted(selector)),
        "Checklist progress did not survive reload",
    )?;
    click(tab, "[data-reset-checklist='client-handoff']")?;
    expect(
        tab,
        "!Array.from(document.querySelectorAll('[data-check-job]')).some(e=>e.checked)",
        "Reset did not clear the checklist",
    )?;
    click(tab, "[data-undo]")?;
    expect(
        tab,
        &format!("document.querySelector({}).checked", quoted(selector)),
        "Undo did not restore the checklist",
    )?;
    let downloads = output.join("downloads");
    fs::create_dir_all(&downloads)?;
    tab.call_method(serde_json::from_value::<Page::SetDownloadBehavior>(
        json!({"behavior":"allow","downloadPath":downloads}),
    )?)?;
    let checklist = downloads.join("INTA-client-handoff-checklist.md");
    if checklist.exists() {
        fs::remove_file(&checklist)?;
    }
    click(tab, "[data-export-job='client-handoff']")?;
    let exported = String::from_utf8(wait_file(&checklist)?)?;
    ensure!(
        exported.contains("- [x]") && exported.contains(&first_title),
        "Export lost the actual checked item"
    );
    let job = array(&data["jobs"])
        .iter()
        .find(|job| text(job, "id") == "client-handoff")
        .context("Missing handoff guide")?;
    for step in array(&job["steps"])
        .iter()
        .filter(|step| text(step, "anchor").starts_with("step-closeout-"))
    {
        ensure!(
            exported.contains(text(step, "title")),
            "Focused export lost checklist step {}",
            text(step, "title")
        );
    }
    let focused = downloads.join("INTA-technician-checklist.md");
    fs::rename(&checklist, &focused)?;
    route(tab, "#job/client-handoff", "[data-job-workspace]")?;
    click(tab, "[data-export-job='client-handoff']")?;
    let full_export = String::from_utf8(wait_file(&checklist)?)?;
    for step in array(&job["steps"]) {
        ensure!(
            full_export.contains(text(step, "title")),
            "Full export lost step {}",
            text(step, "title")
        );
    }
    for value in array(&job["verify"]) {
        if let Some(value) = value.as_str() {
            ensure!(
                full_export.contains(value),
                "Full export lost verification step"
            );
        }
    }
    ensure!(
        full_export.contains("- [x]") && full_export.contains(&first_title),
        "Full guide export lost the shared checklist state"
    );
    route(tab, "#resources", "[data-download-offline]")?;
    let offline = downloads.join("INTA-offline.html");
    if offline.exists() {
        fs::remove_file(&offline)?;
    }
    click(tab, "[data-download-offline]")?;
    let bytes = wait_file(&offline)?;
    ensure!(bytes.len() > 100_000, "Offline export is incomplete");
    let original_url = tab.get_url();
    tab.navigate_to(&format!("file://{}#tools", offline.to_string_lossy()))?
        .wait_until_navigated()?;
    wait(
        tab,
        "document.querySelector('#tool-results [data-tool-id]')!==null",
    )?;
    ensure!(
        set(ids(tab, "#tool-results [data-tool-id]", "data-tool-id")?)
            == set(data_ids(data, "/tools", "id")),
        "Downloaded offline artifact lost tools or did not boot"
    );
    tab.navigate_to(&original_url)?.wait_until_navigated()?;
    wait(
        tab,
        "document.querySelector('[data-download-offline]')!==null",
    )?;
    report["exports"] = json!({"focusedChecklist":focused,"fullChecklist":checklist,"offline":offline,"offlineSha256":format!("{:x}",Sha256::digest(&bytes))});
    push_check(report,"Checklist click/reload/reset/Undo works; actual focused/full Markdown exports retain checked state and complete relevant steps; downloaded offline HTML boots with all tools");
    Ok(())
}

fn validate_news_and_keyboard(tab: &Tab, data: &Value, report: &mut Value) -> Result<()> {
    route(tab, "#news", "#news-search")?;
    ensure!(
        set(ids(tab, "[data-news-id]", "data-news-id")?)
            == set(data_ids(data, "/news/sources", "id")),
        "News directory lost sources"
    );
    ensure!(
        ids(tab, "[data-digest-id]", "data-digest-id")?
            == data_ids(data, "/newsDigest/items", "id"),
        "News digest lost records"
    );
    expect(tab,"document.querySelector('#news-digest iframe,#news-digest video,#news-digest audio')===null","News loaded third-party media without user request")?;
    let id = data_ids(data, "/news/sources", "id")
        .into_iter()
        .next()
        .context("No news sources")?;
    click(tab, &format!("[data-news-save='{id}']"))?;
    click(tab, "#news-saved")?;
    ensure!(
        ids(tab, "[data-news-id]", "data-news-id")? == vec![id.clone()],
        "Saved source filter returned wrong sources"
    );
    reload(tab, "#news-search")?;
    ensure!(
        ids(tab, "[data-news-id]", "data-news-id")? == vec![id.clone()],
        "Saved sources did not persist"
    );
    click(tab, &format!("[data-news-save='{id}']"))?;
    ensure!(
        ids(tab, "[data-news-id]", "data-news-id")?.is_empty(),
        "Unsave did not remove source from saved-only filter"
    );
    push_check(report,"News retains all sources/digest records, media stays opt-in, source saves/filter/reload work");
    route(tab, "#home", "#library-search")?;
    probe(tab, "document.getElementById('main').focus()")?;
    tab.press_key("/")?;
    expect(
        tab,
        "document.activeElement.id==='global-search'",
        "Search shortcut failed to focus global search",
    )?;
    tab.type_str("network")?;
    wait(
        tab,
        "document.querySelectorAll('#search-results [data-search-tool]').length>12",
    )?;
    tab.press_key("ArrowDown")?;
    expect(
        tab,
        "document.getElementById('search-results').contains(document.activeElement)",
        "Search arrow key did not enter results",
    )?;
    tab.press_key("Escape")?;
    expect(
        tab,
        "document.getElementById('search-results').hidden",
        "Escape did not close search results",
    )?;
    input(tab, "#global-search", "winget", "input")?;
    expect(tab,"document.querySelector('#search-results a[href^=\"#command-center\"]')!==null && document.querySelector('#search-results a[href^=\"#updates\"]')!==null","Global search lost command or maintenance results")?;
    tab.press_key("Escape")?;
    click(tab, "#theme-toggle")?;
    expect(
        tab,
        "document.documentElement.dataset.theme==='light'",
        "Theme control did not switch to light",
    )?;
    reload(tab, "#library-search")?;
    expect(
        tab,
        "document.documentElement.dataset.theme==='light'",
        "Light theme did not persist across reload",
    )?;
    click(tab, "#theme-toggle")?;
    viewport(tab, 390, 900)?;
    click(tab, "#mobile-menu")?;
    expect(tab,"document.getElementById('mobile-menu').getAttribute('aria-expanded')==='true' && document.getElementById('sidebar').contains(document.activeElement)","Mobile navigation did not open/focus")?;
    tab.press_key("Escape")?;
    expect(
        tab,
        "document.getElementById('mobile-menu').getAttribute('aria-expanded')==='false'",
        "Escape did not close mobile navigation",
    )?;
    push_check(report,"Global search is uncapped, includes commands, supports keyboard navigation; theme persists and mobile menu opens/closes accessibly");
    Ok(())
}

fn visual_matrix(tab: &Tab, output: &Path, report: &mut Value) -> Result<()> {
    let views = [
        ("home", "#home", ".workspace-home"),
        ("tools-list", "#tools?view=list", "#library-search"),
        ("tools-gallery", "#tools?view=gallery", "#library-search"),
        ("tool-detail", "#tool/malwarebytes", ".detail-header"),
        ("playbooks", "#playbooks", "#jobs-search"),
        ("job", "#job/malware-removal", "[data-job-workspace]"),
        ("checklist", "#checklist", "[data-check-job]"),
        ("services", "#services", ".service-area"),
        ("commands", "#command-center", "#command-center-search"),
        ("updates", "#updates", "#agent-panel-title"),
        ("online", "#online", "#online-search"),
        ("news", "#news", "#news-search"),
        ("knowledge", "#knowledge", "#source-search"),
        ("resources", "#resources", ".resource-list"),
    ];
    for width in [1440, 800, 390, 320] {
        viewport(tab, width, 900)?;
        for (name, hash, selector) in views {
            route(tab, hash, selector)?;
            for theme in ["dark", "light"] {
                probe(
                    tab,
                    &format!("document.documentElement.dataset.theme={}", quoted(theme)),
                )?;
                let metrics=probe(tab,"({width:innerWidth,scroll:document.documentElement.scrollWidth,main:document.getElementById('main').getBoundingClientRect().width})")?;
                if metrics["scroll"].as_f64().unwrap_or(f64::MAX) > f64::from(width + 1) {
                    report["overflow"]=probe(tab,"Array.from(document.querySelectorAll('body *')).filter(e=>{const r=e.getBoundingClientRect();return r.width>0 && (r.right>innerWidth+1 || r.left < -1) && getComputedStyle(e).position!=='fixed'}).slice(0,25).map(e=>({tag:e.tagName,id:e.id,classes:e.className,left:e.getBoundingClientRect().left,right:e.getBoundingClientRect().right,width:e.getBoundingClientRect().width}))")?;
                    bail!("{name} at {width}px {theme} overflows the document: {metrics}");
                }
                let duplicate_ids=probe(tab,"Array.from(document.querySelectorAll('[id]'),e=>e.id).filter((id,index,all)=>all.indexOf(id)!==index)")?;
                ensure!(
                    array(&duplicate_ids).is_empty(),
                    "{name} contains duplicate DOM IDs: {duplicate_ids}"
                );
                if theme == "dark"
                    || (width == 390
                        && ["home", "tools-gallery", "job", "online", "updates", "news"]
                            .contains(&name))
                {
                    let file = format!("{name}-{width}-{theme}");
                    screenshot(tab, output, &file)?;
                    report["screenshots"]
                        .as_array_mut()
                        .unwrap()
                        .push(json!(format!("{file}.png")));
                }
            }
        }
        push_check(report,&format!("All {} major views at {width}px: dark/light, no document overflow or duplicate IDs, screenshots retained",views.len()));
    }
    Ok(())
}
fn run(tab: &Tab, output: &Path, report: &mut Value) -> Result<()> {
    wait(
        tab,
        "document.querySelector('#tool-results [data-tool-id]')!==null",
    )?;
    expect(
        tab,
        "document.documentElement.dataset.theme==='dark'",
        "New workspace must default to dark",
    )?;
    let data = read_data(tab)?;
    report["content"] = json!({"tools":array(&data["tools"]).len(),"jobs":array(&data["jobs"]).len(),"commands":array(&data["commandCenter"]["commands"]).len(),"maintenanceCommands":array(&data["updatesCenter"]["commands"]).len(),"onlineTools":array(&data["onlineTools"]["tools"]).len()});
    validate_library(tab, &data, report)?;
    validate_guides(tab, &data, report)?;
    validate_commands(tab, &data, report)?;
    validate_checklist_and_exports(tab, &data, output, report)?;
    validate_news_and_keyboard(tab, &data, report)?;
    visual_matrix(tab, output, report)?;
    Ok(())
}

fn main() -> Result<()> {
    let args = args()?;
    let artifact = fs::canonicalize(&args.html).context("Locate built HTML artifact")?;
    fs::create_dir_all(&args.output)?;
    let output = fs::canonicalize(&args.output)?;
    let bytes = fs::read(&artifact)?;
    let mut report = json!({"status":"running","artifact":artifact,"sha256":format!("{:x}",Sha256::digest(&bytes)),"checks":[],"screenshots":[],"runtimeErrors":[],"externalRequests":[]});
    let options = LaunchOptions::default_builder()
        .path(args.browser)
        .headless(true)
        .sandbox(false)
        .window_size(Some((1440, 1000)))
        .build()
        .map_err(|error| anyhow::anyhow!(error))?;
    let result = (|| -> Result<()> {
        let browser = Browser::new(options).context("Launch the installed Chrome browser")?;
        let tab = browser.new_tab()?;
        tab.set_default_timeout(Duration::from_secs(20));
        let errors = Arc::new(Mutex::new(Vec::<String>::new()));
        let requests = Arc::new(Mutex::new(Vec::<String>::new()));
        let captured_errors = errors.clone();
        let captured_requests = requests.clone();
        tab.enable_runtime()?;
        tab.call_method(serde_json::from_value::<Network::Enable>(json!({}))?)?;
        tab.add_event_listener(Arc::new(move |event: &Event| match event {
            Event::RuntimeExceptionThrown(event) => captured_errors
                .lock()
                .unwrap()
                .push(format!("{:?}", event.params.exception_details)),
            Event::NetworkRequestWillBeSent(event) => {
                let url = &event.params.request.url;
                if url.starts_with("https://") || url.starts_with("http://") {
                    captured_requests.lock().unwrap().push(url.clone());
                }
            }
            _ => {}
        }))?;
        tab.call_method(serde_json::from_value::<Network::SetBlockedURLs>(
            json!({"urls":["http://*","https://*"]}),
        )?)?;
        let file_url = format!("file://{}#home", artifact.to_string_lossy());
        tab.navigate_to(&file_url)?.wait_until_navigated()?;
        let test_result = run(&tab, &output, &mut report);
        report["runtimeErrors"] = json!(*errors.lock().unwrap());
        report["externalRequests"] = json!(*requests.lock().unwrap());
        report["browser"] = probe(&tab, "navigator.userAgent").unwrap_or(Value::Null);
        if test_result.is_err() {
            let _ = screenshot(&tab, &output, "failure");
            report["failureRoute"] = json!(tab.get_url());
            report["failureDom"] = probe(
                &tab,
                "document.querySelector('#app')?.innerText.slice(0,16000)",
            )
            .unwrap_or(Value::Null);
        }
        test_result?;
        ensure!(
            errors.lock().unwrap().is_empty(),
            "Browser runtime raised errors"
        );
        ensure!(
            requests.lock().unwrap().is_empty(),
            "Offline artifact attempted external network requests"
        );
        push_check(
            &mut report,
            "Offline file has no external requests or unhandled runtime errors",
        );
        Ok(())
    })();
    report["status"] = json!(if result.is_ok() { "passed" } else { "failed" });
    if let Err(ref error) = result {
        report["failure"] = json!(format!("{error:#}"));
    }
    fs::write(
        output.join("rust-runtime.json"),
        serde_json::to_vec_pretty(&report)?,
    )?;
    result
}
