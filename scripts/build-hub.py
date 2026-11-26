#!/usr/bin/env python3
"""Build the offline technician hub and live-wiki source from canonical content."""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import html
import json
import re
import unicodedata
from collections import Counter, defaultdict
from pathlib import Path
from urllib.parse import urlparse

ROOT = Path(__file__).resolve().parents[1]
REPO = "https://github.com/Herbertofury/INTA"
WIKI = REPO + "/wiki"
HUB = "https://herbertofury.github.io/INTA/"
CATEGORY_NAMES = {
    "malware": "Malware removal & endpoint protection", "remote": "Remote support",
    "windows": "Windows diagnosis & repair", "storage": "Storage, recovery & backup",
    "hardware": "Hardware diagnostics", "network": "Networking",
    "boot": "Bootable media", "productivity": "Technician productivity",
    "ai": "AI assistance", "security": "Advanced defensive security",
}


def records(path: Path, key: str) -> list[dict]:
    value = json.loads(path.read_text(encoding="utf-8"))
    return value[key] if isinstance(value, dict) else value


def slug(value: str) -> str:
    return re.sub(r"[^a-z0-9]+", "-", value.lower()).strip("-")


def cell(value) -> str:
    if isinstance(value, list):
        value = ", ".join(str(x) for x in value)
    return str(value).replace("|", "\\|").replace("\n", " ")


def validate(tools: list[dict], jobs: list[dict]) -> None:
    def required(item, fields):
        for field in fields:
            if field not in item or not item[field]:
                raise ValueError(f"{item.get('id', '?')}: missing {field}")

    def ids(items, kind):
        values = [x["id"] for x in items]
        if len(values) != len(set(values)):
            raise ValueError(f"Duplicate {kind} ids")
        if any(not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", x) for x in values):
            raise ValueError(f"Invalid {kind} id")
        return set(values)

    if not tools or not jobs:
        raise ValueError("The hub requires both tools and guides")
    tool_ids, job_ids = ids(tools, "tool"), ids(jobs, "job")
    for tool in tools:
        required(tool, ["id", "name", "category", "summary", "bestFor", "platforms",
                        "mode", "pricing", "homepage", "downloadUrl", "docsUrl", "sourceUrl",
                        "reviewedAt", "tier", "tags", "provenance"])
        if tool["tier"] not in {"core", "specialist", "evaluate"}:
            raise ValueError(f"Invalid tier: {tool['id']}")
        for field in ["homepage", "downloadUrl", "docsUrl", "sourceUrl"]:
            parsed = urlparse(tool[field])
            if parsed.scheme != "https" or not parsed.hostname or parsed.username:
                raise ValueError(f"Non-public HTTPS URL: {tool['id']}.{field}")
            if any(key in parsed.query.lower() for key in ("utm_", "fbclid=", "gclid=")):
                raise ValueError(f"Tracking parameter: {tool['id']}.{field}")
        if not isinstance(tool["platforms"], list) or not isinstance(tool["tags"], list):
            raise ValueError(f"List expected: {tool['id']}")
        if not re.fullmatch(r"\d{4}-\d{2}-\d{2}", tool["reviewedAt"]):
            raise ValueError(f"Invalid review date: {tool['id']}")
    for job in jobs:
        required(job, ["id", "title", "category", "summary", "estimatedTime", "difficulty",
                       "startHere", "steps", "verify", "stopIf", "sources"])
        if not isinstance(job.get("tools"), list):
            raise ValueError(f"{job['id']}: tools must be a list (may be empty for handoff)")
        missing = set(job["tools"]) - tool_ids
        if missing:
            raise ValueError(f"{job['id']}: unknown tools {sorted(missing)}")
        missing = set(job.get("related", [])) - job_ids
        if missing:
            raise ValueError(f"{job['id']}: unknown related jobs {sorted(missing)}")
        step_ids = []
        for number, step in enumerate(job["steps"], 1):
            required(step, ["title", "detail"])
            step_ids.append(step_anchor(step, number))
        if len(set(step_ids)) != len(step_ids):
            raise ValueError(f"{job['id']}: duplicate step anchors")
        for source in job["sources"]:
            required(source, ["title", "url"])
            if urlparse(source["url"]).scheme != "https":
                raise ValueError(f"Non-HTTPS source: {job['id']}")


def link_module():
    spec = importlib.util.spec_from_file_location("inta_wiki_links", ROOT / "scripts/wiki_links.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def validate_layout(layout: dict, jobs: list[dict]) -> None:
    expected = {job["id"] for job in jobs}
    if set(layout.get("jobs", {})) != expected:
        raise ValueError("Wiki layout must cover every playbook exactly")
    grouped = [identity for group in layout.get("groups", []) for identity in group["jobs"]]
    if set(grouped) != expected or len(grouped) != len(expected):
        raise ValueError("Wiki navigation must group every playbook exactly once")
    diagrams = {"home": layout.get("homeDiagram", "")}
    diagrams.update({identity: config.get("diagram", "") for identity, config in layout["jobs"].items()})
    for identity, diagram in diagrams.items():
        if not diagram.startswith("flowchart TD\n") or diagram.count(":::decision") < 2 or diagram.count("-->") < 5:
            raise ValueError(f"{identity}: a substantive top-down decision diagram is required")
        if re.search(r"(?im)^\s*click\s|%%\{|<[^>]+>", diagram):
            raise ValueError(f"{identity}: Mermaid directives and HTML labels are not supported")
        if "https://" in diagram:
            raise ValueError(f"{identity}: put linked references outside the diagram")
    for identity, config in layout["jobs"].items():
        if len(config.get("checkpoints", {})) != 3:
            raise ValueError(f"{identity}: before, during, and handoff checkpoints are required")


def page_nav() -> str:
    return (f"[INTA]({WIKI}) / [Decision desk]({WIKI}/Decision-Desk) · "
            f"[Playbooks]({WIKI}/Playbooks) · [Tools]({WIKI}/Tool-Directory) · "
            f"[Methods]({WIKI}/Methods) · [Open hub]({HUB})")


def page_footer(reviewed: str, wiki: bool = True) -> list[str]:
    lines = ["", "---", "", f"Editorial review: **{reviewed}** · "
             f"[Review standard]({WIKI}/Knowledge-Standards) · [Suggest a correction]({REPO}/issues/new/choose)", ""]
    if not wiki:
        lines += [f"[← All playbooks](README.md) · [Live Wiki]({WIKI}) · [Contribution guide]({REPO}/blob/main/CONTRIBUTING.md)", ""]
    return lines


def diagram_block(source: str) -> list[str]:
    return ["```mermaid", source.rstrip(), "```", ""]


def anchor(identity: str) -> str:
    return f'<a id="{identity}"></a>'


def step_anchor(step: dict, number: int) -> str:
    identity = step.get("anchor", f"step-{number:02d}")
    if not isinstance(identity, str) or not re.fullmatch(r"step-[a-z0-9]+(?:-[a-z0-9]+)*", identity):
        raise ValueError(f"Invalid step anchor: {identity!r}")
    return identity


def legacy_anchors(title: str) -> list[str]:
    """Retain prior heading destinations when presentation wording changes."""
    title = re.sub(r"\[([^\]]+)\]\([^)]*\)", r"\1", title).lower()
    value = "".join(c for c in title if unicodedata.category(c)[0] in "LNM" or c in "_ -").replace(" ", "-")
    # Hosts differ in whether emoji variation selectors survive slug creation.
    alternatives = [value, value.replace("\ufe0f", "").replace("\ufe0e", "")]
    return [anchor(identity) for identity in dict.fromkeys(alternatives) if identity]


def heading(level: int, title: str, previous: str | None = None) -> list[str]:
    aliases = legacy_anchors(previous) + [""] if previous and previous != title else []
    return [*aliases, "#" * level + " " + title, ""]


def command_reference(step: dict, by_id: dict) -> str:
    """Each executable example carries an exact documentation route."""
    command = step["command"].split()[0].lower()
    references = {
        "perfmon": ("Reliability Monitor", "https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/perfmon"),
        "resmon": ("Resource Monitor", "https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/perfmon"),
        "dism.exe": ("DISM", by_id["dism"]["docsUrl"]),
        "sfc": ("System File Checker", by_id["sfc"]["docsUrl"]),
        "lsblk": ("lsblk command manual", "https://man7.org/linux/man-pages/man8/lsblk.8.html"),
        "get-netipconfiguration": ("Get-NetIPConfiguration", "https://learn.microsoft.com/en-us/powershell/module/nettcpip/get-netipconfiguration"),
        "resolve-dnsname": ("Resolve-DnsName", "https://learn.microsoft.com/en-us/powershell/module/dnsclient/resolve-dnsname"),
        "test-computersecurechannel": ("Test-ComputerSecureChannel", "https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.management/test-computersecurechannel?view=powershell-5.1"),
        "winget": ("WinGet upgrade", "https://learn.microsoft.com/en-us/windows/package-manager/winget/upgrade"),
    }
    if step.get("commandUrl"):
        name, url = step.get("commandName", command), step["commandUrl"]
        if urlparse(url).scheme != "https" or not urlparse(url).hostname:
            raise ValueError(f"Command reference must be HTTPS: {command}")
    elif command in references:
        name, url = references[command]
    else:
        raise ValueError(f"Add an exact commandUrl and commandName for: {command}")
    links = [f"[{name}]({url})"]
    if command in {"get-netipconfiguration", "resolve-dnsname", "test-computersecurechannel"}:
        links += [f"[PowerShell]({by_id['powershell']['docsUrl']})"]
    return " · ".join(links)


def job_markdown(job: dict, by_id: dict, wiki: bool, layout: dict | None = None,
                 jobs_by_id: dict | None = None, reviewed: str = "2026-11-26") -> str:
    layout = layout or json.loads((ROOT / "data/wiki-layout.json").read_text(encoding="utf-8"))
    config = layout["jobs"][job["id"]]
    jobs_by_id = jobs_by_id or {j["id"]: j for j in records(ROOT / "data/jobs.json", "jobs")}
    destination = lambda identity: f"{WIKI}/Job-{identity}" if wiki else f"{identity}.md"
    lines = [page_nav(), "", *heading(1, job["title"], f"{config['icon']} {job['title']}"), job["summary"], "",
             f"**{cell(job['category'])}** · {cell(job['difficulty'])} · {cell(job['estimatedTime'])} · {len(job['steps'])} steps", "",
             "[Decision route](#decision-route) · [Tools](#toolkit) · [Procedure](#walkthrough) · "
             f"[Verification](#acceptance) · [Sources](#sources) · [Interactive checklist]({HUB}#job/{job['id']})", "",
             "> [!NOTE]", f"> **Start here.** {job['startHere']}", "",
             anchor("decision-route"), "", *heading(2, "🧭 Choose a route", "🧭 Choose the right branch"),
             "<details>", "<summary>View the decision map and escalation routes</summary>", ""]
    lines += diagram_block(config["diagram"])
    routes = ["[Continue this workflow](#walkthrough)", "[Acceptance checks](#acceptance)"]
    routes += [f"[{jobs_by_id[identity]['title']}]({destination(identity)})" for identity in job.get("related", [])]
    lines += ["**Route links:** " + " · ".join(routes), "", "</details>", ""]
    lines += ["| Checkpoint | What to carry forward |", "| --- | --- |"]
    lines += [f"| **{cell(label)}** | {cell(detail)} |" for label, detail in config["checkpoints"].items()]
    lines += ["", anchor("toolkit"), "", *heading(2, "Recommended tools", "🧰 Build the right toolkit")]
    if job["tools"]:
        lines += ["Select a tool for the next diagnostic or repair step. Check its platform, access and licensing requirements before use.", "",
                  "| Tool / official download | Use it for | Platform & access | Documentation |", "| --- | --- | --- | --- |"]
        for tool_id in job["tools"]:
            tool = by_id[tool_id]
            lines += [f"| [{tool['name']}]({tool['downloadUrl']}) | {cell(tool['bestFor'])} | "
                      f"{cell(tool['platforms'])}; {cell(tool['pricing'])} | [Open guide]({tool['docsUrl']}) |"]
    else:
        lines += [f"Use the approved private service record and the [intake and handoff template]({WIKI}/Technician-Handoff). No specialist application is required."]
    lines += ["", anchor("walkthrough"), "", *heading(2, "Procedure", "🪜 Work the case"),
              "Work in order, record the result of each step, and change route when the evidence calls for it.", ""]
    for number, step in enumerate(job["steps"], 1):
        identity = step_anchor(step, number)
        old_number = re.fullmatch(r"step-(\d+)", identity)
        old_title = f"{int(old_number[1]):02d} · {step['title']}" if old_number else None
        lines += [anchor(identity), "", *heading(3, f"{number:02d} · {step['title']}", old_title),
                  f"- [ ] {step['detail']}", ""]
        if step.get("command"):
            lines += ["**Command for the environment described above:**", "", "```text", step["command"], "```", ""]
            lines += [f"**Command legend:** {command_reference(step, by_id)} · [Return to the step context](#{identity}).", ""]
    lines += [anchor("acceptance"), "", *heading(2, "Verification", "✅ Prove the result")]
    lines += [f"- [ ] {item}" for item in job["verify"]]
    lines += ["", "> [!WARNING]", "> **Pause this route and escalate when:**"]
    lines += [f"> - {item}" for item in job["stopIf"]]
    lines += ["", *heading(2, "Related playbooks", "🔀 Continue with the evidence")]
    if job.get("related"):
        lines += ["| Next playbook | What it resolves |", "| --- | --- |"]
        for identity in job["related"]:
            other = jobs_by_id[identity]
            lines += [f"| [{other['title']}]({destination(identity)}) | {cell(other['summary'])} |"]
    lines += ["", anchor("sources"), "", *heading(2, "Official sources", "🔎 Source trail"),
              "<details>", "<summary>References and review notes</summary>", ""]
    lines += [f"- [{source['title']}]({source['url']})" for source in job["sources"]]
    lines += ["", f"Reviewed **{job.get('reviewedAt', reviewed)}**. Timing is a planning range. "
              "A successful command is evidence about that operation; acceptance belongs to the user’s original task.", "", "</details>"]
    lines += page_footer(job.get("reviewedAt", reviewed), wiki)
    return "\n".join(lines)


def tier_label(value: str) -> str:
    return {"core": "Core", "specialist": "Specialist", "evaluate": "Evaluate"}[value]


def category_markdown(category: str, tools: list[dict], layout: dict, jobs: dict, reviewed: str) -> str:
    info = layout["categories"].get(category, {"icon": "🧰", "intro": "Choose a tool for the observed job.", "jobs": [], "question": "What result do you need?"})
    label = CATEGORY_NAMES.get(category, category)
    ordered = sorted(tools, key=lambda t: ({"core": 0, "specialist": 1, "evaluate": 2}[t["tier"]], t["name"].casefold()))
    lines = [page_nav(), "", *heading(1, label, f"{info['icon']} {label}"), info["intro"], "",
             f"**{len(tools)} tools** · [All categories]({WIKI}/Tool-Directory) · [Selection standards]({WIKI}/Knowledge-Standards)", "",
             f"**Selection question:** {info['question']}", "", *heading(2, "Related playbooks", "🧭 Start with a workflow")]
    lines += [f"- [{jobs[identity]['title']}]({WIKI}/Job-{identity})" for identity in info["jobs"]]
    lines += ["", anchor("compare-options"), "", *heading(2, "Tool comparison", "⚖️ Compare the options"),
              "| Tool | Choose it for | Platforms | Role |", "| --- | --- | --- | --- |"]
    for tool in ordered:
        lines += [f"| [{tool['name']}](#{tool['id']}) | {cell(tool['bestFor'])} | {cell(tool['platforms'])} | {tier_label(tool['tier'])} |"]
    lines += ["", "Core means an everyday candidate; Specialist means a focused advanced use; Evaluate means prove it in a suitable test environment before promotion.", ""]
    for tool in ordered:
        lines += [anchor(tool["id"]), "", f"## {tool['name']}", "", tool["summary"], "",
                  f"[Official download]({tool['downloadUrl']}) · [Documentation]({tool['docsUrl']}) · [Publisher]({tool['homepage']})", "",
                  "| Decision | Detail |", "| --- | --- |",
                  f"| **Choose it for** | {cell(tool['bestFor'])} |",
                  f"| **Where it runs** | {cell(tool['platforms'])} |",
                  f"| **How you use it** | {cell(tool['mode'])} |",
                  f"| **Access / pricing** | {cell(tool['pricing'])} |",
                  f"| **Selection role** | {tier_label(tool['tier'])} |", ""]
        if tool.get("cautions"):
            lines += [f"> **Before using it:** {tool['cautions']}", ""]
        lines += ["<details>", "<summary>Source, review date, and selection history</summary>", "",
                  f"- **Primary source:** [Open the source]({tool['sourceUrl']})",
                  f"- **Editorial review:** {tool['reviewedAt']}",
                  f"- **Evidence status:** {tool.get('evidenceStatus', 'Documentation review; verify suitability for the current job.')}",
                  "- **Selection provenance:**"]
        provenance = tool["provenance"] if isinstance(tool["provenance"], list) else [tool["provenance"]]
        lines += [f"  - {text}" for text in provenance]
        lines += ["", "</details>", "", "[Back to comparison](#compare-options)", ""]
    lines += page_footer(reviewed)
    return "\n".join(lines)


def playbook_index(jobs: list[dict], layout: dict, wiki: bool, reviewed: str) -> str:
    by_id = {job["id"]: job for job in jobs}
    destination = lambda identity: f"{WIKI}/Job-{identity}" if wiki else f"{identity}.md"
    lines = [page_nav(), "", *heading(1, "Playbooks", "📚 The playbook library"),
             "Procedures for common service jobs, with decision diagrams, tool selection, ordered steps, verification and escalation criteria.", "",
             f"**{len(jobs)} playbooks** · [Identify the right route]({WIKI}/Decision-Desk) · [Troubleshooting methods]({WIKI}/Methods)", "",
             " · ".join(f"[{group['title']}](#{group['id']})" for group in layout["groups"]), ""]
    for group in layout["groups"]:
        lines += [anchor(group["id"]), "", *heading(2, f"{group['icon']} {group['title']}"), group["summary"], "",
                  "| Playbook | Outcome | Experience | Plan your time |", "| --- | --- | --- | --- |"]
        for identity in group["jobs"]:
            job = by_id[identity]
            lines += [f"| [{job['title']}]({destination(identity)}) | {cell(job['summary'])} | {job['difficulty']} | {cell(job['estimatedTime'])} |"]
        lines += [""]
    lines += [*heading(2, "Corrections and additions", "🤝 Make the next case easier"), f"Use the [contribution template]({WIKI}/Templates) when a case reveals a better sequence, a changed tool, or a missing stop condition. "
              f"Keep customer evidence in the private service record and add only the reusable method to the [shared knowledge process]({WIKI}/Continuous-Improvement)."]
    lines += page_footer(reviewed, wiki)
    return "\n".join(lines)


def home_markdown(tools: list[dict], jobs: list[dict], categories: dict, layout: dict, meta: dict) -> str:
    by_id = {job["id"]: job for job in jobs}
    lines = [f"![INTA Computer Services — technical knowledge base]({REPO}/raw/main/docs/assets/wiki-hero.svg)", "",
             *heading(1, "INTA knowledge base", "🧭 Your next good fix starts here."),
             "Service procedures, tool references and troubleshooting methods for the INTA team.", "",
             f"[Open technician hub]({HUB}) · [Decision desk](Decision-Desk) · [Playbooks](Playbooks) · [Tool directory](Tool-Directory) · [Methods](Methods)", "",
             f"**{len(tools)} tools · {len(jobs)} playbooks · {len(categories)} tool categories**", "",
             *heading(2, "🧭 Start a job", "🚦 First, choose the right route"),
             "| Observed situation | First route |", "| --- | --- |",
             "| Encryption spreading, unusual privileged access, or several affected machines | [Contain a wider compromise](Job-ransomware-response) |",
             "| Clicking storage, disappearing volumes, or data with no other copy | [Protect and recover the source](Job-failing-drive) |",
             "| The whole office or a shared service is down | [Restore office dependencies](Job-outage-recovery) |",
             "| The cause is still unclear | [Scope and triage the problem](Job-intake-triage) |", "",
             "<details>", "<summary>Decision map: from first observation to verified handoff</summary>", ""]
    lines += diagram_block(layout["homeDiagram"])
    lines += ["**Map routes:** [Containment](Job-ransomware-response) · [Data preservation](Job-failing-drive) · "
              "[Outage recovery](Job-outage-recovery) · [Triage](Job-intake-triage) · "
              "[Verification and handoff](Job-client-handoff) · [Contributions](Contribute)", "", "</details>", "",
              *heading(2, "Playbooks by work area", "🗂️ Find your work area"),
              "| Work area | Playbooks |", "| --- | --- |"]
    for group in layout["groups"]:
        links = " · ".join(f"[{by_id[identity]['title']}]({WIKI}/Job-{identity})" for identity in group["jobs"])
        lines += [f"| **{group['title']}** | {links} |"]
    lines += ["", *heading(2, "Reference", "🧰 Put the right resource within reach"),
              "| Resource | Contents |", "| --- | --- |",
              "| [Tool directory](Tool-Directory) | Use cases, supported platforms, access, official downloads and documentation. |",
              "| [Method library](Methods) | Reusable diagnostic methods, selection criteria and expected evidence. |",
              "| [Rescue media](Rescue-Media) | Boot environments, media preparation and readiness checks. |",
              "| [Intake and handoff](Technician-Handoff) | A private service record template and technician handoff criteria. |",
              "| [Service evidence](Service-Evidence) | Before-and-after measurements, validation records and useful evidence limits. |",
              "| [Tool selection](Tool-Selection) | Match a utility to the job, operating environment and commercial-use requirements. |",
              "| [Knowledge standards](Knowledge-Standards) | Authoring rules for instructions, links, diagrams, evidence and review. |",
              "| [Interface design](Interface-Design) | Visual standards for the hub, Wiki and documentation. |",
              "| [Contribution guide](Contribute) | Review and publication workflow, with a [submission template](Templates). |", "",
              *heading(2, "Service checkpoints", "📐 The INTA working standard"),
              "| Before work | During work | Before closure |", "| --- | --- | --- |",
              "| Establish scope and protect unique data. | Record the result of each purposeful test. | Repeat the real task and document the handoff. |",
              "| Choose recovery or incident response early when indicated. | Preserve rollback and investigate unchanged failures. | Contribute a proven reusable improvement. |", "",
              *heading(2, "Offline access", "🎒 Take the whole bench guide offline"),
              f"Download the [complete repository]({REPO}/archive/refs/heads/main.zip) and open `site/index.html`. Search, filters, saved tools and checklists remain available without a connection. "
              "Official downloads and external references require internet access. Keep customer records outside this public guide.", "",
              *heading(2, "Contributing", "🌱 Improve it after the next real case"),
              "Submit a corrected download, improved diagnostic branch, evaluated tool or missing verification step through the "
              "[contribution guide](Contribute). Follow the [review process](Continuous-Improvement), "
              "[record the evidence](Templates), and [publish the matching outputs](Maintaining-the-Hub)."]
    lines += page_footer(meta["reviewedAt"])
    return "\n".join(lines)


def mirror_markdown(content: str, mirrors: dict[str, str], reviewed: str) -> str:
    for title, filename in mirrors.items():
        content = content.replace(f"]({filename})", f"]({WIKI}/{title})")
        content = content.replace(f"](./{filename})", f"]({WIKI}/{title})")
    content = re.sub(r"\]\((?:\./)?runbooks/([a-z0-9-]+)\.md\)", lambda match: f"]({WIKI}/Job-{match.group(1)})", content)
    return "\n".join([page_nav(), "", content.rstrip(), *page_footer(reviewed)])


def build_outputs() -> dict[str, str]:
    tools = records(ROOT / "data/tools.json", "tools")
    jobs = records(ROOT / "data/jobs.json", "jobs")
    meta = json.loads((ROOT / "data/meta.json").read_text(encoding="utf-8"))
    layout = json.loads((ROOT / "data/wiki-layout.json").read_text(encoding="utf-8"))
    validate(tools, jobs)
    validate_layout(layout, jobs)
    linker = link_module()
    registry = linker.build_registry(tools, jobs)
    data = {"tools": tools, "jobs": jobs, "meta": meta, "links": linker.portal_links(registry)}
    data_json = json.dumps(data, ensure_ascii=False, separators=(",", ":"))
    safe_json = data_json.replace("<", "\\u003c").replace("\u2028", "\\u2028").replace("\u2029", "\\u2029")
    template = (ROOT / "portal/index.template.html").read_text(encoding="utf-8")
    replacements = {
        "{{STYLES}}": (ROOT / "portal/styles.css").read_text(encoding="utf-8"),
        "{{DATA}}": "window.INTA_DATA = " + safe_json + ";",
        "{{APP}}": (ROOT / "portal/app.js").read_text(encoding="utf-8"),
    }
    for marker, value in replacements.items():
        if marker not in template:
            raise ValueError(f"Template missing {marker}")
        template = template.replace(marker, value)
    if re.search(r"\{\{(?:STYLES|DATA|APP)\}\}", template):
        raise ValueError("Unresolved template marker")
    by_id, jobs_by_id = {x["id"]: x for x in tools}, {x["id"]: x for x in jobs}
    categories = defaultdict(list)
    for tool in tools:
        categories[tool["category"]].append(tool)
    outputs = {"site/index.html": template, "site/.nojekyll": "", "site/catalog.json": data_json + "\n"}
    for job in jobs:
        outputs[f"wiki/Job-{job['id']}.md"] = job_markdown(job, by_id, True, layout, jobs_by_id, meta["reviewedAt"])
        outputs[f"docs/runbooks/{job['id']}.md"] = job_markdown(job, by_id, False, layout, jobs_by_id, meta["reviewedAt"])
    outputs["wiki/Playbooks.md"] = playbook_index(jobs, layout, True, meta["reviewedAt"])
    outputs["docs/runbooks/README.md"] = playbook_index(jobs, layout, False, meta["reviewedAt"])
    tool_index = [page_nav(), "", *heading(1, "Tool directory", "🧰 The tool directory"),
                  "Compare use cases, platform support and access requirements. Each entry links to the official publisher, download and documentation.", "",
                  f"**{len(tools)} tools · {len(categories)} categories** · [Category list](#disciplines) · [Complete catalog](#complete-catalog)", "",
                  "| Core | Specialist | Evaluate |", "| --- | --- | --- |",
                  "| Everyday candidates for common work. | Focused tools for a defined advanced task. | Research candidates requiring comparative proof. |", "",
                  "> [!NOTE]", "> A catalog entry describes a selection, not an installed component or a certification. "
                  "Check the [selection standards](Knowledge-Standards) and the current official offering before deployment.", "",
                  anchor("disciplines"), "", *heading(2, "🧰 Categories", "🗂️ Browse by discipline"),
                  "| Category | Selection question | Tools |", "| --- | --- | --- |"]
    for category, items in sorted(categories.items()):
        info = layout["categories"].get(category, {"icon": "🧰", "question": "What result do you need?"})
        label = CATEGORY_NAMES.get(category, category)
        tool_index += [f"| [{label}]({WIKI}/Tools-{slug(category)}) | {cell(info['question'])} | {len(items)} |"]
        outputs[f"wiki/Tools-{slug(category)}.md"] = category_markdown(category, items, layout, jobs_by_id, meta["reviewedAt"])
    tool_index += ["", anchor("complete-catalog"), "", *heading(2, "Complete catalog", "📇 Every tool, one place"),
                  "| Tool | Discipline | Choose it for |", "| --- | --- | --- |"]
    for tool in sorted(tools, key=lambda item: item["name"].casefold()):
        tool_index += [f"| [{tool['name']}]({WIKI}/Tools-{slug(tool['category'])}#{tool['id']}) | "
                       f"{cell(CATEGORY_NAMES.get(tool['category'], tool['category']))} | {cell(tool['bestFor'])} |"]
    tool_index += page_footer(meta["reviewedAt"])
    outputs["wiki/Tool-Directory.md"] = "\n".join(tool_index)
    outputs["wiki/Home.md"] = home_markdown(tools, jobs, categories, layout, meta)
    sidebar = [*heading(2, "INTA knowledge base", "🧭 INTA / Service Atlas"), f"[Open technician hub]({HUB})", "",
               "- [Home](Home)", "- [Decision desk](Decision-Desk)", "- [Playbooks](Playbooks)", "- [Tool directory](Tool-Directory)",
               "- [Methods](Methods)", "- [Rescue media](Rescue-Media)", "- [Intake and handoff](Technician-Handoff)",
               "- [Service evidence](Service-Evidence)", "- [Tool selection](Tool-Selection)", "",
               *legacy_anchors("📏 The shared standard"), "", "<details>", "<summary>Standards and contributions</summary>", "",
               "- [Start here](Start-Here)", "- [Knowledge standards](Knowledge-Standards)", "- [Interface design](Interface-Design)",
               "- [Contribute](Contribute)", "- [Contribution templates](Templates)",
               "- [Continuous improvement](Continuous-Improvement)", "- [Maintain and publish](Maintaining-the-Hub)", "", "</details>", "",
               "### Playbook groups", ""]
    for group in layout["groups"]:
        sidebar += [*legacy_anchors(f"{group['icon']} {group['title']}"), "", "<details>",
                    f"<summary>{group.get('navigationLabel', group['title'])}</summary>", ""]
        sidebar += [f"- [{jobs_by_id[identity]['title']}](Job-{identity})" for identity in group["jobs"]]
        sidebar += ["", "</details>", ""]
    sidebar += [*legacy_anchors("🧰 Tool disciplines"), "", "<details>", "<summary>Tool categories</summary>", ""]
    sidebar += [f"- [{CATEGORY_NAMES.get(category, category)}](Tools-{slug(category)})" for category in sorted(categories)]
    sidebar += ["", "</details>", ""]
    outputs["wiki/_Sidebar.md"] = "\n".join(sidebar).rstrip() + "\n"
    outputs["wiki/_Footer.md"] = (f"**INTA Computer Services**\n\n"
        f"[Hub]({HUB}) · [Standards]({WIKI}/Knowledge-Standards) · "
        f"[Interface design]({WIKI}/Interface-Design) · [Contribute]({WIKI}/Contribute)\n")
    mirrors = {"Start-Here": "start-here.md", "Maintaining-the-Hub": "maintaining-the-hub.md",
               "Rescue-Media": "rescue-media.md", "Technician-Handoff": "technician-handoff.md",
               "Knowledge-Standards": "knowledge-standards.md", "Contribute": "contribute.md",
               "Templates": "contribution-template.md", "Continuous-Improvement": "continuous-improvement.md",
               "Methods": "methods.md", "Decision-Desk": "decision-desk.md",
               "Interface-Design": "interface-design.md", "Service-Evidence": "service-evidence.md",
               "Tool-Selection": "tool-selection.md"}
    for title, filename in mirrors.items():
        outputs[f"wiki/{title}.md"] = mirror_markdown((ROOT / "docs" / filename).read_text(encoding="utf-8"), mirrors, meta["reviewedAt"])
    for path, content in list(outputs.items()):
        if path.endswith(".md"):
            outputs[path] = linker.link_markdown(content, registry)
    manifest = {"schemaVersion": 2, "tools": len(tools), "jobs": len(jobs),
                "methods": len(records(ROOT / "data/methods.json", "methods")),
                "categories": dict(sorted(Counter(t["category"] for t in tools).items())),
                "contentSha256": hashlib.sha256(data_json.encode()).hexdigest(),
                "wikiLayoutSha256": hashlib.sha256(json.dumps(layout, sort_keys=True, ensure_ascii=False).encode()).hexdigest(),
                "linkRegistryEntries": len(registry),
                "registeredLinks": len(registry),
                "wikiPages": len([x for x in outputs if x.startswith("wiki/") and not Path(x).name.startswith("_")]),
                "wikiFiles": len([x for x in outputs if x.startswith("wiki/")]),
                "wikiDiagrams": sum(content.count("```mermaid") for path, content in outputs.items() if path.startswith("wiki/")),
                "files": {path: hashlib.sha256(content.encode()).hexdigest() for path, content in sorted(outputs.items())}}
    outputs["site/build-manifest.json"] = json.dumps(manifest, indent=2) + "\n"
    return outputs


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Fail if generated files differ from source")
    args = parser.parse_args()
    outputs = build_outputs()
    mismatches = []
    for relative, content in outputs.items():
        path = ROOT / relative
        if args.check:
            if not path.exists() or path.read_text(encoding="utf-8") != content:
                mismatches.append(relative)
        else:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(content, encoding="utf-8", newline="\n")
    if mismatches:
        raise SystemExit("Generated output is stale: " + ", ".join(mismatches))
    print(f"{'Verified' if args.check else 'Built'} {len(outputs)} outputs from the complete tool and job catalogs.")


if __name__ == "__main__":
    main()
