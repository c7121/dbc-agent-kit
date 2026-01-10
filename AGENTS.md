# AGENTS.md

## Purpose
This repo uses **Contract‑First Development (Design by Contract)** so agent work is **mechanical, not vibes**:
agents must write down **preconditions, postconditions, invariants, and error behavior** first, then prove them with tests/assertions.

## General
- You may read files in this repo and run **read‑only** commands without asking for permission.
- Ask questions in batches of **at most 3**. After asking, stop and wait for answers before asking more.
- Prefer evidence over assumptions:
  - If something can be checked in the repo, check it.
  - If behavior depends on missing product decisions, ask (max 3).

## Quickstart
- Scaffold a new task bundle:
  - `python scripts/new_task.py --id 0001 --slug data-orchestration`
- CONTRACT mode produces/iterates:
  - `contracts/<id>.contract.json`
  - `bundles/<id>.bundle.json`
- BUILD mode implements and produces:
  - `reports/<id>.evidence.md`

## Workflow (two modes)
### CONTRACT mode (no code changes)
Goal: converge on an implementable contract + a concrete task bundle an agent can loop over.

Outputs (create/update):
- `contracts/<id>.contract.json`
- `bundles/<id>.bundle.json`

Rules:
- Do **not** implement code, refactor, or make patches in this mode.
- Ask **at most 3 blocking questions** per round.
  - Each question must name: (1) which contract field(s) it blocks, and (2) what decision it changes.
- The contract may be set to `status: "ready"` only if `open_questions` is empty.

### BUILD mode (implementation + tests)
You may implement ONLY IF:
- `contracts/<id>.contract.json` has `status: "ready"`
- `open_questions` is empty

Rules:
- Implement the smallest coherent diff that satisfies the contract.
- For every contract clause (pre/post/invariant/error/acceptance test), add:
  - a proving **test**, or
  - a runtime **assertion** (and ideally both for critical invariants).
- Produce/maintain an evidence report:
  - `reports/<id>.evidence.md` mapping **contract clause → code/test location**
- Run checks and include output (or a link to logs) in the evidence report.
- No unrelated diffs.

## Project layout
- `tasks/0001-*.md` — task intent + context (human seed)
- `bundles/0001.bundle.json` — step checklist the agent loops over
- `contracts/0001.contract.json` — Design‑by‑Contract artifact
- `prompts/` — saved prompts for CONTRACT and BUILD modes
- `reports/` — evidence packs (what proves what)
- `scripts/` — small helpers (scaffold/validate)

## Rust backend commands
- Format: `cargo fmt`
- Lint: `cargo clippy -- -D warnings`
- Test: `cargo test`

## TypeScript frontend commands
Do not guess the package manager; detect it:
- If `pnpm-lock.yaml` exists: use `pnpm`
- Else if `yarn.lock` exists: use `yarn`
- Else: use `npm`

Typical commands (verify in `package.json` scripts):
- Install: `pnpm install` / `yarn` / `npm ci`
- Lint: `pnpm lint` / `yarn lint` / `npm run lint`
- Test: `pnpm test` / `yarn test` / `npm test`
- Build: `pnpm build` / `yarn build` / `npm run build`

## Definition of Done
- Contract status = `implemented`
- Acceptance tests listed in the contract are implemented and passing
- Evidence report exists and is consistent with the diff
- No unrelated diffs

## Safety
- Never read secrets or `.env` files (or anything that looks like credentials).
- No dependency additions without approval.
- No schema migrations without an explicit plan + rollback.
