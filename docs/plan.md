# X_eTaL-games -- Implementation Plan

A gallery of small games written in X_eTaL (the eXperimental
eXtensible Typed Array Language, developed in `../X_eTaL`), each
playable from the command line and live in the browser. The source of
the ideas is `docs/research.txt` (archival, not normative); this plan
turns it, and the user's later additions, into sagas and steps.

Development is driven by agentrail sagas (one active saga in
`.agentrail/`, finished sagas archived to `.agentrail-archive/`), as in
`../X_eTaL` and `../X_eTaL-demos`. Every step ends with the gate (`just
gate`), docs updated (`README.md`, `CHANGES.md`, this plan,
`docs/xetal-asks.md`), a sane `.gitignore`, a detailed commit to
`main` (with the `.agentrail/` changes), `agentrail complete`, and a
push.

## Guiding principle

The games are chosen because their rules are array-shaped: boards,
grids, vectors of racers, cellular state, constraint tensors,
probability tables. Each game teaches one array-programming lesson
and happens to be playable:

| Game | The lesson |
| ---- | ---------- |
| Horse race | vectors, random selection, reduction |
| Tic-tac-toe | a 3 x 3 board, line extraction, masks |
| Robot chase | coordinate matrices, simultaneous motion, collision grouping |
| Shut the box | boolean masks, subset sums |
| Minesweeper | neighbourhoods by rotation, flood fill |
| 2048 | compress, merge, pad, rotate: composition |
| Lights out | boolean matrices, XOR, GF(2) linear algebra |
| Sudoku | a 9 x 9 x 9 candidate tensor, constraint propagation |
| Reversi | directional rays, bounded runs, bulk flips |
| Trek | nested spatial state, distances, stochastic simulation |

The rule that keeps this an X_eTaL gallery, not a JavaScript (or
Rust) one:

> If a game's rules can reasonably be expressed in X_eTaL, they MUST
> NOT be implemented in the page. The page turns a click into a move,
> runs the game's X_eTaL, and draws the arrays that come back.

## Architecture decisions

| # | Decision | Why |
| - | -------- | --- |
| A1 | X_eTaL is **vendored** into `vendor/xetal/` as a source snapshot of a committed ref of `../X_eTaL` (`just vendor [REF]`, default `HEAD`), recorded in `vendor/xetal/VENDORED`. Uncommitted work in `../X_eTaL` is never vendored. Same scripts as X_eTaL-demos. | X_eTaL moves fast; games need a recent but stable interpreter, refreshed deliberately, never mid-step. |
| A2 | The vendored CLI builds into `target/xetal/` (`just xetal`); every recipe runs that binary, not one on the PATH. | Goldens are tied to `VENDORED`. |
| A3 | Each game is its own sub-project, `games/<slug>/`: `game.toml` (title, summary, lesson, concepts, status, order, provenance, needs), `README.md`, `<slug>.xtl` (the rules and a scripted game), `expected/` goldens, optional `play.xtl` (interactive, `[]R_EAD`), optional `test.sh`, and `web/` (its own Cargo workspace, a Yew app). A game never reaches into another game. | Games evolve independently; one broken game never blocks another. |
| A4 | **A common game protocol** in X_eTaL, so pages know little about each game. Each game defines what applies of: `u:n_ew` (a seed or size -> state), `u:m_ove` (state, move -> state), `u:l_egal` (state -> moves), `u:s_tatus` (0 playing, 1 won, 2 lost, 3 draw), `u:v_iew` (state -> a display array); optionally `u:s_core`, `u:a_i` (the computer's move), `u:h_int`. State is a flat numeric array (no records yet) that the page passes back in as a literal each turn. | One page pattern for every game; a new game is mostly X_eTaL plus a small renderer. |
| A5 | **Goldens with scripted input.** A program with `expected/NAME.in` is run with that file as standard input, so interactive `play.xtl` sessions are tested move by move like any other program. All runs use `--seed 1`. | Interactive games stay tested without a browser. |
| A6 | The browser shell is copied from X_eTaL-demos' `shared/microscope` into `shared/microscope/` (independent from then on); once three games have pages, the game-specific parts (boards of clickable cells, score panel, move log, new-game/undo controls) are extracted into `shared/arcade/`. | The same "extract from three working pages" approach that worked for the demos. |
| A7 | The live site is built **locally** into `pages/` (`just pages`): a catalog `pages/index.html` from every `game.toml`, plus `pages/<slug>/` from trunk. `pages/` is committed; `.github/workflows/pages.yml` only uploads it. | Same as the sibling repos: simple, deterministic deploys. |
| A8 | A missing X_eTaL feature or a bug a game uncovers is **not** fixed here nor hidden: it goes in `docs/xetal-asks.md` (status, kind, games, why, minimal repro, workaround) and in the game's README. A game that cannot be built waits in the deferred saga. | X_eTaL owns its language decisions; this repo is a consumer. |
| A9 | **Third-party assets are never tracked.** A game that needs a downloaded file (a world map SVG, a star catalog) tracks only a fetch script (`games/<slug>/assets/fetch.sh`, recording the source URL, license and checksum); the fetched files land in a git-ignored `games/<slug>/assets/cache/`. `just fetch SLUG` runs it; builds and tests call it when the cache is empty. | The user's rule: track the scripts that get assets, not the assets. |
| A10 | `just` is the entry point (recipes call `scripts/*.sh`); docs are ASCII-only markdown checked by `sw-markdown-checker`; `CHANGES.md` records every commit, newest first, grouped by day, as in `../X_eTaL`. | One way to do things; consistent with the sibling repos. |

Open question (A9 and A7): the published site needs the map itself.
Until the user decides, a map game's page fetches the asset at build
time into `target/` and trunk copies it into the built `pages/<slug>/`
(which is committed). If built pages must not contain third-party
files either, the page would instead fetch the map from its source URL
at run time (licence and CORS permitting).

## Sources and provenance

| Game | Existing reference |
| ---- | ------------------ |
| Horse race | `sw-comp-history/apl-horse-race`: `src/race.apl` (the reconstruction) and `src/idiomatic-race.apl` (394 bytes) |
| Robot chase, Trek, Trek adventure, Guess | the COR24 BASIC live demos (`sw-embed/web-sw-cor24-basic`, `examples/*.bas`) |
| Sudoku | `sw-fun/sudoku` (local: `~/github/sw-fun/suduko`, Rust/Yew) |
| Shut the box | `softwarewrighter/shut_the_box` (Rust, Three.js) |
| Tic-tac-toe | X_eTaL's `demos/tttml*.xtl` (TTTML) |
| Mastermind | X_eTaL's `demos/classics/mastermind*.xtl` |
| Stargazer | `wrightmikea/stargazer-poc` (Rust/Yew star-name quiz on an SVG sky map) |
| Capitals | new: the stargazer pattern on a world map (user request, 2026-10-02) |
| m3 | `https://wrighter.space/staging/m3/` (not yet inspected: unreachable from the research session) |

## Gallery and sagas

Order: implementable games first, cheapest lesson first; a game whose
feasibility check (the first thing its step does) finds a blocking
X_eTaL gap is moved to saga 4 with its asks filed.

| Pri | Game (slug) | X_eTaL concepts | Est. X_eTaL LOC | Saga |
| --- | ----------- | --------------- | --------------- | ---- |
| P0 | horse-race | vectors, `r_oll!`, masks, reduce, `w_here` | 15-30 | 1 |
| P0 | tic-tac-toe | 3 x 3 board, line extraction (`s_elect` with a lines table), masks, minimax | 40-80 | 2 |
| P0 | robot-chase | N x 2 coordinates, sign of differences, `u_nique`/counts for collisions | 60-120 | 2 |
| P0 | shut-the-box | masks, subset sums via `e_ncode` of 0..511, filtering | 60-100 | 2 |
| P0 | minesweeper | neighbour counts by eight rotations, flood fill by `p_ower` to a fixed point | 70-130 | 2 |
| P0 | 2048 | compress, merge pairs, pad, rotate/reverse to orient | 60-110 | 2 |
| P1 | lights-out | XOR by a plus-shaped stencil; solve over GF(2) | 40-80 | 2 |
| P1 | connect-four | 6 x 7 board, column drop, windows in four directions | 70-130 | 3 |
| P1 | mastermind | equality masks, colour histograms by `t_able` | 40-80 | 3 |
| P1 | sudoku | 9 x 9 x 9 candidate tensor, eliminations by broadcasting | 150-300 | 3 |
| P2 | flood-it | connected region by iterated masks | 70-130 | 3 |
| P2 | fifteen | permutation, `i_ndexOf`, parity of solvability | 35-70 | 3 |
| P2 | nim | XOR of heaps by `e_ncode`/`d_ecode` | 25-50 | 3 |
| P2 | reversi | eight directional rays, bounded runs, bulk flips | 120-220 | 3 |
| P2 | capitals | lon/lat -> map projection of a capitals table, distractors by distance, score | 40-80 | 3 |
| P2 | stargazer | RA/Dec projection of a bright-star table, magnitude masks, nearest-star distractors | 50-100 | 3 |
| P2 | battleship | ship placement masks, probability map | 100-180 | 4 |
| P2 | trek | 8 x 8 galaxy of 8 x 8 sectors, scans, distances, combat | 250-450 | 4 |
| P3 | trek-adventure | tables, a state machine, inventory vectors | 200-400 | 4 |
| P3 | guess | RNG, comparison (a smoke test of input) | 10-20 | 4 |

Not planned: real-time games (Pong, Breakout, platformers): their work
is frame timing and collision geometry, not arrays.

## Saga 1 -- foundation  [ACTIVE]

Goal: the process, the vendored interpreter, the game sub-project
layout with scripted-input goldens, the live-site pipeline, and the
first game published end to end.

| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | scaffold | DONE: agentrail saga; CLAUDE.md (AGENTS.md a symlink); README (intro, games, build, status, copyright, license); COPYRIGHT; LICENSE; CHANGES.md; .gitignore; justfile; `scripts/gate.sh`; this plan; `docs/xetal-asks.md` |
| 2 | vendor-xetal | DONE: vendored 39938f3; the CLI is built with `XETAL_BUILD_SHA` set from `VENDORED`, so `xetal --version` names the vendored commit (checked by the gate). Planned: `just vendor [REF]` (scripts from X_eTaL-demos), `vendor/xetal/VENDORED`, `just xetal`, `just xetal-version`, `just eval`; `tools/vendor-probe` (xetal-play natively and for wasm32); the gate checks the vendored build |
| 3 | game-layout | DONE: as planned, plus `scripts/fetch-assets.sh`; `game.toml` fields slug, title, summary, lesson, concepts, status, order, sources, needs; the self-test covers scripted input. Planned: `games/_template`, `scripts/games.py` (list, check, json over `game.toml`), `scripts/test-games.sh` (goldens, `.in` as stdin, web/ cargo test and wasm32 check, test.sh), its self-test, `just new-game`, `run`, `play`, `show`, `test`, `test-game`, `bless`, `fetch` |
| 4 | pages-pipeline | DONE: shell copied (compiles unchanged against the newer vendored X_eTaL); catalog cards show the lesson; `just pages` fetches assets first; the self-test checks the catalog; Pages enabled (workflow build). Planned: `shared/microscope` copied in; `scripts/build-catalog.py` (cards from `game.toml`: lesson, concepts, status), `scripts/build-pages.sh`, `scripts/serve-pages.sh`, `.github/workflows/pages.yml` (upload only); Pages enabled; the deploy verified |
| 5 | horse-race | the race as one vector: `pos := pos + r_oll! 5 r_eshape 3`; the scripted race and an interactive pick-a-horse `play.xtl` (goldens with `.in`); the APL originals side by side in the README; web page: the track, the roll vector each round, the winner by `w_here pos = 'm_ax r_/ pos`; live |

## Saga 2 -- grids (P0 and the arcade shell)

| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | tic-tac-toe | the protocol (A4) on a 3 x 3 board; lines as an 8 x 3 index table; random and minimax players; play.xtl; page |
| 2 | robot-chase | robots as an N x 2 matrix moving together by the sign of the difference; wrecks where positions repeat; teleport; page |
| 3 | shut-the-box | open tiles as a mask; every subset of 1..9 by `e_ncode`; the legal moves are the subsets summing to the roll; page |
| 4 | arcade-shell | `shared/arcade/`: the game parts the three pages share (clickable boards, score and status, move log, new game/undo), extracted with the pages' tests unchanged |
| 5 | minesweeper | counts by eight rotations; reveal by flood fill to a fixed point; page |
| 6 | 2048 | a move as compress, merge, pad, oriented by `r_ev`/transposition; page (keyboard and swipe) |
| 7 | lights-out | play by XOR with a plus stencil; Solve over GF(2); page |
| 8 | gallery-1-release | catalog, README, per-game docs, screenshots, retrospective |

## Saga 3 -- algorithms as opponents, puzzles and quizzes

| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | connect-four | drop by column; wins by windows in four directions; heuristic and minimax players |
| 2 | mastermind | the code-breaking game; the player that keeps every consistent code (from X_eTaL's classic) |
| 3 | sudoku | candidates as a 9 x 9 x 9 tensor; singles and hidden singles by broadcasting; a candidate microscope |
| 4 | flood-it | the flooded region by iterated masks |
| 5 | fifteen | the sliding puzzle; solvability by permutation parity |
| 6 | nim | heaps, XOR strategy by binary digits |
| 7 | reversi | the eight rays, captures and flips |
| 8 | capitals | world map (fetched, not tracked: A9); a dot per capital; click a dot, pick the country from 4 choices (distractors chosen in X_eTaL by distance), feedback, the right answer shown, score and streak |
| 9 | stargazer | the stargazer-poc quiz in X_eTaL: a bright-star table (fetched, A9) projected by X_eTaL; click a star, name it from the choices, score |
| 10 | gallery-2-release | catalog, docs, retrospective |

## Saga 4 -- larger games and deferred

| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | m3-survey | inspect `wrighter.space/staging/m3/` (ask the user for its source), decide whether it belongs here |
| 2 | battleship | placement masks, a hunting player with a probability map |
| 3 | trek | the BASIC Star Trek as arrays: galaxy, sectors, SRS/LRS, phasers, torpedoes |
| 4 | trek-adventure | the BASIC adventure as tables and a state machine |
| 5 | guess | the smallest game: input and RNG |
| 6 | gallery-3-release | catalog, docs, final retrospective |

## Cross-cutting

- Refresh the vendored X_eTaL (`just vendor`) at a saga start or when
  an ask has landed upstream; never mid-step; its own commit, goldens
  re-run.
- When an ask lands, remove its workaround in the step that refreshes
  the vendor, and mark the ask landed.
