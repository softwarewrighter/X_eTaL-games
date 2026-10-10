#!/usr/bin/env bash
# The entity-component system's promise, checked: a new kind of monster
# is a table in dungeon.toml, not code. A copy of the game gets bats (a
# table and two b's on the map): they spawn with hit points, attack,
# chase and gold from their table, and come after you like the others.
# And every key from a new game keeps it playable.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$here/../.." && pwd)"
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT
cp "$here/Crawl.xtl" "$tmp/"
python3 - "$here/dungeon.toml" "$tmp/dungeon.toml" <<'PY'
import sys
t = open(sys.argv[1]).read()
# two bats on every map: the first two floor cells of its second row
rows = t[t.index("maps = ["):t.index("]", t.index("maps = ["))].split("\n")[1:-1]
for i in range(1, len(rows), 9):
    r = rows[i].strip().strip(",").strip('"')
    b = r.replace("..", "bb", 1)
    t = t.replace('"' + r + '"', '"' + b + '"', 1)
t = t.replace('kinds = ["rat", "goblin", "potion", "gold"]', 'kinds = ["rat", "goblin", "potion", "gold", "bat"]')
t += '\n[kind.bat]\nglyph = "b"\nhp = "2"\natk = "1"\nchase = "1"\nheal = "0"\ngold = "2"\ncolor = "#9b5de5"\n'
open(sys.argv[2], "w").write(t)
PY
cat > "$tmp/t.xtl" <<'XTL'
"cr:" u_se< "Crawl"
"ec:" u_se< "Ecs"
s := cr:n_ew 0
u:w_orld := { (W, g, n, m, lv) -> W }
ec:c_ount u:w_orld s
t_ally w_here '& r_/_2 (cr:h_as s) = ((t_ally cr:h_as s) c_at 5) r_eshape 1 1 1 0 1
t := s cr:t_urn 1 0.0
t := t cr:t_urn 0 1.0
t := t cr:t_urn -1 0.0
t := t cr:t_urn 0 -1.0
t := t cr:t_urn 0 0.0
cr:s_tatus t
XTL
got="$(cd "$tmp" && XETAL_PATH="$root/lib" "$root/bin/xetal" run --seed 1 t.xtl)"
want="12
6
0"
[ "$got" = "$want" ] || { echo "dungeon/test.sh: expected"; echo "$want"; echo "got"; echo "$got"; exit 1; }
