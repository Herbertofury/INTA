use super::{err, Answer};
use serde_json::json;
use std::{io, path::Path};

pub(super) enum Operation {
    Test,
    Create,
    Restore,
}

pub(super) fn test(exe: &Path, cwd: &Path) -> Answer {
    run(exe, cwd, Operation::Test)
}

#[cfg(windows)]
pub(super) fn run(exe: &Path, cwd: &Path, operation: Operation) -> Answer {
    use std::{
        mem::size_of,
        os::windows::{
            ffi::OsStrExt,
            io::{AsRawHandle, FromRawHandle, OwnedHandle},
        },
        ptr::null,
    };
    use windows_sys::Win32::{
        Foundation::{HANDLE, WAIT_OBJECT_0, WAIT_TIMEOUT},
        System::{
            JobObjects::{
                AssignProcessToJobObject, CreateJobObjectW, JobObjectExtendedLimitInformation,
                SetInformationJobObject, JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
                JOB_OBJECT_LIMIT_ACTIVE_PROCESS, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
                JOB_OBJECT_LIMIT_PROCESS_MEMORY,
            },
            SystemInformation::GetSystemWindowsDirectoryW,
            Threading::{
                CreateProcessW, GetExitCodeProcess, ResumeThread, TerminateProcess,
                WaitForSingleObject, CREATE_NO_WINDOW, CREATE_SUSPENDED,
                CREATE_UNICODE_ENVIRONMENT, PROCESS_INFORMATION, STARTF_USESTDHANDLES,
                STARTUPINFOW,
            },
        },
    };
    fn owned(handle: HANDLE) -> Result<OwnedHandle, String> {
        if handle.is_null() {
            Err(err(io::Error::last_os_error()))
        } else {
            Ok(unsafe { OwnedHandle::from_raw_handle(handle) })
        }
    }
    let job = owned(unsafe { CreateJobObjectW(null(), null()) })?;
    let mut limits = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
    limits.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE
        | JOB_OBJECT_LIMIT_ACTIVE_PROCESS
        | JOB_OBJECT_LIMIT_PROCESS_MEMORY;
    limits.BasicLimitInformation.ActiveProcessLimit = 1;
    limits.ProcessMemoryLimit = 512 * 1024 * 1024;
    if unsafe {
        SetInformationJobObject(
            job.as_raw_handle(),
            JobObjectExtendedLimitInformation,
            (&limits as *const JOBOBJECT_EXTENDED_LIMIT_INFORMATION).cast(),
            size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
        )
    } == 0
    {
        return Err(err(io::Error::last_os_error()));
    }
    let exe: Vec<_> = exe.as_os_str().encode_wide().chain(Some(0)).collect();
    let cwd: Vec<_> = cwd.as_os_str().encode_wide().chain(Some(0)).collect();
    let command = match operation {
        Operation::Test => r"7zr.exe t -t7z -mmt1 -bd -bso0 -bse0 -bsp0 -- input.7z",
        Operation::Create => {
            r"7zr.exe a -t7z -mx1 -mmt1 -mtc=off -mta=off -mtm=off -bd -bso0 -bse0 -bsp0 -- ..\input.7z *"
        }
        Operation::Restore => {
            r"7zr.exe x -t7z -mmt1 -y -orestored -bd -bso0 -bse0 -bsp0 -- input.7z"
        }
    };
    let mut cmd: Vec<u16> = command.encode_utf16().chain(Some(0)).collect();
    let mut sysdir = vec![0u16; 32768];
    let len =
        unsafe { GetSystemWindowsDirectoryW(sysdir.as_mut_ptr(), sysdir.len() as u32) } as usize;
    if len == 0 || len >= sysdir.len() {
        return Err("Unable to resolve Windows directory".into());
    }
    let mut environment: Vec<u16> = "SystemRoot=".encode_utf16().collect();
    environment.extend_from_slice(&sysdir[..len]);
    environment.extend_from_slice(&[0, 0]);
    let startup = STARTUPINFOW {
        cb: size_of::<STARTUPINFOW>() as u32,
        dwFlags: STARTF_USESTDHANDLES,
        ..Default::default()
    };
    let mut info = PROCESS_INFORMATION::default();
    // Start suspended: resource limits apply before any archive bytes enter the parser.
    if unsafe {
        CreateProcessW(
            exe.as_ptr(),
            cmd.as_mut_ptr(),
            null(),
            null(),
            0,
            CREATE_SUSPENDED | CREATE_NO_WINDOW | CREATE_UNICODE_ENVIRONMENT,
            environment.as_ptr().cast(),
            cwd.as_ptr(),
            &startup,
            &mut info,
        )
    } == 0
    {
        return Err(err(io::Error::last_os_error()));
    }
    let process = owned(info.hProcess)?;
    let thread = owned(info.hThread)?;
    if unsafe { AssignProcessToJobObject(job.as_raw_handle(), process.as_raw_handle()) } == 0 {
        let e = io::Error::last_os_error();
        unsafe {
            TerminateProcess(process.as_raw_handle(), 1);
            WaitForSingleObject(process.as_raw_handle(), 2000);
        }
        return Err(format!("Could not constrain the archive worker: {e}"));
    }
    if unsafe { ResumeThread(thread.as_raw_handle()) } == u32::MAX {
        return Err(err(io::Error::last_os_error()));
    }
    match unsafe { WaitForSingleObject(process.as_raw_handle(), 30000) } {
        WAIT_TIMEOUT => {
            unsafe {
                TerminateProcess(process.as_raw_handle(), 1);
                WaitForSingleObject(process.as_raw_handle(), 2000);
            }
            Err("Archive test exceeded its 30-second limit".into())
        }
        WAIT_OBJECT_0 => {
            let mut code = 0;
            if unsafe { GetExitCodeProcess(process.as_raw_handle(), &mut code) } == 0 {
                return Err(err(io::Error::last_os_error()));
            }
            match code {
                0 => Ok(json!(code)),
                8 => Err("Archive test exceeded its 512 MiB memory limit".into()),
                n => match operation {
                    Operation::Test|Operation::Restore => Err(format!("Archive could not be verified (7-Zip exit {n}); damaged, split or password-protected archives may fail.")),
                    Operation::Create => Err(format!("Archive creation failed (7-Zip exit {n})")),
                },
            }
        }
        _ => Err(err(io::Error::last_os_error())),
    }
}
#[cfg(not(windows))]
pub(super) fn run(_: &Path, _: &Path, _: Operation) -> Answer {
    Err("Archive test requires Windows".into())
}
