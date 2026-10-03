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
| [Minesweeper](games/minesweeper/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-games/minesweeper/)) | neighbourhoods by rotation, flood fill to a fixed point | live |
| [2048](games/2048/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-games/2048/)) | compress, merge, pad: composition; turning the board | live |
| Lights out | boolean matrices, XOR, GF(2) | planned |
| Connect four, Mastermind, Sudoku, Flood-it, Fifteen, Nim, Reversi | windows, histograms, candidate tensors, regions, permutations, binary digits, rays | planned |
| Capitals, Stargazer | map and sky quizzes: projections, distances, scoring | planned |
| Battleship | placement masks, a probability map | planned |

A game's name links to its own page (`games/<name>/README.md`) once it
exists.

What a game needs from X_eTaL that it does not have yet is listed in
[`docs/xetal-asks.md`](docs/xetal-asks.md).

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

The games run a copy of X_eTaL kept in this repository under
`vendor/xetal/` (a snapshot of a known-good commit, recorded in
`vendor/xetal/VENDORED`), so they do not change under you as X_eTaL
develops. `just xetal-version` shows which commit it is. Maintainers
refresh it from a sibling checkout with `just vendor` (the latest
commit of `../X_eTaL`) or `just vendor REF`; only committed X_eTaL
work is ever copied, and the refresh is committed on its own after
`just gate` passes.

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
just fetch [SLUG]                    # download third-party assets (never committed)
```

Each game is a sub-project, `games/<slug>/`:

| File | What it is |
| ---- | ---------- |
| `game.toml` | title, one-line summary, the lesson, concepts, status (draft, live, deferred), catalog order, sources, the X_eTaL asks it needs |
| `README.md` | the game's own page: how to play, the program, how it works |
| `<Name>.xtl` | the rules, a library (`l:n_ew`, `l:m_ove`, ...) that the scripted game, the terminal game and the page all import |
| `<slug>.xtl` | a scripted game using the rules; run by the tests (seed 1) |
| `play.xtl` | the game at the terminal (it reads moves with `[]R_EAD`), when it has one |
| `expected/` | each program's expected output (`NAME.out`, `NAME.err` when it should fail) and the moves typed into it (`NAME.in`) |
| `assets/fetch.sh` | downloads third-party files into the git-ignored `assets/cache/`, when the game needs any |
| `web/` | its browser app (a Cargo workspace), when it has one |
| `test.sh` | any further tests, when it has them |

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
just serve SLUG       # one game's web app at http://127.0.0.1:8095/, rebuilt on change
just pages            # build the whole site into pages/
just serve-pages      # preview pages/ at http://127.0.0.1:8096/X_eTaL-games/
```

The site is built locally: `just pages` fetches any third-party
assets, builds every game that has a web app into `pages/<slug>/` and
writes the catalog, `pages/index.html`, from the games' `game.toml`
files. `pages/` is committed, and pushing it to `main` runs a GitHub
Actions workflow (`.github/workflows/pages.yml`) that only publishes
the folder, at <https://softwarewrighter.github.io/X_eTaL-games/>.

## Status

Early. The project process, plan and build scaffolding are in place,
the bundled X_eTaL builds and is checked by the gate, the game layout
and its test runner (with scripted terminal input) and the live catalog
are in place; the horse race and the four COR24 BASIC games (guess
the number, robot chase, trek adventure, Star Trek) are live, each
page showing only X_eTaL's own output. Next: the grid games; the browser
terminal moves to X_eTaL's own terminal pane when it lands. See [`docs/plan.md`](docs/plan.md).

## Documentation

- [`docs/plan.md`](docs/plan.md) -- architecture decisions, the
  gallery, the roadmap
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

## Links

- Blog: [Software Wrighter Lab](https://software-wrighter-lab.github.io/)
- Discord: [Join the community](https://discord.com/invite/Ctzk5uHggZ)
- YouTube: [Software Wrighter](https://www.youtube.com/@SoftwareWrighter)

## Copyright

Copyright (c) 2026 Michael A Wright

## License

MIT. See [`LICENSE`](LICENSE) and [`COPYRIGHT`](COPYRIGHT).
