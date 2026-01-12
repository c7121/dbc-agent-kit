# Task 0005 — Init .cbd Structure in Target Repo

## Goal
Provide `cbd init` to create the standard `.cbd` directory structure in a target repo without
copying prompts or scaffolding reviews/epics.

## User story
As a reviewer, I want a one-time init command so that a target repo has the required `.cbd`
folders before I scaffold reviews or epics.

## Scope

### In scope
- Create `.cbd/` and these subdirectories: `requirements`, `tasks`, `contracts`, `bundles`,
  `reports`, `reviews`.
- Run from inside the target repo via `cargo run --manifest-path <kit>/xtask/Cargo.toml`.
- Be idempotent: re-running does not overwrite existing files.

### Out of scope
- Copying prompts, schemas, or templates into the target repo.
- Scaffolding review/epic/task artifacts (handled by other commands).
- Adding new dependencies without approval.

## Acceptance scenarios (examples-first)
Use Given/When/Then style.

### Scenario 1: Init a repo without .cbd
Given a repo without `.cbd`
When I run `cbd init`
Then `.cbd/requirements`, `.cbd/tasks`, `.cbd/contracts`, `.cbd/bundles`, `.cbd/reports`, and `.cbd/reviews` are created
And no prompt, schema, or template files are copied

### Scenario 2: Init is idempotent
Given a repo with an existing `.cbd` directory
When I run `cbd init` again
Then existing files are not overwritten
And missing directories are created if needed

## Context
- Running `cargo run --manifest-path <kit>/xtask/Cargo.toml -- cbd init` from the target repo
  should still operate on the target repo, not the kit repo.
- Epic: `.cbd/requirements/EP-0001-review-other-repos.md`

## Constraints
- Local filesystem only; do not read secrets.
- Keep behavior deterministic and side-effect minimal.

## Dependencies
- Task 0003: target repo support for cbd commands.

## Observability (optional)
- Log which directories were created.

## Unknowns
- None.
