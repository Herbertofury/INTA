# INTA Rust Local Agent 0.2.0

Replaces the C#/.NET local maintenance bridge with Rust while keeping `/api/v1`, camelCase response fields, port 47821, and existing local operation history.

- Preserves 40 supported typed manager actions across 11 manager integrations, with exact package-ID validation and one confirmation per modifying request.
- Uses Rust for manager orchestration, process capture, operation state, history, origin/host checks, session pairing, portable hosting, installation, and release packaging.
- Adds atomic batch validation, including the Windows Update restriction that previously differed between single and batch requests.
- Keeps manager locks until the original installer exits; browser disconnects never kill installers.
- Replaces optimistic post-check claims with explicit completed-review states unless concrete verification evidence exists.
- Provides a current-user native installer (`--install`), portable data directory (`--data-dir`), and bundled local browser workspace (`--serve-site … --open`).
- Produces platform ZIPs and SHA-256 sidecars with the Rust packager.

Read the [agent guide](README.md) for supported operations and platform limits. Linux fixture and HTTP integration tests do not prove real Windows/macOS package-manager behavior. The local browser host does not constitute a native Mobile client or authenticated remote relay. Downloaded binaries remain unsigned unless the published release explicitly says otherwise.
