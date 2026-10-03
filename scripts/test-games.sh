#!/usr/bin/env bash
# Test the games (every games/<slug>/ but the template), or the named ones:
#   - assets/fetch.sh, when present, is run first (it does nothing when
#     its cache is already there): third-party files are never tracked;
#   - each top-level <slug>/*.xtl runs with the vendored xetal (--seed 1,
#     pictures to work/draw/<slug>/, standard input from
#     expected/<name>.in when there is one, else empty) and its stdout
#     must equal expected/<name>.out and its stderr expected/<name>.err
#     (empty when there is no .err file; "drawn PATH" lines from []S_HOW
#     are left out);
#   - web/ (a Cargo workspace), when present: cargo test, and cargo
#     check for wasm32 (the browser build);
#   - test.sh, when present and executable: run it.
# XETAL_BLESS=1 rewrites the expected .out/.err files instead (review
# the diff!); .in files are written by hand.
# XETAL_GAMES_DIR overrides games/ (scripts/selftest-games.sh uses it).
#   scripts/test-games.sh [SLUG...]
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
"$root/scripts/games.py" check
xetal="$("$root/scripts/build-xetal.sh")"
# The shared libraries (lib/) are found by u_se< through XETAL_PATH.
export XETAL_PATH="$root/lib${XETAL_PATH:+:$XETAL_PATH}"
if [ $# -gt 0 ]; then slugs=("$@"); else
  slugs=(); while IFS= read -r s; do [ -n "$s" ] && slugs+=("$s"); done < <("$root/scripts/games.py" list)
fi
# same EXPECTED_STEM DIR: DIR/out and DIR/err match the expected files;
# the differences go to DIR/diff.
same() {
  local ok=0
  diff -u "$1.out" "$2/out" > "$2/diff" || ok=1
  if [ -f "$1.err" ]; then
    diff -u "$1.err" "$2/err" >> "$2/diff" || ok=1
  elif [ -s "$2/err" ]; then
    { echo "unexpected stderr:"; cat "$2/err"; } >> "$2/diff"; ok=1
  fi
  return $ok
}
fail=0; n=0
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT
for slug in ${slugs[@]+"${slugs[@]}"}; do
  d="${XETAL_GAMES_DIR:-$root/games}/$slug"
  [ -f "$d/game.toml" ] || { echo "test: no game $slug" >&2; exit 1; }
  n=$((n + 1))
  if [ -x "$d/assets/fetch.sh" ]; then
    "$d/assets/fetch.sh" >/dev/null || { echo "FAIL: $slug/assets/fetch.sh"; fail=1; continue; }
  fi
  for prog in "$d"/*.xtl; do
    [ -e "$prog" ] || continue
    name="$(basename "$prog" .xtl)"
    exp="$d/expected/$name"
    input=/dev/null; [ -f "$exp.in" ] && input="$exp.in"
    mkdir -p "$root/work/draw/$slug"
    (cd "$d" && "$xetal" run --seed 1 --draw "$root/work/draw/$slug" "$prog" \
      <"$input" >"$tmp/out" 2>"$tmp/err.raw") || true
    # A picture shown ([]S_HOW) is reported as "drawn PATH" on stderr; the
    # path is this machine's, so those lines are not part of the golden.
    grep -v '^drawn .*\.svg$' "$tmp/err.raw" > "$tmp/err" || true
    if [ "${XETAL_BLESS:-}" = 1 ]; then
      mkdir -p "$d/expected"; cp "$tmp/out" "$exp.out"
      if [ -s "$tmp/err" ]; then cp "$tmp/err" "$exp.err"; else rm -f "$exp.err"; fi
      echo "blessed: $slug/$name"; continue
    fi
    if [ ! -f "$exp.out" ]; then
      echo "FAIL: $slug/$name: no expected/$name.out (XETAL_BLESS=1 to create)"; fail=1
    elif same "$exp" "$tmp"; then
      echo "ok: $slug/$name$([ "$input" = /dev/null ] || echo ' (scripted input)')"
    else
      echo "FAIL: $slug/$name"; cat "$tmp/diff"; fail=1
    fi
  done
  if [ -f "$d/web/Cargo.toml" ]; then
    (cd "$d/web" && cargo test -q --workspace >/dev/null 2>&1) \
      && echo "ok: $slug/web" || { echo "FAIL: $slug/web"; (cd "$d/web" && cargo test -q --workspace) || true; fail=1; }
    (cd "$d/web" && cargo check -q --target wasm32-unknown-unknown >/dev/null 2>&1) \
      && echo "ok: $slug/web (wasm32)" || { echo "FAIL: $slug/web (wasm32)"; (cd "$d/web" && cargo check -q --target wasm32-unknown-unknown) || true; fail=1; }
  fi
  if [ -x "$d/test.sh" ]; then
    "$d/test.sh" && echo "ok: $slug/test.sh" || { echo "FAIL: $slug/test.sh"; fail=1; }
  fi
done
echo "test-games: $n game(s)$([ $fail = 0 ] && echo ', all passed' || echo ', FAILURES')"
exit $fail
