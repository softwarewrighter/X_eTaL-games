# X_eTaL games

Small games written in
[X_eTaL](https://github.com/softwarewrighter/X_eTaL), the
eXperimental eXtensible Typed Array Language: board games, puzzles,
simulations and quizzes whose rules are array-shaped, each playable on
the command line and (soon) live in the browser.

## What this is

Every game here teaches one array-programming lesson and happens to be
playable. A horse race is one vector of positions advanced by one
expression; robots chasing you all move at once because the move is a
single matrix operation; a Minesweeper count is eight rotations of the
mine board added together; a 2048 move is compress, merge, pad. The
game's rules live in X_eTaL; the page only turns a click into a move
and draws the arrays that come back.

It is a sibling of
[X_eTaL-demos](https://github.com/softwarewrighter/X_eTaL-demos) and
follows the same layout: every game is its own sub-project under
`games/<name>/`, runs against a vendored, known-good copy of X_eTaL,
and is tested by its expected output.

## Games

| Game | The lesson | Status |
| ---- | ---------- | ------ |
| Horse race | vectors, random selection, reduction | planned |
| Tic-tac-toe | a 3 x 3 board, line extraction, minimax | planned |
| Robot chase | coordinate matrices, simultaneous motion, collisions | planned |
| Shut the box | boolean masks, subset sums | planned |
| Minesweeper | neighbourhoods by rotation, flood fill | planned |
| 2048 | compress, merge, pad: composition | planned |
| Lights out | boolean matrices, XOR, GF(2) | planned |
| Connect four, Mastermind, Sudoku, Flood-it, Fifteen, Nim, Reversi | windows, histograms, candidate tensors, regions, permutations, binary digits, rays | planned |
| Capitals, Stargazer | map and sky quizzes: projections, distances, scoring | planned |
| Battleship, Trek, Trek adventure, Guess | larger and retro games | planned |

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

## Status

Starting. The project process, plan and gate are in place, and the
bundled X_eTaL builds and is checked by the gate (its command-line
interpreter, and its library natively and for WebAssembly). Next: the
game layout and its tests, the live site, and the first game (the
horse race). See [`docs/plan.md`](docs/plan.md).

## Documentation

- [`docs/plan.md`](docs/plan.md) -- architecture decisions, the
  gallery, the roadmap
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
