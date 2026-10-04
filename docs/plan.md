# X_eTaL-games -- Implementation Plan

A gallery of small games written in X_eTaL (the eXperimental
Extensible Typed Array Language, developed in `../X_eTaL`), each
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
| A4 | **Rules as a library, behind a common protocol.** Each game's rules are one X_eTaL library, `games/<slug>/<Name>.xtl` (exports under `l:`), imported by the scripted game, the terminal game and the page (`"g:" u_se< "Name"`; the page writes the library into the engine's in-memory store with `microscope::run::library`). It defines what applies of: `l:n_ew` (a seed or size -> state), `l:m_ove` (state, move -> state), `l:l_egal` (state -> moves), `l:s_tatus` (0 playing, 1 over or won, 2 lost, 3 draw), `l:v_iew` (state -> a display array); optionally `l:s_core`, `l:a_i` (the computer's move), `l:h_int`. State is a flat numeric array (no records yet) that the page passes back in as a literal each turn; randomness comes from `r_oll!` with a seed the page changes every turn. | The rules are written once; one page pattern for every game; a new game is mostly X_eTaL plus a small renderer. (Decided in the horse-race step, when it turned out the engine's store lets a page use a library.) |
| A5 | **Goldens with scripted input.** A program with `expected/NAME.in` is run with that file as standard input, so interactive `play.xtl` sessions are tested move by move like any other program. All runs use `--seed 1`. | Interactive games stay tested without a browser. |
| A6 | The browser shell was copied from X_eTaL-demos' `shared/microscope` into `shared/microscope/` (independent from then on) and became one shared game page (A11). Boards and pictures come from X_eTaL itself (text it prints, `[]G_RID`/`[]S_HOW` pictures), never from page code. | One page for every game; nothing on a page that X_eTaL did not produce. (Revised in saga 2: the planned `shared/arcade` of clickable boards was dropped by A11.) |
| A7 | The live site is built **locally** into `pages/` (`just pages`): a catalog `pages/index.html` from every `game.toml`, plus `pages/<slug>/` from trunk. `pages/` is committed; `.github/workflows/pages.yml` only uploads it. | Same as the sibling repos: simple, deterministic deploys. |
| A8 | A missing X_eTaL feature or a bug a game uncovers is **not** fixed here nor hidden: it goes in `docs/xetal-asks.md` (status, kind, games, why, minimal repro, workaround) and in the game's README. A game that cannot be built waits in the deferred saga. | X_eTaL owns its language decisions; this repo is a consumer. |
| A9 | **Third-party assets are never tracked.** A game that needs a downloaded file (a world map SVG, a star catalog) tracks only a fetch script (`games/<slug>/assets/fetch.sh`, recording the source URL, license and checksum); the fetched files land in a git-ignored `games/<slug>/assets/cache/`. `just fetch SLUG` runs it; builds and tests call it when the cache is empty. | The user's rule: track the scripts that get assets, not the assets. |
| A10 | `just` is the entry point (recipes call `scripts/*.sh`); docs are ASCII-only markdown checked by `sw-markdown-checker`; `CHANGES.md` records every commit, newest first, grouped by day, as in `../X_eTaL`. | One way to do things; consistent with the sibling repos. |
| A11 | **A page shows only what X_eTaL prints or draws.** One shared page (`shared/microscope/src/page.rs`): the game's `play.xtl` unmodified in a terminal, the scripted `<slug>.xtl` as a notebook (each statement, then its output, as `just show`), pictures only from `[]S_HOW`, the sources decorated by X_eTaL's own renderer; controls only start a new game or restart one. No boards, highlights, stage chips, verdicts or panels computed from game data in Rust. Tests: the page's terminal and notebook reproduce the CLI goldens, natively and in headless Chrome (`just browser-test`). | Accurate demos that happen to run in the browser; a Rust UI dressed around X_eTaL would misrepresent what the language does. A fancy UI waits until X_eTaL itself can produce one (a web framework extension serving HTML and CSS). (The user's rule, 2026-10-02.) |

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

Order: the APL original first, then the COR24 BASIC games (the user's
choice), then implementable games, cheapest lesson first; a game whose
feasibility check (the first thing its step does) finds a blocking
X_eTaL gap is moved to saga 9 with its asks filed.

| Pri | Game (slug) | X_eTaL concepts | Est. X_eTaL LOC | Saga |
| --- | ----------- | --------------- | --------------- | ---- |
| P0 | horse-race | vectors, `r_oll!`, masks, reduce, `w_here` | 15-30 | 1 |
| P0 | tic-tac-toe | 3 x 3 board, line extraction (`s_elect` with a lines table), masks, minimax | 40-80 | 3 |
| P0 | robot-chase | N x 2 coordinates, sign of differences, `u_nique`/counts for collisions | 60-120 | 2 |
| P0 | shut-the-box | masks, subset sums via `e_ncode` of 0..511, filtering | 60-100 | 3 |
| P0 | minesweeper | neighbour counts by eight rotations, flood fill by `p_ower` to a fixed point | 70-130 | 3 |
| P0 | 2048 | compress, merge pairs, pad, rotate/reverse to orient | 60-110 | 3 |
| P1 | lights-out | XOR by a plus-shaped stencil; solve over GF(2) | 40-80 | 8 |
| P1 | connect-four | 6 x 7 board, column drop, windows in four directions | 70-130 | 8 |
| P1 | mastermind | equality masks, colour histograms by `t_able` | 40-80 | 8 |
| P1 | sudoku | 9 x 9 x 9 candidate tensor, eliminations by broadcasting | 150-300 | 8 |
| P2 | flood-it | connected region by iterated masks | 70-130 | 8 |
| P2 | fifteen | permutation, `i_ndexOf`, parity of solvability | 35-70 | 8 |
| P2 | nim | XOR of heaps by `e_ncode`/`d_ecode` | 25-50 | 8 |
| P2 | reversi | eight directional rays, bounded runs, bulk flips | 120-220 | 8 |
| P2 | capitals | lon/lat -> map projection of a capitals table, distractors by distance, score | 40-80 | 8 |
| P2 | stargazer | RA/Dec projection of a bright-star table, magnitude masks, nearest-star distractors | 50-100 | 8 |
| P2 | battleship | ship placement masks, probability map | 100-180 | 9 |
| P2 | trek | 8 x 8 galaxy of 8 x 8 sectors, scans, distances, combat | 250-450 | 2 |
| P3 | trek-adventure | tables, a state machine, inventory vectors | 200-400 | 2 |
| P3 | guess | RNG, comparison (a smoke test of input) | 10-20 | 2 |

Not planned: real-time games (Pong, Breakout, platformers): their work
is frame timing and collision geometry, not arrays.

## Priorities (2026-10-03)

Reprioritized after the ecosystem review in `../X_eTaL/docs/research4.txt`:
stop adding breadth; stabilize, synchronize, explain, and give people
one obvious path. Nine live games are enough before a wider launch;
what matters now is showing that different games collapse onto the
same few array idioms. In order:

1. Saga 4, idioms: every game rewritten in idiomatic X_eTaL on the
   shared libraries, ending with an "array idiom -> game" table as the
   games' front door. No new games in it (lights out moves after the
   launch).
2. Saga 5, launch: X_eTaL's terminal pane in every game as soon as it
   lands (the replay workaround gone); the promotion-blocker bugs that
   touch the games; a downstream performance baseline and regression
   check (for X_eTaL's `t_able`/`i_nner` fix, saga 30); the Start Here
   page; a cross-repo status audit and a tagged, known-compatible
   snapshot.
3. Saga 6, macros: `.xtlm` macro libraries (Test/Assert, Control) in
   the games as soon as X_eTaL ships them: "Extensible" is in the name.
4. Post-launch: saga 7 (enums, tuples, records, signatures, typed
   errors, extensions, amend, local functions, screen control beyond
   the terminal pane), saga 8 (new games: opponents, puzzles,
   quizzes, lights out) and saga 9 (larger games).

## Saga 1 -- foundation  [DONE]

Goal: the process, the vendored interpreter, the game sub-project
layout with scripted-input goldens, the live-site pipeline, and the
first game published end to end.

| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | scaffold | DONE: agentrail saga; CLAUDE.md (AGENTS.md a symlink); README (intro, games, build, status, copyright, license); COPYRIGHT; LICENSE; CHANGES.md; .gitignore; justfile; `scripts/gate.sh`; this plan; `docs/xetal-asks.md` |
| 2 | vendor-xetal | DONE: vendored 39938f3; the CLI is built with `XETAL_BUILD_SHA` set from `VENDORED`, so `xetal --version` names the vendored commit (checked by the gate). Planned: `just vendor [REF]` (scripts from X_eTaL-demos), `vendor/xetal/VENDORED`, `just xetal`, `just xetal-version`, `just eval`; `tools/vendor-probe` (xetal-play natively and for wasm32); the gate checks the vendored build |
| 3 | game-layout | DONE: as planned, plus `scripts/fetch-assets.sh`; `game.toml` fields slug, title, summary, lesson, concepts, status, order, sources, needs; the self-test covers scripted input. Planned: `games/_template`, `scripts/games.py` (list, check, json over `game.toml`), `scripts/test-games.sh` (goldens, `.in` as stdin, web/ cargo test and wasm32 check, test.sh), its self-test, `just new-game`, `run`, `play`, `show`, `test`, `test-game`, `bless`, `fetch` |
| 4 | pages-pipeline | DONE: shell copied (compiles unchanged against the newer vendored X_eTaL); catalog cards show the lesson; `just pages` fetches assets first; the self-test checks the catalog; Pages enabled (workflow build). Planned: `shared/microscope` copied in; `scripts/build-catalog.py` (cards from `game.toml`: lesson, concepts, status), `scripts/build-pages.sh`, `scripts/serve-pages.sh`, `.github/workflows/pages.yml` (upload only); Pages enabled; the deploy verified |
| 5 | horse-race | DONE: rules as a library, HorseRace.xtl (A4 revised), used by horse-race.xtl, play.xtl (betting coins; golden from expected/play.in) and the page (`microscope::run::library`, `output_seeded`); the page shows the track X_eTaL draws, each round's rolls and the stage in the rules; green favicon. Planned: the race as one vector: `pos := pos + r_oll! 5 r_eshape 3`; the scripted race and an interactive pick-a-horse `play.xtl` (goldens with `.in`); the APL originals side by side in the README; web page: the track, the roll vector each round, the winner by `w_here pos = 'm_ax r_/ pos`; live |

## Saga 2 -- the COR24 BASIC games  [DONE but the terminal pane]

The user's order (2026-10-02): the APL original first (the horse race,
saga 1), then the BASIC games of the COR24 live demos
(`sw-embed/web-sw-cor24-basic/examples/*.bas`, MIT, the user's own),
which port quickly and show the contrast between line-numbered scalar
BASIC and whole-array X_eTaL. A port is a one-time rewrite of the same
kind of game in X_eTaL's own style: no copy of the `.bas`, no links,
no attribution sections.

| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | guess | DONE: NumberGuess.xtl (answer rule item by item; `l:p_ossible`, every candidate against every guess by `t_able`; the halving player on all 100 secrets at once: 1 2 4 8 16 32 37); play.xtl as the BASIC plays; the page runs play.xtl unmodified in a terminal (`microscope::terminal::session`: typed history replayed, echoed where it is read); the layout check enforces the program/library split. Planned: guess the number (guess.bas, guess-random.bas): the smallest game; the terminal page pattern: an output pane and a command line, the program re-run with the whole typed history as its input (`xetal-store` typed lines) and a fixed seed per game |
| 2 | accurate-pages | DONE: A11; `microscope::page::GamePage` (terminal, notebook, sources), `terminal::notebook`, pictures from `[]S_HOW`, `?seed=N`; HorseRace's view marks the winners itself; the horse race and guess pages are now a few lines each; `scripts/browser-test.mjs` (headless Chrome over the DevTools protocol, no packages) types `play.in` into each built page and compares the terminal and notebook with the goldens; in the gate. |
| 3 | robot-chase | DONE: RobotChase.xtl (state one flat vector; robots a 2 by 12 matrix moved by the sign of P - R; crashes by a 12 by 12 table; new wrecks by a 256 by 12 table; the scan by `'+ r_/_24` on a 4 by 4 by 4 by 4 array; random squares by `g_rade` of random keys); play.xtl as the BASIC plays (keypad, teleport, scan, resign); robot-chase.xtl shows one step and a made-up crash; the shared page. Planned: robot-chase.bas: robots as an N x 2 matrix all moving at once by the sign of the difference; wrecks where positions repeat; teleport; the board page |
| 4 | trek-adventure | DONE: TrekAdventure.xtl: every line of text one row of a table (text, key, condition item, value; 157 rows), a room or message picked by one mask (`m_ember?`, `w_here`), exits a 9 by 9 table, the state one vector of 12 changed by a table-driven update; play.xtl plays as the BASIC (numeric menus, 30 turns, the Klingon clock, three endings), its golden a full winning session; trek-adventure.xtl plays the walkthrough into a 12 by 12 table of states and shows the losses; the shared page. |
| 5 | trek | DONE: StarTrek.xtl (galaxy as three 8 by 8 planes made at once; entering a quadrant places everything by a random order and its inverse; courses a 2 by 8 table, paths by `t_able`, first obstacle by `w_here`; phasers one masked subtraction; LRS a 5 by 5 window by one `s_elect_2` from a 3 by 64 matrix, characters by `r_avel_2`; a destroyed Klingon leaves the galaxy too, unlike the BASIC); play.xtl with the BASIC's prompts; trek.xtl shows the planes, a path, a fight and running out of time; the shared page. |
| 6 | gallery-1-release | DONE: catalog intro and summaries match the accurate pages; screenshots for every game; README and plan; this retrospective. |
| 7 | terminal-pane | WAITING on X_eTaL saga 25 (a resumable evaluator: a run returns "waiting for a line" at `[]R_EAD` and continues on Enter, and a shared terminal pane in Rust and Yew modelled on web-sw-tos). Then: vendor it, swap the replay terminal for that pane, keep the browser tests. There will be no WASI build of `xetal` (X_eTaL decided, 2026-10-02): pages keep the engine, `xetal-play`, in Yew. No game uses a browser dialog for input, ever. |
| 8 | terminal-shell | folded into accurate-pages (the shared page) and terminal-pane |

### Saga 2 retrospective

Delivered the four COR24 BASIC games (guess the number, robot chase,
trek adventure, Star Trek) live, and changed how every page works.
Learned, mostly from the user:

- A page must show only what X_eTaL prints or draws (A11). The first
  pages were a Rust UI dressed around X_eTaL (a highlighted track,
  stage chips, a candidate grid); that misrepresented the language.
  Now every page is the same shared page: the game's `play.xtl` in a
  terminal, its scripted program as a notebook, the sources. A fancy
  UI waits until X_eTaL itself can serve one.
- Tests prove the browser shows what the command line shows: each
  page's terminal and notebook reproduce the goldens natively
  (`tests/page.rs`) and in headless Chrome (`just browser-test`, the
  DevTools protocol over Node's WebSocket, no packages).
- The `xetal` binary will not be built for WASI (X_eTaL's decision):
  input waits on X_eTaL's terminal pane over a resumable evaluator;
  until then the page replays the typed history on every line.
- Ports are one-time rewrites in X_eTaL's own style; no copies, links
  or attribution of the BASIC.
- A game's rules are one library; on macOS a library named like its
  game (`Guess.xtl`, `Trek.xtl`) is the same file as the program, so
  the layout check forbids it.
- Right-to-left reading without precedence caught every port at least
  once (`(p - 1 + t_ally v)` is `p - (1 + t_ally v)`); the goldens and
  the page tests caught each before a commit.
- Array ideas that carried the games: a random permutation by `g_rade`
  of random keys, and its inverse by `g_rade` again; paths as `t_able`
  of steps; collision and membership tables; text as a table of rows
  picked by one mask; several items of a state vector updated at once
  by a table.

## Saga 3 -- grids (P0, boards drawn by X_eTaL)  [DONE]

Opens with a vendor refresh: X_eTaL 70e129d (transpose is `o_\`; a
run that waits for typed lines and resumes, the first part of X_eTaL
saga 25); every golden, page test and browser test passed unchanged.

| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | tic-tac-toe | DONE: TicTacToe.xtl (board a vector of 9; the lines an 8 by 3 index table, every line's sum by one selection and one reduce; the quick player rates all squares at once through a 9 by 8 square-line table; minimax by recursion with `e_ach`, too slow from an empty board for the browser (17 s natively), so shown mid-game); play.xtl (you are X); the shared page. |
| 2 | shut-the-box | DONE: ShutTheBox.xtl (all 512 subsets a 9 by 512 bit matrix by `e_ncode`, their sums one inner product, moves the columns that fit and make the roll, rows by `o_\`; a high-tiles player); play.xtl lists the moves and asks for tiles; the scripted game's high-tiles player shuts the box; the shared page. |
| 3 | x-pictures | DONE: `[]S_HOW []G_RID` boards on the shared page: tic-tac-toe and robot chase show their boards as pictures X_eTaL draws beside the text (`l:p_icture`); the page scales them up (CSS only); the runner leaves out the command line's "drawn PATH" stderr lines; page tests and the browser test check that the pictures appear. Ask filed: numbers in `[]G_RID` cells and a colour scale (2048, counts). |
| 4 | minesweeper | DONE: Mines.xtl (counts by `o_-_12` rotations of the bordered mine board summed, as Life; reveal by a flood fill grown to a fixed point by `m_atch`; flags by `!=`; codes for text and the `[]G_RID` picture); play.xtl (open, f to flag); the scripted game shows the fill growing and clears the field; the shared page. |
| 5 | 2048 | DONE: Twenty48.xtl (compress by running counts and a 4 by 4 by 4 table, merge by places in runs from a max-scan, the moves by turning the board with `r_ev_2` and `o_\`, score, spawn, status); play.xtl (a d w s, q); the scripted corner player; the shared page. Found and fixed `f s = 1 ?` (reads `f (s = 1)`) in minesweeper and robot chase, right only by coincidence. |
| 6 | lights-out | MOVED to saga 4 (written in the new style, after the shared libraries) |
| 7 | gallery-2-release | DONE: nine games live; catalog and README current; this retrospective. The terminal pane moved to saga 5. |

### Saga 3 retrospective

Delivered tic-tac-toe, shut the box, minesweeper and 2048 live, the
vendored X_eTaL refreshed to 70e129d (every output unchanged), and
boards as pictures X_eTaL draws (`[]S_HOW []G_RID`). Learned:

- `[]G_RID` suits boards of characters and of 0/1 values; boards of
  numbers need digits in cells (asked for).
- Right to left without precedence struck again, worst where it was
  silent: `f s = 1 ?` reads `f (s = 1)` and two games were right only
  by coincidence. A new rule of thumb: put the constant on the left
  (`1 = f s ?`).
- The page test that every typed line is read caught a scripted
  session with lines left over.
- The user's review closed the saga early: the games use too small a
  part of the language. Saga 4 rewrites them in idiomatic X_eTaL, and
  lights out will be written in that style from the start.

## Saga 4 -- idioms: the games rewritten in idiomatic X_eTaL  [DONE]

The user's review (2026-10-03): the games so far are "ported", like C
written in assembler style or C++ restricted to its C subset. They use
a narrow, scalar-ish part of X_eTaL: hand-packed state vectors read by
index accessors, long `c_at` chains, recursion where a table or a
scan would do, helpers copied from game to game, comments in place of
types. This saga rewrites every game with the features X_eTaL has
today (vendored 70e129d); saga 5 adopts the features still to come.

Goals for every game: smaller (lines of X_eTaL, measured before and
after), array-style solutions first, small named pieces composed
(trains), shared libraries instead of copied helpers, and the same
outputs: the goldens, page tests and browser tests guard every
refactor (an output that changes is a decision, explained in the
commit, not an accident).

| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | metrics-style | DONE: `just size` and `docs/style.md`; the sizes before the saga are below. Planned: `scripts/size.sh` (`just size`): lines and tokens of X_eTaL per game, recorded in each README and in this plan, before and after; `docs/style.md`: the house style for game code (array first; trains for small point-free pieces, `[f g h]`; libraries for anything two games share; named sub-results over long chains; `# ::` type comments on every export, checked against `xetal type` goldens; status and event codes named as constants; right-to-left traps and how the goldens catch them) |
| 2 | shared-libraries | DONE: `lib/` Play, Text, Board, State with `lib/test.xtl` and type goldens (`scripts/test-lib.sh`, in the gate); `XETAL_PATH` set by the scripts; `page::Game` lists shared libraries (shown as sources); the games adopt them in their own steps. Planned: X_eTaL libraries this repo shares across games (in `lib/`, found through `XETAL_PATH` by the scripts and through the page's store, so `GamePage` takes several libraries): `Play` (terminal input: a number, a choice, a yes/no, a list of lines printed; every `play.xtl` copied these), `Board` (cell numbers to row and column and back, bordered neighbourhoods by rotation, a matrix of codes to text with spacing and labels, the same to a `[]G_RID` picture), `Text` (printing a strand of lines, a table of text rows picked by key and condition, as trek adventure does), `State` (a functional update of several items, as trek and trek adventure each wrote) |
| 3 | horse-race-idioms | DONE: the leaders and winners as trains (`['m_ax r_/ = i_d]`, `[w_here 'm_ax r_/ = i_d]`), the track's table simplified, `Play` for input, type comments on every export (checked by `scripts/check-types.py`, in the gate); outputs unchanged; 56/385 to 55/370 lines/tokens. Planned: trains for the small pieces (`[...]` for the winners, the finish test); `Board`/`Text` for the track |
| 4 | guess-idioms | DONE: the answer rule as a dyadic fork `t [< - >] g` (a named train is monadic, so it is written inline); the range update by a max and a min; `Play` for input; type comments; outputs and types unchanged; 59/429 to 58/407. |
| 5 | robot-chase-idioms | DONE: `Board` (cells, rows/columns, spacing), the sign as a fork `d [> - <] 0`, robots per square as one inner product (`l:o_nSquares`, used for crashes, wrecks, the view and the scan), one `l:c_odes` for text and picture, `Play`; `Board`'s `l:r_c` fixed to give rows over columns; outputs unchanged; 167/1410 to 149/1192. Planned: `Board` for cells, neighbours, view and picture; the state's parts named; fewer index accessors |
| 6 | trek-adventure-idioms | DONE: the text table as one 3-row matrix read by `Text`'s `l:h_olds`, printing by `Text`, updates by `State`; the state's positions named; every action's refusals as data (`l:r_efusal`: failed checks pick the message); `Play`; outputs unchanged; 369/2020 to 365/2031 (names cost tokens; the actions read as their conditions). Planned: `Text` for the table of lines (key, condition) and its printer; `State` for updates; the command dispatch as a table, not a chain of guards |
| 7 | trek-idioms | DONE: `Board` (sectors, spaced scan), `State` (updates, stretches), `Text` (lines, the repeated clamp message as a strand), `Play`; the state's positions named; outputs unchanged; 330/2542 to 317/2402. Planned: `Board`, `State`, `Play`; paths and scans tightened; the long-range scan by `Board` |
| 8 | tic-tac-toe-idioms | DONE: the quick player's rating as one inner product of four features and four weights; minimax as negamax; the status as the first condition that holds; one code board for text (`Board`'s spacing) and picture; `Play`; outputs unchanged; 92/667 to 85/632. |
| 9 | shut-the-box-idioms | DONE: the moves listed by `m_ap` over their row numbers and printed by `Text`; the score an inner product; the status as the first condition; the row spaced by `Board`; tiles read by `Play`; outputs unchanged; 77/498 to 64/473. |
| 10 | minesweeper-idioms | DONE: neighbour counts, squares and spacing from `Board`; input by `Play`'s new `l:l_ine` and `l:n_umbersIn`; type comments; the fill kept as recursion to a fixed point (it reads as one); outputs and types unchanged; 94/774 to 88/688. |
| 11 | 2048-idioms | DONE: the move left as a train of atops `[l:c_ompress [l:m_erge l:c_ompress]]`; turning as transpose and reverse each to a power of 0 or 1 (no cases); the status as the first condition; `Play`; type comments; outputs and types unchanged; 101/633 to 89/641. |
| 12 | idioms-release | DONE: the sizes after (below), the idiom table in the README and the catalog, the style guide's idioms, the asks, this retrospective. Planned: sizes before and after for every game; the "array idiom -> game" table (README and catalog): neighbourhood by rotate and reduce (minesweeper), masks over all subsets (shut the box), compress -> merge -> pad (2048), a coordinate matrix moving at once (robot chase), windows and paths (trek), lines by indexing and minimax (tic-tac-toe), text as a table picked by masks (trek adventure), every candidate against every guess (guess), a vector of racers (horse race); READMEs explain the idioms used; retrospective; the asks it raised |

Sizes before the saga (`just size`, 2026-10-03: lines not blank or
comment / X_eTaL tokens):

| Game | Rules (lines / tokens) | Scripted | Terminal | Total |
| ---- | ---------------------- | -------- | -------- | ----- |
| horse-race | 15 / 164 | 16 / 63 | 25 / 158 | 56 / 385 |
| guess | 16 / 164 | 26 / 176 | 17 / 89 | 59 / 429 |
| robot-chase | 97 / 1023 | 26 / 142 | 44 / 245 | 167 / 1410 |
| trek-adventure | 288 / 1548 | 31 / 231 | 50 / 241 | 369 / 2020 |
| trek | 274 / 2260 | 22 / 93 | 34 / 189 | 330 / 2542 |
| tic-tac-toe | 45 / 444 | 24 / 98 | 23 / 125 | 92 / 667 |
| shut-the-box | 28 / 228 | 20 / 102 | 29 / 168 | 77 / 498 |
| minesweeper | 51 / 489 | 24 / 158 | 19 / 127 | 94 / 774 |
| 2048 | 54 / 389 | 22 / 109 | 25 / 135 | 101 / 633 |
| all | | | | 1345 / 9358 |

Sizes after the saga (`just size`, 2026-10-04):

| Game | Rules (lines / tokens) | Scripted | Terminal | Total |
| ---- | ---------------------- | -------- | -------- | ----- |
| horse-race | 15 / 155 | 16 / 63 | 24 / 152 | 55 / 370 |
| guess | 16 / 156 | 26 / 176 | 16 / 75 | 58 / 407 |
| robot-chase | 81 / 823 | 26 / 142 | 42 / 227 | 149 / 1192 |
| trek-adventure | 289 / 1585 | 31 / 231 | 45 / 215 | 365 / 2031 |
| trek | 266 / 2146 | 22 / 93 | 29 / 163 | 317 / 2402 |
| tic-tac-toe | 39 / 417 | 24 / 98 | 22 / 117 | 85 / 632 |
| shut-the-box | 21 / 240 | 20 / 102 | 23 / 131 | 64 / 473 |
| minesweeper | 45 / 411 | 24 / 158 | 19 / 119 | 88 / 688 |
| 2048 | 43 / 405 | 22 / 109 | 24 / 127 | 89 / 641 |
| (lib/, shared) | 67 / 576 | | | 67 / 576 |
| all | | | | 1337 / 9412 |

### Saga 4 retrospective

Every game was rewritten with the X_eTaL of 70e129d, its outputs
unchanged throughout (each refactor checked by the goldens, the page
tests and headless Chrome; only the libraries' type lists changed, as
new named pieces appeared). The games' own code shrank about 6 % (1345
to 1270 lines, 9358 to 8836 tokens) while 67 lines of shared, tested
library (`lib/`: Play, Text, Board, State) replaced helpers six games
had copied; the total is flat. The gain is in what the code says:

- Branches became data: the first condition that holds picks a
  status or a refusal (`f_irst (conds r_eplicate codes) c_at 0`), in
  trek adventure, tic-tac-toe, shut the box and 2048.
- Small rules became trains: the leaders `['m_ax r_/ = i_d]`, the sign
  `d [> - <] 0` and the answer `t [< - >] g` (dyadic forks, written
  inline: a named train is monadic), the 2048 move as a train of
  atops.
- Loops of weighting became inner products: tic-tac-toe's ratings,
  robot chase's robots per square, shut the box's score.
- Cases became arithmetic: 2048 turns its board by transpose and
  reverse each to a power of 0 or 1.
- Every library export carries its inferred type as a checked comment.

Learned: trains read well for two- or three-part rules and badly
beyond; named state positions cost tokens but make updates
self-explanatory; `m_ap` maps items, not rows (map over row numbers).

Asks this saga is expected to raise (filed when hit): functional
update / amend (`l:u_pdate` was written twice), dyadic `f_ormat`
(aligned numbers), key/group.

## Saga 5 -- launch: stabilize, synchronize, explain

What must be true before a deliberate wider launch (research4's
promotion gate, as it touches the games).

| # | Step slug | Delivers | Waits on |
| - | --------- | -------- | -------- |
| 1 | terminal-pane | every game's page on X_eTaL's terminal pane (a run that waits at `[]R_EAD` and resumes; no replay of the typed history); the page and browser tests kept; the docs no longer describe a workaround | X_eTaL saga 25 |
| 2 | promotion-blockers | re-check the asks that make the games look like a prototype (the program-with-`l:` misclassification, `--draw`, provenance) on the latest X_eTaL; vendor fixes as they land; remove their workarounds | X_eTaL fixes |
| 3 | bench | `just bench`: each game's scripted program and a page session timed natively (and in headless Chrome), a committed baseline, and a check that fails on a slowdown above 15 % without an explicit re-baseline; the numbers sent to X_eTaL for saga 30 (`t_able`, `i_nner`) | |
| 4 | start-here | the catalog's first screen: what you are looking at, why it is an array expression, the X_eTaL that did it; the idiom table; links to the ecosystem's front door (X_eTaL's site) and the sibling repos | |
| 5 | audit-tag | a cross-repo status audit (asks, vendored commit, sibling links); a version tag of a known-compatible snapshot | the other repos |

## Saga 6 -- macros

| # | Step slug | Delivers | Waits on |
| - | --------- | -------- | -------- |
| 1 | test-macros | the scripted games' checks written with a Test/Assert macro library (`.xtlm`), so a game's expectations are stated in X_eTaL, not only in goldens | X_eTaL saga 19 |
| 2 | control-macros | Control macros where they read better than guards (the terminal loops) | X_eTaL saga 19, X_eTaL-libraries |

## Saga 7 -- the language's new features (post-launch)

| # | Step slug | Delivers | Waits on |
| - | --------- | -------- | -------- |
| 1 | screen-control | boards redrawn in place, colours, single keys (`[]A_T`, `[]C_LS`, `[]F_G`, `[]K_EY`, the `Key` enum) | X_eTaL saga 25 step 7 |
| 2 | enums-tuples-records | state as records, statuses/events/moves as enums with matching, tuples for (row, column) and a move with its score | X_eTaL saga 29 |
| 3 | type-signatures | checked `::` signatures (the `# ::` comments become code) | X_eTaL's planned signatures |
| 4 | errors | invalid input as a typed error or a `Result` | X_eTaL saga 21 |
| 5 | extensions | a native extension where it helps | X_eTaL saga 23 |
| 6 | amend-locals | amend and local functions where they shorten the games | asks filed |

## Saga 8 -- new games: opponents, puzzles and quizzes (post-launch)

| # | Step slug | Delivers |
| - | --------- | -------- |
| 0 | lights-out | a 5 by 5 0/1 board, presses as XOR with `Board`'s plus, a solver over GF(2); written in the house style (moved from saga 4) |
| 1 | connect-four | drop by column; wins by windows in four directions; heuristic and minimax players |
| 2 | mastermind | the code-breaking game; the player that keeps every consistent code (from X_eTaL's classic) |
| 3 | sudoku | candidates as a 9 x 9 x 9 tensor; singles and hidden singles by broadcasting; a candidate microscope |
| 4 | flood-it | the flooded region by iterated masks |
| 5 | fifteen | the sliding puzzle; solvability by permutation parity |
| 6 | nim | heaps, XOR strategy by binary digits |
| 7 | reversi | the eight rays, captures and flips |
| 8 | capitals | world map (fetched, not tracked: A9); capitals as a lon/lat table projected by X_eTaL; distractors chosen by distance; scored at the terminal, the map drawn by X_eTaL (`[]P_ATH`/`[]S_HOW`); clicking on the map waits until X_eTaL can take pointer input (an ask) |
| 9 | stargazer | the stargazer-poc quiz in X_eTaL: a bright-star table (fetched, A9) projected and drawn by X_eTaL; name the star from the choices at the terminal; clicking waits like capitals |
| 10 | gallery-3-release | catalog, docs, retrospective |

## Saga 9 -- larger games and deferred (post-launch)

| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | m3-survey | inspect `wrighter.space/staging/m3/` (ask the user for its source), decide whether it belongs here |
| 2 | battleship | placement masks, a hunting player with a probability map |
| 3 | gallery-4-release | catalog, docs, final retrospective |

## Cross-cutting

- Refresh the vendored X_eTaL (`just vendor`) at a saga start or when
  an ask has landed upstream; never mid-step; its own commit, goldens
  re-run.
- When an ask lands, remove its workaround in the step that refreshes
  the vendor, and mark the ask landed.
