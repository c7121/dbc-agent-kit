# Workflow

This workflow is designed to work well with:
- **GPT‑5.2 Pro (Thinking)** for planning, contracts, and review
- **Codex** (and/or Claude Code) for implementation and running commands

## Files that drive the process
- `.cbd/tasks/<id>-*.md` — short task seed (goal/context/constraints)
- `.cbd/contracts/<id>.contract.json` — Design‑by‑Contract spec (pre/post/invariants/errors/tests)
- `.cbd/bundles/<id>.bundle.json` — the step-by-step loop the agent must execute
- `.cbd/reports/<id>.evidence.json` — machine-checkable mapping from clause ids → proving code/tests
- (optional) `.cbd/reports/<id>.evidence.md` — human-friendly narrative + command output

## CONTRACT mode
Input: `.cbd/tasks/<id>-*.md` + repo
Output: `.cbd/contracts/<id>.contract.json`, `.cbd/bundles/<id>.bundle.json`, and up to 3 blocking questions per round (then wait; repeat until `open_questions` is empty)

Completion condition:
- `.cbd/contracts/<id>.contract.json` has `status: "ready"`
- `open_questions` is empty

## BUILD mode
Precondition: contract is `ready`
Output: code + tests + `.cbd/reports/<id>.evidence.json` (and optionally `.cbd/reports/<id>.evidence.md`)

Completion condition:
- tests pass
- evidence pack maps every contract clause id to a proving test/assertion
- contract can be updated to `status: "implemented"`

## Suggested “zip & review” cadence
1) Codex: create task file
2) GPT‑5.2 Pro: CONTRACT mode (ask/answer until ready)
3) Codex: BUILD mode (implement + prove + run `cargo run --manifest-path xtask/Cargo.toml -- cbd verify --id <id>`)
4) GPT‑5.2 Pro: review diffs and propose patches
