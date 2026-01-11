# Invariants catalog

Invariants are “must always be true” properties.

## System invariants
List global invariants, each with:
- invariant_id: INV-001
- statement
- enforcement point(s): where in code/process
- detection: how we know it failed (logs/tests/alerts)
- severity: low/med/high/critical

Example:
- INV-001: No secret leakage in logs/stdout/stderr.
  - enforcement: logging redaction + tests
  - detection: log scanning tests
  - severity: high

## Domain invariants
Business rules that must not be violated:
- …

## Safety invariants (high stakes)
If violated: financial loss, privacy violation, compliance breach, etc.
- …

## Open questions / ambiguous invariants
- …
