#!/usr/bin/env bash
# Test the game tooling itself in a scratch games directory: new-game
# makes a game that passes; a wrong expected output fails; an unexpected
# stderr fails; XETAL_BLESS=1 repairs it; a program reading standard
# input gets expected/<name>.in; a bad game.toml is rejected.
#   scripts/selftest-games.sh
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
XETAL_GAMES_DIR="$(mktemp -d)"; export XETAL_GAMES_DIR
trap 'rm -rf "$XETAL_GAMES_DIR"' EXIT
t="$root/scripts/test-games.sh"
g="$XETAL_GAMES_DIR/probe"
expect() { # expect pass|fail DESCRIPTION
  if "$t" probe >/dev/null 2>&1; then got=pass; else got=fail; fi
  [ "$got" = "$1" ] || { echo "selftest: $2: expected $1, got $got" >&2; "$t" probe || true; exit 1; }
}
"$root/scripts/new-game.sh" probe "Probe & Co" >/dev/null
grep -q 'title = "Probe & Co"' "$g/game.toml"
expect pass "a fresh game"
echo 56 > "$g/expected/probe.out"
expect fail "a wrong expected output"
XETAL_BLESS=1 "$t" probe >/dev/null
expect pass "after blessing"
printf '#!/usr/bin/env xetal\n1 +\n' > "$g/probe.xtl"
printf '' > "$g/expected/probe.out"
expect fail "an unexpected error"
XETAL_BLESS=1 "$t" probe >/dev/null
[ -s "$g/expected/probe.err" ]
expect pass "an expected error"
printf '#!/usr/bin/env xetal\n"you said " c_at []R_EAD @\n' > "$g/probe.xtl"
rm -f "$g/expected/probe.err"
printf 'hello\n' > "$g/expected/probe.in"
printf 'you said hello\n' > "$g/expected/probe.out"
expect pass "scripted input from expected/probe.in"
printf 'goodbye\n' > "$g/expected/probe.in"
expect fail "different scripted input"
sed -i '' 's/^status = .*/status = "bogus"/' "$g/game.toml"
expect fail "a bad status"
echo "selftest-games: ok"
