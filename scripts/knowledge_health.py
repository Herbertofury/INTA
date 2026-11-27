"""Deterministic knowledge organization, coverage and freshness checks.

This module never contacts a vendor or changes a review date. Freshness findings
are a review queue, not a claim that a product is broken or unsafe.
"""
from __future__ import annotations

from collections import Counter, defaultdict
from datetime import date, timedelta
import re
import unicodedata
from urllib.parse import urlsplit, urlunsplit


def public_url(value):
    if not isinstance(value, str):
        return False
    parsed = urlsplit(value)
    return bool(parsed.scheme == "https" and parsed.hostname and not parsed.username
                and not parsed.password and not any(x in parsed.query.lower()
                for x in ("utm_", "fbclid=", "gclid=")))


def normalized_name(value):
    return re.sub(r"[^\w]+", "", unicodedata.normalize("NFKC", value).casefold())


def canonical_url(value):
    parsed = urlsplit(value)
    return urlunsplit((parsed.scheme.lower(), parsed.netloc.lower(),
                       parsed.path.rstrip("/"), parsed.query, ""))


def validate_field_notes(field_notes, jobs, tools):
    if field_notes.get("schemaVersion") != 1 or not isinstance(field_notes.get("notes"), list):
        raise ValueError("Field notes require schemaVersion 1 and a notes list")
    date.fromisoformat(field_notes.get("reviewedAt", ""))
    job_ids, tool_ids = {j["id"] for j in jobs}, {t["id"] for t in tools}
    ids, titles, covered = set(), set(), set()
    for note in field_notes["notes"]:
        identity = note.get("id", "")
        if not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", identity) or identity in ids:
            raise ValueError(f"Duplicate or invalid field-note ID: {identity}")
        ids.add(identity)
        if note.get("jobId") not in job_ids:
            raise ValueError(f"{identity}: unknown field-note job")
        covered.add(note["jobId"])
        for field in ("title", "symptom", "firstAction", "expected", "ifUnresolved"):
            if not isinstance(note.get(field), str) or not note[field].strip():
                raise ValueError(f"{identity}: field note needs {field}")
        title = normalized_name(note["title"])
        if title in titles:
            raise ValueError(f"Duplicate field-note title: {note['title']}")
        titles.add(title)
        steps = note.get("steps")
        if not isinstance(steps, list) or not steps or any(not isinstance(x, str) or not x.strip() for x in steps):
            raise ValueError(f"{identity}: concrete field-note steps are required")
        selected = note.get("toolIds")
        if not isinstance(selected, list) or len(selected) != len(set(selected)) or set(selected) - tool_ids:
            raise ValueError(f"{identity}: unknown or duplicate field-note tools")
        sources = note.get("sourceUrls")
        if not isinstance(sources, list) or not sources or len(sources) != len(set(sources)) or not all(public_url(x) for x in sources):
            raise ValueError(f"{identity}: distinct public HTTPS evidence is required")
        date.fromisoformat(note.get("reviewedAt", ""))
    if covered != job_ids:
        raise ValueError(f"Field notes must cover every job; missing {sorted(job_ids - covered)}")


def validate_organization(policy, queue, tools, jobs, layout, field_notes):
    if policy.get("schemaVersion") != 1 or policy.get("taxonomySource") != "data/wiki-layout.json":
        raise ValueError("Knowledge policy must use the existing canonical layout taxonomy")
    intervals = policy.get("reviewIntervalsDays", {})
    for key in ("coreTool", "specialistTool", "evaluateTool", "guide", "fieldNote", "resource", "acquisition"):
        if not isinstance(intervals.get(key), int) or isinstance(intervals[key], bool) or intervals[key] <= 0:
            raise ValueError(f"Invalid knowledge review interval: {key}")
    groups = layout.get("groups", [])
    group_ids = [group.get("id", "") for group in groups]
    grouped_jobs = [identity for group in groups for identity in group.get("jobs", [])]
    if (not groups or len(group_ids) != len(set(group_ids))
            or any(not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", identity) for identity in group_ids)
            or Counter(grouped_jobs) != Counter(job["id"] for job in jobs)):
        raise ValueError("Every job needs one service home with a distinct stable area ID")
    names, urls = defaultdict(list), defaultdict(list)
    for tool in tools:
        names[normalized_name(tool["name"])].append(tool["id"])
        urls[canonical_url(tool["homepage"])].append(tool["id"])
    duplicates = [identities for identities in names.values() if len(identities) > 1]
    if duplicates:
        raise ValueError(f"Duplicate normalized tool name: {duplicates}")
    allowed = {}
    known_tools = {tool["id"] for tool in tools}
    for exception in policy.get("duplicateOfficialPageAllowlist", []):
        key = frozenset(exception.get("toolIds", []))
        if len(key) < 2 or key - known_tools or not str(exception.get("reason", "")).strip():
            raise ValueError("A shared official-page exception needs known tools and a reason")
        allowed[key] = exception["reason"]
    duplicates = [identities for identities in urls.values() if len(identities) > 1 and frozenset(identities) not in allowed]
    if duplicates:
        raise ValueError(f"Duplicate official tool page needs review: {duplicates}")
    if queue.get("schemaVersion") != 1 or not isinstance(queue.get("items"), list):
        raise ValueError("Research queue requires schemaVersion 1 and items")
    targets = {"job": {job["id"] for job in jobs}, "category": set(layout["categories"])}
    covered = {kind: set() for kind in targets}
    ids, questions = set(), set()
    for item in queue["items"]:
        identity = item.get("id", "")
        if not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", identity) or identity in ids:
            raise ValueError(f"Duplicate or invalid research ID: {identity}")
        ids.add(identity)
        kind = item.get("kind")
        if kind not in targets or item.get("targetId") not in targets[kind]:
            raise ValueError(f"{identity}: unknown research target")
        covered[kind].add(item["targetId"])
        if item.get("status") not in {"queued", "investigating", "ready", "done", "blocked"}:
            raise ValueError(f"{identity}: unknown research status")
        if item.get("priority") not in {1, 2, 3}:
            raise ValueError(f"{identity}: research priority must be 1, 2 or 3")
        for field in ("question", "deliverable", "acceptance"):
            if not isinstance(item.get(field), str) or not item[field].strip():
                raise ValueError(f"{identity}: research needs {field}")
        key = (kind, item["targetId"], normalized_name(item["question"]))
        if key in questions:
            raise ValueError(f"{identity}: duplicate research question")
        questions.add(key)
        if not item.get("sourceUrls") or not all(public_url(x) for x in item["sourceUrls"]):
            raise ValueError(f"{identity}: official research starting sources are required")
        if item["status"] == "done" and not public_url(item.get("resultUrl")):
            raise ValueError(f"{identity}: completed research needs a result URL with public HTTPS")
    for kind in targets:
        if covered[kind] != targets[kind]:
            raise ValueError(f"Research queue must cover every {kind}: missing {sorted(targets[kind] - covered[kind])}")
    validate_field_notes(field_notes, jobs, tools)


def knowledge_report(policy, queue, tools, jobs, layout, field_notes, workspaces, access, as_of):
    validate_organization(policy, queue, tools, jobs, layout, field_notes)
    as_of = date.fromisoformat(as_of) if isinstance(as_of, str) else as_of
    intervals, reviews = policy["reviewIntervalsDays"], []

    def review(kind, identity, title, reviewed_at, interval, url):
        reviewed = date.fromisoformat(reviewed_at)
        if reviewed > as_of:
            raise ValueError(f"Future review date: {kind}/{identity} ({reviewed})")
        due = reviewed + timedelta(days=interval)
        reviews.append({"kind": kind, "id": identity, "title": title,
                        "reviewedAt": reviewed.isoformat(), "dueAt": due.isoformat(),
                        "status": "overdue" if due < as_of else "due" if due == as_of else "current",
                        "daysOverdue": max(0, (as_of - due).days), "url": url})

    for tool in tools:
        review("tool", tool["id"], tool["name"], tool["reviewedAt"], intervals[tool["tier"] + "Tool"], tool["homepage"])
        entry = access[tool["id"]]
        for index, action in enumerate([entry["primary"], *entry.get("alternates", [])]):
            review("acquisition", tool["id"] + "/" + action.get("id", str(index)), action["label"],
                   action["checkedAt"], intervals["acquisition"], action["evidenceUrl"])
    for job in jobs:
        review("guide", job["id"], job["title"], job["reviewedAt"], intervals["guide"],
               "https://github.com/Herbertofury/INTA/wiki/Job-" + job["id"])
    for note in field_notes["notes"]:
        review("field-note", note["id"], note["title"], note["reviewedAt"], intervals["fieldNote"], note["sourceUrls"][0])
    for job_id, workspace in workspaces["jobs"].items():
        for resource in workspace["resources"]:
            review("resource", job_id + "/" + resource["id"], resource["label"], resource["checkedAt"],
                   intervals["resource"], resource["sourceUrl"])
    note_counts = Counter(note["jobId"] for note in field_notes["notes"])
    due = sorted((r for r in reviews if r["status"] != "current"),
                 key=lambda item: (-item["daysOverdue"], item["kind"], item["id"]))
    return {"schemaVersion": 1, "asOf": as_of.isoformat(), "structuralStatus": "passed",
            "counts": {"serviceAreas": len(layout["groups"]), "jobs": len(jobs), "tools": len(tools),
                       "toolCategories": len(layout["categories"]), "fieldNotes": len(field_notes["notes"]),
                       "researchItems": len(queue["items"]), "reviewedRecords": len(reviews),
                       "dueReviews": len(due)},
            "coverage": [{"jobId": j["id"], "fieldNotes": note_counts[j["id"]],
                          "resources": len(workspaces["jobs"][j["id"]]["resources"]),
                          "decisions": len(workspaces["jobs"][j["id"]]["decisions"])} for j in jobs],
            "dueReviews": due, "reviews": reviews,
            "researchQueue": sorted(queue["items"], key=lambda item: (item["priority"], item["id"])),
            "limits": "Structural validation and editorial-age arithmetic only. No network, hardware test, source rereview or date updates."}
