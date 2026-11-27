# Progression prototype: implementation and acceptance

**Branch:** `feat/guided-progress-and-rewards`  
**Current scope:** Browser-local prototype in the existing technician portal. This is deliberately not presented as cross-device account synchronization.

## Implemented in this branch

- Added **Progress & awards** to the existing portal navigation and route.
- Added a compact personal progression view with total XP, distinct completed playbooks, earned milestones and recent completions.
- Added a versioned, idempotent XP event for a playbook only after every authored procedure step and every verification item has been checked.
- Added three cumulative, deterministic milestones: First playbook, Field-ready (three distinct playbooks) and Reliable routine (five distinct playbooks).
- Added optional two-note completion audio, off by default, with a preference stored locally. Visual completion feedback remains available when sound is disabled or unavailable.
- Added a small responsive style layer consistent with the existing neutral INTA theme.

## Honest limitations

- Existing checklist state and the new progression ledger are stored in browser local storage. They are not authenticated, backed up, shared across devices or protected against local editing.
- Checklist completion is a user acknowledgement, not independently collected evidence that a repair succeeded. The UI says so explicitly.
- The current data model awards once per playbook ID and rule version in this browser profile. It is not a per-customer ticket history and must not be used for employee performance scoring.
- There is no login, server-side ledger, tenant authorization, mobile client, game integration, notification integration or account deletion/export flow in this prototype.
- Do not put customer names, device identifiers, case notes, credentials, serial numbers or diagnostic logs into this profile.

## Manual acceptance

1. Open the portal and select **Progress & awards**. Confirm the empty state is clear and the XP total is zero.
2. Open a playbook. Check all procedure steps but leave at least one verification item unchecked. Confirm no XP or award is granted.
3. Complete the remaining verification items. Confirm exactly one +50 XP event appears and First playbook is earned.
4. Reload the page. Confirm the ledger and award remain in the same browser.
5. Uncheck and re-check a step. Confirm the same playbook does not award XP again.
6. Complete two additional distinct playbooks. Confirm Field-ready appears once the third unique playbook is complete.
7. Enable award sound and complete a new eligible playbook. Confirm the short tone is optional and can be disabled again.
8. Use browser storage unavailable/private mode where practical. Confirm checklist use remains possible for the session and the UI does not claim cloud sync.
9. Test keyboard-only operation, screen reader labels, narrow mobile width, reduced motion and muted device audio.
10. Confirm customer-facing handback content does not show XP or badges and that the original checklist export and print actions still work.

The detailed provider-neutral API, identity, tenant, offline conflict and server-authoritative reward design is in [Progression synchronization contract](progression-sync-contract.md). The broader product direction remains in [Guided progress and rewards](guided-progress-and-rewards.md).

## Next engineering gate: real account sync

Before adding a login button or claiming sync, inspect the selected identity provider and server architecture. Define authenticated user and organization scopes, server-authoritative idempotent event writes, conflict-safe checklist revisions, account recovery, export/deletion, offline queue replay and tenant isolation. Keep customer service records in the approved ticketing system, separate from personal learning rewards. Do not store secrets or customer data in local storage.

The optional waiting-room game is a later, isolated milestone. It must run at low priority and must not alter the scheduler, resource allocation, result state or completion notification of scans and diagnostics.
