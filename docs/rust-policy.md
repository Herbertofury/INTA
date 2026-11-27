# Mandatory Rust policy

**All first-party applications and executable functions must be written in [Rust](https://www.rust-lang.org/). A different implementation language is permitted only when an equivalent, reproducible benchmark proves Rust slower for the exact workload and supported target.** This is a requirement, not a preference.

The rule covers Desktop, Mobile, Rescue and Site; their UI behavior, shared engines, services, local agents, persistence, search, synchronization, action handling, update/recovery logic, build tools and test utilities. A new feature, a rewrite, a repair and an extracted helper have the same requirement. A Rust executable that launches or embeds a non-Rust application does not convert that application to Rust.

## What counts as proof

An exception must contain all of the following in the [exception register](../data/language-exceptions.json):

1. **Exact scope.** Identify the alternate implementation files/functions, language, target hardware/OS, representative workload and reason the measured difference matters. Do not turn one faster operation into permission for an entire client or framework.
2. **Comparable working implementations.** Preserve the optimized Rust source and the alternative source, exact revisions and source hashes, release/build settings, toolchain versions and dependency lockfiles. Run the same correct operation on the same complete data with the same output and error semantics. Debug Rust against optimized alternatives is invalid.
3. **Reproduction.** Retain the benchmark command, input fixture or reproducible input generator, environmental details, warm-up procedure, run order and raw timing samples. Include transfer, serialization, boundary crossings, startup and allocation costs when the shipped path incurs them. Include cold and warm paths when both matter.
4. **A measured difference beyond noise.** Use at least 20 paired independent measurements after any declared warm-up. Show the distribution, median and relevant tail latency. The checker requires the lower bound of a 95% confidence interval for the mean paired latency difference to remain positive; that consistency check does not replace reviewing workload validity or reproducing the benchmark. Report memory and throughput when they constrain the function. One selected fastest run or an unrelated microbenchmark is not proof.
5. **Correctness and review evidence.** Keep equivalence tests/results, limitations, reviewer and review date. Retain the source data needed to reproduce the result. An exception must not sacrifice features, accuracy, complete content, accessibility, authorization or failure handling to win the comparison.
6. **A narrow lifetime.** Re-run the comparison when the implementation, workload, compiler, dependency, target or relevant platform changes. Remove the exception when Rust is no longer measurably slower. The exception does not authorize unrelated new non-Rust behavior.

A smaller download, an easier framework, familiarity, faster development, unavailable tooling, a deadline, inherited code, or an unsupported claim that a language is faster is not the required performance proof. Do not fabricate a benchmark to unblock a language choice. When proof is absent, implement the behavior in Rust.

## Content, dependencies and platform boundaries

<a href="https://github.github.com/gfm/" title="Open Markdown official download or service page" aria-label="Open Markdown official download or service page" data-inta-brand="app-markdown"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/99f4edb33272b70febb8.png" alt="Markdown official page" width="18" height="18" valign="middle" /></a> [Markdown](https://github.github.com/gfm/), data records, HTML structure, CSS styles, images, manifests and declarative configuration are not alternate-language application implementations. Embedding scripts, template expressions with business behavior, shell programs or event handlers in those files does not turn executable logic into content.

Third-party software listed in the tool catalog and operating-system dependencies retain their upstream implementation languages. This rule governs <a href="https://herbertofury.github.io/INTA/" title="Open INTA official download or service page" aria-label="Open INTA official download or service page" data-inta-brand="app-inta"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA official page" width="18" height="18" valign="middle" /></a> [INTA](https://herbertofury.github.io/INTA/)'s owned application and utility code; it does not claim to rewrite every tool the technician can launch. Do not move first-party behavior into an unreviewed dependency merely to hide its language.

A compiler or pinned binding generator may emit platform interop from Rust source, such as browser bindings for a WebAssembly module. This is a generated artifact, not permission to author application functions in another language. Register exact artifact hashes, Rust source hashes, generator/version/command and reproducibility evidence. Keep selection, state, rendering decisions, navigation, event semantics and business rules in Rust. Handwritten JavaScript fallback applications and substantial native adapters require the same benchmark exception as any other non-Rust implementation.

## Existing code is unresolved migration work

The register starts with **zero approved performance exceptions**. The initial inventory identifies 58 non-Rust executable files in source revision `d3e901cc8eb5ccb5854eef58b73970bb935e5dbc`: the browser runtime and embedded scripts, <a href="https://www.python.org/" title="Open Python official download or service page" aria-label="Open Python official download or service page" data-inta-brand="app-python"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/e3ea97f62e4259df2d67.png" alt="Python official page" width="18" height="18" valign="middle" /></a> [Python](https://www.python.org/) build/validation/media/Rescue utilities, shell utilities, and the C# maintenance agent/installer. These entries are explicitly **unapproved legacy**, not grandfathered exceptions. Generated pages containing legacy JavaScript do not become Rust because their build command is launched from Rust.

Preserve the working behavior and user data while replacing these implementations. The migration guard checks that unapproved legacy files have not been silently changed or expanded, while the strict gate refuses full Rust compliance whenever any remains. An unchanged legacy entry can keep an incremental migration build reviewable; it cannot satisfy the owner's finished-state requirement. Do not refresh its recorded hash to excuse additional non-Rust work.

No native Desktop or Mobile source was present in the main checkout or fetched branches at the initial inventory. [The ecosystem acceptance gates](ecosystem-development.md) remain separate: a browser layout is not a tested native build, and a local bridge is not the complete Desktop client. Later native source and package evidence must name their actual paths and revisions.

## Policy checks

Run the policy tool from the repository root:

```sh
cargo run --locked -p inta-policy -- --base BASE_COMMIT
cargo run --locked -p inta-policy -- --strict
```

Command legend: [Cargo](https://doc.rust-lang.org/cargo/) builds and runs the [Rust policy checker](../crates/inta-policy/). Replace `BASE_COMMIT` with the actual reviewed parent or pull-request base commit. The first command enforces changes against the established inventory; the second checks the complete finished-state requirement.

Use `--json` for structured results and `--root PATH` to inspect an exact checkout. The checker reads tracked files and non-ignored new files, validates exact source/evidence paths and hashes, and rejects new or modified non-Rust code without a valid scoped exception. Passing a migration guard is reported separately from complete compliance. Reviewers still need to reproduce an exception and assess the equivalence of its actual workload; the checker cannot establish that a supplied benchmark describes real user work merely from a JSON record.

Every release claim must distinguish implemented Rust code, unresolved legacy code, generated bindings, approved benchmark exceptions, compiled packages and actual runtime/device tests. The four-product redesign is complete only when its required code and behavior gates are complete; writing this policy alone does not complete that migration.
