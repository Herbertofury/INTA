# Review the Rescue scanner before a release

Boot or ISO checksum checks do not establish scanner safety. Both Rescue publication paths must read the actual Debian scanner package metadata from the same SHA-256-verified ISO they will publish. The test opens only two metadata files from the embedded SquashFS without mounting the image, booting it, executing the scanner, or reading a file supplied by a technician.

If the Debian stable package remains affected in the [Debian security tracker](https://security-tracker.debian.org/tracker/source-package/clamav), retain the ISO as a build candidate. A fresh candidate, release-package reconstruction test, mirror update, or a review for a different suite is not a substitute for the exact stable package review. Do not copy from unstable or weaken protections just to pass.

Only after the maintainer independently confirms that Debian stable supplies the fixed, signed packages, add `.github/rescue-scanner-review.json`. Include suite `trixie`, architecture `amd64`, source `clamav`, UTC `reviewed_on`, the Debian tracker and authoritative dated package provenance under `evidence`, and both exact installed package records under `packages`. Each record includes package, architecture, source, source version, and binary version. An absent or stale review, changed exact tuple, or the known affected package branch must fail the gate. No review is provided with this gate, and no candidate has been declared fixed.

After proving the gate, continue the existing BIOS/UEFI, recovery, checksum, and package reconstruction acceptance. Preserve earlier public releases for audit; do not replace one or announce an update until the current candidate qualifies.

## Historical download status

The [27 November Rescue image](https://github.com/Herbertofury/INTA/releases/tag/rescue-45951274984.1) is a historical build. Its boot and reconstructed checksum checks passed. Do not rely on its bundled scanner for untrusted files. The [publication job](https://github.com/Herbertofury/INTA/actions/runs/45951274984/job/140347454599) recorded the affected stable package version and the checksum listed on the release. A qualified replacement requires fixed signed stable packages and a fresh review of the exact ISO.

[Release history](https://github.com/Herbertofury/INTA/releases) remains available for audit. The Media Creator may report a missing checksum when GitHub’s latest release is an unrelated toolbox release. This does not indicate a damaged local Rescue image. Do not fall back to an older scanner image to work around that message.
