# Epic EP-0001 — Review Other Repos
Status: draft

## Problem
- This kit is tied to its own repo, so using it to review other codebases requires manual setup or ad-hoc copying.
- Reviewers and agents cannot reliably find prompts when working in a different repo.

## Target users / stakeholders
- Primary user: reviewers applying contract-first reviews and threat models to other repos.
- Secondary users: agents executing REQUIREMENTS/CONTRACT/BUILD or REVIEW prompts.
- Stakeholders: maintainers of this kit and target repo owners.

## Goals (outcomes)
- Run cbd commands from within a target repo via `cargo run --manifest-path <kit>/xtask/Cargo.toml` and write artifacts into that repo's .cbd.
- Keep prompts canonical in this repo, while allowing local .cbd/prompts overrides in target repos.
- Discover canonical prompts from the kit repo with `--local-prompts=false` to force canonical prompts, and log when fallback is used.
- Make applying the kit to a new repo a short, repeatable workflow.

## Success metrics
- A reviewer can initialize a target repo and scaffold a review or epic in under 5 minutes.
- Prompt resolution is deterministic: local .cbd/prompts overrides canonical prompts; fallback works when local is absent.
- No manual copying of prompts is required to use the kit.

## Scope
### In scope
- Add target repo support to cbd commands so they run from the target repo and write under local .cbd using `cargo run --manifest-path <kit>/xtask/Cargo.toml`.
- Define prompt resolution order: target repo .cbd/prompts overrides canonical prompts in this repo, with `--local-prompts=false` to force canonical and a log when discovery is used.
- Provide a minimal init path that creates required .cbd directories only, without copying prompts.
- Document the workflow for applying reviews and threat models to other repos.

### Out of scope / Non-goals
- Copying the entire .cbd/prompts tree into target repos by default.
- Network fetches or sync of prompts.
- Schema migrations or changes to verification gates.
- Adding new dependencies without explicit approval.

## Constraints
- Security/privacy: do not read secrets; operate on local filesystem only.
- Compliance/legal: none identified.
- Performance/latency: local CLI commands should complete quickly.
- Reliability/availability: deterministic results; init should be idempotent.
- Cost: avoid new dependencies unless approved.
- Operational constraints (deploy environment, observability expectations): local dev use; should work when run from the target repo.

## Integrations / dependencies
- External systems: local filesystem; git optional for path resolution.
- Auth model: none.
- Data sources: target repo .cbd artifacts; canonical prompts in this repo under .cbd/prompts (discovered via the kit repo path).
- Data sinks: target repo .cbd artifacts and reports.

## User stories
1) As a reviewer, I want a single command to initialize .cbd in a target repo and scaffold a review, so I can start quickly.
2) As an agent, I want prompt resolution to prefer target repo .cbd/prompts and fall back to canonical prompts, so local overrides are honored.
3) As a maintainer, I want to update prompts in this repo and have other repos pick them up automatically when they do not override, so I avoid drift.

## Acceptance scenarios (examples-first)
Use Given/When/Then style.

### Scenario 1: Init a target repo without copying prompts
Given a target repo with no .cbd directory
When I run `cargo run --manifest-path <kit>/xtask/Cargo.toml -- cbd init` inside that repo
Then .cbd/ is created with required artifact folders
And .cbd/prompts is not copied by default

### Scenario 2: Local prompts override canonical prompts
Given a target repo with .cbd/prompts/REQUIREMENTS.md
When an agent loads the REQUIREMENTS prompt for that repo
Then the local prompt is used instead of the canonical prompt

### Scenario 3: Force canonical prompts with a flag
Given a target repo with .cbd/prompts/CONTRACT.md
When an agent loads the CONTRACT prompt with `--local-prompts=false`
Then the canonical prompt from this repo is used
And the CLI logs that local prompts were disabled and shows the canonical path

### Scenario 4: Canonical prompts used when no local override exists
Given a target repo with no .cbd/prompts directory
When an agent loads the CONTRACT prompt for that repo
Then the canonical prompt from this repo is used
And the CLI logs that a discovered prompts path is being used and can be overridden by `--local-prompts=false`

### Scenario 5: Write review artifacts into the target repo
Given I run `cargo run --manifest-path <kit>/xtask/Cargo.toml -- cbd new-review` inside the target repo
When the command runs
Then the review files are created under .cbd/reviews

## Open questions
(Blocking unknowns; the REQUIREMENTS agent asks 2–3 per round.)

- None.

## Architectural forks (ADRs)
Only if needed. Link MADR files under `docs/decisions/`.

- None yet.

## C4 notes (optional)
- Not required for this epic.

## Task backlog
(High-level list. The machine-readable version lives in `<epic>.tasklist.json`.)

- T-0003: Target repo support for cbd commands (ready)
- T-0004: Prompt discovery order (ready)
- T-0005: Init command for .cbd structure (ready)
- T-0006: Docs and examples for cross-repo reviews (ready)
