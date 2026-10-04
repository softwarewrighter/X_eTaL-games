# Changes

Every commit, newest first, grouped by day. Times are Pacific
(UTC-07:00), as committed.

Categories: `feat` new capability, `fix` a bug or wrong behavior,
`refactor` structure without behavior change, `test` tests only,
`build` build and tooling, `game` a game or a change to one, `docs`
documentation, `plan` saga planning and reordering, `release`
milestone release, `chore` agentrail bookkeeping (step complete, saga
archive), `vendor` a refresh of the vendored X_eTaL.

## 2026-10-04

- 15:33 `test` `just bench` (in the gate): every game's scripted, terminal and page times against a committed baseline, failing above 15 % slower; docs/bench.md, with the games timed before and after X_eTaL's table/inner-product slowdown (robot chase 1.7 times slower) for X_eTaL saga 30.

- 15:06 `fix` The browser test waits for each page to load and retries an evaluation lost to a page reload (an intermittent failure in the gate run of the port change, which was committed despite it; the gate passes on the same content).

- 14:58 `build` This repository's local port is 8473 (`just serve`, `just serve-pages`), unique among the X_eTaL repositories so a demo from each can run at once; automated tests keep picking free ports.

- 14:52 `fix` The browser test and the screenshots pick free ports: a fixed port collided with a sibling repository's server, so the gate failed against the wrong site (and the about step was pushed on that failed run; its content is verified by this run).

- 13:56 `chore` Saga step about completed.

- 13:52 `feat` Game titles, on each page and in the catalog, link to the game's Wikipedia article (a page glyph, a new tab) or open a dialog on its history and play (closed by Escape, a click outside, or its X); wikipedia or about in game.toml; tested in headless Chrome.

- 11:46 `chore` Saga idioms archived; saga launch started (about, bench, start-here, terminal-pane, promotion-blockers, audit-tag).

- 11:42 `chore` Saga step idioms-release completed.

- 11:41 `release` Idioms: all nine games in idiomatic X_eTaL on lib/ with outputs unchanged; the "one array idea per game" table in the README and the catalog (from each game's lesson); the style guide's idioms; asks for named dyadic trains and Bool power counts; saga 4's retrospective.

- 09:46 `chore` Saga step 2048-idioms completed.

- 09:45 `refactor` 2048 in idiomatic X_eTaL: the move left as a train of atops, turning the board as transpose and reverse to a power of 0 or 1, the status as the first condition, Play; outputs unchanged; 101/633 to 89/641.

## 2026-10-03

- 21:47 `chore` Saga step minesweeper-idioms completed.

- 21:45 `refactor` Minesweeper in idiomatic X_eTaL: neighbour counts, squares and spacing from Board, input by Play; outputs unchanged; 94/774 to 88/688.
- 21:45 `feat` lib/Play: l:l_ine (the line typed) and l:n_umbersIn (the numbers in a line); l:n_umbers and l:l_etter built on them.

- 19:54 `chore` Saga step shut-the-box-idioms completed.

- 19:52 `refactor` Shut the box in idiomatic X_eTaL: the moves as a mapped strand printed by Text, the score as an inner product, the status as the first condition, Board and Play; outputs unchanged; 77/498 to 64/473.

- 19:14 `chore` Saga step tic-tac-toe-idioms completed.

- 19:12 `refactor` Tic-tac-toe in idiomatic X_eTaL: the quick player's rating as one inner product (four features, four weights), minimax as negamax, the status as the first condition that holds, one code board for text and picture, Board and Play; outputs unchanged; 92/667 to 85/632.

- 18:36 `chore` Saga step trek-idioms completed.

- 18:28 `refactor` Star Trek in idiomatic X_eTaL: Board, State, Text and Play from lib/, the state's positions named; outputs unchanged; 330/2542 to 317/2402.
- 17:15 `chore` Saga step trek-adventure-idioms completed.

- 17:14 `refactor` Trek adventure in idiomatic X_eTaL: the text table read by Text, updates by State, named state positions, every action's refusals as data (failed checks pick the message), Play; outputs unchanged.

- 15:55 `chore` Saga step robot-chase-idioms completed.

- 15:54 `refactor` Robot chase in idiomatic X_eTaL: Board for squares and spacing, the sign as a fork, robots per square as one inner product, one code board for text and picture, Play; outputs unchanged; 167/1410 to 149/1192 lines/tokens.
- 15:54 `fix` lib/Board's l:r_c gives rows over columns (a 2-row matrix), as documented.

- 14:57 `chore` Saga step guess-idioms completed.

- 14:55 `refactor` Guess the number in idiomatic X_eTaL: the answer as a dyadic fork `t [< - >] g`, the range update by a max and a min, Play for input, type comments; outputs unchanged.

- 13:59 `plan` Reprioritized after the ecosystem review (X_eTaL docs/research4.txt): no new games before the launch; idioms first (ending with an idiom -> game table), then a launch saga (X_eTaL's terminal pane in every page, promotion-blocker checks, a performance baseline and regression check, Start Here, an audit and a tag), then macros; enums/tuples/records/signatures/errors/extensions and new games after the launch. Lights out moves to the post-launch games.

- 13:58 `chore` Saga step horse-race-idioms completed.

- 13:56 `refactor` Horse race in idiomatic X_eTaL: the leaders and winners as trains, a simpler track table, Play for input, type comments on every export; outputs unchanged.
- 13:56 `test` scripts/check-types.py (in the gate): a library's type comments must cover every export and equal the inferred types.

- 13:34 `chore` Saga step shared-libraries completed.

- 13:31 `feat` Shared X_eTaL libraries in lib/ (Play, Text, Board, State) with their own tests in the gate; XETAL_PATH set by the scripts; game pages can load and show shared libraries.

- 12:44 `chore` Saga step metrics-style completed.

- 12:44 `docs` docs/style.md, the house style for the games' X_eTaL (arrays first, trains, shared libraries, type comments, right-to-left traps, what waits on X_eTaL); `just size` measures each game (lines and tokens); the sizes before the idioms saga are in the plan.

- 12:40 `chore` Saga grids archived; saga idioms started (13 steps).

- 12:02 `chore` Saga step gallery-2-release completed.

- 12:01 `release` Gallery 2: nine games live; saga 3 closed early (lights out moves into saga 4, the terminal pane into saga 5); its retrospective.

- 11:50 `plan` Two refactoring sagas after the user's review ("ported" code, a narrow subset of the language): saga 4 rewrites every game in idiomatic X_eTaL with what exists now (metrics and a style guide, shared libraries Play/Board/Text/State, trains, tables over branches), saga 5 adopts X_eTaL's coming features as they land (terminal pane and screen control, macros, enums/tuples/records, signatures, errors, extensions). Asks filed: amend, local functions.

- 10:09 `chore` Saga step 2048 completed.

- 10:07 `game` 2048, live: every row slid and merged at once (running counts, a table, places in runs by a max-scan), the moves by turning the board; the shared page.
- 10:07 `fix` Status tests written `f s = 1 ?` read `f (s = 1)`: corrected in minesweeper and robot chase (right before only by coincidence; outputs unchanged).

- 09:55 `chore` Saga step minesweeper completed.

- 09:52 `game` Minesweeper, live: every square's count at once by rotations (as Life), opening by a flood fill grown to a fixed point, the field also as a picture X_eTaL draws; the shared page.

- 09:05 `chore` Saga step x-pictures completed.

- 09:04 `feat` Boards as pictures X_eTaL draws: tic-tac-toe and robot chase show `[]S_HOW []G_RID` of their boards each turn (the page scales them up; the runner ignores the CLI's "drawn PATH" lines); tests check they appear; ask filed for numbers in `[]G_RID` cells and a colour scale.

- 08:55 `chore` Saga step shut-the-box completed.

- 08:51 `game` Shut the box, live: every subset of the tiles at once (a 9 by 512 bit matrix, all sums by one inner product, the moves by one mask); the terminal game lists the moves; the shared page.

- 08:32 `chore` Saga step tic-tac-toe completed.

- 08:29 `game` Tic-tac-toe, live: the lines as a table of square numbers (every line's sum by one selection and one reduce), a quick player rating every square at once, minimax by recursion; you are X at the terminal; the shared page.

- 07:51 `chore` Saga step vendor-refresh completed.

- 07:50 `vendor` X_eTaL 70e129d vendored (transpose `o_\`, the resumable run of X_eTaL saga 25, kernels for higher-order built-ins); every golden, page test and browser test passes unchanged; pages rebuilt on it.

- 07:16 `chore` Saga basic archived (terminal-pane carried over); saga grids started.

- 07:14 `chore` Saga step terminal-shell completed.

- 07:14 `plan` Step terminal-shell closed: its work (the text games' shared page) was done by accurate-pages; only terminal-pane, waiting on X_eTaL, remains in saga 2.

- 07:11 `chore` Saga step gallery-1-release completed.

- 07:07 `release` Gallery 1: the horse race and the four COR24 BASIC games live; the catalog's intro and summaries describe the accurate pages; every card has its screenshot; saga 2's retrospective; saga 3's plan revised for boards drawn by X_eTaL.

- 06:24 `chore` Saga step trek completed.

- 06:15 `game` Star Trek, live: the galaxy as three 8 by 8 planes made at once, a quadrant entered by a random order and its inverse, courses and torpedo tracks as whole paths, phasers as one masked subtraction, the long-range scan as a window cut by one selection; terminal game, scripted game, the shared page; a destroyed Klingon leaves the galaxy too.

## 2026-10-02

- 19:55 `chore` Saga step trek-adventure completed.

- 19:53 `game` Trek adventure, live: "Star Trek: Decaying Orbit" as tables (157 lines of text with a key and a condition each, a room picked by one mask, exits a 9 by 9 table, the state one vector changed by a table-driven update); terminal game with a full winning session as its golden; the walkthrough played into a table of states; the shared page.

- 19:19 `docs` Ports are one-time rewrites: the links to and quotes of the BASIC originals are gone from the game READMEs.

- 19:17 `docs` The BASIC originals are no longer copied into games/*/original/: each README links the .bas file in web-sw-cor24-basic, pinned to the commit ported from.

- 19:02 `chore` Saga step robot-chase completed.

- 18:57 `game` Robot chase, live: the COR24 BASIC game with every robot moving at once (a 2 by 12 matrix plus the sign of the difference), crashes by a table of every robot against every other, the scan by summing a 4 by 4 by 4 by 4 array; terminal game, scripted game with a made-up crash, the shared page; tested natively and in headless Chrome.
- 18:57 `fix` Empty printed lines keep their height in the page's terminal and notebook.

- 17:32 `chore` Saga step accurate-pages completed.

- 17:31 `fix` Pages show only what X_eTaL prints or draws: one shared game page (the game's play.xtl in a terminal, its scripted program as a notebook, the sources); the Rust-drawn tracks, highlights, chips and candidate grid are gone. The horse race's own view now marks the winners.
- 17:31 `test` `just browser-test`: headless Chrome types each game's play.in into its built page and requires the terminal and notebook to match the goldens; in the gate.
- 17:31 `plan` No WASI build of `xetal` (X_eTaL's decision): the request is updated, the asks entry is filed as X_eTaL's saga 25 (a resumable evaluator and a Rust/Yew terminal pane), and the step waiting on it is terminal-pane.

- 15:23 `docs` A request to X_eTaL for a terminal: a `[]TE` interface, a sw-tos-style browser terminal replacing `window.prompt` for input, and a cargo feature so the `xetal` binary builds for wasm32-wasip1 (docs/xetal-terminal-request.md).

- 15:06 `docs` Ask filed: the `xetal` CLI cannot be built for wasm32-wasip1 (ratatui/crossterm via the editor and line editor); pages keep the engine library until a feature lets the real binary run in the browser.
- 15:06 `build` `just repl SLUG`: the X_eTaL REPL in a game's directory, its rules library importable.

- 15:02 `fix` The logo replaced by the corrected one ("eXperimental Extensible Typed Array Language"), in images/ and the published pages.

- 13:57 `chore` Saga step guess completed.

- 13:55 `game` Guess the number, live: NumberGuess.xtl (the answer rule item by item; the secrets still possible, every candidate against every guess by one table; the halving player winning all 100 games at once), play.xtl as the COR24 BASIC game plays (tested with typed guesses), the page running play.xtl unmodified in a terminal beside the hundred candidates.
- 13:55 `feat` Page shell: `terminal::session` runs a terminal program with everything typed as its keyboard and returns the transcript, typed lines echoed where they are read.
- 13:55 `fix` Pages show the program they run (with its `u_se<`) and the library separately, so `l:` names appear only in the library; stage chips quote the program's calls.
- 13:55 `test` The layout check rejects a program defining `l:` names, a library without exports, and a library named like its game (they collide on a case-insensitive disk); self-tested; ask filed for X_eTaL's own check.

- 12:32 `chore` Saga foundation archived; saga basic (the COR24 BASIC games) started.

- 12:30 `chore` Saga step horse-race completed; saga foundation done.

- 12:29 `game` Horse race, live: the rules as a library (HorseRace.xtl: a round is `p + r_oll! 5 r_eshape 3`, the winners `w_here p = 'm_ax r_/ p`, the track a character matrix), a scripted race, a terminal betting game (tested with typed moves), and the page (the track X_eTaL draws, each round's rolls, the stages highlighted in the rules); README beside the APL original.
- 12:29 `feat` Page shell: `library` puts a game's rules library in the engine's in-memory store so pages `u_se<` it; `output_seeded` gives each turn new rolls.
- 12:29 `build` A green favicon (the X_eTaL-demos icon, recoloured).
- 12:29 `docs` The expansion is "eXperimental Extensible Typed Array Language" (one capital X).
- 12:29 `plan` Rules as one library per game (A4 revised); the COR24 BASIC games (guess, robot chase, trek adventure, trek) are saga 2, before the grid games.

- 09:06 `chore` Saga step pages-pipeline completed.

- 09:04 `build` Live site: shared/microscope (the page shell, copied from X_eTaL-demos) tested in the gate; the catalog (pages/index.html, cards with each game's lesson) from game.toml; `just pages`, `serve-pages`, `serve`, `screenshots`; the upload-only Pages workflow.

- 08:53 `chore` Saga step game-layout completed.

- 08:53 `build` Game layout: games/_template (game.toml, README, the protocol skeleton), scripts/games.py, scripts/test-games.sh (goldens with expected/NAME.in as standard input, web/ tests, test.sh), its self-test in the gate, new-game, run-game, fetch-assets; recipes games, new-game, run, play, show, test, test-game, bless, fetch.

- 07:16 `chore` Saga step vendor-xetal completed.

- 07:09 `build` Vendoring: `just vendor [REF]` snapshots a committed ref of ../X_eTaL into vendor/xetal/ (VENDORED records it); `just xetal`, `xetal-version`, `eval`, `check-vendor`; tools/vendor-probe proves xetal-play works natively and for wasm32; the CLI reports the vendored commit; all in the gate.
- 07:09 `vendor` X_eTaL 39938f3 vendored.

- 06:57 `chore` Saga step scaffold completed.

- 06:57 `plan` Scaffold: the agentrail process (saga foundation), CLAUDE.md/AGENTS.md, README, COPYRIGHT, LICENSE, CHANGES.md, justfile, the gate, docs/plan.md (architecture, the gallery of 20 games, four sagas), docs/xetal-asks.md.

- 06:01 `chore` First commit: an empty README.
