use super::*;
use std::{
    mem::{offset_of, size_of},
    ptr,
};
use windows_sys::Win32::{
    Foundation::{CloseHandle, LocalFree},
    Security::{
        Authorization::{
            ConvertSidToStringSidW, ConvertStringSecurityDescriptorToSecurityDescriptorW,
            SDDL_REVISION_1,
        },
        GetTokenInformation, SECURITY_ATTRIBUTES, TOKEN_QUERY, TOKEN_USER, TokenUser,
    },
    System::Threading::{GetCurrentProcess, OpenProcessToken},
};
fn lock(c: &Connection, id: &str) -> Result<(Vec<File>, File)> {
    let dir = Path::new(c.path().context("Persistent cleanup database required")?)
        .parent()
        .context("State directory unavailable")?
        .join("cleanup-locks");
    let parents = directories(&dir, true)?;
    let path = dir.join(format!("{id}.lock"));
    let until = Instant::now() + Duration::from_secs(2);
    let f =
        loop {
            match OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(false)
                .share_mode(0)
                .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
                .open(&path)
            {
                Ok(file) => break file,
                Err(e) if matches!(e.raw_os_error(), Some(32 | 33)) && Instant::now() < until => {
                    std::thread::sleep(Duration::from_millis(25))
                }
                Err(e) => return Err(e).context(
                    "This review is already open for another operation or its lock is unavailable",
                ),
            }
        };
    let m = f.metadata()?;
    ensure!(
        m.is_file() && m.file_attributes() & CLOUD == 0 && identity(&f)?.1 == 1,
        "Invalid cleanup lock"
    );
    Ok((parents, f))
}
fn private_vault(path: &Path) -> Result<()> {
    let mut token = ptr::null_mut();
    ensure!(
        unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) } != 0,
        "Cannot identify quarantine owner"
    );
    let mut words = [0u64; 128];
    let mut n = 0;
    let ok = unsafe {
        GetTokenInformation(
            token,
            TokenUser,
            words.as_mut_ptr().cast(),
            size_of_val(&words) as u32,
            &mut n,
        )
    };
    unsafe { CloseHandle(token) };
    ensure!(ok != 0, "Cannot identify quarantine owner");
    let user = unsafe { &*words.as_ptr().cast::<TOKEN_USER>() };
    let mut sid = ptr::null_mut();
    ensure!(
        unsafe { ConvertSidToStringSidW(user.User.Sid, &mut sid) } != 0,
        "Cannot read owner SID"
    );
    let mut length = 0;
    let text = unsafe {
        while *sid.add(length) != 0 {
            length += 1
        }
        let s = String::from_utf16_lossy(std::slice::from_raw_parts(sid, length));
        LocalFree(sid.cast());
        s
    };
    let acl = format!("D:P(A;OICI;FA;;;{text})(A;OICI;FA;;;SY)")
        .encode_utf16()
        .chain([0])
        .collect::<Vec<_>>();
    let mut desc = ptr::null_mut();
    ensure!(
        unsafe {
            ConvertStringSecurityDescriptorToSecurityDescriptorW(
                acl.as_ptr(),
                SDDL_REVISION_1,
                &mut desc,
                ptr::null_mut(),
            )
        } != 0,
        "Cannot create private quarantine ACL"
    );
    let security = SECURITY_ATTRIBUTES {
        nLength: size_of::<SECURITY_ATTRIBUTES>() as u32,
        lpSecurityDescriptor: desc,
        bInheritHandle: 0,
    };
    let wide = path
        .as_os_str()
        .encode_wide()
        .chain([0])
        .collect::<Vec<_>>();
    let ok = unsafe { CreateDirectoryW(wide.as_ptr(), &security) };
    let error = io::Error::last_os_error();
    unsafe { LocalFree(desc) };
    ensure!(
        ok != 0,
        "Could not create a new private quarantine directory: {error}"
    );
    Ok(())
}
fn vault(c: &Connection, r: &mut Review, create: bool) -> Result<(PathBuf, Vec<File>)> {
    if let (Some(key), Some(id)) = (&r.vault, &r.vault_identity) {
        ensure!(valid_id(key), "Invalid quarantine identifier");
        let dir = r.root.join(format!(".inta-quarantine-{key}"));
        let held = checked_dirs(&dir)?;
        ensure!(
            identity(held.last().context("Missing quarantine directory")?)?.0 == *id,
            "Quarantine directory identity changed"
        );
        return Ok((dir, held));
    }
    ensure!(create, "There is no quarantine for this review");
    let key = uuid();
    let dir = r.root.join(format!(".inta-quarantine-{key}"));
    private_vault(&dir)?;
    let held = checked_dirs(&dir)?;
    let id = identity(held.last().context("Missing quarantine directory")?)?.0;
    r.vault = Some(key);
    r.vault_identity = Some(id);
    save(c, r, None)?;
    Ok((dir, held))
}
fn attest(f: &mut File, item: &Item) -> Result<()> {
    let (bytes, time, id, hash) = snapshot(f)?;
    ensure!(
        bytes == item.bytes && time == item.modified && id == item.identity && hash == item.sha256,
        "The file changed since review; its bytes were preserved"
    );
    Ok(())
}
fn rename(handle: &File, target: &Path) -> Result<()> {
    let name = target.as_os_str().encode_wide().collect::<Vec<_>>();
    let count = name.len().checked_mul(2).context("Path too long")?;
    let bytes =
        (offset_of!(FILE_RENAME_INFO, FileName) + count + 2).max(size_of::<FILE_RENAME_INFO>());
    let mut buf = vec![0usize; bytes.div_ceil(size_of::<usize>())];
    let info = buf.as_mut_ptr().cast::<FILE_RENAME_INFO>();
    // Zeroed ReplaceIfExists=false. Only this verified handle moves; both parent directories
    // are held against rename. A competing destination wins without being overwritten.
    let ok = unsafe {
        (*info).FileNameLength = count.try_into()?;
        ptr::copy_nonoverlapping(
            name.as_ptr(),
            ptr::addr_of_mut!((*info).FileName).cast::<u16>(),
            name.len(),
        );
        SetFileInformationByHandle(
            handle.as_raw_handle(),
            FileRenameInfo,
            info.cast(),
            bytes.try_into()?,
        )
    };
    ensure!(
        ok != 0,
        "Move did not complete: {}",
        io::Error::last_os_error()
    );
    Ok(())
}
fn set(c: &Connection, r: &mut Review, i: usize, state: &str, detail: &str) -> Result<()> {
    r.files[i].state = state.into();
    r.files[i].detail = detail.into();
    save(c, r, Some((&r.files[i].id, state, detail)))
}
fn parent_guards(root: &Path, item: &Item) -> Result<Vec<File>> {
    ensure!(
        valid_id(&item.id)
            && item.path.starts_with(root)
            && !sensitive(&item.path)
            && candidate(&item.path),
        "Invalid recorded file scope"
    );
    let p = item.path.parent().context("Missing original directory")?;
    let held = checked_dirs(p)?;
    ensure!(
        identity(held.last().context("Missing original directory")?)?.0 == item.parent_identity,
        "Original directory identity changed"
    );
    Ok(held)
}
fn observed(path: &Path, item: &Item) -> Result<bool> {
    match fs::symlink_metadata(path) {
        Ok(m) if !m.is_file() || m.file_attributes() & CLOUD != 0 => Ok(false),
        Ok(_) => {
            let mut f = open_regular(path, false)?;
            Ok(attest(&mut f, item).is_ok())
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e.into()),
    }
}
fn recover(c: &Connection, r: &mut Review, vault: &Path) -> Result<()> {
    for i in 0..r.files.len() {
        let item = &r.files[i];
        if !matches!(item.state.as_str(), "quarantining" | "restoring") {
            continue;
        }
        let Ok(_parents) = parent_guards(&r.root, item) else {
            continue;
        };
        let slot = vault.join(format!("{}.bin", item.id));
        let (Ok(original), Ok(saved)) = (observed(&item.path, item), observed(&slot, item)) else {
            continue;
        };
        let state = match (original, saved) {
            (true, false) if item.state == "restoring" => "restored",
            (true, false) => "planned",
            (false, true) => "quarantined",
            _ => "conflict",
        };
        set(
            c,
            r,
            i,
            state,
            "Reconciled an interrupted operation by file identity and SHA-256",
        )?;
    }
    Ok(())
}
fn move_one(c: &Connection, r: &mut Review, i: usize, op: &str, vault: &Path) -> Result<()> {
    let item = &r.files[i];
    let restoring = op == "restore";
    ensure!(
        if restoring {
            item.state == "quarantined"
        } else {
            matches!(item.state.as_str(), "planned" | "restored")
        },
        "Item is ineligible in state {}",
        item.state
    );
    let _parents = parent_guards(&r.root, item)?;
    let slot = vault.join(format!("{}.bin", item.id));
    let (from, to) = if restoring {
        (slot, item.path.clone())
    } else {
        (item.path.clone(), slot)
    };
    ensure!(
        matches!(fs::symlink_metadata(&to),Err(e) if e.kind()==io::ErrorKind::NotFound),
        "Destination already exists or cannot be inspected; both files were preserved"
    );
    let mut file = open_regular(&from, true)?;
    if !restoring {
        ensure!(
            class(&from, &mut file) == Some(item.rule.as_str()),
            "This file no longer meets the reviewed cache rule"
        );
    }
    attest(&mut file, item)?;
    let prior = item.state.clone();
    set(
        c,
        r,
        i,
        if restoring {
            "restoring"
        } else {
            "quarantining"
        },
        "Approved exact file; verified immediately before rename",
    )?;
    if let Err(e) = rename(&file, &to) {
        set(c, r, i, &prior, &e.to_string())?;
        return Err(e);
    }
    let final_state = if restoring { "restored" } else { "quarantined" };
    set(
        c,
        r,
        i,
        final_state,
        "Moved the verified file without overwriting its destination",
    )
}
pub(super) fn operate(c: &Connection, id: &str, op: Option<(&str, &[&str])>) -> Result<Value> {
    ensure!(valid_id(id), "Invalid review ID");
    if op.is_none() {
        let prior = load(c, id)?;
        if prior
            .files
            .iter()
            .all(|f| !matches!(f.state.as_str(), "quarantining" | "restoring"))
        {
            return Ok(serde_json::to_value(prior)?);
        }
    }
    let _lock = lock(c, id)?;
    let mut r = load(c, id)?;
    let pending = r
        .files
        .iter()
        .any(|i| matches!(i.state.as_str(), "quarantining" | "restoring"));
    if op.is_none() && !pending {
        return Ok(serde_json::to_value(r)?);
    }
    let root = checked_dirs(&r.root)?;
    ensure!(
        identity(root.last().context("Missing review directory")?)?.0 == r.root_identity,
        "Original review folder identity changed"
    );
    let chosen = if let Some((verb, ids)) = op {
        let mut indices = vec![];
        for &id in ids {
            let i = r
                .files
                .iter()
                .position(|f| f.id == id)
                .context("Selected item is outside this review")?;
            indices.push(i);
        }
        Some((verb, indices))
    } else {
        None
    };
    let (v, _guards) = vault(
        c,
        &mut r,
        chosen.as_ref().is_some_and(|(op, _)| *op == "quarantine"),
    )?;
    recover(c, &mut r, &v)?;
    let mut ok = 0;
    let mut failed = 0;
    if let Some((op, indices)) = chosen {
        for i in indices {
            match move_one(c, &mut r, i, op, &v) {
                Ok(()) => ok += 1,
                Err(e) => {
                    let state = r.files[i].state.clone();
                    set(c, &mut r, i, &state, &e.to_string())?;
                    failed += 1
                }
            }
        }
    }
    let mut out = serde_json::to_value(&r)?;
    out["result"] = json!(format!("{ok} moved, {failed} preserved"));
    out["moved"] = json!(ok);
    out["preserved"] = json!(failed);
    Ok(out)
}
