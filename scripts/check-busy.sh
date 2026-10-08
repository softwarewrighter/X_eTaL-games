#!/usr/bin/env bash
# The machine is free to build and to time (after X_eTaL's
# scripts/check-busy.sh): no trunk serve of this repository (it rebuilds
# on every change), no other cargo holding this repository's target/,
# and no other gate of this repository running. Timings taken on a busy
# machine fail for the wrong reason, so the gate checks first.
#   scripts/check-busy.sh
set -uo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
serving="$(pgrep -f "trunk serve" | while read -r p; do lsof -p "$p" 2>/dev/null | grep -q "$root" && echo "$p"; done | head -1)"
if [ -n "$serving" ]; then
  echo "check-busy: a trunk serve of this repository is running (pid $serving): stop it first"; exit 1
fi
for lock in "$root/target/debug/.cargo-lock" "$root/target/release/.cargo-lock"; do
  holder="$(lsof -t "$lock" 2>/dev/null | head -1 || true)"
  if [ -n "$holder" ]; then
    echo "check-busy: another cargo (pid $holder) holds $(basename "$(dirname "$lock")")/ of this repository's target: wait for it"; exit 1
  fi
done
others="$(pgrep -f "$root/scripts/gate.sh|scripts/gate.sh" | while read -r p; do [ "$p" != "$PPID" ] && [ "$p" != "$$" ] && lsof -p "$p" 2>/dev/null | grep -q "cwd.*$root\$" && echo "$p"; done | head -1)"
if [ -n "$others" ]; then
  echo "check-busy: another gate of this repository is running (pid $others): wait for it"; exit 1
fi
echo "check-busy: free to build (load $(uptime | sed 's/.*averages*: //'))"
