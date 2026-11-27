#!/usr/bin/env python3
"""Build the offline technician hub and live-wiki source from canonical content."""
from __future__ import annotations

import argparse
import base64
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
WIKI_MIRRORS = {
    "Start-Here": "start-here.md", "Maintaining-the-Hub": "maintaining-the-hub.md",
    "Rescue-Media": "rescue-media.md", "Technician-Handoff": "technician-handoff.md",
    "Knowledge-Standards": "knowledge-standards.md", "Contribute": "contribute.md",
    "Quick-Commands": "quick-commands.md",
    "Command-Center": "command-center.md",
    "Updates-and-Maintenance": "updates-center.md",
    "Templates": "contribution-template.md", "Continuous-Improvement": "continuous-improvement.md",
    "Methods": "methods.md", "Decision-Desk": "decision-desk.md",
    "Interface-Design": "interface-design.md", "Service-Evidence": "service-evidence.md",
    "Tool-Selection": "tool-selection.md", "Free-Tool-Picks": "free-tool-picks.md",
    "Current-Tool-Choices": "reference/scenario-tool-review-2026-11-26.md",
    "Malware-Tool-Choices": "reference/malware-tool-review-2026-11-26.md",
    "Knowledge-Organization": "knowledge-organization.md",
    "Research-Updates": "reference/catalog-expansion-2026-11-26.md",
    "Online-Tool-Review": "reference/online-tools-review-2026-11-26.md",
}
CATEGORY_NAMES = {
    "malware": "Malware removal & endpoint protection", "remote": "Remote support",
    "windows": "Windows diagnosis & repair", "storage": "Storage, recovery & backup",
    "hardware": "Hardware diagnostics", "network": "Networking",
    "boot": "Bootable media", "productivity": "Technician productivity",
    "ai": "AI assistance", "security": "Advanced defensive security", "online": "Online website tools",
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
        command_count = 0
        for number, step in enumerate(job["steps"], 1):
            required(step, ["title", "detail"])
            step_ids.append(step_anchor(step, number))
            if step.get("command"):
                command_count += 1
                required(step, ["commandShell", "commandPurpose", "commandSafety", "commandUrl", "commandName"])
                if step["commandShell"] not in {"CMD", "PowerShell", "CMD or PowerShell", "PowerShell or CMD (Run as Administrator)", "CMD (Run as Administrator)", "PowerShell (Run as Administrator)", "Linux shell (INTA Rescue)"}:
                    raise ValueError(f"{job['id']}: unsupported command shell label")
                command_url = urlparse(step["commandUrl"])
                if command_url.scheme != "https" or not command_url.hostname or command_url.username or command_url.password:
                    raise ValueError(f"{job['id']}: command reference must be public HTTPS")
            for shortcut in step.get("shortcuts", []):
                required(shortcut, ["label"])
                kind = set(shortcut) - {"label"}
                if kind == {"tool"} and shortcut["tool"] in tool_ids:
                    continue
                if kind == {"job"} and shortcut["job"] in job_ids:
                    continue
                if kind == {"url"}:
                    url = urlparse(shortcut["url"])
                    if url.scheme == "https" and url.hostname and not (url.username or url.password):
                        continue
                raise ValueError(f"{job['id']}: invalid step shortcut: {shortcut}")
        if command_count == 0:
            raise ValueError(f"{job['id']}: every playbook needs at least one copy-ready diagnostic or action command")
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


def knowledge_module():
    spec = importlib.util.spec_from_file_location("inta_knowledge_health", ROOT / "scripts/knowledge_health.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def brand_module(filename="brand_icons.py"):
    spec = importlib.util.spec_from_file_location("inta_" + filename.removesuffix(".py"), ROOT / "scripts" / filename)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def field_note_index(notes: list[dict]) -> list[str]:
    if not notes:
        return []
    return ["### 🎯 Go straight to a specific fix", "",
            *[f"- **[{note['title']}](#fix-{note['id']})** — {note['symptom']}" for note in notes], ""]


def field_notes_markdown(notes: list[dict], by_id: dict, access: dict, workspace: dict) -> list[str]:
    lines = [anchor("specific-fixes"), "", "## 🎯 Specific fixes", "",
             "Choose the case that matches the evidence. Each case gives a first action, a short procedure and a result to prove.", ""]
    for note in notes:
        lines += [anchor("fix-" + note["id"]), "", f"### {note['title']}", "",
                  f"**Use when:** {note['symptom']}", "", f"**Start here:** {note['firstAction']}", ""]
        for tool_id in note["toolIds"]:
            tool = by_id[tool_id]
            selection = next((item for item in workspace.get("tools", []) if item["toolId"] == tool_id), {})
            lines += [f"**[{tool['name']}]({access[tool_id]['officialUrl']})** · "
                      + tool_access_links(tool, access, action_id=selection.get("actionId")), ""]
        lines += [f"{number}. {step}" for number, step in enumerate(note["steps"], 1)]
        if note.get("commands"):
            lines += ["", "#### ⚡ Quick commands", ""]
            for command in note["commands"]:
                language = "powershell" if "PowerShell" in command["shell"] else "bash" if "Linux" in command["shell"] else "cmd"
                lines += [f"**{command['label']}** · {command['shell']}", "", command["purpose"], "",
                          f"```{language}", command["code"], "```", "",
                          f"> **Before running:** {command['safety']}", ""]
                if command.get("sourceUrl"):
                    lines += [f"**Official reference:** [{command.get('sourceTitle', 'Documentation')}]({command['sourceUrl']})", ""]
        lines += ["", f"**Prove it worked:** {note['expected']}", "",
                  f"**If this does not resolve it:** {note['ifUnresolved']}", "",
                  "**Official evidence:** " + " · ".join(f"[Source {number}]({url})" for number, url in enumerate(note["sourceUrls"], 1))
                  + f" · Reviewed {note['reviewedAt']}", "",
                  "[Back to specific fixes](#specific-fixes) · [Full procedure](#walkthrough)", ""]
    return lines


def service_map_markdown(jobs: list[dict], layout: dict, field_notes: dict, workspaces: dict) -> str:
    jobs_by_id = {job["id"]: job for job in jobs}
    grouped_notes = defaultdict(list)
    for note in field_notes["notes"]:
        grouped_notes[note["jobId"]].append(note)
    lines = [page_nav(), "", "# 🗺️ Service map", "",
             "**Choose the field, then the exact job or fix.** Case titles open that fix in the working view; full-guide links open the Wiki. Every guide has one primary home.", "",
             f"**[Open the interactive service map]({HUB}#services)** · [Urgent answers](Home) · [All tools](Tool-Directory)", "",
             " · ".join(f"**[{group['icon']} {group['navigationLabel']}](#area-{group['id']})**" for group in layout["groups"]), ""]
    for group in layout["groups"]:
        lines += [anchor("area-" + group["id"]), "", f"## {group['icon']} {group['title']}", "", group["summary"], ""]
        for identity in group["jobs"]:
            job, notes, workspace = jobs_by_id[identity], grouped_notes[identity], workspaces["jobs"][identity]
            config = layout["jobs"][identity]
            lines += [f"### [{config['navigationIcon']} {config['navigationLabel']}]({WIKI}/Job-{identity})", "",
                      f"**First move:** {workspace['startHere']}", "",
                      f"**[Working view]({HUB}#job/{identity})** · **[Full guide]({WIKI}/Job-{identity})** · "
                      f"{len(notes)} specific fixes · {len(workspace['resources'])} official resources", ""]
            lines += [f"- **[{note['title']}]({HUB}#job/{identity}?section=fix-{note['id']})** — {note['symptom']}" for note in notes]
            lines += [""]
    lines += ["## How this stays organized", "",
              "The service groups come from one [canonical layout](https://github.com/Herbertofury/INTA/blob/main/data/wiki-layout.json). "
              "New cases extend the existing guide. A new guide needs a distinct job, a complete procedure and a place in this map.", "",
              "**[Organization rules](Knowledge-Organization)** · **[Coverage and review health](Knowledge-Health)** · **[Research queue](Research-Queue)**", ""]
    return "\n".join(lines)


def research_queue_markdown(queue: dict, jobs: list[dict], layout: dict) -> str:
    lines = [page_nav(), "", "# 🔎 Research queue", "",
             "**Concrete next investigations, organized by the job they improve.** A queued item is planned work; it is not a completed test or a recommendation.", "",
             "Correct harmful guidance and broken routes first. Then use [review health](Knowledge-Health) and the oldest-reviewed service area to choose substantial work. "
             "Complete and publish a coherent improvement before opening another equivalent investigation.", ""]
    targets = [("job", job["id"], job["title"], f"{WIKI}/Job-{job['id']}") for job in jobs]
    targets += [("category", key, layout["categories"][key]["navigationLabel"], f"{WIKI}/Tools-{key}") for key in layout["categoryOrder"]]
    for kind, identity, title, url in targets:
        lines += [f"## [{title}]({url})", ""]
        for item in [x for x in queue["items"] if x["kind"] == kind and x["targetId"] == identity]:
            lines += [anchor(item["id"]), "", f"### {item['question']}", "",
                      f"**{item['status'].title()} · Priority {item['priority']}**", "",
                      f"**Deliver:** {item['deliverable']}", "", f"**Accept when:** {item['acceptance']}", "",
                      "**Start with official sources:** " + " · ".join(f"[Source {i}]({source})" for i, source in enumerate(item["sourceUrls"], 1)), ""]
            if item.get("resultUrl"):
                lines += [f"**[Published result]({item['resultUrl']})**", ""]
    lines += ["**[Organization rules](Knowledge-Organization)** · **[Service map](Service-Map)** · **[Contribution workflow](Contribute)**", ""]
    return "\n".join(lines)


def knowledge_health_markdown(report: dict, policy: dict, jobs: list[dict]) -> str:
    counts = report["counts"]
    lines = [page_nav(), "", "# 🧭 Knowledge health", "",
             f"**Published coverage snapshot: {report['asOf']}.** Structural checks passed for this build. "
             "This is an editorial-age report, not live vendor monitoring or proof that a repair worked on client hardware.", "",
             f"**{counts['serviceAreas']} service areas · {counts['jobs']} complete guides · {counts['fieldNotes']} specific fixes · "
             f"{counts['tools']} tools · {counts['researchItems']} research tasks**", "",
             "**[Find your job](Service-Map)** · **[Next research](Research-Queue)** · **[Organization rules](Knowledge-Organization)**", "",
             "## Coverage by job", "", "| Job | Specific fixes | Official resources | Diagnostic branches |", "| --- | --- | --- | --- |"]
    titles = {job["id"]: job["title"] for job in jobs}
    lines += [f"| [{cell(titles[row['jobId']])}]({WIKI}/Job-{row['jobId']}) | {row['fieldNotes']} | {row['resources']} | {row['decisions']} |" for row in report["coverage"]]
    lines += ["", "## Review schedule", "",
              "Review intervals are editorial targets. Passing a link or build check never changes a review date.", "",
              "| Record | Review interval |", "| --- | --- |"]
    labels = {"coreTool": "Everyday tool", "specialistTool": "Specialist tool", "evaluateTool": "Evaluation candidate",
              "guide": "Full guide", "fieldNote": "Specific fix", "resource": "Workspace resource", "acquisition": "Download or acquisition route"}
    lines += [f"| {labels[key]} | {days} days |" for key, days in policy["reviewIntervalsDays"].items()]
    lines += [f"| Scenario tool choice | {policy['reviewIntervalsDays']['coreTool']} days |"]
    lines += ["", f"**{counts.get('scenarioToolChoices', 0)} scenario tool choices** are included in the review schedule. "
              "Their task fit, edition and access boundaries are reviewed alongside the tool itself."]
    lines += ["", f"**Reviews due in this snapshot: {counts['dueReviews']} of {counts['reviewedRecords']} dated records.**", ""]
    if report["dueReviews"]:
        lines += ["| Record | Due | Official source |", "| --- | --- | --- |"]
        lines += [f"| {cell(item['title'])} | {item['dueAt']} | [Review]({item['url']}) |" for item in report["dueReviews"]]
        lines += [""]
    lines += ["The [scheduled organization check](https://github.com/Herbertofury/INTA/actions/workflows/knowledge-health.yml) "
              "recalculates age using the run date and preserves the complete report as an artifact. The weekday contribution task uses that evidence to prioritize work.", "",
              "## Checks that prevent clutter", ""]
    lines += [f"- {rule['rule']}" for rule in policy["rules"]]
    lines += ["", "The build rejects duplicate tool identities/names, unexplained duplicate official product pages, "
              "unknown references, missing service homes, empty fixes, invalid sources and gaps in the research queue. "
              "The normal build and Wiki checks also enforce exact acquisition actions and generated-page consistency.", "",
              "**[Read the complete JSON snapshot](https://github.com/Herbertofury/INTA/blob/main/site/knowledge-health.json)** · "
              "**[Contribute a useful improvement](Contribute)**", ""]
    return "\n".join(lines)


def validate_tool_access(access: dict, tools: list[dict]) -> None:
    """Keep a publisher page distinct from an observed acquisition route."""
    if not isinstance(access, dict) or access.get("schemaVersion") != 1:
        raise ValueError("Tool access requires schemaVersion 1")
    entries = access.get("tools", {})
    if not isinstance(entries, dict) or set(entries) != {tool["id"] for tool in tools}:
        raise ValueError("Tool access must cover every catalog tool exactly")
    kinds = {"file", "store", "builtin", "web", "account", "package", "source", "release-page"}

    def public_url(value, identity):
        parsed = urlparse(value) if isinstance(value, str) else None
        if (parsed is None or parsed.scheme != "https" or not parsed.hostname
                or parsed.username or parsed.password
                or any(key in parsed.query.lower() for key in ("utm_", "fbclid=", "gclid="))):
            raise ValueError(f"{identity}: a public HTTPS URL without tracking is required")

    for identity, entry in entries.items():
        public_url(entry.get("officialUrl"), identity)
        primary = entry.get("primary")
        alternates = entry.get("alternates", [])
        if not isinstance(primary, dict) or not isinstance(alternates, list):
            raise ValueError(f"{identity}: primary acquisition and alternate list are required")
        action_ids = set()
        for action in [primary, *alternates]:
            if not isinstance(action, dict) or action.get("kind") not in kinds:
                raise ValueError(f"{identity}: unknown acquisition kind")
            label = action.get("label")
            if not isinstance(label, str) or not label.strip() or re.search(r"[\n\r<>\[\]|`*]", label):
                raise ValueError(f"{identity}: acquisition labels must be plain, useful text")
            public_url(action.get("url"), identity)
            public_url(action.get("evidenceUrl"), identity)
            try:
                date.fromisoformat(action.get("checkedAt", ""))
            except (TypeError, ValueError):
                raise ValueError(f"{identity}: acquisition review date is required") from None
            if action["kind"] == "file" and not all(action.get(k) for k in ("platform", "format")):
                raise ValueError(f"{identity}: direct files must identify platform and format")
            if action["kind"] != "file" and re.search(r"\bdirect download\b", label, re.I):
                raise ValueError(f"{identity}: a non-file route cannot claim a direct download")
            if "id" in action:
                action_id = action["id"]
                if (not isinstance(action_id, str) or not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", action_id)
                        or action_id in action_ids):
                    raise ValueError(f"{identity}: acquisition action IDs must be distinct and stable")
                action_ids.add(action_id)


def selected_tool_action(entry: dict, action_id: str | None = None) -> dict:
    if action_id is None:
        return entry["primary"]
    for action in [entry["primary"], *entry.get("alternates", [])]:
        if action.get("id") == action_id:
            return action
    raise ValueError(f"Unknown scenario acquisition action: {action_id}")


def validate_job_workspaces(workspaces: dict, jobs: list[dict], tools: list[dict], access: dict) -> None:
    """Every service has a useful, source-backed working kit, not an empty panel."""
    if not isinstance(workspaces, dict) or workspaces.get("schemaVersion") != 1:
        raise ValueError("Job workspaces require schemaVersion 1")
    try:
        date.fromisoformat(workspaces.get("reviewedAt", ""))
    except (TypeError, ValueError):
        raise ValueError("Job workspaces require an editorial review date") from None
    entries = workspaces.get("jobs", {})
    if not isinstance(entries, dict) or set(entries) != {job["id"] for job in jobs}:
        raise ValueError("Job workspaces must cover every playbook exactly")
    tool_ids = {tool["id"] for tool in tools}

    def text_field(value, identity):
        if not isinstance(value, str) or not value.strip():
            raise ValueError(f"{identity}: useful workspace text is required")

    def public_url(value, identity):
        parsed = urlparse(value) if isinstance(value, str) else None
        if (parsed is None or parsed.scheme != "https" or not parsed.hostname
                or parsed.username or parsed.password
                or any(key in parsed.query.lower() for key in ("utm_", "fbclid=", "gclid="))):
            raise ValueError(f"{identity}: public HTTPS resource without tracking is required")

    for identity, entry in entries.items():
        for field in ("goal", "startHere"):
            text_field(entry.get(field), identity)
        for field in ("tools", "resources", "decisions", "doneWhen", "escalateWhen"):
            if not isinstance(entry.get(field), list):
                raise ValueError(f"{identity}: workspace {field} must be a list")
        if not entry["tools"] and not entry["resources"]:
            raise ValueError(f"{identity}: workspace requires a working tool or resource")
        seen_tools = set()
        for selection in entry["tools"]:
            tool_id = selection.get("toolId")
            key = (tool_id, selection.get("actionId"))
            if tool_id not in tool_ids or key in seen_tools:
                raise ValueError(f"{identity}: unknown or duplicate workspace tool {tool_id}")
            seen_tools.add(key)
            text_field(selection.get("why"), identity)
            selected_tool_action(access[tool_id], selection.get("actionId"))
        seen_resources = set()
        for resource in entry["resources"]:
            resource_id = resource.get("id", "")
            if not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", resource_id) or resource_id in seen_resources:
                raise ValueError(f"{identity}: distinct stable resource IDs are required")
            seen_resources.add(resource_id)
            for field in ("label", "kind", "group", "description"):
                text_field(resource.get(field), identity)
            if re.search(r"[\n\r<>\[\]|`*]", resource["label"]):
                raise ValueError(f"{identity}: resource labels must be plain text")
            public_url(resource.get("url"), identity)
            public_url(resource.get("sourceUrl"), identity)
            try:
                date.fromisoformat(resource.get("checkedAt", ""))
            except (TypeError, ValueError):
                raise ValueError(f"{identity}: resource review date is required") from None
        if not entry["decisions"] or not entry["doneWhen"] or not entry["escalateWhen"]:
            raise ValueError(f"{identity}: decision, completion and escalation evidence are required")
        for decision in entry["decisions"]:
            for field in ("symptom", "action", "expected"):
                text_field(decision.get(field), identity)
        for value in [*entry["doneWhen"], *entry["escalateWhen"]]:
            text_field(value, identity)


def workspace_resources_markdown(workspace: dict, tools_by_id: dict, access: dict,
                                 working_url: str) -> list[str]:
    """Share the same practical toolkit between the Wiki and the sticky app view."""
    lines = [anchor("working-kit"), "", "## 🧰 Tools and links for this job", "",
             f"**[Open working view]({working_url})** · Keep the tool panel beside the instructions.", "",
             f"**First move:** {workspace['startHere']}", ""]
    driver_resources = [resource for resource in workspace["resources"] if resource["kind"] == "driver"]
    if driver_resources:
        lines += ["**Driver shortcuts:** " + " · ".join(
            f"**[{cell(resource['label'])}]({resource['url']})**" for resource in driver_resources), ""]
    if workspace["tools"]:
        lines += ["| Use this for | Official tool + exact action |", "| --- | --- |"]
        for selection in workspace["tools"]:
            tool = tools_by_id[selection["toolId"]]
            lines += [f"| {cell(selection['why'])} | **[{tool['name']}]({access[tool['id']]['officialUrl']})** · "
                      f"{tool_access_links(tool, access, action_id=selection.get('actionId'))} |"]
        lines += [""]
    groups = {}
    for resource in workspace["resources"]:
        groups.setdefault(resource["group"], []).append(resource)
    for group, resources in groups.items():
        lines += [f"### {group}", "", "| Open the exact resource | What to do there |", "| --- | --- |"]
        lines += [f"| **[{cell(resource['label'])}]({resource['url']})** | {cell(resource['description'])} |"
                  for resource in resources]
        lines += [""]
    return lines


def workspace_decisions_markdown(workspace: dict) -> list[str]:
    lines = [anchor("next-action"), "", "## 🧭 Match the symptom", "",
             "| You see this | Do this next | You should get |", "| --- | --- | --- |"]
    lines += [f"| **{cell(item['symptom'])}** | {cell(item['action'])} | {cell(item['expected'])} |"
              for item in workspace["decisions"]]
    lines += ["", "### ✅ Finish the job", ""]
    lines += [f"- [ ] {item}" for item in workspace["doneWhen"]]
    lines += ["", "**Change route when:** " + " · ".join(workspace["escalateWhen"]), ""]
    return lines


def tool_access_links(tool: dict, access: dict, *, all_routes: bool = False,
                      action_id: str | None = None) -> str:
    entry = access[tool["id"]]
    primary = selected_tool_action(entry, action_id)
    links = [f"[Official page]({entry['officialUrl']})", f"**[{cell(primary['label'])}]({primary['url']})**"]
    seen = {entry["officialUrl"], primary["url"]}
    if all_routes:
        for action in [entry["primary"], *entry.get("alternates", [])]:
            if action["url"] not in seen:
                links += [f"[{cell(action['label'])}]({action['url']})"]
                seen.add(action["url"])
        if tool["downloadUrl"] not in seen:
            links += [f"[All downloads / setup]({tool['downloadUrl']})"]
    return " · ".join(links)


def validate_quick_start(quick_start: dict, tools: list[dict], jobs: list[dict],
                         access: dict | None = None) -> None:
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
    seen_ids, seen_pages = set(), set()
    reserved_pages = {"Home", "Playbooks", "Tool-Directory", "_Sidebar", "_Footer", *WIKI_MIRRORS}
    reserved_pages.update(f"Job-{identity}" for identity in job_ids)
    reserved_pages.update(f"Tools-{slug(tool['category'])}" for tool in tools)
    reserved_pages = {name.casefold() for name in reserved_pages}
    required = ("id", "jobId", "label", "wikiPage", "firstStep", "why", "boundary", "freeUse", "qualityBasis")
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
        page = item["wikiPage"]
        if not re.fullmatch(r"[A-Za-z0-9]+(?:-[A-Za-z0-9]+)*", page) or page.casefold() in seen_pages:
            raise ValueError(f"{identity}: invalid or duplicate quick-answer Wiki page: {page}")
        if page.casefold() in reserved_pages:
            raise ValueError(f"{identity}: quick-answer slug collides with a reserved Wiki page: {page}")
        seen_pages.add(page.casefold())
        if item["jobId"] not in job_ids or (item.get("toolId") not in tool_ids and not
                (item.get("guideOnly") is True and item.get("toolId") is None)):
            raise ValueError(f"{identity}: quick start references an unknown tool or playbook")
        steps = item.get("quickSteps")
        if not isinstance(steps, list) or len(steps) != 3 or any(not isinstance(s, str) or not s.strip() for s in steps):
            raise ValueError(f"{identity}: exactly three concrete first actions are required")
        if item.get("toolAction") is not None:
            if item.get("guideOnly") or not isinstance(item["toolAction"], str):
                raise ValueError(f"{identity}: scenario actions require a tool and a stable action ID")
            if access is not None:
                selected_tool_action(access[item["toolId"]], item["toolAction"])
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
        choices = item.get("choices")
        if not isinstance(choices, list):
            raise ValueError(f"{identity}: tool choices must be a list")
        seen_choices = set()
        for choice in choices:
            if not isinstance(choice, dict) or choice.get("toolId") not in tool_ids:
                raise ValueError(f"{identity}: a tool choice must reference a known tool")
            for field in ("role", "why", "freeUse"):
                if not isinstance(choice.get(field), str) or not choice[field].strip():
                    raise ValueError(f"{identity}: tool choice requires {field}")
            if re.search(r"[\n\r<>\[\]|`*]", choice["role"]):
                raise ValueError(f"{identity}: tool choice roles must be plain single-line text")
            reviewed = choice.get("reviewedAt", "")
            try:
                if date.fromisoformat(reviewed) > date.today():
                    raise ValueError("future review")
            except (TypeError, ValueError):
                raise ValueError(f"{identity}: tool choice requires a real review date") from None
            action_id = choice.get("actionId")
            if action_id is not None and (not isinstance(action_id, str) or not action_id):
                raise ValueError(f"{identity}: tool choice action must be a stable action ID")
            action = selected_tool_action(access[choice["toolId"]], action_id) if access else None
            key = (choice["toolId"], action["url"] if action else action_id)
            if key in seen_choices:
                raise ValueError(f"{identity}: duplicate tool choice")
            seen_choices.add(key)
            evidence = choice.get("evidenceUrls")
            if not isinstance(evidence, list) or not evidence:
                raise ValueError(f"{identity}: tool choice requires source evidence")
            for url in evidence:
                parsed = urlparse(url) if isinstance(url, str) else None
                if (parsed is None or parsed.scheme != "https" or not parsed.hostname
                        or parsed.username or parsed.password
                        or any(key in parsed.query.lower() for key in ("utm_", "fbclid=", "gclid="))):
                    raise ValueError(f"{identity}: tool choice requires public HTTPS evidence without tracking")
    if {item["jobId"] for item in items} != job_ids:
        raise ValueError("Quick start must cover every service scenario")


def resolved_tool_choices(item: dict, access: dict) -> list[dict]:
    """Preserve authored role order and the original scenario-specific acquisition."""
    choices = [dict(choice) for choice in item.get("choices", [])]
    if item.get("toolId"):
        action = selected_tool_action(access[item["toolId"]], item.get("toolAction"))
        if not any(choice["toolId"] == item["toolId"] and
                   selected_tool_action(access[choice["toolId"]], choice.get("actionId"))["url"] == action["url"]
                   for choice in choices):
            primary = {"toolId": item["toolId"], "role": "Starting tool", "why": item["why"],
                       "freeUse": item["freeUse"], "evidenceUrls": item["evidenceUrls"]}
            if item.get("toolAction"):
                primary["actionId"] = item["toolAction"]
            choices.append(primary)
    return choices


def choice_links_markdown(choice: dict, tools_by_id: dict, access: dict) -> str:
    tool = tools_by_id[choice["toolId"]]
    entry = access[tool["id"]]
    action = selected_tool_action(entry, choice.get("actionId"))
    return (f"**[{tool['name']}]({entry['officialUrl']})** · "
            f"**[{cell(action['label'])}]({action['url']})**")


def tool_choices_markdown(item: dict, tools_by_id: dict, access: dict) -> list[str]:
    choices = resolved_tool_choices(item, access)
    if not choices:
        return []
    lines = [anchor("tool-choices"), "", "## 🧰 Choose the tool for this job", "",
             "Choose by the task shown in the first column. Each option includes its acquisition route and access boundary.", "",
             "| Need | Official tool + exact action | Why this choice / access |", "| --- | --- | --- |"]
    for choice in choices:
        lines += [f"| **{cell(choice['role'])}** | {choice_links_markdown(choice, tools_by_id, access)} | "
                  f"{cell(choice['why'])} **Access:** {cell(choice['freeUse'])} |"]
    lines += ["", f"**[Current selection evidence]({WIKI}/Current-Tool-Choices)**", ""]
    return lines


def quick_start_markdown(quick_start: dict, tools_by_id: dict, layout: dict, access: dict) -> str:
    lines = [anchor("free-tools"), "", "## ⚡ What do you need to fix?", "",
             "**Choose your problem for the first three actions.** Tool names open the official page; "
             "download links identify the file and platform. Each role has its own visible choice. Built-in features link to their instructions.", "",
             "| Fix this | Choose the right tool · official page + exact action |", "| --- | --- |"]
    for item in quick_start["items"]:
        job_id = item["jobId"]
        icon = item.get("icon") or layout["jobs"][job_id]["navigationIcon"]
        guide = f"{WIKI}/{item['wikiPage']}"
        actions = [f"No extra download needed · **[Open checklist]({guide})**"] if item.get("guideOnly") else []
        for choice in resolved_tool_choices(item, access):
            actions.append(f"**{cell(choice['role'])}:** {choice_links_markdown(choice, tools_by_id, access)} "
                           f"({cell(choice['freeUse'])})")
        action = "<br>".join(actions)
        lines += [f"| **[{icon} {cell(item['label'])}]({guide})** | {action} |"]
    lines += ["", f"Selection review: **{quick_start['reviewedAt']}** · "
              f"[Complete tool directory]({WIKI}/Tool-Directory) · [Why these tools]({WIKI}/Free-Tool-Picks) · "
              f"[Current tool choices]({WIKI}/Current-Tool-Choices)", ""]
    return "\n".join(lines)


def quick_answer_markdown(item: dict, tools_by_id: dict, access: dict, reviewed: str,
                          workspace: dict | None = None) -> str:
    """Give a scenario its own reliable Wiki destination, independent of fragments."""
    lines = [page_nav(), "", f"# {item.get('icon', '🛠️')} {item['label']}", "",
             f"**{item['firstStep']}**", ""]
    lines += tool_choices_markdown(item, tools_by_id, access)
    if workspace:
        selected_workspace = dict(workspace, startHere=item["firstStep"])
        if not item.get("guideOnly"):
            selected = {"toolId": item["toolId"], "why": item["label"]}
            if item.get("toolAction"):
                selected["actionId"] = item["toolAction"]
            selected_workspace["tools"] = [selected, *[
                selection for selection in workspace["tools"]
                if (selection["toolId"], selection.get("actionId")) !=
                   (selected["toolId"], selected.get("actionId"))]]
        lines += workspace_resources_markdown(selected_workspace, tools_by_id, access,
                    f"{HUB}#job/{item['jobId']}/quick/{item['id']}")
        lines += ["## ⚡ First three actions", ""]
    lines += [f"{number}. {step}" for number, step in enumerate(item["quickSteps"], 1)]
    lines += [""]
    if item.get("guideOnly"):
        lines += ["**No extra download needed.**", ""]
    else:
        tool = tools_by_id[item["toolId"]]
        entry = access[tool["id"]]
        action = selected_tool_action(entry, item.get("toolAction"))
        lines += [f"**[{tool['name']}]({entry['officialUrl']})** · "
                  f"**[{cell(action['label'])}]({action['url']})**", ""]
    lines += [f"**[Complete procedure]({WIKI}/Job-{item['jobId']})** · "
              f"**[Specific fixes for this job]({HUB}#job/{item['jobId']}/quick/{item['id']}?section=fixes)** · "
              f"**[Selection evidence]({WIKI}/Free-Tool-Picks)**"]
    if workspace:
        lines += [""] + workspace_decisions_markdown(workspace)
    lines += page_footer(reviewed)
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
            f"[Service map]({WIKI}/Service-Map) · [Playbooks]({WIKI}/Playbooks) · [Tools]({WIKI}/Tool-Directory) · "
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


def scenario_link(identity: str, layout: dict, absolute: bool = False,
                  wiki_page: str | None = None) -> str:
    config = layout["jobs"][identity]
    destination = wiki_page or f"Job-{identity}"
    if absolute:
        destination = f"{WIKI}/{destination}"
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
                 jobs_by_id: dict | None = None, reviewed: str = "2026-11-26",
                 quick_items: list[dict] | None = None, access: dict | None = None,
                 workspace: dict | None = None, field_notes: list[dict] | None = None) -> str:
    layout = layout or json.loads((ROOT / "data/wiki-layout.json").read_text(encoding="utf-8"))
    config = layout["jobs"][job["id"]]
    jobs_by_id = jobs_by_id or {j["id"]: j for j in records(ROOT / "data/jobs.json", "jobs")}
    if access is None:
        access = json.loads((ROOT / "data/tool-access.json").read_text(encoding="utf-8"))["tools"]
    if quick_items is None:
        quick_items = [item for item in json.loads((ROOT / "data/quick-start.json").read_text(encoding="utf-8"))["items"]
                       if item["jobId"] == job["id"]]
    destination = lambda identity: f"{WIKI}/Job-{identity}" if wiki else f"{identity}.md"
    lines = [page_nav(), "", *heading(1, job["title"], f"{config['icon']} {job['title']}"), job["summary"], "",
             f"**{cell(job['category'])}** · {cell(job['difficulty'])} · {cell(job['estimatedTime'])} · {len(job['steps'])} steps", "",
              "[Tools and useful links](#working-kit) · [Quick answer](#quick-answer) · [Next action](#next-action) · [Specific fixes](#specific-fixes) · [Decision route](#decision-route) · [Tools](#toolkit) · [Procedure](#walkthrough) · "
             f"[Verification](#acceptance) · [Sources](#sources) · [Interactive checklist]({HUB}#job/{job['id']})", ""]
    if quick_items:
        lines += tool_choices_markdown(quick_items[0], by_id, access)
    if job["id"] == "client-handoff":
        lines += [f"**[Open the dedicated nine-step Checklist tab]({HUB}#checklist)**", ""]
    if workspace:
        lines += workspace_resources_markdown(workspace, by_id, access, f"{HUB}#job/{job['id']}")
    if quick_items:
        lines += [anchor("quick-answer"), "", "## ⚡ Quick answer", ""]
        for item in quick_items:
            lines += [anchor(f"quick-{item['id']}"), "", f"### {item.get('icon', '🛠️')} {item['label']}", "",
                      f"**{item['firstStep']}**", ""]
            lines += [f"{n}. {step}" for n, step in enumerate(item["quickSteps"], 1)]
            lines += [""]
            if item.get("guideOnly"):
                lines += ["**No extra download needed.** Follow the checklist below.", ""]
            else:
                tool = by_id[item["toolId"]]
                lines += [f"**[{tool['name']}]({access[tool['id']]['officialUrl']})** · "
                          + tool_access_links(tool, access, action_id=item.get("toolAction")), ""]
    if workspace:
        lines += workspace_decisions_markdown(workspace)
    lines += field_note_index(field_notes or [])
    lines += ["> [!NOTE]", f"> **Start here.** {job['startHere']}", "",
             anchor("decision-route"), "", *heading(2, "🧭 Choose a route", "🧭 Choose the right branch")]
    lines += diagram_block(config["diagram"])
    routes = ["[Continue this workflow](#walkthrough)", "[Acceptance checks](#acceptance)"]
    routes += [f"[{jobs_by_id[identity]['title']}]({destination(identity)})" for identity in job.get("related", [])]
    lines += ["**Route links:** " + " · ".join(routes), ""]
    lines += ["| Checkpoint | What to carry forward |", "| --- | --- |"]
    lines += [f"| **{cell(label)}** | {cell(detail)} |" for label, detail in config["checkpoints"].items()]
    lines += ["", anchor("toolkit"), "", *heading(2, "Recommended tools", "🧰 Build the right toolkit")]
    recommended_ids = list(dict.fromkeys([*job["tools"], *[item["toolId"] for item in (workspace or {}).get("tools", [])]]))
    if recommended_ids:
        lines += ["Select a tool for the next diagnostic or repair step. Check its platform, access and licensing requirements before use.", "",
                  "| Tool / official page + download | Use it for | Platform & access | Documentation |", "| --- | --- | --- | --- |"]
        for tool_id in recommended_ids:
            tool = by_id[tool_id]
            selection = next((item for item in (workspace or {}).get("tools", []) if item["toolId"] == tool_id), {})
            lines += [f"| **[{tool['name']}]({access[tool_id]['officialUrl']})** · {tool_access_links(tool, access, action_id=selection.get('actionId'))} | {cell(selection.get('why', tool['bestFor']))} | "
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
        for shortcut in step.get("shortcuts", []):
            if "tool" in shortcut:
                lines += [f"- [{shortcut['label']}]({access[shortcut['tool']]['officialUrl']}) · "
                          + tool_access_links(by_id[shortcut["tool"]], access)]
            else:
                target = destination(shortcut["job"]) if "job" in shortcut else shortcut["url"]
                lines += [f"- [{shortcut['label']}]({target})"]
        if step.get("shortcuts"):
            lines += [""]
        if step.get("command"):
            lines += [f"**{cell(step.get('commandShell', 'Command'))} · {cell(step.get('commandPurpose', 'Follow the step above'))}:**", "", "```text", step["command"], "```", ""]
            if step.get("commandSafety"):
                lines += [f"> **Before running:** {cell(step['commandSafety'])}", ""]
            if step.get("commandUrl"):
                lines += [f"**Official command reference:** [{cell(step.get('commandName', 'Documentation'))}]({step['commandUrl']})", ""]
            lines += [f"**Command legend:** {command_reference(step, by_id)} · [Return to the step context](#{identity}).", ""]
    lines += [anchor("acceptance"), "", *heading(2, "Verification", "✅ Prove the result")]
    lines += [f"- [ ] {item}" for item in job["verify"]]
    lines += ["", "> [!WARNING]", "> **Pause this route and escalate when:**"]
    lines += [f"> - {item}" for item in job["stopIf"]]
    if field_notes:
        lines += [""] + field_notes_markdown(field_notes, by_id, access, workspace or {})
    lines += ["", *heading(2, "Related playbooks", "🔀 Continue with the evidence")]
    if job.get("related"):
        lines += ["| Next playbook | What it resolves |", "| --- | --- |"]
        for identity in job["related"]:
            other = jobs_by_id[identity]
            lines += [f"| [{other['title']}]({destination(identity)}) | {cell(other['summary'])} |"]
    lines += ["", anchor("sources"), "", *heading(2, "Official sources", "🔎 Source trail")]
    lines += [f"- [{source['title']}]({source['url']})" for source in job["sources"]]
    lines += ["", f"Reviewed **{job.get('reviewedAt', reviewed)}**. Timing is a planning range. "
              "A successful command is evidence about that operation; acceptance belongs to the user’s original task.", ""]
    lines += page_footer(job.get("reviewedAt", reviewed), wiki)
    return "\n".join(lines)


def tier_label(value: str) -> str:
    return {"core": "Core", "specialist": "Specialist", "evaluate": "Evaluate"}[value]


def category_markdown(category: str, tools: list[dict], layout: dict, jobs: dict, reviewed: str, access: dict) -> str:
    info = layout["categories"].get(category, {"icon": "🧰", "intro": "Choose a tool for the observed job.", "jobs": [], "question": "What result do you need?"})
    label = CATEGORY_NAMES.get(category, category)
    ordered = sorted(tools, key=lambda t: ({"core": 0, "specialist": 1, "evaluate": 2}[t["tier"]], t["name"].casefold()))
    lines = [page_nav(), "", *heading(1, label, f"{info['icon']} {label}"), info["intro"], "",
             f"**{len(tools)} tools** · [All categories]({WIKI}/Tool-Directory) · [Selection standards]({WIKI}/Knowledge-Standards)", "",
             f"**Selection question:** {info['question']}", "", *heading(2, "Related playbooks", "🧭 Start with a workflow")]
    lines += [f"- {scenario_link(identity, layout, absolute=True)} · **[Open working view]({HUB}#job/{identity})**"
              for identity in info["jobs"]]
    lines += ["", anchor("compare-options"), "", *heading(2, "Tool comparison", "⚖️ Compare the options"),
              "| Tool | Choose it for | Platforms / role | Official page + download |", "| --- | --- | --- | --- |"]
    for tool in ordered:
        lines += [f"| [{tool['name']}](#{tool['id']}) | {cell(tool['bestFor'])} | {cell(tool['platforms'])} · {tier_label(tool['tier'])} | {tool_access_links(tool, access)} |"]
    lines += ["", "Core means an everyday candidate; Specialist means a focused advanced use; Evaluate means prove it in a suitable test environment before promotion.", ""]
    for tool in ordered:
        lines += [anchor(tool["id"]), "", f"## {tool['name']}", "", tool["summary"], "",
                  tool_access_links(tool, access, all_routes=True) + f" · [Documentation]({tool['docsUrl']})", "",
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
            navigation = layout["jobs"][identity]
            lines += [f"| **[{navigation['navigationIcon']} {navigation['navigationLabel']}]({destination(identity)})** · "
                      f"[Working view]({HUB}#job/{identity}) | {cell(job['summary'])} | {job['difficulty']} | {cell(job['estimatedTime'])} |"]
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
             f"**[🧰 Open technician hub]({HUB})** · **[🗺️ Service map](Service-Map)** · [⚡ Command Center](Command-Center) · [⚡ Quick commands](Quick-Commands) · [Decision desk](Decision-Desk) · [Playbooks](Playbooks) · [Tool directory](Tool-Directory) · [Methods](Methods)", "",
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


def validate_reputation(snapshot: dict, tools: list[dict]) -> None:
    def day(value):
        try:
            return isinstance(value, str) and date.fromisoformat(value).isoformat() == value
        except ValueError:
            return False

    def count(value):
        return type(value) is int and 0 <= value <= 2**53 - 1

    def source(value, host=None):
        value = urlparse(value) if isinstance(value, str) else None
        return (value and value.scheme == "https" and value.hostname and
                not value.username and not value.password and
                (not host or value.hostname == host))

    states = {"observed", "unverified", "unreported", "not-applicable", "unavailable"}
    entries = snapshot.get("tools")
    if (snapshot.get("schemaVersion") != 1 or not day(snapshot.get("reviewedAt")) or
            not isinstance(entries, dict) or set(entries) != {tool["id"] for tool in tools}):
        raise ValueError("Reputation requires a dated evidence or explicit gap record for every catalog tool")

    for tool_id, record in entries.items():
        if not isinstance(record, dict) or set(record) != {"github", "sourceforge", "usage"}:
            raise ValueError(f"{tool_id}: invalid reputation evidence streams")
        for kind, evidence in record.items():
            if not isinstance(evidence, dict) or evidence.get("status") not in states:
                raise ValueError(f"{tool_id}: invalid {kind} observation status")
            status = evidence["status"]
            host = {"github": "github.com", "sourceforge": "sourceforge.net"}.get(kind)
            observed = status == "observed"
            metrics = {"github": {"stars", "forks"}, "sourceforge": {"rating", "weeklyDownloads"},
                       "usage": {"weeklyActiveUsers", "approximate", "reportedAt"}}[kind]
            if (not observed and metrics.intersection(evidence)) or (
                status in {"observed", "unavailable", "unreported"} and (
                    not source(evidence.get("url"), host) or not day(evidence.get("checkedAt"))
                    or evidence["checkedAt"] > snapshot["reviewedAt"]
                    or (kind != "usage" and not source(evidence.get("identitySource")))
                )
            ):
                raise ValueError(f"{tool_id}: invalid dated {kind} evidence or unknown metric set to a value")
            if not observed:
                continue
            if kind == "github":
                if (not re.fullmatch(r"/[\w.-]+/[\w.-]+/?", urlparse(evidence["url"]).path)
                        or not all(count(evidence.get(k)) for k in metrics)):
                    raise ValueError(f"{tool_id}: invalid GitHub repository metrics")
            elif kind == "sourceforge":
                if (not re.fullmatch(r"/projects/[\w.-]+/?", urlparse(evidence["url"]).path)
                        or not set(evidence).intersection(metrics)
                        or ("weeklyDownloads" in evidence and not count(evidence["weeklyDownloads"]))):
                    raise ValueError(f"{tool_id}: invalid SourceForge popularity evidence")
                if "rating" in evidence:
                    rating = evidence["rating"]
                    if (not isinstance(rating, dict) or set(rating) != {"value", "scale", "reviews"}
                            or type(rating["value"]) not in (int, float) or type(rating["scale"]) not in (int, float)
                            or rating["scale"] != 5 or not 1 <= rating["value"] <= 5
                            or not count(rating["reviews"]) or not rating["reviews"]):
                        raise ValueError(f"{tool_id}: invalid native rating or sample size")
            elif (not count(evidence.get("weeklyActiveUsers")) or type(evidence.get("approximate")) is not bool
                  or not day(evidence.get("reportedAt")) or evidence["reportedAt"] > evidence["checkedAt"]):
                raise ValueError(f"{tool_id}: invalid weekly active user evidence")


def validate_news(catalog: dict) -> None:
    def is_date(value):
        try:
            return isinstance(value, str) and date.fromisoformat(value).isoformat() == value
        except ValueError:
            return False
    def safe_url(value):
        if not isinstance(value, str) or any(c.isspace() for c in value):
            return False
        try:
            u = urlparse(value)
            return u.scheme == "https" and bool(u.hostname) and not u.username and not u.password
        except ValueError:
            return False
    categories = {"advisory", "compliance", "vendor", "reporting", "community", "status"}
    ids, links = set(), set()
    if catalog.get("schemaVersion") != 1 or not is_date(catalog.get("reviewedAt")) or not isinstance(catalog.get("sources"), list) or not catalog["sources"]:
        raise ValueError("News requires a versioned and dated source catalog")
    for source in catalog["sources"]:
        if (not isinstance(source, dict) or not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", str(source.get("id", "")))
                or source["id"] in ids or source.get("url") in links or not safe_url(source.get("url"))
                or source.get("category") not in categories
                or not all(isinstance(source.get(k), str) and bool(source[k].strip()) for k in ("name","description"))
                or ("feed" in source and not safe_url(source.get("feed")))):
            raise ValueError("Invalid or duplicate News source")
        ids.add(source["id"])
        links.add(source["url"])
    for item in catalog.get("briefing", []):
        if (item.get("sourceId") not in ids or not safe_url(item.get("url")) or not is_date(item.get("date"))
                or item["date"] > catalog["reviewedAt"] or not item.get("title") or not item.get("description")):
            raise ValueError("Invalid News briefing item")


def validate_updates_center(catalog: dict) -> None:
    """Validate update channels, copy-ready commands and recovery workflows."""
    if catalog.get("schemaVersion") != 1 or not isinstance(catalog.get("commands"), list) or not catalog["commands"]:
        raise ValueError("Updates Center requires a versioned command catalog")
    categories = {"Windows Update", "Microsoft Store", "Installer recovery", "Application inventory", "WinGet", "Chocolatey", "Scoop", "UniGetUI", "Ninite Pro", "Update inventory", "Linux", "macOS Software Update"}
    commands = {}
    for item in catalog["commands"]:
        identity = item.get("id", "?")
        if identity in commands or not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", identity):
            raise ValueError(f"{identity}: invalid or duplicate update command ID")
        commands[identity] = item
        for field in ("title", "category", "shell", "privilege", "risk", "summary", "code", "safety", "sourceUrl", "sourceTitle"):
            if not isinstance(item.get(field), str) or not item[field].strip():
                raise ValueError(f"{identity}: update command requires {field}")
        if item["category"] not in categories or item["shell"] not in {"PowerShell", "CMD", "Bash"}:
            raise ValueError(f"{identity}: unsupported update category or shell")
        if item["risk"] not in {"read-only", "long-running", "downloads-file", "installs-software", "changes-system"}:
            raise ValueError(f"{identity}: unsupported update risk")
        source = urlparse(item["sourceUrl"])
        if source.scheme != "https" or not source.hostname or source.username or source.password:
            raise ValueError(f"{identity}: source must be public HTTPS")
        if len(item["code"]) > 16000:
            raise ValueError(f"{identity}: command exceeds size limit")
        if re.search(r"(?i)(?:irm|iwr|curl)\b[^\n|]*\|\s*(?:iex|invoke-expression)", item["code"]):
            raise ValueError(f"{identity}: remote download-and-execute pipeline prohibited")
    channel_ids = set()
    for channel in catalog.get("channels", []):
        identity = channel.get("id", "?")
        if identity in channel_ids or not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", identity):
            raise ValueError(f"{identity}: invalid or duplicate update channel")
        channel_ids.add(identity)
        for field in ("name", "scope", "manager", "auditCommand", "updateCommand", "note"):
            if not channel.get(field):
                raise ValueError(f"{identity}: update channel requires {field}")
        if channel["auditCommand"] not in commands or channel["updateCommand"] not in commands:
            raise ValueError(f"{identity}: update channel references an unknown command")
    workflow_ids = set()
    for workflow in catalog.get("workflows", []):
        identity = workflow.get("id", "?")
        if identity in workflow_ids or not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", identity):
            raise ValueError(f"{identity}: invalid or duplicate update workflow")
        workflow_ids.add(identity)
        if not workflow.get("commandIds") or any(command_id not in commands for command_id in workflow["commandIds"]):
            raise ValueError(f"{identity}: workflow references an unknown command")


    platform_ids = set()
    for platform in catalog.get("platforms", []):
        identity = platform.get("id", "?")
        if identity in platform_ids or not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", identity):
            raise ValueError(f"{identity}: invalid or duplicate fleet patch platform")
        platform_ids.add(identity)
        for field in ("name", "kind", "platform", "summary", "url", "docsUrl", "trust"):
            if not isinstance(platform.get(field), str) or not platform[field].strip():
                raise ValueError(f"{identity}: fleet patch platform requires {field}")
        for field in ("url", "docsUrl"):
            parsed = urlparse(platform[field])
            if parsed.scheme != "https" or not parsed.hostname or parsed.username or parsed.password:
                raise ValueError(f"{identity}: platform {field} must be public HTTPS")


def validate_command_center(catalog: dict) -> None:
    """Validate the complete authored command library and its relationships."""
    def text_fields(item, fields, label):
        if not isinstance(item, dict):
            raise ValueError(f"{label}: expected an object")
        for field in fields:
            if not isinstance(item.get(field), str) or not item[field].strip():
                raise ValueError(f"{label}: missing {field}")

    def records_by_id(items, kind, fields):
        if not isinstance(items, list) or not items:
            raise ValueError(f"Command Center requires a nonempty {kind} list")
        found = {}
        for item in items:
            text_fields(item, ("id", *fields), kind)
            identity = item["id"]
            if not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", identity) or identity in found:
                raise ValueError(f"Invalid or duplicate Command Center {kind} ID: {identity}")
            found[identity] = item
        return found

    def source_url(value, label):
        if not isinstance(value, str) or any(character.isspace() for character in value):
            raise ValueError(f"{label}: reference must be public HTTPS")
        try:
            parsed = urlparse(value)
            valid = (parsed.scheme == "https" and parsed.hostname
                     and not parsed.username and not parsed.password)
        except ValueError:
            valid = False
        if not valid or any(key in parsed.query.lower() for key in ("utm_", "fbclid=", "gclid=")):
            raise ValueError(f"{label}: reference must be public HTTPS without tracking parameters")

    text_fields(catalog, ("title", "description", "reviewedAt"), "Command Center")
    if type(catalog.get("schemaVersion")) is not int or catalog["schemaVersion"] != 1:
        raise ValueError("Command Center requires schemaVersion 1")
    try:
        valid_date = date.fromisoformat(catalog["reviewedAt"]).isoformat() == catalog["reviewedAt"]
    except ValueError:
        valid_date = False
    if not valid_date:
        raise ValueError("Command Center requires a valid review date")
    shells = catalog.get("shells")
    if (not isinstance(shells, list) or not shells
            or any(not isinstance(shell, str) or shell not in {"PowerShell", "CMD", "Bash"} for shell in shells)
            or len(set(shells)) != len(shells)):
        raise ValueError("Command Center requires unique supported shell labels")
    policies = catalog.get("policies")
    required_policies = {
        "noAutoExecution", "requireExplicitUserCopyOrRun", "noRemoteScriptPipeToInterpreter",
        "requirePreviewBeforeBulkUpdates", "neverBypassPackagePinsByDefault",
        "requireOfficialSourceAndSafetyNotes", "doNotMixPackageManagersBlindly",
        "separateOsUpdatesFromApplicationUpdates", "requireRollbackOrRecoveryPlanForDestructiveActions",
    }
    if not isinstance(policies, dict) or any(policies.get(key) is not True for key in required_policies):
        raise ValueError("Command Center safety policies must remain enabled")
    categories = records_by_id(catalog.get("categories"), "categories", ("label", "description"))
    if "all" in categories:
        raise ValueError("Command Center category 'all' is reserved for the complete library")
    commands = records_by_id(catalog.get("commands"), "commands",
                             ("title", "category", "shell", "privilege", "risk", "summary",
                              "code", "safety", "sourceUrl", "sourceTitle"))
    risks = {"read-only", "long-running", "downloads-file", "installs-software", "changes-system"}
    for identity, command in commands.items():
        if command["category"] not in categories or command["shell"] not in shells or command["risk"] not in risks:
            raise ValueError(f"{identity}: unknown command category, shell or risk")
        source_url(command["sourceUrl"], identity)
        if len(command["code"]) > 16000:
            raise ValueError(f"{identity}: command block exceeds size limit")
        code = command["code"].replace(chr(96) + "\n", " ").replace("\\\n", " ")
        fetch = r"(?:irm|iwr|invoke-restmethod|invoke-webrequest|curl(?:\.exe)?|wget(?:\.exe)?)"
        interpreter = r"(?:iex|invoke-expression|bash|sh|zsh|pwsh(?:\.exe)?|powershell(?:\.exe)?)"
        if (re.search(rf"(?i)\b{fetch}\b[^\n;]*\|\s*(?:sudo\s+)?{interpreter}\b", code)
                or re.search(rf"(?i)\b(?:iex|invoke-expression)\s*\(\s*{fetch}\b", code)):
            raise ValueError(f"{identity}: remote download-and-execute commands are prohibited")
    workflows = records_by_id(catalog.get("workflows"), "workflows",
                              ("title", "category", "description", "sequence", "warning"))
    for identity, workflow in workflows.items():
        ids = workflow.get("commandIds")
        if (not isinstance(ids, list) or not ids
                or any(not isinstance(command_id, str) or command_id not in commands for command_id in ids)
                or len(ids) != len(set(ids))):
            raise ValueError(f"{identity}: unknown or missing command, or duplicate workflow command")
        if workflow["category"] not in categories:
            raise ValueError(f"{identity}: unknown workflow category")
    toolkits = records_by_id(catalog.get("toolkits"), "toolkits",
                             ("name", "kind", "platform", "summary", "url", "trust"))
    for identity, toolkit in toolkits.items():
        source_url(toolkit["url"], identity)
        if "repository" in toolkit:
            source_url(toolkit["repository"], identity)
        install = toolkit.get("installCommandId")
        if install is not None and (not isinstance(install, str) or install not in commands):
            raise ValueError(f"{identity}: unknown or missing command for toolkit setup")


def merge_web_brands(base: dict, extension: dict) -> dict:
    """Extend web/document identities without mutating the native icon registry."""
    if (not isinstance(extension, dict) or type(extension.get("schemaVersion")) is not int
            or extension["schemaVersion"] != 1 or not isinstance(extension.get("assets"), dict)
            or not isinstance(extension.get("brands"), list)):
        raise ValueError("Web brand extension requires schemaVersion 1, assets and brands")
    assets = dict(base["assets"])
    for identity, asset in extension["assets"].items():
        if identity in assets:
            if assets[identity] != asset:
                raise ValueError(f"Conflicting web brand asset: {identity}")
            continue
        if not isinstance(asset, dict):
            raise ValueError(f"Invalid web brand asset: {identity}")
        if not isinstance(asset.get("path"), str):
            raise ValueError(f"Invalid web brand asset path: {identity}")
        path = Path(asset["path"])
        if path.parts[:2] != ("assets", "web-brand-icons") or ".." in path.parts or path.suffix != ".png":
            raise ValueError(f"Web brand asset must stay in assets/web-brand-icons: {identity}")
        assets[identity] = asset
    seen = {brand["id"] for brand in base["brands"]}
    for brand in extension["brands"]:
        if not isinstance(brand, dict) or not isinstance(brand.get("id"), str) or not brand["id"] or brand["id"] in seen:
            raise ValueError("Invalid or duplicate web brand identity")
        seen.add(brand["id"])
    # Web refinements win resolver precedence; the complete native registry stays
    # present, including its original paths and immutable Media Creator inputs.
    return {**base, "assets": assets, "brands": [*extension["brands"], *base["brands"]]}


def extend_web_brand_links(registry: list[dict], brands: list[dict]) -> list[dict]:
    """Link new web aliases while preserving every existing deliberate route."""
    entries = {entry["name"].casefold(): entry for entry in registry}
    ordinary = {"home", "support", "download", "downloads", "documentation", "source", "help", "free", "tools", "media"}
    for brand in brands:
        for name in [brand.get("name", ""), *brand.get("names", [])]:
            if not name or len(name) < 2 or name.casefold() in entries or name.casefold() in ordinary:
                continue
            entries[name.casefold()] = {"name": name, "url": brand["downloadPage"],
                                       "kind": brand.get("kind", "application"), "id": brand["id"],
                                       "caseSensitive": True}
    return sorted(entries.values(), key=lambda item: (-len(item["name"]), item["name"].casefold()))


def build_outputs() -> dict[str, str]:
    tools = records(ROOT / "data/tools.json", "tools")
    jobs = records(ROOT / "data/jobs.json", "jobs")
    meta = json.loads((ROOT / "data/meta.json").read_text(encoding="utf-8"))
    layout = json.loads((ROOT / "data/wiki-layout.json").read_text(encoding="utf-8"))
    quick_start = json.loads((ROOT / "data/quick-start.json").read_text(encoding="utf-8"))
    tool_access = json.loads((ROOT / "data/tool-access.json").read_text(encoding="utf-8"))
    job_workspaces = json.loads((ROOT / "data/job-workspaces.json").read_text(encoding="utf-8"))
    field_notes = json.loads((ROOT / "data/field-notes.json").read_text(encoding="utf-8"))
    validate_field_note_commands(field_notes)
    knowledge_policy = json.loads((ROOT / "data/knowledge-policy.json").read_text(encoding="utf-8"))
    research_queue = json.loads((ROOT / "data/research-queue.json").read_text(encoding="utf-8"))
    online_data = json.loads((ROOT / "data/online-tools.json").read_text(encoding="utf-8"))
    command_center = json.loads((ROOT / "data/command-center.json").read_text(encoding="utf-8"))
    updates_center = json.loads((ROOT / "data/updates-center.json").read_text(encoding="utf-8"))
    validate_updates_center(updates_center)
    validate_command_center(command_center)
    online_module = brand_module("online_tools.py")
    online_module.validate_online(online_data, tools, jobs)
    reputation = json.loads((ROOT / "data/tool-reputation.json").read_text(encoding="utf-8"))
    validate(tools, jobs)
    validate_reputation(reputation, tools)
    news = json.loads((ROOT / "data/news-sources.json").read_text(encoding="utf-8"))
    validate_news(news)
    validate_layout(layout, jobs, tools)
    validate_tool_access(tool_access, tools)
    validate_quick_start(quick_start, tools, jobs, tool_access["tools"])
    validate_job_workspaces(job_workspaces, jobs, tools, tool_access["tools"])
    health_report = knowledge_module().knowledge_report(knowledge_policy, research_queue, tools, jobs, layout,
                                                       field_notes, job_workspaces, tool_access["tools"], meta["reviewedAt"], quick_start)
    if not (ROOT / "docs/free-tool-picks.md").is_file():
        raise ValueError("Quick-start selections require docs/free-tool-picks.md for their evidence and limits")
    linker = link_module()
    registry = linker.build_registry(tools, jobs)
    brands_module = brand_module()
    web_brand_path = ROOT / "data/web-brand-icons.json"
    web_brands = (json.loads(web_brand_path.read_text(encoding="utf-8")) if web_brand_path.is_file()
                  else {"schemaVersion": 1, "assets": {}, "brands": []})
    brands = merge_web_brands(brands_module.load_registry(ROOT), web_brands)
    brand_errors = brands_module.validate_registry(brands, ROOT)
    if brand_errors:
        raise ValueError("Brand icon validation failed: " + "; ".join(brand_errors))
    if {entry.get("toolId") for entry in brands["brands"] if entry.get("toolId")} != {tool["id"] for tool in tools}:
        raise ValueError("Every catalog tool requires one authentic brand icon")
    registry = extend_web_brand_links(registry, web_brands["brands"])
    brand_markdown = brand_module("brand_markdown.py")
    decorator = brand_markdown.BrandMarkdown(brands, linker)
    data = {"tools": [{**tool, "reputation": reputation["tools"][tool["id"]]} for tool in tools], "jobs": jobs, "meta": meta, "links": linker.portal_links(registry),
            "quickStart": quick_start, "toolAccess": tool_access["tools"], "jobWorkspaces": job_workspaces, "commandCenter": command_center, "updatesCenter": updates_center,
            "layout": layout, "fieldNotes": field_notes, "onlineTools": online_data, "brands": brands_module.portal_icons(brands), "news": news}
    data_json = json.dumps(data, ensure_ascii=False, separators=(",", ":"))
    safe_json = data_json.replace("<", "\\u003c").replace("\u2028", "\\u2028").replace("\u2029", "\\u2029")
    template = (ROOT / "portal/index.template.html").read_text(encoding="utf-8")
    replacements = {
        "{{STYLES}}": (ROOT / "portal/styles.css").read_text(encoding="utf-8"),
        "{{DATA}}": "window.INTA_DATA = " + safe_json + ";",
        "{{APP}}": (ROOT / "portal/app.js").read_text(encoding="utf-8"),
        "{{FAVICON}}": "data:image/svg+xml;base64," + base64.b64encode((ROOT / "docs/assets/inta-mark.svg").read_bytes()).decode("ascii"),
    }
    for marker, value in replacements.items():
        if marker not in template:
            raise ValueError(f"Template missing {marker}")
        template = template.replace(marker, value)
    if re.search(r"\{\{(?:STYLES|DATA|APP|FAVICON)\}\}", template):
        raise ValueError("Unresolved template marker")
    by_id, jobs_by_id = {x["id"]: x for x in tools}, {x["id"]: x for x in jobs}
    access = tool_access["tools"]
    quick_start_block = linker.link_markdown(quick_start_markdown(quick_start, by_id, layout, access), registry)
    quick_start_block = online_module.start_markdown(online_data, by_id, access) + "\n" + quick_start_block
    managed_sources = {
        name: replace_quick_start_block((ROOT / name).read_bytes().decode("utf-8"), quick_start_block, name)
        for name in ("README.md", "docs/start-here.md", "docs/free-tool-picks.md")
    }
    categories = defaultdict(list)
    for tool in tools:
        categories[tool["category"]].append(tool)
    pwa_manifest = {
        "id": "./", "name": "INTA Computer Services", "short_name": "INTA",
        "description": "INTA technician tools, diagnostics, updates and maintenance.",
        "start_url": "./#updates", "scope": "./", "display": "standalone",
        "background_color": "#111214", "theme_color": "#111214",
        "categories": ["productivity", "utilities"],
        "icons": [{"src": "./inta-app-icon.svg", "sizes": "any", "type": "image/svg+xml", "purpose": "any maskable"}],
    }
    pwa_icon = """<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
      <rect width="512" height="512" rx="104" fill="#111214"/>
      <g fill="#e7e9ee"><path d="M104 104h128v128H104zM280 104h128v128H280zM104 280h128v128H104z"/></g>
      <path d="M280 280h128v128H280z" fill="none" stroke="#8bb8ff" stroke-width="18"/>
    </svg>"""
    service_worker = """const CACHE = "inta-shell-v1";
const SHELL = ["./", "./manifest.webmanifest", "./inta-app-icon.svg"];
self.addEventListener("install", event => {
  event.waitUntil(caches.open(CACHE).then(cache => cache.addAll(SHELL)).then(() => self.skipWaiting()));
});
self.addEventListener("activate", event => {
  event.waitUntil(caches.keys().then(keys => Promise.all(keys.filter(key => key.startsWith("inta-shell-") && key !== CACHE).map(key => caches.delete(key)))).then(() => self.clients.claim()));
});
self.addEventListener("fetch", event => {
  const request = event.request;
  if (request.method !== "GET" || new URL(request.url).origin !== self.location.origin) return;
  if (request.mode === "navigate") {
    event.respondWith(fetch(request).then(response => {
      if (response && response.ok) { const copy = response.clone(); caches.open(CACHE).then(cache => cache.put("./", copy)); }
      return response;
    }).catch(() => caches.match("./").then(cached => cached || Response.error())));
    return;
  }
  event.respondWith(fetch(request).then(response => {
    if (response && response.ok) { const copy = response.clone(); caches.open(CACHE).then(cache => cache.put(request, copy)); }
    return response;
  }).catch(() => caches.match(request)));
});
"""
    outputs = {"site/index.html": template, "site/.nojekyll": "", "site/catalog.json": data_json + "\n",
               "site/knowledge-health.json": json.dumps(health_report, ensure_ascii=False, indent=2) + "\n",
               "site/manifest.webmanifest": json.dumps(pwa_manifest, ensure_ascii=False, indent=2) + "\n",
               "site/inta-app-icon.svg": pwa_icon + "\n", "site/sw.js": service_worker}
    outputs.update(managed_sources)
    outputs.update(online_module.wiki_outputs(online_data, tools, access))
    for job in jobs:
        quick_items = [item for item in quick_start["items"] if item["jobId"] == job["id"]]
        workspace = job_workspaces["jobs"][job["id"]]
        notes = [note for note in field_notes["notes"] if note["jobId"] == job["id"]]
        outputs[f"wiki/Job-{job['id']}.md"] = job_markdown(job, by_id, True, layout, jobs_by_id, meta["reviewedAt"], quick_items, access, workspace, notes)
        outputs[f"docs/runbooks/{job['id']}.md"] = job_markdown(job, by_id, False, layout, jobs_by_id, meta["reviewedAt"], quick_items, access, workspace, notes)
    outputs["wiki/Service-Map.md"] = service_map_markdown(jobs, layout, field_notes, job_workspaces)
    outputs["wiki/Research-Queue.md"] = research_queue_markdown(research_queue, jobs, layout)
    outputs["wiki/Knowledge-Health.md"] = knowledge_health_markdown(health_report, knowledge_policy, jobs)
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
        outputs[f"wiki/Tools-{slug(category)}.md"] = category_markdown(category, items, layout, jobs_by_id, meta["reviewedAt"], access)
    tool_index += ["", anchor("complete-catalog"), "", *heading(2, "Complete catalog", "📇 Every tool, one place"),
                  "| Tool | Discipline | Choose it for | Official page + download / access |", "| --- | --- | --- | --- |"]
    for tool in sorted(tools, key=lambda item: item["name"].casefold()):
        tool_index += [f"| [{tool['name']}]({WIKI}/Tools-{slug(tool['category'])}#{tool['id']}) | "
                       f"{cell(CATEGORY_NAMES.get(tool['category'], tool['category']))} | {cell(tool['bestFor'])} | "
                       f"{tool_access_links(tool, access, all_routes=True)} |"]
    tool_index += page_footer(meta["reviewedAt"])
    outputs["wiki/Tool-Directory.md"] = "\n".join(tool_index)
    outputs["wiki/Home.md"] = home_markdown(tools, jobs, categories, layout, meta, quick_start_block)
    default_answers = {}
    for item in quick_start["items"]:
        default_answers.setdefault(item["jobId"], item)
    sidebar = [*heading(2, "INTA knowledge base", "🧭 INTA / Service Atlas"),
               f"**[🏠 Home](Home)** · **[🧰 Open hub]({HUB})** · **[🗺️ Service map](Service-Map)**", "",
               *heading(3, "Fix a problem", "Playbook groups")]
    group_anchors = {group["jobs"][0]: "".join(legacy_anchors(f"{group['icon']} {group['title']}"))
                     for group in layout["groups"]}
    for identity in layout["scenarioOrder"]:
        sidebar += [f"- {group_anchors.get(identity, '')}"
                    f"{scenario_link(identity, layout, wiki_page=default_answers[identity]['wikiPage'])}"]
    sidebar += ["", "### 🌐 Online tools", "", f"- **[Open all {len(online_data["tools"])} online services](Online-Tools)**"]
    sidebar += [f"- **[{group["label"]}](Online-{group["id"]})**" for group in online_data["groups"]]
    sidebar += ["", "### Quick tool downloads", "",
                "Tool name = official page. File buttons name the platform; built-in tools open instructions.", ""]
    for shortcut in layout["quickDownloads"]:
        tool = by_id[shortcut["toolId"]]
        primary = access[tool["id"]]["primary"]
        sidebar += [f"- **[{shortcut['icon']} {shortcut['label']}]({access[tool['id']]['officialUrl']})** · "
                    f"**[{cell(primary['label'])}]({primary['url']})**"]
    sidebar += ["", *heading(3, "Tool categories", "🧰 Tool disciplines")]
    for category in layout["categoryOrder"]:
        info = layout["categories"][category]
        sidebar += [f"- **[{info['navigationIcon']} {info['navigationLabel']}](Tools-{slug(category)})**"]
    sidebar += ["", "### Reference", "",
                "- [Decision desk](Decision-Desk)", "- [⚡ Command Center](Command-Center)", "- [⚡ Quick commands](Quick-Commands)",
                "- [All playbooks](Playbooks)", "- [Complete tool directory](Tool-Directory)",
                "- [Updates & Maintenance](Updates-and-Maintenance)",
                "- [Methods](Methods)", "- [Rescue media](Rescue-Media)", "- [Intake and handoff](Technician-Handoff)",
                "- [Service evidence](Service-Evidence)", "- [Tool selection](Tool-Selection)",
                "- [Free Tool Picks](Free-Tool-Picks)", "- [Current tool choices](Current-Tool-Choices)",
                "- [Malware tool choices](Malware-Tool-Choices)", "",
                *legacy_anchors("📏 The shared standard"), "", "### Standards and contributions", "",
                "- [Start here](Start-Here)", "- [Knowledge standards](Knowledge-Standards)", "- [Interface design](Interface-Design)",
                "- [Contribute](Contribute)", "- [Contribution templates](Templates)",
                "- [Organization rules](Knowledge-Organization)", "- [Coverage and review health](Knowledge-Health)",
                "- [Research queue](Research-Queue)", "- [Research updates](Research-Updates)", "- [Online tool review](Online-Tool-Review)",
                "- [Continuous improvement](Continuous-Improvement)", "- [Maintain and publish](Maintaining-the-Hub)", ""]
    outputs["wiki/_Sidebar.md"] = "\n".join(sidebar).rstrip() + "\n"
    outputs["wiki/_Footer.md"] = (f"**INTA Computer Services**\n\n"
        f"[Hub]({HUB}) · [Standards]({WIKI}/Knowledge-Standards) · "
        f"[Interface design]({WIKI}/Interface-Design) · [Contribute]({WIKI}/Contribute)\n")
    mirrors = WIKI_MIRRORS
    for title, filename in mirrors.items():
        source_name = f"docs/{filename}"
        source = managed_sources[source_name] if source_name in managed_sources else (ROOT / source_name).read_text(encoding="utf-8")
        outputs[f"wiki/{title}.md"] = mirror_markdown(source, mirrors, meta["reviewedAt"])
    for item in quick_start["items"]:
        outputs[f"wiki/{item['wikiPage']}.md"] = quick_answer_markdown(item, by_id, access, quick_start["reviewedAt"],
                                                                    job_workspaces["jobs"][item["jobId"]])
    # Apply the same source-owned identities to authored repository documents as
    # the generated Wiki. Rebuilding never adds a second icon to a mention.
    authored = [ROOT / name for name in ("README.md", "AGENTS.md", "CONTRIBUTING.md")]
    authored += sorted((ROOT / "docs").rglob("*.md"))
    for source in authored:
        path = str(source.relative_to(ROOT))
        if path not in outputs and source.is_file():
            outputs[path] = source.read_text(encoding="utf-8")
    for path, content in list(outputs.items()):
        if path.endswith(".md"):
            outputs[path] = decorator.decorate(linker.link_markdown(brand_markdown.strip_icons(content), registry))
    manifest = {"schemaVersion": 2, "tools": len(tools), "jobs": len(jobs),
                "methods": len(records(ROOT / "data/methods.json", "methods")),
                "quickStartItems": len(quick_start["items"]),
                "quickAnswerPages": len(quick_start["items"]),
                "scenarioToolChoices": sum(len(resolved_tool_choices(item, access)) for item in quick_start["items"]),
                "scenariosWithToolChoices": sum(bool(resolved_tool_choices(item, access)) for item in quick_start["items"]),
                "quickStartSha256": hashlib.sha256(json.dumps(quick_start, sort_keys=True, ensure_ascii=False).encode()).hexdigest(),
                "onlineTools": len(online_data["tools"]),
                "commandCenterCommands": len(command_center["commands"]),
                "updatesCenterCommands": len(updates_center["commands"]),
                "updatesCenterChannels": len(updates_center["channels"]),
                "updatesCenterWorkflows": len(updates_center["workflows"]),
                "commandCenterWorkflows": len(command_center["workflows"]),
                "commandCenterToolkits": len(command_center["toolkits"]),
                "onlineGroups": len(online_data["groups"]),
                "onlineWorkflows": len(online_data["workflows"]),
                "toolAccessEntries": len(access),
                "directFileDownloads": sum(entry["primary"]["kind"] == "file" for entry in access.values()),
                "toolAccessSha256": hashlib.sha256(json.dumps(tool_access, sort_keys=True, ensure_ascii=False).encode()).hexdigest(),
                "jobWorkspaces": len(job_workspaces["jobs"]),
                "serviceAreas": len(layout["groups"]),
                "fieldNotes": len(field_notes["notes"]),
                "researchItems": len(research_queue["items"]),
                "jobResources": sum(len(entry["resources"]) for entry in job_workspaces["jobs"].values()),
                "jobDecisionBranches": sum(len(entry["decisions"]) for entry in job_workspaces["jobs"].values()),
                "jobWorkspacesSha256": hashlib.sha256(json.dumps(job_workspaces, sort_keys=True, ensure_ascii=False).encode()).hexdigest(),
                "categories": dict(sorted(Counter(t["category"] for t in tools).items())),
                "contentSha256": hashlib.sha256(data_json.encode()).hexdigest(),
                "wikiLayoutSha256": hashlib.sha256(json.dumps(layout, sort_keys=True, ensure_ascii=False).encode()).hexdigest(),
                "linkRegistryEntries": len(registry),
                "registeredLinks": len(registry),
                "brandIdentities": len(brands["brands"]),
                "brandAssets": len(brands["assets"]),
                "toolsWithIcons": len({item["toolId"] for item in brands["brands"] if item.get("toolId")}),
                "brandedMentions": sum(content.count('data-inta-brand="') for path, content in outputs.items() if path.endswith(".md")),
                "brandRegistrySha256": hashlib.sha256(json.dumps(brands, sort_keys=True, ensure_ascii=False).encode()).hexdigest(),
                "wikiPages": len([x for x in outputs if x.startswith("wiki/") and not Path(x).name.startswith("_")]),
                "wikiFiles": len([x for x in outputs if x.startswith("wiki/")]),
                "wikiDiagrams": sum(content.count("```mermaid") for path, content in outputs.items() if path.startswith("wiki/")),
                "files": {path: hashlib.sha256(content.encode()).hexdigest() for path, content in sorted(outputs.items())}}
    outputs["site/build-manifest.json"] = json.dumps(manifest, indent=2) + "\n"
    return outputs




def validate_field_note_commands(field_notes: dict) -> None:
    """Validate every copy-ready command embedded in a specific-fix article."""
    if not isinstance(field_notes, dict) or not isinstance(field_notes.get("notes"), list):
        raise ValueError("Field notes require a notes list")
    shells = {"PowerShell", "PowerShell (Run as Administrator)", "CMD", "CMD or PowerShell",
              "CMD (Run as Administrator)", "PowerShell or CMD (Run as Administrator)",
              "Linux shell (INTA Rescue)"}
    seen = set()
    for note in field_notes["notes"]:
        for command in note.get("commands", []):
            identity = f"{note.get('id', '?')}/{command.get('id', '?')}"
            if not isinstance(command, dict):
                raise ValueError(f"{identity}: command must be an object")
            for field in ("id", "label", "shell", "purpose", "safety", "code", "sourceUrl", "sourceTitle"):
                if not isinstance(command.get(field), str) or not command[field].strip():
                    raise ValueError(f"{identity}: command requires {field}")
            if not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", command["id"]) or identity in seen:
                raise ValueError(f"{identity}: invalid or duplicate command ID")
            seen.add(identity)
            if command["shell"] not in shells:
                raise ValueError(f"{identity}: unsupported shell label")
            if len(command["code"]) > 12000 or re.search(r"(?i)(?:password|recovery key|access token)\s*=", command["code"]):
                raise ValueError(f"{identity}: command is oversized or appears to embed a secret")
            parsed = urlparse(command["sourceUrl"])
            if parsed.scheme != "https" or not parsed.hostname or parsed.username or parsed.password:
                raise ValueError(f"{identity}: official command reference must be public HTTPS")
            if re.search(r"(?i)\b(?:irm|iwr|curl)\b[^\n|]*\|\s*(?:iex|invoke-expression)", command["code"]):
                raise ValueError(f"{identity}: remote script download-and-execute pipelines are prohibited")


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
