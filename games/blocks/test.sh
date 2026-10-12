#!/usr/bin/env bash
# The pieces are data: a copy of the game gets an eighth piece, a plus
# (its rows, its name and a color in blocks.toml, no code). It comes,
# turns and settles like the others: after forty pieces placed by the
# computer player, blocks of the eighth kind are in the well and the
# game goes on.
set -euo pipefail
. "$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)/scripts/venv.sh"
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$here/../.." && pwd)"
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT
cp "$here/Stack.xtl" "$tmp/"
sed -e 's/^pieces = \[\(.*\)\]/pieces = [\1, "P"]/; s/^colors = \[\(.*\)\]/colors = [\1, "#ffffff"]/' "$here/blocks.toml" > "$tmp/blocks.toml"
printf 'P = [".#.", "###", ".#."]\n' >> "$tmp/blocks.toml"
cat > "$tmp/t.xtl" <<'XTL'
"st:" u_se< "Stack"
u:a_uto := { s n -> (n = 0) | 0 < st:s_tatus s ? s; (s st:k_ey "h") u:a_uto n - 1 }
s := (st:n_ew 0) u:a_uto 40
0 < '+ r_/ r_avel 8 = st:w_ell s
st:s_tatus s
XTL
got="$(cd "$tmp" && XETAL_PATH="$root/lib" "$root/bin/xetal" run --seed 1 t.xtl)"
want="1
0"
[ "$got" = "$want" ] || { echo "blocks/test.sh: expected"; echo "$want"; echo "got"; echo "$got"; exit 1; }
