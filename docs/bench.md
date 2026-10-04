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
the page's replay of everything typed so far, so they compare runs,
not machines.

## The baseline

Host max (M1 Max), X_eTaL 70e129d, 2026-10-04:

| Game | Scripted (ms) | Terminal (ms) | Page (ms) |
| ---- | ------------- | ------------- | --------- |
| horse-race | 5 | 6 | 221 |
| guess | 6 | 5 | 369 |
| robot-chase | 43 | 79 | 704 |
| trek-adventure | 13 | 25 | 1404 |
| trek | 19 | 17 | 1057 |
| tic-tac-toe | 27 | 8 | 321 |
| shut-the-box | 142 | 38 | 433 |
| minesweeper | 38 | 13 | 320 |
| 2048 | 686 | 26 | 553 |

## X_eTaL's table and inner product slowdown

X_eTaL 70e129d made elementwise operations much faster but higher-order
built-ins (`t_able`, `i_nner`) slower (X_eTaL saga 30 is fixing it).
The games, timed on 39938f3 (before) and 70e129d (after), same host,
same programs (`scripts/bench.py --xetal PATH --out FILE`):

| Game | 39938f3 scripted / terminal (ms) | 70e129d scripted / terminal (ms) |
| ---- | -------------------------------- | -------------------------------- |
| robot-chase | 26 / 46 | 44 / 80 |
| trek | 14 / 14 | 19 / 18 |
| tic-tac-toe | 22 / 8 | 26 / 8 |
| minesweeper | 36 / 13 | 38 / 13 |
| trek-adventure | 13 / 23 | 14 / 25 |
| horse-race | 5 / 6 | 6 / 6 |
| guess | 6 / 5 | 6 / 5 |

Robot chase is the clearest case, 1.7 times slower: each turn counts
the robots on every square as one inner product of a 256 by 12 table
(`((r_ange 256) '= t_able k) '+ '* i_nner w`). Shut the box and 2048
use transpose, which 39938f3 does not have.
