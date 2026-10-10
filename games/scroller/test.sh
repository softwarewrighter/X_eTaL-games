#!/usr/bin/env bash
# The systems act on every body, whatever the level: a copy of the game
# gets a walker dropped from the sky (a w high above the ground). Gravity
# brings it down to stand on the ground like the others (its y, in
# hundredths of a tile), and the walkers keep walking.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$here/../.." && pwd)"
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT
cp "$here/Jump.xtl" "$tmp/"
python3 - "$here/scroller.toml" "$tmp/scroller.toml" <<'PY'
import sys
t = open(sys.argv[1]).read()
rows = t[t.index("level = ["):t.index("]", t.index("level = ["))].split("\n")[1:-1]
r = rows[1].strip().strip(",").strip('"')
t = t.replace('"' + r + '"', '"' + r[:10] + "w" + r[11:] + '"', 1)
open(sys.argv[2], "w").write(t)
PY
cat > "$tmp/t.xtl" <<'XTL'
"j:" u_se< "Jump"
s := j:n_ew 0
2 s_elect j:c_ounts s
u:w_ait := { s n -> n = 0 ? s; (s j:t_ick 0.05) u:w_ait n - 1 }
s := s u:w_ait 40
f_loor 0.5 + 100 * 2 s_elect_2 j:b_odies s
j:s_tatus s
XTL
got="$(cd "$tmp" && XETAL_PATH="$root/lib" "$root/bin/xetal" run --seed 1 t.xtl)"
want="4
720 720 720 720 720
0"
[ "$got" = "$want" ] || { echo "scroller/test.sh: expected"; echo "$want"; echo "got"; echo "$got"; exit 1; }
