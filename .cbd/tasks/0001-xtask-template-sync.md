# Task 0001 — Align xtask interactive scaffolding to .cbd templates

## Goal
Ensure `xtask` interactive scaffolding uses `.cbd` template files as the source of truth so template edits are reflected in generated artifacts.

## User story
As a maintainer, I want interactive scaffolding to render from `.cbd` templates so updating templates does not require updating `xtask` code.

## Scope

### In scope
- Use `.cbd` templates as the base text for interactive scaffolding in `cbd new-task`, `cbd new-epic`, and `cbd new-review`.
- Map interactive prompt responses into the matching template sections without altering headings.
- Add tests that prove template-driven rendering and placeholder replacement.

### Out of scope
- Change non-interactive scaffolding behavior.
- Modify `.cbd` template content or schemas.
- Add new CLI flags or subcommands.

## Acceptance scenarios (examples-first)
Use Given/When/Then style.

### Scenario 1: new-task reflects template edits
Given `.cbd/tasks/TEMPLATE.md` contains a unique marker line
When I run `cbd new-task --interactive` with valid inputs
Then the generated task includes the marker line and replaces placeholder tokens with the provided values

### Scenario 2: new-epic inserts prompted content
Given I provide non-empty responses for Problem, Goals, and Success metrics prompts
When I run `cbd new-epic --interactive`
Then those responses appear under the matching template headings and headings remain unchanged

### Scenario 3: new-review preserves template defaults
Given I leave optional prompts blank in `cbd new-review --interactive`
When the review seed is generated
Then the corresponding sections retain the default template text

## Context
- `xtask/src/main.rs` contains hard-coded renderers: `render_task_markdown`, `render_epic_markdown`, `render_review_seed_markdown`.
- `.cbd/tasks/TEMPLATE.md`, `.cbd/requirements/TEMPLATE.epic.md`, and `.cbd/reviews/TEMPLATE/review.seed.md` are the source templates.

## Constraints
- No new dependencies.
- Keep template files unchanged.

## Dependencies
- (none)

## Observability (optional)
- Logs (not applicable)
- Metrics
- Traces

## Unknowns
- (none)
