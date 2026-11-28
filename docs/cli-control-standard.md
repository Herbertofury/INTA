# CLI control for INTA tools

Every tool INTA recommends, ships, installs, launches or puts in a repair
plan needs a tested, machine-readable control surface. Use
[CLI-Anything](https://github.com/HKUDS/CLI-Anything) conventions: prefer an
official CLI or API, otherwise make a reviewed harness against the genuine
backend. Pin product and wrapper versions and reviewed bytes; use one-shot
operation IDs, structured output and an observable result. Never substitute
a GUI macro, download link or installer for the actual task.

Review the platform, edition, applicable install/discovery, launch, and
every operation exposed by INTA. Provide primary sources, exact successful
runtime checks and predictable errors. Record whether the operation only
reads or modifies the device, its required privileges, cancellation and
rollback/review conditions. If something cannot be controlled, keep its
reference clearly manual-only, explain why and surface a qualifying alternative.
Do not silently remove useful existing links or advertise stub automation.

Desktop, Mobile and agent actions must share the registry. A reconnect plan
can refer only to a registered version and an actually tested operation with
validated arguments. Authorize the exact target, outcome, and effect before
queueing. Never forward raw shell text. Live dispatch must validate the
approved operation at execution time, and failures must not trigger automatic
repetition of a possibly completed mutation.

```sh
python3 scripts/check-tool-cli.py --json
python3 scripts/check-tool-cli.py --require-full TOOL_ID --platform windows
python3 scripts/check-tool-cli.py --require-operation TOOL_ID OPERATION_ID --platform windows \
  --version REVIEWED_VERSION --artifact-sha256 REVIEWED_SHA256
```

The catalog report is read-only. Unmapped tools stay visible as `needs-tool-review`;
they do not become automation-ready by omission. The ledger currently tracks
main's tool catalog, not an exhaustive inventory of bundled rescue packages or
hardware support. A `candidate` is not runtime verification; a `tested` record
requires primary documentation, exact version, artifact SHA-256 and actual
runtime evidence. The `complete` scope claim is set only after every relevant
feature has been reviewed; one qualified operation does not imply parity.

The first candidate harness is a read-only native Power Plans inventory through
the pinned INTA agent and documented Windows power APIs:

```sh
python3 scripts/cli-anything-inta.py --json power-plans \
  --agent-binary ABSOLUTE_PATH_TO_REVIEWED_INTA_AGENT \
  --agent-sha256 APPROVED_PACKAGE_EXECUTABLE_SHA256
```

Use only an administrator-controlled package location whose bytes are already
trusted. Use the repository’s self-contained single-file packaged Local Agent,
not a thin executable with unpinned sidecar DLLs. This command is not admitted into remote job dispatch. It requires no
agent service or background polling and never changes a power plan. Exit 0
means ready data, 1 means a supported command returned a non-ready observation
(e.g. unsupported OS), and 2 means the request or backend contract was refused.

A Windows CI artifact is evidence for an exact short-lived preview, not
production approval. An unexpired read-only preview may be gated locally with
`--require-operation ... --preview` on its exact platform, version and executable
hash. It is never counted as full coverage or automatically accepted for a
Desktop, Mobile or agent repair. Expired previews remain visible for history
but are refused for use.

### Windows startup registration preview

```text
python3 scripts/cli-anything-inta.py --json startup-inventory --agent-binary ABSOLUTE_PATH --agent-sha256 EXACT_SHA256
```

The command reads names only from documented Run and RunOnce registry scopes.
It does not expose command targets, enable or disable entries, enumerate the
Startup folders, or assess other autostart sources. Missing sources are
distinguished from unreadable or truncated ones. Names can be sensitive, so
keep output local and do not send or store it in shared logs by default.
The command is not a production certification for a full startup manager.

Optional `--include-folders` keeps that registry inventory and adds base names
from the current-user and all-users Startup folders and a Windows Settings link
(unopened). It uses the same collector as Desktop and does not open files, resolve
shortcut targets, disclose full paths, start items, or change startup settings.
A redirected or unreadable folder is reported as incomplete, not empty.
The default remains the original registry-only scope.

### Registered desktop programs preview

```text
python3 scripts/cli-anything-inta.py --json installed-programs --agent-binary ABSOLUTE_PATH --agent-sha256 EXACT_SHA256
```

This uses the same reader as the Windows Desktop view. It returns registered
display name, version, publisher and scope. It does not expose registry IDs,
installation paths or uninstall commands, inventory Store apps, or modify
the system. Skipped display rows and capped or unreadable scopes are reported
rather than guessed. Keep this inventory local by default. This command does
not authorize an uninstall, package identity or tool-wide admission.

### Selected folder storage preview

```text
python3 scripts/cli-anything-inta.py --json storage-preview --folder ABSOLUTE_CHILD_FOLDER --minimum-age-days 7 --entry-limit 100000 --agent-binary ABSOLUTE_PATH --agent-sha256 EXACT_SHA256
```

This calls the existing native metadata-only storage preview and reports totals,
not names, paths, file contents, per-file actions or reclaimable space. Logical
bytes are not physical disk usage. Exit 1 means usable but incomplete evidence
(an entry or depth limit or a read failure); exit 2 means refused. Choose a
specific folder you are authorized to inspect. This is local-only; it does not
grant mobile queue admission or probe, optimize, delete, clean or repair files.

### rclone local SHA-256 preview

```text
python3 scripts/cli-anything-inta.py --json rclone-sha256 --rclone-binary ABSOLUTE_EXECUTABLE --rclone-sha256 EXACT_BINARY_SHA256 --file ABSOLUTE_FILE --expect-sha256 EXPECTED_FILE_SHA256
```

Use an already acquired [official rclone build](https://rclone.org/downloads/).
The explicit [rclone stdin hash command](https://rclone.org/commands/rclone_hashsum/)
receives file bytes only, never a filename, remote or arbitrary options.
The OS null config disables the account configuration. One selected regular
file up to 32 MiB; nothing is uploaded, written or changed. Exit 1 means the
expected hash differed; exit 2 means refused. A hash is not a signature.
Keep hashes local unless the operator chooses to share them. This single
operation is a preview, not Sync, cloud-account or tool-wide CLI admission.
