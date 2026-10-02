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
| open | feature | Build the `xetal` CLI for `wasm32-wasip1` (a cargo feature to leave out the terminal editor and line editor) | all (pages run the real binary) | blocked: pages still use the engine library (`xetal-play`) compiled into each page |
| open | bug | An executable program (shebang) that defines `l:` names is taken for a library: MC8 row 9 is never reported | all | `scripts/games.py check` rejects `l:` definitions in programs |

## Details

### The `xetal` binary for WebAssembly (WASI)

The game pages should run the real `xetal` binary, the one the goldens
test, compiled to WebAssembly: `xetal run --seed S play.xtl` in the
browser under a small WASI shim, with the game's files in an in-memory
file system and what the player typed as standard input, and `xetal
render --html` for the decorated source. Today `xetal-cli` cannot be
built for `wasm32-wasip1`:

```
$ cd vendor/xetal/components/cli
$ cargo build --release -p xetal-cli --target wasm32-wasip1
error[E0432]: unresolved import `sys::position`
  --> crossterm-0.29.0/src/cursor.rs:52:9
...
```

`crossterm` (no WASI support) comes in through `ratatui` in
`xetal-edit` (`xetal edit`) and `xetal-line` (the REPL's line editor),
which `xetal-cli` always depends on.

Ask: a default-on cargo feature in `xetal-cli` (say `terminal`) that
brings in `xetal-edit` and the `ratatui` line editor; built with
`--no-default-features`, `xetal edit` reports it is unavailable and
`xetal repl` reads plain lines from standard input. Then

```
cargo build --release -p xetal-cli --no-default-features --target wasm32-wasip1
```

gives a `xetal.wasm` exposing `run`, `eval`, `render`, `type` and the
rest unchanged. Nothing else in the CLI looked platform-bound (it
reads files and standard input, writes standard output; `[]S_HOW`
writes numbered files, which a shim can collect).

Workaround meanwhile: each page links the X_eTaL engine library
(`xetal-play`, the same crate X_eTaL's own live demo uses) compiled to
wasm32, and runs the game's unmodified `.xtl` files with it.

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
