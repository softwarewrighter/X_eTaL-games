#!/usr/bin/env bash
# Build the live site into pages/, which is committed: the Pages
# workflow publishes that folder as it is (nothing is built on GitHub).
#   - every game with a web app (games/<slug>/web/) is built with trunk
#     into pages/<slug>/, served under /X_eTaL-games/<slug>/;
#   - pages/index.html, the catalog, from every game.toml.
# A game removed from games/ loses its pages/<slug>/.
#   scripts/build-pages.sh
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
base="/X_eTaL-games"
"$root/scripts/games.py" check
# Third-party assets are fetched (never tracked), so trunk can copy them.
"$root/scripts/fetch-assets.sh"
mkdir -p "$root/pages"
touch "$root/pages/.nojekyll"
keep=()
while IFS= read -r slug; do
  [ -n "$slug" ] || continue
  web="$root/games/$slug/web"
  [ -f "$web/Cargo.toml" ] || continue
  keep+=("$slug")
  dist="$root/target/pages-dist/$slug"
  echo "==> trunk build $slug"
  # The app's own index.html is the trunk entry: games/<slug>/web/index.html.
  (cd "$web" && trunk build --release --public-url "$base/$slug/" --dist "$dist")
  rsync -a --delete "$dist/" "$root/pages/$slug/"
  # The catalog card's picture, when the game has one (just screenshots).
  if [ -f "$root/games/$slug/screenshot.png" ]; then cp "$root/games/$slug/screenshot.png" "$root/pages/$slug/"; fi
done < <("$root/scripts/games.py" list)
# Drop pages/<slug>/ of games that no longer have a web app.
for d in "$root"/pages/*/; do
  [ -d "$d" ] || continue
  s="$(basename "$d")"
  printf '%s\n' ${keep[@]+"${keep[@]}"} | grep -qx "$s" || { echo "removing pages/$s/"; rm -rf "$d"; }
done
cp "$root/images/modern-xetal-logo.jpg" "$root/images/favicon.ico" "$root/pages/"
"$root/scripts/build-catalog.py"
echo "pages/ built; commit it (git add pages/) and push to publish."
