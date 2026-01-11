# System decomposition (components/containers)

This file exists to prevent “one giant review” when the system has multiple components.

## Components list
List each component with a stable id and slug.

Example:

- C-API (`api`): Public HTTP API service
  - responsibilities:
  - entrypoints:
  - data stores:
  - external deps:
  - trust boundaries crossed:
  - notes:

- C-WORKER (`worker`): Background job runner
  - responsibilities:
  - entrypoints:
  - data stores:
  - external deps:
  - trust boundaries crossed:
  - notes:

## Review granularity decision
Choose one:
- component_mode: single | per_component

Guidance:
- If >1 component/container, prefer `per_component`.
- In `per_component` mode, create:
  - `components/<component_slug>/contract-map.md`
  - `components/<component_slug>/invariants.md`
  - `components/<component_slug>/abuse-cases.md`
  - `components/<component_slug>/threats.stride.md`
  - `components/<component_slug>/privacy.linddun.md` (if applicable)
  - `components/<component_slug>/controls.asvs.md` (if applicable)

## Cross-cutting concerns
List anything that must be reviewed globally across components:
- shared authn/authz
- shared secrets/key management
- shared deployment/runtime environment
- shared logging/metrics/audit trails
- shared dependency/supply-chain risk


## Bundle expansion checklist (per_component mode)
If `component_mode=per_component`:
1) Create folders: `components/<component_slug>/`
2) Copy templates: `.cbd/reviews/TEMPLATE/component/*` into each folder
3) In `review.bundle.json`, set:
   - `component_mode: "per_component"`
   - optionally fill `components: [...]`
4) Expand `phases.review[]` with per-component work items so each component can be reviewed independently.
