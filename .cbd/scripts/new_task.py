#!/usr/bin/env python3
"""Scaffold a new Contract‑First task bundle.

Creates:
  - .cbd/tasks/<id>-<slug>.md
  - .cbd/contracts/<id>.contract.json
  - .cbd/bundles/<id>.bundle.json
  - .cbd/reports/<id>.evidence.md

By copying templates in this repo.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path

CBD_ROOT = Path(__file__).resolve().parents[1]
REPO_ROOT = CBD_ROOT.parent

def load_json(path: Path) -> dict:
    with path.open("r", encoding="utf-8") as f:
        return json.load(f)

def write_text(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")

def write_json(path: Path, obj: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(obj, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")

def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--id", required=True, help="Task id, e.g. 0001")
    ap.add_argument("--slug", required=True, help="Slug, e.g. data-orchestration")
    args = ap.parse_args()

    tid = str(args.id)
    slug = str(args.slug)

    task_path = CBD_ROOT / "tasks" / f"{tid}-{slug}.md"
    contract_path = CBD_ROOT / "contracts" / f"{tid}.contract.json"
    bundle_path = CBD_ROOT / "bundles" / f"{tid}.bundle.json"
    evidence_path = CBD_ROOT / "reports" / f"{tid}.evidence.md"

    # Templates
    task_template = (CBD_ROOT / "tasks" / "TEMPLATE.md").read_text(encoding="utf-8")
    contract_template = load_json(CBD_ROOT / "contracts" / "TEMPLATE.contract.json")
    bundle_template = load_json(CBD_ROOT / "bundles" / "TEMPLATE.bundle.json")
    evidence_template = (CBD_ROOT / "reports" / "TEMPLATE.evidence.md").read_text(encoding="utf-8")

    # Fill a few obvious fields
    task_text = task_template.replace("<ID>", tid)
    contract_template["id"] = tid
    bundle_template["id"] = tid
    bundle_template["artifact_paths"]["task"] = f".cbd/tasks/{tid}-{slug}.md"
    bundle_template["artifact_paths"]["contract"] = f".cbd/contracts/{tid}.contract.json"
    bundle_template["artifact_paths"]["evidence"] = f".cbd/reports/{tid}.evidence.md"

    write_text(task_path, task_text)
    write_json(contract_path, contract_template)
    write_json(bundle_path, bundle_template)
    write_text(evidence_path, evidence_template.replace("<ID>", tid))

    print("Created:")
    print(f"  {task_path.relative_to(REPO_ROOT)}")
    print(f"  {contract_path.relative_to(REPO_ROOT)}")
    print(f"  {bundle_path.relative_to(REPO_ROOT)}")
    print(f"  {evidence_path.relative_to(REPO_ROOT)}")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
