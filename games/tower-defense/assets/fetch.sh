#!/usr/bin/env bash
# Fetch tower defense's pieces into assets/cache/art/ (git-ignored:
# outside files are never committed here, docs/plan.md A9). The pages
# build copies them next to the game's page, where the game's X_eTaL
# draws them (td.toml: art = "1").
#
# Source: X_eTaL-extensions, made by X_eTaL driving Blender
#   (extensions/blender/demos/game-assets.xtl, at f231c88), published at
#   https://softwarewrighter.github.io/X_eTaL-extensions/blender/td/ and
#   fetched here from the repository at the commit that published them.
#   What was asked: docs/extensions-ask.md.
# License: MIT (the X_eTaL-extensions repository's, per its manifest.toml)
set -euo pipefail
. "$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)/scripts/venv.sh"
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cache="$here/cache/art"
commit=3a14f1e3b09245ca2ec8801f8d9fda3e184b01b2
base="https://raw.githubusercontent.com/softwarewrighter/X_eTaL-extensions/$commit/pages/blender/td"
mkdir -p "$cache"
get() {  # get FILE SHA256
  local f="$cache/$1"
  if [ ! -f "$f" ] || ! echo "$2  $f" | shasum -a 256 -c --status; then
    curl -sSfL -o "$f.part" "$base/$1"
    echo "$2  $f.part" | shasum -a 256 -c --status || { echo "fetch: $1 checksum mismatch" >&2; rm -f "$f.part"; exit 1; }
    mv "$f.part" "$f"
  fi
}
get arrow-1.glb ed49dc05d3db5e8520ea5b3b6d0b5cfd04594ead703553a10de4a317ec13637c
get arrow-1.png 96b2190ccd18d1ed4035c855168add283ed01cfeab9f949c844b426d3dd78cbd
get arrow-2.glb 2aedc7d5817610a17c25353955b2f51dc9a36d870c1177aa97485c32335dce1c
get arrow-2.png bcf544673f1778afcda2f5ac32c1d702075ac0751a6fdfb72f14758c20e4ea70
get arrow-3.glb 63442d5d0798fee6f5b9245145810c02592e047afc04ca315d9da58825a1b57d
get arrow-3.png d96d0014aa4f67491816dd735a66ea276a8595a8de57e914c80b5d1ed95b2d35
get cannon-1.glb 79066db0d0ac05e494005895644af39571dfceb60e857b80bf2200bf2bd6f0e8
get cannon-1.png 67f2a3b047a1dacd6ed8df1599e436cc1564d20d4862d52e22a27675e44a3e7a
get cannon-2.glb 4836f71365cd902a98521bbff7bd88ad6d5c223a7f27792c28dc870b655b501b
get cannon-2.png 21bb5a29763dc2f11d05040065c687539e53dc7b22f90c0275aef95bdf4364fc
get cannon-3.glb 76f910032c1a894ecb4291006cc99ea4dbdd24e1cbdb3eaaba684839dc918e61
get cannon-3.png 4323807f4e18f31db6c63359cb25d1fa23e950f2ebfc9fb5d5481cd8048eb57f
get frost-1.glb 6bb1423e1fdf51bf4601261700afacf96e0f4830e59da8ea3693ecb953936806
get frost-1.png b9389df0179f67852bba24e953bb1160c2da6c311d8d4ed4fd4133a7a74b28ab
get frost-2.glb 29fd408e90a5d2b34e59ac2b39bf4a52b4b939baeb6690ce093939acfb1b0114
get frost-2.png 9782513c3ee7d334d56114248a95d34c89d59b26af2423211d1892710f32c418
get frost-3.glb 164c302d03a8977ce8278b8872748bac4e965dc9a9826700f705d00d948d9815
get frost-3.png 81cbb2dc8c64ee35443ec67e70a94cb108e7645b5709f5f038fab71bce211c9e
get runner.glb b735aa8f472977ff4653afe1c7a6de9481a2104a26aafc38eb562e3bfe405c2d
get runner.png 91085e8132337679f7659b4925c9548fb328133ffbeb32c43e27e0165925698e
get brute.glb cb8fb94deb142472375ea3ea57eff95fd5e9e4f491dd329e412e439f6fc08a4a
get brute.png 7a81fca8d05feb2e1851a03cea6c504f4e28088f842085dedc1c526a744d813e
get shield.glb a6cd7ba72e66b96ab0a3c562c9b7fe935b86c48a3d60d4aabc66a5d6ed67b7a3
get shield.png 463b4f8f802557be29565f07a9d58bb63e08101547f2df122283a539d022bd52
