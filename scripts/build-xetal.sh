#!/usr/bin/env bash
# Build xetal at the known-good commit (scripts/xetal.sh: the clone in
# work/xetal, the binary linked as bin/xetal), quietly, and print the
# binary's path. Every script runs that binary, never one on PATH.
#   scripts/build-xetal.sh
set -euo pipefail
# A binary already built (the gate builds once and exports XETAL_BIN, so
# its parallel jobs never wait on cargo's lock).
if [ -n "${XETAL_BIN:-}" ] && [ -x "$XETAL_BIN" ]; then echo "$XETAL_BIN"; exit 0; fi
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
"$root/scripts/xetal.sh" 2>/dev/null || "$root/scripts/xetal.sh"
echo "$root/bin/xetal"
