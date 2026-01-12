# Task 0004 — Prompt Discovery and Local Override

## Goal
Resolve prompt paths deterministically when running from a target repo, using local prompts by default
and allowing canonical prompts from the kit repo via `--local-prompts=false`.

## User story
As an agent, I want prompt resolution to prefer local overrides but allow a canonical fallback so that
prompt selection is predictable across repos.

## Scope

### In scope
- Default prompt root is `<target>/.cbd/prompts` when it exists and `--local-prompts` is true.
- If `--local-prompts=false`, use `<kit>/.cbd/prompts` even when local prompts exist.
- If local prompts are missing, fall back to `<kit>/.cbd/prompts` and log that fallback.
- Log when `--local-prompts=false` is set and include the canonical prompt path in the message.

### Out of scope
- Copying prompts into the target repo.
- Network fetch or sync of prompts.
- Changing prompt content (only discovery/selection behavior).

## Acceptance scenarios (examples-first)
Use Given/When/Then style.

### Scenario 1: Local prompts by default
Given `.cbd/prompts` exists in the target repo
When the REQUIREMENTS prompt is loaded with `--local-prompts=true`
Then the local prompt is used

### Scenario 2: Force canonical prompts
Given `.cbd/prompts` exists in the target repo
When the CONTRACT prompt is loaded with `--local-prompts=false`
Then the canonical prompt from the kit repo is used
And the CLI logs that local prompts were disabled and shows the canonical path

### Scenario 3: Fallback when local prompts are missing
Given the target repo does not have `.cbd/prompts`
When the CONTRACT prompt is loaded
Then the canonical prompt from the kit repo is used
And the CLI logs that a discovered prompts path is being used and can be overridden by `--local-prompts=false`

## Context
- The kit repo stores canonical prompts at `.cbd/prompts`.
- Running `cargo run --manifest-path <kit>/xtask/Cargo.toml` from a target repo should still resolve
  the kit repo path for canonical prompts.
- Epic: `.cbd/requirements/EP-0001-review-other-repos.md`

## Constraints
- No new dependencies without approval.
- Local filesystem only; do not read secrets.

## Dependencies
- Task 0003: target repo support for cbd commands.

## Observability (optional)
- Log the resolved prompt root and whether it came from local or canonical prompts.

## Unknowns
- None.
