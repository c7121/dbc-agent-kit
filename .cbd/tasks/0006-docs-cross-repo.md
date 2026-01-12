# Task 0006 — Docs for Cross-Repo Reviews

## Goal
Provide clear documentation and examples so a reviewer can apply this kit to another repo,
including init, prompt selection, and review scaffolding.

## User story
As a reviewer, I want a short, explicit workflow so that I can initialize a repo and run a review
without guessing prompt behavior.

## Scope

### In scope
- Document running `cargo run --manifest-path <kit>/xtask/Cargo.toml` inside a target repo.
- Document `cbd init`, `cbd new-review`, and how to run threat model templates.
- Document prompt selection order and how to force canonical prompts with `--local-prompts=false`.

### Out of scope
- Writing or modifying the prompts themselves.
- Publishing a standalone binary or packaging changes.
- Adding new dependencies without approval.

## Acceptance scenarios (examples-first)
Use Given/When/Then style.

### Scenario 1: Two-command quickstart
Given the README instructions
When a user follows the quickstart
Then they can run `cbd init` and `cbd new-review` in a target repo successfully

### Scenario 2: Prompt override behavior is clear
Given the README instructions
When a user looks for prompt behavior
Then they can see local-vs-canonical precedence and `--local-prompts=false`

## Context
- Epic: `.cbd/requirements/EP-0001-review-other-repos.md`
- Existing docs: `README.md` and `.cbd/README.md`

## Constraints
- Keep docs concise and accurate to implemented behavior.
- Do not hand-edit `CHANGELOG.md`.

## Dependencies
- Task 0003: target repo support for cbd commands.
- Task 0004: prompt discovery order.
- Task 0005: init .cbd structure.

## Observability (optional)
- None.

## Unknowns
- None.
