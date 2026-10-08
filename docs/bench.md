# How fast the games run

`just bench` times every game: its scripted program and its terminal
session run by the vendored `xetal` (the fastest of 7 runs, in
milliseconds) and its page's scripted session in headless Chrome. It
compares the times with `bench/baseline.json` and fails when one is
more than 15 % (and 25 ms) slower; the gate runs it. Timings belong to
a machine: on another host the comparison is skipped. After a change
that is meant to cost time (a new X_eTaL, a bigger game), write a new
baseline with `just bench --baseline` and say why in the commit.

The page times include the test's own pauses between typed lines and
the page's replay of everything typed so far, and are one run each:
they are shown for information, and only the native times (scripted
and terminal) can fail the check.

## The baseline

Host max (M1 Max), X_eTaL f823212, 2026-10-04:

| Game | Scripted (ms) | Terminal (ms) | Page (ms) |
| ---- | ------------- | ------------- | --------- |
| horse-race | 6 | 7 | 218 |
| guess | 7 | 6 | 373 |
| robot-chase | 15 | 25 | 409 |
| trek-adventure | 15 | 34 | 1396 |
| trek | 42 | 43 | 1454 |
| tic-tac-toe | 45 | 17 | 320 |
| shut-the-box | 34 | 14 | 372 |
| minesweeper | 38 | 14 | 319 |
| 2048 | 419 | 16 | 529 |
| sudoku | 809 | 29 | 549 |
| capitals | 291 | 635 | 3443 |
| stargazer | 145 | 169 | 1245 |

Sudoku was added 2026-10-05 at X_eTaL 512b3ee (v0.1.0), when it went
live; its scripted program solves four puzzles, the last by search.
Capitals was added 2026-10-07 at X_eTaL 75e6a5c and rebuilt the same
day as a map you click: its terminal run replays the golden's eight
clicks (each drawing the whole map as SVG); the page time is the
browser test's clicks in the dialog.

Star Trek's row was moved 2026-10-07 from 15, 16 and 1059 ms (X_eTaL
f823212) to 42, 43 and 1454 ms: it became 2.7 times slower between
f823212 and v0.1.0 (39 ms with a v0.1.0 build, measured side by side),
not through this repository; the slowdown is filed in
[`xetal-asks.md`](xetal-asks.md), and the row goes back down when
X_eTaL's speed does.

## X_eTaL's table and inner product slowdown

X_eTaL 70e129d made elementwise operations much faster but higher-order
built-ins (`t_able`, `i_nner`) slower (X_eTaL saga 30 is fixing it).
The games, timed on 39938f3 (before) and 70e129d (after), same host,
same programs (`scripts/bench.py --xetal PATH --out FILE`):

| Game | 39938f3 (ms) | 70e129d (ms) | f823212 (ms) |
| ---- | ------------ | ------------ | ------------ |
| robot-chase | 26 / 46 | 44 / 80 | 16 / 26 |
| trek | 14 / 14 | 19 / 18 | 14 / 14 |
| tic-tac-toe | 22 / 8 | 26 / 8 | 27 / 8 |
| minesweeper | 36 / 13 | 38 / 13 | 40 / 16 |
| trek-adventure | 13 / 23 | 14 / 25 | 15 / 31 |
| horse-race | 5 / 6 | 6 / 6 | 9 / 8 |
| guess | 6 / 5 | 6 / 5 | 8 / 6 |
| shut-the-box | (no transpose) | 142 / 38 | 33 / 14 |
| 2048 | (no transpose) | 686 / 26 | 422 / 17 |

(scripted / terminal, the fastest of 7 runs.)

At f823212 the games are as fast as before the slowdown or faster:
robot chase three times faster than at 70e129d, shut the box four
times, 2048 1.6 times. Robot chase had been the clearest case, 1.7
times slower: each turn counts
the robots on every square as one inner product of a 256 by 12 table
(`((r_ange 256) '= t_able k) '+ '* i_nner w`). Shut the box and 2048
use transpose, which 39938f3 does not have.
