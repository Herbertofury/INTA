#!/usr/bin/env python3
"""Build the offline technician hub and live-wiki source from canonical content."""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import html
import json
import re
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
        for step in job["steps"]:
            required(step, ["title", "detail"])
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
    return (f"[🏠 Home]({WIKI}) · [🧭 Decision desk]({WIKI}/Decision-Desk) · "
            f"[📚 Playbooks]({WIKI}/Playbooks) · [🧰 Tool directory]({WIKI}/Tool-Directory) · "
            f"[✨ Open interactive hub]({HUB})")


def page_footer(reviewed: str, wiki: bool = True) -> list[str]:
    lines = ["", "---", "", f"**Source review: {reviewed}.** Guidance review and tool release versions are separate. "
             f"[Report a correction]({REPO}/issues/new/choose) · [How we maintain this guide]({WIKI}/Knowledge-Standards)", ""]
    if not wiki:
        lines += [f"[← All playbooks](README.md) · [Live Wiki]({WIKI}) · [Contribution guide]({REPO}/blob/main/CONTRIBUTING.md)", ""]
    return lines


def diagram_block(source: str) -> list[str]:
    return ["```mermaid", source.rstrip(), "```", ""]


def anchor(identity: str) -> str:
    return f'<a id="{identity}"></a>'


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
    lines = [page_nav(), "", f"# {config['icon']} {job['title']}", "", job["summary"], "",
             "| Work area | Experience | Plan your time | Workflow |", "| --- | --- | --- | --- |",
             f"| {cell(job['category'])} | {cell(job['difficulty'])} | {cell(job['estimatedTime'])} | {len(job['steps'])} steps + acceptance |", "",
             f"**[▶ Open the interactive checklist]({HUB}#job/{job['id']})** · [Decision route](#decision-route) · "
             "[Toolkit](#toolkit) · [Walkthrough](#walkthrough) · [Acceptance](#acceptance) · [Sources](#sources)", "",
             "> [!TIP]", f"> **Your starting move:** {job['startHere']}", "",
             anchor("decision-route"), "", "## 🧭 Choose the right branch", ""]
    lines += diagram_block(config["diagram"])
    routes = ["[Continue this workflow](#walkthrough)", "[Acceptance checks](#acceptance)"]
    routes += [f"[{jobs_by_id[identity]['title']}]({destination(identity)})" for identity in job.get("related", [])]
    lines += ["**Diagram routes:** " + " · ".join(routes), ""]
    lines += ["| Checkpoint | What to carry forward |", "| --- | --- |"]
    lines += [f"| **{cell(label)}** | {cell(detail)} |" for label, detail in config["checkpoints"].items()]
    lines += ["", anchor("toolkit"), "", "## 🧰 Build the right toolkit", ""]
    if job["tools"]:
        lines += ["Choose the tools that answer the case’s next question. Opening a tool is a separate decision from changing the customer’s system.", "",
                  "| Tool / official download | Use it for | Platform & access | Documentation |", "| --- | --- | --- | --- |"]
        for tool_id in job["tools"]:
            tool = by_id[tool_id]
            lines += [f"| [{tool['name']}]({tool['downloadUrl']}) | {cell(tool['bestFor'])} | "
                      f"{cell(tool['platforms'])}; {cell(tool['pricing'])} | [Open guide]({tool['docsUrl']}) |"]
    else:
        lines += [f"Use the approved private service record and the [intake and handoff template]({WIKI}/Technician-Handoff). No specialist application is required."]
    lines += ["", anchor("walkthrough"), "", "## 🪜 Work the case", "",
              "Work in order, record the result of each step, and change route when the evidence calls for it.", ""]
    for number, step in enumerate(job["steps"], 1):
        lines += [anchor(f"step-{number:02d}"), "", f"### {number:02d} · {step['title']}", "", f"- [ ] {step['detail']}", ""]
        if step.get("command"):
            lines += ["**Command for the environment described above:**", "", "```text", step["command"], "```", ""]
            lines += [f"**Command legend:** {command_reference(step, by_id)} · [Return to the step context](#step-{number:02d}).", ""]
    lines += [anchor("acceptance"), "", "## ✅ Prove the result", ""]
    lines += [f"- [ ] {item}" for item in job["verify"]]
    lines += ["", "> [!WARNING]", "> **Pause this route and escalate when:**"]
    lines += [f"> - {item}" for item in job["stopIf"]]
    lines += ["", "## 🔀 Continue with the evidence", ""]
    if job.get("related"):
        lines += ["| Next playbook | What it resolves |", "| --- | --- |"]
        for identity in job["related"]:
            other = jobs_by_id[identity]
            lines += [f"| [{other['title']}]({destination(identity)}) | {cell(other['summary'])} |"]
    lines += ["", f"Found a reusable improvement? [Contribute the correction]({WIKI}/Contribute) and include the test that proved it.", "",
              anchor("sources"), "", "## 🔎 Source trail", "", "<details>", "<summary>Official references and editorial review</summary>", ""]
    lines += [f"- [{source['title']}]({source['url']})" for source in job["sources"]]
    lines += ["", f"Reviewed **{job.get('reviewedAt', reviewed)}**. Timing is a planning range. "
              "A successful command is evidence about that operation; acceptance belongs to the user’s original task.", "", "</details>"]
    lines += page_footer(job.get("reviewedAt", reviewed), wiki)
    return "\n".join(lines)


def tier_label(value: str) -> str:
    return {"core": "🟢 Core", "specialist": "🔬 Specialist", "evaluate": "🧪 Evaluate"}[value]


def category_markdown(category: str, tools: list[dict], layout: dict, jobs: dict, reviewed: str) -> str:
    info = layout["categories"].get(category, {"icon": "🧰", "intro": "Choose a tool for the observed job.", "jobs": [], "question": "What result do you need?"})
    label = CATEGORY_NAMES.get(category, category)
    ordered = sorted(tools, key=lambda t: ({"core": 0, "specialist": 1, "evaluate": 2}[t["tier"]], t["name"].casefold()))
    lines = [page_nav(), "", f"# {info['icon']} {label}", "", info["intro"], "",
             f"**{len(tools)} tools** · [All disciplines]({WIKI}/Tool-Directory) · [Selection standards]({WIKI}/Knowledge-Standards)", "",
             "> [!TIP]", f"> **Choose by the job:** {info['question']}", "", "## 🧭 Start with a workflow", ""]
    lines += [f"- [{jobs[identity]['title']}]({WIKI}/Job-{identity})" for identity in info["jobs"]]
    lines += ["", "## ⚖️ Compare the options", "", "| Tool | Choose it for | Platforms | Role |", "| --- | --- | --- | --- |"]
    for tool in ordered:
        lines += [f"| [{tool['name']}](#{tool['id']}) | {cell(tool['bestFor'])} | {cell(tool['platforms'])} | {tier_label(tool['tier'])} |"]
    lines += ["", "Core means an everyday candidate; Specialist means a focused advanced use; Evaluate means prove it in a suitable test environment before promotion.", ""]
    for tool in ordered:
        lines += ["---", "", anchor(tool["id"]), "", f"## {tool['name']}", "", tool["summary"], "",
                  f"**[{tool['name']}: official download →]({tool['downloadUrl']})** · [Documentation]({tool['docsUrl']}) · [Publisher]({tool['homepage']})", "",
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
        lines += ["", "</details>", "", "[↑ Compare the options](#compare-options)", ""]
    lines = [line.replace("## ⚖️ Compare the options", anchor("compare-options") + "\n\n## ⚖️ Compare the options") for line in lines]
    lines += page_footer(reviewed)
    return "\n".join(lines)


def playbook_index(jobs: list[dict], layout: dict, wiki: bool, reviewed: str) -> str:
    by_id = {job["id"]: job for job in jobs}
    destination = lambda identity: f"{WIKI}/Job-{identity}" if wiki else f"{identity}.md"
    lines = [page_nav(), "", "# 📚 The playbook library", "",
             "Find the symptom, follow the evidence, and leave a tested result. Every playbook includes a decision route, the right toolkit, a numbered walkthrough, and handoff checks.", "",
             f"**{len(jobs)} complete playbooks** · [Choose at the decision desk]({WIKI}/Decision-Desk) · [Browse methods]({WIKI}/Methods)", "",
             " · ".join(f"[{group['icon']} {group['title']}](#{group['id']})" for group in layout["groups"]), ""]
    for group in layout["groups"]:
        lines += [anchor(group["id"]), "", f"## {group['icon']} {group['title']}", "", group["summary"], "",
                  "| Playbook | Outcome | Experience | Plan your time |", "| --- | --- | --- | --- |"]
        for identity in group["jobs"]:
            job = by_id[identity]
            lines += [f"| [{job['title']}]({destination(identity)}) | {cell(job['summary'])} | {job['difficulty']} | {cell(job['estimatedTime'])} |"]
        lines += [""]
    lines += ["## 🤝 Make the next case easier", "", f"Use the [contribution template]({WIKI}/Templates) when a case reveals a better sequence, a changed tool, or a missing stop condition. "
              f"Keep customer evidence in the private service record and add only the reusable method to the [shared knowledge process]({WIKI}/Continuous-Improvement)."]
    lines += page_footer(reviewed, wiki)
    return "\n".join(lines)


def home_markdown(tools: list[dict], jobs: list[dict], categories: dict, layout: dict, meta: dict) -> str:
    by_id = {job["id"]: job for job in jobs}
    lines = [f"![INTA Service Atlas: find the right path, leave a better one]({REPO}/raw/main/docs/assets/wiki-hero.svg)", "",
             "# 🧭 Your next good fix starts here.", "",
             "The shared home for INTA Computer Services: a clear route through the job, the tools to do it, and the proof that it worked.", "",
             f"**[✨ Open the interactive technician hub]({HUB})** · [Find a playbook](Playbooks) · [Browse every tool](Tool-Directory) · [Contribute a better way](Contribute)", "",
             "| Ready on the bench | Knowledge with a path | Built to keep improving |", "| --- | --- | --- |",
             f"| **{len(tools)} selected tools** across {len(categories)} disciplines | **{len(jobs)} complete job guides** with decision maps | [Standards](Knowledge-Standards), [templates](Templates), and [manual review](Continuous-Improvement) |", "",
             "> [!TIP]", "> **New case?** Start at the [decision desk](Decision-Desk). **Already know the job?** Open its [playbook](Playbooks). **Need a download?** Use the [tool directory](Tool-Directory).", "",
             "## 🚦 First, choose the right route", ""]
    lines += diagram_block(layout["homeDiagram"])
    lines += ["| What you see | Open this route first |", "| --- | --- |",
              "| Encryption spreading, unusual privileged access, or several affected machines | [🚨 Contain a wider compromise](Job-ransomware-response) |",
              "| Clicking storage, disappearing volumes, or data with no other copy | [💾 Protect and recover the source](Job-failing-drive) |",
              "| The whole office or a shared service is down | [🔌 Restore office dependencies](Job-outage-recovery) |",
              "| The cause is still unclear | [🧭 Scope and triage the problem](Job-intake-triage) |", "",
              "## 🗂️ Find your work area", "", "| Work area | Pick a playbook |", "| --- | --- |"]
    for group in layout["groups"]:
        links = " · ".join(f"[{by_id[identity]['title']}]({WIKI}/Job-{identity})" for identity in group["jobs"])
        lines += [f"| **{group['icon']} {group['title']}** | {links} |"]
    lines += ["", "## 🧰 Put the right resource within reach", "",
              "| Resource | What you can do here |", "| --- | --- |",
              "| [🧰 Tool directory](Tool-Directory) | Compare use cases, platforms, access, official downloads, and documentation. |",
              "| [🧠 Method library](Methods) | Learn repeatable ways to isolate a cause, protect the source, and prove the result. |",
              "| [🥾 Rescue media](Rescue-Media) | Choose and maintain the boot environment for the task. |",
              "| [🤝 Intake and handoff](Technician-Handoff) | Start a clear private service record and leave the next technician a useful checkpoint. |",
              "| [📏 Knowledge standards](Knowledge-Standards) | Follow the shared rules for clear instructions, evidence, links, and review. |",
              "| [🛠️ Contribute](Contribute) | Submit a tested correction or a useful new guide with the [template](Templates). |", "",
              "## 📐 The INTA working standard", "",
              "| At the start | While you work | Before you close |", "| --- | --- | --- |",
              "| Establish scope and protect unique data. | Follow one evidence-led test and keep the result. | Repeat the real task and document the handoff. |",
              "| Choose the appropriate recovery or incident route early. | Preserve a usable rollback and avoid repeating unchanged failures. | Turn a proven improvement into a reusable contribution. |", "",
              "## 🎒 Take the whole bench guide offline", "",
              f"Download the [complete repository]({REPO}/archive/refs/heads/main.zip) and open `site/index.html`. The hub keeps search, filters, saved tools, and checklists available without a connection. "
              "Official downloads and external reference pages need internet access. Keep customer records outside this public guide.", "",
              "## 🌱 Improve it after the next real case", "",
              "A changed download, a clearer branch, a newly proven tool, or a missing verification step is a useful contribution. "
              "[See the review loop](Continuous-Improvement), [use the template](Templates), and [check the publication workflow](Maintaining-the-Hub)."]
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
    tool_index = [page_nav(), "", "# 🧰 The tool directory", "",
                  "Find the tool that fits the job, compare its role, and go straight to its official source.", "",
                  f"**{len(tools)} tools · {len(categories)} disciplines · Official publisher links**", "",
                  "| 🟢 Core | 🔬 Specialist | 🧪 Evaluate |", "| --- | --- | --- |",
                  "| Everyday candidates for common work. | Focused tools for a defined advanced task. | Research candidates requiring comparative proof. |", "",
                  "> [!NOTE]", "> A catalog entry describes a selection, not an installed component or a certification. "
                  "Check the [selection standards](Knowledge-Standards) and the current official offering before deployment.", "",
                  anchor("disciplines"), "", "## 🗂️ Browse by discipline", "", "| Discipline | Start by asking | Tools |", "| --- | --- | --- |"]
    for category, items in sorted(categories.items()):
        info = layout["categories"].get(category, {"icon": "🧰", "question": "What result do you need?"})
        label = CATEGORY_NAMES.get(category, category)
        tool_index += [f"| [{info['icon']} {label}]({WIKI}/Tools-{slug(category)}) | {cell(info['question'])} | {len(items)} |"]
        outputs[f"wiki/Tools-{slug(category)}.md"] = category_markdown(category, items, layout, jobs_by_id, meta["reviewedAt"])
    tool_index += ["", anchor("complete-catalog"), "", "## 📇 Every tool, one place", "",
                  "| Tool | Discipline | Choose it for |", "| --- | --- | --- |"]
    for tool in sorted(tools, key=lambda item: item["name"].casefold()):
        tool_index += [f"| [{tool['name']}]({WIKI}/Tools-{slug(tool['category'])}#{tool['id']}) | "
                       f"{cell(CATEGORY_NAMES.get(tool['category'], tool['category']))} | {cell(tool['bestFor'])} |"]
    tool_index += page_footer(meta["reviewedAt"])
    outputs["wiki/Tool-Directory.md"] = "\n".join(tool_index)
    outputs["wiki/Home.md"] = home_markdown(tools, jobs, categories, layout, meta)
    sidebar = ["## 🧭 INTA / Service Atlas", "", f"**[✨ Open interactive hub]({HUB})**", "",
               "- [🏠 Home](Home)", "- [🚦 Decision desk](Decision-Desk)", "- [📚 All playbooks](Playbooks)", "- [🧰 Tool directory](Tool-Directory)",
               "- [🧠 Method library](Methods)", "- [🥾 Rescue & USB media](Rescue-Media)", "- [🤝 Intake & handoff](Technician-Handoff)", "",
               "### 📏 The shared standard", "", "- [Start here](Start-Here)", "- [Knowledge standards](Knowledge-Standards)",
               "- [Contribute an improvement](Contribute)", "- [Contribution templates](Templates)",
               "- [Continuous improvement](Continuous-Improvement)", "- [Maintain and publish](Maintaining-the-Hub)", ""]
    for group in layout["groups"]:
        sidebar += [f"### {group['icon']} {group['title']}", ""]
        sidebar += [f"- [{jobs_by_id[identity]['title']}](Job-{identity})" for identity in group["jobs"]]
        sidebar += [""]
    sidebar += ["### 🧰 Tool disciplines", ""]
    sidebar += [f"- [{layout['categories'].get(category, {}).get('icon', '🧰')} {CATEGORY_NAMES.get(category, category)}](Tools-{slug(category)})" for category in sorted(categories)]
    outputs["wiki/_Sidebar.md"] = "\n".join(sidebar) + "\n"
    outputs["wiki/_Footer.md"] = (f"**🧭 INTA Computer Services · Service Atlas**\n\n"
        f"[Hub]({HUB}) · [Decision desk]({WIKI}/Decision-Desk) · [Tools]({WIKI}/Tool-Directory) · "
        f"[Standards]({WIKI}/Knowledge-Standards) · [Contribute]({WIKI}/Contribute)\n\n"
        "**Find the right path. Leave a better one.**\n")
    mirrors = {"Start-Here": "start-here.md", "Maintaining-the-Hub": "maintaining-the-hub.md",
               "Rescue-Media": "rescue-media.md", "Technician-Handoff": "technician-handoff.md",
               "Knowledge-Standards": "knowledge-standards.md", "Contribute": "contribute.md",
               "Templates": "contribution-template.md", "Continuous-Improvement": "continuous-improvement.md",
               "Methods": "methods.md", "Decision-Desk": "decision-desk.md"}
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
