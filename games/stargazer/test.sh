#!/usr/bin/env bash
# Stars of your own are asked like the named stars: a copy of the game
# with one star added to stars.toml, as its comment shows, has one more
# star, in its part of the sky and its brightness band, with its own
# name among its choices.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$here/../.." && pwd)"
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT
mkdir -p "$tmp/assets/cache"
cp "$here/Sky.xtl" "$tmp/"
cp "$here/assets/cache/sky.toml" "$tmp/assets/cache/"
sed 's/^stars = \[\]$/stars = ["my-star"]/' "$here/stars.toml" > "$tmp/stars.toml"
cat >> "$tmp/stars.toml" <<'TOML'

[star.my-star]
name = "My Star"
ra = "83.82"
dec = "-5.39"
mag = "4.0"
con = "Ori"
TOML
cat > "$tmp/t.xtl" <<'XTL'
"k:" u_se< "Sky"
n := t_ally k:p_ool 0
n
k:n_ame n
n m_ember? k:p_ool 1
n m_ember? 7 k:c_hoices n
XTL
got="$(cd "$tmp" && XETAL_PATH="$root/lib" "$root/bin/xetal" run --seed 1 t.xtl)"
want="308
My Star
1
1"
[ "$got" = "$want" ] || { echo "stargazer/test.sh: expected"; echo "$want"; echo "got"; echo "$got"; exit 1; }
