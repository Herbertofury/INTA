use anyhow::{Context, Result, bail};
use serde_json::{Value, json};
use std::{
    io::Read,
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

pub const KINDS: &[&str] = &[
    "overview",
    "storage",
    "security",
    "network",
    "startup",
    "drivers",
    "events",
    "performance",
    "applications",
    "updates",
    "power",
    "printing",
    "directory",
    "recovery",
    "disks",
    "models",
];

#[cfg(windows)]
fn windows_dir() -> Result<std::path::PathBuf> {
    use std::os::windows::ffi::OsStringExt;
    let mut buf = [0u16; 32768];
    // The buffer is writable for its advertised length; the API returns UTF-16 units.
    let n = unsafe {
        windows_sys::Win32::System::SystemInformation::GetWindowsDirectoryW(
            buf.as_mut_ptr(),
            buf.len() as u32,
        )
    } as usize;
    anyhow::ensure!(n > 0 && n < buf.len(), "Windows directory unavailable");
    Ok(std::ffi::OsString::from_wide(&buf[..n]).into())
}
#[cfg(not(windows))]
fn windows_dir() -> Result<std::path::PathBuf> {
    bail!("Windows host required")
}

fn query(kind: &str) -> Result<&'static str> {
    Ok(match kind {
        "overview" => {
            r#"$os=Get-CimInstance Win32_OperatingSystem; $cpu=Get-CimInstance Win32_Processor | Select-Object -First 1; [pscustomobject]@{system=$os.Caption;build=$os.BuildNumber;version=$os.Version;architecture=$os.OSArchitecture;processor=$cpu.Name;logicalProcessors=[int]$env:NUMBER_OF_PROCESSORS;memoryGiB=[math]::Round($os.TotalVisibleMemorySize/1MB,1);freeMemoryGiB=[math]::Round($os.FreePhysicalMemory/1MB,1);lastBootUtc=$os.LastBootUpTime.ToUniversalTime().ToString('o')}"#
        }
        "storage" => {
            r#"Get-CimInstance Win32_LogicalDisk -Filter 'DriveType=3' | Sort-Object DeviceID | ForEach-Object { [pscustomobject]@{volume=$_.DeviceID;filesystem=$_.FileSystem;sizeBytes=$_.Size;freeBytes=$_.FreeSpace} }"#
        }
        "network" => {
            r#"Get-NetAdapter -Physical | Sort-Object ifIndex | ForEach-Object { [pscustomobject]@{interfaceIndex=$_.ifIndex;description=$_.InterfaceDescription;status=[string]$_.Status;linkSpeed=$_.LinkSpeed} }"#
        }
        "startup" => {
            r#"Get-CimInstance Win32_StartupCommand | Group-Object Location | ForEach-Object { $scope=if($_.Name -match '^HKLM'){ 'Machine registry' }elseif($_.Name -match '^HK'){ 'User registry' }else{'Startup folder'}; [pscustomobject]@{scope=$scope;entries=$_.Count} }"#
        }
        "security" => {
            r#"try { $d=Get-MpComputerStatus; [pscustomobject]@{source='Microsoft Defender Antivirus';antivirusEnabled=$d.AntivirusEnabled;realtimeProtection=$d.RealTimeProtectionEnabled;signatureVersion=$d.AntivirusSignatureVersion;signatureAgeDays=$d.AntivirusSignatureAge;status='observed'} }catch { [pscustomobject]@{source='Microsoft Defender Antivirus';status='unavailable'} }; try { Get-NetFirewallProfile -PolicyStore ActiveStore | ForEach-Object {[pscustomobject]@{source='Windows Firewall';profile=$_.Name;enabled=[string]$_.Enabled;status='observed'}} }catch { [pscustomobject]@{source='Windows Firewall';status='unavailable'} }"#
        }
        "printing" => {
            r#"$spool=Get-Service Spooler; [pscustomobject]@{source='Print Spooler';status=[string]$spool.Status}; Get-CimInstance Win32_Printer | Group-Object PrinterStatus,WorkOffline | ForEach-Object { [pscustomobject]@{source='Configured printers';statusCode=$_.Group[0].PrinterStatus;workOffline=$_.Group[0].WorkOffline;count=$_.Count} }"#
        }
        "directory" => {
            r#"$sys=Get-CimInstance Win32_ComputerSystem; [pscustomobject]@{source='Local domain configuration';joined=$sys.PartOfDomain;domainRoleCode=$sys.DomainRole;netlogon=[string](Get-Service Netlogon).Status;timeService=[string](Get-Service W32Time).Status}"#
        }
        "recovery" => {
            r#"$svc=Get-CimInstance Win32_Service -Filter "Name='VSS' OR Name='swprv' OR Name='wbengine'"; $svc | ForEach-Object { [pscustomobject]@{source='Recovery service';name=$_.Name;state=$_.State;startMode=$_.StartMode} }; $base=[Environment]::SystemDirectory; foreach($n in 'reagentc.exe','bcdboot.exe','wbadmin.exe'){ [pscustomobject]@{source='System utility';name=$n;present=[IO.File]::Exists([IO.Path]::Combine($base,$n))} }"#
        }
        "disks" => {
            r#"Get-PhysicalDisk | ForEach-Object { [pscustomobject]@{source='Windows storage provider';deviceId=$_.DeviceId;bus=[string]$_.BusType;media=[string]$_.MediaType;health=[string]$_.HealthStatus;operational=[string]($_.OperationalStatus -join ', ');sizeBytes=$_.Size} }"#
        }
        _ => bail!("Unknown inspection provider"),
    })
}
fn bounded_read(mut pipe: impl Read) -> std::io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    pipe.by_ref().take(1_048_577).read_to_end(&mut buf)?;
    Ok(buf)
}
pub fn run(kind: &str) -> Result<Value> {
    if kind == "models" {
        return crate::storage::models();
    }
    if matches!(kind, "applications" | "updates" | "power") {
        return crate::system_inspection::run(kind);
    }
    if matches!(kind, "drivers" | "events" | "performance") {
        return crate::health::inspect(kind);
    }
    run_query(kind, query(kind)?)
}
pub(crate) fn run_query(kind: &str, body: &'static str) -> Result<Value> {
    let system = windows_dir()?.join("System32");
    let psdir = system.join("WindowsPowerShell/v1.0");
    let script = format!(
        "$ErrorActionPreference='Stop'; $ProgressPreference='SilentlyContinue'; [Console]::OutputEncoding=[Text.UTF8Encoding]::new($false); try {{ $rows=@(& {{ {body} }}); ConvertTo-Json -InputObject $rows -Compress -Depth 4 }} catch {{ exit 2 }}"
    );
    let mut cmd = Command::new(psdir.join("powershell.exe"));
    cmd.args([
        "-NoLogo",
        "-NoProfile",
        "-NonInteractive",
        "-Command",
        &script,
    ])
    .env("PSModulePath", psdir.join("Modules"))
    .current_dir(&system)
    .stdin(Stdio::null())
    .stdout(Stdio::piped())
    .stderr(Stdio::null());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x08000000);
    }
    let mut child = cmd
        .spawn()
        .context("Cannot start the built-in Windows inventory provider")?;
    let pipe = child
        .stdout
        .take()
        .context("Inventory output unavailable")?;
    let out = thread::spawn(move || bounded_read(pipe));
    let started = Instant::now();
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break status;
        }
        if started.elapsed() > Duration::from_secs(35) {
            let _ = child.kill();
            let _ = child.wait();
            bail!("Inventory timed out after 35 seconds")
        }
        thread::sleep(Duration::from_millis(35))
    };
    anyhow::ensure!(
        status.success(),
        "The Windows {kind} provider is unavailable or access was denied"
    );
    let bytes = out
        .join()
        .map_err(|_| anyhow::anyhow!("Inventory reader failed"))??;
    anyhow::ensure!(
        bytes.len() <= 1_048_576,
        "Inventory output exceeds the report limit"
    );
    let rows: Vec<Value> = serde_json::from_slice(&bytes).context("Invalid inventory response")?;
    Ok(json!({"schemaVersion":1,"kind":kind,"observedAt":crate::db::now(),"observations":rows}))
}
