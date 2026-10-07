#!/usr/bin/env bash
# Places of your own are asked like the capitals: a copy of the game
# with New York and Sydney added to places.toml, as its comment shows,
# has two more places, each in its region, each with its own name among
# its choices (the answer) beside nearby cities.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$here/../.." && pwd)"
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT
mkdir -p "$tmp/assets/cache"
cp "$here/Atlas.xtl" "$tmp/"
cp "$here/assets/cache/world.toml" "$tmp/assets/cache/"
sed 's/^places = \[\]$/places = ["new-york", "sydney"]/' "$here/places.toml" > "$tmp/places.toml"
cat >> "$tmp/places.toml" <<'TOML'

[place.new-york]
name = "New York, United States"
lat = "40.71"
lon = "-74.01"
region = "Americas"

[place.sydney]
name = "Sydney, Australia"
lat = "-33.87"
lon = "151.21"
region = "Oceania"
TOML
cat > "$tmp/t.xtl" <<'XTL'
"a:" u_se< "Atlas"
'{ r -> t_ally a:p_ool r } e_ach 0 2 5
n := t_ally a:p_ool 0
a:n_ame n - 1
a:t_own a:a_nswer n - 1
a:n_ame n
(a:a_nswer n) m_ember? a:c_hoices n
XTL
got="$(cd "$tmp" && XETAL_PATH="$root/lib" "$root/bin/xetal" run --seed 1 t.xtl)"
want="202 38 14
New York, United States
New York
Sydney, Australia
1"
[ "$got" = "$want" ] || { echo "capitals/test.sh: expected"; echo "$want"; echo "got"; echo "$got"; exit 1; }
