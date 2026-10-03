# Changes

Every commit, newest first, grouped by day. Times are Pacific
(UTC-07:00), as committed.

Categories: `feat` new capability, `fix` a bug or wrong behavior,
`refactor` structure without behavior change, `test` tests only,
`build` build and tooling, `game` a game or a change to one, `docs`
documentation, `plan` saga planning and reordering, `release`
milestone release, `chore` agentrail bookkeeping (step complete, saga
archive), `vendor` a refresh of the vendored X_eTaL.

## 2026-10-02

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
