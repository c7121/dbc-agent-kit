# Identity / AuthN / AuthZ review

## Identity model
- Who are the actors? (users/services)
- How is identity established? (tokens, keys, mTLS, sessions)
- How is identity propagated downstream?

## Authorization model
- What permissions/roles exist?
- Where is authz enforced? (gateway/service/db)
- How is least privilege enforced?

## Token/session details
- token lifetime, refresh, revocation
- replay protections
- anti-CSRF (if web)
- service-to-service auth (if any)

## Gaps / risks
- …
