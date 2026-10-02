#!/usr/bin/env bash
# Test the game tooling itself in a scratch games directory: new-game
# makes a game that passes; a wrong expected output fails; an unexpected
# stderr fails; XETAL_BLESS=1 repairs it; a program reading standard
# input gets expected/<name>.in; the catalog shows it (escaped, its
# lesson, no play link without a web app); programs may not define l:
# names, libraries must export some and not be named like the game; a
# bad game.toml is rejected.
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
"$root/scripts/build-catalog.py" "$XETAL_GAMES_DIR/index.html" >/dev/null
grep -q '<h2>Probe &amp; Co</h2>' "$XETAL_GAMES_DIR/index.html" \
  || { echo "selftest: the catalog has no card for the game" >&2; exit 1; }
grep -q '<p class="lesson">' "$XETAL_GAMES_DIR/index.html" \
  || { echo "selftest: the catalog card has no lesson" >&2; exit 1; }
! grep -q 'href="probe/"' "$XETAL_GAMES_DIR/index.html" \
  || { echo "selftest: the catalog links a game with no web app" >&2; exit 1; }
# The program/library split: a program defining l: names is rejected,
# as is a library without exports or one named like the game.
printf '#!/usr/bin/env xetal\nl:finish := 15\n' > "$g/probe.xtl"
expect fail "a program defining l: names"
printf '#!/usr/bin/env xetal\n"r:" u_se< "Rules"\nr:finish + 1\n' > "$g/probe.xtl"
printf 'l:finish := 15\n' > "$g/Rules.xtl"
rm -f "$g/expected/probe.in"
XETAL_BLESS=1 "$t" probe >/dev/null
grep -qx 16 "$g/expected/probe.out" || { echo "selftest: a program using a library" >&2; exit 1; }
expect pass "a program using a library"
printf 'finish := 15\n' > "$g/Rules.xtl"
expect fail "a library exporting nothing"
printf 'l:finish := 15\n' > "$g/Rules.xtl"
# (On a case-insensitive file system Probe.xtl would be probe.xtl, so
# the program goes first.)
rm -f "$g/probe.xtl" "$g/expected/probe.out"
cp "$g/Rules.xtl" "$g/Probe.xtl"
said="$("$root/scripts/games.py" check 2>&1 || true)"
grep -q "may not share the game's name" <<<"$said" \
  || { echo "selftest: a library named like the game was not reported" >&2; exit 1; }
expect fail "a library named like the game"
rm -f "$g/Probe.xtl"
sed -i '' 's/^status = .*/status = "bogus"/' "$g/game.toml"
expect fail "a bad status"
echo "selftest-games: ok"
