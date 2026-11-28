#!/usr/bin/env bash
set -euo pipefail
if (( $# != 8 )); then
  printf '%s\n' 'Usage: package-release.sh BUILD_TOOLS VERIFIED_UNSIGNED_APK APK_SHA256 SIGNER_SHA256 KEYSTORE KEY_ALIAS SOURCE_REVISION NEW_OUTPUT_DIRECTORY' >&2
  exit 2
fi
tools=$(realpath -- "$1")
apk=$(realpath -- "$2")
expected=${3,,}
signer=${4,,}
keystore=$(realpath -- "$5")
alias=$6
revision=$7
[[ "$expected" =~ ^[a-f0-9]{64}$ && "$signer" =~ ^[a-f0-9]{64}$ && "$revision" =~ ^[a-f0-9]{40}$ ]]
[[ "$alias" =~ ^[A-Za-z0-9._-]+$ && -f "$apk" && -f "$keystore" ]]
: "${INTA_ANDROID_STORE_PASS:?Set the owner keystore password in the environment}"
: "${INTA_ANDROID_KEY_PASS:?Set the owner key password in the environment}"
actual=$(sha256sum < "$apk")
[[ "${actual%% *}" == "$expected" ]] || { echo 'Unsigned Android artifact differs from the accepted digest.' >&2; exit 1; }
badging=$("$tools/aapt2" dump badging "$apk")
grep -Eq "^package: name='com[.]inta[.]mobile' versionCode='[0-9]+'" <<< "$badging" || {
  echo 'Unexpected Android package identifier or version.' >&2; exit 1;
}
grep -Eq "^(minSdkVersion|sdkVersion):'26'$" <<< "$badging" || {
  echo 'Unexpected Android minimum SDK.' >&2; exit 1;
}
grep -Fxq "targetSdkVersion:'36'" <<< "$badging" || {
  echo 'Unexpected Android target SDK.' >&2; exit 1;
}
"$tools/zipalign" -c -p 4 "$apk"
parent=$(realpath -- "$(dirname -- "$8")")
out="$parent/$(basename -- "$8")"
[[ ! -e "$out" && ! -L "$out" ]]
scratch=$(mktemp -d "$parent/.inta-android-release.XXXXXXXX")
cleanup() {
  rm -f -- "$scratch/INTA-Mobile-Android.apk" "$scratch/SHA256SUMS" "$scratch/verification.txt"
  rmdir -- "$scratch" 2>/dev/null || true
}
trap cleanup EXIT
candidate="$scratch/INTA-Mobile-Android.apk"
"$tools/apksigner" sign --v1-signing-enabled false --v2-signing-enabled true \
  --v3-signing-enabled true --v4-signing-enabled false --ks "$keystore" --ks-key-alias "$alias" \
  --ks-pass env:INTA_ANDROID_STORE_PASS --key-pass env:INTA_ANDROID_KEY_PASS --out "$candidate" "$apk"
info=$("$tools/apksigner" verify --verbose --print-certs --min-sdk-version 26 "$candidate")
grep -Fxq 'Number of signers: 1' <<< "$info" || { echo 'Expected one Android signer.' >&2; exit 1; }
grep -Fxq 'Verified using v2 scheme (APK Signature Scheme v2): true' <<< "$info" || {
  echo 'The Android v2 signature did not verify.' >&2; exit 1;
}
grep -Fxq "Signer #1 certificate SHA-256 digest: $signer" <<< "$info" || {
  echo 'Android signer differs from the approved certificate fingerprint.' >&2; exit 1;
}
"$tools/zipalign" -c -p 4 "$candidate"
{
  printf 'Source revision: %s\nVerified input SHA-256: %s\n' "$revision" "$expected"
  printf '%s\n' "$info"
} > "$scratch/verification.txt"
(cd "$scratch"; sha256sum INTA-Mobile-Android.apk verification.txt > SHA256SUMS)
# Reserve the destination without clobbering another run.
mkdir -- "$out"
mv -- "$scratch/INTA-Mobile-Android.apk" "$scratch/verification.txt" "$out/"
mv -- "$scratch/SHA256SUMS" "$out/"
