[Home](https://github.com/Herbertofury/INTA/wiki) · [All playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Interactive checklist](https://herbertofury.github.io/INTA/#job/remote-support)

# Start and finish a clean remote-support session

> Choose the right attended or managed tool and leave a clear, auditable handoff.

| Job profile | Details |
| --- | --- |
| Category | Remote support |
| Working time | 5–15 min setup |
| Experience | Starter |

## Start here

Confirm the customer, intended device, and requested task through the established service channel before exchanging a session code.

## Your toolkit

| Tool | Why it belongs here |
| --- | --- |
| [Microsoft Quick Assist](https://apps.microsoft.com/detail/9P7BP5VNWKX5) | A quick one-time Windows support session without deploying an RMM agent. |
| [RustDesk](https://github.com/rustdesk/rustdesk/releases/latest) | A flexible remote-control option across mixed client platforms. |
| [MeshCentral](https://github.com/Ylianst/MeshCentral) | INTA-managed unattended support with its own server and device groups. |
| [Chrome Remote Desktop](https://remotedesktop.google.com/support) | Attended support or an explicitly configured personal remote-access endpoint. |

## Follow the workflow

### 01. Choose the session type

- [ ] For a one-time Windows session, Quick Assist offers attended sharing and a separate control request. Use the organization’s approved RustDesk or MeshCentral deployment for recurring managed support. Cross-platform support needs a tool explicitly supporting both endpoints.

### 02. Use an official client and the right endpoint

- [ ] Open the tool from the official catalog link or the already managed installation. Confirm the device label with the user and close unrelated sensitive windows. Do not install a second unattended agent merely to avoid fixing an existing approved connection.

### 03. Connect with the user present

- [ ] For Quick Assist, the helper shares the code, the recipient allows screen sharing, and the helper requests control separately. Explain the immediate action before taking control. Let the user enter passwords or handle protected prompts when the tool requires it.

### 04. Maintain a recovery path

- [ ] Before a reboot, adapter change, VPN modification, or security-product restart, confirm how the session will resume and whether someone can act locally. Record work in the private ticket and capture only the evidence needed for the repair.

### 05. Handle recurring access deliberately

- [ ] For an explicitly managed endpoint, use named technician accounts, the deployment’s access roles, MFA where supported, and a documented device owner. Review existing access rather than sharing one permanent password across coworkers.

### 06. Close and verify

- [ ] Ask the user to perform the original task, end the session, and remove temporary access if it was created only for this case. Confirm any approved persistent agent remains assigned to the correct customer and that the user knows how to contact support again.

## Verify before handoff

- [ ] The intended device was reached and the user’s original task succeeds.
- [ ] The session is ended and temporary access is removed.
- [ ] Persistent access, if requested, has a named owner and correct access scope.

## Stop and escalate when

- The device or caller identity does not match the service request.
- The next action could disconnect a critical system without a local recovery path.

## Related playbooks

- [Intake Triage](https://github.com/Herbertofury/INTA/wiki/Job-intake-triage)
- [Network Dns](https://github.com/Herbertofury/INTA/wiki/Job-network-dns)
- [Client Handoff](https://github.com/Herbertofury/INTA/wiki/Job-client-handoff)

## Official references

- [Microsoft: Quick Assist](https://support.microsoft.com/en-us/windows/apps/solve-pc-problems-remotely-using-quick-assist)

---

INTA Computer Services · Editorial review: 2026-11-26. Timing is an estimate; stop conditions take priority.
