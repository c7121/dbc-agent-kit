# Review <REVIEW_ID> — <TITLE>
Kind: PRD | Implementation | Both
Date: YYYY-MM-DD
Status: draft | in-progress | done

## What is being reviewed?
- PRD link / doc path:
- Repo / commit / PR link (if implementation):
- Owner / stakeholders:

## Scope
### In scope
- …

### Out of scope
- …

## Assumptions
- …

## Primary assets (initial)
List what matters most (money, keys, PII, positions, audit logs, etc.).
- …

## External systems / integrations
- …

## Security / privacy constraints (explicit)
- …

## Threat modeling frameworks
Select applicability (agents must apply all that are appropriate; minimum 1, maximum all 3):
- STRIDE: applicable (default)
- LINDDUN: applicable | not_applicable | unknown
- OWASP ASVS: applicable | not_applicable | unknown

Rationale / notes:
- …


## Known risks / incidents / history (if any)
- …

## System decomposition (components/containers)
If the system has multiple components/containers, list them here and also fill `components.md`.

- Component list (id + slug + short purpose):
  - C-001 (`api`): …
  - C-002 (`worker`): …
  - C-003 (`web`): …

Review granularity:
- component_mode: single | per_component

If `per_component`:
- create `.cbd/reviews/<review_id>-<slug>/components/<component_slug>/` folders
- copy templates from `.cbd/reviews/TEMPLATE/component/`
- expand the review bundle with per-component work items


## Architectural forks / ADRs
List ADRs relevant to this review.
- ADR-0001: docs/decisions/0001-...

## Review outputs
Artifacts in this folder:
- components.md (system decomposition; required for multi-component reviews)
- contract-map.md
- invariants.md
- assets.md
- dfd.md
- abuse-cases.md
- identity-authz.md
- secrets-keys.md
- reliability-failure.md
- observability-audit.md
- supply-chain.md
- threats.stride.md
- privacy.linddun.md (optional)
- controls.asvs.md (optional)
- findings.tasklist.json
