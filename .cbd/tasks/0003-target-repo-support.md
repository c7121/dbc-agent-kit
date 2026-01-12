# Task 0003 — Target Repo Support for Cbd Commands

## Goal
When running cbd commands from inside a target repo via `cargo run --manifest-path <kit>/xtask/Cargo.toml`,
artifacts are read/written under that repo's `.cbd` directory, not the kit repo.

## User story
As a reviewer, I want to run cbd commands from inside a target repo so that artifacts are created in that repo.

## Scope

### In scope
- Determine the target repo root from the current working directory for cbd commands.
- Read and write `.cbd` artifacts relative to that target repo root.
- Non-init commands return a clear error when `.cbd` is missing and guide the user to run `cbd init`.

### Out of scope
- Adding a `--target` flag or other remote path execution.
- Copying prompts or other `.cbd` contents (handled by init and prompt discovery tasks).
- Changing verification gates or schemas.

## Acceptance scenarios (examples-first)
Use Given/When/Then style.

### Scenario 1: Write review artifacts in the target repo
Given I run `cargo run --manifest-path <kit>/xtask/Cargo.toml -- cbd new-review` inside a target repo
When the command runs
Then the review files are created under `.cbd/reviews`

### Scenario 2: Missing .cbd yields guidance
Given a repo without `.cbd`
When I run a cbd command other than `cbd init`
Then the command exits with a clear error and guidance to run `cbd init`

## Context
- Current behavior resolves the repo root from the xtask location, so running with a manifest path still writes to the kit repo.
- Epic: `.cbd/requirements/EP-0001-review-other-repos.md`

## Constraints
- No new dependencies without approval.
- Local filesystem only; do not read secrets.
- Keep path resolution deterministic.

## Dependencies
- None.

## Observability (optional)
- Log the resolved target repo root for troubleshooting (no sensitive paths beyond the local repo).

## Unknowns
- None.
