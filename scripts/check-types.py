#!/usr/bin/env python3
"""Check the type comments of the X_eTaL libraries against their goldens.

A library (games/<slug>/<Name>.xtl, lib/<Name>.xtl) that has type
comments must have one, "# :: TYPE", in the comment block just above
every export (l:name := ...), and each must equal the type X_eTaL
infers, as the library's golden (expected/<Name>.out, "l:name : TYPE")
records. Libraries without any type comment are skipped (not yet
refactored; docs/style.md).

  scripts/check-types.py        # exit 1 on a problem
"""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def libraries():
    for f in sorted(ROOT.glob("games/*/*.xtl")) + sorted(ROOT.glob("lib/*.xtl")):
        if f.stem[:1].isupper() and not f.parent.name.startswith("_"):
            yield f


def comments(text):
    """{name: type or None} for every export, from the comment block above it."""
    out, block = {}, []
    for line in text.splitlines():
        if line.startswith("#"):
            block.append(line)
            continue
        m = re.match(r"(l:[A-Za-z_][A-Za-z0-9_]*[!?<]?)\s*:=", line)
        if m:
            types = [b[len("# ::"):].strip() for b in block if b.startswith("# ::")]
            out[m.group(1)] = types[0] if types else None
        block = []
    return out


def main():
    problems, checked = [], 0
    for f in libraries():
        found = comments(f.read_text())
        if not any(found.values()):
            continue
        golden = f.parent / "expected" / f"{f.stem}.out"
        inferred = dict(re.match(r"(\S+) : (.*)", l).groups()
                        for l in golden.read_text().splitlines() if " : " in l)
        rel = f.relative_to(ROOT)
        for name, typ in found.items():
            if typ is None:
                problems.append(f"{rel}: {name} has no type comment (# :: {inferred.get(name, '?')})")
            elif inferred.get(name) != typ:
                problems.append(f"{rel}: {name}: comment says {typ}, X_eTaL infers {inferred.get(name)}")
        checked += 1
    print("\n".join(problems) or f"check-types: ok ({checked} libraries with type comments)")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
