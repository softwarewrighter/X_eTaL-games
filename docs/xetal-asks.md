# Asks for X_eTaL

Features the games need that X_eTaL does not have yet, and bugs the
games uncovered. This repo does not change X_eTaL: each ask is filed
here (and taken to `../X_eTaL`), the game uses the workaround noted
below or waits, and the workaround is removed when the ask lands in a
release this repository builds (`XETAL_COMMIT`).

Each entry: status (open, filed, landed, dropped), kind (feature or
bug), which game(s) need it, why, a minimal repro or example, and the
workaround in use.

| Status | Kind | Ask | Games | Workaround |
| ------ | ---- | --- | ----- | ---------- |
| filed | feature | A terminal for interactive programs (the request: [`xetal-terminal-request.md`](xetal-terminal-request.md)); X_eTaL is building it as its saga 25 (at 70e129d: a run waits for typed lines and resumes, D50; the pane not yet): a resumable evaluator and a shared terminal pane in Rust and Yew, on the web-sw-tos model, replacing `window.prompt`. No WASI build of `xetal` (decided upstream) | all interactive games | pages run the engine (`xetal-play`) in Yew and replay the typed history on every line |
| open | feature | `[]G_RID` of numbers (still so at f823212): draw the numbers in the cells, and let a program choose the color scale (logarithmic, or a few fixed colors) | 2048, minesweeper (counts), any numeric board | boards of numbers are printed as text; pictures are used for character and 0/1 boards |
| open | feature | A functional update (amend / "at"; still none at f823212): items of an array replaced at given positions, returning a new array | trek adventure, Star Trek (each wrote `l:u_pdate`), minesweeper, robot chase | a table of positions against indices (`l:u_pdate`) |
| open | feature | Local functions: a function defined inside a lambda (`n_ear := { t -> ... }`) and applied there | tic-tac-toe, Star Trek | top-level (`l:` or private) helpers that take the extra values as arguments |
| open | feature | Named dyadic trains (still so at f823212): `u:s_ign := [> - <]` is monadic only, so a dyadic fork must be written inline in a lambda (`{ t g -> t [< - >] g }`) | guess, robot chase | the fork written inline |
| open | feature | `p_ower` with a Bool count (still so at f823212) (a 0/1 condition), as other numeric places accept Truthy values | 2048 | `1 *` before the condition |
| landed | bug | `t_able` and `i_nner` slower since the higher-order built-ins became kernels (X_eTaL saga 30): robot chase 1.7 times slower (26 to 44 ms) between 39938f3 and 70e129d; at f823212 it is 16 ms, shut the box 4 times and 2048 1.6 times faster than at 70e129d; numbers in [`bench.md`](bench.md) | robot chase, Star Trek, tic-tac-toe | none; `just bench` (in the gate) watches for slowdowns |
| landed | bug | An executable program (shebang) that defines `l:` names is taken for a library: MC8 row 9 is never reported (landed by f823212: `error[library-name-in-program]`) | all | none: the layout check's own test was removed |

## Details

### A functional update (amend)

Games keep their state as arrays and change a few items each turn.
Without an amend, two games wrote the same helper, an update of
several items at once by a table of every position against every
index:

```
l:u_pdate := { s iv ->
  k := (t_ally iv) d_iv 2
  m := (r_ange t_ally s) '= t_able ((2 * r_ange k) - 1) s_elect iv
  v := ((t_ally s) c_at k) r_eshape (2 * r_ange k) s_elect iv
  (s * n_ot '| r_/_2 m) + '+ r_/_2 m * v
}
```

Ask: APL's amend or an "at" operator (`v 3 7 a_t s`: s with items 3
and 7 replaced by v), along an axis too. Already on X_eTaL's wish list
(priority 1, unplanned).

### Local functions

Inside a function, a helper that uses the function's locals cannot be
defined and applied: `n_ear := { t -> ... }` inside a lambda is not a
function binding there, and a plain name (`dig := { p -> ... }`) is a
variable that cannot be applied. Workaround: top-level helpers that
take the values as arguments (`b l:n_ear t`). Ask: local function
bindings in a lambda's body, closing over its parameters.

### Numbers on a `[]G_RID` board

`[]G_RID` draws a character matrix with each character in its cell
and a 0/1 matrix as dark and light cells, which suits boards of marks
(tic-tac-toe, robot chase) and of lights (lights out). A matrix of
other numbers is drawn as colors from the least to the greatest, on a
linear scale and without the numbers:

```
[]S_HOW []G_RID 4 4 r_eshape 0 2 4 8 16 32 64 128 256 512 1024 2048 0 0 2 4
```

shows a 2048 board as almost all one dark color, with no way to read
the tiles. Ask: an option (a variant or a left argument) to draw each
number in its cell, and a choice of scale (logarithmic, or a small
palette indexed by the value). Workaround: numeric boards are printed
as text, and pictures are used only for character and 0/1 boards.

### The `xetal` binary for WebAssembly (WASI): withdrawn

This repo first asked for a `wasm32-wasip1` build of `xetal-cli` so
pages could run the binary; it fails in `crossterm`, which `ratatui`
brings in. X_eTaL declined it (2026-10-02) in favor of a terminal
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
