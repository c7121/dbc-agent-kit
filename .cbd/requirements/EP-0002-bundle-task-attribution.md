# Epic EP-0002 - Bundle Task Attribution and Timestamp
Status: draft

## Problem
- Bundle plan/build entries do not record which agent/model completed a step or when it completed.
- Reviewers cannot link completed work to commits without manual git history lookup.

## Target users / stakeholders
- Primary user: maintainers/reviewers validating contract/build evidence.
- Secondary users: agents running CONTRACT/BUILD loops.
- Stakeholders: repo maintainers who need auditability.

## Goals (outcomes)
- Record model and completion timestamp for every plan step and build work item marked done.
- Record commit hashes (multiple allowed) associated with each completed step when commits exist.
- Keep bundles readable and compatible with existing tooling.

## Success metrics
- For any bundle with done steps, each done item includes model + completed_at, and commits when provided.
- Bundles without attribution fields remain valid and verifiable.
- Attribution fields are populated consistently by the CLI/tooling (no manual edits in the typical flow).

## Scope
### In scope
- Extend bundle schema to include per-item attribution metadata for plan and build arrays.
- Capture attribution when a step transitions to done (model from agent runtime, completed_at in RFC3339 UTC, commits[]).
- Support an optional reasoning_level (thinking level) field when the runtime provides it; allow later manual addition.
- Update templates/docs/tests to reflect the new fields and readiness rules.

### Out of scope / Non-goals
- Identity verification or authentication of agents.
- Automatic inference of commit lists from remote services.
- Backfilling historical bundles beyond optional manual edits.

## Constraints
- Security/privacy: do not capture secrets; only store model identifier, timestamp (RFC3339 UTC), optional reasoning_level, and commit hashes.
- Model identifiers are stored exactly as reported by the agent runtime (no normalization).
- Compliance/legal: none identified.
- Performance/latency: attribution capture should be local and fast.
- Reliability/availability: backward-compatible JSON; verification remains deterministic.
- Cost: no new dependencies without approval.
- Operational constraints (deploy environment, observability expectations): local CLI usage.

## Integrations / dependencies
- External systems: local git repo for commit hashes if recorded.
- Auth model: none.
- Data sources: bundle JSON, agent runtime model identifier, optional reasoning_level, optional commit hash list when provided.
- Data sinks: updated bundle JSON files.

## User stories
1) As a reviewer, I want to see which model completed each plan/build step and when, so I can audit provenance.
2) As a maintainer, I want commit hashes attached to completed work items, so I can trace code changes to contract evidence.
3) As an agent, I want attribution captured automatically when I mark steps done, so I do not hand-edit JSON.

## Acceptance scenarios (examples-first)
Use Given/When/Then style.

### Scenario 1: Plan step attribution captured
Given a bundle plan step marked done
When the step is written to the bundle
Then the step includes the agent model and a completed_at timestamp in RFC3339 UTC
And any associated commit hashes are recorded when provided as a list

### Scenario 2: Build work item attribution captured
Given a build work item transitions to done
When the CLI records completion
Then the work item includes the agent model and completed_at timestamp in RFC3339 UTC
And commit hashes are recorded when provided as a list

### Scenario 3: Backward compatibility
Given an existing bundle without attribution fields
When the bundle is loaded or verified
Then it remains valid and processing does not fail

## Open questions
(Blocking unknowns; the REQUIREMENTS agent asks 2-3 per round.)

- None.

## Architectural forks (ADRs)
Only if needed. Link MADR files under `docs/decisions/`.

- None yet.

## C4 notes (optional)
- Not required for this epic.

## Task backlog
(High-level list. The machine-readable version lives in `<epic>.tasklist.json`.)

- T-0007: Define bundle attribution fields and schema updates (ready)
- T-0008: Capture attribution on plan/build completion in CLI/tooling (ready)
- T-0009: Add validation/tests for attribution rules (ready)
- T-0010: Optional manual reasoning_level backfill/input (deferred)
