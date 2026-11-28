# Native capability CLI

Run **inta-capabilities.exe** to discover the catalog and perform seven local, read-only operations. The Windows x64 executable embeds the public catalog, its action and coverage metadata, and the additional Workstation records. Each public record takes precedence over an additional record with the same ID. The checked Windows release returned 319 distinct tools from 260 public records and 59 additional records. [Canonical records](../../data/tools.json) · [Additional records](../workbench/data/additional-tools.json) · [Native acceptance](evidence/windows-x64.json)

## Build and inspect

From this directory, build with a stable Rust toolchain and Windows x64 MSVC build tools. A configured Cargo dependency cache supports an offline rebuild.

~~~powershell
cargo build --release --locked
$cli = '.\target\release\inta-capabilities.exe'
& $cli discover
& $cli tools search 'power' 20
& $cli tools inspect powercfg
& $cli schema system.power
& $cli run system.power
~~~

All commands emit one UTF-8 JSON object to standard output. Success returns **schemaVersion**, **ok** and **result**. A failure returns **schemaVersion**, **ok** and **error** containing a stable code and a human-readable message. Exit 0 means success, 1 identifies an unavailable or incomplete observation or I/O failure, and 2 identifies a refused request, integrity or test failure. A leading **--json** is accepted. An empty search returns all matching records up to the selected limit. The default is 50 and the maximum is 500.

Each operation descriptor carries version 1, input and output schemas, backend, platform, effect, elevation requirement, network requirement, associated catalog IDs and Forge eligibility. A tool inspection preserves the canonical metadata, the source coverage record, publisher access information and the native operations linked to that tool. These records can be read offline. A publisher link or historical preview receipt grants no launch or installation permission. [Existing coverage standard](../../docs/cli-control-standard.md)

## Available operations

- **tools.search** takes a query and an optional limit. It matches all search terms across ID, name, category, summary and tags, with exact ID and name matches first.
- **tools.inspect** takes one catalog ID and returns its source fields and operation coverage.
- **system.memory** reads total physical bytes, available physical bytes and memory-load percentage through the Windows memory status API.
- **system.power** enumerates power-plan GUIDs through the documented Windows power APIs and verifies the active GUID before and after enumeration. The implementation follows the source agent's existing bounded, stable-snapshot model. [Power Plans collector](../../agent/INTA.LocalAgent/PowerPlans.cs) · [Microsoft PowerEnumerate](https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerenumerate)
- **startup.counts** reads registry metadata for Run and RunOnce in the shared current-user view and both machine views. It returns the number of values in each, including any unnamed value. A missing key has a distinct status. This operation collects zero registration names, target paths or command arguments. [Existing startup collector](../../agent/INTA.LocalAgent/StartupInventory.cs)
- **file.sha256** takes an absolute path to a regular locally attached file of up to 256 MiB. It holds the ancestor directories during the read and opens the file without write or delete sharing. Unsupported device names, alternate streams, UNC syntax, links, junctions, traversal and offline cloud placeholders are refused.
- **text.sha256** hashes up to 8 KiB of literal UTF-8 text. It preserves line endings and Unicode normalization.

To supply typed inputs, create a small local JSON file and run **run OPERATION --input FILE**. For example, use the following document with **file.sha256**. JSON documents must contain unique object keys. The application returns the resulting byte count and digest.

~~~json
{"path":"I:\\Selected fixtures\\verified.bin"}
~~~

## Offline Forge

A recipe is a small JSON document that combines reviewed operation IDs, version numbers, typed inputs and expected output values. **forge schema** returns its JSON Schema. **forge verify** validates the document, runs the approved local observations and checks every vector. **forge import** repeats verification before atomically publishing one record to a selected local directory. The parent directory must already exist. **forge run** checks the saved record and repeats the test vectors before returning observations. [Example recipe](examples/verify-workstation-basics.json)

~~~powershell
& $cli forge schema
& $cli forge verify .\examples\verify-workstation-basics.json
& $cli forge import .\examples\verify-workstation-basics.json --store 'I:\INTA\My recipes'
& $cli forge run verify-workstation-basics --store 'I:\INTA\My recipes'
~~~

V1 accepts up to eight steps and 24 vectors. Every step has at least one test. Each vector compares one scalar in a recorded result with a declared expected value through a bounded JSON pointer. A failed vector rejects the import. Inputs are literal values. The Forge interpreter has six allowlisted operation types and no process, shell, network, plugin, targeting or mutation instruction. A file digest requires a separate direct invocation with a user-selected path. Adding an operation requires review, implementation, tests and rebuilding the executable.

Reimporting identical content succeeds. A different recipe using an existing ID returns a conflict. The receipt includes a SHA-256 of the normalized manifest. Saving an expected digest separately allows an operator to compare revisions. Local store checks detect damaged or inconsistent records. Authentication of a recipe's author requires a separately trusted expected digest or signature. A user with write access to the store can replace a record and its stored digest. All loaded content still passes the same closed schema and operation restrictions.

## Compose a baseline with a local model

Start the prepared local-only Ollama service in Workstation. In PowerShell, choose the installed model and a new recipe filename in an existing directory. The **ai forge** command runs in the Workstation CLI. The resulting file also works with the standalone v1 capability CLI.

~~~powershell
$models = (inta assistant models | ConvertFrom-Json).models
$model = $models | Where-Object name -eq 'qwen2.5:0.5b' | Select-Object -First 1
$recipe = 'I:\INTA\readiness-baseline.json'
$store = 'I:\INTA\My recipes'
$request = 'Check installed memory capacity and the active Windows power plan.'
$receipt = inta ai forge $model.name $model.digest $request $recipe --approve | ConvertFrom-Json
$receipt.selectedOperations
inta forge verify $recipe
inta forge import $recipe --store $store
inta forge run $receipt.id --store $store
~~~

The model receives the one-line request and a fixed description of three choices. It can select Windows memory capacity, Windows power plans, or the six Run and RunOnce registration counts. INTA reads the selected observations locally and fills in expected values. It reads them again before publishing the new file. A later verification flags changes to the selected capacity, active power scheme and scheme count, or per-view startup counts. Review the selected operations before importing. A small model can misinterpret a request.

The receipt contains the selected IDs, normalized manifest digest, model identity and number of passing checks. The exported recipe contains expected baseline values. The request and model response stay out of the recipe. The integration test ran on Windows x64 with the prepared Qwen2.5 0.5B model and verified compatibility with the original standalone Forge binary. [Windows integration receipt](evidence/forge-local-model.json)

## Verification and boundaries

~~~powershell
cargo fmt -- --check
cargo clippy --offline --locked --all-targets -- -D warnings
cargo test --offline --locked
python tests/acceptance.py $cli 'I:\INTA\QA\new-capability-receipt.json'
~~~

The release acceptance read back all 319 exact tool records and all seven schemas. An independent Windows API call checked physical-memory capacity, the native powercfg utility checked the plan IDs and selected plan, and Python checked all six registry views. The file fixture retained its original bytes. Two concurrent imports published one record and both callers obtained a truthful result. The integration tests cover malformed schemas, unknown operations, a device path, a network path, junctions, a named stream, tampering, conflicting imports, an omitted vector and failed expected output. Keep results for each newly packaged executable because they identify the tested bytes. [Runtime receipt](evidence/windows-x64.json) · [Integration tests](tests/contracts.rs)

The host evidence covers Windows 11 x64 build 22631. A Windows ARM64 build, physical-device certification, remote agent dispatch, package installation, the full operation set of every catalog product and public signing have separate release gates. The previous agent's historical preview for powercfg stays in its original versioned ledger. The new native operation has its own implementation and executable receipt. [Coverage ledger](../../data/tool-cli-coverage.json) · [Agent request model](../../agent/INTA.LocalAgent/Models.cs)

For packaging, run **python package_notices.py EXE NEW_DIRECTORY** from this crate. It reads the locked Windows target graph from the offline Cargo cache, records the selected binary's SHA-256 in a CycloneDX inventory, and preserves the original license files for each dependency. Include that directory with the reviewed executable. The intellectual-property and release decisions for original application code and embedded catalog content remain separate from the dependency inventory.

## Upstream review

The CLI-Anything design informed the single-purpose native operation IDs, real backend calls, structured output and reproducible fixtures. The reviewed reference was commit **34f519533bc175d2fe287ab8316b0dd99bb9cc43**. The original upstream README, license and registry were acquired into the private development reference cache. This crate contains an independent Rust implementation and consumes the existing INTA catalog and coverage records. [Reviewed design](https://github.com/HKUDS/CLI-Anything/blob/34f519533bc175d2fe287ab8316b0dd99bb9cc43/README.md) · [Upstream registry](https://github.com/HKUDS/CLI-Anything/blob/34f519533bc175d2fe287ab8316b0dd99bb9cc43/registry.json) · [Upstream license](https://github.com/HKUDS/CLI-Anything/blob/34f519533bc175d2fe287ab8316b0dd99bb9cc43/LICENSE)
