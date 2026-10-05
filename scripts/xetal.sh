#!/usr/bin/env bash
# Get xetal (X_eTaL's docs/vendoring.md): clone X_eTaL into work/xetal
# (gitignored), check out the known-good commit in XETAL_COMMIT, build
# the release binary, and symlink bin/xetal to it. Safe to run again:
# with nothing to do it only confirms the build. Progress goes to stderr.
#   scripts/xetal.sh
#   XETAL_SOURCE=../X_eTaL scripts/xetal.sh   # clone from a local checkout
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
commit="$(tr -d '[:space:]' < "$root/XETAL_COMMIT")"
source="${XETAL_SOURCE:-https://github.com/softwarewrighter/X_eTaL.git}"
clone="$root/work/xetal"

if [ ! -d "$clone/.git" ]; then
    mkdir -p "$root/work"
    git clone --quiet "$source" "$clone" >&2
fi
if ! git -C "$clone" cat-file -e "$commit^{commit}" 2>/dev/null; then
    git -C "$clone" fetch --quiet origin >&2
fi
if [ "$(git -C "$clone" rev-parse HEAD)" != "$commit" ]; then
    git -C "$clone" checkout --quiet --detach "$commit" >&2
fi

(cd "$clone/components/cli" && cargo build --quiet --release -p xetal-cli >&2)

mkdir -p "$root/bin"
ln -sfn "../work/xetal/target/release/xetal" "$root/bin/xetal"
"$root/bin/xetal" --version | grep Commit >&2
