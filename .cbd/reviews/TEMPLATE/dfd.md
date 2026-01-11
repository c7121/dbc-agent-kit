# Data flow & trust boundaries (DFD notes)

Describe the system as a data flow diagram (DFD) in text.
(You can later convert to a diagram if desired.)

## External entities
- EE-001: …
- EE-002: …

## Processes (components)
- P-001: …
- P-002: …

## Data stores
- DS-001: …
- DS-002: …

## Data flows
For each flow:
- FLOW-001: from X → Y
- data: what data moves
- protocol: http/grpc/jsonrpc/etc.
- trust boundary crossed? yes/no
- authn/authz: how identity is established/propagated
- integrity: how tampering is prevented/detected
- confidentiality: encryption? redaction?

## Trust boundaries
List boundaries explicitly:
- TB-001: …
- TB-002: …

## Notes
- …
