# X_eTaL-games tasks. Recipes call scripts/*.sh, which hold the logic
# and work without just too. `just` alone lists the recipes.

set positional-arguments

# List the recipes
default:
    @just --list

# Snapshot a committed ref of ../X_eTaL into vendor/xetal/ (default HEAD); commit it on its own
vendor ref="HEAD":
    scripts/vendor-xetal.sh "$1"

# Build the vendored xetal CLI into target/xetal/
xetal:
    @scripts/build-xetal.sh

# The vendored X_eTaL: what was vendored (VENDORED) and the binary's version
xetal-version:
    @cat vendor/xetal/VENDORED
    @"$(scripts/build-xetal.sh)" --version | head -1

# Evaluate an expression with the vendored xetal: just eval "'+ r_/ 1 2 3"
eval expr:
    @"$(scripts/build-xetal.sh)" eval -e "$1"

# Check the vendored X_eTaL: CLI builds and answers; xetal-play usable natively and for wasm32
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
    cd games/{{slug}} && "$(../../scripts/build-xetal.sh)" repl

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

# Build the live site into pages/ (committed; the Pages workflow publishes it)
pages:
    scripts/build-pages.sh

# Serve the built pages/ as GitHub Pages will: http://127.0.0.1:8096/X_eTaL-games/
serve-pages port="8096":
    scripts/serve-pages.sh "$1"

# Serve one game's web app locally, rebuilt on change: just serve horse-race
serve slug port="8095":
    cd games/{{slug}}/web && trunk serve --release --port {{port}} --address 127.0.0.1

# Screenshot every game (from the built pages/) into games/<slug>/screenshot.png
screenshots *slugs:
    scripts/screenshots.sh "$@"

# Play every game in headless Chrome (from the built pages/) and compare with the goldens
browser-test *slugs:
    scripts/browser-test.mjs "$@"

# The full pre-commit gate
gate:
    scripts/gate.sh

# Show the agentrail saga state and the current step
status:
    agentrail status

# Open the saga plan
plan:
    @cat docs/plan.md
