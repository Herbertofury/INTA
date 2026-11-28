# Local proof for <a href="https://herbertofury.github.io/INTA/" title="Open INTA official download or service page" aria-label="Open INTA official download or service page" data-inta-brand="app-inta"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/70ad9f0981b05341142b.png" alt="INTA official page" width="18" height="18" valign="middle" /></a> [INTA](https://herbertofury.github.io/INTA/) reconnect repair plans

This pilot is deliberately limited to two fixed read-only operations,
`powercfg:power-plans` and the optional, separately bound
`inta:startup-registration-count`. The second returns a count of the existing
Run and RunOnce registration names. It does not report names, paths, arguments,
or enabled state. A host without the second adapter refuses that step.
An ordered plan binds the device, pairing epoch, expiration, operation, effect,
specific target and catalog hash to a signed approval. A different approval,
changed target, unsupported operating system or ambiguous completion cannot
be treated as success. A persisted in-flight lease becomes Needs Review after
restart rather than being repeated. Successful steps survive process reopen.

`tests/INTA.RepairQueue.Proof` uses generated synthetic signing keys and a private
temporary journal. Its read-only worker calls the same native implementation
as the existing Local Agent CLI, not a mock [Windows](https://www.microsoft.com/windows) process. [Windows](https://www.microsoft.com/windows) CI also
checks the active power plan independently before and after. The two-step proof
closes and reopens the journal between operations and checks that each executes
only once in the approved order. <a href="https://www.kernel.org/" title="Open Linux official download or service page" aria-label="Open Linux official download or service page" data-inta-brand="app-linux"><img src="https://raw.githubusercontent.com/Herbertofury/INTA/main/assets/brand-icons/c274b9b1ea45948db04e.png" alt="Linux official page" width="18" height="18" valign="middle" /></a> [Linux](https://www.kernel.org/) reports
unsupported and exercises refusal and journal behavior. The fixture's catalog
pin is synthetic; it does not certify a production package or authorize a PC.

No remote endpoint, service, network enrollment, shell, installation,
uninstall, changes to [Windows](https://www.microsoft.com/windows) settings, or real user credentials are included.
The pilot takes a trusted local adapter selected by its host. A production
implementation still needs reviewed immutable catalog artifacts, pairing and
revocation, authenticated delivery and receipts, and independent acceptance
for each operation. The Mobile editor is not yet an authorized dispatch path.

### Review an exported phone draft on Windows

In the native INTA workbench, open **Diagnostics → Review phone repair
draft...** and choose the unchanged text file from INTA Mobile's **Review
draft export**. The workbench shows the original step order and labels. It
opens only a selected file on an ordinary local fixed drive, with a 16 KiB
limit. It does not keep a copy, choose a device, approve a plan, run commands
or dispatch work. Deliberately edited, malformed or unsupported exports are
refused. Keep the original file private and unchanged.
