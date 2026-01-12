# Task 0007 - Bundle attribution fields and schema updates

## Goal
Bundle schema, template, and docs accept attribution metadata for plan/build items without breaking existing bundles.

## User story
As a maintainer, I want bundle artifacts to document attribution fields so I can capture provenance consistently.

## Scope

### In scope
- Update `.cbd/schemas/bundle.schema.json` to allow optional attribution fields on plan and build items:
  - `model` (string, stored as runtime-reported value)
  - `completed_at` (string, RFC3339 UTC)
  - `commits` (array of strings)
  - `reasoning_level` (optional string)
- Ensure `.cbd/bundles/TEMPLATE.bundle.json` remains valid against the updated schema.
- Update `.cbd/README.md` to mention attribution fields for bundle plan/build items.

### Out of scope
- Capturing attribution values in the CLI or tooling (handled in task 0008).
- Validating that done steps require attribution fields (handled in task 0009).

## Acceptance scenarios (examples-first)
Use Given/When/Then style.

### Scenario 1: Plan item attribution schema
Given a bundle plan item includes model, completed_at (RFC3339 UTC), commits, and optional reasoning_level
When it is validated against the bundle schema
Then it passes validation

### Scenario 2: Build item attribution schema
Given a bundle build item includes model, completed_at (RFC3339 UTC), commits, and optional reasoning_level
When it is validated against the bundle schema
Then it passes validation

### Scenario 3: Backward compatibility and template validity
Given a bundle without attribution fields and the TEMPLATE.bundle.json file
When they are validated against the bundle schema
Then both pass validation

### Scenario 4: Documentation updated
Given `.cbd/README.md`
When I read the bundle artifact description
Then it mentions the attribution fields (model, completed_at, commits, optional reasoning_level)

## Context
- Bundle schema: `.cbd/schemas/bundle.schema.json`
- Bundle template: `.cbd/bundles/TEMPLATE.bundle.json`
- Bundle artifacts: `.cbd/bundles/*.bundle.json`
- Artifact overview: `.cbd/README.md`

## Constraints
- No new dependencies without approval.
- `completed_at` must be RFC3339 UTC (string).
- `model` is stored exactly as reported by the agent runtime (no normalization).

## Dependencies
- None.

## Observability (optional)
- None.

## Unknowns
- None.
