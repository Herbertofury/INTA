#!/usr/bin/env python3
"""Check organization and emit a complete, non-mutating maintenance report."""
from __future__ import annotations

import argparse
from datetime import date
import importlib.util
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))
from knowledge_health import knowledge_report


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--as-of", default=date.today().isoformat(), help="ISO date for review-age arithmetic; defaults to today")
    parser.add_argument("--output", type=Path, help="Write the complete JSON report here")
    args = parser.parse_args()
    try:
        spec = importlib.util.spec_from_file_location("inta_builder", ROOT / "scripts/build-hub.py")
        builder = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(builder)
        read = lambda name: json.loads((ROOT / "data" / (name + ".json")).read_text(encoding="utf-8"))
        tools, jobs, layout, access, workspaces = (read(x) for x in ("tools", "jobs", "wiki-layout", "tool-access", "job-workspaces"))
        builder.validate(tools, jobs)
        builder.validate_layout(layout, jobs, tools)
        builder.validate_tool_access(access, tools)
        builder.validate_job_workspaces(workspaces, jobs, tools, access["tools"])
        report = knowledge_report(read("knowledge-policy"), read("research-queue"), tools, jobs, layout,
                                  read("field-notes"), workspaces, access["tools"], args.as_of)
    except (ValueError, KeyError, TypeError, OSError) as error:
        print(f"Knowledge check failed: {error}", file=sys.stderr)
        return 1
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    counts = report["counts"]
    print(f"Knowledge organization passed: {counts['serviceAreas']} service areas, {counts['jobs']} jobs, "
          f"{counts['tools']} tools, {counts['fieldNotes']} specific fixes, {counts['researchItems']} research tasks.")
    print(f"Editorial reviews due as of {report['asOf']}: {counts['dueReviews']}. Dates were not changed.")
    for item in report["dueReviews"]:
        print(f"  {item['status']}: {item['kind']}/{item['id']} (due {item['dueAt']})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
