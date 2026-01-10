# Workflow

This workflow is designed to work well with:
- **GPT‑5.2 Pro (Thinking)** for planning, contracts, and review
- **Codex** (and/or Claude Code) for implementation and running commands

## Files that drive the process
- `tasks/<id>-*.md` — short task seed (goal/context/constraints)
- `contracts/<id>.contract.json` — Design‑by‑Contract spec (pre/post/invariants/errors/tests)
- `bundles/<id>.bundle.json` — the step-by-step loop the agent must execute
- `reports/<id>.evidence.md` — mapping from contract clauses to proving code/tests

## CONTRACT mode
Input: `tasks/<id>-*.md` + repo
Output: `contracts/<id>.contract.json`, `bundles/<id>.bundle.json`, and up to 3 blocking questions

Completion condition:
- `contracts/<id>.contract.json` has `status: "ready"`
- `open_questions` is empty

## BUILD mode
Precondition: contract is `ready`
Output: code + tests + `reports/<id>.evidence.md`

Completion condition:
- tests pass
- evidence report maps every contract clause to a proving test/assertion
- contract can be updated to `status: "implemented"`

## Suggested “zip & review” cadence
1) Codex: create task file
2) GPT‑5.2 Pro: CONTRACT mode (ask/answer until ready)
3) Codex: BUILD mode (implement + run tests)
4) GPT‑5.2 Pro: review diffs and propose patches
