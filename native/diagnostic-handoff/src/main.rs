use aes_gcm::{
    Aes128Gcm, KeyInit,
    aead::{Aead, Payload},
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    io::{BufRead, Write},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};
use zeroize::Zeroizing;

#[derive(Debug)]
struct Error {
    code: &'static str,
    message: String,
}
type Result<T> = std::result::Result<T, Error>;
fn err(code: &'static str, message: impl Into<String>) -> Error {
    Error {
        code,
        message: message.into(),
    }
}
fn invalid(message: impl Into<String>) -> Error {
    err("invalid-input", message)
}
fn expect(condition: bool, message: &'static str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(invalid(message))
    }
}
// Reuse the Workstation's local-volume and held-directory validation for artifact publication.
#[allow(dead_code)]
mod files {
    include!("../../capabilities/src/files.rs");
    #[cfg(windows)]
    pub fn validated_path(path: &Path) -> Result<PathBuf> {
        host::local(path)
    }
    #[cfg(not(windows))]
    pub fn validated_path(_: &Path) -> Result<PathBuf> {
        Err(err("unavailable", "Windows is required."))
    }
}
const MAGIC: &[u8; 8] = b"INTADG01";
const LIFETIME: u64 = 600;
const FIELDS: [&str; 6] = [
    "capturedAtUnix",
    "totalPhysicalBytes",
    "availablePhysicalBytes",
    "memoryLoadPercent",
    "acLine",
    "batteryPercent",
];
fn now() -> Result<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .map_err(|_| err("clock", "System time is unavailable."))
}
struct Pair {
    issued: u64,
    key: Zeroizing<[u8; 16]>,
}
impl Pair {
    fn parse(code: &str, at: u64) -> Result<Self> {
        let mut parts = code.split('.');
        expect(
            parts.next() == Some("INTA1"),
            "Invalid Android pairing code.",
        )?;
        let issued = parts
            .next()
            .and_then(|s| s.parse::<u64>().ok())
            .ok_or_else(|| invalid("Invalid Android pairing code."))?;
        let secret = parts
            .next()
            .ok_or_else(|| invalid("Invalid Android pairing code."))?;
        expect(
            parts.next().is_none()
                && secret.len() == 32
                && secret.bytes().all(|c| c.is_ascii_hexdigit()),
            "Invalid Android pairing code.",
        )?;
        expect(
            issued <= at.saturating_add(30) && at < issued.saturating_add(LIFETIME),
            "The pairing has expired or the device clocks differ.",
        )?;
        let mut key = Zeroizing::new([0; 16]);
        hex::decode_to_slice(secret, &mut key[..])
            .map_err(|_| invalid("Invalid Android pairing code."))?;
        Ok(Self { issued, key })
    }
}
fn encrypt(pair: &Pair, at: u64, body: &[u8]) -> Result<Vec<u8>> {
    let mut header = Vec::with_capacity(44);
    header.extend_from_slice(MAGIC);
    header.extend_from_slice(&pair.issued.to_be_bytes());
    let mut tag = Sha256::new();
    tag.update(b"INTA-DIAGNOSTIC-SESSION-1");
    tag.update(&pair.key[..]);
    header.extend_from_slice(&tag.finalize()[..16]);
    let mut nonce = [0; 12];
    getrandom::getrandom(&mut nonce)
        .map_err(|_| err("unavailable", "A secure random nonce was unavailable."))?;
    header.extend_from_slice(&nonce);
    let mut plain = Zeroizing::new(Vec::with_capacity(27));
    plain.extend_from_slice(&at.to_be_bytes());
    plain.extend_from_slice(body);
    let cipher =
        Aes128Gcm::new_from_slice(&pair.key[..]).map_err(|_| invalid("Invalid pairing key."))?;
    let ciphertext = cipher
        .encrypt(
            (&nonce).into(),
            Payload {
                msg: &plain,
                aad: &header,
            },
        )
        .map_err(|_| err("unavailable", "Could not seal this snapshot."))?;
    header.extend_from_slice(&ciphertext);
    Ok(header)
}
#[cfg(windows)]
fn observe() -> Result<Vec<u8>> {
    use windows_sys::Win32::System::{
        Power::{GetSystemPowerStatus, SYSTEM_POWER_STATUS},
        SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX},
    };
    let mut mem = MEMORYSTATUSEX {
        dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as u32,
        ..Default::default()
    };
    let mut power = SYSTEM_POWER_STATUS::default();
    if unsafe { GlobalMemoryStatusEx(&mut mem) } == 0
        || unsafe { GetSystemPowerStatus(&mut power) } == 0
    {
        return Err(err(
            "unavailable",
            "Windows did not provide this diagnostic snapshot.",
        ));
    }
    expect(
        mem.ullTotalPhys > 0 && mem.ullAvailPhys <= mem.ullTotalPhys && mem.dwMemoryLoad <= 100,
        "The Windows memory response was incomplete.",
    )?;
    expect(
        matches!(power.ACLineStatus, 0 | 1 | 255)
            && (power.BatteryLifePercent <= 100 || power.BatteryLifePercent == 255),
        "The Windows power response was incomplete.",
    )?;
    let mut data = Vec::new();
    data.extend_from_slice(&mem.ullTotalPhys.to_be_bytes());
    data.extend_from_slice(&mem.ullAvailPhys.to_be_bytes());
    data.extend([
        mem.dwMemoryLoad as u8,
        power.ACLineStatus,
        power.BatteryLifePercent,
    ]);
    Ok(data)
}
#[cfg(not(windows))]
fn observe() -> Result<Vec<u8>> {
    Err(err(
        "unavailable",
        "Windows is required to collect the diagnostic snapshot.",
    ))
}

#[cfg(windows)]
fn pairing_input() -> Result<Zeroizing<String>> {
    use std::io::IsTerminal;
    use windows_sys::Win32::System::Console::{
        ENABLE_ECHO_INPUT, GetConsoleMode, GetStdHandle, STD_INPUT_HANDLE, SetConsoleMode,
    };
    struct Echo {
        handle: windows_sys::Win32::Foundation::HANDLE,
        mode: u32,
    }
    impl Drop for Echo {
        fn drop(&mut self) {
            unsafe { SetConsoleMode(self.handle, self.mode) };
            eprintln!();
        }
    }
    let _echo = if std::io::stdin().is_terminal() {
        let handle = unsafe { GetStdHandle(STD_INPUT_HANDLE) };
        let mut mode = 0;
        if unsafe { GetConsoleMode(handle, &mut mode) } == 0
            || unsafe { SetConsoleMode(handle, mode & !ENABLE_ECHO_INPUT) } == 0
        {
            return Err(err("unavailable", "Could not disable keyboard echo."));
        }
        eprint!("Enter the pairing code shown on the Android phone: ");
        Some(Echo { handle, mode })
    } else {
        None
    };
    let mut line = Zeroizing::new(String::new());
    use std::io::Read;
    std::io::stdin()
        .lock()
        .take(97)
        .read_line(&mut line)
        .map_err(|_| invalid("Pairing code could not be read."))?;
    expect(line.len() <= 96, "Pairing code exceeds its maximum length.")?;
    Ok(line)
}
#[cfg(not(windows))]
fn pairing_input() -> Result<Zeroizing<String>> {
    Err(err(
        "unavailable",
        "Windows is required to export a diagnostic snapshot.",
    ))
}
fn export(path: &Path) -> Result<Value> {
    expect(
        path.is_absolute()
            && path
                .extension()
                .and_then(|e| e.to_str())
                .is_some_and(|e| e.eq_ignore_ascii_case("intadiag")),
        "Select a new absolute local .intadiag file.",
    )?;
    // Validate the full filename with the same reserved-name, ADS and local-drive rules as the parent.
    let safe = files::validated_path(path)?;
    let directory = files::directory(
        safe.parent()
            .ok_or_else(|| invalid("Select a local directory."))?,
        false,
    )?;
    let dest = directory.path.join(
        safe.file_name()
            .ok_or_else(|| invalid("Select a filename."))?,
    );
    expect(
        !dest
            .try_exists()
            .map_err(|_| err("io", "The destination could not be checked."))?,
        "Destination already exists.",
    )?;
    let code = pairing_input()?;
    let at = now()?;
    let pair = Pair::parse(code.trim_end_matches(['\n', '\r']), at)?;
    drop(code);
    let body = observe()?;
    let sealed = encrypt(&pair, at, &body)?;
    let mut temp = tempfile::NamedTempFile::new_in(&directory.path)
        .map_err(|_| err("io", "Could not stage the encrypted report."))?;
    temp.write_all(&sealed)
        .and_then(|_| temp.as_file().sync_all())
        .map_err(|_| err("io", "Could not flush the encrypted report."))?;
    temp.persist_noclobber(&dest).map_err(|_| {
        err(
            "io",
            "A new report could not be published. Existing files are preserved.",
        )
    })?;
    Ok(
        json!({"state":"exported","schemaVersion":1,"capturedAtUnix":at,"expiresAtUnix":pair.issued+LIFETIME,"bytes":sealed.len(),
        "sha256":format!("{:x}",Sha256::digest(&sealed)),"fields":FIELDS}),
    )
}
fn main() {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    let words = args
        .iter()
        .map(|w| w.to_str().unwrap_or(""))
        .collect::<Vec<_>>();
    let result = match words.as_slice() {
        ["preview"] => Ok(
            json!({"operation":"diagnostic-snapshot","schemaVersion":1,"fields":FIELDS,"transport":"Android Storage Access Framework","lifetimeSeconds":LIFETIME,"writes":"one new encrypted report","requiresApproval":true}),
        ),
        ["export", path, "--approve"] => export(Path::new(path)),
        [] | ["--help"] => Ok(
            json!({"commands":["preview","export ABSOLUTE_NEW_FILE.intadiag --approve"],"pairingCodeInput":"stdin only"}),
        ),
        _ => Err(invalid(
            "Use preview or export ABSOLUTE_NEW_FILE.intadiag --approve.",
        )),
    };
    let (code, out) = match result {
        Ok(v) => (0, json!({"schemaVersion":1,"ok":true,"result":v})),
        Err(e) => (
            2,
            json!({"schemaVersion":1,"ok":false,"error":{"code":e.code,"message":e.message}}),
        ),
    };
    println!("{out}");
    std::process::exit(code)
}
