# basic

Saga 2 of X_eTaL-games (docs/plan.md): the COR24 BASIC games
(sw-embed/web-sw-cor24-basic/examples/*.bas, MIT, the user's own),
ported to X_eTaL. The user's order: the APL original first (done),
then these, before the grid games.

Each port: the original .bas kept in games/<slug>/original/; the rules
as one library (<Name>.xtl, docs/plan.md A4) imported by the scripted
game, play.xtl (goldens with expected/play.in) and the page; arrays
where the BASIC had loops; the README reads the two side by side.
Text games share a terminal page: an output pane and a command line;
the program re-run with the whole typed history as input and a fixed
seed per game.

Every step: `just gate` passes, docs (README, CHANGES.md, plan, asks)
updated, .gitignore sane, a detailed commit to main including
.agentrail/, `agentrail complete`, push.

## Steps

1. guess -- guess the number; the terminal page pattern.
2. robot-chase -- robots as an N x 2 matrix moving at once; board page.
3. trek-adventure -- tables and a state machine; terminal page.
4. terminal-shell -- the text pages' shared parts moved to shared/.
5. trek -- Star Trek: galaxy and sectors as arrays.
6. gallery-1-release -- catalog, docs, screenshots, retrospective.
