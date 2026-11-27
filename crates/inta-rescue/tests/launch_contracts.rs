use inta_rescue::{action, assets, launch, launch_arguments, ACTIONS};
use std::collections::HashSet;
#[test]
fn preserves_all_fourteen_launchers_and_embedded_official_brands() {
    assert_eq!(ACTIONS.len(), 14);
    assert_eq!(
        ACTIONS.iter().map(|a| a.id).collect::<HashSet<_>>().len(),
        14
    );
    assert!(assets::self_test().unwrap() >= 350);
}
#[test]
fn disk_and_recovery_commands_keep_explicit_targets_inside_the_tool() {
    assert_eq!(
        launch_arguments(action("disks").unwrap(), None).unwrap().1,
        [
            "--hold",
            "--execute",
            "lsblk",
            "-e7",
            "-o",
            "NAME,SIZE,MODEL,FSTYPE,RO,MOUNTPOINTS"
        ]
    );
    assert_eq!(
        launch_arguments(action("photorec").unwrap(), None)
            .unwrap()
            .1,
        ["--hold", "--execute", "sudo", "photorec"]
    );
    assert_eq!(
        launch_arguments(action("clonezilla").unwrap(), None)
            .unwrap()
            .1,
        ["--hold", "--execute", "sudo", "clonezilla"]
    );
}
#[test]
fn scan_requires_directory_and_never_requests_removal() {
    assert!(launch_arguments(action("scan").unwrap(), None).is_err());
    let folder = std::env::temp_dir();
    let (_, args) = launch_arguments(action("scan").unwrap(), Some(&folder)).unwrap();
    assert_eq!(
        &args[..6],
        [
            "--hold",
            "--execute",
            "clamscan",
            "--recursive",
            "--infected",
            "--"
        ]
    );
    assert_eq!(args[6], folder.to_string_lossy());
    assert!(!args.iter().any(|s| s.contains("remove")));
}
#[test]
fn ai_requires_confirmation_and_preserves_codex_read_only_sandbox() {
    assert!(launch(action("codex").unwrap(), None, false).is_err());
    assert!(launch(action("claude").unwrap(), None, false).is_err());
    let (_, args) = launch_arguments(action("codex").unwrap(), None).unwrap();
    assert_eq!(&args[args.len() - 2..], ["--sandbox", "read-only"]);
}
