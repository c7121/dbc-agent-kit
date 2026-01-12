# Task 0008 - Capture attribution on plan/build completion

## Goal
When a plan step or build work item is marked done, the bundle item records model, completed_at (RFC3339 UTC), and optional commits (and reasoning_level if available).

## User story
As a reviewer, I want completed plan/build steps to include attribution so I can audit provenance without digging through git history.

## Scope

### In scope
- Update `.cbd/prompts/CONTRACT.md` to require stamping attribution fields on plan steps when status moves to done:
  - model (runtime string, no normalization)
  - completed_at (RFC3339 UTC)
  - commits (optional list of hashes)
  - reasoning_level (optional, when known)
- Update `.cbd/prompts/BUILD.md` to require the same stamping for build work items when status moves to done.
- Provide a canonical timestamp example in the prompts (RFC3339 UTC), and note that commits are optional.

### Out of scope
- Schema changes (task 0007).
- Validation that done steps require attribution (task 0009).
- Inferring commit lists automatically.

## Acceptance scenarios (examples-first)
Use Given/When/Then style.

### Scenario 1: Plan step attribution capture
Given a plan step is marked done
When the agent updates the bundle following CONTRACT prompt guidance
Then the plan item includes model, completed_at (RFC3339 UTC), and optional commits (and reasoning_level if known)

### Scenario 2: Build work item attribution capture
Given a build work item is marked done
When the agent updates the bundle following BUILD prompt guidance
Then the work item includes model, completed_at (RFC3339 UTC), and optional commits (and reasoning_level if known)

### Scenario 3: Optional commit list
Given no commit hashes are available
When a step is marked done
Then the commits field may be omitted or set to an empty list

## Context
- Contract prompt: `.cbd/prompts/CONTRACT.md`
- Build prompt: `.cbd/prompts/BUILD.md`
- Bundle format: `.cbd/bundles/<id>.bundle.json`

## Constraints
- `completed_at` must be RFC3339 UTC.
- `model` must be stored as the runtime-reported string (no normalization).
- Do not add dependencies without approval.

## Dependencies
- Task 0007 (schema/template/docs updates).

## Observability (optional)
- None.

## Unknowns
- None.
