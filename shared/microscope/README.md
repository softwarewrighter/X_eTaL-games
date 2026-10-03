# The microscope: the game pages' shared shell

Every game page runs the game's X_eTaL programs in the browser and
shows what they print, beside their decorated source. This crate (`microscope`) is what the pages share.
It started as a copy of X_eTaL-demos' shell of the same name and is
maintained here independently; a game's page is `page::GamePage`,
which shows only what X_eTaL prints or draws.

| Module | What it gives a page |
| ------ | -------------------- |
| `run` | `output(src, lines)` runs a program (the vendored `xetal-play`) and returns its printed lines, or the X_eTaL error; `output_seeded` the same with a seed for `r_oll!`; `library(name, text)` puts a game's rules library where `u_se<` finds it (an in-memory store, in the browser and in tests); `numbers(line, n)` reads one printed array (`r_avel`); `lit(x)` writes a Float as an X_eTaL literal (plain decimal: X_eTaL literals have no exponent); `matrix(name, rows, cols, items)` binds a matrix; `section(src, start, end)` cuts the core out of a game's `.xtl`; `now()` times a run |
| `source` | `code(src)` draws any snippet decorated and coloured, as X_eTaL renders it; `line(src, range)` and `block(src, range)` draw a line or a program with `range` highlighted as one block; `between` and `find` compute ranges; `shape(dims, meaning)` labels a shape as `s_hape = ...` with a tooltip |
| `canvas`, `colour` | `Canvas` draws an RGBA array scaled to its box and reports clicks as (row, column); `colour` turns arrays into pixels (`field`, `scaled`, `signed`, `mask`, `ramp`) |
| `cells` | small boards as clickable HTML cells (0 / 1 boards, shaded counts with numbers) |
| `terminal` | `session(libraries, src, typed, seed)` runs a terminal program with everything typed so far as its keyboard and returns the transcript (printed lines, typed lines and `[]S_HOW` pictures, in order), whether it is waiting for the next line, and any error; `notebook(libraries, src, seed)` runs a program as `just show` does: each statement and what it printed |
| `page` | `GamePage`: a game's page from its files (`Game`: title, lede, `play.xtl`, the scripted program, the library): the terminal, the notebook, the sources; nothing else |
| `chrome` | `header` (logo, title, lede), `chip` (a stage: name, code, shape), `panel`, `notice`, `footer` (copyright, license, repository, all games, the vendored X_eTaL commit, build host, sha and time) |

`microscope.css` is the shared stylesheet (light and dark themes, the
highlight, panels, chips, canvases); a page links it with
`<link data-trunk rel="css" href="../../../shared/microscope/microscope.css" />`.

## A new game's web app

A game page shows only what X_eTaL prints or draws (`docs/plan.md`
A11), so a game's web app is a few lines: `page::GamePage` with the
game's files.

1. `just new-game SLUG "Title"`; write the rules library
   `<Name>.xtl`, the scripted `SLUG.xtl`, the terminal game
   `play.xtl`, and their goldens (`expected/play.in` holds a session's
   typed lines).
2. `games/SLUG/web/`, a Cargo workspace like `games/guess/web/`:
   `Cargo.toml` (microscope, yew, console_error_panic_hook),
   `index.html` (the trunk links: the binary, logo, favicon,
   `microscope.css`), `src/lib.rs` returning the `page::Game` (title,
   lede, the three files by `include_str!`, and the shared `lib/`
   libraries it imports, in `shared`), `src/main.rs`
   rendering `GamePage` with it, and `tests/page.rs` checking that the
   terminal given `expected/play.in` on seed 1 prints
   `expected/play.out` and the notebook prints `expected/SLUG.out`.
3. `just serve SLUG` while working; `just test-game SLUG`; `just
   pages`; `just browser-test SLUG` (headless Chrome plays the built
   page); set `status = "live"` in `game.toml`.

The page: the terminal (`play.xtl`, everything typed replayed on the
same seed; `?seed=N` in the address replays game N), the notebook
(`SLUG.xtl` on seed 1, as `just show`), and the program and library
sources, decorated by X_eTaL's renderer. Its only controls are New
game and Restart. Pictures appear only when the program shows them
(`[]S_HOW`, typically `[]S_HOW []G_RID board`): the command line writes
them to `work/draw/<slug>/` (the tests ignore its "drawn PATH" lines),
the page shows them in the terminal or the notebook, scaled up to read. The other modules (`canvas`, `cells`, `colour`, stage
chips and panels in `chrome`) came from X_eTaL-demos; a game page must
not use them to draw what the game computes.

Conventions: no browser dialogs (alert, prompt) for input, ever; an
X_eTaL error is shown as X_eTaL reports it; anything X_eTaL lacks goes
in `docs/xetal-asks.md` and the game's README.
