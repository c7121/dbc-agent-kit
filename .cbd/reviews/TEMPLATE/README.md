# Review templates

Copy this folder to start a new review run:

Recommended: scaffold a run using `xtask`:

```bash
cargo run --manifest-path xtask/Cargo.toml -- cbd new-review --id R-0001 --slug login-redesign
```

Manual:

1) Create folder: `.cbd/reviews/<review_id>-<slug>/`

2) Copy files from TEMPLATE into it:
   - `review.seed.md`
   - `review.bundle.json`
   - the artifact markdown files
   - `findings.tasklist.json`

3) Fill in `review.seed.md` and replace placeholders in `review.bundle.json` (`<REVIEW_ID>`, `<slug>`, `<TITLE>`).

4) Run the REVIEW agent **one bundle item at a time**:
   - complete `phases.plan[]` first
   - then execute `phases.review[]` work items

Frameworks:
- Apply STRIDE by default.
- Apply LINDDUN and ASVS when applicable (skip with justification when not).

Component reviews (multi-component systems):
- During PLAN, fill `components.md` and decide `component_mode`.
- If `component_mode=per_component`, create `.cbd/reviews/<review_id>-<slug>/components/<component_slug>/` folders
  and copy templates from `.cbd/reviews/TEMPLATE/component/`.
- Expand `review.bundle.json` with per-component work items so each component can be reviewed independently.
