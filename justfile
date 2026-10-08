# X_eTaL-games tasks. Recipes call scripts/*.sh, which hold the logic
# and work without just too. `just` alone lists the recipes.

set positional-arguments

# List the recipes
default:
    @just --list

# Get and build xetal at the known-good commit in XETAL_COMMIT (work/xetal, bin/xetal)
xetal:
    scripts/xetal.sh

# Move to another X_eTaL (default origin/main): writes XETAL_COMMIT and builds it; then the gate
xetal-bump ref="origin/main":
    scripts/xetal-bump.sh "$1"

# The known-good X_eTaL: XETAL_COMMIT and the binary's version
xetal-version:
    @cat XETAL_COMMIT
    @"$(scripts/build-xetal.sh)" --version | head -1

# Evaluate an expression with the vendored xetal: just eval "'+ r_/ 1 2 3"
eval expr:
    @"$(scripts/build-xetal.sh)" eval -e "$1"

# Check X_eTaL: the CLI builds and answers; xetal-play usable natively and for wasm32
check-vendor:
    scripts/check-vendor.sh

# The games, in catalog order
games:
    @scripts/games.py list

# Start a game sub-project from games/_template: just new-game horse-race "Horse race"
new-game slug title:
    scripts/new-game.sh "$1" "$2"

# Run a game's program (default SLUG.xtl, a scripted game): just run horse-race
run slug file="":
    @scripts/run-game.sh "$1" ${2:+"$2"}

# Play a game at the terminal (its play.xtl): just play horse-race
play slug:
    @scripts/run-game.sh "$1" play.xtl

# The X_eTaL REPL in a game's directory, its rules library importable: just repl horse-race
repl slug:
    cd games/{{slug}} && XETAL_PATH="$(cd ../../lib && pwd)" "$(../../scripts/build-xetal.sh)" repl

# Run a game's program as a notebook: each statement drawn, then its output
show slug file="":
    @scripts/run-game.sh --echo "$1" ${2:+"$2"}

# Test every game: expected outputs (with scripted input), web/ tests, test.sh
test:
    scripts/test-games.sh

# Test one game: just test-game horse-race
test-game slug:
    scripts/test-games.sh "$1"

# Rewrite one game's expected outputs from its programs (review the diff!)
bless slug:
    XETAL_BLESS=1 scripts/test-games.sh "$1"

# Fetch third-party assets (never tracked) into games/<slug>/assets/cache/
fetch *slugs:
    scripts/fetch-assets.sh "$@"

# Build the live site into pages/ (not tracked; the gate builds it too)
pages:
    scripts/build-pages.sh

# Serve the built pages/ as GitHub Pages will: http://127.0.0.1:8473/X_eTaL-games/
serve-pages port="8473":
    scripts/serve-pages.sh "$1"

# Serve one game's web app locally, rebuilt on change: just serve horse-race
serve slug port="8473":
    cd games/{{slug}}/web && trunk serve --release --port {{port}} --address 127.0.0.1

# Screenshot every game (from the built pages/) into games/<slug>/screenshot.png
screenshots *slugs:
    scripts/screenshots.sh "$@"

# Play every game in headless Chrome (from the built pages/) and compare with the goldens
browser-test *slugs:
    scripts/browser-test.mjs "$@"

# How big each game's X_eTaL is: lines and tokens (a markdown table)
size *slugs:
    @scripts/size.sh "$@"

# Time every game (scripted, terminal, page) and compare with bench/baseline.json; --baseline rewrites it
bench *args:
    scripts/bench.py "$@"

# Publish pages/ as the gh-pages branch's only commit (the live site); needs a clean work tree
publish:
    scripts/publish-pages.sh

# Play every game on the deployed site (after just publish) and compare with the goldens
verify-live:
    scripts/browser-test.mjs --url https://softwarewrighter.github.io/X_eTaL-games/

# The pre-commit gate: the sample (default, under a minute), --affected
# (plus what the change touches), --full (everything)
gate *args:
    scripts/gate.sh {{args}}

# Show the agentrail saga state and the current step
status:
    agentrail status

# Open the saga plan
plan:
    @cat docs/plan.md
