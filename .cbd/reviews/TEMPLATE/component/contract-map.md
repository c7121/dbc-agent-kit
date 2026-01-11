# Component contract surfaces & boundaries

Component: <component_slug>

## Entry points (contracts)
List externally visible entrypoints for this component and their contracts:
- HTTP endpoints
- message/event handlers
- jobs/cron
- internal service APIs (if applicable)

For each entrypoint:
- Inputs (schema, auth context)
- Outputs (response/event, errors)
- Preconditions / Postconditions
- Trust boundary crossed? (Y/N)
- Data classification (public/internal/secret/PII)

## Dependencies
- upstream callers:
- downstream services/APIs:
- data stores:
- secrets/signing systems:

## Gaps / risks
- ...
