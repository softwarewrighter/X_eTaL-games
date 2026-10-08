#!/usr/bin/env bash
# Fetch the stargazer game's sky into assets/cache/ (git-ignored:
# third-party data is never committed, docs/plan.md A9), then turn it
# into the data file assets/cache/sky.toml with convert.py.
#
# Sources:
#   The Yale Bright Star Catalog, 5th revised edition (Hoffleit and
#   Warren, 1991; NASA, public domain), from CDS (catalog V/50):
#   https://cdsarc.cds.unistra.fr/ftp/V/50/
#   The IAU Catalog of Star Names (IAU Working Group on Star Names;
#   IAU products are Creative Commons Attribution: the source is the
#   IAU, https://www.iau.org/public/themes/naming_stars/), the WGSN's
#   text version: https://www.pas.rochester.edu/~emamajek/WGSN/
# Checked by SHA-256: an upstream update fails here until the sum is
# updated on purpose.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cache="$here/cache"
mkdir -p "$cache"
get() {  # get FILE URL SHA256
  local f="$cache/$1"
  if [ ! -f "$f" ] || ! echo "$3  $f" | shasum -a 256 -c --status; then
    curl -sSfL -o "$f.part" "$2"
    echo "$3  $f.part" | shasum -a 256 -c --status || { echo "fetch: $1 checksum mismatch" >&2; rm -f "$f.part"; exit 1; }
    mv "$f.part" "$f"
  fi
}
get bsc5.dat.gz https://cdsarc.cds.unistra.fr/ftp/V/50/catalog.gz 3dc44b1e90be8fbe5bcc7656032560f51275f985c7e3f783c9028e1838ec7bed
get iau-csn.txt https://www.pas.rochester.edu/~emamajek/WGSN/IAU-CSN.txt 84fac0c90f1b19abc491c2793469e0caa7b003ad1bc93a790ca41147010d0eb0
"$here/convert.py" "$cache/bsc5.dat.gz" "$cache/iau-csn.txt" "$cache/sky.toml"
