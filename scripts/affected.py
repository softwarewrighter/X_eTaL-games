#!/usr/bin/env python3
"""What a change touches, for scripts/gate.sh --affected: the games whose
pages must be built, tested in a browser and timed, from the files
changed since the merge base with origin/main (GATE_BASE names another),
committed or not, tracked or new.

  scripts/affected.py              # one line per game, or "all"
  scripts/affected.py --self-test

A file under games/<slug>/ touches that game; lib/<Name>.xtl every game
whose page carries it; shared/ (the page shell), the scripts that build,
test or time the pages, XETAL_COMMIT and Cargo locks touch all of them.
Docs, the saga record and the other scripts touch none (the sample
gate checks them).
"""
import os, re, subprocess, sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
ALL = ("shared/", "XETAL_COMMIT", "scripts/build-pages.sh", "scripts/build-catalog.py",
       "scripts/browser-test.mjs", "scripts/bench.py", "scripts/fetch-assets.sh",
       "scripts/serve-pages.sh", "images/", ".cargo/")


def games():
    out = subprocess.run([str(ROOT / "scripts/games.py"), "list"], capture_output=True, text=True, check=True)
    return [s for s in out.stdout.split() if (ROOT / "games" / s / "web/Cargo.toml").exists()]


def carries(slug, lib):
    """Whether game slug's page carries the shared library lib (Text, Svg, ...)."""
    rs = ROOT / "games" / slug / "web/src/lib.rs"
    return rs.exists() and f"lib/{lib}.xtl" in rs.read_text()


def touched(files, slugs, carries=carries):
    if any(f.startswith(ALL) or f.endswith("Cargo.lock") and not f.startswith("games/") for f in files):
        return ["all"]
    hit = set()
    for f in files:
        m = re.match(r"games/([^/]+)/", f)
        if m and m.group(1) in slugs:
            hit.add(m.group(1))
        m = re.match(r"lib/([A-Z]\w*)\.xtl$", f)
        if m:
            hit.update(s for s in slugs if carries(s, m.group(1)))
    return [s for s in slugs if s in hit]


def changed():
    base = os.environ.get("GATE_BASE") or subprocess.run(
        ["git", "merge-base", "HEAD", "origin/main"], capture_output=True, text=True, cwd=ROOT).stdout.strip() or "HEAD"
    git = lambda *a: subprocess.run(["git", *a], capture_output=True, text=True, cwd=ROOT).stdout.split("\n")
    files = set(git("diff", "--name-only", base)) | set(git("ls-files", "--others", "--exclude-standard"))
    return sorted(f for f in files if f)


def self_test():
    s = ["horse-race", "capitals", "stargazer"]
    libs = {("capitals", "Svg"), ("stargazer", "Svg"), ("capitals", "Text")}
    c = lambda slug, lib: (slug, lib) in libs
    cases = [
        (["games/capitals/Atlas.xtl"], ["capitals"]),
        (["lib/Svg.xtl"], ["capitals", "stargazer"]),
        (["lib/Text.xtl", "games/horse-race/play.xtl"], ["horse-race", "capitals"]),
        (["shared/microscope/src/page.rs"], ["all"]),
        (["XETAL_COMMIT"], ["all"]),
        (["docs/plan.md", "CHANGES.md", ".agentrail/steps/x/step.toml"], []),
        (["games/capitals/web/Cargo.lock"], ["capitals"]),
    ]
    bad = [(f, want, touched(f, s, c)) for f, want in cases if touched(f, s, c) != want]
    for f, want, got in bad:
        print(f"affected.py self-test: {f} gave {got}, wanted {want}")
    print("affected: self-test ok" if not bad else "affected: self-test FAILED")
    return 1 if bad else 0


if __name__ == "__main__":
    if "--self-test" in sys.argv:
        sys.exit(self_test())
    print("\n".join(touched(changed(), games())))
