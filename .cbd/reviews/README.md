# Reviews

This folder contains review templates and review runs.

## Template
Use `.cbd/reviews/TEMPLATE/` to start a new review run:
- copy the template folder into `.cbd/reviews/<review_id>-<slug>/`
- fill in `review.seed.md`
- update `review.bundle.json` (optional: tailor work items)
- then run the REVIEW agent one work item at a time

## Outputs
The goal is to produce durable artifacts that:
- map contracts/boundaries and invariants
- perform threat modeling (STRIDE; LINDDUN when relevant)
- record control coverage (ASVS) when relevant
- generate a findings backlog that can be promoted into `.cbd/tasks/*`
