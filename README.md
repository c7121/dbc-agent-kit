# dbc_agent_kit

This repo is set up for **Contract‑First Development (Design by Contract)** so agent work is **mechanical, not vibes**.

## What this gives you

Three tight loops:

```mermaid
flowchart LR
  subgraph REQ["REQUIREMENTS loop (DISCOVER)"]
    R1["seed epic"]
    R2["ask 2-3 questions (round)"]
    R3["update epic + tasklist + task seeds"]
    Rready{tasks ready?}
    R1 --> R2 --> R3 --> Rready
    Rready -- "no" --> R2
    Rready -- "yes" --> C1
  end

  subgraph CONTRACT["CONTRACT loop (PLAN)"]
    C1["read context"]
    C2["draft contract"]
    C3["critique (may add questions)"]
    Cready{ready?}
    Cq{open_questions?}
    C5["ask 2-3 questions (round)"]
    C6["wait for answers"]
    C4["revise (may add questions)"]
    C7["status: ready"]
    C1 --> C2 --> C3 --> Cready
    Cready -- "yes" --> C7
    Cready -- "no" --> Cq
    Cq -- "yes" --> C5 --> C6 --> Cq
    Cq -- "no" --> C4 --> C3
    C7 --> C1
  end

  subgraph BUILD["BUILD loop (CREATE + VERIFY)"]
    B1["execute phases.build work items"]
    B2["produce .cbd/reports/<id>.evidence.json"]
    B3["run cbd verify"]
    Bq{verify OK?}
    B4["status: implemented"]
    B1 --> B2 --> B3 --> Bq
    Bq -- "no" --> B1
    Bq -- "yes" --> B4
    B4 --> B1
  end
```

- CONTRACT loop output: `.cbd/contracts/<id>.contract.json` + a build-ready `.cbd/bundles/<id>.bundle.json`
- BUILD loop hard gate: `cargo run --manifest-path xtask/Cargo.toml -- cbd verify --id <id>`

The bundle is the **handoff runbook** from CONTRACT to BUILD.

See:
- `.cbd/README.md` for artifacts and quickstart
- `.cbd/prompts/REQUIREMENTS.md`, `.cbd/prompts/CONTRACT.md`, and `.cbd/prompts/BUILD.md` for canonical agent instructions

## Quickstart

Seed an epic requirements doc (PRD-level):

```bash
cargo run --manifest-path xtask/Cargo.toml -- cbd new-epic --id EP-0001 --slug your-epic --interactive
```

Then scaffold a task bundle:

```bash
cargo run --manifest-path xtask/Cargo.toml -- cbd new-task --id 0001 --slug data-orchestration
```
