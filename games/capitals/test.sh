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

# Every state a player can reach by clicking stays playable: from every
# view, every button starts the right round; in every round, every
# marker's menu opens and every choice answers it (a part of the sky
# with no bright star once stopped the game: the bug the user met).
cat > "$tmp/every.xtl" <<'XTL'
"a:" u_se< "Atlas"
"sv:" u_se< "Svg"
# The middle of rectangle r.
u:m_id := { r -> ((f_irst r) + 0.5 * 3 s_elect r) c_at (2 s_elect r) + 0.5 * 4 s_elect r }
nb := 1 + t_ally a:r_egions @
# From region a's view, button j starts a round in region j - 1.
u:b_utton := { a j ->
  s := a:n_ew a
  (j - 1) = f_irst s a:c_lick u:m_id j s_elect (a:v_iew a) sv:b_uttons nb
}
# In region a's round, dot i's menu opens, and choice m answers it.
u:o_pened := { s i -> s a:c_lick a:a_t i s_elect a:d_ots s }
u:a_nswered := { s im ->
  t := s u:o_pened f_irst im
  t a:c_lick u:m_id (2 s_elect im) s_elect a:m_enu t
}
u:r_ound := { a ->
  s := a:n_ew a
  n := 4 s_elect s
  opened := '+ r_/ '{ i -> i = 2 s_elect s u:o_pened i } e_ach r_ange n
  answered := '+ r_/ r_avel (r_ange n) '{ i m -> 0 < (4 + n + i) s_elect s u:a_nswered i c_at m } t_able 1 2 3 4
  opened c_at answered
}
# Every button from every view (nb in each), and every dot's menu and
# every answer in every region's round (5 and 20 in each).
'{ a -> '+ r_/ '{ j -> a u:b_utton j } e_ach r_ange nb } e_ach (r_ange nb) - 1
'{ a -> e_nclose u:r_ound a } e_ach (r_ange nb) - 1
XTL
got="$(cd "$tmp" && cp "$here/places.toml" "$here/stars.toml" . 2>/dev/null; XETAL_PATH="$root/lib" "$root/bin/xetal" run --seed 1 every.xtl 2>&1 | grep -v '[┌└]' | tr -d '│' | tr -s ' ' | sed 's/^ //; s/ $//')"
want="6 6 6 6 6 6
5 20 5 20 5 20 5 20 5 20 5 20"
[ "$got" = "$want" ] || { echo "capitals/test.sh: every click: expected"; echo "$want"; echo "got"; echo "$got"; exit 1; }
