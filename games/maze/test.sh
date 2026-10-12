#!/usr/bin/env bash
# The maze is data: a copy of the game 12 cells wide and 5 high
# (maze.toml, no code). The maze carved has the blocks for that size,
# every open block can be reached from the start (the flood finds a
# distance for each), and the way out is the farthest of them.
set -euo pipefail
. "$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)/scripts/venv.sh"
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$here/../.." && pwd)"
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT
cp "$here/Walls.xtl" "$tmp/"
sed -e 's/^size = .*/size = ["12", "5"]/' "$here/maze.toml" > "$tmp/maze.toml"
cat > "$tmp/t.xtl" <<'XTL'
"mz:" u_se< "Walls"
s := mz:n_ew 0
s_hape mz:g_rid s
D := r_avel mz:d_istances s
'& r_/ (0 <= D) = 0 = r_avel mz:g_rid s
y := mz:y_ou s
((4 s_elect y) s_elect D) = 'm_ax r_/ D
XTL
got="$(cd "$tmp" && XETAL_PATH="$root/lib" "$root/bin/xetal" run --seed 1 t.xtl)"
want="11 25
1
1"
[ "$got" = "$want" ] || { echo "maze/test.sh: expected"; echo "$want"; echo "got"; echo "$got"; exit 1; }
