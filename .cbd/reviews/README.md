# Reviews

This folder contains review templates and review runs. Reviews are designed to be executed as **isolated work items**
(one bundle item per session) so agents can work reliably without trying to “do everything at once”.

## Start a new review run (initiate the bundle)

1) Pick a review id + slug, e.g. `R-0001` + `login-redesign`.

2) Recommended: scaffold the review run using `xtask`:
```bash
cargo run --manifest-path xtask/Cargo.toml -- cbd new-review --id R-0001 --slug login-redesign
```

If you prefer manual scaffolding, use the template folder directly:

3) Create the review folder and copy the template files:
```bash
mkdir -p .cbd/reviews/R-0001-login-redesign
cp -R .cbd/reviews/TEMPLATE/* .cbd/reviews/R-0001-login-redesign/
```

4) Edit:
- `.cbd/reviews/R-0001-login-redesign/review.seed.md` (scope, assets, links)
- `.cbd/reviews/R-0001-login-redesign/review.bundle.json` (replace `<REVIEW_ID>`, `<slug>`, `<TITLE>`)

5) Run the REVIEW agent and complete bundle items one at a time:
- first complete the PLAN steps (`phases.plan[]`)
- then complete REVIEW work items (`phases.review[]`)

## Frameworks (minimum 1, maximum all 3)
The template includes work items for:
- STRIDE (security)
- LINDDUN (privacy)
- OWASP ASVS (control baseline)

Apply **all** frameworks that are appropriate for the system under review.
If a framework is not applicable, mark that work item as `skipped` with a clear justification.

## Outputs
The goal is to produce durable artifacts that:
- map contracts/boundaries and invariants
- perform threat modeling (STRIDE + LINDDUN when relevant)
- record control coverage (ASVS) when relevant
- generate a findings backlog that can be promoted into `.cbd/tasks/*`

## Multi-component reviews
If the system has multiple components/containers:
- Fill `components.md` during PLAN and set `component_mode` in `review.bundle.json`.
- Prefer `component_mode=per_component` so each component can be reviewed independently.
- Create `.cbd/reviews/<review_id>-<slug>/components/<component_slug>/` folders and copy templates from `.cbd/reviews/TEMPLATE/component/`.
- Expand the bundle with per-component work items (contract map, invariants, abuse cases, STRIDE, LINDDUN, ASVS) so agents can focus on one component per session.
