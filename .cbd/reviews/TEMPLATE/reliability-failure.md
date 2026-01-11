# Reliability & failure semantics

## Failure modes
List expected failures and intended behavior:
- network timeouts
- partial writes
- retries/backoff
- idempotency
- dependency outages
- restart/recovery
- clock skew
- concurrency races

## Idempotency & retries
- What operations are idempotent?
- What keys/identifiers exist to enforce idempotency?

## Backpressure / DoS resilience
- rate limits
- input validation
- resource bounds

## Gaps / risks
- …
