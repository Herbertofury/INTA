# Native Windows repair lab

The INTA CLI and desktop use Rust to start and supervise an isolated x64 UEFI guest on Windows. The graphical backend is the pinned native Windows build of QEMU 11.1.3. A separate Rust VMM integration provides an additional repair-test path. The QEMU backend runs as the logged-in user. An operating system booted from the selected ISO supplies the guest console and its guest privileges.

## Offline Rust guest on this workstation

In **Virtual machines**, choose **Isolated repair test** for a verified recovery exercise or **Rust rescue console** for an interactive Linux root shell. The prepared Debian WSL2 distribution runs the pinned Cloud Hypervisor v55.0 engine through KVM. It uses a dedicated unprivileged Linux host account, two virtual processors, 2 GiB of guest RAM, and zero virtual network devices. Windows runs under the signed-in account. Starting either guest works offline.

The automated test creates a damaged FAT volume, clones it using ddrescue, repairs the clone and verifies the recovered file and original disk by SHA-256. The console can attach a regular, sector-aligned raw .img file up to 8 GiB. Guest writes land on a disposable private copy at /dev/vdc. With no selected image, the guest gets a blank 256 MiB scratch disk. Exit the shell to power off and discard the scratch disk.

```powershell
inta vm repair-test
inta vm console --approve --base 'I:\Lab inputs\disk.img'
```

The Cloud Hypervisor bridge is available on machines with the prepared Debian lab. The native graphical profile below also supports PCs without that WSL2 environment. The portable kit contains local launchers, an ISO and verified runtime components. The development owner's private use and third-party distribution conditions are documented with the kit.

## Use the graphical lab

From **Virtual machines**, choose a local boot ISO. For a repair case, also select a regular raw test disk image ending in `.img`. INTA copies and hashes the selected base, verifies the snapshot by reading it back, and presents the guest with a writable qcow2 overlay. The source file and session base remain separate. Without a base image the guest receives an empty thin-provisioned 64 GiB disk.

```powershell
inta vm status
inta vm plan 'I:\Media\INTA-rescue.iso' --base 'I:\Lab inputs\damaged.img'
inta vm start 'I:\Media\INTA-rescue.iso' --base 'I:\Lab inputs\damaged.img' --approve
inta vm sessions
inta vm stop SESSION-UUID --approve
```

The ISO must be a regular, local ISO9660 or UDF file. A raw base must be sector aligned and between 512 bytes and 8 GiB. The guest has 2 virtual CPUs, 4 GiB RAM, a local SDL display and private UEFI variables. WHPX hardware acceleration is attempted first. Early accelerator initialization failures fall back to TCG emulation. The default profile has no guest network device. Add `--nat` explicitly when the guest needs outbound user networking. NAT has no inbound port forwards. Guest-initiated connections may reach local and external network services. Physical media, USB passthrough, shared directories, host shell services, VNC and remote monitors are outside the profile.

Interactive INTA sessions retain `disk.qcow2`, private firmware variables, a bounded diagnostic log, and any `base.raw` in the session directory. Powering off from INTA immediately stops the disposable guest. Shut down within the guest OS for a clean filesystem shutdown. Only the supervisor that owns the session can receive its named power-off event. Stopping the CLI or closing the desktop leaves the guest running under its own supervisor. If the supervisor dies, Windows terminates the associated QEMU job. Session logs and disks may contain sensitive lab data. Store them on an access-controlled local volume.

The default lab location is `INTA VM Labs` beside the selected toolbox. Set `INTA_VM_LAB_DIR` to change it. The parent directory must exist. The plan compares free space with the maximum virtual disk capacity plus temporary overhead. Concurrent workloads can change available free space after that check. Session files stay until their owner handles retention.

## Reproduce the native runtime

```powershell
python vm-lab/acquire.py --root 'I:\INTA Toolbox'
python vm-lab/acquire.py --root 'I:\INTA Toolbox' --offline
```

The Windows build is from the MSYS2 UCRT64 package family linked by QEMU's official download page. The acquisition lock records the specific published package, version, size, SHA-256, declared license and signing fingerprint. During initial selection the upstream detached package signatures validated against the pinned MSYS2 keyring. The acquisition script validates every archive and selected member against that frozen set. Python 3.14 supports zstd natively. For a fresh extraction with Python 3.11–3.13 install `zstandard==0.25.0` in the maintenance environment. A toolbox already populated can be verified without this dependency. [QEMU downloads](https://www.qemu.org/download/) · [MSYS2 UCRT package](https://packages.msys2.org/packages/mingw-w64-ucrt-x86_64-qemu) · [Pinned keyring](https://github.com/msys2/MSYS2-keyring/tree/5d249077a0cfeec9dbf51515e5e01da413d28859)

The 121-file executable, DLL and firmware inventory is approximately 146 MiB and resolves x64 platform dependencies from 89 packages. A closed list of executable/firmware files, checked and held open by the Rust supervisor, governs startup. The offline cache used for source review holds additional packages. All binaries live outside the application installer. The initial full QEMU v11.1.3 upstream clone was pinned at `005c61c0ef6f6ae615283440b4bb7be51b2c6ddd`. The runtime carries 108 selected upstream notice files. QEMU contains GPL and other file-specific components. MSYS2 DLLs and firmware have separate terms. The two lockfiles and notices give attribution and package identities for local use. Public redistribution requires the owner to resolve complete corresponding-source and third-party obligations. [QEMU source and licensing](https://github.com/qemu/qemu/tree/005c61c0ef6f6ae615283440b4bb7be51b2c6ddd) · [Runtime pins](runtime-lock.json) · [MSYS2 package and notice pins](msys2-lock.json)

## Native repair proof

On Windows 11 x64, the native QEMU process booted the project's INTA Rescue optical image under WHPX, and the owning supervisor accepted the power-off request. A separate full guest-side test booted the UEFI fixture with networking off and a disposable copy of a deliberately broken FAT12 disk. The input was unreadable through mtools. Guest code repaired the bytes-per-sector field in the qcow2 overlay. The source and copied base remained byte-identical. Only byte 12 changed in the repaired virtual disk and the original file was readable with its expected SHA-256.

The fixture source requires all identifying fields and the known synthetic defect before writing. The proof script requires Visual C++ build tools and a named WSL distribution with mtools and xorriso for preparing and checking the fixture. The actual guest runs in the native Windows QEMU process.

```powershell
cargo test -p inta-vm-lab --locked
python vm-lab/tests/native-proof.py --cli target/debug/inta.exe --toolbox 'I:\INTA Toolbox' --work 'I:\INTA\vm-build\new-proof'
```

The evidence directory must be new. The script checks the original source hash, session snapshot bytes, exact modified byte and recovered payload, then writes `proof.json`. The validation covers this host, QEMU version and synthetic filesystem repair. It does not certify arbitrary guest operating systems, TPM passthrough, Secure Boot, Windows Setup or malware containment.
