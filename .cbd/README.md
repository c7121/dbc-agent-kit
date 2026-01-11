# Contract‑First Development (DbC): mechanical, not vibes

This repo is set up for **Contract‑First Development (Design by Contract)** so that agent work becomes **mechanical, not vibes**.

Instead of letting an agent “declare done,” we require concrete artifacts:

- A **contract** that states *preconditions, postconditions, invariants, and error behavior*.
- A **task bundle** (runbook) that decomposes work into build-ready work items tied to contract clause ids (`phases.plan` + `phases.build`).
- **Acceptance tests** that prove the contract.
- An **evidence pack** mapping each contract clause to the code/tests that prove it.

## Quickstart
Scaffold a new task bundle (creates task/contract/bundle/evidence files):
```bash
cargo run --manifest-path xtask/Cargo.toml -- cbd new-task --id 0001 --slug data-orchestration
```

## How to use this with GPT‑5.2 Pro (Thinking) + Codex

Your working pattern can be:

1) Use Codex to create/update a task seed in `.cbd/tasks/<id>-*.md` (short, human‑written intent + context).
2) Use GPT‑5.2 Pro (Thinking) to run **CONTRACT mode**:
   - GPT produces `.cbd/contracts/<id>.contract.json` and `.cbd/bundles/<id>.bundle.json`
   - GPT asks up to 3 blocking questions per round (then waits) until the contract is `ready`
3) Use Codex to run **BUILD mode**:
   - implement the contract
   - execute the bundle’s `phases.build` work items
   - add tests/assertions
   - generate `.cbd/reports/<id>.evidence.json` (optional: `.evidence.md`)
   - run checks (Rust + TS)
   - run the hard gate: `cargo run --manifest-path xtask/Cargo.toml -- cbd verify --id <id>`
4) Optionally upload the updated repo back to GPT‑5.2 Pro (Thinking) for review + patch suggestions.

The key is the contract file: it’s the handoff artifact between “planning/review” and “execution.”
