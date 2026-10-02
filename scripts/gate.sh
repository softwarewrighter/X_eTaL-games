#!/usr/bin/env bash
# The pre-commit gate: the vendored X_eTaL (scripts/check-vendor.sh),
# then ASCII-only markdown for the docs we own. Later steps add the
# game tooling's self-test and every game's tests.
#   scripts/gate.sh
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
"$root/scripts/check-vendor.sh"
md=(README.md CHANGES.md docs/plan.md docs/xetal-asks.md)
for f in games/*/README.md shared/*/README.md; do [ -e "$f" ] && md+=("$f"); done
for f in "${md[@]}"; do sw-markdown-checker -f "$f" >/dev/null || { sw-markdown-checker -f "$f"; exit 1; }; done
echo "gate: ok"
