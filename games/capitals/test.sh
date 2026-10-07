#!/usr/bin/env bash
# Places of your own are asked like the capitals: a copy of the game
# with New York and Sydney added to places.toml, as its comment shows,
# has 201 places, each in its region, with the nearest capitals as its
# wrong choices.
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
a:c_ount @
'{ r -> t_ally a:p_ool r } e_ach 0 2 5
a:n_ame 200
a:n_ame f_irst 200 a:n_earest a:p_ool 2
a:n_ame f_irst 201 a:n_earest a:p_ool 5
XTL
got="$(cd "$tmp" && "$root/bin/xetal" run --seed 1 t.xtl)"
want="201
201 37 14
New York, United States
Washington, D.C., United States of America
Canberra, Australia"
[ "$got" = "$want" ] || { echo "capitals/test.sh: expected"; echo "$want"; echo "got"; echo "$got"; exit 1; }
