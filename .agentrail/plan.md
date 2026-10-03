# idioms

Saga 4 of X_eTaL-games (docs/plan.md): every game rewritten in
idiomatic X_eTaL with the features of the vendored X_eTaL (70e129d):
trains, shared libraries instead of copied helpers, tables over
branches, named parts, array-style solutions; smaller and easier to
read, with the same outputs (goldens, page tests and browser tests
guard every refactor; a changed output is an explained decision).

Every step: `just gate`, docs (README sizes, CHANGES.md, plan, asks,
docs/style.md), a detailed commit to main including .agentrail/,
`agentrail complete`, push.

## Steps

1. metrics-style -- just size; docs/style.md.
2. shared-libraries -- lib/ Play, Board, Text, State; pages load them.
3-9. horse-race, guess, robot-chase, trek-adventure, trek,
   tic-tac-toe, shut-the-box idioms.
10. lights-out -- new, in the new style.
11-12. minesweeper, 2048 idioms.
13. idioms-release.
