#!/usr/bin/env bash
# The map is data: a copy of the game in an empty room, 9 wide and 5
# high, the door in the middle of the east side (raycaster.toml, no
# code). Facing east from the west side, the middle ray meets the door
# 5.5 tiles away (in tenths, 55), and a walk east gets out (status 1).
set -euo pipefail
. "$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)/scripts/venv.sh"
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$here/../.." && pwd)"
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT
cp "$here/Rays.xtl" "$tmp/"
python3 - "$here/raycaster.toml" "$tmp/raycaster.toml" <<'PY'
import sys, re
t = open(sys.argv[1]).read()
room = ['"111111111"', '"1.......1"', '"1@.....E1"', '"1.......1"', '"111111111"']
t = re.sub(r"map = \[.*?\]", "map = [\n  " + ",\n  ".join(room) + ",\n]", t, flags=re.S)
open(sys.argv[2], "w").write(t)
PY
cat > "$tmp/t.xtl" <<'XTL'
"rc:" u_se< "Rays"
s := rc:n_ew 0
f_loor 0.5 + 10 * 3 s_elect 60 s_elect rc:s_ees s
u:w_alk := { s n -> n = 0 ? s; (s rc:k_ey "UP") u:w_alk n - 1 }
rc:s_tatus s u:w_alk 60
XTL
got="$(cd "$tmp" && XETAL_PATH="$root/lib" "$root/bin/xetal" run --seed 1 t.xtl)"
want="55
1"
[ "$got" = "$want" ] || { echo "raycaster/test.sh: expected"; echo "$want"; echo "got"; echo "$got"; exit 1; }
