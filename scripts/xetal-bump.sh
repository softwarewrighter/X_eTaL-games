#!/usr/bin/env bash
# Move to another X_eTaL: resolve REF (default origin/main) in the clone
# (work/xetal, fetched first), write its full SHA to XETAL_COMMIT, and
# build it (scripts/xetal.sh). Then run the gate, and commit
# XETAL_COMMIT with whatever the new version changed.
#   scripts/xetal-bump.sh [REF]
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ref="${1:-origin/main}"
"$root/scripts/xetal.sh" >/dev/null 2>&1 || true
git -C "$root/work/xetal" fetch --quiet origin
sha="$(git -C "$root/work/xetal" rev-parse --verify "$ref^{commit}")"
echo "$sha" > "$root/XETAL_COMMIT"
echo "XETAL_COMMIT: $sha ($(git -C "$root/work/xetal" log -1 --format=%s "$sha"))"
"$root/scripts/xetal.sh"
