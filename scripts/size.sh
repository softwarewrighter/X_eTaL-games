#!/usr/bin/env bash
# How big each game's X_eTaL is: lines (not blank, not only a comment)
# and tokens (X_eTaL's own lexer, newlines left out) per game, its rules
# library, its scripted game and its terminal game; and lib/ when it
# exists. A markdown table, for the plan and the READMEs.
#   scripts/size.sh [SLUG...]
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
xetal="$("$root/scripts/build-xetal.sh")"
if [ $# -gt 0 ]; then slugs=("$@"); else
  slugs=(); while IFS= read -r s; do [ -n "$s" ] && slugs+=("$s"); done < <("$root/scripts/games.py" list)
fi
lines() { grep -v '^[[:space:]]*#' "$1" | grep -cv '^[[:space:]]*$' || true; }
tokens() { "$xetal" lex "$1" 2>/dev/null | grep -cv ' Newline$' || true; }
# size FILE...: "lines tokens" summed over the files
size() { local l=0 t=0 f; for f in "$@"; do [ -f "$f" ] || continue; l=$((l + $(lines "$f"))); t=$((t + $(tokens "$f"))); done; echo "$l $t"; }
echo "| Game | Rules (lines / tokens) | Scripted | Terminal | Total |"
echo "| ---- | ---------------------- | -------- | -------- | ----- |"
tl=0; tt=0
for slug in "${slugs[@]}"; do
  d="$root/games/$slug"
  lib=(); for f in "$d"/*.xtl; do b="$(basename "$f" .xtl)"; [[ "$b" =~ ^[A-Z] ]] && lib+=("$f"); done
  read -r rl rt < <(size ${lib[@]+"${lib[@]}"})
  read -r sl st < <(size "$d/$slug.xtl")
  read -r pl pt < <(size "$d/play.xtl")
  read -r al at < <(size ${lib[@]+"${lib[@]}"} "$d/$slug.xtl" "$d/play.xtl")
  tl=$((tl + al)); tt=$((tt + at))
  echo "| $slug | $rl / $rt | $sl / $st | $pl / $pt | $al / $at |"
done
if [ -d "$root/lib" ] && ls "$root"/lib/*.xtl >/dev/null 2>&1; then
  read -r ll lt < <(size "$root"/lib/*.xtl)
  echo "| (lib/, shared) | $ll / $lt | | | $ll / $lt |"
  tl=$((tl + ll)); tt=$((tt + lt))
fi
echo "| all | | | | $tl / $tt |"
