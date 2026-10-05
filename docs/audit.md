# Status at v0.1.0

The state of this repository when it was tagged `v0.1.0` (2026-10-05),
for a release of the linked X_eTaL repositories. What changed and when
is in [`CHANGES.md`](../CHANGES.md); what comes next is in
[`plan.md`](plan.md).

## What is here

- Nine games, live at <https://softwarewrighter.github.io/X_eTaL-games/>:
  the horse race, guess the number, robot chase, trek adventure, Star
  Trek, tic-tac-toe, shut the box, minesweeper and 2048. Each is a
  rules library, a scripted game and a terminal game in X_eTaL, written
  around one array idea (the README's idiom table), on the shared
  libraries in `lib/` (Play, Text, Board, State).
- Each game's page shows only what X_eTaL prints or draws: the game in
  a terminal, its scripted program as a notebook, and its source; its
  title links to Wikipedia or opens a short history.
- X_eTaL is pinned at `f823212f87bb46292f98435225334330e8701ed6`
  (`XETAL_COMMIT`), cloned and built by `just xetal`; nothing of
  X_eTaL's is tracked here. The site is built locally and published as
  the one commit of the `gh-pages` branch (`just publish`).

## What the gate checks

`just gate`: X_eTaL builds at the pinned commit and answers; American
spellings only (with the checker's self-test); the game tooling's
self-test; the shared libraries' tests and types; every library's type
comments against the inferred types; the page shell's tests; every
game's goldens (scripted programs, terminal sessions with typed input,
library types), page tests and wasm build; the site built fresh and
played in headless Chrome (terminal and notebook against the goldens,
pictures, title links and dialogs, the catalog); every game timed
against the baseline (more than 15 % slower fails); ASCII-only
markdown.

The timing baseline (host max, X_eTaL f823212, the fastest of 7
runs; [`bench.md`](bench.md)):

| Game | Scripted (ms) | Terminal (ms) |
| ---- | ------------- | ------------- |
| horse-race | 6 | 7 |
| guess | 7 | 6 |
| robot-chase | 15 | 25 |
| trek-adventure | 15 | 34 |
| trek | 15 | 16 |
| tic-tac-toe | 45 | 17 |
| shut-the-box | 34 | 14 |
| minesweeper | 38 | 14 |
| 2048 | 419 | 16 |

## Asks to X_eTaL

[`xetal-asks.md`](xetal-asks.md) in full; their states at this tag:

| State | Ask |
| ----- | --- |
| filed | A terminal for interactive programs (the request: [`xetal-terminal-request.md`](xetal-terminal-request.md)); X... |
| open | `[]G_RID` of numbers (still so at f823212): draw the numbers in the cells, and let a program choose the color ... |
| open | A functional update (amend / "at"; still none at f823212): items of an array replaced at given positions, retu... |
| open | Local functions: a function defined inside a lambda (`n_ear := { t -> ... }`) and applied there |
| open | Named dyadic trains (still so at f823212): `u:s_ign := [> - <]` is monadic only, so a dyadic fork must be writ... |
| open | `p_ower` with a Bool count (still so at f823212) (a 0/1 condition), as other numeric places accept Truthy valu... |
| landed | `t_able` and `i_nner` slower since the higher-order built-ins became kernels (X_eTaL saga 30): robot chase 1.7... |
| landed | An executable program (shebang) that defines `l:` names is taken for a library: MC8 row 9 is never reported (l... |

## The linked repositories

At this tag: X_eTaL (no tag yet), X_eTaL-demos v0.1.0, X_eTaL-ML (no tag
yet), X_eTaL-libraries v0.4.0, X_eTaL-extensions v0.1.0. Every live site
and repository the README and the catalog link to answered (checked
2026-10-05).

## The saga record

`agentrail audit`: every work commit is claimed by a saga step; the
commits left unclaimed are the bookkeeping commits that close each
step ("saga: complete ..."). The history was rewritten once before
this tag (the user's decision, 2026-10-05) to drop the previously
tracked site and X_eTaL copy; the saga records were mapped to the new
commits.

## Known limits

- The page's terminal replays everything typed on every line; it moves
  to X_eTaL's own terminal pane when X_eTaL releases it (`plan.md`,
  saga 5).
- `[]G_RID` cannot draw numbers in cells, so boards of numbers (2048)
  are text.
- Named dyadic trains, Bool power counts and an amend are still asked
  for; the games use the documented workarounds.
