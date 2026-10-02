#!/usr/bin/env bash
# Run a game's program with the vendored xetal, in the game's directory,
# pictures ([]S_HOW) to work/draw/SLUG/. FILE defaults to SLUG.xtl.
# Standard input is the terminal, so play.xtl is played interactively.
#   scripts/run-game.sh SLUG [FILE]
#   scripts/run-game.sh --echo SLUG [FILE]     # as a notebook
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
flags=()
if [ "${1:-}" = "--echo" ]; then flags+=(--echo); shift; fi
slug="${1:?usage: run-game.sh [--echo] SLUG [FILE]}"
d="$root/games/$slug"
[ -f "$d/game.toml" ] || { echo "run-game: no game $slug" >&2; exit 1; }
file="${2:-$slug.xtl}"
[ -f "$d/$file" ] || { echo "run-game: no $slug/$file" >&2; exit 1; }
if head -1 "$d/$file" | grep -q -- '--untyped'; then flags+=(--untyped); fi
if [ -x "$d/assets/fetch.sh" ]; then "$d/assets/fetch.sh" >/dev/null; fi
xetal="$("$root/scripts/build-xetal.sh")"
mkdir -p "$root/work/draw/$slug"
cd "$d" && exec "$xetal" run ${flags[@]+"${flags[@]}"} --draw "$root/work/draw/$slug" "$file"
