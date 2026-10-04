#!/usr/bin/env bash
# The pre-commit gate: the vendored X_eTaL (scripts/check-vendor.sh),
# the game tooling (scripts/selftest-games.sh), the shared page shell,
# every game's tests, every game played in headless Chrome,
# then ASCII-only markdown for the docs we own.
#   scripts/gate.sh
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
"$root/scripts/check-vendor.sh"
"$root/scripts/selftest-games.sh"
# The shared shell of the game pages (shared/microscope).
(cd "$root/shared/microscope" && cargo test -q >/dev/null 2>&1 && cargo check -q --target wasm32-unknown-unknown) \
  || { (cd "$root/shared/microscope" && cargo test -q); echo "FAIL: shared/microscope"; exit 1; }
echo "ok: shared/microscope"
"$root/scripts/test-lib.sh"
"$root/scripts/check-types.py"
"$root/scripts/test-games.sh"
# The built pages played in a real browser (run just pages after a change),
# and every game timed against bench/baseline.json (scripts/bench.py runs
# the browser test itself).
"$root/scripts/bench.py"
md=(README.md CHANGES.md lib/README.md docs/bench.md docs/plan.md docs/style.md docs/xetal-asks.md docs/xetal-terminal-request.md)
for f in games/*/README.md shared/*/README.md; do [ -e "$f" ] && md+=("$f"); done
for f in "${md[@]}"; do sw-markdown-checker -f "$f" >/dev/null || { sw-markdown-checker -f "$f"; exit 1; }; done
echo "gate: ok"
