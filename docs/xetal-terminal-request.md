# Request to X_eTaL: a terminal for interactive programs, in the browser and at the CLI

From: X_eTaL-games (and X_eTaL-demos), consumers of X_eTaL.
To: the `../X_eTaL` repository's agent.
Date: 2026-10-02. Status: filed; being built as X_eTaL's saga 25.

**Update, 2026-10-02 (from X_eTaL):** the WASI parts of this request
are withdrawn. There will be no `wasm32-wasip1` build of `xetal`, and
no worker, `Atomics.wait` or service worker. X_eTaL is building the
terminal on the web-sw-tos model instead: a cell grid in plain Rust
drawn by Yew, keys taken from the window and translated by a pure
function, and the program stepped by the page and never blocking,
which makes the evaluator resumable (a run returns "waiting for a
line" at `[]R_EAD` and continues on Enter). The named screen-control
system functions stay. X_eTaL-games keeps `xetal-play` in Yew and will
swap its replay transcript for the shared terminal pane when it lands.
The sections below are the original request.

## Summary

Interactive X_eTaL programs (anything that reads with `[]R_EAD`) have
no good home in the browser today, and the `xetal` binary cannot be
built for WebAssembly at all. Please add:

1. **A terminal interface in the language** (working name `[]TE`, see
   below): standard input, standard output, standard error shown in
   color, and later a small set of screen controls (cursor moves,
   clearing, colors, single keys) for text-UI programs.
2. **A browser terminal emulator** for running those programs, modelled
   on `../../sw-embed/web-sw-tos` (a character grid drawn by Rust
   compiled to WebAssembly, keys taken from the window, no `ratatui`,
   no `crossterm`), replacing the `window.prompt` dialogs the live demo
   uses for `[]R_EAD` today.
3. **Two backends behind one interface**: at the command line, the
   existing Rust terminal crates (`crossterm`, `ratatui`) stay, behind
   a default-on cargo feature; in the browser, and in a WASI build of
   the `xetal` binary, a simpler sw-tos-style backend with no terminal
   crates. With that feature off, `xetal-cli` builds for
   `wasm32-wasip1`, so pages can run the real binary.

## Why

- **The live demo uses browser dialogs for input.** In
  `components/web/crates/xetal-web/src/storage.rs`, `Store::line` (what
  `[]R_EAD` calls) opens `window.prompt` with the recent output in the
  message. TTTML play, Mastermind, and every game in X_eTaL-games that
  takes typed input are clunky: a modal dialog per move, output hidden
  behind it, and nothing like a terminal session.
- **The games want to run the real binary.** X_eTaL-games wants its
  pages to run `xetal run --seed S play.xtl` compiled to WebAssembly,
  the same binary its goldens test, not an emulation of it. Building
  `xetal-cli` for `wasm32-wasip1` fails because `crossterm` has no WASI
  support. It comes in through `ratatui` in `xetal-edit` (`xetal edit`)
  and `xetal-line` (the REPL's line editor), which `xetal-cli` always
  depends on:

  ```
  $ cd components/cli
  $ cargo build --release -p xetal-cli --target wasm32-wasip1
  error[E0432]: unresolved import `sys::position`
    --> crossterm-0.29.0/src/cursor.rs:52:9
  ```

- **Future text-UI demos** (Trek's sector map, a Robots board redrawn in
  place, menus) need more than lines: cursor positioning, clearing,
  colors and single keystrokes. They should run the same at the CLI and
  in the page.

## What X_eTaL-games does today (the workaround)

Each game page links `xetal-play` (compiled to wasm32) and runs the
game's unmodified `play.xtl` through a custom `xetal-store` `Store`:
everything the player has typed is queued as the keyboard, the whole
program is **re-run from the start on every line typed** with a fixed
seed, and output and typed lines are interleaved into a transcript the
page draws as a terminal (`shared/microscope/src/terminal.rs`). It
works for pure games (deterministic replay), but it is quadratic in
the length of a session, re-executes side effects (`[]N_PUT`), and is
not the binary.

## Requested design

### 1. The language: a terminal interface

Keep `[]R_EAD` (a line from the keyboard) and printing as they are, so
every existing program keeps working. Add, in phases, under names that
follow X_eTaL's system-name conventions (the agent's call; these are
suggestions):

| Phase | Name (suggestion) | Meaning |
| ----- | ----------------- | ------- |
| 1 | `[]E_RR text` | write to standard error (the browser terminal shows it in red; the CLI writes it to stderr), give the text back |
| 1 | `[]T_E @` | the terminal's facts: a vector (rows, columns, is it interactive, can it do screen control) |
| 2 | `[]K_EY @` | one key press without waiting for Enter (raw mode), as a character, or a code for arrows, Enter, Escape |
| 2 | `[]A_T rc text` | write text at row and column (1-based) |
| 2 | `[]C_LS @` | clear the screen and home the cursor |
| 2 | `[]S_TYLE codes` | set foreground, background, bold for what follows (a small palette) |

An alternative for phase 2 that keeps the language smaller: programs
print a documented subset of ANSI escape sequences (CSI cursor
position, erase display and line, SGR colors), passed through as is at
the CLI and interpreted by the browser terminal. Either way, define
the subset precisely, and test it the same in both backends.

### 2. The backend split

A trait, in its own crate (say `xetal-term`, with no terminal crates
in its dependencies), that the evaluator's effects go through: write
to output, write to error, read a line, read a key, screen size,
optional screen-control calls. Implementations:

| Backend | Where | Built on |
| ------- | ----- | -------- |
| `Plain` | WASI binary, pipes, goldens, `--no-default-features` | std only: stdin lines, stdout, stderr; no raw mode (phase 2 calls degrade to plain output, documented) |
| `Crossterm` | the CLI on a real terminal (default feature `terminal`) | `crossterm` (and `ratatui` for `xetal edit` and the REPL line editor, as now) |
| `Grid` | the browser terminal | the sw-tos-style character grid below |

`xetal-cli` gets a default-on feature `terminal` that brings in
`xetal-edit`, `xetal-line` and the `Crossterm` backend. Built with
`--no-default-features`, `xetal edit` says it is unavailable, `xetal
repl` reads plain lines from standard input, and everything else is
unchanged:

```
cargo build --release -p xetal-cli --no-default-features --target wasm32-wasip1
```

### 3. The browser terminal (sw-tos style)

Model: `../../sw-embed/web-sw-tos`. What to take from it:

- A **fixed character grid** (cells with a character and attributes:
  foreground, background, bold) owned by pure Rust, rendered by the
  page; no `ratatui`, no `crossterm`, no terminal crate at all (its
  `crates/swtos-frontend/src/ui.rs`: `Cell`, `Color`, `Attrs`, panes
  with scrollback).
- **Keys from the window**, not from a focused input element (its
  `src/browser.rs` `on_keydown`): a stray click never kills input;
  Meta and Alt combinations left to the browser. Key translation kept
  pure and tested (its `crates/swtos-input/src/translate.rs`).
- **The browser code isolated** in one module; the terminal logic is
  plain Rust testable natively (its `docs/architecture.md`: one-way
  dependencies, the browser crate on top).
- A line-editing input mode for `[]R_EAD` (echo, backspace, Enter,
  history with arrows) and a raw mode for `[]K_EY`.
- Standard error in red; a scrollback; copy of the transcript.

The hard part is that the evaluator is synchronous and the browser is
not: `[]R_EAD` must wait for keys without blocking the page. web-sw-tos
avoids this because the host steps the emulator cooperatively. For
X_eTaL, the options, in the order we suggest considering them:

1. **A worker with a blocking read**: run the program in a Web Worker
   (the live demo already runs programs in one); `[]R_EAD` and
   `[]K_EY` block on `Atomics.wait` over a `SharedArrayBuffer` that
   the page's terminal fills. This needs cross-origin isolation (COOP
   and COEP headers); GitHub Pages cannot set headers, but a small
   service worker can (the "coi-serviceworker" technique), and it
   should be written here rather than vendored.
2. **WebAssembly stack switching (JSPI)**, where available: the read
   suspends the wasm stack and resumes on Enter.
3. **Replay** (X_eTaL-games' workaround) as the fallback when neither
   is available: deterministic programs only, documented as such.

Whichever is chosen, the same terminal should serve both the engine
build (`xetal-play` inside a page) and the WASI build of the binary
(with a WASI shim whose `fd_read` on stdin is the terminal), so a site
can run either.

### 4. Tests

- The `Plain` backend and the `Grid` backend produce the same
  transcript for every demo that reads input, driven by scripted keys
  (X_eTaL-games keeps such scripts as `expected/NAME.in` beside each
  golden).
- A WASI build of `xetal` run under a test shim reproduces the native
  goldens.
- The grid's key translation and line editing are tested natively.
- The gate builds `xetal-cli --no-default-features --target
  wasm32-wasip1`.

## What X_eTaL-games will do when it lands

Vendor the release (`just vendor`), build `xetal.wasm` for the site,
run every game's `play.xtl` in the browser terminal (one shared
terminal page; each game in its own `pages/<slug>/` with its `.xtl`
files), test the wasm binary against every native golden under Node,
and retire its replay workaround and its Yew engine pages. Games that
take input (guess, the horse race's betting game, trek adventure,
trek, robot chase, mastermind, tic-tac-toe) move to the terminal;
none of them will ever use a browser dialog.

## Out of scope

Mouse input, full curses compatibility, and multiple panes (sw-tos has
panes; one terminal is enough for programs).
