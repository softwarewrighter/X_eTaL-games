# Asks for X_eTaL

Features the games need that X_eTaL does not have yet, and bugs the
games uncovered. This repo does not change X_eTaL: each ask is filed
here (and taken to `../X_eTaL`), the game uses the workaround noted
below or waits, and the workaround is removed when the ask lands in a
vendored release (`vendor/xetal/VENDORED`).

Each entry: status (open, filed, landed, dropped), kind (feature or
bug), which game(s) need it, why, a minimal repro or example, and the
workaround in use.

| Status | Kind | Ask | Games | Workaround |
| ------ | ---- | --- | ----- | ---------- |
| filed | feature | A terminal for interactive programs (the request: [`xetal-terminal-request.md`](xetal-terminal-request.md)); X_eTaL is building it as its saga 25 (at 70e129d: a run waits for typed lines and resumes, D50; the pane not yet): a resumable evaluator and a shared terminal pane in Rust and Yew, on the web-sw-tos model, replacing `window.prompt`. No WASI build of `xetal` (decided upstream) | all interactive games | pages run the engine (`xetal-play`) in Yew and replay the typed history on every line |
| open | feature | `[]G_RID` of numbers: draw the numbers in the cells, and let a program choose the colour scale (logarithmic, or a few fixed colours) | 2048, minesweeper (counts), any numeric board | boards of numbers are printed as text; pictures are used for character and 0/1 boards |
| open | bug | An executable program (shebang) that defines `l:` names is taken for a library: MC8 row 9 is never reported (still so at 70e129d) | all | `scripts/games.py check` rejects `l:` definitions in programs |

## Details

### Numbers on a `[]G_RID` board

`[]G_RID` draws a character matrix with each character in its cell
and a 0/1 matrix as dark and light cells, which suits boards of marks
(tic-tac-toe, robot chase) and of lights (lights out). A matrix of
other numbers is drawn as colours from the least to the greatest, on a
linear scale and without the numbers:

```
[]S_HOW []G_RID 4 4 r_eshape 0 2 4 8 16 32 64 128 256 512 1024 2048 0 0 2 4
```

shows a 2048 board as almost all one dark colour, with no way to read
the tiles. Ask: an option (a variant or a left argument) to draw each
number in its cell, and a choice of scale (logarithmic, or a small
palette indexed by the value). Workaround: numeric boards are printed
as text, and pictures are used only for character and 0/1 boards.

### The `xetal` binary for WebAssembly (WASI): withdrawn

This repo first asked for a `wasm32-wasip1` build of `xetal-cli` so
pages could run the binary; it fails in `crossterm`, which `ratatui`
brings in. X_eTaL declined it (2026-10-02) in favour of a terminal
pane in Rust and Yew over a resumable evaluator (its saga 25, from the
terminal request), and pages keep the engine library, `xetal-play`.

### A program defining `l:` names

X_eTaL decides that a file is a library when it defines `l:` names.
So a program (a file with a `#!/usr/bin/env xetal` shebang, run with
`xetal run`) that defines one is treated as a library, and the error
MC8 row 9 ("a program defines `l:` names") never appears: with an
expression the message is the library's ("a library holds definitions
only"), and with only definitions it runs, printing types, exit 0.

```
$ printf '#!/usr/bin/env xetal\nl:finish := 15\nl:finish + 1\n' > app.xtl
$ xetal run app.xtl
error[expression-in-library]: a library holds definitions only; ...
$ printf '#!/usr/bin/env xetal\nl:finish := 15\n' > app.xtl
$ xetal run app.xtl      # exit 0
l:finish : Int
```

Ask: treat a file starting with a shebang as a program and report row
9 for its `l:` definitions. Workaround: this repo's layout check
(`scripts/games.py check`, in the gate) rejects them.

Asks already filed by X_eTaL-demos
(`../X_eTaL-demos/docs/xetal-asks.md`) that a game also hits are
copied here with the game named, so this list stands on its own.
