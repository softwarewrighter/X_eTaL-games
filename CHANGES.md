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
