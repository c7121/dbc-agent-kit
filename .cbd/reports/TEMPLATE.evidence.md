# Evidence Pack — <ID>

Canonical (machine-checkable) evidence:
- `.cbd/reports/<ID>.evidence.json`

## Repo Recon
- Relevant files:
  - <path>: <why it's relevant>

## Contract clause → proof mapping
Source of truth is `.cbd/reports/<ID>.evidence.json` (`clause_proofs[]`).
This markdown file is optional/human-friendly; keep it consistent with the JSON.

### Preconditions
- <CLAUSE_ID>: <statement> → <file:line>, <test/assertion>

### Postconditions
- <CLAUSE_ID>: <statement> → <file:line>, <test/assertion>

### Invariants
- <CLAUSE_ID>: <statement> → <file:line>, <test/assertion>

### Errors
- <CLAUSE_ID> (<ERROR_CODE>): <when> → <handler>, <test>

## Commands run + outputs

### Rust
- cargo fmt: <output>
- cargo clippy -- -D warnings: <output>
- cargo test: <output>

### TypeScript
- <install cmd>: <output>
- <lint cmd>: <output>
- <test cmd>: <output>
- <build cmd>: <output>

## Checklist
- [ ] Acceptance criteria met
- [ ] Tests added/updated
- [ ] Tests passing
- [ ] No unrelated diffs
- [ ] No secrets touched/logged
