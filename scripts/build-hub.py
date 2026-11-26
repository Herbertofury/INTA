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
from datetime import date
from pathlib import Path
from urllib.parse import urlparse

ROOT = Path(__file__).resolve().parents[1]
REPO = "https://github.com/Herbertofury/INTA"
WIKI = REPO + "/wiki"
HUB = "https://herbertofury.github.io/INTA/"
QUICK_START_START = "<!-- INTA:QUICK-START:START -->"
QUICK_START_END = "<!-- INTA:QUICK-START:END -->"
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


def validate_quick_start(quick_start: dict, tools: list[dict], jobs: list[dict]) -> None:
    if not isinstance(quick_start, dict) or quick_start.get("schemaVersion") != 1:
        raise ValueError("Quick start requires schemaVersion 1")
    try:
        date.fromisoformat(quick_start.get("reviewedAt", ""))
    except (TypeError, ValueError):
        raise ValueError("Quick start requires a valid editorial review date") from None
    items = quick_start.get("items")
    if not isinstance(items, list) or not items:
        raise ValueError("Quick start requires reviewed tool selections")
    tool_ids, job_ids = {tool["id"] for tool in tools}, {job["id"] for job in jobs}
    seen_ids = set()
    required = ("id", "jobId", "toolId", "label", "why", "boundary", "freeUse", "qualityBasis")
    for item in items:
        if not isinstance(item, dict):
            raise ValueError("Quick-start entries must be objects")
        for field in required:
            if not isinstance(item.get(field), str) or not item[field].strip():
                raise ValueError(f"{item.get('id', '?')}: quick start requires {field}")
        identity = item["id"]
        if not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", identity) or identity in seen_ids:
            raise ValueError(f"Invalid or duplicate quick-start id: {identity}")
        seen_ids.add(identity)
        if item["toolId"] not in tool_ids or item["jobId"] not in job_ids:
            raise ValueError(f"{identity}: quick start references an unknown tool or playbook")
        if re.search(r"[\n\r<>\[\]|`*]", item["label"]):
            raise ValueError(f"{identity}: quick-start labels must be plain single-line text")
        icon = item.get("icon")
        if icon is not None and (not isinstance(icon, str)
                                 or not any(unicodedata.category(char) == "So" for char in icon)
                                 or any(char.isascii() and not char.isspace() for char in icon)):
            raise ValueError(f"{identity}: quick-start icons must be meaningful emoji")
        sources = item.get("evidenceUrls")
        if not isinstance(sources, list) or not sources:
            raise ValueError(f"{identity}: official evidence URLs are required")
        for url in sources:
            parsed = urlparse(url) if isinstance(url, str) else None
            if (parsed is None or parsed.scheme != "https" or not parsed.hostname or parsed.username
                    or any(key in parsed.query.lower() for key in ("utm_", "fbclid=", "gclid="))):
                raise ValueError(f"{identity}: evidence must use public HTTPS URLs without tracking")


def quick_start_markdown(quick_start: dict, tools_by_id: dict, layout: dict) -> str:
    lines = [anchor("free-tools"), "", "## ⚡ Free tools for common jobs", "",
             "Open a tool below, or select the scenario for its step-by-step guide. "
             f"[How these picks were chosen]({WIKI}/Free-Tool-Picks).", "",
             "| Scenario | Primary free tool |", "| --- | --- |"]
    for item in quick_start["items"]:
        job_id, tool = item["jobId"], tools_by_id[item["toolId"]]
        icon = item.get("icon") or layout["jobs"][job_id]["navigationIcon"]
        guide = f"{WIKI}/Job-{job_id}"
        lines += [f"| **[{icon} {cell(item['label'])}]({guide})** | **[{tool['name']}]({tool['downloadUrl']})** |"]
    lines += ["", f"Selection review: **{quick_start['reviewedAt']}** · "
              f"[Complete tool directory]({WIKI}/Tool-Directory)", ""]
    return "\n".join(lines)


def replace_quick_start_block(content: str, block: str, source_name: str) -> str:
    """Replace only the managed region, preserving every surrounding byte."""
    if content.count(QUICK_START_START) != 1 or content.count(QUICK_START_END) != 1:
        raise ValueError(f"{source_name}: exactly one quick-start marker pair is required")
    start = content.index(QUICK_START_START) + len(QUICK_START_START)
    end = content.index(QUICK_START_END)
    if end < start:
        raise ValueError(f"{source_name}: quick-start markers are reversed")
    return content[:start] + "\n\n" + block.strip() + "\n\n" + content[end:]


def validate_layout(layout: dict, jobs: list[dict], tools: list[dict] | None = None) -> None:
    def navigation_metadata(label, icon, identity):
        if (not isinstance(label, str) or not label.strip() or label != label.strip()
                or len(label) > 48 or re.search(r"[\n\r<>\[\]|`*]", label)):
            raise ValueError(f"{identity}: a short plain navigation label is required")
        if (not isinstance(icon, str) or len(icon) > 8
                or not any(unicodedata.category(char) == "So" for char in icon)
                or any(char.isascii() and not char.isspace() for char in icon)):
            raise ValueError(f"{identity}: a purposeful navigation emoji is required")

    def complete_order(key, expected_ids):
        order = layout.get(key, [])
        if not isinstance(order, list) or set(order) != expected_ids or len(order) != len(expected_ids):
            raise ValueError(f"{key}: every destination must appear exactly once")

    expected = {job["id"] for job in jobs}
    if set(layout.get("jobs", {})) != expected:
        raise ValueError("Wiki layout must cover every playbook exactly")
    grouped = [identity for group in layout.get("groups", []) for identity in group["jobs"]]
    if set(grouped) != expected or len(grouped) != len(expected):
        raise ValueError("Wiki navigation must group every playbook exactly once")
    complete_order("scenarioOrder", expected)
    complete_order("categoryOrder", set(layout.get("categories", {})))
    for identity, config in layout["jobs"].items():
        navigation_metadata(config.get("navigationLabel"), config.get("navigationIcon"), identity)
    for identity, config in layout.get("categories", {}).items():
        navigation_metadata(config.get("navigationLabel"), config.get("navigationIcon"), identity)
    shortcuts = layout.get("quickDownloads", [])
    if not isinstance(shortcuts, list) or not shortcuts:
        raise ValueError("Quick downloads must contain useful official routes")
    shortcut_ids = []
    for shortcut in shortcuts:
        navigation_metadata(shortcut.get("label"), shortcut.get("icon"), shortcut.get("toolId", "quick download"))
        if shortcut.get("routeType") not in {"download", "setup"}:
            raise ValueError("Quick download routes must distinguish downloads from built-in setup")
        shortcut_ids.append(shortcut.get("toolId"))
    if len(shortcut_ids) != len(set(shortcut_ids)):
        raise ValueError("Quick downloads must not repeat a tool")
    if tools is not None:
        if set(layout["categories"]) != {tool["category"] for tool in tools}:
            raise ValueError("Wiki category navigation must cover the complete tool catalog")
        missing = set(shortcut_ids) - {tool["id"] for tool in tools}
        if missing:
            raise ValueError(f"Quick downloads reference unknown tools: {sorted(missing)}")
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


def scenario_link(identity: str, layout: dict, absolute: bool = False) -> str:
    config = layout["jobs"][identity]
    destination = f"{WIKI}/Job-{identity}" if absolute else f"Job-{identity}"
    return f"**[{config['navigationIcon']} {config['navigationLabel']}]({destination})**"


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


def home_markdown(tools: list[dict], jobs: list[dict], categories: dict, layout: dict, meta: dict,
                  quick_start_block: str) -> str:
    by_id = {job["id"]: job for job in jobs}
    lines = [f"![INTA Computer Services — technical knowledge base]({REPO}/raw/main/docs/assets/wiki-hero.svg)", "",
             *heading(1, "INTA knowledge base", "🧭 Your next good fix starts here."),
             "Service procedures, tool references and troubleshooting methods for the INTA team.", "",
             f"[Open technician hub]({HUB}) · [Decision desk](Decision-Desk) · [Playbooks](Playbooks) · [Tool directory](Tool-Directory) · [Methods](Methods)", "",
             f"**{len(tools)} tools · {len(jobs)} playbooks · {len(categories)} tool categories**", "",
             quick_start_block.rstrip(), "",
             *heading(2, "🧭 Start a job", "🚦 First, choose the right route"),
             *legacy_anchors("Playbooks by work area"), *legacy_anchors("🗂️ Find your work area"), "",
             "| Scenario | Outcome |", "| --- | --- |"]
    for identity in layout["scenarioOrder"]:
        lines += [f"| {scenario_link(identity, layout)} | {cell(by_id[identity]['summary'])} |"]
    lines += ["", *heading(2, "Before you begin"),
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
              *heading(2, "Reference", "🧰 Put the right resource within reach"),
              "| Resource | Contents |", "| --- | --- |",
              "| [Tool directory](Tool-Directory) | Use cases, supported platforms, access, official downloads and documentation. |",
              "| [Method library](Methods) | Reusable diagnostic methods, selection criteria and expected evidence. |",
              "| [Rescue media](Rescue-Media) | Boot environments, media preparation and readiness checks. |",
              "| [Intake and handoff](Technician-Handoff) | A private service record template and technician handoff criteria. |",
              "| [Service evidence](Service-Evidence) | Before-and-after measurements, validation records and useful evidence limits. |",
              "| [Tool selection](Tool-Selection) | Match a utility to the job, operating environment and commercial-use requirements. |",
              "| [Free Tool Picks](Free-Tool-Picks) | Business-use terms, practical limits and source evidence for the quick-start selections. |",
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
    quick_start = json.loads((ROOT / "data/quick-start.json").read_text(encoding="utf-8"))
    validate(tools, jobs)
    validate_layout(layout, jobs, tools)
    validate_quick_start(quick_start, tools, jobs)
    if not (ROOT / "docs/free-tool-picks.md").is_file():
        raise ValueError("Quick-start selections require docs/free-tool-picks.md for their evidence and limits")
    linker = link_module()
    registry = linker.build_registry(tools, jobs)
    data = {"tools": tools, "jobs": jobs, "meta": meta, "links": linker.portal_links(registry),
            "quickStart": quick_start}
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
    quick_start_block = linker.link_markdown(quick_start_markdown(quick_start, by_id, layout), registry)
    managed_sources = {
        name: replace_quick_start_block((ROOT / name).read_bytes().decode("utf-8"), quick_start_block, name)
        for name in ("README.md", "docs/start-here.md")
    }
    categories = defaultdict(list)
    for tool in tools:
        categories[tool["category"]].append(tool)
    outputs = {"site/index.html": template, "site/.nojekyll": "", "site/catalog.json": data_json + "\n"}
    outputs.update(managed_sources)
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
    for category in layout["categoryOrder"]:
        items = categories[category]
        info = layout["categories"].get(category, {"icon": "🧰", "question": "What result do you need?"})
        tool_index += [f"| **[{info['navigationIcon']} {info['navigationLabel']}]({WIKI}/Tools-{slug(category)})** | {cell(info['question'])} | {len(items)} |"]
        outputs[f"wiki/Tools-{slug(category)}.md"] = category_markdown(category, items, layout, jobs_by_id, meta["reviewedAt"])
    tool_index += ["", anchor("complete-catalog"), "", *heading(2, "Complete catalog", "📇 Every tool, one place"),
                  "| Tool | Discipline | Choose it for | Official download / access |", "| --- | --- | --- | --- |"]
    for tool in sorted(tools, key=lambda item: item["name"].casefold()):
        tool_index += [f"| [{tool['name']}]({WIKI}/Tools-{slug(tool['category'])}#{tool['id']}) | "
                       f"{cell(CATEGORY_NAMES.get(tool['category'], tool['category']))} | {cell(tool['bestFor'])} | "
                       f"**[↗ Open official page]({tool['downloadUrl']})** |"]
    tool_index += page_footer(meta["reviewedAt"])
    outputs["wiki/Tool-Directory.md"] = "\n".join(tool_index)
    outputs["wiki/Home.md"] = home_markdown(tools, jobs, categories, layout, meta, quick_start_block)
    sidebar = [*heading(2, "INTA knowledge base", "🧭 INTA / Service Atlas"),
               f"**[🏠 Home](Home)** · **[🧰 Open hub]({HUB})**", "",
               *heading(3, "Fix a problem", "Playbook groups")]
    group_anchors = {group["jobs"][0]: "".join(legacy_anchors(f"{group['icon']} {group['title']}"))
                     for group in layout["groups"]}
    for identity in layout["scenarioOrder"]:
        sidebar += [f"- {group_anchors.get(identity, '')}{scenario_link(identity, layout)}"]
    sidebar += ["", "### Quick tool downloads", "",
                "Official download pages; built-in tools link to setup instructions.", ""]
    for shortcut in layout["quickDownloads"]:
        tool = by_id[shortcut["toolId"]]
        sidebar += [f"- **[{shortcut['icon']} {shortcut['label']}]({tool['downloadUrl']})**"]
    sidebar += ["", *heading(3, "Tool categories", "🧰 Tool disciplines")]
    for category in layout["categoryOrder"]:
        info = layout["categories"][category]
        sidebar += [f"- **[{info['navigationIcon']} {info['navigationLabel']}](Tools-{slug(category)})**"]
    sidebar += ["", "### Reference", "",
                "- [Decision desk](Decision-Desk)", "- [All playbooks](Playbooks)", "- [Complete tool directory](Tool-Directory)",
                "- [Methods](Methods)", "- [Rescue media](Rescue-Media)", "- [Intake and handoff](Technician-Handoff)",
                "- [Service evidence](Service-Evidence)", "- [Tool selection](Tool-Selection)",
                "- [Free Tool Picks](Free-Tool-Picks)", "",
                *legacy_anchors("📏 The shared standard"), "", "### Standards and contributions", "",
                "- [Start here](Start-Here)", "- [Knowledge standards](Knowledge-Standards)", "- [Interface design](Interface-Design)",
                "- [Contribute](Contribute)", "- [Contribution templates](Templates)",
                "- [Continuous improvement](Continuous-Improvement)", "- [Maintain and publish](Maintaining-the-Hub)", ""]
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
               "Tool-Selection": "tool-selection.md", "Free-Tool-Picks": "free-tool-picks.md"}
    for title, filename in mirrors.items():
        source_name = f"docs/{filename}"
        source = managed_sources[source_name] if source_name in managed_sources else (ROOT / source_name).read_text(encoding="utf-8")
        outputs[f"wiki/{title}.md"] = mirror_markdown(source, mirrors, meta["reviewedAt"])
    for path, content in list(outputs.items()):
        if path.endswith(".md") and path not in managed_sources:
            outputs[path] = linker.link_markdown(content, registry)
    manifest = {"schemaVersion": 2, "tools": len(tools), "jobs": len(jobs),
                "methods": len(records(ROOT / "data/methods.json", "methods")),
                "quickStartItems": len(quick_start["items"]),
                "quickStartSha256": hashlib.sha256(json.dumps(quick_start, sort_keys=True, ensure_ascii=False).encode()).hexdigest(),
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
