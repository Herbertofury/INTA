# INTA progression synchronization contract

**Status:** Architecture contract only; no server endpoint or identity provider is deployed.  
**Applies to:** INTA website, desktop client and future mobile client.

## Current architecture boundary

The current public hub is a static site generated from the repository's portal template, application JavaScript and stylesheet. It has browser-local checklist/favorites state and no authenticated service API in the inspected portal entry point. A static GitHub Pages client cannot safely hold a client secret or establish trusted XP by itself.

Therefore:
- Do not add a decorative login button that cannot authenticate or synchronize.
- Do not embed a client secret, signing key, service token or privileged API key in JavaScript, generated HTML, a public repository or a desktop/mobile bundle.
- Do not describe localStorage as an account, backup or cross-device sync.
- Keep local checklist/progression usable without login. Add cloud sync only after a real API and identity provider are selected, reviewed and deployed.

## Identity and authorization

Use an established OIDC provider with Authorization Code + PKCE for public clients. The server validates issuer, audience, signature, expiry and nonce; clients never validate identity claims as the authority for server writes.

Authorization boundaries:
- **Personal profile:** owned by one immutable account subject; XP and learning awards are private by default.
- **Organization membership:** separate tenant-scoped membership with explicit roles. A personal account joining an organization does not automatically share its private learning history.
- **Workflow definition:** published, versioned content is read-only to ordinary users. Authors/admins publish a new version rather than mutating an in-progress definition.
- **Service case:** customer/ticket records remain in the approved service system. Progress sync must not become a shadow ticketing database.

Every API request is authorized from the validated server-side identity and tenant membership. Never trust account IDs, organization IDs, XP totals, role names or achievement claims supplied by the client.

## Proposed API surface

All endpoints require TLS and authenticated access except public workflow-definition reads.

| Method and route | Purpose | Required safety behavior |
| --- | --- | --- |
| `GET /v1/me` | Return minimal account and capability metadata | No unnecessary profile disclosure |
| `GET /v1/workflows/{workflowId}/versions/{version}` | Fetch immutable workflow definition | Verify published status and content version |
| `GET /v1/me/workflow-progress?cursor=...` | Fetch the user's progress changes | Paginated, account-scoped, stable cursor |
| `PUT /v1/me/workflow-progress/{workflowId}` | Submit a progress snapshot | Require client revision and workflow version; return conflict when stale |
| `POST /v1/me/progression-events` | Submit a completion/learning event for server evaluation | Idempotency key; validate allowed event type and evidence requirements |
| `GET /v1/me/progression` | Read server-calculated XP and earned awards | Server-derived only; no client-set balance |
| `GET /v1/me/sync-state` | Fetch sync cursor and last successful sync | No polling loop; use reconnect/resume/manual sync |
| `DELETE /v1/me/progression` | Delete personal progression data | Re-authenticate; explain irreversible effect; audit deletion without retaining deleted content |

Use consistent JSON error envelopes and machine-readable codes: `unauthorized`, `forbidden`, `not_found`, `workflow_version_retired`, `revision_conflict`, `idempotency_conflict`, `validation_failed`, `rate_limited` and `server_error`. Never return internal stack traces or another account's record.

## Progress snapshot contract

A synchronized workflow-progress record should contain only:
- workflow stable ID and immutable version;
- per-step state (`not_started`, `in_progress`, `completed`, `blocked`, `skipped_with_reason`, `needs_verification`);
- optional non-sensitive note limited to a documented length and content policy;
- client-generated revision UUID, monotonic local revision and server revision;
- updated timestamp and originating client type/version.

Do not include customer names, email addresses, physical addresses, serial numbers, passwords, recovery keys, raw diagnostic output, screenshots, private ticket URLs or unrestricted free-text case notes. If a workflow needs evidence, store a reference to an approved external record with its own access controls, not the evidence payload in the progression service.

## Conflict and offline rules

1. Every client keeps a durable local queue of pending operations and a last acknowledged server revision.
2. Each operation has a stable UUID and is retried with the same idempotency key.
3. The server returns the canonical revision and accepted fields.
4. If the server revision changed since the client's base revision, return a conflict; do not silently last-write-wins the whole checklist.
5. Merge independent step-state changes when they do not violate workflow rules. For incompatible changes, show a compact comparison and let the user choose; retain both values until resolved.
6. Never downgrade a completed step silently. Reopening a step should be an explicit new event with reason where policy requires it.
7. A workflow definition version change never remaps progress by array index. Stable step IDs are required. Preserve the old version for existing work or provide a reviewed migration map.
8. Sync is triggered by user action, app resume/reconnect, or bounded platform-appropriate background sync. Do not add aggressive polling.

## XP and achievement authority

The client may display a provisional local preview, but only the server ledger is authoritative after account sync.

- Event IDs are globally unique within the account and idempotent.
- The server validates event type, workflow version, completion policy and allowed evidence references.
- XP amount and rule version are selected server-side, never accepted from a client-provided balance.
- Award issuance is a transaction: validated event, ledger entry and any newly satisfied achievement criteria commit atomically.
- Duplicate retries return the original result without duplicating XP.
- Rule changes create a new rule version and do not retroactively rewrite historical awards without a documented migration.
- No reward is granted for idle time, scan duration, unnecessary diagnostics, click volume or speed-only closure.
- An achievement is a personal learning/progression milestone, not a regulated credential or a claim of independently audited technician competence unless a separate assessment process proves that.

## Account lifecycle and portability

Before production release, support:
- account recovery and sign-out on all devices;
- visible per-device sync state and revocation;
- export of personal progression in a documented machine-readable format;
- deletion request and confirmation;
- tenant leave/removal behavior;
- retention and backup policy;
- abuse/rate-limit handling;
- data minimization and audit access controls;
- migration/rollback plan and a tested service outage mode.

Do not make sign-in mandatory for public guides or local checklist work. Clearly distinguish **local only**, **sync pending**, **synced** and **sync conflict** states.

## Shared engine boundary for Athena and other INTA-family apps

Do not prematurely extract a shared library before INTA's workflow semantics are proven. First stabilize:
1. workflow/step state vocabulary;
2. idempotent event envelope;
3. award definition and versioning;
4. local queue and conflict semantics;
5. accessible progress and feedback preferences.

Then extract a small platform-neutral progression core with adapters for identity, storage, notifications and UI. Athena may host and maintain the reusable achievement/progression engine, while INTA retains its own technician workflow definitions, policy, brand and reward catalog. Projects must not share XP balances or user data automatically; reuse code and schemas, not identities or records. Each consuming app opts in and applies its own safety policy.

## Production acceptance gate

- A user can start offline, complete steps, close the app, reconnect and resume without losing changes.
- A replayed completion request never duplicates XP or awards.
- A stale client cannot overwrite newer progress without a conflict.
- Account A cannot read or write account B's records.
- Organization A cannot read Organization B's records.
- Removing an organization member does not delete or expose their personal profile.
- A customer-data scan finds no prohibited fields in progression records, events, logs or exports.
- Export and deletion work end-to-end.
- Website, desktop and mobile use the same contract and stable workflow/step IDs.
- Server outage leaves local workflows usable and clearly marks unsynced state.
- No release claims account sync until these checks pass.
