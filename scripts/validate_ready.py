#!/usr/bin/env python3
"""Fail fast if a contract is not ready.

Exit codes:
  0 = ready
  2 = file missing
  3 = invalid JSON
  4 = not ready / open questions present
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--id", required=True, help="Task id, e.g. 0001")
    args = ap.parse_args()
    tid = str(args.id)

    path = ROOT / "contracts" / f"{tid}.contract.json"
    if not path.exists():
        print(f"Missing contract: {path}")
        return 2

    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except Exception as e:
        print(f"Invalid JSON in {path}: {e}")
        return 3

    status = data.get("status")
    open_q = data.get("open_questions", [])
    if status != "ready" or (isinstance(open_q, list) and len(open_q) > 0):
        print(f"Contract {tid} not ready:")
        print(f"  status={status!r}")
        print(f"  open_questions={open_q}")
        return 4

    print(f"Contract {tid} is ready.")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
