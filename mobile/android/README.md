# INTA Mobile: native offline repair job preview

One Android job with a repair checklist, a durable stopwatch, and an explicit
complete action that freezes recorded time without marking any unchecked steps
as done. The timer needs no foreground service. Closing the app on the same
phone boot retains the running interval. If the phone itself restarts, the
app pauses and retains only the last saved duration rather than guessing time.
A restart of the serviced PC does not modify this independent phone timer.

This does not sign in, sync or upload records, control another device, produce
an invoice, or replace the site, Desktop, or Rescue. It creates one real native
Activity, not a WebView. No network, notification or storage permission is
requested. The job record is private application data, not a portable folder
or a cross-device backup. Uninstalling the preview removes the local record.
This first current-job view does not yet create, index, or reopen past jobs.

Build in the cloud or on a Linux builder with JDK 21, `zip`, Android API 36,
and Android Build Tools 36.0.0 from the official Android SDK:

```sh
bash mobile/android/build-preview.sh \
  "$ANDROID_HOME/platforms/android-36/android.jar" \
  "$ANDROID_HOME/build-tools/36.0.0" \
  path-to-new-candidate
```

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
not a device-transfer service. No automatic or explicit job export exists yet.
