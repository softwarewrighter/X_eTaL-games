#!/usr/bin/env python3
"""Every library documents itself for `xetal doc` (X_eTaL's lang-choices
S9): a ## block at the top for the file, and a ## block directly above
every export (l:name := ...), directly (a # :: type line goes above the
block). Programs
(<slug>.xtl, play.xtl) need a ## block at the top. The site the docs
make is pages/doc/ (scripts/build-docs.sh).

  scripts/check-docs.py        # exit 1 on a problem
"""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def files():
    for f in sorted(ROOT.glob("lib/*.xtl")) + sorted(ROOT.glob("games/*/*.xtl")):
        if f.parent.name.startswith("_") or f.name == "test.xtl":
            continue
        yield f


def problems(f):
    lines = f.read_text().splitlines()
    i = 1 if lines and lines[0].startswith("#!") else 0
    out = []
    if i >= len(lines) or not lines[i].startswith("##") or lines[i].startswith("###"):
        out.append(f"{f.relative_to(ROOT)}: no ## block at the top (the file's documentation)")
    for n, line in enumerate(lines):
        m = re.match(r"(l:\w+) :=", line)
        if not m:
            continue
        k = n - 1
        documented = False
        while k >= 0 and lines[k].startswith("##") and not lines[k].startswith("###"):
            documented = True
            k -= 1
        while k >= 0 and lines[k].startswith("# ::"):
            k -= 1
        if not documented:
            out.append(f"{f.relative_to(ROOT)}:{n + 1}: {m.group(1)} has no ## block above it")
    return out


def main():
    bad = [p for f in files() for p in problems(f)]
    for p in bad:
        print(f"check-docs: {p}")
    n = sum(1 for _ in files())
    print(f"check-docs: {'ok' if not bad else 'FAILED'} ({n} files)")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
