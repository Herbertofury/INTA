pub mod ui;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};
use tempfile::NamedTempFile;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub const ISO: &str = "inta-rescue-amd64.iso";
pub const SUMS: &str = "SHA256SUMS";
pub const PARTS: &str = "inta-rescue-amd64.iso.parts.json";
pub const RELEASES_API: &str =
    "https://api.github.com/repos/Herbertofury/INTA/releases?per_page=30";
pub const ASSET_ROOT: &str = "https://github.com/Herbertofury/INTA/releases/download/";
pub const MAX_PART_SIZE: u64 = 1024 * 1024 * 1024;
pub const MAX_MANIFEST_SIZE: u64 = 1024 * 1024;

#[derive(Clone, Debug, Deserialize)]
pub struct Asset {
    pub name: String,
    pub size: u64,
    pub browser_download_url: String,
}
#[derive(Clone, Debug, Deserialize)]
pub struct Release {
    pub assets: Vec<Asset>,
    #[serde(default)]
    pub draft: bool,
    #[serde(default)]
    pub prerelease: bool,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Manifest {
    pub schema_version: u32,
    pub filename: String,
    pub size: u64,
    pub sha256: String,
    pub parts: Vec<ManifestPart>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestPart {
    pub name: String,
    pub size: u64,
    pub sha256: String,
}
#[derive(Clone, Debug)]
pub struct DownloadPart {
    pub name: String,
    pub url: String,
    pub size: u64,
    pub sha256: String,
}
#[derive(Clone, Debug)]
pub struct DownloadPlan {
    pub size: u64,
    pub sha256: String,
    pub parts: Vec<DownloadPart>,
}
#[derive(Clone, Debug)]
pub struct Progress {
    pub received: u64,
    pub total: u64,
    pub phase: &'static str,
}

fn digest_valid(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|c| c.is_ascii_hexdigit())
}
pub fn expected_digest(contents: &str) -> Result<String> {
    let matches: Vec<_> = contents
        .lines()
        .filter_map(|line| {
            let fields: Vec<_> = line.split_whitespace().collect();
            if fields.len() == 2
                && fields[1].trim_start_matches('*') == ISO
                && digest_valid(fields[0])
            {
                Some(fields[0].to_ascii_lowercase())
            } else {
                None
            }
        })
        .collect();
    if matches.len() != 1 {
        return Err(format!("Expected exactly one checksum for {ISO}.").into());
    }
    Ok(matches[0].clone())
}
pub fn asset_map(release: Release) -> Result<BTreeMap<String, Asset>> {
    let mut assets = BTreeMap::new();
    for asset in release.assets {
        if assets.insert(asset.name.clone(), asset).is_some() {
            return Err("The release contains duplicate asset names.".into());
        }
    }
    Ok(assets)
}
pub fn release_asset<'a>(assets: &'a BTreeMap<String, Asset>, name: &str) -> Result<&'a Asset> {
    let asset = assets
        .get(name)
        .ok_or_else(|| format!("The release is missing {name}."))?;
    let url = url::Url::parse(&asset.browser_download_url)?;
    if url.scheme() != "https"
        || url.host_str() != Some("github.com")
        || !url.username().is_empty()
        || url.password().is_some()
        || url.port().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || !asset.browser_download_url.starts_with(ASSET_ROOT)
        || !url
            .path()
            .starts_with("/Herbertofury/INTA/releases/download/")
        || !url.path().ends_with(&format!("/{name}"))
        || url.path().contains('%')
    {
        return Err("Unexpected release asset URL.".into());
    }
    Ok(asset)
}
pub fn verified_parts(
    manifest: Manifest,
    assets: &BTreeMap<String, Asset>,
    expected: &str,
) -> Result<DownloadPlan> {
    if manifest.schema_version != 1
        || manifest.filename != ISO
        || manifest.size == 0
        || !digest_valid(&manifest.sha256)
        || manifest.sha256.to_ascii_lowercase() != expected.to_ascii_lowercase()
        || manifest.parts.is_empty()
    {
        return Err("Invalid ISO parts manifest identity or checksum.".into());
    }
    let mut parts = Vec::new();
    let mut total = 0u64;
    for (index, part) in manifest.parts.into_iter().enumerate() {
        let name = format!("{ISO}.part{:03}", index + 1);
        if part.name != name
            || part.size == 0
            || part.size > MAX_PART_SIZE
            || !digest_valid(&part.sha256)
        {
            return Err(format!("Invalid ISO part {name}.").into());
        }
        let asset = release_asset(assets, &name)?;
        if asset.size != part.size {
            return Err(format!("Release size differs from manifest for {name}.").into());
        }
        total = total
            .checked_add(part.size)
            .ok_or("ISO parts size overflow.")?;
        parts.push(DownloadPart {
            name,
            url: asset.browser_download_url.clone(),
            size: part.size,
            sha256: part.sha256.to_ascii_lowercase(),
        });
    }
    if total != manifest.size {
        return Err("ISO part sizes do not add up to the original ISO size.".into());
    }
    Ok(DownloadPlan {
        size: total,
        sha256: expected.to_ascii_lowercase(),
        parts,
    })
}

pub trait Fetcher {
    fn fetch(&self, url: &str) -> Result<Box<dyn Read>>;
}
pub struct Network {
    agent: ureq::Agent,
}
impl Default for Network {
    fn default() -> Self {
        Self {
            agent: ureq::AgentBuilder::new()
                .timeout_connect(Duration::from_secs(15))
                .timeout_read(Duration::from_secs(45))
                .timeout_write(Duration::from_secs(45))
                .user_agent("INTA-Media/0.2.0")
                .build(),
        }
    }
}
impl Fetcher for Network {
    fn fetch(&self, url: &str) -> Result<Box<dyn Read>> {
        Ok(self.agent.get(url).call()?.into_reader())
    }
}
fn bounded(fetcher: &dyn Fetcher, url: &str, limit: u64) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    fetcher
        .fetch(url)?
        .take(limit + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 > limit {
        return Err("Release metadata exceeds its size limit.".into());
    }
    Ok(bytes)
}

pub fn resolve_plan(release: Release, fetcher: &dyn Fetcher) -> Result<DownloadPlan> {
    let assets = asset_map(release)?;
    let sums = release_asset(&assets, SUMS)?;
    let expected = expected_digest(std::str::from_utf8(&bounded(
        fetcher,
        &sums.browser_download_url,
        65_536,
    )?)?)?;
    if assets.contains_key(ISO) {
        let image = release_asset(&assets, ISO)?;
        if image.size == 0 {
            return Err("Release ISO is empty.".into());
        }
        return Ok(DownloadPlan {
            size: image.size,
            sha256: expected.clone(),
            parts: vec![DownloadPart {
                name: ISO.into(),
                url: image.browser_download_url.clone(),
                size: image.size,
                sha256: expected,
            }],
        });
    }
    let asset = release_asset(&assets, PARTS)?;
    if asset.size == 0 || asset.size > MAX_MANIFEST_SIZE {
        return Err("The ISO parts manifest must be nonempty and at most 1 MiB.".into());
    }
    let bytes = bounded(fetcher, &asset.browser_download_url, asset.size)?;
    if bytes.len() as u64 != asset.size {
        return Err("The parts manifest size differs from its release asset.".into());
    }
    let manifest: Manifest = serde_json::from_slice(&bytes)?;
    verified_parts(manifest, &assets, &expected)
}

pub fn atomic_download(
    plan: &DownloadPlan,
    folder: &Path,
    fetcher: &dyn Fetcher,
    cancel: &AtomicBool,
    progress: &mut dyn FnMut(Progress),
) -> Result<PathBuf> {
    if !folder.is_dir() {
        return Err("Choose an existing destination directory.".into());
    }
    if fs2::available_space(folder)? < plan.size {
        return Err("The destination has insufficient free space for a safe replacement.".into());
    }
    let mut partial = NamedTempFile::new_in(folder)?;
    let mut overall = Sha256::new();
    let mut total = 0;
    for part in &plan.parts {
        if cancel.load(Ordering::Relaxed) {
            return Err("Transfer canceled. The previous ISO is intact.".into());
        }
        let mut input = fetcher.fetch(&part.url)?;
        let mut digest = Sha256::new();
        let mut size = 0u64;
        let mut buffer = vec![0; 1024 * 1024];
        loop {
            if cancel.load(Ordering::Relaxed) {
                return Err("Transfer canceled. The previous ISO is intact.".into());
            }
            let n = input.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            size = size.checked_add(n as u64).ok_or("Part size overflow.")?;
            if size > part.size {
                return Err(format!(
                    "Download exceeds release size for {}. The previous ISO is intact.",
                    part.name
                )
                .into());
            }
            digest.update(&buffer[..n]);
            overall.update(&buffer[..n]);
            partial.write_all(&buffer[..n])?;
            total += n as u64;
            progress(Progress {
                received: total,
                total: plan.size,
                phase: "Downloading & verifying",
            });
        }
        if size != part.size || format!("{:x}", digest.finalize()) != part.sha256 {
            return Err(format!(
                "Verification failed for {}. The previous ISO is intact.",
                part.name
            )
            .into());
        }
    }
    if total != plan.size || format!("{:x}", overall.finalize()) != plan.sha256 {
        return Err("Combined ISO verification failed. The previous ISO is intact.".into());
    }
    if cancel.load(Ordering::Relaxed) {
        return Err("Transfer canceled. The previous ISO is intact.".into());
    }
    partial.flush()?;
    partial.as_file().sync_all()?;
    let destination = folder.join(ISO);
    partial.persist(&destination)?;
    let mut sidecar = NamedTempFile::new_in(folder)?;
    writeln!(sidecar, "{}  {}", plan.sha256, ISO)?;
    sidecar.flush()?;
    sidecar.as_file().sync_all()?;
    sidecar.persist(folder.join(SUMS))?;
    progress(Progress {
        received: total,
        total: plan.size,
        phase: "Verified ISO ready",
    });
    Ok(destination)
}
pub fn download_release(
    folder: &Path,
    cancel: &AtomicBool,
    progress: &mut dyn FnMut(Progress),
) -> Result<PathBuf> {
    let network = Network::default();
    let bytes = bounded(&network, RELEASES_API, 4 * 1024 * 1024)?;
    let releases: Vec<Release> = serde_json::from_slice(&bytes)?;
    let release = releases
        .into_iter()
        .find(|r| {
            !r.draft
                && !r.prerelease
                && r.assets.iter().any(|a| a.name == SUMS)
                && r.assets.iter().any(|a| a.name == ISO || a.name == PARTS)
        })
        .ok_or("No published Rescue image was found in the recent official releases.")?;
    let plan = resolve_plan(release, &network)?;
    atomic_download(&plan, folder, &network, cancel, progress)
}

pub fn stream_verified(
    mut source: impl Read,
    target: &Path,
    expected: &str,
    expected_size: u64,
    cancel: &AtomicBool,
    progress: &mut dyn FnMut(Progress),
) -> Result<()> {
    if !digest_valid(expected) {
        return Err("Invalid expected checksum.".into());
    }
    let folder = target.parent().ok_or("Target has no parent directory.")?;
    let mut partial = NamedTempFile::new_in(folder)?;
    let mut digest = Sha256::new();
    let mut total = 0u64;
    let mut buffer = vec![0; 1024 * 1024];
    loop {
        if cancel.load(Ordering::Relaxed) {
            return Err("Copy canceled. The previous ISO is intact.".into());
        }
        let n = source.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        total += n as u64;
        if total > expected_size {
            return Err("Copy exceeds the expected image size.".into());
        }
        digest.update(&buffer[..n]);
        partial.write_all(&buffer[..n])?;
        progress(Progress {
            received: total,
            total: expected_size,
            phase: "Copying & verifying",
        });
    }
    if total != expected_size || format!("{:x}", digest.finalize()) != expected.to_ascii_lowercase()
    {
        return Err("Copy verification failed. The previous ISO is intact.".into());
    }
    partial.flush()?;
    partial.as_file().sync_all()?;
    partial.persist(target)?;
    Ok(())
}
pub fn is_mount_root(path: &Path) -> bool {
    let Ok(path) = path.canonicalize() else {
        return false;
    };
    if !path.is_dir() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let Some(parent) = path.parent() else {
            return true;
        };
        match (fs::metadata(&path), fs::metadata(parent)) {
            (Ok(a), Ok(b)) => a.dev() != b.dev(),
            _ => false,
        }
    }
    #[cfg(windows)]
    {
        path.parent().is_none()
    }
    #[cfg(not(any(unix, windows)))]
    {
        false
    }
}
pub fn copy_to_ventoy(
    source: &Path,
    volume: &Path,
    cancel: &AtomicBool,
    progress: &mut dyn FnMut(Progress),
) -> Result<PathBuf> {
    let parent = source
        .parent()
        .ok_or("Image has no containing directory.")?;
    let expected = expected_digest(&fs::read_to_string(parent.join(SUMS))?)?;
    if !is_mount_root(volume) {
        return Err("Select the root of the mounted Ventoy data volume.".into());
    }
    let size = source.metadata()?.len();
    if fs2::available_space(volume)? < size {
        return Err("The Ventoy volume has insufficient free space for a safe replacement.".into());
    }
    let target = volume.join(ISO);
    stream_verified(
        File::open(source)?,
        &target,
        &expected,
        size,
        cancel,
        progress,
    )?;
    Ok(target)
}
pub fn valid_official_executable(tool: &str, path: &Path, windows: bool) -> bool {
    let Some(name) = path.file_name().and_then(|s| s.to_str()) else {
        return false;
    };
    if tool == "Ventoy" {
        return if windows {
            name.eq_ignore_ascii_case("Ventoy2Disk.exe")
        } else {
            name == "VentoyGUI.x86_64"
        };
    }
    if tool != "Rufus" || !windows {
        return false;
    }
    let name = name.to_ascii_lowercase();
    let Some(stem) = name.strip_suffix(".exe") else {
        return false;
    };
    if stem == "rufus" {
        return true;
    }
    let Some(version) = stem.strip_prefix("rufus-") else {
        return false;
    };
    let version = version.strip_suffix('p').unwrap_or(version);
    !version.is_empty() && version.bytes().all(|b| b.is_ascii_digit() || b == b'.')
}
pub fn launch_official(tool: &str, path: &Path) -> Result<()> {
    if !valid_official_executable(tool, path, cfg!(windows)) || !path.is_file() {
        return Err(format!("Choose the publisher's {tool} executable. A filename check is not signature verification.").into());
    }
    #[cfg(windows)]
    {
        open::that_detached(path)?;
    }
    #[cfg(not(windows))]
    {
        std::process::Command::new(path)
            .current_dir(path.parent().ok_or("Executable has no parent")?)
            .spawn()?;
    }
    Ok(())
}

/// Exercises the real streaming downloader against locally prepared release bytes.
/// The expected digest must come from the independently boot-tested original ISO.
pub fn verify_local_release_directory(
    directory: &Path,
    expected: &str,
) -> Result<serde_json::Value> {
    if !digest_valid(expected) {
        return Err("Supply the original ISO SHA-256 digest.".into());
    }
    let directory = directory.canonicalize()?;
    if expected_digest(&fs::read_to_string(directory.join(SUMS))?)? != expected.to_ascii_lowercase()
    {
        return Err("Release checksums differ from the supplied original ISO digest.".into());
    }
    struct LocalFiles(BTreeMap<String, PathBuf>);
    impl Fetcher for LocalFiles {
        fn fetch(&self, url: &str) -> Result<Box<dyn Read>> {
            Ok(Box::new(File::open(
                self.0.get(url).ok_or("Requested a non-release asset")?,
            )?))
        }
    }
    let mut files = BTreeMap::new();
    let mut assets = Vec::new();
    for entry in fs::read_dir(&directory)? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        let url = format!("{ASSET_ROOT}local-proof/{name}");
        assets.push(Asset {
            name,
            size: entry.metadata()?.len(),
            browser_download_url: url.clone(),
        });
        files.insert(url, entry.path());
    }
    let fetcher = LocalFiles(files);
    let plan = resolve_plan(
        Release {
            assets,
            draft: false,
            prerelease: false,
        },
        &fetcher,
    )?;
    let temporary = tempfile::tempdir()?;
    let target = atomic_download(
        &plan,
        temporary.path(),
        &fetcher,
        &AtomicBool::new(false),
        &mut |_| {},
    )?;
    let mut input = File::open(&target)?;
    let mut digest = Sha256::new();
    let mut buffer = [0u8; 1024 * 1024];
    loop {
        let n = input.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        digest.update(&buffer[..n]);
    }
    let actual = format!("{:x}", digest.finalize());
    if actual != expected.to_ascii_lowercase() {
        return Err("Reconstructed ISO differs from original independent checksum.".into());
    }
    if fs::read_dir(temporary.path())?.count() != 2 {
        return Err("Temporary download files remained after verification.".into());
    }
    Ok(
        serde_json::json!({"mode":if plan.parts.len()>1{"parts"}else{"iso"},"filename":ISO,"size":target.metadata()?.len(),"sha256":actual,"sourceDirectory":directory,"requestedAssets":plan.parts.iter().map(|p|&p.name).collect::<Vec<_>>()}),
    )
}
