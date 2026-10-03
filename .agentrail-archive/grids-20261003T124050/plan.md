# grids

Saga 3 of X_eTaL-games (docs/plan.md): the grid games, P0 first, each
a library of rules plus a terminal game and a scripted game on the
shared page, which shows only what X_eTaL prints or draws (A11):
boards are text X_eTaL prints or pictures it draws (`[]S_HOW
[]G_RID`), never page code. Ports and new games are written in
X_eTaL's own style.

Opens with a refresh of the vendored X_eTaL (transpose landed; the
resumable run, the first part of X_eTaL saga 25). terminal-pane, from
saga 2, waits on the rest of X_eTaL saga 25.

Every step: `just gate` (goldens, page tests, headless Chrome), docs
(README, CHANGES.md, plan, asks), .gitignore sane, a detailed commit
to main including .agentrail/, `agentrail complete`, push.

## Steps

1. vendor-refresh -- `just vendor`, goldens re-run, asks updated.
2. tic-tac-toe -- 3 by 3 board, lines table, random and minimax.
3. shut-the-box -- masks, subset sums.
4. x-pictures -- boards as pictures X_eTaL draws; asks if needed.
5. minesweeper -- counts by rotation, flood fill.
6. 2048 -- compress, merge, pad, transpose.
7. lights-out -- XOR stencil, GF(2) solve.
8. terminal-pane -- X_eTaL's terminal pane replaces the replay.
9. gallery-2-release -- catalog, docs, retrospective.
