---
name: permission-command-center
description: Fast, task-scoped preflight for connected-app access. Trigger on /permissions, /access, "check or enable permissions", "full access", "allow for this conversation", "give GitHub/Drive access", or automatically before work that needs GitHub, Google Drive, or another external connector. Surface necessary approval prompts with the first legitimate task action, then resume without repeated checks.
---

# Permission Command Center

## Mission

Prevent connector authorization interruptions from consuming the user's task time or conversation context. Identify the exact external services a task needs, check their real access state, surface required ChatGPT approval prompts as early as safely possible, and continue the original task immediately after the user approves.

This skill coordinates authorization. It does not grant itself access, click approval buttons, bypass provider controls, or promise that a conversation-level grant persists into a new chat.

## When to activate

Activate when the user:
- Says /permissions, /access, "check permissions", "enable permissions", "full access", or "allow for this conversation".
- Names a provider and asks for access, such as "GitHub full access", "give Drive access", or "connect Outlook".
- Starts a task whose completion requires an external connected app, repository, cloud drive, mailbox, calendar, or other provider.

For an ordinary task that needs no external connector, do not run a permission audit.

## Fast workflow

### 1. Scope the providers

Before lengthy research or artifact creation, identify only the providers required to finish the current task. Build a short internal list of the exact operations needed: read, search, create, update, upload, share, publish, or delete.

"All permissions" means all permissions needed by this task, not every app on the user's account. Do not scan or change unrelated integrations. If the user asks for a standalone permission audit without naming a task or provider, ask one concise question about which provider(s) they mean.

### 2. Check the actual access layers

Keep these separate; one does not imply another:

1. **Connector availability:** Is the provider tool available and connected in this session? Use the matching connector first. If unavailable, discover the exact plugin/app and suggest its connection; the user must complete any install or connect flow.
2. **ChatGPT app permission mode:** When the user explicitly asks for full access to a named app, inspect that exact app's setting with Plugin Management. If supported, set the app-specific permission to full access. Do not change the global default just because one provider is needed. Verify the result. If the update action fails or is unavailable, say so plainly and do not claim it changed.
3. **Provider account/OAuth scopes:** Confirm the connected account and required scopes where the provider exposes them. A connected account may still lack a required scope.
4. **Resource permissions:** Check the exact repository, folder, file, mailbox, calendar, or workspace. For GitHub, inspect the authenticated identity and the target repository's actual permissions; a successful login alone does not prove push/admin access. For Drive, verify access to the exact destination, not merely that Drive is connected.
5. **ChatGPT action-consent prompt:** The UI may ask "Allow once" or "Allow for this conversation" for a particular action. Only the user can choose. This is separate from app permission mode and provider OAuth.

Use exact provider/app identifiers. Never pass broad names such as "all", "global", or an ambiguous provider to a plugin-specific permission action. Never infer that an app-level full-access setting overrides provider scopes, workspace policy, resource ACLs, or a separate UI consent gate.

### 3. Surface prompts early, without fake work

- Prefer the first **real, necessary, low-risk, reversible operation** that belongs to the user's requested task. Call it early, before spending substantial time on work that depends on the connection.
- For a GitHub task, use the first real task write (for example, creating the task branch or the first intended file) as the early authorization checkpoint. Verify push permission on the exact repository first.
- For a Drive task, use the first real operation on the intended folder or file. If the deliverable is not ready yet, perform a relevant read-only destination check and continue building; invoke the real upload/convert action as soon as the artifact is ready.
- For other providers, use the first legitimate operation that the task already requires.
- Do not create dummy files, empty commits, fake issues, test uploads, or unrelated writes just to force a popup unless the user explicitly requests a permission test.
- If the user explicitly requests a standalone GitHub permission test, this user's designated scratch repository is Herbertofury/ProjectDump. Re-check current push/admin access before using it. Prefer a temporary branch only when the available GitHub tools can also delete that branch. If branch deletion is unavailable, a create-then-delete file on the default branch leaves two audit commits in history; disclose that before doing it unless the user has already explicitly authorized that exact test. Retrieve the actual blob SHA before deleting; never guess or precompute it.
- For a Drive permission test, require a user-designated scratch folder and a confirmed delete capability. Do not upload test files into a customer, company, or project folder.
- If a safe operation cannot surface the needed prompt, do not manufacture one. Explain the exact limitation briefly and trigger the prompt at the first real operation that requires it.

When multiple providers are required, preflight them before lengthy work and surface prompts one provider at a time to avoid overlapping dialogs. Do not launch redundant or speculative actions.

### 4. Handle the prompt cleanly

When a ChatGPT approval dialog appears:
- Tell the user exactly which provider/action is waiting and recommend **Allow for this conversation** when they want the current task to proceed without repeated prompts in this chat.
- Stop calling the blocked action until the user responds.
- Do not ask the user to repeat the task, re-explain the destination, or paste information already available.
- After approval, immediately retry the same intended action and continue from the existing checkpoint. Do not restart discovery, redo completed work, or send a long status recap.
- If the user denies, stop only the blocked operation and continue independent work where possible.

Never use browser automation, keyboard simulation, scripts, or another route to click or bypass the approval dialog on the user's behalf.

## Permission modes and persistence

- **Allow once** applies to the current action.
- **Allow for this conversation** is a conversation-scoped approval. Do not promise that it will carry into a new chat.
- **App-level full access** is a separate setting. Change it only for the exact named app when the user explicitly requests it and the management action supports it. Prefer app-specific changes over global changes.
- **OAuth scopes, organization policy, repository permissions, and Drive folder ACLs** remain controlled by their respective providers or administrators.

Never report "all permissions enabled" unless each required layer was actually checked and the relevant change was verified. Use precise status words: connected, app permission set, scope missing, resource denied, approval pending, approved for this conversation, or unavailable.

## Resume and minimize context use

Once access is granted:
- Cache the result in the current task context: provider, connected account, resource, allowed operation, and whether the grant is conversation-scoped.
- Do not repeat the same preflight in this conversation unless an action fails, the target resource changes, the grant expires, or the user starts a new conversation.
- Continue the original task immediately. Keep the user's original requirements and completed work; do not restart or summarize everything.
- Verify the actual requested outcome, not a pile of redundant permission checks.

Keep user-facing updates short. Report only providers that matter to the current task and the one action the user needs to take.

## Invocation aliases

Recognize these as intent aliases:
- /permissions — preflight the providers required by the current task.
- /access <provider> — check one named provider.
- /github access or "GitHub full access" — check GitHub account and target-repository permissions.
- /drive access or "Google Drive full access" — check Drive connection and the exact target folder/file.
- "Start this task and get the permissions out of the way" — preflight task dependencies, surface required prompts early, then proceed.

These aliases are trigger phrases, not newly registered native slash commands. Use the host's actual skill invocation syntax when needed: ChatGPT Work uses the skill picker/@ invocation; Codex CLI or IDE uses $skill-name (and /skills to list skills). Automatic invocation should remain enabled.

## Completion report

Use a compact report, for example:

- GitHub — connected as the expected account; push permission verified on the target repository; conversation approval pending.
- Google Drive — connected; destination folder readable; upload approval pending.
- Outlook — not required for this task.

Then continue as soon as the user approves. Do not call a task complete merely because a permission prompt was displayed.
