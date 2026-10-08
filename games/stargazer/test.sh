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

# Every state a player can reach by clicking stays playable: from every
# view, every button starts the right round; in every round, every
# marker's menu opens and every choice answers it (a part of the sky
# with no bright star once stopped the game: the bug the user met).
cat > "$tmp/every.xtl" <<'XTL'
"k:" u_se< "Sky"
"sv:" u_se< "Svg"
# The middle of rectangle r.
u:m_id := { r -> ((f_irst r) + 0.5 * 3 s_elect r) c_at (2 s_elect r) + 0.5 * 4 s_elect r }
nb := 1 + t_ally k:p_arts @
# From part a's view, button j starts a round in part j - 1.
u:b_utton := { a j ->
  s := k:n_ew a
  (j - 1) = f_irst s k:c_lick u:m_id j s_elect (k:v_iew a) sv:b_uttons nb
}
# In part a's round, star i's menu opens, and choice m answers it.
u:o_pened := { s i -> s k:c_lick (f_irst s) k:a_t i s_elect k:m_arks s }
u:a_nswered := { s im ->
  t := s u:o_pened f_irst im
  t k:c_lick u:m_id (2 s_elect im) s_elect k:m_enu t
}
u:r_ound := { a ->
  s := k:n_ew a
  n := 4 s_elect s
  opened := '+ r_/ '{ i -> i = 2 s_elect s u:o_pened i } e_ach r_ange n
  answered := '+ r_/ r_avel (r_ange n) '{ i m -> 0 < (5 + n + i) s_elect s u:a_nswered i c_at m } t_able 1 2 3 4
  opened c_at answered
}
# Every button from every view (nb in each), and every star's menu and
# every answer in every part's round (5 and 20 in each).
'{ a -> '+ r_/ '{ j -> a u:b_utton j } e_ach r_ange nb } e_ach (r_ange nb) - 1
'{ a -> e_nclose u:r_ound a } e_ach (r_ange nb) - 1
XTL
got="$(cd "$tmp" && cp "$here/places.toml" "$here/stars.toml" . 2>/dev/null; XETAL_PATH="$root/lib" "$root/bin/xetal" run --seed 1 every.xtl 2>&1 | grep -v '[┌└]' | tr -d '│' | tr -s ' ' | sed 's/^ //; s/ $//')"
want="7 7 7 7 7 7 7
5 20 5 20 5 20 5 20 5 20 5 20 5 20"
[ "$got" = "$want" ] || { echo "stargazer/test.sh: every click: expected"; echo "$want"; echo "got"; echo "$got"; exit 1; }
