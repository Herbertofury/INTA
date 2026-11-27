# Guided progress, achievements and optional downtime play

**Status:** Product direction and implementation contract  
**Scope:** INTA website, desktop application, mobile experience and shared account  
**Design constraint:** Professional technician software first. Progression should make real work clearer and more satisfying, never turn service work into a game or compromise safety.

## Product intent

INTA should help a person understand what to do next, complete a procedure confidently, prove the result and hand the work over cleanly. A calm progress layer can make that journey more rewarding: visible completion, saved progress, meaningful experience, skill milestones and optional downtime entertainment.

This is a shared experience across the INTA website, desktop application and mobile app. It should feel like one product, not three unrelated reward systems. Reuse the existing INTA visual system: neutral charcoal surfaces, restrained blue interaction accents, compact readable rows, meaningful status labels and accessible controls. Do not introduce a separate neon/arcade visual theme into the technician workspace.

## Experience pillars

### 1. Guided work and checklists

- Turn suitable playbooks, onboarding paths, diagnostic procedures and handback tutorials into structured, resumable guided flows.
- Show the current step, total progress, prerequisites, expected result, stop conditions and next action.
- Support checkboxes, required acknowledgements, notes, evidence references and optional steps where the underlying procedure allows them.
- Distinguish **not started**, **in progress**, **completed**, **blocked**, **skipped with reason** and **needs verification**. A checked box alone must not imply that a repair succeeded.
- Allow a user to pause and resume. Preserve a local working copy if offline; synchronize later with explicit conflict handling.
- Keep the full procedure, sources, warnings, commands and escalation route accessible. Never hide important safety instructions behind a game-like stepper.
- Provide separate technician and customer-facing handback views when appropriate. Customer handback should explain what was done, what was verified, what remains and what the customer should do next, without exposing internal notes or sensitive diagnostics.
- Use clear completion criteria. Some tasks require a test result or human confirmation before the workflow can be marked verified.

### 2. Meaningful XP and progression

Award experience for demonstrated progress, not raw activity. Examples include:
- completing a verified learning module or onboarding milestone;
- correctly completing a guided procedure and its acceptance checks;
- passing a knowledge check or practical simulation;
- documenting a useful, reviewed knowledge contribution;
- completing a safe handback with required checks;
- mentoring or helping a teammate where that contribution is acknowledged.

Do not award XP for repeated clicks, opening pages, idle time, launching scans, scan duration, arbitrary volume of tickets, bypassing verification or repeatedly restarting a task. Do not create incentives to run unnecessary diagnostics or prolong customer work.

Keep XP rules transparent and versioned. Explain why an award was granted. Make corrections auditable and idempotent so retrying a sync cannot duplicate rewards. Do not rank technicians by speed alone or create public leaderboards by default.

### 3. Achievements, badges and awards

Use a small, curated set of meaningful awards, such as:
- **First Response** — completed the service-desk onboarding;
- **Evidence First** — consistently completed required pre-change checks;
- **Recovery Ready** — passed a recovery readiness exercise;
- **Verified Fix** — completed a workflow with its required acceptance evidence;
- **Knowledge Contributor** — submitted a contribution accepted through review;
- **Customer Handoff** — completed the required handback checklist.

Awards must describe a real, defined achievement. Provide criteria and earned date; distinguish personal learning awards from team or organization awards. Avoid fake scarcity, streak pressure, random reward boxes, paid boosts, competitive shame, or badges for unsafe shortcuts. Awards should be optional to display and must not reveal customer or incident information.

### 4. Optional waiting-room roguelite

Build a small, self-contained roguelite minigame as an optional way to use otherwise idle time while a long-running task continues. It should be fun on its own, with short sessions, satisfying movement/combat, readable controls and a clear pause/exit path.

- The game runs as a separate, low-priority workload. It must not block, starve, pause, reprioritize or change scan/repair/diagnostic jobs.
- The user can disable it entirely. Never auto-launch it or steal focus when a task starts or finishes.
- Respect battery, thermal, memory and reduced-performance modes. On mobile, support touch controls and immediate suspension when the app backgrounds.
- Show honest task status separately. Never imply that playing accelerates a scan or that a scan has succeeded because a game session ended.
- Keep rewards cosmetic or limited to the learning/game profile. Do not grant technician XP for idling during scans.
- Do not send game audio when muted, in quiet mode, or when the user has disabled sound. Never play sound automatically on page load.
- A completed task notification must remain visible and accessible even while the game is open.

The game can share a reusable INTA game shell and account identity, but it must remain optional and isolated from operational code. Keep the initial game deliberately small; add depth only after performance, lifecycle and accessibility are proven.

### 5. Visual, auditory and haptic feedback

Use feedback to confirm an action and communicate state:
- checkbox completion: a crisp visual state change and subtle optional tick;
- verified milestone: a brief, restrained confirmation animation and optional sound;
- level/award earned: a short, skippable celebration with a clear explanation;
- error or blocked step: a clear text explanation and next action, not an alarming sound alone;
- long-running task completion: a system notification or in-app status that does not interrupt active work.

Controls:
- Global sound toggle and separate volume control;
- respect system mute, browser autoplay restrictions and mobile silent/quiet modes;
- reduced motion and low-stimulation mode;
- optional haptics on supported devices;
- no flashing, persistent confetti, looping celebratory audio or surprise loud sounds;
- every sound/animation has an equivalent text or visual status.

Keep effects short and infrequent. Do not play a sound for every field edit, keystroke, background poll or routine navigation.

### 6. Shared INTA account and synchronization

One account identity should work across the website, desktop app and mobile app. Reuse the repository's existing authentication and account architecture where present; do not create a parallel identity provider or assume a specific vendor before inspecting the implementation.

Sync only the state needed for continuity:
- user preferences, accessibility and feedback settings;
- checklist/workflow progress and explicit completion states;
- training modules, assessment results and learning milestones;
- XP ledger, achievement definitions/awards and earned timestamps;
- game save/profile state where the user opts in.

Security and privacy requirements:
- follow least privilege and established authentication/session protections;
- do not store credentials, access tokens or sensitive customer case data in public profile fields or achievement metadata;
- separate personal learning progress from organization/team reporting;
- provide clear sync status, last-synced time, retry and conflict resolution;
- support local/offline progress and safe retry without duplicate awards;
- allow users to export their progression data and request account/data deletion through the documented account process;
- do not make sign-in mandatory for basic public guides or safe local checklist use unless a feature genuinely requires an account;
- never expose customer records across accounts or organizations.

Treat account identity as the synchronization boundary, not permission to share every piece of state. Organization-owned workflows must additionally enforce tenant and role authorization.

## Interface placement

Keep the core technician workspace compact and task-focused.

- A small progress summary may appear on Home or the user profile: current learning path, in-progress checklist and recent earned awards.
- In a playbook, show a restrained step indicator and completion state adjacent to the procedure.
- Put the full award collection and learning history on a dedicated profile/progression destination, not across every tool row.
- Keep customer handback focused on service outcome and next steps. Do not display XP, game rewards or public rankings in customer-facing reports.
- Provide a compact, optional game launcher in the waiting/task area only when an eligible long-running task is active. It must never obscure task controls or status.
- On mobile, prioritize the current step, clear next action, accessible completion control and sync status.

Do not add oversized dashboard cards, decorative stats, animated backgrounds, constant motion, noisy badges or a second competing color system. Progress is information; celebration is a brief response to a meaningful milestone.

## Data and behavior model

Keep definitions separate from user progress. Suggested conceptual entities (adapt names to the actual schema):

- `workflow_definition`: stable ID, version, title, audience, ordered steps, prerequisites, safety notes, completion policy and source references.
- `workflow_step`: stable ID, instruction, expected result, required/optional flag, verification type and stop conditions.
- `workflow_progress`: account/tenant scope, workflow ID/version, current step, per-step state, local revision, server revision and updated timestamp.
- `learning_module` / `assessment_attempt`: content version, outcome and evidence needed to establish mastery.
- `xp_event`: immutable event ID, account, event type, rule version, amount, timestamp and idempotency key.
- `achievement_definition`: stable ID, criteria, version, description and visibility policy.
- `achievement_award`: account, achievement ID/version, earned timestamp and qualifying event references.
- `user_feedback_preferences`: sound, volume, haptics, motion, quiet mode and game availability.
- `game_save`: isolated game state, version and synchronization metadata.

These are conceptual boundaries, not a mandate to introduce a new database or duplicate existing user/account tables. Inspect the current architecture first. Use migrations, stable IDs, schema versioning and reversible rollout. Do not store secrets in these records.

## Reliability and performance requirements

- Checklist interactions should feel immediate and remain usable if synchronization is unavailable.
- Sync should be event-driven (user action, explicit resume/reconnect, or bounded background sync where the platform requires it), not an aggressive polling loop.
- Use idempotency keys, ordered revisions and conflict policies. Never silently discard a user's completed work.
- Keep XP derived from validated events, not a mutable client-side counter that can be trivially forged.
- Keep the game process/thread/resource budget separate from scans and other critical operations.
- Completion and scan status must come from the real task state, not UI timers or game lifecycle.
- Preserve keyboard navigation, screen-reader labels, high contrast, touch targets, reduced motion and offline usability.
- Measure startup, memory, CPU, battery and foreground responsiveness before and after introducing progression or game features.

## Rollout and acceptance gates

1. **Architecture discovery:** map current site/app/mobile entry points, auth, storage, offline behavior, task runner, notifications and design tokens. Identify existing reusable systems before implementation.
2. **Guided checklist foundation:** convert one low-risk onboarding or handback flow. Prove pause/resume, offline state, accessible controls, correct completion criteria and safe synchronization.
3. **Progress and awards:** add a transparent XP event ledger and a small set of verifiable achievements. Test duplicate events, replay, account switching and tenant boundaries.
4. **Cross-platform sync:** prove one workflow can be started on one platform and resumed on another without data loss or unauthorized disclosure.
5. **Feedback system:** add restrained visual/audio/haptic feedback with all preferences and accessibility fallbacks.
6. **Waiting-room game prototype:** build an isolated, optional prototype and test that scans continue at the same priority and finish correctly while the game is active, paused, backgrounded or closed.
7. **Release gate:** verify desktop, mobile, narrow web, keyboard, screen reader, offline/reconnect, reduced motion, mute, battery saver, failure recovery and migration/rollback. Publish only the features that pass.

Do not block core INTA service functionality on the game or rewards system. Ship the guided-work foundation first; progression and the game are additive layers.

## Explicit non-goals

- No gamification of dangerous repair actions, security bypasses, data deletion or customer-impacting changes.
- No XP for scan duration, idle time, speed-only ticket closure or unnecessary tool launches.
- No compulsory game, sound, login or public leaderboard for ordinary use.
- No loot boxes, paid XP, streak manipulation, dark patterns or attention traps.
- No customer-facing disclosure of internal staff performance or private service details.
- No claim that a checked step proves a successful repair without the procedure's required verification.

## Definition of done

A technician can open a guided procedure, understand the next action and expected result, complete and verify steps, pause, work offline, resume on another signed-in device, and hand back a clear outcome. Meaningful milestones are credited once with transparent criteria. Feedback is professional, brief and configurable. The optional game remains isolated and cannot degrade or interfere with real work. Core INTA remains fully useful when every gamification feature is disabled.


## Current implementation checkpoint

The first browser-local progression prototype is tracked in [Progression prototype: implementation and acceptance](progression-implementation.md). The proposed secure cross-device design is separately documented in [Progression synchronization contract](progression-sync-contract.md). These documents distinguish shipped branch behavior from future account, mobile and game work. Do not claim cloud sync until the production acceptance gate passes.
