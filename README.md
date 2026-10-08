# X_eTaL games

<p align="center">
  <b><a href="https://softwarewrighter.github.io/X_eTaL-games/">The live game catalog</a></b>
  -- every game running in your browser (WebAssembly)
</p>

Small games written in
[X_eTaL](https://github.com/softwarewrighter/X_eTaL), the
eXperimental Extensible Typed Array Language: board games, puzzles,
simulations and quizzes whose rules are array-shaped, each playable on
the command line and (soon) live in the browser.

## What this is

Every game here teaches one array-programming lesson and happens to be
playable. A horse race is one vector of positions advanced by one
expression; robots chasing you all move at once because the move is a
single matrix operation; a Minesweeper count is eight rotations of the
mine board added together; a 2048 move is compress, merge, pad. The
game's rules live in X_eTaL, and a game's page shows only what X_eTaL
prints or draws: the game running in a terminal, its scripted program
as a notebook, and the source. Nothing on a page is a user interface
written in another language.

It is a sibling of
[X_eTaL-demos](https://github.com/softwarewrighter/X_eTaL-demos) and
follows the same layout: every game is its own sub-project under
`games/<name>/`, runs against a vendored, known-good copy of X_eTaL,
and is tested by its expected output.

## Games

| Game | The lesson | Status |
| ---- | ---------- | ------ |
| [Horse race](games/horse-race/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-games/horse-race/)) | vectors, random rolls, reduction | live |
| [Guess the number](games/guess/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-games/guess/)) | comparison tables, masks, every game at once (from COR24 BASIC) | live |
| [Robot chase](games/robot-chase/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-games/robot-chase/)) | coordinate matrices, simultaneous motion, collision tables (from COR24 BASIC) | live |
| [Trek adventure](games/trek-adventure/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-games/trek-adventure/)) | tables, masks, a state machine as data (from COR24 BASIC) | live |
| [Star Trek](games/trek/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-games/trek/)) | planes of a galaxy, paths as matrices, windows, masked updates (from COR24 BASIC) | live |
| [Tic-tac-toe](games/tic-tac-toe/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-games/tic-tac-toe/)) | a 3 by 3 board, line extraction by indexing, minimax | live |
| [Shut the box](games/shut-the-box/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-games/shut-the-box/)) | boolean masks, every subset at once, subset sums | live |
| [Minesweeper](games/minesweeper/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-games/minesweeper/)) | neighborhoods by rotation, flood fill to a fixed point | live |
| [2048](games/2048/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-games/2048/)) | compress, merge, pad: composition; turning the board | live |
| Lights out | boolean matrices, XOR, GF(2) | planned |
| [Sudoku](games/sudoku/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-games/sudoku/)) | a candidate tensor; every single at once by sums over units; rounds to a fixed point | live |
| Connect four, Mastermind, Flood-it, Fifteen, Nim, Reversi | windows, histograms, regions, permutations, binary digits, rays | planned |
| [Capitals](games/capitals/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-games/capitals/)) | a map you click: every distance at once, nearest by grade; the map drawn as SVG text; clicks as events; data from TOML | live |
| [Stargazer](games/stargazer/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-games/stargazer/)) | a sky you click: brightness bands as a table; the sky drawn as SVG text; clicks as events; data from TOML | live |
| Battleship | placement masks, a probability map | planned |

A game's name links to its own page (`games/<name>/README.md`) once it
exists.

What a game needs from X_eTaL that it does not have yet is listed in
[`docs/xetal-asks.md`](docs/xetal-asks.md).

## Array idioms, one per game

Different games, the same few array ideas. Each game's README shows
its idiom in the code.

| The array idiom | Where it carries a game |
| --------------- | ----------------------- |
| a vector of racers moved by one expression; the leaders by a fork | [Horse race](games/horse-race/README.md) |
| every candidate against every guess by one table; all games played at once | [Guess the number](games/guess/README.md) |
| a coordinate matrix moving at once (the sign as a fork); collisions as an inner product | [Robot chase](games/robot-chase/README.md) |
| text as a table of rows picked by masks; refusals as data | [Trek adventure](games/trek-adventure/README.md) |
| a galaxy of planes; paths as tables of steps; windows by selection | [Star Trek](games/trek/README.md) |
| lines by indexing; ratings as one inner product; minimax as negamax | [Tic-tac-toe](games/tic-tac-toe/README.md) |
| masks over all 512 subsets at once; sums by one inner product | [Shut the box](games/shut-the-box/README.md) |
| neighborhoods by rotate and reduce; flood fill to a fixed point | [Minesweeper](games/minesweeper/README.md) |
| compress, merge, compress as a train; turning the board by powers of 0 or 1 | [2048](games/2048/README.md) |
| every cell's candidates as one array; every unit counted at once by selecting with a table; rounds to a fixed point | [Sudoku](games/sudoku/README.md) |
| every distance at once, nearest by grade; a map, its menus and score written as SVG text; clicks as events (`[]E_VENT`) | [Capitals](games/capitals/README.md) |
| every star's brightness band by one table; 2887 stars placed at once; the same picture-and-click pieces (`lib/Svg.xtl`) | [Stargazer](games/stargazer/README.md) |

## Build

Prerequisites: [Rust](https://rustup.rs) (stable) and
[`just`](https://github.com/casey/just); for the browser games,
`rustup target add wasm32-unknown-unknown` and
[trunk](https://trunkrs.dev); for the gate (maintainers),
`sw-markdown-checker`.

```bash
just                                 # list the tasks
just xetal                           # build the bundled X_eTaL interpreter
just eval "'+ r_/_2 2 3 r_eshape r_ange 6"   # try it: row sums, 6 15
just gate                            # the pre-commit gate
```

The games run a known-good X_eTaL, recorded as one commit in
`XETAL_COMMIT`: `just xetal` clones X_eTaL into `work/xetal` (not
tracked), checks that commit out, builds it and links the binary as
`bin/xetal`, which every recipe uses (never a `xetal` on your `PATH`),
so the games do not change under you as X_eTaL develops. `just
xetal-version` shows the commit. Maintainers move to a newer X_eTaL
with `just xetal-bump [REF]` (default the latest on GitHub), run
`just gate`, and commit `XETAL_COMMIT`.

## Playing and adding games

```bash
just games                           # the games, in catalog order
just run SLUG                        # run a game's program (a scripted game)
just play SLUG                       # play it at the terminal (play.xtl)
just show SLUG                       # the program as a notebook: each statement, then its output
just repl SLUG                       # the X_eTaL REPL in the game's directory; "h:" u_se< "HorseRace" loads its rules
just test-game SLUG                  # check its output against expected/
just new-game nim "Nim"              # start a new game from games/_template
just bless SLUG                      # rewrite its expected output (review the diff)
just size [SLUG]                     # lines and tokens of X_eTaL per game
just bench                           # time every game; fails if one got more than 15 % slower
just fetch [SLUG]                    # download third-party assets (never committed)
```

Each game is a sub-project, `games/<slug>/`:

| File | What it is |
| ---- | ---------- |
| `game.toml` | title, one-line summary, the lesson (its array idiom), concepts, status (draft, live, deferred), catalog order, sources, the X_eTaL asks it needs, and its `wikipedia` link or, without an article, `about` (a short history and how to play) |
| `README.md` | the game's own page: how to play, the program, how it works |
| `<Name>.xtl` | the rules, a library (`l:n_ew`, `l:m_ove`, ...) that the scripted game, the terminal game and the page all import |
| `<slug>.xtl` | a scripted game using the rules; run by the tests (seed 1) |
| `play.xtl` | the game at the terminal (it reads moves with `[]R_EAD`), when it has one |
| `expected/` | each program's expected output (`NAME.out`, `NAME.err` when it should fail) and the moves typed into it (`NAME.in`) |
| `assets/fetch.sh` | downloads third-party files into the git-ignored `assets/cache/`, when the game needs any |
| `web/` | its browser app (a Cargo workspace), when it has one |
| `test.sh` | any further tests, when it has them |

Helpers several games need are shared X_eTaL libraries in
[`lib/`](lib/README.md) (`Play`, `Text`, `Board`, `State`), found
through `XETAL_PATH`.

The rules are written once, as a library with a small protocol
(`l:n_ew`, `l:m_ove`, `l:l_egal`, `l:s_tatus`, `l:v_iew`), so a page
needs to know little about each game: it turns a click into a move, runs the game's X_eTaL
and draws the arrays that come back.

Every game's page is the same shared page, `shared/microscope/`
(`page::GamePage`): the game's `play.xtl` in a terminal (what you type
is its keyboard), its scripted program as a notebook, and the sources,
all run by the X_eTaL engine compiled to WebAssembly. Its
[README](shared/microscope/README.md) walks through adding a game's
web app. `just browser-test` plays every built page in headless Chrome
and checks that the terminal and the notebook show exactly the
command line's expected output.

## The live site

```bash
just serve SLUG       # one game's web app at http://127.0.0.1:8473/, rebuilt on change
just pages            # build the whole site into pages/
just serve-pages      # preview pages/ at http://127.0.0.1:8473/X_eTaL-games/
just publish          # publish pages/ as the gh-pages branch (the live site)
just verify-live      # play every game on the live site in headless Chrome
```

Port 8473 is this repository's own (each X_eTaL repository has its
own port, so one demo from each can run at the same time); the
automated browser test and the screenshots pick free ports instead.

The site is built locally: `just pages` builds every game that has a
web app into `pages/<slug>/` and writes the catalog, `pages/index.html`,
from the games' `game.toml` files (the gate does this too, then plays
every page in headless Chrome). `pages/` is not tracked: `just publish`
pushes it to the `gh-pages` branch as that branch's only commit,
replaced on every publish, and GitHub Pages serves it at
<https://softwarewrighter.github.io/X_eTaL-games/>.

## Status

Nine games are live, each page showing only X_eTaL's own output, and
all nine are written in idiomatic X_eTaL on shared libraries (`lib/`):
trains, tables and inner products over branches, named parts, checked
type comments. Next: the launch work (X_eTaL's terminal pane in every
page, a performance baseline, a Start Here page); new games come after
that. See [`docs/plan.md`](docs/plan.md).

## Documentation

- [`docs/plan.md`](docs/plan.md) -- architecture decisions, the
  gallery, the roadmap
- [The cross-reference](https://softwarewrighter.github.io/X_eTaL-games/doc/) -- every program and library, documented and linked (`xetal doc`; `just pages` builds it into `pages/doc/`)
- [`docs/audit.md`](docs/audit.md) -- the state at the v0.1.0 tag
- [`docs/style.md`](docs/style.md) -- how the games' X_eTaL is written
- [`docs/xetal-asks.md`](docs/xetal-asks.md) -- features and fixes the
  games need from X_eTaL
- [`CHANGES.md`](CHANGES.md) -- every change, by day
- `docs/research.txt` -- the archival list of game ideas
- [`CLAUDE.md`](CLAUDE.md) (also `AGENTS.md`) -- the agent workflow
  (agentrail sagas) and rules

## Development

Development is tracked with agentrail sagas, as in X_eTaL: `agentrail
status` shows the current step, `agentrail next` its instructions.
Every step ends with the gate passing, docs updated, a commit to `main`
and a push.

## Related Projects

- [X_eTaL](https://github.com/softwarewrighter/X_eTaL) -- the language
  ([try it live](https://softwarewrighter.github.io/X_eTaL/))
- [X_eTaL-demos](https://github.com/softwarewrighter/X_eTaL-demos) --
  visual demos in X_eTaL
  ([live](https://softwarewrighter.github.io/X_eTaL-demos/))
- [X_eTaL-ML](https://github.com/softwarewrighter/X_eTaL-ML) -- machine
  learning in X_eTaL
  ([live](https://softwarewrighter.github.io/X_eTaL-ML/))
- [X_eTaL-libraries](https://github.com/softwarewrighter/X_eTaL-libraries)
  -- libraries that extend the vocabulary
  ([live](https://softwarewrighter.github.io/X_eTaL-libraries/))
- [X_eTaL-extensions](https://github.com/softwarewrighter/X_eTaL-extensions)
  -- native extensions that extend the machine
  ([live](https://softwarewrighter.github.io/X_eTaL-extensions/))

## Links

- Blog: [Software Wrighter Lab](https://software-wrighter-lab.github.io/)
- Discord: [Join the community](https://discord.com/invite/Ctzk5uHggZ)
- YouTube: [Software Wrighter](https://www.youtube.com/@SoftwareWrighter)

## Copyright

Copyright (c) 2026 Michael A Wright

## License

MIT. See [`LICENSE`](LICENSE) and [`COPYRIGHT`](COPYRIGHT).
