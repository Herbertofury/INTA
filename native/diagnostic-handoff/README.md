# Attended Windows-to-Android diagnostic handoff

This helper exports one authenticated memory and power snapshot for a phone that has issued a ten-minute pairing. The Android app reads the selected file through its system document picker. Desktop and phone can operate without internet or a local network. The producer and receiver do not start a service or take control of either device.

## Send a snapshot

1. On the phone, open **Repair job → Receive PC diagnostics → Create one-time pairing** and confirm. Keep the pairing screen open. Read the code directly to the operator at the Windows PC. Handle the file and the code separately.
2. On Windows, run the preview below to review the six outgoing fields. Run the approved export with a new, absolute filename on a local volume. Type the code at the hidden console prompt. A desktop client can pipe the code to standard input without placing it in command-line arguments, shell history or application logs.
3. Move the resulting **.intadiag** file to the phone through a transfer method appropriate for the case. Select **Choose encrypted report** and pick the local file. A successful import displays capture time, available and total physical memory, memory load, AC power state and battery percentage. The phone consumes the pairing. The mobile app keeps the displayed snapshot in memory.
4. **Revoke pairing** immediately refuses further imports with that key on the phone. Expiry, closing the receiving screen or loss of the app process also requires a new pairing. The file owner controls retention of transferred ciphertext and any record the operator makes after reading it.

~~~powershell
.\inta-diagnostic-handoff.exe preview
.\inta-diagnostic-handoff.exe export 'I:\Approved transfers\visit-01.intadiag' --approve
~~~

The Windows producer calls **GlobalMemoryStatusEx** and **GetSystemPowerStatus** locally. Its receipt contains capture and expiry times, field names, byte count and the ciphertext SHA-256. It publishes a new file through no-clobber staging. The mobile picker requests local documents and accepts exactly 87 bytes. The receiver has a bounded, cancellable read and releases its document handle and temporary URI grant. Neither application requests customer credentials or sends a command, file path, application list, device hostname, serial number or network address in the report.

## Pairing and trust

The phone issues a fresh 128-bit cryptographically random symmetric secret. Its pairing code contains the protocol version, issuance time in Unix seconds and the secret in hexadecimal. A new pairing replaces the previous one. Workstation authenticates the fixed binary header and encrypts the six fields with AES-128-GCM, a random 96-bit nonce and a 128-bit tag. The receiver compares the session identifier and verifies the tag before interpreting any field. The receiver enforces both wall-clock and elapsed-time expiry. One successful import consumes the key. Invalid reports leave a valid pairing available for another attempt. [RustCrypto AES-GCM](https://docs.rs/aes-gcm/0.10.3/aes_gcm/) · [Android GCM parameters](https://developer.android.com/reference/javax/crypto/spec/GCMParameterSpec)

Possession of the pairing secret authorizes a producer for this one phone session. Keep the pairing code private. Revocation prevents a later import by the active mobile reader. It cannot recall information already displayed or protect a captured file from somebody who already knows its secret. This release implements attended snapshot transfer. Persistent device enrollment, live streaming, job synchronization, screen control and Internet remote management remain separate capabilities.

### Version 1 wire layout

All integers are unsigned, big endian. Bytes 0–7 contain **INTADG01**, 8–15 contain the issued time, 16–31 contain the first 16 bytes of SHA-256 over the ASCII domain **INTA-DIAGNOSTIC-SESSION-1** followed by the secret, and 32–43 hold the random nonce. These 44 header bytes are GCM additional authenticated data. Bytes 44–70 hold ciphertext and 71–86 hold the authentication tag. The plaintext consists of three 64-bit values for capture time, total memory and available memory, followed by three one-byte values for load, AC state and battery percentage. AC state uses 0 for offline, 1 for online and 255 for unknown. Battery percentage uses 255 for unknown.

## Build and prove

~~~powershell
cargo build --release --offline --locked
cargo clippy --offline --locked --all-targets -- -D warnings
python tests/interop.py .\target\release\inta-diagnostic-handoff.exe 'I:\INTA\QA\new-handoff-receipt.json'
python ..\capabilities\package_notices.py .\target\release\inta-diagnostic-handoff.exe 'I:\INTA\QA\new-handoff-notices' --project .
~~~

The interoperability test runs the real Rust Windows producer and the production Android cryptographic reader in separate Windows and OpenJDK processes. It verifies the observed physical memory independently, rejects modified headers and ciphertext, checks expiry on both clocks, and exercises revocation, replay, wrong pairing, failed approval and pre-existing destination handling. The Android emulator harness has a separate **diagnostic-import** stage that exercises the Activity, Android cryptographic provider, one-use state and document-picker intent against a local content-provider fixture. Consult that build's emulator receipt for its result. [Windows/JVM receipt](evidence/windows-android-jvm.json) · [Emulator harness](../../mobile/android/verify-emulator.sh) · [Mobile receiver](../../mobile/android/src/com/inta/mobile/DiagnosticHandoffActivity.java)
