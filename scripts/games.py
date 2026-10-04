#!/usr/bin/env python3
"""The games' descriptions (games/<slug>/game.toml) and their files.

  scripts/games.py list     # the slugs, in catalog order
  scripts/games.py check    # validate every game.toml (exit 1 on a problem)
  scripts/games.py json     # every description as JSON, in catalog order

Directories starting with "_" (the template) are not games.

`check` also enforces the program/library split (docs/plan.md A4):
a program is a lowercase <name>.xtl starting with a shebang and must
not define l: names (only libraries may: X_eTaL would quietly take
such a file for a library); a library is a capitalized <Name>.xtl,
without a shebang, exporting at least one l: name; and no library may
share the game's slug ignoring case (on a case-insensitive file system
Guess.xtl and guess.xtl are one file).
XETAL_GAMES_DIR overrides games/ (the runner's self-test uses it).
"""
import json
import os
import re
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
STATUSES = {"draft", "live", "deferred"}
FIELDS = {"slug": str, "title": str, "summary": str, "lesson": str, "concepts": list,
          "status": str, "order": int, "sources": list, "needs": list}


def game_dirs():
    base = Path(os.environ.get("XETAL_GAMES_DIR", ROOT / "games"))
    return sorted(d for d in base.iterdir()
                  if d.is_dir() and not d.name.startswith(("_", ".")))


def problems(d, meta):
    out = [f"{d.name}: missing or wrong type: {k}" for k, t in FIELDS.items()
           if not isinstance(meta.get(k), t)]
    if meta.get("slug") != d.name:
        out.append(f"{d.name}: slug {meta.get('slug')!r} is not the directory name")
    if meta.get("status") not in STATUSES:
        out.append(f"{d.name}: status must be one of {sorted(STATUSES)}")
    if not (isinstance(meta.get("wikipedia"), str) or isinstance(meta.get("about"), str)):
        out.append(f"{d.name}: needs wikipedia (a URL) or about (a short history and how to play)")
    if not (d / "README.md").is_file():
        out.append(f"{d.name}: no README.md")
    return out


DEFINES_L = re.compile(r"^\s*l:[A-Za-z]\w*\s*:=", re.M)


def xtl_problems(d):
    out = []
    for f in sorted(d.glob("*.xtl")):
        text = f.read_text()
        shebang = text.startswith("#!")
        if f.stem[:1].isupper():
            if shebang:
                out.append(f"{d.name}/{f.name}: a library (capitalized) must not start with a shebang")
            if not DEFINES_L.search(text):
                out.append(f"{d.name}/{f.name}: a library (capitalized) exports no l: names")
            if f.stem.lower() == d.name.lower():
                out.append(f"{d.name}/{f.name}: a library may not share the game's name ignoring case")
        else:
            if not shebang:
                out.append(f"{d.name}/{f.name}: a program (lowercase) must start with #!/usr/bin/env xetal")
            if DEFINES_L.search(text):
                out.append(f"{d.name}/{f.name}: a program defines l: names (only a library may)")
    return out


def load():
    games, errs = [], []
    for d in game_dirs():
        f = d / "game.toml"
        if not f.is_file():
            errs.append(f"{d.name}: no game.toml")
            continue
        try:
            meta = tomllib.loads(f.read_text())
        except tomllib.TOMLDecodeError as e:
            errs.append(f"{d.name}: game.toml: {e}")
            continue
        errs += problems(d, meta) + xtl_problems(d)
        meta["web"] = (d / "web" / "Cargo.toml").is_file()
        meta["picture"] = (d / "screenshot.png").is_file()
        meta["fetch"] = (d / "assets" / "fetch.sh").is_file()
        games.append(meta)
    games.sort(key=lambda m: (m.get("order", 0), m.get("slug", "")))
    return games, errs


def main(cmd):
    games, errs = load()
    if errs:
        print("\n".join(errs), file=sys.stderr)
        return 1
    if cmd == "list":
        print("\n".join(m["slug"] for m in games))
    elif cmd == "json":
        print(json.dumps(games, indent=2))
    elif cmd != "check":
        print(__doc__, file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1] if len(sys.argv) > 1 else "check"))
