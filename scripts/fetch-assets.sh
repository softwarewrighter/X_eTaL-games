#!/usr/bin/env bash
# Fetch the third-party assets of every game that has
# games/<slug>/assets/fetch.sh (or of the named games) into its
# git-ignored games/<slug>/assets/cache/. Assets are never committed
# (docs/plan.md A9): only the scripts that fetch them are.
#   scripts/fetch-assets.sh [SLUG...]
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if [ $# -gt 0 ]; then slugs=("$@"); else
  slugs=(); while IFS= read -r s; do [ -n "$s" ] && slugs+=("$s"); done < <("$root/scripts/games.py" list)
fi
for slug in ${slugs[@]+"${slugs[@]}"}; do
  f="$root/games/$slug/assets/fetch.sh"
  [ -x "$f" ] || continue
  echo "==> $slug"; "$f"
done
