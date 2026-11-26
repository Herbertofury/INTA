[🏠 Home](https://github.com/Herbertofury/INTA/wiki) · [🧭 Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [📚 Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [🧰 Tool directory](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [✨ Open interactive hub](https://herbertofury.github.io/INTA/)

# ✨ AI assistance

Use sanitized evidence, verified sources, and reviewed commands throughout an assisted investigation.

**5 tools** · [All disciplines](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Selection standards](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards)

> [!TIP]
> **Choose by the job:** Explain evidence, review a script, or compare hypotheses?

## 🧭 Start with a workflow

- [Use AI as a careful troubleshooting copilot](https://github.com/Herbertofury/INTA/wiki/Job-ai-triage)
- [Start here: triage any computer problem](https://github.com/Herbertofury/INTA/wiki/Job-intake-triage)

<a id="compare-options"></a>

## ⚖️ Compare the options

| Tool | Choose it for | Platforms | Role |
| --- | --- | --- | --- |
| [ChatGPT](#chatgpt) | Explaining redacted errors, drafting job notes and reviewing a proposed troubleshooting plan. | Web, [Windows](https://www.microsoft.com/windows), [macOS](https://support.apple.com/macos) | 🟢 Core |
| [Claude Code](#claude-code) | Explaining logs and developing a reviewed automation script. | [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos), [Windows](https://www.microsoft.com/windows) | 🟢 Core |
| [OpenAI Codex CLI](#codex-cli) | Reviewing or building technician scripts in a controlled working folder. | [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos), [Windows](https://www.microsoft.com/windows) | 🟢 Core |
| [Gemini CLI](#gemini-cli) | An additional AI-assisted terminal workflow for reviewed scripts and documentation. | [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos), [Windows](https://www.microsoft.com/windows) | 🔬 Specialist |
| [Ollama](#ollama) | A local AI assistant when the machine has the capacity and model files are available. | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) | 🔬 Specialist |

Core means an everyday candidate; Specialist means a focused advanced use; Evaluate means prove it in a suitable test environment before promotion.

---

<a id="chatgpt"></a>

## [ChatGPT](https://chatgpt.com/)

AI assistant available through the official web service and supported desktop apps.

**[ChatGPT: official download →](https://chatgpt.com/download/)** · [Documentation](https://help.openai.com/) · [Publisher](https://chatgpt.com/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Explaining redacted errors, drafting job notes and reviewing a proposed troubleshooting plan. |
| **Where it runs** | Web, [Windows](https://www.microsoft.com/windows), [macOS](https://support.apple.com/macos) |
| **How you use it** | web |
| **Access / pricing** | Free and paid plans |
| **Selection role** | 🟢 Core |

> **Before using it:** Use redacted evidence. A browser shortcut in the [Linux](https://www.kernel.org/) rescue environment is not a native offline [ChatGPT](https://chatgpt.com/) app.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://help.openai.com/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[↑ Compare the options](#compare-options)

---

<a id="claude-code"></a>

## [Claude Code](https://code.claude.com/docs/en/overview)

Terminal-based coding assistant for navigating and changing a project.

**[Claude Code: official download →](https://code.claude.com/docs/en/setup)** · [Documentation](https://code.claude.com/docs/en/overview) · [Publisher](https://code.claude.com/docs/en/overview)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Explaining logs and developing a reviewed automation script. |
| **Where it runs** | [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos), [Windows](https://www.microsoft.com/windows) |
| **How you use it** | installed |
| **Access / pricing** | Account or API access according to current plan |
| **Selection role** | 🟢 Core |

> **Before using it:** Use an isolated job workspace and keep client secrets out of prompts and public files.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://code.claude.com/docs/en/overview)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[↑ Compare the options](#compare-options)

---

<a id="codex-cli"></a>

## [OpenAI Codex CLI](https://github.com/openai/codex)

Coding agent that works with local project files and tools through a terminal interface.

**[OpenAI Codex CLI: official download →](https://github.com/openai/codex/releases/latest)** · [Documentation](https://developers.openai.com/codex/cli/) · [Publisher](https://github.com/openai/codex)

| Decision | Detail |
| --- | --- |
| **Choose it for** | Reviewing or building technician scripts in a controlled working folder. |
| **Where it runs** | [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos), [Windows](https://www.microsoft.com/windows) |
| **How you use it** | installed |
| **Access / pricing** | Open-source client; account or API usage terms apply |
| **Selection role** | 🟢 Core |

> **Before using it:** Review generated commands and their effects before using them on client systems.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://developers.openai.com/codex/cli/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[↑ Compare the options](#compare-options)

---

<a id="gemini-cli"></a>

## [Gemini CLI](https://github.com/google-gemini/gemini-cli)

Open-source command-line agent with Google Gemini integration.

**[Gemini CLI: official download →](https://github.com/google-gemini/gemini-cli/releases/latest)** · [Documentation](https://geminicli.com/docs/) · [Publisher](https://github.com/google-gemini/gemini-cli)

| Decision | Detail |
| --- | --- |
| **Choose it for** | An additional AI-assisted terminal workflow for reviewed scripts and documentation. |
| **Where it runs** | [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos), [Windows](https://www.microsoft.com/windows) |
| **How you use it** | installed |
| **Access / pricing** | Open-source client; account and usage limits vary |
| **Selection role** | 🔬 Specialist |

> **Before using it:** Confirm the current authentication path, quotas and tool permissions before deployment.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://geminicli.com/docs/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[↑ Compare the options](#compare-options)

---

<a id="ollama"></a>

## [Ollama](https://ollama.com/)

Local model runtime with downloadable models and a local API.

**[Ollama: official download →](https://ollama.com/download)** · [Documentation](https://docs.ollama.com/) · [Publisher](https://ollama.com/)

| Decision | Detail |
| --- | --- |
| **Choose it for** | A local AI assistant when the machine has the capacity and model files are available. |
| **Where it runs** | [Windows](https://www.microsoft.com/windows), [Linux](https://www.kernel.org/), [macOS](https://support.apple.com/macos) |
| **How you use it** | installed |
| **Access / pricing** | Runtime available freely; model licenses differ |
| **Selection role** | 🔬 Specialist |

> **Before using it:** Offline use requires downloaded models. Measure CPU, RAM and GPU cost before adding it to a rescue image.

<details>
<summary>Source, review date, and selection history</summary>

- **Primary source:** [Open the source](https://docs.ollama.com/)
- **Editorial review:** 2026-11-26
- **Evidence status:** Documentation reviewed; not an INTA runtime certification
- **Selection provenance:**
  - Official project or vendor documentation

</details>

[↑ Compare the options](#compare-options)


---

**Source review: 2026-11-26.** Guidance review and tool release versions are separate. [Report a correction](https://github.com/Herbertofury/INTA/issues/new/choose) · [How we maintain this guide](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards)
