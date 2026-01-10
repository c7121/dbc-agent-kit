# AGENTS.md

## Purpose
This repo uses **Contract‑First Development (Design by Contract)** so agent work is **mechanical, not vibes**:
agents must write down **preconditions, postconditions, invariants, and error behavior** first, then prove them with tests/assertions.

## General
- You may read files in this repo and run **read‑only** commands without asking for permission.
- Ask blocking questions in **rounds of at most 3**. After 1–3 questions, stop and wait for answers, then repeat as needed.
- Prefer evidence over assumptions:
  - If something can be checked in the repo, check it.
  - If behavior depends on missing product decisions, ask (≤3 per round).

## Quickstart
- Scaffold a new task bundle:
  - `python .cbd/scripts/new_task.py --id 0001 --slug data-orchestration`
- CONTRACT mode produces/iterates:
  - `.cbd/contracts/<id>.contract.json`
  - `.cbd/bundles/<id>.bundle.json`
- BUILD mode implements and produces:
  - `.cbd/reports/<id>.evidence.md`

## Workflow (two modes)
### CONTRACT mode (no code changes)
Goal: converge on an implementable contract + a concrete task bundle an agent can loop over.

Outputs (create/update):
- `.cbd/contracts/<id>.contract.json`
- `.cbd/bundles/<id>.bundle.json`

Rules:
- Do **not** implement code, refactor, or make patches in this mode.
- Ask **≤3 blocking questions PER ROUND**, then stop and wait for the human’s answers. Repeat this question/answer cycle until `open_questions` is empty.
  - Do not ask a 4th question in the same round. After 1–3 questions, stop.
  - Each question must name: (1) which contract field(s) it blocks, and (2) what decision it changes.
- The contract may be set to `status: "ready"` only if `open_questions` is empty.

### BUILD mode (implementation + tests)
You may implement ONLY IF:
- `.cbd/contracts/<id>.contract.json` has `status: "ready"`
- `open_questions` is empty

Rules:
- Implement the smallest coherent diff that satisfies the contract.
- For every contract clause (pre/post/invariant/error/acceptance test), add:
  - a proving **test**, or
  - a runtime **assertion** (and ideally both for critical invariants).
- Produce/maintain an evidence report:
  - `.cbd/reports/<id>.evidence.md` mapping **contract clause → code/test location**
- Run checks and include output (or a link to logs) in the evidence report.
- No unrelated diffs.

## Project layout
- `.cbd/tasks/0001-*.md` — task intent + context (human seed)
- `.cbd/bundles/0001.bundle.json` — step checklist the agent loops over
- `.cbd/contracts/0001.contract.json` — Design‑by‑Contract artifact
- `.cbd/prompts/` — saved prompts for CONTRACT and BUILD modes
- `.cbd/reports/` — evidence packs (what proves what)
- `.cbd/scripts/` — small helpers (scaffold/validate)

## Conventional Commits + git-cliff
- All commits MUST follow Conventional Commits: `https://www.conventionalcommits.org/`.
- Use types like: `feat`, `fix`, `docs`, `chore`, `refactor`, `test`, `ci`, `build`.
- Include a scope when helpful (examples: `backend`, `frontend`, `cbd`, `contracts`).
- If a change is breaking, use `!` in the header and/or a `BREAKING CHANGE:` footer.
- When proposing changes, always include a suggested commit message in Conventional Commit format.
- Changelog is generated with git-cliff; do not hand-edit `CHANGELOG.md`.
- If asked to generate release notes/changelog, use git-cliff (and respect existing `cliff.toml` if present). If `cliff.toml` is not present, do not add it unless explicitly asked.

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
