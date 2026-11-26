#!/usr/bin/env python3
"""Build the offline technician hub and live-wiki source from canonical content."""
from __future__ import annotations

import argparse
import hashlib
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


def job_markdown(job: dict, by_id: dict, wiki: bool) -> str:
    job_prefix = WIKI + "/Job-" if wiki else ""
    suffix = "" if wiki else ".md"
    nav = f"[Home]({WIKI}) · [All playbooks]({WIKI}/Playbooks) · [Interactive checklist]({HUB}#job/{job['id']})"
    lines = [nav, "", f"# {job['title']}", "", f"> {job['summary']}", "",
             "| Job profile | Details |", "| --- | --- |",
             f"| Category | {cell(job['category'])} |",
             f"| Working time | {cell(job['estimatedTime'])} |",
             f"| Experience | {cell(job['difficulty'])} |", "",
             "## Start here", "", job["startHere"], "", "## Your toolkit", ""]
    if job["tools"]:
        lines += ["| Tool | Why it belongs here |", "| --- | --- |"]
    else:
        lines += ["Use the approved private service record and the intake/handoff template. No specialist application is required for this workflow."]
    for tool_id in job["tools"]:
        tool = by_id[tool_id]
        lines.append(f"| [{tool['name']}]({tool['downloadUrl']}) | {cell(tool['bestFor'])} |")
    lines += ["", "## Follow the workflow", ""]
    for i, step in enumerate(job["steps"], 1):
        lines += [f"### {i:02d}. {step['title']}", "", f"- [ ] {step['detail']}", ""]
        if step.get("command"):
            lines += ["```text", step["command"], "```", ""]
    lines += ["## Verify before handoff", ""] + [f"- [ ] {x}" for x in job["verify"]]
    lines += ["", "## Stop and escalate when", ""] + [f"- {x}" for x in job["stopIf"]]
    if job.get("related"):
        lines += ["", "## Related playbooks", ""]
        lines += [f"- [{x.replace('-', ' ').title()}]({job_prefix}{x}{suffix})" for x in job["related"]]
    lines += ["", "## Official references", ""]
    lines += [f"- [{x['title']}]({x['url']})" for x in job["sources"]]
    lines += ["", "---", "", "INTA Computer Services · Editorial review: 2026-11-26. Timing is an estimate; stop conditions take priority.", ""]
    return "\n".join(lines)


def build_outputs() -> dict[str, str]:
    tools = records(ROOT / "data/tools.json", "tools")
    jobs = records(ROOT / "data/jobs.json", "jobs")
    meta = json.loads((ROOT / "data/meta.json").read_text(encoding="utf-8"))
    validate(tools, jobs)
    data = {"tools": tools, "jobs": jobs, "meta": meta}
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
    by_id = {x["id"]: x for x in tools}
    categories = defaultdict(list)
    for tool in tools:
        categories[tool["category"]].append(tool)
    outputs = {"site/index.html": template, "site/.nojekyll": "", "site/catalog.json": data_json + "\n"}
    for job in jobs:
        outputs[f"wiki/Job-{job['id']}.md"] = job_markdown(job, by_id, True)
        outputs[f"docs/runbooks/{job['id']}.md"] = job_markdown(job, by_id, False)
    nav = f"[Open the technician hub]({HUB}) · [Wiki home]({WIKI}) · [Source repository]({REPO})"
    tool_index = [nav, "", "# Tool directory", "", f"**{len(tools)} tools · {len(categories)} disciplines · Official publisher links**", "",
                  "Choose **Core** for everyday work, **Specialist** for a specific advanced job, and **Evaluate** for research candidates. A catalog entry is a recommendation to consider, not evidence that a tool is installed in INTA Rescue or approved on every customer system.", "",
                  "## Browse by job family", "", "| Discipline | Tools |", "| --- | --- |"]
    for category, items in sorted(categories.items()):
        category_page = "Tools-" + slug(category)
        category_label = CATEGORY_NAMES.get(category, category)
        tool_index.append(f"| [{category_label}]({WIKI}/{category_page}) | {len(items)} |")
        lines = [nav, "", f"# {category_label}", "", f"{len(items)} purpose-selected tools. Use the official download link for the current publisher offering; check architecture and release notes before installing.", "",
                 "| Tool | Best use | Platform | Kit level |", "| --- | --- | --- | --- |"]
        for tool in items:
            lines.append(f"| [{tool['name']}](#{tool['id']}) | {cell(tool['bestFor'])} | {cell(tool['platforms'])} | {tool['tier'].title()} |")
        for tool in items:
            lines += ["", f'<a name="{tool["id"]}"></a>', "", f"## {tool['name']}", "", tool["summary"], "",
                      f"**Choose it for:** {cell(tool['bestFor'])}", "",
                      f"**Platforms:** {cell(tool['platforms'])} · **Delivery:** {cell(tool['mode'])} · **Kit level:** {tool['tier'].title()}", "",
                      f"**Access / pricing:** {cell(tool['pricing'])}", "",
                      f"[Official download]({tool['downloadUrl']}) · [Documentation]({tool['docsUrl']}) · [Publisher]({tool['homepage']})", ""]
            if tool.get("cautions"):
                lines += [f"> **Before you use it:** {cell(tool['cautions'])}", ""]
            lines += [f"**Source:** {tool['sourceUrl']}  ", f"**Editorial review:** {tool['reviewedAt']}  ",
                      f"**Selection provenance:** {cell(tool['provenance'])}", ""]
        outputs[f"wiki/{category_page}.md"] = "\n".join(lines)
    tool_index += ["", "## Complete catalog", "", "| Tool | Discipline | Best use |", "| --- | --- | --- |"]
    for tool in tools:
        tool_index.append(f"| [{tool['name']}]({WIKI}/Tools-{slug(tool['category'])}#{tool['id']}) | {cell(CATEGORY_NAMES.get(tool['category'], tool['category']))} | {cell(tool['bestFor'])} |")
    outputs["wiki/Tool-Directory.md"] = "\n".join(tool_index) + "\n"
    playbooks = [nav, "", "# Playbooks", "", "Start with the symptom. Each guide includes a toolkit, an ordered checklist, verification, escalation conditions, and official references.", "",
                 "| Job | Outcome | Working time |", "| --- | --- | --- |"]
    for job in jobs:
        playbooks.append(f"| [{job['title']}]({WIKI}/Job-{job['id']}) | {cell(job['summary'])} | {cell(job['estimatedTime'])} |")
    outputs["wiki/Playbooks.md"] = "\n".join(playbooks) + "\n"
    home = [f"![INTA technician field guide]({REPO}/raw/main/docs/assets/inta-banner.svg)", "",
            "# Every fix starts here.", "", "### The shared technician home for INTA Computer Services.", "",
            f"**[Open the interactive hub →]({HUB})** · [Choose a playbook](Playbooks) · [Browse tools](Tool-Directory) · [Rescue & USB media](Rescue-Media)", "",
            f"**{len(tools)} curated tools** · **{len(jobs)} complete job guides** · **{len(categories)} tool disciplines**", "",
            "> **Start with the job.** Find the symptom below, open its playbook, choose the recommended tools, and tick through the workflow. The interactive hub remembers checklist progress and your saved tools in your browser.", "",
            "## What are you working on?", "", "| Job | Start here |", "| --- | --- |"]
    for job in jobs:
        home.append(f"| [{job['title']}](Job-{job['id']}) | {cell(job['summary'])} |")
    home += ["", "## Your bench, organized", "", "| Section | What you get |", "| --- | --- |",
             "| [Tool directory](Tool-Directory) | Official downloads, when to choose each tool, platforms, pricing and selection notes. |",
             "| [Start here](Start-Here) | The triage workflow, toolkit tiers and a clear path for a new technician. |",
             "| [Rescue media](Rescue-Media) | INTA Rescue, Windows recovery, Rufus and Ventoy, with environment boundaries. |",
             "| [Keeping the hub current](Maintaining-the-Hub) | On-demand checks, source review, testing and publication. |",
             "| [Intake and handoff template](Technician-Handoff) | A reusable service record without publishing customer data. |", "",
             "## The INTA way", "", "**Understand → protect → diagnose → repair → verify → hand off.**", "",
             "Use evidence to choose one scoped change at a time. For a failing drive, preserve data before repair attempts. For a compromised system, contain first and decide whether to rebuild. A quiet scan alone does not prove a machine is clean.", "",
             "## Take it offline", "", f"Download the complete source from [Code]({REPO}) and open `site/index.html`. Search, filters, favorites and checklists work without a connection. Official downloads and external references need internet access. Keep the HTML on your technician drive beside your rescue images.", "",
             "---", "", f"Editorial review: **{meta['reviewedAt']}**. Review date describes the guidance, not the installed version of a downloaded tool. [Contribute an improvement]({REPO}/issues/new/choose).", ""]
    outputs["wiki/Home.md"] = "\n".join(home)
    sidebar = ["## INTA / Field guide", "", "[**Open interactive hub →**](" + HUB + ")", "",
               "- [Home](Home)", "- [Start here](Start-Here)", "- [All playbooks](Playbooks)", "- [Tool directory](Tool-Directory)",
               "- [Rescue & USB media](Rescue-Media)", "- [Keep the hub current](Maintaining-the-Hub)", "- [Intake & handoff](Technician-Handoff)", "", "### Playbooks", ""]
    sidebar += [f"- [{j['title']}](Job-{j['id']})" for j in jobs]
    sidebar += ["", "### Tool disciplines", ""]
    sidebar += [f"- [{CATEGORY_NAMES.get(c,c)}](Tools-{slug(c)})" for c in sorted(categories)]
    outputs["wiki/_Sidebar.md"] = "\n".join(sidebar) + "\n"
    outputs["wiki/_Footer.md"] = f"**INTA Computer Services** · [Hub]({HUB}) · [Playbooks]({WIKI}/Playbooks) · [Tools]({WIKI}/Tool-Directory) · [Improve this guide]({REPO}/issues/new/choose)\n"
    mirrors = {"Start-Here": "start-here.md", "Maintaining-the-Hub": "maintaining-the-hub.md", "Rescue-Media": "rescue-media.md", "Technician-Handoff": "technician-handoff.md"}
    for title, filename in mirrors.items():
        outputs[f"wiki/{title}.md"] = (ROOT / "docs" / filename).read_text(encoding="utf-8")
    manifest = {"schemaVersion": 1, "tools": len(tools), "jobs": len(jobs), "categories": dict(sorted(Counter(t["category"] for t in tools).items())),
                "contentSha256": hashlib.sha256(data_json.encode()).hexdigest(),
                "wikiPages": len([x for x in outputs if x.startswith("wiki/") and not Path(x).name.startswith("_")]),
                "wikiFiles": len([x for x in outputs if x.startswith("wiki/")]),
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
