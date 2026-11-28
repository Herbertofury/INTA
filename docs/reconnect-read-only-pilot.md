# Local proof for INTA reconnect repair plans

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
as the existing Local Agent CLI, not a mock Windows process. Windows CI also
checks the active power plan independently before and after. The two-step proof
closes and reopens the journal between operations and checks that each executes
only once in the approved order. Linux reports
unsupported and exercises refusal and journal behavior. The fixture's catalog
pin is synthetic; it does not certify a production package or authorize a PC.

No remote endpoint, service, network enrollment, shell, installation,
uninstall, changes to Windows settings, or real user credentials are included.
The pilot takes a trusted local adapter selected by its host. A production
implementation still needs reviewed immutable catalog artifacts, pairing and
revocation, authenticated delivery and receipts, and independent acceptance
for each operation. The Mobile editor is not yet an authorized dispatch path.
