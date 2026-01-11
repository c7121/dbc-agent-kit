# Review templates

Copy this folder to start a new review run:

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
