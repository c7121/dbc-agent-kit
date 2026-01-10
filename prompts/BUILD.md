You are in BUILD mode.

You may implement ONLY IF:
- contracts/<id>.contract.json has status "ready"
- open_questions is empty

Rules:
- Implement the smallest coherent diff that satisfies the contract.
- For each contract clause (pre/post/invariant/error/acceptance test), add:
  - a proving test, or
  - a runtime assertion (and ideally both for critical invariants).
- Produce reports/<id>.evidence.md mapping clause -> code/test locations.
- Run Rust + TS checks and paste outputs into the evidence report.
- Do not declare done unless acceptance tests pass.
