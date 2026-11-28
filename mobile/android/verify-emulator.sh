#!/usr/bin/env bash
set -euo pipefail
if (( $# != 5 )); then
  printf '%s\n' 'Usage: bash verify-emulator.sh ANDROID_JAR BUILD_TOOLS ADB TEST_AVD NEW_OUTPUT_DIRECTORY' >&2
  exit 2
fi
root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
jar=$(realpath -- "$1")
tools=$(realpath -- "$2")
adb=$(realpath -- "$3")
avd=$4
[[ "$avd" =~ ^inta-test-[a-zA-Z0-9_-]+$ ]]
mkdir -- "$5"
out=$(cd -- "$5" && pwd)
scratch=$(mktemp -d "$out/.verify.XXXXXX")
cleanup() {
  if [[ -n "${owned_pid:-}" ]]; then
    timeout 12 "$adb" -s emulator-5584 logcat -d -b crash > "$out/android-crash.log" 2>&1 || true
    kill "$owned_pid" 2>/dev/null || true
    wait "$owned_pid" 2>/dev/null || true
  fi
  rm -rf -- "$scratch"
}
trap cleanup EXIT
serial=emulator-5584
"$adb" start-server
# A populated test AVD is not valid evidence. Run only on a fresh CI-owned emulator.
if "$adb" devices | awk -v serial="$serial" '$1 == serial { matched=1 } END { exit !matched }'; then
  printf '%s\n' 'The test emulator serial is already in use.' >&2
  exit 2
fi
: "${ANDROID_HOME:?Android SDK required}"
mode=${INTA_ANDROID_EMULATOR_ACCEL:-on}
[[ "$mode" == on || "$mode" == off ]]
boot_seconds=180
if [[ "$mode" == off ]]; then
  # Official software emulation for test builders without access to /dev/kvm.
  export ANDROID_I_WANT_MY_TCG=yes
  boot_seconds=540
fi
"$ANDROID_HOME/emulator/emulator" -avd "$avd" -port 5584 -accel "$mode" -no-window -no-audio \
  -no-snapshot -no-boot-anim -camera-back none -camera-front none -gpu swiftshader_indirect \
  >"$out/emulator.log" 2>&1 &
owned_pid=$!
bash "$root/build-preview.sh" "$jar" "$tools" "$scratch/app" > "$out/build.log"
mkdir "$scratch"/{classes,dex}
javac --release 11 -Xlint:all -Werror -cp "$jar" -d "$scratch/classes" \
  "$root"/instrumentation/com/inta/mobile/test/OfflineJobAcceptance.java
mapfile -d '' -t classes < <(find "$scratch/classes" -name '*.class' -print0 | sort -z)
"$tools/d8" --release --min-api 26 --lib "$jar" --output "$scratch/dex" "${classes[@]}"
"$tools/aapt2" link --manifest "$root/instrumentation/AndroidManifest.xml" -I "$jar" -o "$scratch/test-unsigned.apk"
(cd "$scratch/dex"; zip -q -X -j ../test-unsigned.apk classes.dex)
"$tools/zipalign" -f -p 4 "$scratch/test-unsigned.apk" "$scratch/test-aligned.apk"
keytool -genkeypair -noprompt -alias ci -keystore "$scratch/ci-only.p12" -storepass local-ci-key \
  -keypass local-ci-key -keyalg RSA -keysize 2048 -validity 1 -dname 'CN=Disposable INTA CI' > "$scratch/keygen.log" 2>&1
unsigned="$scratch/app/inta-mobile-unsigned.apk"
expected=$(sha256sum < "$unsigned")
keytool -exportcert -alias ci -keystore "$scratch/ci-only.p12" -storepass local-ci-key -file "$scratch/ci-cert.der"
signer=$(sha256sum < "$scratch/ci-cert.der")
revision=$(git -C "$root" rev-parse HEAD)
bad_digest=$(printf '%064d' 0)
for check in input signer; do
  apk_pin=${expected%% *}
  signer_pin=${signer%% *}
  if [[ "$check" == input ]]; then apk_pin=$bad_digest; else signer_pin=$bad_digest; fi
  if INTA_ANDROID_STORE_PASS=local-ci-key INTA_ANDROID_KEY_PASS=local-ci-key \
    bash "$root/package-release.sh" "$tools" "$unsigned" "$apk_pin" "$signer_pin" \
      "$scratch/ci-only.p12" ci "$revision" "$scratch/invalid-$check"; then
    echo "Android packager accepted the wrong $check digest" >&2
    exit 2
  fi
  [[ ! -e "$scratch/invalid-$check" ]]
done
INTA_ANDROID_STORE_PASS=local-ci-key INTA_ANDROID_KEY_PASS=local-ci-key \
  bash "$root/package-release.sh" "$tools" "$unsigned" "${expected%% *}" "${signer%% *}" \
    "$scratch/ci-only.p12" ci "$revision" "$scratch/signed"
"$tools/apksigner" sign --v4-signing-enabled false --ks "$scratch/ci-only.p12" --ks-pass pass:local-ci-key \
  --out "$scratch/test.apk" "$scratch/test-aligned.apk"
"$tools/apksigner" verify "$scratch/test.apk"
deadline=$((SECONDS + boot_seconds))
while (( SECONDS < deadline )); do
  kill -0 "$owned_pid" || { tail -30 "$out/emulator.log"; exit 2; }
  booted=$(timeout 5 "$adb" -s "$serial" shell getprop sys.boot_completed 2>/dev/null || true)
  [[ "$booted" == 1 ]] && break
  sleep 2
done
if [[ "${booted:-}" != 1 ]]; then
  printf 'Android boot did not complete within %s seconds (acceleration %s).\n' "$boot_seconds" "$mode" >&2
  "$adb" devices -l
  tail -50 "$out/emulator.log"
  exit 2
fi
"$adb" -s "$serial" install "$scratch/signed/INTA-Mobile-Android.apk"
"$adb" -s "$serial" install "$scratch/test.apk"
test_id=com.inta.mobile.test/.OfflineJobAcceptance
for stage in optics start resume verify-completed draft draft-verify invoice invoice-verify handoff diagnostic-import jobs-create jobs-switch jobs-return; do
  "$adb" -s "$serial" shell am force-stop com.inta.mobile
  if [[ "$stage" == resume ]]; then sleep 2; fi
  timeout 80 "$adb" -s "$serial" shell am instrument -w -e stage "$stage" "$test_id" | tee "$out/$stage.log"
  grep -Fxq "INTA_ANDROID_ACCEPTANCE_OK $stage" "$out/$stage.log"
done
"$adb" -s "$serial" shell am start -W -n com.inta.mobile/.JobActivity
sleep 1
"$adb" -s "$serial" exec-out screencap -p > "$out/inta-job-completed.png"
cp "$scratch/app/inta-mobile-unsigned.apk" "$out/"
cp "$scratch/signed/verification.txt" "$out/disposable-signing-proof.txt"
(cd "$out"; sha256sum inta-mobile-unsigned.apk > SHA256SUMS)
if [[ -n "${INTA_OWNER_SIGNED_APK:-}" ]]; then
  signed=$INTA_OWNER_SIGNED_APK
  proof="$out/owner-install.txt"
  "$tools/apksigner" verify --verbose --print-certs "$signed" > "$proof"
  grep -Fxq 'Signer #1 certificate SHA-256 digest: eb1e89cf61e1c5e3c1ec8f2d8e46b91a6a5a66feaf3283189ddec5a224d02c0d' "$proof"
  "$tools/zipalign" -c -P 16 4 "$signed"
  "$tools/aapt2" dump badging "$signed" >> "$proof"
  grep -Eq "^package: name='com[.]inta[.]mobile' versionCode='[0-9]+'" "$proof"
  grep -Eq "^(minSdkVersion|sdkVersion):'26'$" "$proof"
  grep -Fxq "targetSdkVersion:'36'" "$proof"
  "$adb" -s "$serial" uninstall com.inta.mobile.test
  "$adb" -s "$serial" uninstall com.inta.mobile
  "$adb" -s "$serial" install "$signed"
  "$adb" -s "$serial" shell am start -W -n com.inta.mobile/.JobActivity >> "$proof"
  "$adb" -s "$serial" shell uiautomator dump /sdcard/inta-acceptance.xml
  "$adb" -s "$serial" exec-out cat /sdcard/inta-acceptance.xml > "$scratch/owner-ui.xml"
  grep -Fq 'text="Repair job"' "$scratch/owner-ui.xml"
  "$adb" -s "$serial" install -r "$signed"
  sha256sum "$signed" >> "$proof"
  printf '%s\n' 'Owner-signed APK installed, opened Repair job, and reinstalled with the persistent identity.' >> "$proof"
fi
