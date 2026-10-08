#!/usr/bin/env bash
# Every state a click can reach stays playable, and the solver solves:
# from a new puzzle, each of the 25 squares pressed, and both buttons;
# fifty random puzzles each turned off by its solution.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$here/../.." && pwd)"
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT
cp "$here/Lamps.xtl" "$tmp/"
cat > "$tmp/t.xtl" <<'XTL'
"lo:" u_se< "Lamps"
"sv:" u_se< "Svg"
s := lo:n_ew 0
q := lo:s_quares @
u:m_id := { r -> ((f_irst r) + 0.5 * 3 s_elect r) c_at (2 s_elect r) + 0.5 * 4 s_elect r }
'+ r_/ '{ k -> 1 = lo:m_oves s lo:c_lick u:m_id k s_elect q } e_ach r_ange 25
0 = lo:m_oves s lo:c_lick 175 67.0
0 < 2 s_elect s lo:c_lick 500 67.0
u:o_k := { i ->
  b := lo:l_ights lo:n_ew 0
  0 = '+ r_/ b lo:a_fter lo:s_olve b
}
'+ r_/ 'u:o_k e_ach r_ange 50
XTL
got="$(cd "$tmp" && XETAL_PATH="$root/lib" "$root/bin/xetal" run --seed 1 t.xtl)"
want="25
1
1
50"
[ "$got" = "$want" ] || { echo "lights-out/test.sh: expected"; echo "$want"; echo "got"; echo "$got"; exit 1; }
