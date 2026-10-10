#!/usr/bin/env bash
# The cars are data: a copy of the game gets a fifth rival (a paint and a
# top speed in racer.toml, no code). It lines up on the grid, and twenty
# seconds later every rival has done a lap, by the same systems as the
# others.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$here/../.." && pwd)"
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT
cp "$here/Race.xtl" "$tmp/"
sed -e 's/^colors = \[\(.*\)\]/colors = [\1, "#2a9d8f"]/; s/^tops = \[\(.*\)\]/tops = [\1, "7.6"]/' "$here/racer.toml" > "$tmp/racer.toml"
cat > "$tmp/t.xtl" <<'XTL'
"rc:" u_se< "Race"
s := rc:n_ew 0
t_ally rc:c_ars s
u:w_ait := { s n -> n = 0 ? s; (s rc:t_ick 0.05) u:w_ait n - 1 }
s := s u:w_ait 440
t_ally w_here 1 <= 1 d_rop 1 s_elect_2 rc:c_ars s
XTL
got="$(cd "$tmp" && XETAL_PATH="$root/lib" "$root/bin/xetal" run --seed 1 t.xtl)"
want="5
4"
[ "$got" = "$want" ] || { echo "racer/test.sh: expected"; echo "$want"; echo "got"; echo "$got"; exit 1; }
