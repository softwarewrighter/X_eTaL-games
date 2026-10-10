#!/usr/bin/env bash
# The churn, checked: a copy of the game with one wave of one big rock
# (asteroids.toml, no code). A gunner that turns to the nearest rock and
# fires breaks it into two medium rocks, those into four small ones, and
# those to nothing: every wave cleared, for 20 + 2 x 50 + 4 x 100 = 520
# points, with every rock spawned and despawned in the world's slots.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$here/../.." && pwd)"
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT
cp "$here/Rocks.xtl" "$tmp/"
sed -e 's/^waves = .*/waves = ["1"]/' "$here/asteroids.toml" > "$tmp/asteroids.toml"
cat > "$tmp/t.xtl" <<'XTL'
"ro:" u_se< "Rocks"
u:k_ey := { s ->
  c := ro:s_hip s
  R := ro:r_ocks s
  0 = t_ally R ? " "
  dx := (1 s_elect_2 R) - f_irst c
  dy := (2 s_elect_2 R) - 2 s_elect c
  i := f_irst g_rade (dx * dx) + dy * dy
  d := 3 s_elect c
  side := (((c_os d) * i s_elect dy) - (s_in d) * i s_elect dx) / (0.001 m_ax ((i s_elect dx) * i s_elect dx) + (i s_elect dy) * i s_elect dy) ^ 0.5
  side > 0.12 ? "RIGHT"
  side < -0.12 ? "LEFT"
  " "
}
u:g_o := { s n -> (n > 1200) | 0 < ro:s_tatus s ? s; ((s ro:k_ey u:k_ey s) ro:t_ick 0.05) u:g_o n + 1 }
s := (ro:n_ew 0) u:g_o 1
ro:s_tatus s
f_irst ro:s_core s
XTL
got="$(cd "$tmp" && XETAL_PATH="$root/lib" "$root/bin/xetal" run --seed 1 t.xtl)"
want="1
520"
[ "$got" = "$want" ] || { echo "asteroids/test.sh: expected"; echo "$want"; echo "got"; echo "$got"; exit 1; }
