use sha2::{Digest, Sha256};
use std::{
    env,
    fs::{self, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
};
use zip::{write::SimpleFileOptions, CompressionMethod, ZipWriter};

fn files(root: &Path, path: &Path, output: &mut Vec<(PathBuf, PathBuf)>) -> io::Result<()> {
    let mut entries = fs::read_dir(path)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let path = entry.path();
        // Do not package symlinks escaping the selected release directory.
        if entry.file_type()?.is_symlink() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Release files must not be symlinks",
            ));
        }
        if path.is_dir() {
            files(root, &path, output)?;
        } else if path.is_file() {
            output.push((path.clone(), path.strip_prefix(root).unwrap().to_path_buf()));
        }
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args_os().skip(1).collect();
    if args.len() != 2 {
        return Err("Usage: inta-agent-package <publish-directory> <output.zip>".into());
    }
    let source = PathBuf::from(&args[0]).canonicalize()?;
    let output = PathBuf::from(&args[1]);
    let mut entries = Vec::new();
    files(&source, &source, &mut entries)?;
    if entries.is_empty() {
        return Err("Publish directory is empty".into());
    }
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut archive = ZipWriter::new(File::create(&output)?);
    for (path, relative) in &entries {
        let options = SimpleFileOptions::default()
            .compression_method(CompressionMethod::Deflated)
            .unix_permissions(if path.extension().is_none() {
                0o755
            } else {
                0o644
            });
        archive.start_file(relative.to_string_lossy().replace('\\', "/"), options)?;
        io::copy(&mut File::open(path)?, &mut archive)?;
    }
    archive.finish()?.sync_all()?;
    let mut hasher = Sha256::new();
    let mut file = File::open(&output)?;
    let mut block = [0; 64 * 1024];
    loop {
        let n = file.read(&mut block)?;
        if n == 0 {
            break;
        }
        hasher.update(&block[..n]);
    }
    let checksum = format!("{:x}", hasher.finalize());
    let sidecar = PathBuf::from(format!("{}.sha256", output.to_string_lossy()));
    writeln!(
        File::create(sidecar)?,
        "{}  {}",
        checksum,
        output.file_name().unwrap().to_string_lossy()
    )?;
    println!(
        "Packaged {} files into {} (SHA-256 {})",
        entries.len(),
        output.display(),
        checksum
    );
    Ok(())
}
