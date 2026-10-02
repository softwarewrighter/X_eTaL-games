#!/usr/bin/env bash
# Start a game sub-project from games/_template: games/SLUG/ with its
# game.toml, README.md, SLUG.xtl and expected/SLUG.out.
#   scripts/new-game.sh SLUG "Title"
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
base="${XETAL_GAMES_DIR:-$root/games}"
slug="${1:?usage: new-game.sh SLUG \"Title\"}"
title="${2:?usage: new-game.sh SLUG \"Title\"}"
[[ "$slug" =~ ^[a-z0-9][a-z0-9-]*$ ]] || { echo "new-game: slug must be lowercase letters, digits and -" >&2; exit 1; }
dest="$base/$slug"
[ ! -e "$dest" ] || { echo "new-game: $dest exists" >&2; exit 1; }
cp -R "$root/games/_template" "$dest"
mv "$dest/__SLUG__.xtl" "$dest/$slug.xtl"
mv "$dest/expected/__SLUG__.out" "$dest/expected/$slug.out"
chmod +x "$dest/$slug.xtl"
esc="$(printf '%s' "$title" | sed 's/[&|\\]/\\&/g')"
for f in "$dest/game.toml" "$dest/README.md" "$dest/$slug.xtl"; do
  sed -i '' -e "s|__SLUG__|$slug|g" -e "s|__TITLE__|$esc|g" "$f" 2>/dev/null \
    || sed -i -e "s|__SLUG__|$slug|g" -e "s|__TITLE__|$esc|g" "$f"
done
echo "new game: $dest (edit game.toml, README.md and $slug.xtl; just test-game $slug)"
