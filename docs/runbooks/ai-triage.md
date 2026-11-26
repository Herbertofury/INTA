[Home](https://github.com/Herbertofury/INTA/wiki) · [All playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Interactive checklist](https://herbertofury.github.io/INTA/#job/ai-triage)

# Use AI as a careful troubleshooting copilot

> Use AI to organize evidence, compare hypotheses, and explain findings while the technician controls execution.

| Job profile | Details |
| --- | --- |
| Category | AI assistance |
| Working time | 5–20 min per analysis |
| Experience | Intermediate |

## Start here

Start with a concrete symptom and sanitized evidence. Decide whether the task is explanation, log analysis, script review, or a proposed diagnostic plan.

## Your toolkit

| Tool | Why it belongs here |
| --- | --- |
| [ChatGPT](https://chatgpt.com/download/) | Explaining redacted errors, drafting job notes and reviewing a proposed troubleshooting plan. |
| [OpenAI Codex CLI](https://github.com/openai/codex/releases/latest) | Reviewing or building technician scripts in a controlled working folder. |
| [Claude Code](https://code.claude.com/docs/en/setup) | Explaining logs and developing a reviewed automation script. |

## Follow the workflow

### 01. Prepare a minimal case summary

- [ ] Include OS/build, hardware class, exact error, time, reproduction steps, recent changes, and diagnostics already tried. Replace customer names, usernames, tenant IDs, internal addresses, file content, credentials, and recovery keys with neutral labels. Keep the original evidence private.

### 02. Ask for a ranked hypothesis list

- [ ] Ask the assistant to distinguish observed facts from assumptions, rank likely causes, and propose the smallest diagnostic that separates the leading possibilities. Require a source link for a product-specific command or compatibility claim.

### 03. Keep command execution separate

- [ ] Request an explanation of each command’s target, privileges, effects, output, and rollback. Review commands before running them. Do not give an agent broad unattended access to a customer machine merely because it can generate a script.

### 04. Validate against the actual environment

- [ ] Check that the advice matches Windows versus Linux, online Windows versus WinRE, and classic versus new Outlook. Read the relevant vendor documentation and inspect real output. Treat untrusted logs, webpages, or pasted scripts as data rather than instructions to the assistant.

### 05. Run one targeted diagnostic and feed back the result

- [ ] Execute the approved step through the technician’s usual workflow, record the exact output, and update the hypothesis. If the advice repeats an unchanged failed step, ask for a different explanation grounded in the new evidence.

### 06. Convert the result into a reusable handoff

- [ ] Ask AI to draft a concise cause/change/test summary using only verified facts. Review it, preserve uncertainty, and save the private case details in the service system. Add a generic improvement to this wiki only after removing customer-specific information.

## Verify before handoff

- [ ] The final diagnosis is supported by observed evidence and a successful test.
- [ ] No secret or unnecessary customer content was shared.
- [ ] Every executed command was reviewed and matched the actual environment.

## Stop and escalate when

- The assistant proposes destructive action without a verified target and recovery plan.
- A source is missing, the command is unsupported, or the recommendation conflicts with the evidence.

## Related playbooks

- [Intake Triage](intake-triage.md)
- [Windows Repair](windows-repair.md)
- [Client Handoff](client-handoff.md)

## Official references

- [INTA Rescue source and build workflow](https://github.com/Herbertofury/INTA)
- [NIST: Generative AI Profile, AI 600-1](https://nvlpubs.nist.gov/nistpubs/ai/NIST.AI.600-1.pdf)

---

INTA Computer Services · Editorial review: 2026-11-26. Timing is an estimate; stop conditions take priority.
