# Claude Code Project Instructions

This repo uses **Contract‑First Development (Design by Contract)**.

Follow the workflow described in:
- @AGENTS.md
- @WORKFLOW.md

Mode prompts (use exactly):
- @.cbd/prompts/CONTRACT.md
- @.cbd/prompts/BUILD.md

House rules:
- Ask at most **3** blocking questions per round, then wait. Repeat until `open_questions` is empty.
- Do not implement while contract status is `draft`.
