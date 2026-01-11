# Contract surfaces & boundaries

## Entry points (contracts)
List externally visible entrypoints and their contracts:
- HTTP endpoints (method + path)
- CLI commands
- message/event handlers
- scheduled jobs / cron
- admin tools

For each entrypoint:
- Inputs (schema, auth context)
- Outputs (response/event, errors)
- Preconditions (validation, authz, existence, idempotency)
- Postconditions (state changes, events)
- Trust boundary crossed? (Y/N)
- Data classification (public/internal/secret/PII)

## Trust boundaries
Where does the trust level change?
Examples:
- internet → API gateway
- API → internal service
- service → database
- service → third-party API

## External dependencies
- services/APIs:
- databases/queues:
- key management / signer:
- observability systems:

## Boundary enforcement notes
Where are contracts enforced in code?
- runtime validators
- auth middleware
- schema validators
- typed domain model
- tests

## Gaps / risks
- ...
