# Evidence Pack — <ID>

## Repo Recon
- Relevant files:
  - <path>: <why it's relevant>

## Contract clause → proof mapping
For each clause below, link to:
- code location(s)
- test(s) and/or runtime assertion(s)

### Preconditions
- <clause> → <file:line>, <test>

### Postconditions
- <clause> → <file:line>, <test>

### Invariants
- <clause> → <file:line>, <test/assert>

### Errors
- <ERROR_CODE>: <when> → <handler>, <test>

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
