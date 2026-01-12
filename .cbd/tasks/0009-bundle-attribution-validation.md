# Task 0009 - Validate attribution rules

## Goal
Ensure done plan/build items include required attribution while keeping legacy bundles valid.

## User story
As a maintainer, I want cbd verify to enforce attribution on done steps so provenance is reliable without breaking older bundles.

## Scope

### In scope
- Add attribution validation in `xtask cbd verify` for bundle plan/build items.
- Rule: if any plan/build item in a bundle includes attribution fields (model/completed_at/commits/reasoning_level), then every item with status "done" must include:
  - `model` (string, runtime-reported)
  - `completed_at` (RFC3339 UTC string)
- Optional fields:
  - `commits` may be omitted or an empty list; if present, it must be an array of strings.
  - `reasoning_level` may be omitted; if present, it must be a string.
- If no attribution fields exist in the bundle, skip attribution validation (legacy compatibility).
- Add tests covering validation failures and the legacy pass-through behavior.

### Out of scope
- Schema changes (task 0007).
- Prompt capture guidance (task 0008).
- Manual reasoning_level backfill (task 0010).

## Acceptance scenarios (examples-first)
Use Given/When/Then style.

### Scenario 1: Attribution enforced when present
Given a bundle where at least one plan/build item includes attribution fields
When `xtask cbd verify --id <id>` runs
Then any done item missing model or completed_at fails with a clear error referencing the item

### Scenario 2: Legacy bundle passes
Given a bundle with no attribution fields
When `xtask cbd verify --id <id>` runs
Then attribution validation is skipped and the bundle passes this check

### Scenario 3: Optional fields allowed
Given a done item with model and completed_at present
When commits is omitted or empty and reasoning_level is omitted
Then attribution validation passes

### Scenario 4: Invalid completed_at rejected
Given a done item with completed_at not in RFC3339 UTC
When `xtask cbd verify --id <id>` runs
Then it fails with a clear error

## Context
- Verification logic: `xtask/src/main.rs` (cbd_verify)
- Bundle schema: `.cbd/schemas/bundle.schema.json`
- Bundles: `.cbd/bundles/<id>.bundle.json`

## Constraints
- No new dependencies without approval.
- Model strings are stored exactly as reported by the runtime.

## Dependencies
- Task 0007 (schema/template/docs updates)
- Task 0008 (prompt capture guidance)

## Observability (optional)
- None.

## Unknowns
- None.
