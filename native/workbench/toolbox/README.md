# INTA managed toolbox

The desktop and CLI open a local toolbox by its versioned UUID marker. The managed package set overlays the existing IDs in the [canonical catalog](../data/tools.json). The [deployment pins](pins.json) bind official release URLs, source revisions, archive size, archive SHA-256 and the hash of the one supported executable. Rust compiles these pins into the broker. An editable receipt or HTML index cannot authorize another executable or new arguments.

## Technician use

Open **INTA → Toolbox**, choose a marked folder, verify a package, and run its version probe. Select your technician drive’s marked `INTA Toolbox` folder. Open `00 - START HERE/INTA Toolbox Index.html` for the offline package directory. Each version carries the observed fixture results, license text and a machine-readable receipt. Keep case evidence in the approved private case system.

The archive integrity operation accepts one local single-volume, unencrypted `.7z` archive from 32 bytes through 256 MiB. It reads an ephemeral snapshot using the pinned reduced 7-Zip engine in test mode. The worker is limited to one process, 512 MiB of committed memory and 30 seconds. The report contains size, checksum and result. The snapshot lives in the current user's temporary directory for the operation and normal cleanup removes it. Abrupt process termination may leave a temporary copy. Testing integrity checks the archive's data streams. It cannot establish the safety of archived content.

## Verified single-file backup

Choose **Create verified .7z backup** in the Toolbox view. Select a regular local file up to 64 MiB and a new destination ending in `.7z`. The archive holds the chosen file under its original basename. The broker snapshots the input, creates the archive with the pinned 7-Zip engine, restores it privately, and checks the restored length and SHA-256 against the held source file. It copies the verified archive through a same-volume staging file, validates the final bytes and publishes by handle without replacing an existing file. The receipt includes the input and archive digests. The maximum published archive is 65 MiB. The CLI requires explicit approval.

```powershell
inta toolbox backup 'I:\Example\service-note.txt' 'I:\Example\new-backup.7z' --approve
inta toolbox archive 'I:\Example\new-backup.7z'
```

This creates an unencrypted content backup. Choose a destination with suitable access controls. The operation rejects encrypted source files, additional named streams, reparse points and volumes where it cannot verify the source stream layout. Filesystem ACLs, timestamps, folder trees and metadata outside the default data stream require a filesystem-aware backup. The worker uses temporary local bytes during the round trip. An abrupt shutdown may leave temporary working data. A file that is open for modification, or whose directory cannot be pinned, produces a safe failure.

## Maintainer commands

From the repository root on Windows x64 with Python 3.11+, Git and network access, use an empty or previously marked destination. The PowerShell variable below can point to another drive or a renamed folder.

```powershell
$kit = 'I:\INTA Toolbox'
python toolbox/manage.py init --root $kit
python toolbox/manage.py acquire --root $kit
python toolbox/manage.py reference --root $kit
python toolbox/manage.py verify --offline --root $kit
python toolbox/manage.py check-updates --root $kit
python toolbox/attestation.py --root $kit --gpg 'C:\Program Files\Git\usr\bin\gpg.exe'
python toolbox/sbom.py --root $kit
```

`acquire --tool restic` selects a single reviewed pin. The complete history and pinned source checkout stay in `90 - Upstream Source Repositories`, with release archives in a content-addressed cache. Add `--offline` to acquire again using the source and bytes already cached. Unverified bytes never enter the active pointer. A repeat acquisition checks existing versions and preserves technician notes. Reacquisition does not silently replace broken or edited active versions. An operator must review conflicting files. Index and HTML reports can be regenerated with `index`. The provenance SBOM command requires a Go toolchain supporting `go version -m -json`.

Update checks compare four release tags on demand. Review a new upstream pin, its upstream signature/checksum, source commit, licenses and tested binary digest before updating source and rebuilding the broker. Prior version folders are retained. Rolling the broker back to a prior pin requires reindexing the matching retained version. The Desktop release provides the broker. Third-party downloads use their respective upstream terms.

## Move and reconnect

Rename the root freely. Move complete version directories with their `tool.json`, `PORTABLE`, `LICENSES` and `TEST-RESULTS` together. Run `python toolbox/manage.py reindex --root $kit` after rearranging categories. Reindex validates the compiled set's hashes and stops on duplicate active versions. It skips source clones, caches and the logs area. Path checks reject traversal, alternate streams and reparse points.

For a verified copy, close users of the source toolbox and use the script below. The destination parent must exist on a trusted local volume. The script checks free space, copies all regular files and directories, validates the complete source and copy by SHA-256, and validates active executable pins before renaming the staged directory. It preserves the source and requires explicit selection of the new root. An interrupted attempt can resume into the same destination as long as its receipt matches. Additional unique data appearing in staging requires manual review. The copy verifies regular file contents and directory membership. A linked file or a Windows named stream stops migration for review. Destination permissions inherit from the new storage location. Use a filesystem-aware backup to preserve custom ACLs and filesystem-specific metadata.

```powershell
python toolbox/relocate.py $kit 'X:\Technician\@toolbox'
inta toolbox use 'X:\Technician\@toolbox'
```

Bounded discovery inspects selected anchors and their first 512 direct entries. A deeper custom location can always be selected explicitly. Two copies may share a UUID. The operator selects the authoritative copy.

## Verification

```powershell
python -m unittest discover -s toolbox/tests -v
cargo test -p inta-toolbox
$env:INTA_TEST_TOOLBOX = $kit
cargo test -p inta-toolbox -- --include-ignored
```

The [verification record](validation.md) gives the actual workstation and fixture coverage. The [third-party report](third-party.md) records selection, attribution and distribution boundaries.
