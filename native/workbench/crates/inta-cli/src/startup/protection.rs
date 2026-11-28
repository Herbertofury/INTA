use super::{Phase, Raw, supported};
use anyhow::{Result, bail, ensure};
use std::ptr;
use windows_sys::Win32::{
    Foundation::LocalFree,
    Security::Cryptography::{
        CRYPT_INTEGER_BLOB, CRYPTPROTECT_UI_FORBIDDEN, CryptProtectData, CryptUnprotectData,
    },
};
use winreg::{RegValue, enums::*};

fn crypt(bytes: &[u8], id: &str, seal: bool) -> Result<Vec<u8>> {
    let block = |data: &[u8]| CRYPT_INTEGER_BLOB {
        cbData: data.len() as u32,
        pbData: data.as_ptr().cast_mut(),
    };
    let data = block(bytes);
    let entropy = block(id.as_bytes());
    let mut output = CRYPT_INTEGER_BLOB::default();
    // Both calls allocate with LocalAlloc. The buffers and entry-specific entropy outlive the call.
    unsafe {
        let ok = if seal {
            CryptProtectData(
                &data,
                ptr::null(),
                &entropy,
                ptr::null(),
                ptr::null(),
                CRYPTPROTECT_UI_FORBIDDEN,
                &mut output,
            )
        } else {
            CryptUnprotectData(
                &data,
                ptr::null_mut(),
                &entropy,
                ptr::null(),
                ptr::null(),
                CRYPTPROTECT_UI_FORBIDDEN,
                &mut output,
            )
        };
        ensure!(
            ok != 0,
            "Windows could not verify or protect the current-user recovery record"
        );
        let result = if output.cbData == 0 {
            Vec::new()
        } else {
            ensure!(
                !output.pbData.is_null(),
                "Windows returned an invalid protection buffer"
            );
            std::slice::from_raw_parts(output.pbData, output.cbData as usize).to_vec()
        };
        if !output.pbData.is_null() {
            LocalFree(output.pbData.cast());
        }
        Ok(result)
    }
}
pub(super) fn seal(id: &str, phase: Phase, raw: &Raw) -> Result<Raw> {
    ensure!(supported(raw), "Unsupported registration bytes");
    let mut bytes = vec![phase as u8, raw.vtype.clone() as u8];
    bytes.extend_from_slice(&raw.bytes);
    Ok(RegValue {
        vtype: REG_BINARY,
        bytes: crypt(&bytes, id, true)?.into(),
    })
}
pub(super) fn open(id: &str, protected: &Raw) -> Result<(Phase, Raw)> {
    ensure!(
        protected.vtype == REG_BINARY && protected.bytes.len() <= 131_072,
        "Invalid startup recovery record"
    );
    let mut bytes = crypt(&protected.bytes, id, false)?;
    let phase = match bytes.first() {
        Some(1) => Phase::Prepared,
        Some(2) => Phase::Held,
        Some(3) => Phase::Restoring,
        _ => bail!("Invalid recovery phase"),
    };
    let vtype = match bytes.get(1) {
        Some(1) => REG_SZ,
        Some(2) => REG_EXPAND_SZ,
        _ => bail!("Unknown recovery value type"),
    };
    let value = RegValue {
        vtype,
        bytes: bytes.split_off(2).into(),
    };
    ensure!(supported(&value), "Invalid saved registration format");
    Ok((phase, value))
}
