use crate::{Result, catalog::Catalog, expect, files, invalid, registry::Op, unknown};
use serde::Deserialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::path::Path;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Empty {}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Search {
    query: String,
    #[serde(default = "default_limit")]
    limit: usize,
}
fn default_limit() -> usize {
    50
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Inspect {
    id: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Text {
    text: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FileInput {
    path: String,
}
fn args<T: serde::de::DeserializeOwned>(value: Value) -> Result<T> {
    expect(value.is_object(), "Operation input must be an object.")?;
    serde_json::from_value(value)
        .map_err(|_| invalid("Input does not match this operation's schema."))
}
pub fn run(catalog: &Catalog, id: &str, input: Value) -> Result<Value> {
    let op = Op::find(id).ok_or_else(unknown)?;
    match op {
        Op::Search => {
            let q: Search = args(input)?;
            catalog.search(&q.query, q.limit)
        }
        Op::Inspect => {
            let q: Inspect = args(input)?;
            catalog.inspect(&q.id)
        }
        Op::HashText => {
            let q: Text = args(input)?;
            expect(q.text.len() <= 8192, "Text exceeds 8 KiB.")?;
            Ok(
                json!({"bytes":q.text.len(),"sha256":format!("{:x}",Sha256::digest(q.text.as_bytes()))}),
            )
        }
        Op::HashFile => {
            let q: FileInput = args(input)?;
            let p = Path::new(&q.path);
            expect(p.is_absolute(), "Select an absolute local file.")?;
            let (bytes, digest) = files::digest_file(p)?;
            Ok(json!({"bytes":bytes,"sha256":digest}))
        }
        _ => {
            let _: Empty = args(input)?;
            native(op)
        }
    }
}
#[cfg(not(all(windows, target_arch = "x86_64")))]
fn native(_: Op) -> Result<Value> {
    Err(crate::err(
        "unavailable",
        "This operation requires a Windows x64 host.",
    ))
}
#[cfg(all(windows, target_arch = "x86_64"))]
fn native(op: Op) -> Result<Value> {
    match op {
        Op::Memory => windows::memory(),
        Op::Power => windows::power(),
        Op::Startup => windows::startup(),
        _ => Err(unknown()),
    }
}
#[cfg(all(windows, target_arch = "x86_64"))]
mod windows {
    use super::*;
    use crate::err;
    use std::collections::BTreeSet;
    use std::io::ErrorKind;
    use std::ptr::{null, null_mut};
    use windows_sys::{
        Win32::{
            Foundation::{ERROR_NO_MORE_ITEMS, LocalFree},
            System::{
                Power::{ACCESS_SCHEME, PowerEnumerate, PowerGetActiveScheme},
                SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX},
            },
        },
        core::GUID,
    };
    use winreg::{HKCU, HKLM, enums::*};
    pub(super) fn memory() -> Result<Value> {
        let mut data = MEMORYSTATUSEX {
            dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as u32,
            ..Default::default()
        };
        if unsafe { GlobalMemoryStatusEx(&mut data) } == 0
            || data.ullTotalPhys == 0
            || data.ullAvailPhys > data.ullTotalPhys
            || data.dwMemoryLoad > 100
        {
            return Err(err(
                "unavailable",
                "Windows did not supply a complete memory observation.",
            ));
        }
        Ok(
            json!({"totalPhysicalBytes":data.ullTotalPhys,"availablePhysicalBytes":data.ullAvailPhys,"loadPercent":data.dwMemoryLoad}),
        )
    }
    fn guid(g: &GUID) -> String {
        let d = g.data4;
        format!(
            "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            g.data1, g.data2, g.data3, d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7]
        )
    }
    fn active() -> Result<String> {
        let mut data = null_mut();
        let code = unsafe { PowerGetActiveScheme(null_mut(), &mut data) };
        if code != 0 || data.is_null() {
            if !data.is_null() {
                unsafe {
                    LocalFree(data.cast());
                }
            }
            return Err(err(
                "unavailable",
                "The active Windows power plan could not be read.",
            ));
        }
        let id = unsafe {
            let id = guid(&*data);
            LocalFree(data.cast());
            id
        };
        Ok(id)
    }
    pub(super) fn power() -> Result<Value> {
        let before = active()?;
        let mut ids = BTreeSet::new();
        for index in 0..=256 {
            let mut g = GUID::default();
            let mut size = std::mem::size_of::<GUID>() as u32;
            let result = unsafe {
                PowerEnumerate(
                    null_mut(),
                    null(),
                    null(),
                    ACCESS_SCHEME,
                    index,
                    (&mut g as *mut GUID).cast(),
                    &mut size,
                )
            };
            if result == ERROR_NO_MORE_ITEMS {
                break;
            }
            if index == 256 || result != 0 || size != 16 || !ids.insert(guid(&g)) {
                return Err(err("incomplete", "Power plan enumeration was incomplete."));
            }
        }
        if !ids.contains(&before) || active()? != before {
            return Err(err(
                "incomplete",
                "The active plan changed or was missing during enumeration.",
            ));
        }
        Ok(json!({"activeId":before,"planIds":ids,"count":ids.len()}))
    }
    pub(super) fn startup() -> Result<Value> {
        let mut sources = Vec::new();
        let mut total = 0u64;
        for key in ["Run", "RunOnce"] {
            let path = format!(r"Software\Microsoft\Windows\CurrentVersion\{key}");
            for (scope, view, hive, flags) in [
                ("current-user", "shared", &HKCU, 0),
                ("machine", "64-bit", &HKLM, KEY_WOW64_64KEY),
                ("machine", "32-bit", &HKLM, KEY_WOW64_32KEY),
            ] {
                let (state, registered) = match hive
                    .open_subkey_with_flags(&path, KEY_QUERY_VALUE | flags)
                {
                    Ok(k) => (
                        "read",
                        k.query_info()
                            .map_err(|_| {
                                err("incomplete", "A startup registry count could not be read.")
                            })?
                            .values,
                    ),
                    Err(e) if e.kind() == ErrorKind::NotFound => ("not-present", 0),
                    Err(_) => {
                        return Err(err("incomplete", "A startup registry view is unavailable."));
                    }
                };
                total += u64::from(registered);
                sources.push(json!({"scope":scope,"view":view,"key":key,"state":state,"registered":registered}));
            }
        }
        Ok(json!({"sources":sources,"registeredValues":total}))
    }
}
