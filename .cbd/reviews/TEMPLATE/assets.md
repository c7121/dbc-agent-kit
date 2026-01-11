# Asset inventory & data classification

## Assets (what we must protect)
List assets and why they matter.
Examples:
- credentials, API keys, signing keys
- user identifiers / PII
- funds / positions / balances
- audit logs
- proprietary data/models

For each asset:
- asset_id: ASSET-001
- description
- owner
- sensitivity: public/internal/secret/PII
- storage locations (db/bucket/env/kms/etc.)
- in-transit paths
- retention requirements
- deletion requirements (if applicable)

## Data classification rules
Define what “secret” means for this system.
- …

## Redaction policy
What must never appear in logs/errors?
- …
