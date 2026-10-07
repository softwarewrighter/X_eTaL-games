#!/usr/bin/env bash
# Fetch the capitals game's map data into assets/cache/ (git-ignored:
# third-party assets are never committed, docs/plan.md A9), then turn
# it into the data file assets/cache/world.toml with convert.py.
#
# Source: Natural Earth, 1:50m countries (outlines and UN regions) and
# 1:50m populated places (the capitals, and the cities for the choices)
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
get ne_50m_admin_0_countries.geojson 3e458fc036ad0a66411f2c1e6cac49c5d7bfb81cb1123bc513b22511a2b7fdeb
get ne_50m_populated_places.geojson da4662b7bbfeb897d02f228c5839131dce27acff5717630f91ccff4f67828ee7
"$here/convert.py" "$cache/ne_50m_admin_0_countries.geojson" \
  "$cache/ne_50m_populated_places.geojson" "$cache/world.toml"
