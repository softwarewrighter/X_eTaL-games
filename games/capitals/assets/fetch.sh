#!/usr/bin/env bash
# Fetch the capitals game's map data into assets/cache/ (git-ignored:
# third-party assets are never committed, docs/plan.md A9), then turn
# it into the data file assets/cache/world.toml with convert.py.
#
# Source: Natural Earth, 1:110m physical land and populated places, and
# 1:50m countries (only each country's UN region is used)
#   https://www.naturalearthdata.com/  (GeoJSON from
#   https://github.com/nvkelso/natural-earth-vector, pinned below)
# License: public domain (https://www.naturalearthdata.com/about/terms-of-use/)
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cache="$here/cache"
commit=ca96624a56bd078437bca8184e78163e5039ad19
base="https://raw.githubusercontent.com/nvkelso/natural-earth-vector/$commit/geojson"
mkdir -p "$cache"
get() {  # get FILE SHA256
  local f="$cache/$1"
  if [ ! -f "$f" ] || ! echo "$2  $f" | shasum -a 256 -c --status; then
    curl -sSfL -o "$f.part" "$base/$1"
    echo "$2  $f.part" | shasum -a 256 -c --status || { echo "fetch: $1 checksum mismatch" >&2; rm -f "$f.part"; exit 1; }
    mv "$f.part" "$f"
  fi
}
get ne_110m_land.geojson 9e0729ee253ca7d7a5c4ae9395fb1902264c5377c52e224d13dd85010e2835d9
get ne_110m_populated_places.geojson a86028b083182b68c7620fc6e1a8a47ee547cb9cd2fb62ccbb78bea786440899
get ne_50m_admin_0_countries.geojson 3e458fc036ad0a66411f2c1e6cac49c5d7bfb81cb1123bc513b22511a2b7fdeb
"$here/convert.py" "$cache/ne_110m_land.geojson" "$cache/ne_110m_populated_places.geojson" \
  "$cache/ne_50m_admin_0_countries.geojson" "$cache/world.toml"
