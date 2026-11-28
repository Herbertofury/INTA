use super::{err, guarded_dirs, read_file, session_id, wide, win_dir, R};
use std::{
    collections::BTreeMap,
    ffi::{OsStr, OsString},
    io,
    os::windows::{
        ffi::OsStrExt,
        io::{AsRawHandle, FromRawHandle, OwnedHandle},
    },
    path::Path,
    ptr,
};
use windows_sys::Win32::{
    Foundation::{CloseHandle, WAIT_OBJECT_0, WAIT_TIMEOUT},
    System::Threading::{
        CreateProcessW, GetExitCodeProcess, WaitForSingleObject, CREATE_NO_WINDOW,
        CREATE_UNICODE_ENVIRONMENT, PROCESS_INFORMATION, STARTUPINFOW,
    },
};

pub(super) struct Supervisor(OwnedHandle);

impl Supervisor {
    pub(super) fn start(toolbox: &Path, root: &Path, id: &str) -> R<Self> {
        let id = session_id(id)?;
        let system = win_dir()?.into_os_string();
        let mut env = BTreeMap::from([
            ("INTA_TOOLBOX", toolbox.as_os_str().to_owned()),
            ("INTA_VM_LAB_DIR", root.as_os_str().to_owned()),
            ("INTA_VM_WORKER_ID", OsString::from(id)),
            ("SYSTEMROOT", system.clone()),
            ("WINDIR", system),
        ]);
        for key in [
            "INTA_DATA_DIR",
            "LOCALAPPDATA",
            "TEMP",
            "TMP",
            "USERPROFILE",
        ] {
            if let Some(value) = std::env::var_os(key) {
                env.insert(key, value);
            }
        }
        Self::spawn_current(&format!("inta.exe vm supervise {id}"), &env)
    }

    fn spawn_current(args: &str, env: &BTreeMap<&str, OsString>) -> R<Self> {
        let exe = std::env::current_exe().map_err(err)?;
        let dir = exe.parent().ok_or("Executable directory unavailable")?;
        let _dirs = guarded_dirs(dir)?;
        let _exe = read_file(&exe)?;
        // Win32 requires a case-insensitively sorted, double-NUL-terminated Unicode block.
        // All keys are internal uppercase ASCII. Values remain data, including spaces and '='.
        let mut block = Vec::<u16>::new();
        for (key, value) in env {
            block.extend(key.encode_utf16());
            block.push(b'=' as u16);
            block.extend(value.encode_wide());
            block.push(0);
        }
        block.push(0);
        let executable = wide(exe.as_os_str());
        let directory = wide(dir.as_os_str());
        let mut command = wide(OsStr::new(args));
        let startup = STARTUPINFOW {
            cb: std::mem::size_of::<STARTUPINFOW>() as u32,
            ..Default::default()
        };
        let mut process = PROCESS_INFORMATION::default();
        // CreateProcessW may modify the command buffer. Keep every pointer live until it returns.
        // No handles, including duplicate pipe writers hidden by a bootstrap, cross this boundary.
        let created = unsafe {
            CreateProcessW(
                executable.as_ptr(),
                command.as_mut_ptr(),
                ptr::null(),
                ptr::null(),
                0,
                CREATE_NO_WINDOW | CREATE_UNICODE_ENVIRONMENT,
                block.as_ptr().cast(),
                directory.as_ptr(),
                &startup,
                &mut process,
            )
        };
        if created == 0 {
            return Err(format!(
                "Cannot start VM supervisor: {}",
                io::Error::last_os_error()
            ));
        }
        // Both returned handles belong to us; only the process handle is needed after creation.
        unsafe {
            CloseHandle(process.hThread);
            Ok(Self(OwnedHandle::from_raw_handle(process.hProcess)))
        }
    }

    pub(super) fn try_wait(&self) -> io::Result<Option<u32>> {
        let handle = self.0.as_raw_handle();
        // A signalled process can legitimately have exited with code 259 (STILL_ACTIVE).
        match unsafe { WaitForSingleObject(handle, 0) } {
            WAIT_TIMEOUT => Ok(None),
            WAIT_OBJECT_0 => {
                let mut code = 0;
                if unsafe { GetExitCodeProcess(handle, &mut code) } == 0 {
                    Err(io::Error::last_os_error())
                } else {
                    Ok(Some(code))
                }
            }
            _ => Err(io::Error::last_os_error()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::windows::event;
    use std::{io::Read, sync::mpsc, thread, time::Duration};
    use windows_sys::Win32::{
        Foundation::{DuplicateHandle, DUPLICATE_SAME_ACCESS},
        System::Threading::{GetCurrentProcess, SetEvent},
    };

    #[test]
    fn child_waits_for_release() {
        let Ok(id) = std::env::var("INTA_HANDLE_PROBE_READY") else {
            return;
        };
        let ready = event(&id, false).unwrap();
        let stop = event(&std::env::var("INTA_HANDLE_PROBE_STOP").unwrap(), false).unwrap();
        assert_ne!(unsafe { SetEvent(ready.as_raw_handle()) }, 0);
        assert_eq!(
            unsafe { WaitForSingleObject(stop.as_raw_handle(), 15000) },
            WAIT_OBJECT_0
        );
    }

    #[test]
    fn captured_pipe_closes_while_supervisor_with_no_inherited_handles_lives() {
        let ready_id = uuid::Uuid::new_v4().to_string();
        let stop_id = uuid::Uuid::new_v4().to_string();
        let ready = event(&ready_id, true).unwrap();
        let stop = event(&stop_id, true).unwrap();
        let (mut reader, writer) = io::pipe().unwrap();
        let mut copied = ptr::null_mut();
        let me = unsafe { GetCurrentProcess() };
        assert_ne!(
            unsafe {
                DuplicateHandle(
                    me,
                    writer.as_raw_handle(),
                    me,
                    &mut copied,
                    0,
                    1,
                    DUPLICATE_SAME_ACCESS,
                )
            },
            0
        );
        let duplicate = unsafe { OwnedHandle::from_raw_handle(copied) };
        let env = BTreeMap::from([
            ("INTA_HANDLE_PROBE_READY", OsString::from(ready_id)),
            ("INTA_HANDLE_PROBE_STOP", OsString::from(stop_id)),
            ("SYSTEMROOT", win_dir().unwrap().into_os_string()),
        ]);
        let child = Supervisor::spawn_current(
            "probe.exe --exact windows::detached::tests::child_waits_for_release",
            &env,
        )
        .unwrap();
        drop(writer);
        drop(duplicate);
        assert_eq!(
            unsafe { WaitForSingleObject(ready.as_raw_handle(), 10000) },
            WAIT_OBJECT_0
        );
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let _ = tx.send(reader.read(&mut [0; 1]));
        });
        let eof = rx.recv_timeout(Duration::from_secs(2));
        let running = child.try_wait().unwrap().is_none();
        assert_ne!(unsafe { SetEvent(stop.as_raw_handle()) }, 0);
        assert_eq!(
            unsafe { WaitForSingleObject(child.0.as_raw_handle(), 10000) },
            WAIT_OBJECT_0
        );
        assert_eq!(child.try_wait().unwrap(), Some(0));
        assert!(running, "probe exited before the EOF check");
        assert_eq!(
            eof.expect("child retained a non-stdio pipe handle")
                .unwrap(),
            0
        );
    }
}
