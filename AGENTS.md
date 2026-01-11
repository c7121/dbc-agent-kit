# AGENTS.md

## Purpose
This repo uses **Contract‑First Development (Design by Contract)** so agent work is **mechanical, not vibes**.

The contract is the source of truth (preconditions / postconditions / invariants / error behavior), and BUILD must prove compliance with evidence.

## Canonical mode prompts
Mode-specific instructions live in one place to avoid drift:
- CONTRACT mode: `.cbd/prompts/CONTRACT.md`
- BUILD mode: `.cbd/prompts/BUILD.md`

Before doing any work, **read the relevant mode prompt and follow it exactly**.

## Global rules (apply to all modes)
- You may read files in this repo and run read-only commands without asking for permission.
- Ask blocking questions in rounds of **at most 3 at a time**, then STOP and wait for answers. Repeat rounds as needed.
- Prefer evidence over assumptions: verify repo facts (search/read) before guessing.

## Hard gate
CI/verification must use:
- `cargo run --manifest-path xtask/Cargo.toml -- cbd verify --id <id>`

`xtask cbd verify` fails if:
- contract is not `ready` or has `open_questions`
- any contract clause id lacks proof in `.cbd/reports/<id>.evidence.json`
- Rust checks fail (`cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`)
- TS checks fail (only if a frontend `package.json` exists, and only for scripts that exist: `lint`, `test`, `build`)

## Project layout
- `.cbd/tasks/` — task intent + context (human seed)
- `.cbd/contracts/` — Design‑by‑Contract artifacts
- `.cbd/bundles/` — build runbooks (phases + build work items)
- `.cbd/reports/` — evidence packs (what proves what)
- `.cbd/prompts/` — canonical prompts for CONTRACT and BUILD
- `.cbd/schemas/` — JSON Schemas
- `xtask/` — Rust automation helpers (scaffold/verify)

## Conventional Commits + git-cliff
- All commits MUST follow Conventional Commits: `https://www.conventionalcommits.org/`.
- Use types like: `feat`, `fix`, `docs`, `chore`, `refactor`, `test`, `ci`, `build`.
- Include a scope when helpful (examples: `backend`, `frontend`, `cbd`, `contracts`).
- If a change is breaking, use `!` in the header and/or a `BREAKING CHANGE:` footer.
- When proposing changes, always include a suggested commit message in Conventional Commit format.
- Changelog is generated with git-cliff; do not hand-edit `CHANGELOG.md`.

## Safety
- Never read secrets or `.env` files (or anything that looks like credentials).
- No dependency additions without approval.
- No schema migrations without an explicit plan + rollback.
