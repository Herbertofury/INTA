# INTA Mobile: native offline repair job preview

Each Android repair job has a checklist, a durable stopwatch, and an explicit
complete action that freezes recorded time without marking any unchecked steps
as done. The timer needs no foreground service. Closing the app on the same
phone boot retains the running interval. If the phone itself restarts, the
app pauses and retains only the last saved duration rather than guessing time.
A restart of the serviced PC does not modify this independent phone timer.

Install the [signed Android APK](https://github.com/Herbertofury/INTA/releases/tag/mobile-0.2-preview.2)
on Android 8.0 or later. **Switch or create job** opens named local work orders. Each
keeps its own stopwatch, checklist, invoice and repair draft. An existing
installation's original record stays accessible under **Repair job**. Use **Review
handoff export** to save a completed job before uninstalling. **Receive PC
diagnostics** opens an encrypted Desktop handoff through the system file picker.
Create the one-time pairing code on the phone, enter it on Desktop, then open the
transferred file on the phone to review its observations.
The application works offline and requests no network or broad storage permission.

Build on Linux with JDK 21, `zip`, a current stable Rust toolchain, Android API 36,
and Android Build Tools 36.0.0. Install the four Rust Android targets once per toolchain.

```sh
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
bash mobile/android/build-preview.sh \
  "$ANDROID_HOME/platforms/android-36/android.jar" \
  "$ANDROID_HOME/build-tools/36.0.0" \
  path-to-new-candidate
```

Both screens offer five local **Appearance** profiles. The default Full Prism
matrix responds to touch and stops drawing when the ripple settles. Deep Focus
and Silent Eco are static. Classic uses a solid background. System high contrast,
reduced animations and battery saver take precedence. The native Android controls
remain available if the optical library cannot load.

The script runs deterministic state and persistence tests and produces an
**unsigned** APK. Use an isolated test key for emulator testing; don't publish
that key or ship this as a production release. The stopwatch uses [Android's
monotonic elapsed time](https://developer.android.com/reference/android/os/SystemClock)
and checks [the system boot count](https://developer.android.com/reference/android/provider/Settings.Global#BOOT_COUNT).
A missing boot count leaves checklist operations available but the timer disabled.

The live job and locally generated clock-owner marker are kept in Android's
[no-backup directory](https://developer.android.com/reference/android/content/Context#getNoBackupFilesDir()).
Copying only a job record onto another installation never carries a running
interval: even identical phone boot counters leave it paused at its last
measured checkpoint. This was verified with an independent-store fixture,
not a device-transfer service.

## Draft a reconnect plan offline

Use **Draft a repair plan** to add typed steps, change their order or remove
them. The list is saved separately from the current timer, survives a cold
launch, and stays on this phone. A draft is never a command, approved job,
or remote executable. Pairing, exact tool selection, signing and dispatch
must be completed separately before INTA can run it on a PC.

A completed local job shows an offline USD **Invoice draft**. It uses the
recorded duration, or explicitly entered whole billable minutes; original
time stays untouched. The technician selects residential or business and
enters a rate. The subtotal previews immediately, rounds once to cents,
and saves only on **Save invoice draft**. It survives process restart.
There is no guessed rate, tax, invoice numbering, payment, sending, or
sync in this preview.

## Export a completed job

Tap **Review handoff export** after completion. Read the exact text, then use
**Choose save location** to save a UTF-8 text file through Android's document
picker. The export contains recorded time, the checklist, and the last saved
invoice draft when one exists. Unsaved billing edits and the separate repair
plan stay out of the file. The selected document provider controls where the
export is stored. Select an on-device location for a local handoff.

## Package an accepted Android build

The native emulator workflow records the exact unsigned APK checksum. A
release operator can sign that APK on the release host with the approved
Android application key. Use the checksum from the accepted workflow, the
expected SHA-256 of the app-signing certificate, and the tested Git commit.
Set `INTA_ANDROID_STORE_PASS` and `INTA_ANDROID_KEY_PASS` from the release
host's secret store before running:

```sh
bash mobile/android/package-release.sh \
  "$ANDROID_HOME/build-tools/36.0.0" inta-mobile-unsigned.apk \
  "$APK_SHA256" "$SIGNER_SHA256" "$KEYSTORE" "$KEY_ALIAS" "$TESTED_REVISION" new-release
```

The output contains the signed APK, its SHA-256 and the signature verification
receipt. The emulator uses a disposable key to exercise packaging and install.
Public updates require the approved persistent app signing key.

On the Windows release host, `owner-sign.ps1 -Action Initialize` creates or
checks the dedicated release identity. Its encrypted keystore and Windows
CurrentUser DPAPI-protected password live together under the owner's local
application data. Keep an owner-controlled recoverable backup of the Windows
identity and signing material for future updates. `-Action Identity` prints
the public SHA-256 certificate fingerprint. `-Action Sign` takes the tested
unsigned APK, expected SHA-256, source revision, apksigner Java classpath and
a new output directory. The public release includes its signature receipt.
