pub mod assets;
pub mod capture;
pub mod ui;
use std::{
    env, fs, io,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Group {
    Disks,
    System,
    Ai,
}
impl Group {
    pub fn label(self) -> &'static str {
        match self {
            Self::Disks => "Disks & recovery",
            Self::System => "System & security",
            Self::Ai => "AI workspace",
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Action {
    pub id: &'static str,
    pub title: &'static str,
    pub brand: &'static str,
    pub purpose: &'static str,
    pub label: &'static str,
    pub group: Group,
    pub program: &'static str,
    pub arguments: &'static [&'static str],
    pub terminal: bool,
    pub online_confirmation: bool,
}

pub const ACTIONS: &[Action] = &[
    Action {
        id: "disks",
        title: "Disk inventory",
        brand: "lsblk",
        purpose: "List device size, model, filesystem, read-only state and mount points.",
        label: "List devices",
        group: Group::Disks,
        program: "lsblk",
        arguments: &["-e7", "-o", "NAME,SIZE,MODEL,FSTYPE,RO,MOUNTPOINTS"],
        terminal: true,
        online_confirmation: false,
    },
    Action {
        id: "smart",
        title: "GSmartControl",
        brand: "GSmartControl",
        purpose: "Inspect drive health and SMART attributes before repair writes.",
        label: "Inspect health",
        group: Group::Disks,
        program: "gsmartcontrol",
        arguments: &[],
        terminal: false,
        online_confirmation: false,
    },
    Action {
        id: "partitions",
        title: "GParted",
        brand: "GParted",
        purpose: "Inspect partitions. Review the exact device before applying any changes.",
        label: "Open editor",
        group: Group::Disks,
        program: "gparted",
        arguments: &[],
        terminal: false,
        online_confirmation: false,
    },
    Action {
        id: "testdisk",
        title: "TestDisk",
        brand: "TestDisk",
        purpose: "Inspect and recover disk partitions with an explicit target.",
        label: "Open recovery",
        group: Group::Disks,
        program: "sudo",
        arguments: &["testdisk"],
        terminal: true,
        online_confirmation: false,
    },
    Action {
        id: "photorec",
        title: "PhotoRec",
        brand: "PhotoRec",
        purpose: "Recover files to a separate destination; keep the source untouched.",
        label: "Recover files",
        group: Group::Disks,
        program: "sudo",
        arguments: &["photorec"],
        terminal: true,
        online_confirmation: false,
    },
    Action {
        id: "clonezilla",
        title: "Clonezilla",
        brand: "Clonezilla",
        purpose: "Save or restore images. Confirm source and destination inside the tool.",
        label: "Open imaging",
        group: Group::Disks,
        program: "sudo",
        arguments: &["clonezilla"],
        terminal: true,
        online_confirmation: false,
    },
    Action {
        id: "hardware",
        title: "Hardware inventory",
        brand: "inxi",
        purpose: "Inspect CPU, graphics, storage and network details with addresses redacted.",
        label: "Inspect hardware",
        group: Group::System,
        program: "inxi",
        arguments: &["-F", "-z"],
        terminal: true,
        online_confirmation: false,
    },
    Action {
        id: "network",
        title: "Network inventory",
        brand: "iproute2",
        purpose: "Show local interfaces and assigned addresses. No network scan runs.",
        label: "Inspect network",
        group: Group::System,
        program: "ip",
        arguments: &["-brief", "address"],
        terminal: true,
        online_confirmation: false,
    },
    Action {
        id: "signatures",
        title: "ClamAV signatures",
        brand: "ClamAV",
        purpose: "Download current detection signatures for this live session.",
        label: "Update signatures",
        group: Group::System,
        program: "sudo",
        arguments: &["freshclam"],
        terminal: true,
        online_confirmation: false,
    },
    Action {
        id: "scan",
        title: "ClamAV scan",
        brand: "ClamAV",
        purpose: "Choose a folder to scan. Report infected files without automatic removal.",
        label: "Choose folder",
        group: Group::System,
        program: "clamscan",
        arguments: &["--recursive", "--infected", "--"],
        terminal: true,
        online_confirmation: false,
    },
    Action {
        id: "terminal",
        title: "Terminal tools",
        brand: "XFCE",
        purpose: "Use ddrescue, YARA, Sleuth Kit, dislocker and tcpdump with an explicit target.",
        label: "Open terminal",
        group: Group::System,
        program: "xfce4-terminal",
        arguments: &[],
        terminal: false,
        online_confirmation: false,
    },
    Action {
        id: "chatgpt",
        title: "ChatGPT",
        brand: "ChatGPT",
        purpose: "Open the provider in Firefox. Internet access and sign-in may be required.",
        label: "Open browser",
        group: Group::Ai,
        program: "firefox-esr",
        arguments: &["--new-window", "https://chatgpt.com/"],
        terminal: false,
        online_confirmation: false,
    },
    Action {
        id: "codex",
        title: "Codex CLI",
        brand: "Codex CLI",
        purpose: "Start in a private work folder using the read-only shell sandbox.",
        label: "Start client",
        group: Group::Ai,
        program: "/opt/inta-ai/node_modules/.bin/codex",
        arguments: &["--sandbox", "read-only"],
        terminal: true,
        online_confirmation: true,
    },
    Action {
        id: "claude",
        title: "Claude Code",
        brand: "Claude Code",
        purpose: "Start in a private work folder with the client's default permission prompts.",
        label: "Start client",
        group: Group::Ai,
        program: "/opt/inta-ai/node_modules/.bin/claude",
        arguments: &[],
        terminal: true,
        online_confirmation: true,
    },
];

pub fn action(id: &str) -> Option<&'static Action> {
    ACTIONS.iter().find(|a| a.id == id)
}
pub fn available(name: &str) -> bool {
    let path = Path::new(name);
    if path.is_absolute() {
        return path.is_file();
    }
    env::split_paths(&env::var_os("PATH").unwrap_or_default())
        .any(|folder| folder.join(name).is_file())
}
pub fn installed(a: &Action) -> bool {
    available(a.program)
        && (!a.terminal || available("xfce4-terminal"))
        && (a.program != "sudo" || a.arguments.first().is_some_and(|tool| available(tool)))
}
pub fn launch_arguments(a: &Action, directory: Option<&Path>) -> io::Result<(String, Vec<String>)> {
    let mut arguments: Vec<String> = a.arguments.iter().map(|s| (*s).into()).collect();
    if a.id == "scan" {
        let path = directory.ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "Choose a directory to scan.")
        })?;
        if !path.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "The scan target must be an existing directory.",
            ));
        }
        arguments.push(path.to_string_lossy().into_owned());
    }
    if a.terminal {
        let mut terminal = vec!["--hold".into(), "--execute".into(), a.program.into()];
        terminal.extend(arguments);
        Ok(("xfce4-terminal".into(), terminal))
    } else {
        Ok((a.program.into(), arguments))
    }
}
pub fn launch(a: &Action, directory: Option<&Path>, online_confirmed: bool) -> io::Result<u32> {
    if a.online_confirmation && !online_confirmed {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Confirm the online AI session before sharing content.",
        ));
    }
    let (program, arguments) = launch_arguments(a, directory)?;
    let mut command = Command::new(program);
    command.args(arguments);
    if a.online_confirmation {
        let folder = PathBuf::from(env::var_os("HOME").ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "The live user's home directory is unavailable.",
            )
        })?)
        .join("INTA-AI")
        .join(a.id);
        fs::create_dir_all(&folder)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&folder, fs::Permissions::from_mode(0o700))?;
        }
        command.current_dir(folder);
    }
    Ok(command.spawn()?.id())
}
