[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

<a id="-ai-assistance"></a>

# AI assistance

Use sanitized evidence, verified sources, and reviewed commands throughout an assisted investigation.

**5 tools** · [All categories](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Selection standards](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards)

**Selection question:** Explain evidence, review a script, or compare hypotheses?

<a id="-start-with-a-workflow"></a>

## Related playbooks

- [Use AI as a careful troubleshooting copilot](https://github.com/Herbertofury/INTA/wiki/Job-ai-triage)
- [Start here: triage any computer problem](https://github.com/Herbertofury/INTA/wiki/Job-intake-triage)

<a id="compare-options"></a>

<a id="️-compare-the-options"></a>
<a id="-compare-the-options"></a>

## Tool comparison

| Tool | Choose it for | Platforms | Role |
| --- | --- | --- | --- |
| [ChatGPT](#chatgpt) | Explaining redacted errors, drafting job notes and reviewing a proposed troubleshooting plan. | Web, [Windows](https://www.microsoft.com/windows), [macOS](https://support.apple.com/macos) | Core |
| [Claude Code](#claude-code) | Explaining logs and developing a reviewed automation script. | [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos), [Windows](https://www.microsoft.com/windows) | Core |
| [OpenAI Codex CLI](#codex-cli) | Reviewing or building technician scripts in a controlled working folder. | [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos), [Windows](https://www.microsoft.com/windows) | Core |
| [Gemini CLI](#gemini-cli) | An additional AI-assisted terminal workflow for reviewed scripts and documentation. | [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos), [Windows](https://www.microsoft.com/windows) | Specialist |
| [Ollama](#ollama) | A local AI assistant when the machine has the capacity and model files are available. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) | Specialist |

Core means an everyday candidate; Specialist means a focused advanced use; Evaluate means prove it in a suitable test environment before promotion.

<a id="chatgpt"></a>

## [ChatGPT](https://chatgpt.com/)

AI assistant available through the official web service and supported desktop apps.

[Official download](https://chatgpt.com/download/) · [Documentation](https://help.openai.com/) · [Publisher](https://chatgpt.com/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Explaining redacted errors, drafting job notes and reviewing a proposed troubleshooting plan. |
| **Where it runs** | Web, [Windows](https://www.microsoft.com/windows), [macOS](https://support.apple.com/macos) |
| **How you use it** | web |
| **Access / pricing** | Free and paid plans |
| **Selection role** | Core |

> **Before using it:** Use redacted evidence. A browser shortcut in the [Linux](https://www.kernel.org/) rescue environment is not a native offline [ChatGPT](https://chatgpt.com/) app.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://help.openai.com/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="claude-code"></a>

## [Claude Code](https://code.claude.com/docs/en/overview)

Terminal-based coding assistant for navigating and changing a project.

[Official download](https://code.claude.com/docs/en/setup) · [Documentation](https://code.claude.com/docs/en/overview) · [Publisher](https://code.claude.com/docs/en/overview)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Explaining logs and developing a reviewed automation script. |
| **Where it runs** | [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos), [Windows](https://www.microsoft.com/windows) |
| **How you use it** | installed |
| **Access / pricing** | Account or API access according to current plan |
| **Selection role** | Core |

> **Before using it:** Use an isolated job workspace and keep client secrets out of prompts and public files.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://code.claude.com/docs/en/overview)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="codex-cli"></a>

## [OpenAI Codex CLI](https://github.com/openai/codex)

Coding agent that works with local project files and tools through a terminal interface.

[Official download](https://github.com/openai/codex/releases/latest) · [Documentation](https://developers.openai.com/codex/cli/) · [Publisher](https://github.com/openai/codex)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Reviewing or building technician scripts in a controlled working folder. |
| **Where it runs** | [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos), [Windows](https://www.microsoft.com/windows) |
| **How you use it** | installed |
| **Access / pricing** | Open-source client; account or API usage terms apply |
| **Selection role** | Core |

> **Before using it:** Review generated commands and their effects before using them on client systems.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://developers.openai.com/codex/cli/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="gemini-cli"></a>

## [Gemini CLI](https://github.com/google-gemini/gemini-cli)

Open-source command-line agent with Google Gemini integration.

[Official download](https://github.com/google-gemini/gemini-cli/releases/latest) · [Documentation](https://geminicli.com/docs/) · [Publisher](https://github.com/google-gemini/gemini-cli)

| Decision | Detail |
| --- | --- |
| **Choose it for** | An additional AI-assisted terminal workflow for reviewed scripts and documentation. |
| **Where it runs** | [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos), [Windows](https://www.microsoft.com/windows) |
| **How you use it** | installed |
| **Access / pricing** | Open-source client; account and usage limits vary |
| **Selection role** | Specialist |

> **Before using it:** Confirm the current authentication path, quotas and tool permissions before deployment.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://geminicli.com/docs/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)

<a id="ollama"></a>

## [Ollama](https://ollama.com/)

Local model runtime with downloadable models and a local API.

[Official download](https://ollama.com/download) · [Documentation](https://docs.ollama.com/) · [Publisher](https://ollama.com/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A local AI assistant when the machine has the capacity and model files are available. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | installed |
| **Access / pricing** | Runtime available freely; model licenses differ |
| **Selection role** | Specialist |

> **Before using it:** Offline use requires downloaded models. Measure CPU, RAM and GPU cost before adding it to a rescue image.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://docs.ollama.com/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[Back to comparison](#compare-options)


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
