#!/usr/bin/env python3
"""The games' descriptions (games/<slug>/game.toml).

  scripts/games.py list     # the slugs, in catalog order
  scripts/games.py check    # validate every game.toml (exit 1 on a problem)
  scripts/games.py json     # every description as JSON, in catalog order

Directories starting with "_" (the template) are not games.
XETAL_GAMES_DIR overrides games/ (the runner's self-test uses it).
"""
import json
import os
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
    if not (d / "README.md").is_file():
        out.append(f"{d.name}: no README.md")
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
        errs += problems(d, meta)
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
