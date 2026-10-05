#!/usr/bin/env bash
# Check X_eTaL at the known-good commit: the CLI builds, answers and
# reports the commit in XETAL_COMMIT, and a workspace of this repository
# can use xetal-play from the clone natively and for wasm32.
#   scripts/check-vendor.sh
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
xetal="$("$root/scripts/build-xetal.sh")"
sha="$(cut -c1-7 "$root/XETAL_COMMIT")"
got="$("$xetal" eval -e "'+ r_/_2 2 3 r_eshape r_ange 6")"
[ "$got" = "6 15" ] || { echo "check-vendor: eval gave '$got', expected '6 15'" >&2; exit 1; }
"$xetal" --version | grep -q "$sha" || { echo "check-vendor: xetal --version does not name $sha" >&2; "$xetal" --version >&2; exit 1; }
"$xetal" run "$root/work/xetal/demos/life.xtl" >/dev/null
cd "$root/tools/vendor-probe"
cargo test -q >/dev/null 2>&1 || { cargo test; exit 1; }
cargo check -q --target wasm32-unknown-unknown
echo "check-vendor: ok ($sha)"
