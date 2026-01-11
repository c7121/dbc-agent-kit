# Contract‑First Development (DbC)

This folder contains the durable artifacts that make agent work **mechanical, not vibes**.

## Artifacts
- `.cbd/tasks/<id>-*.md` — human-written task seed (goal/context/constraints)
- `.cbd/contracts/<id>.contract.json` — Design-by-Contract spec (pre/post/invariants/errors/tests)
- `.cbd/bundles/<id>.bundle.json` — build runbook (phases + work items with `owner`, `proves`, `outputs`, `status`)
- `.cbd/reports/<id>.evidence.json` — machine-checkable mapping from clause ids → proving code/tests
- `.cbd/prompts/` — canonical prompts for CONTRACT and BUILD modes
- `.cbd/schemas/` — JSON Schemas for contracts and bundles

## Quickstart
Scaffold a new task bundle (task/contract/bundle/evidence templates):

```bash
cargo run --manifest-path xtask/Cargo.toml -- cbd new-task --id 0001 --slug data-orchestration
```

Optional interactive prompting (fills the task markdown):

```bash
cargo run --manifest-path xtask/Cargo.toml -- cbd new-task --id 0001 --slug data-orchestration --interactive
```

## Recommended cadence
1) Seed the task in `.cbd/tasks/<id>-*.md` (human intent + constraints).
2) Run CONTRACT mode using `.cbd/prompts/CONTRACT.md` until the contract is `ready`.
3) Run BUILD mode using `.cbd/prompts/BUILD.md` to implement/prove and run the hard gate.
4) Iterate via CONTRACT/BUILD as needed.

Note: this README is intentionally high-level to avoid duplicating the canonical mode prompts.
