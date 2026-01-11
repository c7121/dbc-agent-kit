# Secrets & key management review

## Secrets inventory
- SECRET-001: …
  - type: api_key/signing_key/password/token
  - storage: env/secret manager/kms/file
  - rotation: how + frequency
  - access control: who/what can read
  - logging risk: redaction enforced?

## Key custody / signing
If signing keys exist:
- where is the key held?
- how are signing requests authorized?
- audit trail for signing operations?
- limits on what can be signed?

## Build/CI exposure
- Are secrets ever present in CI logs?
- Are secrets present in build artifacts?
- Are secrets committed accidentally? prevention?

## Gaps / risks
- …
