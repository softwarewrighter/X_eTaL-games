#!/usr/bin/env bash
# The cross-reference of every game's programs and libraries and of the
# shared libraries (lib/), written by X_eTaL's own `xetal doc --out` into
# pages/doc/ (part of the live site: scripts/build-pages.sh calls it).
# Each definition with its type, its ## documentation and ## >> examples,
# its source drawn decorated, and every name linked to its definition
# and its uses (X_eTaL's lang-choices S9).
#   scripts/build-docs.sh
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
xetal="$(scripts/build-xetal.sh)"
files=(); for f in lib/*.xtl; do [ "$(basename "$f")" = test.xtl ] || files+=("$f"); done
while IFS= read -r slug; do
  [ -n "$slug" ] && files+=(games/"$slug"/*.xtl)
done < <(scripts/games.py list)
rm -rf pages/doc
XETAL_PATH="$root/lib" "$xetal" doc --out pages/doc "${files[@]}" >/dev/null
echo "pages/doc: the cross-reference of ${#files[@]} files"
