#!/usr/bin/env bash
# The pre-commit gate, in three sizes (after X_eTaL's D108).
#   scripts/gate.sh              sample: the end-to-end checks, in parallel,
#                                each to its own log under work/gate/: the
#                                pinned X_eTaL, spelling, the game tooling's
#                                self-test, the shared libraries, type
#                                comments, every game's goldens (a job per
#                                game, no Rust build), markdown. About 5
#                                seconds. Before every commit and push.
#   scripts/gate.sh --affected   the sample, then what the change touches
#                                (scripts/affected.py, from the files changed
#                                since origin/main): those games' page crates
#                                natively and for wasm32 (and the page shell's
#                                tests when it changed), their pages built,
#                                played in headless Chrome and timed. Before
#                                agentrail complete of a game or page step,
#                                and before just publish.
#   scripts/gate.sh --full       everything, every game: before a release and
#                                after an X_eTaL move (just xetal-bump).
# Every step of 5 seconds or more prints its time.
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
mode=sample
case "${1:-}" in
  --full) mode=full ;;
  --affected) mode=affected ;;
  "") ;;
  *) echo "usage: scripts/gate.sh [--affected|--full]" >&2; exit 2 ;;
esac
started=$SECONDS
timed() {  # timed LABEL CMD...: run, print the time when 5 s or more
  local t=$SECONDS; "${@:2}"; local s=$?
  [ $((SECONDS - t)) -lt 5 ] || echo "    ($1: $((SECONDS - t)) s)"
  return $s
}

scripts/check-busy.sh
XETAL_BIN="$(scripts/build-xetal.sh)"; export XETAL_BIN
logs="$root/work/gate"; rm -rf "$logs"; mkdir -p "$logs"

# The sample: every check in the background, each to its own log.
md=(README.md CHANGES.md lib/README.md docs/audit.md docs/bench.md docs/plan.md docs/style.md docs/xetal-asks.md docs/xetal-terminal-request.md)
for f in games/*/README.md shared/*/README.md; do [ -e "$f" ] && md+=("$f"); done
markdown() { for f in "${md[@]}"; do sw-markdown-checker -f "$f" >/dev/null || { sw-markdown-checker -f "$f"; return 1; }; done; echo "markdown: ok (${#md[@]} files)"; }
names=(); pids=()
job() { names+=("$1"); ( t=$SECONDS; "${@:2}"; s=$?; echo "($1: $((SECONDS - t)) s)"; exit $s ) > "$logs/$1.log" 2>&1 & pids+=($!); }
job vendor scripts/check-vendor.sh
job spelling bash -c 'scripts/check-spelling.py --self-test && scripts/check-spelling.py'
job tooling scripts/selftest-games.sh
job libraries scripts/test-lib.sh
job types scripts/check-types.py
job docs scripts/check-docs.py
job doctests bash -c 'cd lib && for f in [A-Z]*.xtl; do XETAL_PATH=. ../bin/xetal doc --test "$f" >/dev/null || { XETAL_PATH=. ../bin/xetal doc --test "$f"; exit 1; }; done; echo "doc tests: ok"'
job xref bash -c 'd="$(mktemp -d)"; ( ls lib/*.xtl | grep -v /test.xtl; ls games/*/*.xtl | grep -v _template ) | XETAL_PATH="$PWD/lib" xargs bin/xetal doc --out "$d" >/dev/null && echo "xref: builds"; s=$?; rm -rf "$d"; exit $s' 
job affected python3 scripts/affected.py --self-test
job markdown markdown
web=0; [ "$mode" = full ] && web=1
while IFS= read -r slug; do
  [ -n "$slug" ] && job "game-$slug" env TEST_GAMES_WEB=$web scripts/test-games.sh "$slug"
done < <(scripts/games.py list)
fail=0
for i in "${!pids[@]}"; do
  if wait "${pids[$i]}"; then echo "ok: ${names[$i]} $(tail -1 "$logs/${names[$i]}.log" | grep -o '[0-9]* s)' | tr -d ')')"; else echo "FAIL: ${names[$i]} (work/gate/${names[$i]}.log)"; cat "$logs/${names[$i]}.log"; fail=1; fi
done
echo "    (sample: $((SECONDS - started)) s)"
[ $fail = 0 ] || { echo "gate: FAILED"; exit 1; }

if [ "$mode" = sample ]; then echo "gate: ok (sample)"; exit 0; fi

# What the change touches (the affected gate), or every game (the full gate).
if [ "$mode" = full ]; then games=(all); else
  games=(); while IFS= read -r s; do [ -n "$s" ] && games+=("$s"); done < <(python3 scripts/affected.py)
fi
if [ ${#games[@]} = 0 ]; then echo "gate: ok ($mode: no game or page touched)"; exit 0; fi
if [ "${games[0]}" = all ]; then
  echo "==> every game's page (the shell, the page scripts or X_eTaL changed)"
  timed "the page shell" bash -c '(cd shared/microscope && cargo test -q >/dev/null 2>&1 && cargo check -q --target wasm32-unknown-unknown) || { (cd shared/microscope && cargo test -q); echo "FAIL: shared/microscope"; exit 1; }'
  echo "ok: shared/microscope"
  set --
else
  echo "==> the touched games: ${games[*]}"
  set -- "${games[@]}"
fi
# The page crates (the full gate tested them with the goldens above).
if [ "$mode" = affected ]; then
  for slug in $( [ $# -gt 0 ] && printf '%s\n' "$@" || scripts/games.py list ); do
    d="games/$slug/web"; [ -f "$d/Cargo.toml" ] || continue
    timed "$slug/web" bash -c "(cd $d && cargo test -q --workspace >/dev/null 2>&1 && cargo check -q --target wasm32-unknown-unknown >/dev/null 2>&1) || { (cd $d && cargo test -q --workspace; cargo check -q --target wasm32-unknown-unknown); exit 1; }" \
      && echo "ok: $slug/web (native and wasm32)" || { echo "FAIL: $slug/web"; exit 1; }
  done
fi
# Their pages built and played in a real browser.
timed "pages" scripts/build-pages.sh "$@" >/dev/null
if [ "$mode" = full ]; then
  # Every game timed, natively and in the browser (bench.py runs the
  # browser test itself).
  timed "browser and timings" scripts/bench.py
else
  timed "browser" scripts/browser-test.mjs "$@"
  # Timed natively: only the games whose X_eTaL ran differently (the page
  # shell and the scripts do not change how fast X_eTaL runs).
  t=(); while IFS= read -r s; do [ -n "$s" ] && t+=("$s"); done < <(python3 scripts/affected.py --timed)
  if [ ${#t[@]} -gt 0 ]; then
    [ "${t[0]}" = all ] && t=()
    timed "timings" scripts/bench.py --native ${t[@]+"${t[@]}"}
  fi
fi
echo "gate: ok ($mode, $((SECONDS - started)) s)"
