"""Verify upstream checksum attestations for the pinned restic and rclone releases."""

import argparse
import io
import re
import shutil
import subprocess
import tempfile
from pathlib import Path

import manage


def get(url):
    buf = io.BytesIO()
    manage.fetch(url, buf, 1024 * 1024)
    return buf.getvalue()


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--root", type=Path, required=True)
    p.add_argument("--gpg", type=Path, default=shutil.which("gpg"))
    args = p.parse_args()
    root = manage.safe_root(args.root)
    manage.identity(root)
    if not args.gpg:
        p.error("Pass a trusted GnuPG executable with --gpg")
    evidence = []
    for pin in [x for x in manage.PINS if x["id"] in ("restic", "rclone")]:
        name = pin["id"]
        output = manage.within(
            root,
            "92 - Manifests, Checksums & Signatures/" + name + "/" + pin["version"],
        )
        output.mkdir(parents=True, exist_ok=True)
        sums = get(manage.url(pin).rsplit("/", 1)[0] + "/SHA256SUMS")
        source = manage.source(root, pin, True)
        if name == "restic":
            key = get("https://restic.net/gpg-key-alex.asc")
            sig = get(manage.url(pin).rsplit("/", 1)[0] + "/SHA256SUMS.asc")
            trusted_fp = "CF8F18F2844575973F79D4E191A6868BD3F7A907"
        else:
            code = manage.git(
                source, "cat-file", "blob", pin["commit"] + ":cmd/selfupdate/verify.go"
            )
            key = (
                re.search(
                    rb"-----BEGIN PGP PUBLIC KEY BLOCK-----.*?-----END PGP PUBLIC KEY BLOCK-----",
                    code,
                    re.DOTALL,
                ).group()
                + b"\n"
            )
            sig = None
            trusted_fp = (
                None  # Exact armored key comes from the already pinned upstream commit.
            )
        with tempfile.TemporaryDirectory(dir=output) as tmp:
            work = Path(tmp)

            def gpg(*params, work=work):
                return subprocess.run(
                    [
                        str(args.gpg),
                        "--no-options",
                        "--homedir",
                        ".",
                        "--batch",
                        "--no-auto-key-retrieve",
                        *params,
                    ],
                    cwd=work,
                    check=False,
                    capture_output=True,
                    timeout=30,
                )

            (work / "key.asc").write_bytes(key)
            (work / "SHA256SUMS").write_bytes(sums)
            imported = gpg("--import", "key.asc")
            if imported.returncode:
                raise manage.Failure(
                    "Unable to import upstream public key: "
                    + imported.stderr.decode(errors="replace")[-500:]
                )
            args_verify = ["--status-fd", "1", "--verify"]
            if sig:
                (work / "SHA256SUMS.asc").write_bytes(sig)
                args_verify.append("SHA256SUMS.asc")
            proc = gpg(*args_verify, "SHA256SUMS")
            status = proc.stdout.decode("ascii", "replace")
            valid = re.search(
                r"^\[GNUPG:\] VALIDSIG ([A-F0-9 ]+[^\r\n]*)$", status, re.MULTILINE
            )
            if proc.returncode == 0 and valid:
                details = valid.group(1).split()
                fingerprint = details[-1]
                if trusted_fp and fingerprint != trusted_fp:
                    raise manage.Failure("Unexpected publisher fingerprint")
                # RFC 4880 hash algorithm 2 is SHA-1, retained solely as a legacy attestation.
                signature = (
                    "verified" if details[7] != "2" else "legacy-sha1-insufficient"
                )
            else:
                fingerprint = trusted_fp
                signature = "unverified"
            if name == "restic" and signature != "verified":
                raise manage.Failure(
                    "The restic checksum lacks a verified current publisher signature"
                )
            hit = re.search(
                rb"^([0-9a-fA-F]{64})\s+\*?"
                + re.escape(pin["asset"].encode())
                + rb"\s*$",
                sums,
                re.MULTILINE,
            )
            if not hit or hit.group(1).decode() != pin["assetSha256"]:
                raise manage.Failure("Publisher digest differs from release lock")
        for n, blob in [
            ("key.asc", key),
            ("SHA256SUMS", sums),
            ("SHA256SUMS.asc", sig),
        ]:
            if blob:
                manage.write(
                    manage.within(root, (output.relative_to(root) / n).as_posix()), blob
                )
        evidence.append(
            {
                "id": name,
                "version": pin["version"],
                "sha256": pin["assetSha256"],
                "publisherChecksum": "matches",
                "signature": signature,
                "fingerprint": fingerprint,
                "checkedAt": manage.date(),
            }
        )
    manage.dump(
        manage.within(
            root, "92 - Manifests, Checksums & Signatures/publisher-checks.json"
        ),
        evidence,
    )
    print(evidence)


if __name__ == "__main__":
    main()
